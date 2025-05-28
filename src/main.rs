use crate::components::emergency::start_scheduler;
use crate::open_api::init;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use chrono::Local;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use listenfd::ListenFd;
use log::error;
use std::env;
use utoipa_swagger_ui::SwaggerUi;

mod components;
mod db;
mod entity;
mod error_handler;
mod http_response;
mod open_api;
mod shared;
mod tests;
mod utils;

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
            error!("Scheduler crashed: {:?}", e);
        }
    });
    let data_base_conn = conn.clone();

    let mut listened = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_origin("https://tevet-troc-client.vercel.app")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .supports_credentials();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(data_base_conn.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    .configure(components::ambulance::init_routes)
                    .configure(components::emergency::init_routes)
                    .configure(components::dashboard::init_routes)
                    .configure(components::card::init_routes)
                    .configure(components::hospital::init_routes),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", init()))
    });

    server = match listened.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
