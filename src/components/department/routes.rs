use super::services::DepartmentService;
use crate::entity::department::DepartmentRequestBody;
use crate::http_response::error_handler::CustomError;
use crate::http_response::http_response_builder;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

#[post("/department")]
async fn create(
    department: web::Json<DepartmentRequestBody>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = DepartmentService::new(db_conn.get_ref());
    let hospital = service
        .create(Option::from(department.into_inner()))
        .await?;
    let response = http_response_builder::ok(hospital);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/department")]
pub async fn find_all(
    db_conn: web::Data<DatabaseConnection>,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = DepartmentService::new(db_conn.get_ref());

    // These lines correctly extract Option<&String> from the HashMap
    let field_option_string: Option<&String> = params.get("field");
    let value_option_string: Option<&String> = params.get("value");

    // This is the crucial step: Convert Option<&String> to Option<&str>
    let field_str: Option<&str> = field_option_string.map(String::as_str);
    let value_str: Option<&str> = value_option_string.map(String::as_str);
    // Parse pagination parameters
    let page = params.get("page").and_then(|p| p.parse::<u64>().ok());
    let per_page = params.get("per_page").and_then(|l| l.parse::<u64>().ok());
    // *** THE FIX IS HERE: Use field_str and value_str ***
    let person = service_instance
        .find_departments(field_str, value_str, page, per_page)
        .await?;

    let response = http_response_builder::ok(person);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
    config.service(find_all);
}
