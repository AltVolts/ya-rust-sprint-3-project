use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("User already exists: {0}")]
    UserAlreadyExists(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Post not found: {0}")]
    PostNotFound(Uuid),

    #[error("Forbidden: user {user_id} is not the author of post {post_id}")]
    Forbidden { user_id: Uuid, post_id: Uuid },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal domain error: {0}")]
    Internal(String),
}
