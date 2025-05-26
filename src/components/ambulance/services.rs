use crate::entity::ambulance::AmbulancePayload;
use crate::entity::ambulance::Model;
use crate::entity::sea_orm_active_enums::{
    AmbulanceCarDetailsMakeEnum, AmbulanceCarDetailsModelEnum, AmbulanceStatusEnum,
    AmbulanceTypeEnum,
};
use crate::entity::{ambulance, hospital};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::generate_ic;
use hospital::Column::Name as HospitalName;
use hospital::Entity as HospitalEntity;
use sea_orm::prelude::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{NotSet, QueryFilter, Set};

pub struct AmbulanceService {
    conn: DatabaseConnection,
}

impl AmbulanceService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        AmbulanceService { conn: conn.clone() }
    }

    pub async fn create_ambulance(
        self,
        payload: Option<AmbulancePayload>,
    ) -> Result<Model, CustomError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }
            let mut active_model = generate_payload_to_create_ambulance(payload.clone());
            let hospital_name = payload
                .as_ref()
                .and_then(|p| p.hospital_name.as_deref())
                .ok_or(CustomError::new(
                    500,
                    "hospital_name is required".to_string(),
                ))?;

            let hospital = HospitalEntity::find()
                .filter(HospitalName.eq(hospital_name))
                .one(&self.conn)
                .await;
            if let Ok(Some(hospital_model)) = &hospital {
                active_model.hospital_id = Set(hospital_model.id.clone().to_string());
            } else {
                return Err(CustomError::new(500, "hospital not found".to_string()));
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
    pub async fn find_by_ic(
        &self,
        ambulance_ic: i32,
    ) -> Result<Option<ambulance::Model>, CustomError> {
        ambulance::Entity::find()
            .filter(ambulance::Column::AmbulanceIc.eq(ambulance_ic))
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))
    }

    pub async fn find_all(
        &self,         // Changed to &self as we're not modifying the service state
        page: u64,     // Use u64 for pagination
        per_page: u64, // Use u64 for pagination
    ) -> Result<PaginatedResponse<Vec<ambulance::Model>>, CustomError> {
        let paginator = ambulance::Entity::find().paginate(&self.conn, per_page);

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

pub fn generate_payload_to_create_ambulance(
    payload: Option<AmbulancePayload>,
) -> ambulance::ActiveModel {
    let now = chrono::Utc::now().naive_utc();
    let payload = payload.unwrap_or_default();

    ambulance::ActiveModel {
        // Always set these system fields
        id: NotSet,
        created_at: Set(now),
        updated_at: Set(now),

        // Fields from payload or default values
        ambulance_ic: Set(generate_ic()),

        vehicle_number: if let Some(val) = payload.vehicleNumber {
            Set(val)
        } else {
            NotSet
        },

        make: Set(payload.make),

        year: Set(payload.year),

        capacity: Set(payload.capacity),

        mission: Set(payload.mission),

        passengers: Set(payload.passengers),

        driver_name: Set(payload.driver_name),

        driver_license: Set(payload.driver_license),

        last_service_date: Set(payload.last_service_date),

        next_service_date: Set(payload.next_service_date),

        mileage: Set(payload.mileage),

        fuel_type: Set(payload.fuel_type),

        registration_number: Set(payload.registration_number),

        insurance_provider: Set(payload.insurance_provider),

        insurance_expiry_date: Set(payload.insurance_expiry_date),

        notes: Set(payload.notes),

        car_details_year: if let Some(val) = payload.car_details_year {
            Set(val)
        } else {
            Set(2023) // Default value
        },

        car_details_color: if let Some(val) = payload.car_details_color {
            Set(val)
        } else {
            Set("White".to_string()) // Default value
        },

        car_details_isambulance: if let Some(val) = payload.car_details_isambulance {
            Set(val)
        } else {
            Set(true) // Default value
        },

        car_details_licenseplate: Set(payload.car_details_licenseplate),

        car_details_mileage: Set(payload.car_details_mileage),

        location_latitude: if let Some(val) = payload.location_latitude {
            Set(val)
        } else {
            Set(Decimal::new(0, 6)) // Default value
        },

        location_longitude: if let Some(val) = payload.location_longitude {
            Set(val)
        } else {
            Set(Decimal::new(0, 6))
        },

        r#type: if let Some(val) = payload.r#type {
            Set(val)
        } else {
            Set(AmbulanceTypeEnum::BasicLifeSupport)
        },

        status: if let Some(val) = payload.status {
            Set(val)
        } else {
            Set(AmbulanceStatusEnum::Available)
        },

        car_details_make: if let Some(val) = payload.car_details_make {
            Set(val)
        } else {
            Set(AmbulanceCarDetailsMakeEnum::Toyota)
        },

        car_details_model: if let Some(val) = payload.car_details_model {
            Set(val)
        } else {
            Set(AmbulanceCarDetailsModelEnum::Nv350)
        },
        hospital_id: Default::default(),
    }
}
