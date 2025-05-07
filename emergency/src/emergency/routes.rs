use crate::emergency::services::EmergencyService;
use crate::emergency::{Emergency, NewEmergencyRequest};
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::http_response::ResponseObject;
use crate::shared::PaginationParams;
use actix_web::{get, post, web, HttpResponse};

#[get("/emergency/{id}")]
async fn find(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let mut service = EmergencyService::new()?;
    let emergency = service.find_one(&id)?;
    let response = http_response_builder::ok(emergency);
    Ok(HttpResponse::Ok().json(response))
}
#[post("/emergency")]
async fn create(emergency: web::Json<NewEmergencyRequest>) -> Result<HttpResponse, CustomError> {
    let mut service = EmergencyService::new()?;
    let created_emergency = service.create(Emergency::from(emergency.into_inner()))?;
    let response = http_response_builder::ok(created_emergency);
    Ok(HttpResponse::Created().json(response))
}

#[get("/emergency")]
async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let mut service = EmergencyService::new()?;
    let emergency = service.find_all(query.page, query.per_page)?;

    let response: ResponseObject<_> = http_response_builder::ok(emergency);
    Ok(HttpResponse::Ok().json(response))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("v1")
            .service(find)
            .service(find_all)
            .service(create),
    );
}
