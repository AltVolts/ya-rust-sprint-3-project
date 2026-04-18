use crate::domain::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<Self, DomainError> {
        if username.is_empty() {
            return Err(DomainError::Validation("username is empty".to_string()));
        }
        if email.is_empty() {
            return Err(DomainError::Validation("email is empty".to_string()));
        }

        Ok(Self {
            id: Uuid::now_v7(),
            username,
            email,
            password_hash,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}
