use crate::emergency::services::EmergencyService;
use crate::emergency::PaginationParams;
use crate::error_handler::CustomError;

use crate::http_response::ResponseObject;
use actix_web::{get, web, HttpResponse};

#[get("/emergency/{id}")]
async fn find(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let emergency = EmergencyService::find_one(&id)?;
    Ok(HttpResponse::Ok().json(emergency))
}

#[get("/emergency")]
async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let emergency = EmergencyService::find_all(query.page, query.per_page)?;
    let response: ResponseObject<_> = crate::http_response::http_response_builder::ok(emergency);
    Ok(HttpResponse::Ok().json(response))

}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
}
