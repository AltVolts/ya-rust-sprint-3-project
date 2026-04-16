use crate::presentation::RequestId;
use actix_web::{HttpMessage, HttpRequest};
use serde::Serialize;

pub mod auth;
pub mod posts;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
}

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}
