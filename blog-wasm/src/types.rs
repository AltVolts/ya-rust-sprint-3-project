use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostsListResponse {
    pub posts: Vec<Post>,
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
}

// Ошибки API (упрощённо)
#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    BadRequest(String),
    ServerError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Не авторизован"),
            ApiError::Forbidden => write!(f, "Доступ запрещён"),
            ApiError::NotFound => write!(f, "Не найдено"),
            ApiError::Conflict => write!(f, "Конфликт (пользователь уже существует)"),
            ApiError::BadRequest(msg) => write!(f, "Ошибка запроса: {}", msg),
            ApiError::ServerError(msg) => write!(f, "Ошибка сервера: {}", msg),
        }
    }
}
