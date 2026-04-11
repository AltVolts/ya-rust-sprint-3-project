use super::DomainError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(id: Uuid, title: String, content: String, author_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            content,
            author_id,
            created_at: now,
            updated_at: now,
        }
    }
}

pub struct CreatePost {
    pub title: String,
    pub content: String,
}

impl CreatePost {
    pub fn new(title: String, content: String) -> Result<Self, DomainError> {
        if title.is_empty() {
            return Err(DomainError::Validation("Title is empty".to_string()));
        }
        if content.is_empty() {
            return Err(DomainError::Validation("Content is empty".to_string()));
        }
        Ok(Self { title, content })
    }
}

pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl UpdatePost {
    pub fn new(title: Option<String>, content: Option<String>) -> Result<Self, DomainError> {
        if title.is_none() && content.is_none() {
            return Err(DomainError::Validation(
                "Title and content are empty".to_string(),
            ));
        }

        Ok(Self { title, content })
    }
}
