use crate::error::BlogClientError;
use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Display)]
#[display("{username} ({email})")]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
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

#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
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

#[derive(Debug, Clone, Deserialize, Display)]
#[display(
    "\
ID: {id}
Title: {title}
Content: {content}
Author ID: {author_id}
Created at: {created_at}
Updated at: {updated_at}
"
)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedPosts {
    pub posts: Vec<Post>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
