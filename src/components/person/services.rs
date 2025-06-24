use crate::entity::person::{ActiveModel, Column, Entity, Model, PersonRequestBody};
use crate::error_handler::CustomError;
use crate::utils::helpers::check_if_is_duplicate_key_from_data_base;
use chrono::{Local, NaiveDateTime};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, Set};
use uuid::Uuid;
use crate::http_response::HttpCodeW;

pub struct PersonService {
    conn: DatabaseConnection,
}

impl PersonService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { conn: db.clone() }
    }

    pub async fn find_by_field(
        &self,
        field: &str,
        value: &str,
    ) -> Result<Option<Model>, CustomError> {
        let query = match field {
            "id" => Entity::find().filter(Column::Id.like(value)),
            "first_name" => Entity::find().filter(Column::FirstName.like(value)),
            "date_of_birth" => Entity::find().filter(Column::DateOfBirth.like(value)),
            "gender" => Entity::find().filter(Column::Gender.like(value)),
            "phone" => Entity::find().filter(Column::Phone.like(value)),
            "email" => Entity::find().filter(Column::Email.like(value)),
            "address" => Entity::find().filter(Column::Address.like(value)),
            "nationality" => Entity::find().filter(Column::Nationality.like(value)),
            "marital_status" => Entity::find().filter(Column::MaritalStatus.like(value)),
            _ => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Unsupported field: {}", field),
                ));
            }
        };
        let patient = query
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("Database error: {}", e)))?;
        if let Some(person_model) = patient {
            Ok(Some(person_model))
        } else {
            Err(CustomError::new(
                HttpCodeW::NotFound,
                format!("Person not found for {} = '{}'", field, value),
            ))
        }
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
