use crate::db::config::connection;
use crate::entity::emergency;
use crate::entity::emergency::Model;
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};

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
}
