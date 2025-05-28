use crate::entity::patient::{ActiveModel, Model, PatientRequestBody};
use chrono::{NaiveDateTime, Utc};

use crate::entity::patient;
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic};
use sea_orm::{ActiveModelTrait, PaginatorTrait, Set};
use sea_orm::{ColumnTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct PatientService {
    conn: DatabaseConnection,
}

impl PatientService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        PatientService { conn: conn.clone() }
    }
    pub async fn create_patient(
        &self,
        patient_data: Option<PatientRequestBody>,
    ) -> Result<Model, CustomError> {
        // Check if patient_data exists
        let payload = match patient_data.clone() {
            Some(data) => data,
            None => return Err(CustomError::new(400, "Missing patient data".to_string())),
        };

        let now = Utc::now().naive_utc();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique patient IC after multiple attempts.".to_string(),
                ));
            }

            let active_model = Self::generate_model(Some(payload.clone()), now);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;

            if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                return value;
            }
        }
    }
    pub async fn find_by_name(&self, first_name: String) -> Result<Option<Model>, CustomError> {
        let patient = patient::Entity::find()
            .filter(patient::Column::FirstName.like(&first_name))
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)));

        match patient {
            Ok(Some(patient_model)) => Ok(Option::from(patient_model)),
            Ok(None) => Err(CustomError::new(
                404,
                format!("Patient with name '{}' not found", first_name),
            )),
            Err(e) => Err(CustomError::new(500, format!("Database error: {}", e))),
        }
    }

    pub async fn find_all(
        &self,         // Changed to &self as we're not modifying the service state
        page: u64,     // Use u64 for pagination
        per_page: u64, // Use u64 for pagination
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let paginator = patient::Entity::find().paginate(&self.conn, per_page);

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

    fn generate_model(p0: Option<PatientRequestBody>, p1: NaiveDateTime) -> ActiveModel {
        let payload = p0.unwrap_or_default();
        ActiveModel {
            patient_ic: Set(Some(generate_ic().to_string())),
            hospital_id: if let Some(value) = payload.hospital_id {
                Set(value)
            } else {
                Set(Default::default())
            },
            first_name: if let Some(value) = payload.first_name {
                Set(value)
            } else {
                Set(Default::default())
            },
            last_name: if let Some(value) = payload.last_name {
                Set(value)
            } else {
                Set(Default::default())
            },
            date_of_birth: if let Some(value) = payload.date_of_birth {
                Set(value)
            } else {
                Set(Default::default())
            },
            gender: if let Some(value) = payload.gender {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            phone: if let Some(value) = payload.phone {
                Set(value)
            } else {
                Set(Default::default())
            },
            address: if let Some(value) = payload.address {
                Set(value)
            } else {
                Set(Default::default())
            },
            created_at: Set(p1),
            updated_at: Set(p1),
            email: if let Some(value) = payload.email {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            emergency_contact: if let Some(value) = payload.emergency_contact {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            blood_type: if let Some(value) = payload.blood_type {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            allergies: if let Some(value) = payload.allergies {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            medical_history: if let Some(value) = payload.medical_history {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            id: Default::default(),
        }
    }
}
