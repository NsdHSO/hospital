use crate::components::ambulance::services::AmbulanceService;
use crate::entity::ambulance::AmbulancePayload;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/ambulance")]
async fn create(
    ambulance: web::Json<AmbulancePayload>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = AmbulanceService::new(db_conn.get_ref());
    let ambulance = service
        .create_ambulance(Option::from(ambulance.into_inner()))
        .await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/ambulance")]
pub async fn find_all(
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = AmbulanceService::new(db_conn.get_ref());

    let ambulance = service_instance
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
            query.filter.clone(),
        )
        .await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(create);
}
