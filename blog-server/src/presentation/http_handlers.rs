use actix_web::{HttpResponse, Responder, Scope, get, web};
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[get("/health_protected")]
pub async fn health_protected() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope: Scope = web::scope("/api").service(health).service(health_protected);
    cfg.service(scope);
}
