use crate::components::patient::PatientService;
use crate::entity;
use crate::entity::ambulance::{AmbulanceId};
use crate::entity::{ambulance, emergency};
use crate::entity::emergency::{ActiveModel, EmergencyRequestBody, Entity, Model};
use crate::entity::sea_orm_active_enums::{
    AmbulanceStatusEnum, EmergencySeverityEnum, EmergencyStatusEnum,
};
use crate::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic, now_time};
use chrono::NaiveDateTime;
use percent_encoding::percent_decode_str;
use sea_orm::{ActiveModelTrait, ColumnTrait, NotSet, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, Set};
use uuid::Uuid;
use crate::entity::emergency::Column::{EmergencyIc, Id};
// Adjust the path if needed

pub struct EmergencyService {
    conn: DatabaseConnection,
    patient_service: PatientService,
}

impl EmergencyService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        EmergencyService {
            conn: conn.clone(),
            patient_service: PatientService::new(conn),
        }
    }

    pub async fn find_by_ic(
        &self,
        emergency_ic: &str,
    ) -> Result<Option<serde_json::Value>, CustomError> {
        let result = emergency::Entity::find()
            .filter(EmergencyIc.eq(emergency_ic))
            .find_also_related(ambulance::Entity)
            .one(&self.conn)
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Database error: {e}"),
                )
            })?;

        if let Some((emergency, ambulance_opt)) = result {
            let patient_models = self
                .patient_service
                .find_patients_by_emergency_id(emergency.id);
            let patients_json =
                serde_json::to_value(patient_models.await?).unwrap_or(serde_json::json!([]));
            let ambulance_json =
                ambulance_opt.map(|a| serde_json::to_value(a).unwrap_or(serde_json::json!({})));
            let emergency_json = serde_json::to_value(&emergency).unwrap_or(serde_json::json!({}));
            let mut merged = emergency_json.as_object().cloned().unwrap_or_default();
            merged.insert(
                "ambulance".to_string(),
                ambulance_json.unwrap_or(serde_json::json!({})),
            );
            merged.insert("patients".to_string(), patients_json);
            Ok(Some(serde_json::Value::Object(merged)))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all(
        &self,         // Changed to &self as we're not modifying the service state
        page: u64,     // Use u64 for pagination
        per_page: u64, // Use u64 for pagination
        filter: Option<String>,
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let mut query = Entity::find();

        if let Some(filter_str) = filter {
            match filter_str.split_once('=') {
                Some(("ic", encoded_name)) => {
                    let emergency_ic = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|ic| ic.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
               

                    query = query.filter(EmergencyIc.eq(emergency_ic));
                }
                Some(("id", encoded_name)) => {
                    let ambulance_id = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
                    let emergency_id = Uuid::parse_str(&ambulance_id).map_err(|_| {
                        CustomError::new(HttpCodeW::BadRequest, "Invalid UUID".to_string())
                    })?;
                    query = query.filter(Id.eq(emergency_id));
                }
                _ => {}
            }
        }
        
        let paginator = query.paginate(&self.conn, per_page);
        let total_items = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        let records = paginator
            .fetch_page(page) // Page is 0-indexed in SeaORM
            .await?;

        let pagination = PaginationInfo {
            current_page: page as i64, // Convert back to i64 if needed for your PaginatedResponse
            page_size: per_page as i64, // Convert back to i64
            total_items: total_items as i64, // Convert back to i64
            total_pages: total_pages as i64, // Convert back to i64
            has_next_page: page < total_pages,
            has_previous_page: page > 1,
        };

        Ok(PaginatedResponse {
            data: records,
            pagination,
        })
    }

    pub async fn create_emergency(
        &self,
        emergency_data: EmergencyRequestBody,
    ) -> Result<Model, CustomError> {
        self.create_emergency_internal(emergency_data).await
    }

    async fn create_emergency_internal(
        &self,
        emergency_data: EmergencyRequestBody,
    ) -> Result<Model, CustomError> {
        let now = now_time();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }

            let emergency_ic = generate_ic();
            let active_model =
                Self::generate_model(emergency_data.clone(), now, emergency_ic.to_string());
            let result = active_model.insert(&self.conn).await;
            match result {
                Ok(model) => {
                    // Associate patients if provided
                    if let Some(patients) = emergency_data.patients.as_ref() {
                        self.patient_service
                            .associate_patients_with_emergency(model.id, patients, &self.conn)
                            .await?;
                    }
                    return Ok(model);
                }
                Err(e) => {
                    let err_string = e.to_string();
                    if check_if_is_duplicate_key_from_data_base::<Model>(&mut attempts, Err(e))
                        .is_some()
                    {
                        println!("Duplicate key error: {err_string}, retrying...");
                        // continue loop with new attempt
                    } else {
                        return Err(CustomError::new(
                            HttpCodeW::InternalServerError,
                            format!("Database error: {err_string}"),
                        ));
                    }
                }
            }
        }
    }

    pub async fn schedule_emergency(self) -> Result<(), CustomError> {
        let available_ambulances = ambulance::Entity::find()
            .filter(ambulance::Column::Status.eq(AmbulanceStatusEnum::Available)) // Assuming AmbulanceStatusEnum::Available exists
            .all(&self.conn)
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Failed to fetch available ambulances: {e}"),
                )
            })?;

        if available_ambulances.is_empty() {
            println!("No available ambulances to schedule the emergency.");
            return Ok(());
        }

        let assigned_ambulance = &available_ambulances[0];

        let mut ambulance_active_model: ambulance::ActiveModel = assigned_ambulance.clone().into(); // Convert to ActiveModel
        ambulance_active_model.status = Set(AmbulanceStatusEnum::Dispatched); // Assuming AmbulanceStatusEnum::EnRoute exists
        ambulance_active_model.updated_at = Set(now_time());
        ambulance_active_model
            .update(&self.conn)
            .await // Save the changes to the database
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Failed to update ambulance status: {e}"),
                )
            })?;

        Ok(())
    }

    /// Returns the passengers JSON for the ambulance currently assigned to an emergency, if any.
    pub async fn get_passengers_json_for_ambulance(
        &self,
        ambulance_id: AmbulanceId,
    ) -> Result<Option<serde_json::Value>, CustomError> {
        use crate::entity::emergency;
        // Find the emergency where this ambulance is assigned
        let mut query = emergency::Entity::find(); // Start with the base query

        // Apply the filter based on the AmbulanceId variant
        query = match ambulance_id {
            AmbulanceId::Uuid(uuid_value) => {
                // Assuming emergency::Column::AmbulanceId is of type Uuid
                query.filter(emergency::Column::AmbulanceId.eq(Some(uuid_value)))
            }
            AmbulanceId::Integer(_) => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Cannot filter emergencies by integer ambulance ID directly for this operation. UUID required.".to_string(),
                ))
            }
        };
        let emergency_entity = query.one(&self.conn).await?;

        if let Some(emergency) = emergency_entity {
            // Update emergency.updated_at
            let mut emergency_active: ActiveModel = emergency.clone().into();
            emergency_active.updated_at = Set(now_time());
            let _ = emergency_active.update(&self.conn).await;

            // Fetch all patients for this emergency
            let patients = self
                .patient_service
                .find_patients_by_emergency_id(emergency.id)
                .await?;
            let passengers_json = serde_json::to_value(&patients).map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Serialization error: {e}"),
                )
            })?;
            Ok(Some(passengers_json))
        } else {
            Ok(None)
        }
    }

    fn generate_model(
        emergency_data: EmergencyRequestBody,
        now: NaiveDateTime,
        emergency_ic: String,
    ) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            emergency_ic: Set(emergency_ic),
            created_at: Set(now),
            updated_at: Set(now),
            reported_by: Set(Some(1)),
            hospital_id: Set(None), // Explicitly set to NULL when creating
            notes: Set(emergency_data.notes),
            resolved_at: Set(Option::from(now)),
            modification_attempts: Set(None),
            ambulance_id: NotSet,
            emergency_latitude: Set(emergency_data.emergency_latitude),
            emergency_longitude: Set(emergency_data.emergency_longitude),
            status: Set(EmergencyStatusEnum::Pending),
            severity: Set(EmergencySeverityEnum::Unknown),
            incident_type: Set(emergency_data.incident_type),
            description: Set(emergency_data.description),
        }
    }
}
