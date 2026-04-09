use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {

        Self {
            id: Uuid::now_v7(),
            username,
            email,
            password_hash,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl RegisterUser {
    pub fn new(username: String, email: String, password: String) -> Result<Self, DomainError> {
        if username.is_empty() || email.is_empty() {
            return Err(DomainError::Validation("username or email cannot be empty".to_string()));
        }
        if password.len() < 8 {
            return Err(DomainError::Validation("password must be at least 8 characters".to_string()));
        }
        if username.len() > 255 || email.len() > 255 {
            return Err(DomainError::Validation("username or email cannot be more than 255".to_string()));
        }

        Ok(Self {
            username,
            email,
            password
        })
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}