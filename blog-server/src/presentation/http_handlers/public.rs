use crate::presentation::http_handlers::HealthResponse;
use actix_web::{HttpResponse, Responder, Scope, get, web};
use chrono::Utc;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}
