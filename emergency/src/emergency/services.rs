use crate::db::config::connection;
use crate::entity::emergency;
use crate::entity::emergency::{ActiveModel, EmergencyRequestBody, Model};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use chrono::{NaiveDateTime, Utc};
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, NotSet, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, Set};

use crate::entity::sea_orm_active_enums::{EmergencySeverityEnum, EmergencyStatusEnum};
// Adjust the path if needed

pub struct EmergencyService {
    conn: DatabaseConnection,
}

impl EmergencyService {
    pub async fn new() -> Result<Self, CustomError> {
        let conn = connection().await?; // Changed connection handling
        Ok(EmergencyService { conn: conn.clone() })
    }

    pub async fn find_by_ic(&self, ambulance_ic: &str) -> Result<Option<Model>, CustomError> {
        emergency::Entity::find()
            .filter(emergency::Column::EmergencyIc.eq(ambulance_ic))
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))
    }

    pub async fn find_all(
        &self,         // Changed to &self as we're not modifying the service state
        page: u64,     // Use u64 for pagination
        per_page: u64, // Use u64 for pagination
    ) -> Result<PaginatedResponse<Vec<emergency::Model>>, CustomError> {
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
    ) -> Result<emergency::Model, CustomError> {
        // Generate unique emergency_ic (using nanoid for a short, unique string)
        let emergency_ic = nanoid!(30, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
        let now = Utc::now().naive_utc();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5; 

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }

            // Generate a unique emergency_ic (using nanoid for a short, unique string)
            let emergency_ic = nanoid!(30, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);

            let active_model = Self::generate_model(emergency_data.clone(), now, emergency_ic);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;

            match result {
                Ok(model) => return Ok(model), // Successfully inserted, return the model
                Err(DbErr::Exec(e)) => {
                    // Check if the error is a unique constraint violation
                    // The exact string to check for might vary slightly depending on the database
                    if e.to_string()
                        .contains("duplicate key value violates unique constraint")
                    {
                        // It's a unique constraint violation, retry with a new IC
                        attempts += 1;
                        // Continue the loop to generate a new IC and retry
                    } else {
                        // Some other execution error, return it
                        return Err(CustomError::from(DbErr::Exec(e)));
                    }
                }
                Err(e) => {
                    // Other types of database errors, return them
                    return Err(CustomError::from(e));
                }
            }
        }
    }

    fn generate_model(emergency_data: EmergencyRequestBody, now: NaiveDateTime, emergency_ic: String) -> ActiveModel {
        emergency::ActiveModel {
            id: NotSet,
            emergency_ic: Set(emergency_ic),
            created_at: Set(now),
            updated_at: Set(now),
            reported_by: Set(Some(1)),
            notes: Set(emergency_data.notes.clone()), // Clone notes if needed for retries
            resolved_at: Set(now),
            // Handle the modification_attempts field
            modification_attempts: Set(None),
            id_ambulance: NotSet,
            emergency_latitude: Set(emergency_data.emergency_latitude.clone()), // Clone if needed
            emergency_longitude: Set(emergency_data.emergency_longitude.clone()), // Clone if needed
            status: Set(EmergencyStatusEnum::Pending),
            severity: Set(EmergencySeverityEnum::Unknown),
            incident_type: Set(emergency_data.incident_type.clone()), // Clone if needed
            description: Set(emergency_data.description.clone()),     // Clone if needed
        }
    }
}
