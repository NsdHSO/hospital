use crate::components::hospital::HospitalService;
use crate::entity::department::{ActiveModel, Column, DepartmentRequestBody, Entity, Model};
use crate::entity::sea_orm_active_enums::DepartmentNameEnum;
use crate::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic, now_time};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub struct DepartmentService {
    conn: DatabaseConnection,
    hospital_service: HospitalService,
}

impl DepartmentService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            conn: conn.clone(),
            hospital_service: HospitalService::new(conn),
        }
    }

    pub async fn find_by_field(
        &self,
        field: &str,
        value: &str,
    ) -> Result<Option<Model>, CustomError> {
        let query = match field {
            "name" => Entity::find().filter(Column::Name.eq(value)),
            _ => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Unsupported field: {field}"),
                ));
            }
        };
        let department = query.one(&self.conn).await.map_err(|e| {
            CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error: {e}"),
            )
        })?;
        if let Some(department_model) = department {
            Ok(Some(department_model))
        } else {
            Err(CustomError::new(
                HttpCodeW::NotFound,
                format!("Department not found for {field} = '{value}'")
            ))
        }
    }
    pub async fn create(
        &self,
        department_data: Option<DepartmentRequestBody>,
    ) -> Result<Model, CustomError> {
        let payload = match department_data {
            None => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Missing department data".to_string(),
                ));
            }
            Some(value) => value,
        };
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;
        let hospital_id = &self
            .hospital_service
            .find_by_field("name", &payload.hospital_name)
            .await?
            .unwrap()
            .id;
        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Failed to generate a unique department IC after multiple attempts."
                        .to_string(),
                ));
            }

            let active_model = generate_payload(&payload, *hospital_id);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;

            if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                return value;
            }
        }
    }
}
fn generate_payload(payload: &DepartmentRequestBody, hospital_id: Uuid) -> ActiveModel {
    ActiveModel {
        created_at: Set(now_time()),
        updated_at: Set(now_time()),
        id: Set(Uuid::new_v4()),
        hospital_id: Set(hospital_id),
        floor: Default::default(),
        head_of_department: if let Some(value) = payload.head_of_department.clone() {
            Set(Option::from(value))
        } else {
            Set(None)
        },
        phone: if let Some(value) = payload.phone.clone() {
            Set(Option::from(value))
        } else {
            Set(None)
        },
        description: if let Some(value) = payload.description.clone() {
            Set(Option::from(value))
        } else {
            Set(None)
        },
        capacity: if let Some(value) = &payload.capacity {
            Set(Option::from(*value))
        } else {
            Set(None)
        },
        name: if let Some(value) = &payload.name {
            Set(value.clone())
        } else {
            Set(DepartmentNameEnum::Pediatrics)
        },
        department_ic: Set(Option::from(generate_ic().to_string())),
    }
}
