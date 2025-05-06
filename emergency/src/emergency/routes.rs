use crate::emergency::services::EmergencyService;
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    page: i64,
    #[serde(default = "default_per_page")]
    per_page: i64,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    10
}

#[get("/emergency/{id}")]
async fn find(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let emergency = EmergencyService::find_one(&id)?;
    Ok(HttpResponse::Ok().json(emergency))
}

#[get("/emergency")]
async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let emergency = EmergencyService::find_all(query.page, query.per_page)?;
    Ok(HttpResponse::Ok().json(emergency))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
}
