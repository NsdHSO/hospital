use crate::entity::dashboard;
use crate::entity::dashboard::{ActiveModel, Model, PayloadBodyDashboard};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::check_if_is_duplicate_key_from_data_base;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use sea_orm::{PaginatorTrait, Set};
use uuid::Uuid;

pub struct DashboardService {
    conn: DatabaseConnection,
}

impl DashboardService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        DashboardService { conn: conn.clone() }
    }

    pub async fn find_all(
        &self,         // Changed to &self as we're not modifying the service state
        page: u64,     // Use u64 for pagination
        per_page: u64, // Use u64 for pagination
    ) -> Result<PaginatedResponse<Vec<dashboard::Model>>, CustomError> {
        let paginator = dashboard::Entity::find().paginate(&self.conn, per_page);

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
    pub async fn create(&self, dashboard_data: PayloadBodyDashboard) -> Result<Model, CustomError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }

            let active_model = Self::generate_payload(dashboard_data.clone());
            let result = active_model.insert(&self.conn).await;
            if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                return value;
            }
        }
    }

    fn generate_payload(payload_body_dashboard: PayloadBodyDashboard) -> ActiveModel {
        ActiveModel {
            created_at: Default::default(), // These will be set to current timestamp by Sea ORM
            updated_at: Default::default(), // These will be set to current timestamp by Sea ORM
            id: Set(Uuid::new_v4()),        // Generate a new UUID
            name: Set(payload_body_dashboard.name),
            description: Set(payload_body_dashboard.description),
            is_active: Set(payload_body_dashboard.is_active),
            owner_id: Set(payload_body_dashboard.owner_id),
            layout_config: Set(payload_body_dashboard.layout_config),
        }
    }
}
