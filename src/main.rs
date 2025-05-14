use crate::components::emergency::start_scheduler;
use crate::open_api::init;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use listenfd::ListenFd;
use std::env;
use chrono::Local;
use utoipa_swagger_ui::SwaggerUi;

mod components;
mod db;
mod entity;
mod error_handler;
mod http_response;
mod open_api;
mod shared;
mod utils;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let conn: sea_orm::DatabaseConnection = db::config::init().await.expect("Failed to initialize database connection"); // Initialize connection here
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
        start_scheduler(&scheduler_conn).await.expect("Failed to start scheduler");
    });
    let server_conn = conn.clone();

    let mut listened = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_conn.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    .configure(components::ambulance::init_routes)
                    .configure(components::emergency::init_routes)
                    .configure(components::dashboard::init_routes)
                    .configure(components::card::init_routes),
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