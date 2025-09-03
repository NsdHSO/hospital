use crate::components::emergency::start_scheduler;
use crate::open_api::init;
use crate::security::jwt::JwtAuth;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use chrono::Local;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use listenfd::ListenFd;
use log::error;
use std::env;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use jsonwebtoken::DecodingKey;
use utoipa_swagger_ui::SwaggerUi;
use crate::components::config::ConfigService;

mod components;
mod db;
mod entity;
mod http_response;
mod open_api;
mod security;
mod shared;
mod tests;
mod utils;
fn config_service() -> ConfigService {
    ConfigService::new().clone()
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let conn: sea_orm::DatabaseConnection = db::config::init()
        .await
        .expect("Failed to initialize database connection"); // Initialize connection here
    Builder::from_env(Env::default().default_filter_or("debug"))
        .format(|buf, record| {
            use std::io::Write;
            let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            writeln!(
                buf,
                "[{}] {} {} - {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
    let scheduler_conn = conn.clone();
    tokio::spawn(async move {
        if let Err(e) = start_scheduler(&scheduler_conn).await {
            error!("Scheduler crashed: {e:?}");
        }
    });
    let data_base_conn = conn.clone();

    let mut listened = ListenFd::from_env();
    let auth_base_url = env::var("AUTH_BASE_URL")
        .expect("Please set AUTH_BASE_URL in .env (e.g., http://localhost:8081)");
    let pem_bytes = STANDARD
        .decode(config_service().access_token_public_key)
        .expect("ACCESS_TOKEN_PUBLIC_KEY is not valid base64");
    let decoding_key = DecodingKey::from_rsa_pem(&pem_bytes)
        .expect("ACCESS_TOKEN_PUBLIC_KEY is not a valid PEM");
    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req| origin.as_bytes().starts_with(b"http://"))
            .allowed_origin_fn(|origin, _req| {
                origin.as_bytes().starts_with(b"https://")
                    && origin.to_str().unwrap().contains("vercel")
            })
            .allowed_origin("https://nsdhso.github.io")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT, header::AUTHORIZATION])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(data_base_conn.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    // Public routes can be added here before the protected scope if needed
                    .service(
                        web::scope("")
                            .wrap(JwtAuth::new(auth_base_url.clone()))
                            .app_data(web::Data::new(decoding_key.clone()))
                            .configure(components::ambulance::init_routes)
                            .configure(components::emergency::init_routes)
                            .configure(components::dashboard::init_routes)
                            .configure(components::card::init_routes)
                            .configure(components::patient::init_routes)
                            .configure(components::person::init_routes)
                            .configure(components::staff::init_routes)
                            .configure(components::department::init_routes)
                            .configure(components::hospital::init_routes)
                            .configure(components::appointment::init_routes),
                    ),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", init()))
    });

    server = match listened.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server
                .bind(format!("{host}:{port}"))
                .unwrap_or_else(|_| panic!("host: {host}> Port {port}"))
        }
    };

    server.run().await
}
