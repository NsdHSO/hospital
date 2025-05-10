use crate::emergency::services::EmergencyService;
use crate::entity::emergency::EmergencyRequestBody;
use crate::error_handler::CustomError;
use crate::http_response::{created, http_response_builder};
use crate::shared::PaginationParams;
use actix_web::{get, post, web, HttpResponse};
use web::Path;

#[get("/emergency/{id}")]
async fn find(id: Path<String>) -> Result<HttpResponse, CustomError> {
    let service = EmergencyService::new().await?;
    let ambulance = service.find_by_ic(&**id).await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/emergency")]
pub async fn find_all(query: web::Query<PaginationParams>) -> Result<HttpResponse, CustomError> {
    let service_instance = EmergencyService::new().await;
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
#[post("/emergency")]
async fn create(emergency: web::Json<EmergencyRequestBody>) -> Result<HttpResponse, CustomError> {
    let service = EmergencyService::new().await?;
    let created_emergency = service.create_emergency(emergency.into_inner()).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Emergency created successfully",
        "data": created_emergency
    })))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(create);
}
