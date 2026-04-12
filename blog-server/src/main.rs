use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::Config;
use crate::infrastructure::security::JwtService;
use crate::presentation::{RequestIdMiddleware, TimingMiddleware, http_handlers};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use infrastructure::database;
use infrastructure::logging;
use std::sync::Arc;
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

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));

    let jwt_service = JwtService::new(&cfg.jwt_secret);
    let auth_service = AuthService::new(user_repo.clone(), jwt_service);

    let jwt_service_data = web::Data::new(auth_service.jwt_service().clone());

    let addr = format!("{}:{}", cfg.host, cfg.port);
    info!("→ listening on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(jwt_service_data.clone())
            .app_data(web::Data::new(auth_service.clone()))
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
            .configure(presentation::configure)
    })
    .bind(addr)?
    .run()
    .await
}
