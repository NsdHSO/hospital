use crate::entity::card::{ActiveModel, CardPayload, Model};
use crate::entity::{card, dashboard};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::utils::generate_ic;
use percent_encoding::percent_decode_str;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, PaginatorTrait};
use sea_orm::{ColumnTrait, DbErr, Set};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct CardService {
    conn: DatabaseConnection,
}

impl CardService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        CardService { conn: conn.clone() }
    }

    pub async fn create_card(self, payload: Option<CardPayload>) -> Result<Model, CustomError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique  IC after multiple attempts.".to_string(),
                ));
            }
            let mut active_model = generate_payload_to_create_card(payload.clone());
            let dashboard =
                payload
                    .as_ref()
                    .and_then(|p| p.dashboard_id)
                    .ok_or(CustomError::new(
                        500,
                        "dashboard_id is required".to_string(),
                    ))?;

            let dashboard_entity = dashboard::Entity::find()
                .filter(dashboard::Column::Id.eq(dashboard))
                .one(&self.conn)
                .await;
            if let Ok(Some(card_model)) = &dashboard_entity {
                active_model.dashboard_id = Set(Some(card_model.id.clone()));
            } else {
                return Err(CustomError::new(500, "Dashboard not found".to_string()));
            }

            let result = active_model.insert(&self.conn).await;
            match result {
                Ok(model) => return Ok(model),
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
                    Err(_) => encoded_name.to_string(),
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

fn generate_payload_to_create_card(payload: Option<CardPayload>) -> ActiveModel {
    let payload = payload.unwrap_or_default();

    ActiveModel {
        created_at: Default::default(),
        updated_at: Default::default(),
        id: Default::default(),
        title: if let Some(value) = payload.title {
            Set(value)
        } else {
            Set(Default::default())
        },
        content: if let Some(value) = payload.content {
            Set(value)
        } else {
            Set(Default::default())
        },
        card_ic: Set(generate_ic()),
        icon: if let Some(value) = payload.icon {
            Set(Some(value))
        } else {
            Set(Default::default())
        },
        position: if let Some(value) = payload.position {
            Set(Some(value))
        } else {
            Set(Default::default())
        },
        data_config: if let Some(value) = payload.data_config {
            Set(Some(value))
        } else {
            Set(Default::default())
        },
        dashboard_id: if let Some(value) = payload.dashboard_id {
            Set(Some(value))
        } else {
            Set(Default::default())
        },
        card_type: if let Some(value) = payload.card_type {
            Set(Some(value))
        } else {
            Set(Default::default())
        },
        size: if let Some(value) = payload.size {
            Set(Some(value))
        } else {
            Set(Default::default())
        },
    }
}
