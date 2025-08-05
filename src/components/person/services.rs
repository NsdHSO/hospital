use crate::entity::person::{ActiveModel, Column, Entity, Model, PersonRequestBody};
use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::check_if_is_duplicate_key_from_data_base;
use chrono::{Local, NaiveDateTime};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait};
use sea_orm::{QueryFilter, Set};
use uuid::Uuid;

pub struct PersonService {
    conn: DatabaseConnection,
}

impl PersonService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { conn: db.clone() }
    }

    pub async fn find_persons(
        &self,
        field: Option<&str>, // field can now be None
        value: Option<&str>, // value can now be None
        page: Option<u64>,   // page number (1-based)
        limit: Option<u64>,  // number of records per page
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let query_builder = Entity::find();

        let query = match (field, value) {
            (Some(f), Some(v)) => {
                match f {
                    "id" => {
                        // Parse the UUID string and handle potential errors
                        match Uuid::parse_str(v) {
                            Ok(uuid_val) => query_builder.filter(Column::Id.eq(uuid_val)),
                            Err(_) => {
                                return Err(CustomError::new(
                                    HttpCodeW::BadRequest,
                                    format!("Invalid UUID format for id: {v}"),
                                ));
                            }
                        }
                    }
                    "first_name" => query_builder.filter(Column::FirstName.like(format!("%{v}%"))),
                    "date_of_birth" => {
                        query_builder.filter(Column::DateOfBirth.like(format!("%{v}%")))
                    }
                    "gender" => query_builder.filter(Column::Gender.like(format!("%{v}%"))),
                    "phone" => query_builder.filter(Column::Phone.like(format!("%{v}%"))),
                    "email" => query_builder.filter(Column::Email.like(format!("%{v}%"))),
                    "address" => query_builder.filter(Column::Address.like(format!("%{v}%"))),
                    "nationality" => {
                        query_builder.filter(Column::Nationality.like(format!("%{v}%")))
                    }
                    "marital_status" => {
                        query_builder.filter(Column::MaritalStatus.like(format!("%{v}%")))
                    }
                    _ => {
                        // If field is provided but unsupported
                        return Err(CustomError::new(
                            HttpCodeW::BadRequest,
                            format!("Unsupported field for search: {f}"),
                        ));
                    }
                }
            }
            // If field is None or value is None, return all persons
            _ => {
                println!(
                    "No specific field or value provided, returning all persons with pagination."
                );
                query_builder
            }
        };

        // Default pagination values
        let page_num = page.unwrap_or(1);
        let per_page = limit.unwrap_or(10);

        // Convert to 0-based indexing for SeaORM paginator
        let page_index = page_num.saturating_sub(1);

        // Create paginator
        let paginator = query.paginate(&self.conn, per_page);

        // Get pagination metadata
        let total_items = paginator.num_items().await.map_err(|e| {
            CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error getting total items: {e}"),
            )
        })?;

        let total_pages = paginator.num_pages().await.map_err(|e| {
            CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error getting total pages: {e}"),
            )
        })?;

        // Fetch the records for the current page
        let records = paginator
            .fetch_page(page_index) // Page is 0-indexed in SeaORM
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Database error fetching page: {e}"),
                )
            })?;

        // Create pagination info
        let pagination = PaginationInfo {
            current_page: page_num as i64,
            page_size: per_page as i64,
            total_items: total_items as i64,
            total_pages: total_pages as i64,
            has_next_page: page_num < total_pages,
            has_previous_page: page_num > 1,
        };

        Ok(PaginatedResponse {
            data: records,
            pagination,
        })
    }
    pub async fn create(
        &self,
        person_data: Option<PersonRequestBody>,
    ) -> Result<Model, CustomError> {
        // Check if patient_data exists
        let payload = person_data;

        let now = Local::now().naive_utc();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Failed to generate a unique Person IC after multiple attempts.".to_string(),
                ));
            }

            let active_model = Self::generate_model(payload.clone(), now);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;

            if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                return value;
            }
        }
    }

    fn generate_model(p0: Option<PersonRequestBody>, p1: NaiveDateTime) -> ActiveModel {
        let payload = p0.unwrap_or_default();
        ActiveModel {
            id: Set(Uuid::new_v4()),
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
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            address: if let Some(value) = payload.address {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            email: if let Some(value) = payload.email {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },

            phone: if let Some(value) = payload.phone {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            gender: if let Some(value) = payload.gender {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            photo_url: if let Some(value) = payload.photo_url {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            marital_status: if let Some(value) = payload.marital_status {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            nationality: if let Some(value) = payload.nationality {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            created_at: Set(p1),
            updated_at: Set(p1),
        }
    }
}
