use crate::components::dashboard::DashboardService;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{get, web, HttpResponse};
use sea_orm::DatabaseConnection;
use sea_orm::sqlx::query;
use crate::components::card::CardService;

#[get("/card")]
pub async fn find_all(
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = CardService::new(db_conn.get_ref());

    let card = service_instance
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(), 
            query.filter.clone().try_into().unwrap()
        )
        .await?;
    let response = http_response_builder::ok(card);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
}
