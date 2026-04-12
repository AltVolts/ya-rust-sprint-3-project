use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub mod auth;
mod error;
pub mod http_handlers;
mod middleware;

pub use middleware::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Создаём middleware аутентификации прямо здесь
    let auth_middleware = HttpAuthentication::bearer(auth::jwt_validator);

    cfg.service(
        web::scope("/api")
            // Публичные маршруты (без аутентификации)
            .service(http_handlers::public::health)
            // Защищённые маршруты (с Bearer-аутентификацией)
            .service(
                web::scope("")
                    .wrap(auth_middleware)
                    .service(http_handlers::protected::scope()),
            ),
    );
}
