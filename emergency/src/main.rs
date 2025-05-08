#[macro_use]
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use listenfd::ListenFd;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::open_api::ApiDoc;

mod ambulance;
mod db;
mod emergency;
mod error_handler;
mod http_response;
mod open_api;
mod schema;
mod shared;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::config::init();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    .configure(ambulance::init_routes)
                    .configure(emergency::init_routes),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
