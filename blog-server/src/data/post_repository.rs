use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{DomainError, PaginatedPosts, Post};

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        DomainError::Internal(err.to_string())
    }
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError>;
    async fn update(&self, post: Post) -> Result<Post, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list_paginated(&self, limit: i64, offset: i64) -> Result<PaginatedPosts, DomainError>;
}

#[derive(Clone)]
pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self { Self{ pool } }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, post: Post) -> Result<Post, DomainError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            post.id,
            post.title,
            post.content,
            post.author_id,
            post.created_at,
            post.updated_at,
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(post)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(post)
    }

    async fn update(&self, mut post: Post) -> Result<Post, DomainError> {
        post.updated_at = chrono::Utc::now();
        let updated = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $1, content = $2, author_id = $3, updated_at = $4
            WHERE id = $5
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            post.title,
            post.content,
            post.author_id,
            post.updated_at,
            post.id,
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| DomainError::PostNotFound(post.id))?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let rows_affected = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::PostNotFound(id));
        }
        Ok(())
    }

    async fn list_paginated(&self, limit: i64, offset: i64) -> Result<PaginatedPosts, DomainError> {
        let mut tx = self.pool.begin().await?;
        let total = sqlx::query_scalar!("SELECT COUNT(*) FROM posts")
            .fetch_one(&mut *tx)
            .await?
            .unwrap_or(0);

        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
            .fetch_all(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(PaginatedPosts { posts, total})
    }
}