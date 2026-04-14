use crate::application::AppError;
use crate::data::post_repository::PostRepository;
use crate::domain::{CreatePost, DomainError, Post, UpdatePost};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BlogService<R: PostRepository + 'static> {
    post_repo: Arc<R>,
}

impl<R> BlogService<R>
where
    R: PostRepository + 'static,
{
    pub fn new(post_repo: Arc<R>) -> Self {
        Self { post_repo }
    }

    pub async fn create_post(&self, create: CreatePost, author_id: Uuid) -> Result<Post, AppError> {
        let post = Post::new(Uuid::now_v7(), create.title, create.content, author_id);
        Ok(self.post_repo.create(post).await?)
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post, AppError> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))?;
        Ok(post)
    }

    pub async fn update_post(
        &self,
        post_id: Uuid,
        user_id: Uuid,
        update: UpdatePost,
    ) -> Result<Post, AppError> {
        let mut post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))?;

        if post.author_id != user_id {
            return Err(DomainError::Forbidden { user_id, post_id }.into());
        }

        if let Some(title) = update.title {
            post.title = title;
        }
        if let Some(content) = update.content {
            post.content = content;
        }
        Ok(self.post_repo.update(post).await?)
    }

    pub async fn delete_post(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))?;

        if post.author_id != user_id {
            return Err(DomainError::Forbidden { user_id, post_id }.into());
        }

        Ok(self.post_repo.delete(post_id).await?)
    }

    pub async fn list_posts(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<crate::domain::PaginatedPosts, AppError> {
        let limit = limit.clamp(1, 100);
        let offset = offset.max(0);
        Ok(self.post_repo.list_paginated(limit, offset).await?)
    }
}
