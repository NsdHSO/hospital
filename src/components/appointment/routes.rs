use super::services::AppointmentService;
use crate::entity::appointment::AppointmentRequestBody;
use crate::http_response::error_handler::CustomError;
use crate::http_response::{HttpCodeW, http_response_builder};
use actix_web::{HttpResponse, post, web};
use sea_orm::DatabaseConnection;

#[post("/appointment")]
async fn create(
    staff: web::Json<AppointmentRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = AppointmentService::new(db_conn.get_ref());
    let appointment = service.create(staff.into_inner()).await;
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
}
