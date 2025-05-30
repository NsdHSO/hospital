use crate::components::patient::PatientService;
use crate::entity;
use crate::entity::emergency::{ActiveModel, EmergencyRequestBody, Model};
use crate::entity::sea_orm_active_enums::{
    AmbulanceStatusEnum, EmergencySeverityEnum, EmergencyStatusEnum,
};
use crate::entity::{emergency, emergency_patient, patient};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic};
use chrono::{NaiveDateTime, Utc};
use entity::ambulance;
use sea_orm::{ActiveModelTrait, ColumnTrait, NotSet, PaginatorTrait, QuerySelect, RelationTrait, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, Set};
// Adjust the path if needed

pub struct EmergencyService {
    conn: DatabaseConnection,
    patient_service: PatientService,
}

impl EmergencyService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        EmergencyService {
            conn: conn.clone(),
            patient_service: PatientService::new(&conn),
        }
    }

    pub async fn find_by_ic(&self, ambulance_ic: &str) -> Result<Option<serde_json::Value>, CustomError> {
        let result = emergency::Entity::find()
            .filter(emergency::Column::EmergencyIc.eq(ambulance_ic))
            .find_also_related(ambulance::Entity)
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))?;

        if let Some((emergency, ambulance_opt)) = result {
            let patient_models = self.patient_service.find_patients_by_emergency_id(emergency.id);
            let patients_json = serde_json::to_value(
                patient_models.await?
            ).unwrap_or(serde_json::json!([]));
            let ambulance_json = ambulance_opt.map(|a| serde_json::to_value(a).unwrap_or(serde_json::json!({})));
            let emergency_json = serde_json::to_value(&emergency).unwrap_or(serde_json::json!({}));
            let mut merged = emergency_json.as_object().cloned().unwrap_or_default();
            merged.insert("ambulance".to_string(), ambulance_json.unwrap_or(serde_json::json!({})));
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
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let paginator = emergency::Entity::find().paginate(&self.conn, per_page);

        let total_items = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        let records = paginator
            .fetch_page(page - 1) // Page is 0-indexed in SeaORM
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
        emergency_data: EmergencyRequestBody
    ) -> Result<Model, CustomError> {
        let transaction = self.conn.begin().await?;

        let now = Utc::now().naive_utc();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;
        let mut emergency_model_result: Result<Model, CustomError>;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }

            let emergency_ic = generate_ic();
            let active_model = Self::generate_model(emergency_data.clone(), now, emergency_ic.to_string());

            let result = active_model.insert(&transaction).await;
            match result {
                Ok(model) => {
                    emergency_model_result = Ok(model);
                    break;
                }
                Err(e) => {
                    // Convert error to string so it can be reused
                    let err_string = e.to_string();
                    if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, Err(e)) {
                        emergency_model_result = value;
                        // continue loop
                    } else {
                        emergency_model_result = Err(CustomError::new(500, format!("Database error: {}", err_string)));
                        break;
                    }
                }
            }
        }

        let emergency_model = emergency_model_result?; // Propagate error if IC generation failed

        // Associate patients if provided
        if let Some(patients) = emergency_data.patients.as_ref() {
            self.patient_service
                .associate_patients_with_emergency(
                    emergency_model.id,
                    patients,
                    &transaction,
                )
                .await?;
        }

        transaction.commit().await?; // Commit all operations in the transaction
        Ok(emergency_model)
    }
    
    pub async fn schedule_emergency(self) -> Result<(), CustomError> {
        let available_ambulances = ambulance::Entity::find()
            .filter(ambulance::Column::Status.eq(AmbulanceStatusEnum::Available)) // Assuming AmbulanceStatusEnum::Available exists
            .all(&self.conn)
            .await
            .map_err(|e| {
                CustomError::new(500, format!("Failed to fetch available ambulances: {}", e))
            })?;

        if available_ambulances.is_empty() {
            println!("No available ambulances to schedule the emergency.");
            return Ok(());
        }

        let assigned_ambulance = &available_ambulances[0];

        let mut ambulance_active_model: ambulance::ActiveModel = assigned_ambulance.clone().into(); // Convert to ActiveModel
        ambulance_active_model.status = Set(AmbulanceStatusEnum::Dispatched); // Assuming AmbulanceStatusEnum::EnRoute exists
        ambulance_active_model.updated_at = Set(Utc::now().naive_utc());
        let updated_ambulance = ambulance_active_model
            .update(&self.conn)
            .await // Save the changes to the database
            .map_err(|e| {
                CustomError::new(500, format!("Failed to update ambulance status: {}", e))
            })?; // This returns the updated Model

        println!(
            "Emergency scheduled and ambulance assigned. {:?}",
            updated_ambulance
        );

        Ok(())
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
            notes: Set(emergency_data.notes.clone()), // Clone notes if needed for retries
            resolved_at: Set(Option::from(now)),
            // Handle the modification_attempts field
            modification_attempts: Set(None),
            id_ambulance: NotSet,
            emergency_latitude: Set(emergency_data.emergency_latitude), // Clone if needed
            emergency_longitude: Set(emergency_data.emergency_longitude), // Clone if needed
            status: Set(EmergencyStatusEnum::Pending),
            severity: Set(EmergencySeverityEnum::Unknown),
            incident_type: Set(emergency_data.incident_type.clone()), // Clone if needed
            description: Set(emergency_data.description.clone()),     // Clone if needed
        }
    }
}
