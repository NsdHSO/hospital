use crate::entity::hospital::{ActiveModel, Column, Entity, HospitalRequestBody, Model};
use chrono::NaiveDateTime;

use crate::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic, now_time};
use sea_orm::{ActiveModelTrait, PaginatorTrait, Set};
use sea_orm::{ColumnTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct HospitalService {
    conn: DatabaseConnection,
}

impl HospitalService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        HospitalService { conn: conn.clone() }
    }
    pub async fn create_emergency(
        &self,
        emergency_data: Option<HospitalRequestBody>,
    ) -> Result<Model, CustomError> {
        // Generate unique emergency_ic (using nanoid for a short, unique string)
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

            let active_model = Self::generate_model(emergency_data.clone(), now);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;

            if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                return value;
            }
        }
    }
    pub async fn find_by_ic(&self, hospital_name: String) -> Result<Option<Model>, CustomError> {
        let hospital = Entity::find()
            .filter(Column::Name.like(&hospital_name))
            .one(&self.conn)
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Database error: {e}"),
                )
            });

        match hospital {
            Ok(Some(hospital_model)) => Ok(Option::from(hospital_model)),
            Ok(None) => Err(CustomError::new(
                HttpCodeW::NotFound,
                format!("Hospital with name '{hospital_name}' not found"),
            )),
            Err(e) => Err(CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error: {e}"),
            )),
        }
    }

    pub async fn find_all(
        &self,         // Changed to &self as we're not modifying the service state
        page: u64,     // Use u64 for pagination
        per_page: u64, // Use u64 for pagination
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let paginator = Entity::find().paginate(&self.conn, per_page);

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
    pub async fn find_by_field(
        &self,
        field: &str,
        value: &str,
    ) -> Result<Option<Model>, CustomError> {
        let query = match field {
            "name" => Entity::find().filter(Column::Name.like(value)),
            _ => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Unsupported field: {field}"),
                ));
            }
        };
        let hospital = query.one(&self.conn).await.map_err(|e| {
            CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error: {e}"),
            )
        })?;
        if let Some(hospital_model) = hospital {
            Ok(Some(hospital_model))
        } else {
            Err(CustomError::new(
                HttpCodeW::NotFound,
                format!("Hospital not found for {field} = '{value}'"),
            ))
        }
    }
    fn generate_model(p0: Option<HospitalRequestBody>, p1: NaiveDateTime) -> ActiveModel {
        let payload = p0.unwrap_or_default();
        ActiveModel {
            hospital_ic: Set(generate_ic().to_string()),
            created_at: Set(p1),
            updated_at: Set(p1),
            id: Default::default(),
            name: if let Some(value) = payload.name {
                Set(value)
            } else {
                Set(Default::default())
            },
            address: if let Some(value) = payload.address {
                Set(value)
            } else {
                Set(Default::default())
            },
            phone: Default::default(),
            website: if let Some(value) = payload.website {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            description: if let Some(value) = payload.description {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            capacity: if let Some(value) = payload.capacity {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            established: Default::default(),
            ceo: if let Some(value) = payload.ceo {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            trauma_level: if let Some(value) = payload.trauma_level {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            revenue: Default::default(),
            non_profit: if let Some(value) = payload.non_profit {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            license_number: if let Some(value) = payload.license_number {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            accreditation: if let Some(value) = payload.accreditation {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            patient_satisfaction_rating: Default::default(),
            average_stay_length: Default::default(),
            annual_budget: Default::default(),
            owner: if let Some(value) = payload.owner {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            latitude: if let Some(value) = payload.latitude {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            longitude: if let Some(value) = payload.longitude {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
        }
    }
}
