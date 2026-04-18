use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::Config;
use crate::infrastructure::security::JwtService;
use crate::presentation::http_handlers::auth::health;
use crate::presentation::http_handlers::posts::{
    create_post, delete_post, get_post, list_posts, update_post,
};
use crate::presentation::{
    BlogServiceImpl, RequestIdMiddleware, TimingMiddleware, http_handlers, middleware,
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use infrastructure::database;
use infrastructure::logging;
use std::sync::Arc;
use tonic::transport::Server;

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
    let post_repo = Arc::new(PostgresPostRepository::new(pool.clone()));

    let jwt_service = JwtService::new(&cfg.jwt_secret);
    let auth_service = AuthService::new(user_repo.clone(), jwt_service);

    let blog_service = BlogService::new(post_repo.clone());

    let jwt_service_data = web::Data::new(auth_service.jwt_service().clone());
    let auth_service_data = web::Data::new(auth_service.clone());
    let blog_service_data = web::Data::new(blog_service.clone());

    let auth = HttpAuthentication::bearer(middleware::jwt::jwt_validator);

    let http_addr = format!("{}:{}", cfg.host, cfg.port);
    info!("→ HTTP server listening on http://{}", http_addr);

    let grpc_addr = format!("{}:{}", cfg.host, cfg.grpc_port);
    let grpc_auth_service = auth_service.clone();
    let grpc_blog_service = blog_service.clone();

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(jwt_service_data.clone())
            .app_data(auth_service_data.clone())
            .app_data(blog_service_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(&cfg.cors_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(600),
            )
            .wrap(TimingMiddleware)
            .wrap(RequestIdMiddleware)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(health)
                    .service(http_handlers::auth::auth_scope())
                    .service(
                        web::scope("/posts")
                            .route("", web::get().to(list_posts))
                            .route("", web::post().to(create_post).wrap(auth.clone()))
                            .route("/{id}", web::get().to(get_post))
                            .route("/{id}", web::put().to(update_post).wrap(auth.clone()))
                            .route("/{id}", web::delete().to(delete_post).wrap(auth.clone())),
                    ),
            )
    })
    .bind(http_addr)?
    .run();

    let grpc_service =
        BlogServiceImpl::new(Arc::new(grpc_auth_service), Arc::new(grpc_blog_service));

    let grpc_server = Server::builder()
        .add_service(blog_proto::blog_service_server::BlogServiceServer::new(
            grpc_service,
        ))
        .serve(grpc_addr.parse().expect("Invalid gRPC address"));

    info!("→ gRPC server listening on http://{}", grpc_addr);

    tokio::select! {
        res = http_server => {
            if let Err(e) = res {
                eprintln!("HTTP server error: {}", e);
            }
        }
        res = grpc_server => {
            if let Err(e) = res {
                eprintln!("gRPC server error: {}", e);
            }
        }
    }

    Ok(())
}
