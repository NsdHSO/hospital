use crate::ambulance::services::AmbulanceService;
use crate::error_handler::CustomError;
use crate::http_response::{http_response_builder, ResponseObject};
use crate::shared::PaginationParams;
use actix_web::{get, web, HttpResponse};
use utoipa::ToSchema;

#[get("/ambulance")]
#[utoipa::path(
    get,
    path = "/v1/ambulance",
    tag = "ambulance",
    responses(
        (status = 200, description = "List of ambulances retrieved successfully", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let mut service = AmbulanceService::new()?;
    let ambulance = service.find_all(query.page, query.per_page)?;

    let response: ResponseObject<_> = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
}
