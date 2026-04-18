use crate::error::BlogClientError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedPosts {
    pub posts: Vec<Post>,
    pub total: i64,
}

impl TryFrom<blog_proto::Post> for Post {
    type Error = BlogClientError;

    fn try_from(value: blog_proto::Post) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            title: value.title,
            content: value.content,
            author_id: value.author_id,
            created_at: DateTime::from_timestamp(value.created_at, 0).ok_or(
                BlogClientError::InvalidResponse(
                    "Unrecognized datetime from timestamp".to_string(),
                ),
            )?,
            updated_at: DateTime::from_timestamp(value.updated_at, 0).ok_or(
                BlogClientError::InvalidResponse(
                    "Unrecognized datetime from timestamp".to_string(),
                ),
            )?,
        })
    }
}

impl From<blog_proto::User> for User {
    fn from(value: blog_proto::User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}
