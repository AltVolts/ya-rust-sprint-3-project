// use crate::domain::{Post, CreatePost, UpdatePost, DomainError};
// use crate::application::AppError;
// use async_trait::async_trait;
//
// // Интерфейс репозитория постов
// #[async_trait]
// pub trait PostRepository: Send + Sync {
//     async fn find_by_id(&self, id: i32) -> Result<Option<Post>, AppError>;
//     async fn find_by_author(&self, author_id: i32) -> Result<Vec<Post>, AppError>;
//     async fn create(&self, post: &Post) -> Result<Post, AppError>; // возвращает пост с id
//     async fn update(&self, post: &Post) -> Result<(), AppError>;
//     async fn delete(&self, id: i32) -> Result<(), AppError>;
// }
//
// // Сервис блога
// pub struct BlogService<R> {
//     post_repo: R,
// }
//
// impl<R> BlogService<R> {
//     pub fn new(post_repo: R) -> Self {
//         Self { post_repo }
//     }
// }
//
// #[async_trait]
// impl<R> BlogService<R>
// where
//     R: PostRepository + Send + Sync,
// {
//     /// Создание нового поста
//     pub async fn create_post(&self, author_id: i32, req: CreatePost) -> Result<Post, AppError> {
//         // Валидация доменных правил (можно вынести в доменный метод Post::validate)
//         if req.title.is_empty() {
//             return Err(DomainError::Validation("Title cannot be empty".into()).into());
//         }
//         if req.content.is_empty() {
//             return Err(DomainError::Validation("Content cannot be empty".into()).into());
//         }
//
//         let mut post = Post::new(req.title, req.content, author_id);
//         // Репозиторий создаст запись и вернёт пост с присвоенным id
//         let created_post = self.post_repo.create(&post).await?;
//         Ok(created_post)
//     }
//
//     /// Обновление поста (только для автора)
//     pub async fn update_post(&self, post_id: i32, author_id: i32, req: UpdatePost) -> Result<Post, AppError> {
//         let mut post = self.post_repo
//             .find_by_id(post_id)
//             .await?
//             .ok_or_else(|| DomainError::PostNotFound(post_id))?;
//
//         // Проверка прав
//         if post.author_id != author_id {
//             return Err(DomainError::Forbidden { user_id: author_id, post_id }.into());
//         }
//
//         // Обновление полей
//         post.title = req.title;
//         post.content = req.content;
//         post.updated_at = chrono::Utc::now();
//
//         self.post_repo.update(&post).await?;
//         Ok(post)
//     }
//
//     /// Удаление поста (только для автора)
//     pub async fn delete_post(&self, post_id: i32, author_id: i32) -> Result<(), AppError> {
//         let post = self.post_repo
//             .find_by_id(post_id)
//             .await?
//             .ok_or_else(|| DomainError::PostNotFound(post_id))?;
//
//         if post.author_id != author_id {
//             return Err(DomainError::Forbidden { user_id: author_id, post_id }.into());
//         }
//
//         self.post_repo.delete(post_id).await?;
//         Ok(())
//     }
//
//     /// Получение поста по ID (без проверки прав)
//     pub async fn get_post(&self, post_id: i32) -> Result<Post, AppError> {
//         self.post_repo
//             .find_by_id(post_id)
//             .await?
//             .ok_or_else(|| DomainError::PostNotFound(post_id).into())
//     }
//
//     /// Получение всех постов автора
//     pub async fn get_user_posts(&self, author_id: i32) -> Result<Vec<Post>, AppError> {
//         self.post_repo.find_by_author(author_id).await
//     }
// }