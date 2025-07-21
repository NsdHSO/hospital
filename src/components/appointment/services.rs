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
        let mut hospital_id : Uuid = self.hospital_service.find_by_field("name", appointment_data.hospital_name.as_str()).await?.unwrap().id;
        
        Err(CustomError::new(
            HttpCodeW::InternalServerError,
            format!("Database error: "),
        ))
    }
}
