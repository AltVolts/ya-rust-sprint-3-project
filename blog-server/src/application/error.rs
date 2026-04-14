use thiserror::Error;

use crate::domain::DomainError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Hashing error: {0}")]
    Hash(String),

    #[error("JWT error: {0}")]
    Jwt(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal server error")]
    Internal(String),

    #[error("unauthorized")]
    Unauthorized,
}
