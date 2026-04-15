use serde::Serialize;

pub mod auth;
pub mod posts;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
}
