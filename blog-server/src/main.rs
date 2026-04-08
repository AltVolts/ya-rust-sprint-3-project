use crate::infrastructure::Config;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use infrastructure::logging;
use std::sync::Arc;
use tracing::info;

mod infrastructure;
mod presentation;
mod data;
mod application;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    logging::init_logging();

    let cfg = Config::from_env().expect("Invalid configuration");

    let addr = format!("{}:{}", cfg.host, cfg.port);
    info!("→ listening on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&cfg.cors_origin)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                    ])
                    .supports_credentials()
                    .max_age(600),
            )
            .wrap(Logger::default())
            .app_data(cfg.clone())
    })
    .bind(addr)?
    .run()
    .await
}
