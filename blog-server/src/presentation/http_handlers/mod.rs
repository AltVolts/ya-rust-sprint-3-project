use serde::Serialize;

pub mod public;
pub mod protected;

pub use public::scope as public_scope;
pub use protected::scope as protected_scope;


#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
}
