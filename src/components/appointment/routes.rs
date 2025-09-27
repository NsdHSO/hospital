use super::services::AppointmentService;
use crate::entity::appointment::AppointmentRequestBody;
use crate::http_response::error_handler::CustomError;
use crate::http_response::{http_response_builder, HttpCodeW};
use crate::shared::{
    AppointmentCreatePermission, AppointmentReadPermission, PaginationParams, Require,
};
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/appointment")]
async fn create(
    _perm: Require<AppointmentCreatePermission>,
    appointment_body: web::Json<AppointmentRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = AppointmentService::new(db_conn.get_ref());
    let appointment = service.create(appointment_body.into_inner()).await;
    match appointment {
        Ok(data) => {
            let response = http_response_builder::ok(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => Err(CustomError::new(
            HttpCodeW::InternalServerError,
            err.to_string(),
        )),
    }
}

#[get("/appointment")]
async fn get_appointment(
    _perm: Require<AppointmentReadPermission>,
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service = AppointmentService::new(db_conn.get_ref());
    let appointment = service
        .get_appointments(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
            query.filter.clone(),
        )
        .await;
    match appointment {
        Ok(data) => {
            let response = http_response_builder::ok(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => Err(CustomError::new(
            HttpCodeW::InternalServerError,
            err.to_string(),
        )),
    }
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
    config.service(get_appointment);
}
