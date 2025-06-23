use super::services::StaffService;
use crate::entity::staff::StaffRequestBody;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use actix_web::{HttpResponse, post, web};
use sea_orm::DatabaseConnection;

#[post("/staff")]
async fn create(
    staff: web::Json<StaffRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = StaffService::new(db_conn.get_ref());
    let hospital = service.create(Option::from(staff.into_inner())).await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}
