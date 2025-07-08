use crate::components::ambulance::services::AmbulanceService;
use crate::entity::ambulance::{AmbulanceId, AmbulancePayload};
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{HttpResponse, get, patch, post, web};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

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

#[patch("/ambulance/{uuid_ambulance}")]
async fn update(
    uuid_ambulance: web::Path<Uuid>,
    ambulance_payload: web::Json<AmbulancePayload>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service = AmbulanceService::new(db_conn.get_ref());
    let ambulance = service
        .update_ambulance(
            AmbulanceId::Uuid(uuid_ambulance.into_inner()),
            ambulance_payload.clone(),
        )
        .await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}
#[patch("/ambulance/ic/{id_ambulance}")]
async fn update_by_ic(
    id_ambulance: web::Path<i32>,
    ambulance_payload: web::Json<AmbulancePayload>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service = AmbulanceService::new(db_conn.get_ref());
    let ambulance = service
        .update_ambulance(
            AmbulanceId::Integer(id_ambulance.into_inner()),
            ambulance_payload.clone(),
        )
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

#[get("/ambulance/status")]
pub async fn find_all_statuses(
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = AmbulanceService::new(db_conn.get_ref());
    let ambulance = service_instance.find_all_status().await?;
    let response = http_response_builder::ok(ambulance);
    Ok(HttpResponse::Ok().json(response))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(update);
    config.service(find_all);
    config.service(create);
    config.service(update_by_ic);
    config.service(find_all_statuses);
}
