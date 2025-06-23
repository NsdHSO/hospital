use super::services::DepartmentService;
use crate::entity::department::DepartmentRequestBody;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use actix_web::{post, web, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/department")]
async fn create(
    department: web::Json<DepartmentRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = DepartmentService::new(db_conn.get_ref());
    let hospital = service
        .create(Option::from(department.into_inner()))
        .await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}
