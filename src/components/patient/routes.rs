use crate::components::patient::PatientService;
use crate::entity::patient::PatientRequestBody;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/patient")]
async fn create(
    patient: web::Json<PatientRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = PatientService::new(db_conn.get_ref());
    let hospital = service
        .patient_emergency(Option::from(patient.into_inner()))
        .await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/patient/{id}")]
async fn find(
    id: web::Path<String>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = PatientService::new(db_conn.get_ref());
    let hospital = service.find_by_name(id.to_string()).await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/patient")]
pub async fn find_all(
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = PatientService::new(db_conn.get_ref());

    let hospital = service_instance
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
        )
        .await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(create);
}
