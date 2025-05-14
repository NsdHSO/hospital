use percent_encoding::percent_decode_str;
use crate::entity::{card, dashboard};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use sea_orm::ColumnTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct CardService {
    conn: DatabaseConnection,
}

impl CardService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        CardService { conn: conn.clone() }
    }

    pub async fn find_all(
        &self,     // Changed to &self as we're not modifying the service state
        page: u64, // Use u64 for pagination
        per_page: u64,
        filter: Option<String>,
    ) -> Result<PaginatedResponse<Vec<card::Model>>, CustomError> {
        let mut query = card::Entity::find();

        // Inside your function
        if let Some(filter_str) = filter {
            if filter_str.starts_with("dashboard=") {
                // Extract the dashboard name portion after "dashboard="
                let encoded_name = filter_str.strip_prefix("dashboard=").unwrap_or("");

                // URL decode the dashboard name
                let dashboard_name = match percent_decode_str(encoded_name).decode_utf8() {
                    Ok(name) => name.to_string(),
                    Err(_) => encoded_name.to_string()
                };

                println!("Dashboard name after decoding: '{}'", dashboard_name);

                // First, find the dashboard by name
                let dashboard = dashboard::Entity::find()
                    .filter(dashboard::Column::Name.eq(&dashboard_name))
                    .one(&self.conn)
                    .await?;
                println!("Dashboard: {:?}", dashboard);

                if let Some(dashboard) = dashboard {
                    // Then filter cards by that dashboard's ID
                    query = query.filter(card::Column::DashboardId.eq(dashboard.id));
                }
            }
        }


        let paginator = query.paginate(&self.conn, per_page);
        let total_items = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        let records = paginator
            .fetch_page(page - 1) // Page is 0-indexed in SeaORM
            .await?;

        let pagination = PaginationInfo {
            current_page: page as i64,
            page_size: per_page as i64,
            total_items: total_items as i64,
            total_pages: total_pages as i64,
            has_next_page: page < total_pages,
            has_previous_page: page > 1,
        };

        Ok(PaginatedResponse {
            data: records,
            pagination,
        })
    }
}
