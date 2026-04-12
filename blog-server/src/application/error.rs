use std::fmt;
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

// impl AppError {
//     pub fn database<E: fmt::Display>(err: E) -> Self {
//         AppError::Database(err.to_string())
//     }
//
//     pub fn hash<E: fmt::Display>(err: E) -> Self {
//         AppError::Hash(err.to_string())
//     }
//
//     pub fn jwt<E: fmt::Display>(err: E) -> Self {
//         AppError::Jwt(err.to_string())
//     }
// }
