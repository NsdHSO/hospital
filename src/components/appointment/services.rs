use crate::components::hospital::HospitalService;
use crate::components::patient::PatientService;
use crate::components::staff::StaffService;
use crate::entity;
use crate::entity::appointment::Column::HospitalId;
use crate::entity::appointment::{ActiveModel, AppointmentRequestBody, Entity, Model};
use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{
    check_if_is_duplicate_key_from_data_base, generate_ic, now_time, parse_date,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use std::str::FromStr;
use uuid::Uuid;

pub struct AppointmentService {
    conn: DatabaseConnection,
    staff_service: StaffService,
    hospital_service: HospitalService,
    patient_service: PatientService,
}

impl AppointmentService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            conn: conn.clone(),
            staff_service: StaffService::new(conn),
            patient_service: PatientService::new(conn),
            hospital_service: HospitalService::new(conn),
        }
    }

    pub async fn get_appointments(
        &self,     // Changed to &self as we're not modifying the service state
        page: u64, // Use u64 for pagination
        per_page: u64,
        filter: Option<String>,
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let mut query = Entity::find();
        if let Some(filter_str) = filter {
            match filter_str.split_once('=') {
                Some(("hospital_id", encoded_name)) => match Uuid::parse_str(encoded_name) {
                    Ok(uuid_val) => query = query.filter(HospitalId.eq(uuid_val)),
                    Err(_) => {
                        return Err(CustomError::new(
                            HttpCodeW::BadRequest,
                            format!("Invalid UUID format for id: {encoded_name}"),
                        ));
                    }
                },
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

    pub async fn create(
        &self,
        appointment_data: AppointmentRequestBody,
    ) -> Result<Model, CustomError> {
        let hospital_id: Uuid = self
            .hospital_service
            .find_by_field("name", appointment_data.hospital_name.as_str())
            .await?
            .unwrap()
            .id;
        if hospital_id.is_nil() {
            return Err(CustomError::new(
                HttpCodeW::InternalServerError,
                "Database error: hospital_id".to_string(),
            ));
        }
        let doctor = self
            .staff_service
            .find_by_field("name", appointment_data.doctor_name.as_str())
            .await?;
        if doctor.is_none() {
            return Err(CustomError::new(
                HttpCodeW::NotFound,
                format!(
                    "Doctor with name '{}' not found",
                    appointment_data.doctor_name
                ),
            ));
        } else {
            let doctor_data = doctor.unwrap();
            if doctor_data.hospital_id != hospital_id {
                return Err(CustomError::new(
                    HttpCodeW::NotFound,
                    format!(
                        "Doctor with name '{}' not found in this hospital {}",
                        appointment_data.doctor_name, appointment_data.hospital_name
                    ),
                ));
            }
            let patient = self
                .patient_service
                .find_by_field("name", appointment_data.patient_name.as_str())
                .await?;

            if let Some(patient_data) = patient {
                let mut attempts = 0;
                const MAX_ATTEMPTS: usize = 5;

                // This is intentionally a loop that runs at most once
                // It's designed to use the check_if_is_duplicate_key_from_data_base function consistently
                {
                    if attempts >= MAX_ATTEMPTS {
                        return Err(CustomError::new(
                            HttpCodeW::InternalServerError,
                            "Failed to generate a unique emergency IC after multiple attempts."
                                .to_string(),
                        ));
                    }

                    let active_model = self.generate_model(
                        &appointment_data,
                        hospital_id,
                        &doctor_data,
                        &patient_data,
                    );

                    let result = active_model?.insert(&self.conn).await;

                    return if let Some(value) =
                        check_if_is_duplicate_key_from_data_base(&mut attempts, result)
                    {
                        value
                    } else {
                        Err(CustomError::new(
                            HttpCodeW::NotFound,
                            format!(
                                "Patient with name '{}' not found",
                                appointment_data.patient_name
                            ),
                        ))
                    };
                }
            }
        }

        Err(CustomError::new(
            HttpCodeW::InternalServerError,
            format!("Database error: {hospital_id}"),
        ))
    }

    fn generate_model(
        &self,
        appointment_data: &AppointmentRequestBody,
        hospital_id: Uuid,
        doctor_data: &entity::staff::Model,
        patient_data: &entity::patient::Model,
    ) -> Result<ActiveModel, CustomError> {
        let now = now_time();

        // Parse the appointment date and handle potential errors
        let appointment_date = parse_date(appointment_data.appointment_date.as_str())
            .map_err(|e| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Invalid appointment date: {e}"),
                )
            })
            .unwrap_or_else(|_| DateTime::<Utc>::from_naive_utc_and_offset(now, Utc));
        let cost = match &appointment_data.cost {
            Some(cost) => Decimal::from_str(cost).map_err(|e| {
                CustomError::new(HttpCodeW::BadRequest, format!("Invalid cost value: {e}"))
            })?,
            None => Decimal::from_str("0").expect("Zero should always parse"),
        };

        Ok(ActiveModel {
            created_at: Set(now),
            updated_at: Set(now),
            id: Set(Uuid::new_v4()),
            appointment_ic: Set(generate_ic()),
            patient_id: Set(patient_data.id),
            doctor_id: Set(doctor_data.id),
            hospital_id: Set(hospital_id),
            appointment_date: Set(appointment_date.naive_utc()),
            reason: Set(Option::from(appointment_data.reason.clone())),
            notes: Set(Option::from(appointment_data.notes.clone())),
            cost: Set(cost),
            scheduled_by: Set(Option::from(appointment_data.scheduled_by.clone())),
            appointment_type: Set(Option::from(appointment_data.appointment_type.clone())),
            status: Set(appointment_data.status.clone()),
        })
    }
}
