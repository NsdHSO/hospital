use super::services::AppointmentService;
use crate::entity::appointment::AppointmentRequestBody;
use crate::http_response::error_handler::CustomError;
use crate::http_response::{http_response_builder, require_permission, Claims, HttpCodeW};
use actix_web::{post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::shared::PermissionCode;

#[post("/appointment")]
async fn create(
    claims: Claims,
    staff: web::Json<AppointmentRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    require_permission(&claims, PermissionCode::AppointmentCreate).expect("You don't Have allow here");
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
