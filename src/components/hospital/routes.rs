
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{get, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::components::hospital::HospitalService;

#[get("/hospital/{id}")]
async fn find(
    id: web::Path<String>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = HospitalService::new(db_conn.get_ref());
    let hospital = service.find_by_ic(id.to_string()).await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/hospital")]
pub async fn find_all(
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = HospitalService::new(db_conn.get_ref());

    let hospital = service_instance
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
        )
        .await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
}
