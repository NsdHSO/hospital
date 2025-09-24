use crate::components::me::services::{MeService, UpsertProfileBody};
use crate::http_response::error_handler::CustomError;
use crate::http_response::http_response_builder;
use crate::security::subject::Subject;
use actix_web::{get, put, web, HttpResponse};
use sea_orm::DatabaseConnection;

#[get("/me/profile")]
pub async fn get_profile(
    subject: Subject,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, CustomError> {
    let svc = MeService::new(db.get_ref());
    let res = svc.get_profile(&subject.sub).await?;
    Ok(HttpResponse::Ok().json(http_response_builder::ok(res)))
}

#[put("/me/profile")]
pub async fn upsert_profile(
    subject: Subject,
    db: web::Data<DatabaseConnection>,
    body: web::Json<UpsertProfileBody>,
) -> Result<HttpResponse, CustomError> {
    let svc = MeService::new(db.get_ref());
    let res = svc.upsert_profile(&subject.sub, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(http_response_builder::ok(res)))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_profile);
    cfg.service(upsert_profile);
}
