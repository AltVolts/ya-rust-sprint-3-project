use crate::infrastructure::Config;
use crate::presentation::{RequestIdMiddleware, TimingMiddleware};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use infrastructure::database;
use infrastructure::logging;
use presentation::http_handlers;
use tracing::info;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    logging::init_logging();

    let cfg = Config::from_env().expect("Invalid configuration");

    let pool = database::create_pool(&cfg.database_url)
        .await
        .expect("Failed to create db_pool");
    database::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

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
            .wrap(TimingMiddleware)
            .wrap(RequestIdMiddleware)
            .wrap(Logger::default())
            // .app_data(cfg.clone())
            .configure(http_handlers::configure)
    })
    .bind(addr)?
    .run()
    .await
}
