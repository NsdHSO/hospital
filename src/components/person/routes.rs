use crate::components::person::PersonService;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use crate::components::patient::PatientService;
use crate::entity::patient::PatientRequestBody;
use crate::entity::person::PersonRequestBody;

#[get("/person")]
pub async fn find_all(
    db_conn: web::Data<DatabaseConnection>,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = PersonService::new(db_conn.get_ref());

    // Example: find by "email"
    let field = params.get("field").cloned().unwrap_or_default();
    let value = params.get("value").cloned().unwrap_or_default();

    let hospital = service_instance.find_by_field(&field, &value).await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}

#[post("/person")]
async fn create(
    patient: web::Json<PersonRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = PersonService::new(db_conn.get_ref());
    let hospital = service
        .create(Option::from(patient.into_inner()))
        .await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(create);
}
