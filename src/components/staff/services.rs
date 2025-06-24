use crate::components::person::PersonService;
use crate::entity::person;
use crate::entity::person::PersonRequestBody;
use crate::entity::sea_orm_active_enums::StaffRoleEnum;
use crate::entity::staff::{ActiveModel, Column, Entity, Model, StaffRequestBody, StaffWithPerson};
use crate::error_handler::CustomError;
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic};
use chrono::{Local, NaiveDateTime};
use sea_orm::ActiveModelTrait;
use sea_orm::{ColumnTrait, QueryFilter, Set};
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;
use crate::http_response::HttpCodeW;

pub struct StaffService {
    conn: DatabaseConnection,
    person_service: PersonService,
}

impl StaffService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            conn: db.clone(),
            person_service: PersonService::new(db),
        }
    }
    pub async fn create(
        &self,
        staff_data: Option<StaffRequestBody>,
    ) -> Result<StaffWithPerson, CustomError> {
        let payload = match staff_data.clone() {
            Some(data) => data,
            None => return Err(CustomError::new(HttpCodeW::BadRequest, "Missing patient data".to_string())),
        };

        let now = Local::now().naive_utc();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;
        let staff_body: StaffRequestBody = staff_data.unwrap();
        let person = self
            .person_service
            .create(Option::from(PersonRequestBody {
                first_name: staff_body.first_name,
                last_name: staff_body.last_name,
                date_of_birth: staff_body.date_of_birth,
                gender: staff_body.gender,
                phone: staff_body.phone,
                email: staff_body.email,
                address: staff_body.address,
                nationality: Some(String::from("ROM")),
                marital_status: None,
                photo_url: None,
            }))
            .await
            .or(Err(CustomError::new(
                HttpCodeW::InternalServerError,
                "Internal server error".to_string(),
            )))?;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Failed to generate a unique staff IC after multiple attempts.".to_string(),
                ));
            }

            let active_model = Self::generate_model(Some(payload.clone()), now, person.clone().id);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;
            check_if_is_duplicate_key_from_data_base(&mut attempts, result);
            let (staff, person) = Entity::find_by_id(person.id)
                .find_also_related(person::Entity)
                .one(&self.conn)
                .await?
                .ok_or_else(|| CustomError::new(HttpCodeW::BadRequest, "Staff not found".to_string()))?;
            return Ok(StaffWithPerson {
                staff,
                person: person.unwrap(),
            });
        }
    }

    /// Find a patient by a given column and value (generic for ic or name)
    #[allow(dead_code)]
    pub async fn find_by_field(
        &self,
        field: &str,
        value: &str,
    ) -> Result<Option<Model>, CustomError> {
        let query = match field {
            "staff_ic" => Entity::find().filter(Column::StaffIc.like(value)),
            _ => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Unsupported field: {}", field),
                ));
            }
        };
        let staff = query
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("Database error: {}", e)))?;
        if let Some(patient_model) = staff {
            Ok(Some(patient_model))
        } else {
            Err(CustomError::new(
                HttpCodeW::NotFound,
                format!("Patient not found for {} = '{}'", field, value),
            ))
        }
    }

    fn generate_model(
        p0: Option<StaffRequestBody>,
        p1: NaiveDateTime,
        id_person: Uuid,
    ) -> ActiveModel {
        let payload = p0.unwrap();
        ActiveModel {
            id: Set(id_person),
            hospital_id: Set(Uuid::new_v4()),
            department_id: Set(Uuid::new_v4()),
            specialization: if let Some(value) = payload.specialization {
                Set(Option::from(value))
            } else {
                Set(None)
            },
            role: if let Some(value) = payload.role {
                Set(value)
            } else {
                Set(StaffRoleEnum::Technician)
            },
            staff_ic: Set(Option::from(generate_ic().to_string())),
            created_at: Set(p1),
            updated_at: Set(p1),
        }
    }
}
