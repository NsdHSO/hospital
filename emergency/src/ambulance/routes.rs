use crate::ambulance::services::AmbulanceService;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{get, web, HttpResponse};

#[get("/ambulance/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let service = AmbulanceService::new().await?;
    let emergency = service.find_by_ic(*id).await?;
    let response = http_response_builder::ok(emergency);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/ambulance")]
pub async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let service = AmbulanceService::new().await?;
    let emergency = service
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
        )
        .await?;
    let response = http_response_builder::ok(emergency);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
}
