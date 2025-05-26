use crate::components::dashboard::DashboardService;
use crate::entity::dashboard::PayloadBodyDashboard;
use crate::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::shared::PaginationParams;
use actix_web::{HttpResponse, get, post, web};
use sea_orm::DatabaseConnection;

#[get("/dashboard")]
pub async fn find_all(
    query: web::Query<PaginationParams>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = DashboardService::new(db_conn.get_ref());

    let dashboard = service_instance
        .find_all(
            query.page.try_into().unwrap(),
            query.per_page.try_into().unwrap(),
        )
        .await?;
    let response = http_response_builder::ok(dashboard);
    Ok(HttpResponse::Ok().json(response))
}
#[post("/dashboard")]
pub async fn create(
    payload: web::Json<PayloadBodyDashboard>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let service_instance = DashboardService::new(db_conn.get_ref());

    let dashboard = service_instance.create(payload.into_inner()).await?;
    let response = http_response_builder::ok(dashboard);
    Ok(HttpResponse::Ok().json(response))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(create);
}
