use crate::components::hospital::HospitalService;
use crate::components::patient::PatientService;
use crate::components::staff::StaffService;
use crate::entity;
use crate::entity::appointment::{ActiveModel, AppointmentRequestBody, Model};
use crate::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use crate::utils::helpers::{
    check_if_is_duplicate_key_from_data_base, generate_ic, now_time, parse_date,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
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
        if (hospital_id.is_nil()) {
            return Err(CustomError::new(
                HttpCodeW::InternalServerError,
                "Database error: hospital_id".to_string(),
            ));
        }
        let doctor = self
            .staff_service
            .find_by_field("name", appointment_data.doctor_name.as_str())
            .await?;
        if (doctor.is_none()) {
            return Err(CustomError::new(
                HttpCodeW::InternalServerError,
                "Database error: doctor_id".to_string(),
            ));
        } else {
            let doctor_data = doctor.unwrap();
            if doctor_data.hospital_id != hospital_id {
                println!("{:?}", doctor_data.hospital_id);
                return Err(CustomError::new(
                    HttpCodeW::InternalServerError,
                    "Database error: Doctor doesn't work at this hospital".to_string(),
                ));
            }
            let patient = self
                .patient_service
                .find_by_field("name", appointment_data.patient_name.as_str())
                .await?;

            if let Some(patient_data) = patient {
                let mut attempts = 0;
                const MAX_ATTEMPTS: usize = 5;

                loop {
                    if attempts >= MAX_ATTEMPTS {
                        return Err(CustomError::new(
                            HttpCodeW::InternalServerError,
                            "Failed to generate a unique emergency IC after multiple attempts."
                                .to_string(),
                        ));
                    }

                    let active_model = self.generate_model(
                        &appointment_data,
                        hospital_id, // Copy types like Uuid don't need &
                        &doctor_data,
                        &patient_data,
                    );

                    // Insert the record into the database
                    let result = active_model.insert(&self.conn).await;

                    if let Some(value) =
                        check_if_is_duplicate_key_from_data_base(&mut attempts, result)
                    {
                        return value;
                    }
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
    ) -> ActiveModel {
        let now = now_time();

        // Parse the appointment date and handle potential errors
        let appointment_date = parse_date(appointment_data.appointment_date.as_str())
            .map_err(|e| {
                // Log the error or handle it appropriately
                println!("Error parsing appointment date: {}", e);
                // Return a default date or handle as needed for your application
                CustomError::new(
                    HttpCodeW::BadRequest,
                    format!("Invalid appointment date: {}", e),
                )
            })
            .unwrap_or_else(|_| DateTime::<Utc>::from_naive_utc_and_offset(now, Utc));

        ActiveModel {
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
            cost: match &appointment_data.cost {
                Some(cost) => Set(Decimal::from_str(cost).unwrap()),
                None => Set(Decimal::from_str("0").unwrap()),
            },
            scheduled_by: Set(Option::from(appointment_data.scheduled_by.clone())),
            appointment_type: Set(Option::from(appointment_data.appointment_type.clone())),
            status: Set(Option::from(appointment_data.status.clone()).expect("REASON")),
        }
    }
}
