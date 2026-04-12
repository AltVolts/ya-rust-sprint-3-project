use actix_web::{get, web, HttpResponse, Responder, Scope};
use chrono::Utc;
use crate::presentation::http_handlers::HealthResponse;

pub fn scope() -> Scope {
    web::scope("")
        .service(health)
        // .service(register)
        // .service(login)
        // .service(token)
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}