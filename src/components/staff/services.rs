use crate::components::department::DepartmentService;
use crate::components::hospital::HospitalService;
use crate::components::person::PersonService;
use crate::entity::person;
use crate::entity::person::PersonRequestBody;
use crate::entity::sea_orm_active_enums::StaffRoleEnum;
use crate::entity::staff::{
    ActiveModel, Column, Entity, Model, Relation, StaffRequestBody, StaffWithPerson,
};
use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic};
use chrono::{Local, NaiveDateTime};
use sea_orm::{ActiveModelTrait, Condition, QuerySelect, RelationTrait};
use sea_orm::{ColumnTrait, QueryFilter, Set};
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct StaffService {
    conn: DatabaseConnection,
    person_service: PersonService,
    hospital_service: HospitalService,
    department_service: DepartmentService,
}

impl StaffService {
    pub(crate) async fn find_staff_with_person(
        &self,
        field: Option<&str>,
        value: Option<&str>,
        hospital_id: Option<&str>,
        page: Option<u64>,
        limit: Option<u64>,
    ) -> Result<PaginatedResponse<Vec<(Model, Option<person::Model>)>>, CustomError> {
        let mut query_builder = Entity::find().find_with_related(person::Entity);

        // Build the query with a Condition
        let mut condition = Condition::all();

        if let (Some(field), Some(value)) = (field, value) {
            match field {
                "hospital_id" => match Uuid::parse_str(value) {
                    Ok(uuid_val) => {
                        condition = condition.add(Column::HospitalId.eq(uuid_val));
                    }
                    Err(_) => {
                        return Err(CustomError::new(
                            HttpCodeW::BadRequest,
                            format!("Invalid UUID format for hospital_id: {value}"),
                        ));
                    }
                },
                "role" => {
                    condition = condition.add(Column::Role.eq(value));
                }
                "name" => {
                    // If the field is name, we must apply the filter to the related person entity
                    // This requires a manual join filter
                    query_builder = query_builder.filter(
                        Condition::all()
                            .add(person::Column::FirstName.like(&format!("%{}%", value))),
                    );
                }
                _ => {
                    return Err(CustomError::new(
                        HttpCodeW::BadRequest,
                        format!("Unsupported field: {field}"),
                    ));
                }
            }
        }

        if let Some(hospital_id_str) = hospital_id {
            let hospital_id_uuid = match Uuid::parse_str(hospital_id_str) {
                Ok(uuid) => uuid,
                Err(e) => {
                    return Err(CustomError::new(
                        HttpCodeW::BadRequest,
                        format!("Invalid UUID format for hospital_id: {}", e),
                    ));
                }
            };
            condition = condition.add(Column::HospitalId.eq(hospital_id_uuid));
        }

        // Apply the constructed condition
        query_builder = query_builder.filter(condition);

        // Default pagination values
        let page_num = page.unwrap_or(1);
        let per_page = limit.unwrap_or(10);
        let page_index = page_num.saturating_sub(1);
        let offset = page_index * per_page;

        // First query: Get the total count
        let total_items = query_builder
            .clone()
            .all(&self.conn)
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Database error getting total items: {e}"),
                )
            })?
            .len() as u64; // Count the number of items after fetching them all

        let tuples: Vec<(Model, Vec<person::Model>)> = query_builder
            .offset(offset)
            .limit(per_page)
            .all(&self.conn)
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Database error fetching page: {e}"),
                )
            })?;

        // Map the returned Vec<(Staff, Vec<Person>)> to Vec<(Staff, Option<Person>)>
        let records: Vec<(Model, Option<person::Model>)> = tuples
            .into_iter()
            .map(|(staff, persons)| {
                // Take the first element of the inner vector, or None if it's empty
                (staff, persons.into_iter().next())
            })
            .collect();

        // Calculate total pages
        let total_pages = (total_items as f64 / per_page as f64).ceil() as u64;

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
}

impl StaffService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            conn: db.clone(),
            person_service: PersonService::new(db),
            hospital_service: HospitalService::new(db),
            department_service: DepartmentService::new(db),
        }
    }
    pub async fn create(
        &self,
        staff_data: Option<StaffRequestBody>,
    ) -> Result<StaffWithPerson, CustomError> {
        let payload = match staff_data.clone() {
            Some(data) => data,
            None => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    "Missing patient data".to_string(),
                ));
            }
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
                "Internal When created Person in staff Service server error".to_string(),
            )))?;
        let hospital_id: Uuid = self
            .hospital_service
            .find_by_field("name", &staff_body.hospital_name.unwrap())
            .await?
            .unwrap()
            .id;
        let department_id: Uuid = self
            .department_service
            .find_by_field("name", &staff_body.department_name.unwrap())
            .await?
            .unwrap()
            .id;
        // This is intentionally a loop that runs at most once
        // It's designed to use the check_if_is_duplicate_key_from_data_base function consistently
        {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Failed to generate a unique staff IC after multiple attempts.".to_string(),
                ));
            }

            let active_model = Self::generate_model(
                Some(payload.clone()),
                now,
                person.clone().id,
                department_id,
                hospital_id,
            );

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;
            check_if_is_duplicate_key_from_data_base(&mut attempts, result);
            let (staff, person) = Entity::find_by_id(person.id)
                .find_also_related(person::Entity)
                .one(&self.conn)
                .await?
                .ok_or_else(|| {
                    CustomError::new(HttpCodeW::BadRequest, "Staff not found".to_string())
                })?;
            Ok(StaffWithPerson {
                staff,
                person: person.unwrap(),
            })
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
            "id" => match Uuid::parse_str(value) {
                Ok(uuid_val) => Entity::find().filter(Column::Id.eq(uuid_val)),
                Err(_) => {
                    return Err(CustomError::new(
                        HttpCodeW::BadRequest,
                        format!("Invalid UUID format for id: {value}"),
                    ));
                }
            },
            "staff_ic" => Entity::find().filter(Column::StaffIc.like(value)),
            "name" => Entity::find()
                .join(sea_orm::JoinType::InnerJoin, Relation::Person.def())
                .filter(person::Column::FirstName.like(value)),
            _ => {
                return Err(CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Unsupported field: {field}"),
                ));
            }
        };
        let staff = query.one(&self.conn).await.map_err(|e| {
            println!("Error: {}", e);
            CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error: {e}"),
            )
        })?;
        if let Some(staff_model) = staff {
            Ok(Some(staff_model))
        } else {
            Err(CustomError::new(
                HttpCodeW::NotFound,
                format!("Staff not found for {field} = '{value}'"),
            ))
        }
    }

    fn generate_model(
        p0: Option<StaffRequestBody>,
        p1: NaiveDateTime,
        id_person: Uuid,
        department_id: Uuid,
        hospital_id: Uuid,
    ) -> ActiveModel {
        let payload = p0.unwrap();
        ActiveModel {
            id: Set(id_person),
            hospital_id: Set(hospital_id),
            department_id: Set(department_id),
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
