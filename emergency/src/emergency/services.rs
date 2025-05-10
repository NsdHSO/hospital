use crate::db::config::connection;
use crate::entity::emergency;
use crate::entity::emergency::{EmergencyRequestBody, Model};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use chrono::Utc;
use nanoid::nanoid;
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, Set};
use uuid::Uuid;

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
        let emergency_ic = nanoid!();

        // Generate a new UUID for the id
        let id = Uuid::new_v4();

        // Get current timestamps
        let now = Utc::now().naive_utc();

        let active_model = emergency::ActiveModel {
            id: Set(id),
            emergency_ic: Set(emergency_ic),
            created_at: Set(now),
            updated_at: Set(now),
            reported_by: Set(emergency_data.reported_by),
            notes: Set(emergency_data.notes),
            resolved_at: Set(emergency_data.resolved_at),
            // Handle the modification_attempts field
            modification_attempts: Set(emergency_data.modification_attempts.map(|p| SeaOrmJson(p))),
            id_ambulance: Set(emergency_data.id_ambulance),
            emergency_latitude: Set(emergency_data.emergency_latitude),
            emergency_longitude: Set(emergency_data.emergency_longitude),
            status: Set(emergency_data.status),
            severity: Set(emergency_data.severity),
            incident_type: Set(emergency_data.incident_type),
            description: Set(emergency_data.description),
        };

        // Insert the record into the database
        let result = active_model
            .insert(&self.conn)
            .await
            .map_err(|e| CustomError::from(e))?; // Use the From<DbErr> implementation

        Ok(result)
    }
}
