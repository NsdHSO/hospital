use crate::entity::user_profile::{
    ActiveModel as UserProfileActive, Column as UserProfileCol, Entity as UserProfile,
    Model as UserProfileModel,
};
use crate::components::auth_identity::AuthIdentityService;
use crate::components::person::PersonService;
use crate::components::staff::StaffService;
use crate::components::hospital::HospitalService;
use crate::http_response::HttpCodeW;
use crate::http_response::error_handler::CustomError;
use sea_orm::DatabaseConnection;
use sea_orm::JsonValue;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileDto {
    pub schema_version: i32,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertProfileBody {
    pub attributes: serde_json::Value,
    #[serde(default)]
    pub schema_version: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkBody {
    pub person_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub staff_ic: Option<String>,
    pub email: Option<String>,
}

pub struct MeService {
    conn: DatabaseConnection,
    person_service: PersonService,
    staff_service: StaffService,
    hospital_service: HospitalService,
    auth_identity_service: AuthIdentityService,
}

impl MeService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            conn: conn.clone(),
            person_service: PersonService::new(conn),
            staff_service: StaffService::new(conn),
            hospital_service: HospitalService::new(conn),
            auth_identity_service: AuthIdentityService::new(conn),
        }
    }

    #[inline]
    fn dto_from_model(m: UserProfileModel) -> ProfileDto {
        ProfileDto { schema_version: m.schema_version, attributes: m.attributes_json.into() }
    }

    async fn insert_profile_internal(
        &self,
        user_sub: &str,
        schema_version: i32,
        attrs: JsonValue,
    ) -> Result<ProfileDto, CustomError> {
        let active = UserProfileActive {
            user_sub: Set(user_sub.to_string()),
            schema_version: Set(schema_version),
            attributes_json: Set(attrs),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };
        let created = active
            .insert(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB insert error: {e}")))?;
        Ok(Self::dto_from_model(created))
    }

    async fn single_hospital_default(&self) -> Result<Option<JsonValue>, CustomError> {
        let page = self.hospital_service.find_all(0, 1).await?;
        if page.pagination.total_items == 1 {
            let h = &page.data[0];
            let obj = serde_json::json!({
                "hospital_id": h.id.to_string(),
                "role": "USER",
                "on_call": false
            });
            return Ok(Some(obj.into()));
        }
        Ok(None)
    }

    async fn fetch_profile_model(&self, user_sub: &str) -> Result<Option<UserProfileModel>, CustomError> {
        let rec: Option<UserProfileModel> = UserProfile::find()
            .filter(UserProfileCol::UserSub.eq(user_sub))
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB error: {e}")))?;
        Ok(rec)
    }

    pub async fn get_profile(&self, user_sub: &str) -> Result<Option<ProfileDto>, CustomError> {
        let rec = self.fetch_profile_model(user_sub).await?;

        Ok(rec.map(|m| ProfileDto {
            schema_version: m.schema_version,
            attributes: m.attributes_json.into(),
        }))
    }

    pub async fn get_or_provision_profile(&self, user_sub: &str) -> Result<ProfileDto, CustomError> {
        if let Some(p) = self.get_profile(user_sub).await? {
            return Ok(p);
        }
        if let Some(attrs) = self.derive_defaults_from_domain(user_sub).await? {
            return self.insert_profile_internal(user_sub, 1, attrs.clone()).await;
        }
        Err(CustomError::new(
            HttpCodeW::Forbidden,
            "Profile not found and cannot auto-provision".to_string(),
        ))
    }

    pub async fn link_identity(&self, user_sub: &str, body: LinkBody) -> Result<ProfileDto, CustomError> {
        // Resolve a person_id from one of the provided identifiers
        let person_id = if let Some(pid) = body.person_id {
            // Validate person exists via PersonService
            let page = self
                .person_service
                .find_persons(Some("id"), Some(&pid.to_string()), Some(1), Some(1))
                .await?;
            if page.data.is_empty() {
                return Err(CustomError::new(HttpCodeW::BadRequest, "person_id not found".into()));
            }
            pid
        } else if let Some(sid) = body.staff_id {
            match self.staff_service.find_by_field("id", &sid.to_string()).await {
                Ok(Some(st)) => st.id,
                Ok(None) => return Err(CustomError::new(HttpCodeW::BadRequest, "staff_id not found".into())),
                Err(e) if matches!(e.error_status_code as u16, 404) => {
                    return Err(CustomError::new(HttpCodeW::BadRequest, "staff_id not found".into()))
                }
                Err(e) => return Err(e),
            }
        } else if let Some(ic) = body.staff_ic {
            match self.staff_service.find_by_field("staff_ic", &ic).await {
                Ok(Some(st)) => st.id,
                Ok(None) => return Err(CustomError::new(HttpCodeW::BadRequest, "staff_ic not found".into())),
                Err(e) if matches!(e.error_status_code as u16, 404) => {
                    return Err(CustomError::new(HttpCodeW::BadRequest, "staff_ic not found".into()))
                }
                Err(e) => return Err(e),
            }
        } else if let Some(email) = body.email {
            let page = self
                .person_service
                .find_persons(Some("email"), Some(&email), Some(1), Some(1))
                .await?;
            if let Some(p) = page.data.first() {
                p.id
            } else {
                return Err(CustomError::new(HttpCodeW::BadRequest, "email not found".into()));
            }
        } else {
            return Err(CustomError::new(HttpCodeW::BadRequest, "provide one of: person_id | staff_id | staff_ic | email".into()));
        };

        // Upsert mapping via service
        self.auth_identity_service.upsert_mapping(user_sub, person_id).await?;

        // Return the (now) provisioned profile
        self.get_or_provision_profile(user_sub).await
    }

    async fn derive_defaults_from_domain(&self, user_sub: &str) -> Result<Option<JsonValue>, CustomError> {
        // First, map auth user_sub -> person_id if present
        if let Some(map) = self.auth_identity_service.find_by_user_sub(user_sub).await? {
            let pid = map.person_id;
            if let Ok(Some(st)) = self.staff_service.find_by_field("id", &pid.to_string()).await {
                let role_str = serde_json::to_value(&st.role)
                    .unwrap_or(serde_json::Value::String("USER".into()));
                let obj = serde_json::json!({
                    "hospital_id": st.hospital_id.to_string(),
                    "department_id": st.department_id.to_string(),
                    "role": role_str,
                    "on_call": false
                });
                return Ok(Some(obj));
            }
        }
        // Fallback: if exactly one hospital exists, default to it
        if let Some(def) = self.single_hospital_default().await? { return Ok(Some(def)); }
        Ok(None)
    }

    pub async fn upsert_profile(
        &self,
        user_sub: &str,
        body: UpsertProfileBody,
    ) -> Result<ProfileDto, CustomError> {
        if let Some(mut existing) = self.fetch_profile_model(user_sub).await? {
            existing.schema_version = body.schema_version.unwrap_or(existing.schema_version);
            // Convert serde_json::Value to sea_orm::JsonValue
            let attributes_json: JsonValue = body.attributes.clone().into();
            let mut active: UserProfileActive = existing.into();
            active.attributes_json = Set(attributes_json);
            let updated = active.update(&self.conn).await.map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("DB update error: {e}"),
                )
            })?;
            return Ok(ProfileDto {
                schema_version: updated.schema_version,
                attributes: updated.attributes_json.into(),
            });
        }

        // Create new
        let attributes_json: JsonValue = body.attributes.clone().into();
        self.insert_profile_internal(user_sub, body.schema_version.unwrap_or(1), attributes_json).await
    }
}
