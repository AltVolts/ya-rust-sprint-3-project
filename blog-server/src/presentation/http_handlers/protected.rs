use actix_web::{get, web, HttpResponse, Responder, Scope};
use chrono::Utc;
use crate::presentation::http_handlers::HealthResponse;

pub fn scope() -> Scope {
    web::scope("")
        .service(health_protected)
}

#[get("/health_protected")]
pub async fn health_protected() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}