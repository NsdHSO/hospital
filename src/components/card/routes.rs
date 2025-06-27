use crate::components::card::CardService;
use crate::entity::card::CardPayload;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{HttpResponse, get, post, web};
use sea_orm::DatabaseConnection;

#[post("/card")]
async fn create(
    ambulance: web::Json<CardPayload>,
    db_conn: web::Data<DatabaseConnection>, // Inject the database connection
) -> Result<HttpResponse, CustomError> {
    let service = CardService::new(db_conn.get_ref());
    let card = service
        .create_card(Option::from(ambulance.into_inner()))
        .await?;
    let response = http_response_builder::ok(card);
    Ok(HttpResponse::Ok().json(response))
}
#[get("/card")]
pub async fn find_all(
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = CardService::new(db_conn.get_ref());

    let card = service_instance
        .find_all(
            query.page.try_into().unwrap(),     // Add the page parameter
            query.per_page.try_into().unwrap(), // Keep the per_page parameter
            query.filter.clone(), // No need to unwrap and re-wrap in Some; it's already an Option<String>
        )
        .await?;
    let response = http_response_builder::ok(card);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(create);
}
