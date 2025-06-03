use crate::entity::ambulance::Column::Id;
use crate::entity::ambulance::Model;
use crate::entity::ambulance::{AmbulancePayload, Column, Entity};
use crate::entity::sea_orm_active_enums::{
    AmbulanceCarDetailsMakeEnum, AmbulanceCarDetailsModelEnum, AmbulanceStatusEnum,
    AmbulanceTypeEnum,
};

use crate::entity::{ambulance, hospital};
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic};
use chrono::Utc;

use crate::components::emergency::EmergencyService;
use hospital::Column::Name as HospitalName;
use hospital::Entity as HospitalEntity;
use percent_encoding::percent_decode_str;
use sea_orm::prelude::Decimal;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{NotSet, QueryFilter, Set};

pub struct AmbulanceService {
    conn: DatabaseConnection,
    emergency_service: EmergencyService,
}

impl AmbulanceService {
    pub(crate) async fn update_ambulance(
        &self,
        uuid: Uuid,
        payload: AmbulancePayload,
    ) -> Result<Model, CustomError> {
        let now = Utc::now().naive_utc();
        let query = Entity::find().filter(Id.eq(uuid)).one(&self.conn).await?;

        let model = match query {
            Some(model) => model,
            None => {
                return Err(CustomError::new(404, "Ambulance not found".to_string()));
            }
        };

        let mut active_model: ambulance::ActiveModel = model.into();

        match payload.status {
            Some(AmbulanceStatusEnum::Available) => {
                active_model.status = Set(AmbulanceStatusEnum::Available);
            }
            Some(AmbulanceStatusEnum::Dispatched) => {
                active_model.status = Set(AmbulanceStatusEnum::Dispatched);
            }
            Some(AmbulanceStatusEnum::EnRouteToScene) => {
                if let Some(json) = active_model.passengers.as_ref() {
                    if let Some(array) = json.as_array() {
                        if array.len() > 0 {
                            return Err(CustomError::new(400, "Cannot set status to EnRouteToScene when passengers are present".to_string()));
                        }
                    }
                }
                
                active_model.status = Set(AmbulanceStatusEnum::EnRouteToScene);
            }
            Some(AmbulanceStatusEnum::TransportingPatient) => {
                active_model.status = Set(AmbulanceStatusEnum::TransportingPatient);
                let passengers_json = self
                    .emergency_service
                    .get_passengers_json_for_ambulance(uuid);
                active_model.passengers = Set(passengers_json.await.map_err(|e| {
                    CustomError::new(500, format!("Error fetching passengers: {}", e))
                })?);
            }
            Some(status) => {
                active_model.status = Set(status);
            }
            None => {}
        }

        active_model.updated_at = Set(now);
        // Save changes
        let updated = active_model
            .update(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))?;

        Ok(updated)
    }
}

impl AmbulanceService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        AmbulanceService {
            conn: conn.clone(),
            emergency_service: EmergencyService::new(conn),
        }
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
            if let Some(value) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                return value;
            }
        }
    }

    pub async fn find_all(
        &self,     // Changed to &self as we're not modifying the service state
        page: u64, // Use u64 for pagination
        per_page: u64,
        filter: Option<String>,
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let mut query = Entity::find();
        if let Some(filter_str) = filter {
            match filter_str.split_once('=') {
                Some(("ic", encoded_name)) => {
                    let ambulance_ic = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|ic| ic.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
                    query = query.filter(Column::AmbulanceIc.like(ambulance_ic));
                }
                Some(("id", encoded_name)) => {
                    let ambulance_id = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
                    let ambulance_uuid = Uuid::parse_str(&ambulance_id)
                        .map_err(|_| CustomError::new(400, "Invalid UUID".to_string()))?;
                    query = query.filter(Column::Id.eq(ambulance_uuid));
                }
                _ => {}
            }
        }
        let paginator = query.paginate(&self.conn, per_page);
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
    let car_details = payload.car_details.unwrap_or_default();
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

        car_details_year: if let Some(val) = car_details.year {
            Set(val)
        } else {
            Set(2023) // Default value
        },

        car_details_color: if let Some(val) = car_details.color {
            Set(val)
        } else {
            Set("White".to_string()) // Default value
        },

        car_details_is_ambulance: if let Some(val) = car_details.is_ambulance {
            Set(val)
        } else {
            Set(true) // Default value
        },

        car_details_license_plate: if let Some(val) = car_details.license_plate {
            Set(Option::from(val))
        } else {
            Set(Default::default()) // Default value
        },

        car_details_mileage: if let Some(val) = car_details.mileage {
            Set(Option::from(val))
        } else {
            Set(Default::default()) // Default value
        },

        car_details_make: if let Some(val) = car_details.make {
            Set(val)
        } else {
            Set(AmbulanceCarDetailsMakeEnum::Toyota)
        },

        car_details_model: if let Some(val) = car_details.model {
            Set(val)
        } else {
            Set(AmbulanceCarDetailsModelEnum::Nv350)
        },

        location_latitude: if let Some(val) = payload.location_longitude {
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
        hospital_id: Default::default(),
    }
}
