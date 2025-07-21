use crate::components::hospital::HospitalService;
use crate::components::patient::PatientService;
use crate::components::staff::StaffService;
use crate::entity::appointment::{AppointmentRequestBody, Model};
use crate::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use sea_orm::DatabaseConnection;
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
            if let Some(doctor_data) = doctor {
                if doctor_data.hospital_id != hospital_id {
                    println!("{:?}", doctor_data.hospital_id);
                    return Err(CustomError::new(
                        HttpCodeW::InternalServerError,
                        "Database error: Doctor doesn't work at this hospital".to_string(),
                    ));
                }
            }
            let patient = self
                .patient_service
                .find_by_field("name", appointment_data.patient_name.as_str())
                .await?;
            
            
            

        }

        Err(CustomError::new(
            HttpCodeW::InternalServerError,
            format!("Database error: {hospital_id}"),
        ))
    }
}
