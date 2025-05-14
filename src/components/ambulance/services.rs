use crate::entity::ambulance;
use crate::entity::ambulance::AmbulancePayload;
use crate::entity::emergency::Model;
use crate::entity::sea_orm_active_enums::{
    AmbulanceCarDetailsMakeEnum, AmbulanceCarDetailsModelEnum, AmbulanceStatusEnum,
    AmbulanceTypeEnum,
};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::utils::generate_ic;
use sea_orm::prelude::Decimal;
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{NotSet, QueryFilter, Set};

pub struct AmbulanceService {
    conn: DatabaseConnection,
}

impl AmbulanceService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        AmbulanceService { conn: conn.clone() }
    }

    pub async fn create_ambulance(payload: Option<AmbulancePayload>) -> Result<Model, CustomError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }
            let payload = generate_payload_to_create_ambulance(payload.clone());
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

        vehicle_number: if let Some(val) = payload.vehicle_number {
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
            Set(Decimal::new(0, 6)) // Default value
        },

        r#type: if let Some(val) = payload.r#type {
            Set(val)
        } else {
            Set(AmbulanceTypeEnum::BasicLifeSupport) // Default value
        },

        status: if let Some(val) = payload.status {
            Set(val)
        } else {
            Set(AmbulanceStatusEnum::Available) // Default value
        },

        car_details_make: if let Some(val) = payload.car_details_make {
            Set(val)
        } else {
            Set(AmbulanceCarDetailsMakeEnum::Toyota) // Default value
        },

        car_details_model: if let Some(val) = payload.car_details_model {
            Set(val)
        } else {
            Set(AmbulanceCarDetailsModelEnum::HiAce) // Default value
        },

        // Set hospital_id with a sensible default since you mentioned excluding it from the payload
        // but it's likely required
        hospital_id: Set("DEFAULT_HOSPITAL".to_string()),
    }
}
