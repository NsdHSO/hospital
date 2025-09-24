use crate::entity::user_profile::{ActiveModel as UserProfileActive, Column as UserProfileCol, Entity as UserProfile, Model as UserProfileModel};
use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm::JsonValue;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

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

pub struct MeService {
    conn: DatabaseConnection,
}

impl MeService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self { conn: conn.clone() }
    }

    pub async fn get_profile(&self, user_sub: &str) -> Result<Option<ProfileDto>, CustomError> {
let rec: Option<UserProfileModel> = UserProfile::find()
            .filter(UserProfileCol::UserSub.eq(user_sub))
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB error: {e}")))?;

        Ok(rec.map(|m| ProfileDto { schema_version: m.schema_version, attributes: m.attributes_json.into() }))
    }

    pub async fn upsert_profile(&self, user_sub: &str, body: UpsertProfileBody) -> Result<ProfileDto, CustomError> {
        // Try existing
if let Some(mut existing) = UserProfile::find()
            .filter(UserProfileCol::UserSub.eq(user_sub))
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB error: {e}")))?
        {
            existing.schema_version = body.schema_version.unwrap_or(existing.schema_version);
            // Convert serde_json::Value to sea_orm::JsonValue
            let attributes_json: JsonValue = body.attributes.clone().into();
            let mut active: UserProfileActive = existing.into();
            active.attributes_json = Set(attributes_json);
            let updated = active
                .update(&self.conn)
                .await
                .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB update error: {e}")))?;
            return Ok(ProfileDto { schema_version: updated.schema_version, attributes: updated.attributes_json.into() });
        }

        // Create new
        let attributes_json: JsonValue = body.attributes.clone().into();
        let active = UserProfileActive {
            user_sub: Set(user_sub.to_string()),
            schema_version: Set(body.schema_version.unwrap_or(1)),
            attributes_json: Set(attributes_json),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };
        let created = active
            .insert(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB insert error: {e}")))?;

        Ok(ProfileDto { schema_version: created.schema_version, attributes: created.attributes_json.into() })
    }
}
