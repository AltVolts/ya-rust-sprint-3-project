use crate::application::AppError;
use crate::data::post_repository::PostRepository;
use crate::domain::{CreatePost, DomainError, PaginatedPosts, Post, UpdatePost};
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

    #[tracing::instrument(skip(self))]
    pub async fn create_post(&self, create: CreatePost, author_id: Uuid) -> Result<Post, AppError> {
        let post = Post::new(Uuid::now_v7(), create.title, create.content, author_id);
        let post = self.post_repo.create(post).await?;

        tracing::debug!(post_id = %post.id, "post created in repository");
        Ok(post)
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post, AppError> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))?;

        tracing::debug!(post_id = %post.id, "post retrieved from repository");
        Ok(post)
    }

    #[tracing::instrument(skip(self))]
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
        let updated = self.post_repo.update(post).await?;

        tracing::debug!(post_id = %updated.id, "post updated in repository");
        Ok(self.post_repo.update(updated).await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_post(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))?;

        if post.author_id != user_id {
            return Err(DomainError::Forbidden { user_id, post_id }.into());
        }
        self.post_repo.delete(post_id).await?;

        tracing::debug!(post_id = %post.id, "post deleted in repository");
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<PaginatedPosts, AppError> {
        let limit = limit.clamp(1, 100);
        let offset = offset.max(0);
        let post_list = self.post_repo.list_paginated(limit, offset).await?;

        tracing::debug!(
            limit,
            offset,
            total = post_list.total,
            "posts listed from repository"
        );
        Ok(post_list)
    }
}
