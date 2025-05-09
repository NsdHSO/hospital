use crate::ambulance::services::AmbulanceService;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{get, web, HttpResponse};

#[get("/ambulance/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let service = AmbulanceService::new().await?;
    let ambulance = service.find_by_ic(*id).await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/ambulance")]
pub async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let service_instance = AmbulanceService::new().await;
    let service = match service_instance {
        Ok(service) => service,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let ambulance = service
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
        )
        .await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
}
