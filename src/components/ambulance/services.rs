use crate::entity::ambulance::Column::Id;
use crate::entity::ambulance::{ActiveModel, AmbulanceId, Model, StatusDto};
use crate::entity::ambulance::{AmbulancePayload, Column, Entity};
use crate::entity::sea_orm_active_enums::{
    AmbulanceCarDetailsMakeEnum, AmbulanceCarDetailsModelEnum, AmbulanceStatusEnum,
    AmbulanceTypeEnum,
};

use crate::entity::hospital;
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic, now_time};

use crate::components::emergency::EmergencyService;
use crate::components::hospital::HospitalService;
use crate::components::patient::PatientService;
use crate::http_response::HttpCodeW;
use hospital::Column::Name as HospitalName;
use hospital::Entity as HospitalEntity;
use percent_encoding::percent_decode_str;
use sea_orm::prelude::Decimal;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, ColumnTrait, Iterable, PaginatorTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{NotSet, QueryFilter, Set};
use Column::AmbulanceIc;

pub struct AmbulanceService {
    conn: DatabaseConnection,
    emergency_service: EmergencyService,
    patient_service: PatientService,
    hospital_service: HospitalService,
}

impl AmbulanceService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        AmbulanceService {
            conn: conn.clone(),
            emergency_service: EmergencyService::new(conn),
            patient_service: PatientService::new(conn),
            hospital_service: HospitalService::new(conn),
        }
    }
    pub(crate) async fn update_ambulance(
        &self,
        id: AmbulanceId,
        mut payload: AmbulancePayload,
    ) -> Result<Model, CustomError> {
        let now = now_time();
        // Declare a mutable variable to hold the query result (Option<Model>)
        let model_option;

        // Initialize a base query outside the match if there are common parts
        let base_query = Entity::find();

        match id {
            AmbulanceId::Uuid(value) => {
                model_option = base_query
                    .filter(Id.eq(value)) // Assuming `Id` is the Uuid column
                    .one(&self.conn) // Use self.db_pool if that's your connection
                    .await
                    .map_err(|e| {
                        CustomError::new(
                            HttpCodeW::InternalServerError,
                            format!("DB error fetching by UUID: {}", e),
                        )
                    })?; // Handle SeaORM errors
            }
            AmbulanceId::Integer(value) => {
                model_option = base_query
                    .filter(AmbulanceIc.eq(value)) // Assuming `AmbulanceIc` is the i32 column
                    .one(&self.conn) // Use self.db_pool if that's your connection
                    .await
                    .map_err(|e| {
                        CustomError::new(
                            HttpCodeW::InternalServerError,
                            format!("DB error fetching by Integer ID: {}", e),
                        )
                    })?; // Handle SeaORM errors
            }
        }

        let model = match model_option {
            Some(m) => m,
            None => {
                return Err(CustomError::new(
                    HttpCodeW::NotFound,
                    "Ambulance not found".to_string(),
                ));
            }
        };

        let mut active_model: ActiveModel = model.into();

        match payload.status {
            Some(AmbulanceStatusEnum::TransportingPatient) => {
                if payload.hospital_name.eq(&None) {
                    return Err(CustomError::new(
                        HttpCodeW::BadRequest,
                        "hospital_name is required for transporting patient".to_string(),
                    ));
                }
                let mut hospital = self
                    .hospital_service
                    .find_by_field("id", payload.hospital_name.unwrap().as_str())
                    .await?;
                let hospital_uid: Uuid = match hospital {
                    None => {
                        return Err(CustomError::new(
                            HttpCodeW::BadRequest,
                            "Invalid Hospital Name".to_string(),
                        ));
                    }
                    Some(value) => value.id,
                };
                active_model.hospital_id = Set(hospital_uid);
                self.set_transport_patient(id, &mut active_model).await?;
            }
            Some(status) => {
                active_model.status = Set(status);
            }
            None => {}
        }
        if payload.driver_name.is_some() {
            active_model.driver_name = Set(payload.driver_name);
        }

        active_model.updated_at = Set(now);
        // Save changes
        let updated = active_model.update(&self.conn).await.map_err(|e| {
            CustomError::new(
                HttpCodeW::InternalServerError,
                format!("Database error: {e}"),
            )
        })?;

        Ok(updated)
    }

    async fn set_transport_patient(
        &self,
        uuid: AmbulanceId,
        active_model: &mut ActiveModel,
    ) -> Result<(), CustomError> {
        active_model.status = Set(AmbulanceStatusEnum::TransportingPatient);
        // Fetch passengers as Option<serde_json::Value>
        let passengers_json_opt = self
            .emergency_service
            .get_passengers_json_for_ambulance(uuid)
            .await
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::InternalServerError,
                    format!("Error fetching passengers: {e}"),
                )
            })?;

        // Get ambulance hospital_id as string
        let ambulance_hospital_id = active_model.hospital_id.clone().unwrap();

        // Set hospital_id for each passenger object
        let mut passengers_with_hospital = Vec::new();
        if let Some(serde_json::Value::Array(ref passenger_array)) = passengers_json_opt {
            for passenger in passenger_array.iter() {
                if let serde_json::Value::Object(obj) = passenger {
                    let mut obj = obj.clone();

                    // Extract patient id from the Value object
                    if let Some(id_value) = obj.get("id") {
                        if let Some(id_str) = id_value.as_str() {
                            if let Ok(patient_uuid) = Uuid::parse_str(id_str) {
                                self.patient_service
                                    .associate_hospital_with_patient(
                                        patient_uuid,
                                        ambulance_hospital_id,
                                    )
                                    .await;
                            }
                        }
                    }

                    obj.insert(
                        "hospital_id".to_string(),
                        serde_json::Value::String(ambulance_hospital_id.to_string()),
                    );
                    passengers_with_hospital.push(serde_json::Value::Object(obj));
                }
            }
        }

        // Serialize to JSON as a flat array
        let passengers_json = serde_json::Value::Array(passengers_with_hospital);
        active_model.passengers = Set(Some(passengers_json));
        Ok(())
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
                    HttpCodeW::InternalServerError,
                    "Failed to generate a unique emergency IC after multiple attempts.".to_string(),
                ));
            }
            let mut active_model = generate_payload_to_create_ambulance(payload.clone());
            let hospital_name = payload
                .as_ref()
                .and_then(|p| p.hospital_name.as_deref())
                .ok_or(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "hospital_name is required".to_string(),
                ))?;

            let hospital = HospitalEntity::find()
                .filter(HospitalName.eq(hospital_name))
                .one(&self.conn)
                .await;
            if let Ok(Some(hospital_model)) = &hospital {
                active_model.hospital_id = Set(hospital_model.id);
            } else {
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "hospital not found".to_string(),
                ));
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
                    // Parse to integer
                    let ic_value: i32 = ambulance_ic.parse().map_err(|_| {
                        CustomError::new(
                            HttpCodeW::BadRequest,
                            "Invalid integer for ic".to_string(),
                        )
                    })?;

                    query = query.filter(AmbulanceIc.eq(ic_value));
                }
                Some(("id", encoded_name)) => {
                    let ambulance_id = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
                    let ambulance_uuid = Uuid::parse_str(&ambulance_id).map_err(|_| {
                        CustomError::new(HttpCodeW::BadRequest, "Invalid UUID".to_string())
                    })?;
                    query = query.filter(Id.eq(ambulance_uuid));
                }
                _ => {}
            }
        }
        let paginator = query.paginate(&self.conn, per_page);
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

    pub async fn find_all_status(&self) -> Result<Vec<StatusDto>, CustomError> {
        let statuses: Vec<StatusDto> = AmbulanceStatusEnum::iter() // Use EnumIter to get all variants
            .map(|status_enum_variant| {
                let value = serde_json::to_string(&status_enum_variant)
                    .unwrap_or_else(|_| "\"UNKNOWN\"".to_string()) // Fallback for error
                    .trim_matches('"') // Remove quotes from the string
                    .to_string();

                // Format the label
                let label = format_status_label(&value);

                StatusDto { value, label }
            })
            .collect();

        Ok(statuses)
    }
}

pub fn generate_payload_to_create_ambulance(payload: Option<AmbulancePayload>) -> ActiveModel {
    let now = now_time();
    let payload = payload.unwrap_or_default();
    let car_details = payload.car_details.unwrap_or_default();
    ActiveModel {
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
// Helper function to convert SCREAMING_SNAKE_CASE to a more readable format
// This is the same function as before, ensuring consistent labels
fn format_status_label(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => {
                    first_char.to_uppercase().collect::<String>()
                        + chars.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
