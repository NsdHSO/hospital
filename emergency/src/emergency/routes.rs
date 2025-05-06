use actix_web::{get, web, HttpResponse};
use crate::emergency::services::EmergencyService;
use crate::error_handler::CustomError;

#[get("/emergency/{id}")]
async fn find(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let emergency = EmergencyService::find_one(&id)?;
    Ok(HttpResponse::Ok().json(emergency))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
}
