use serde::Serialize;

pub mod protected;
pub mod public;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
}
