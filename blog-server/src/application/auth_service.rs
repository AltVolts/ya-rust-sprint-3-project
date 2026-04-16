use std::sync::Arc;

use tracing::instrument;

use crate::application::error::AppError;
use crate::data::user_repository::UserRepository;
use crate::domain::DomainError;
use crate::domain::user::User;
use crate::infrastructure::security::{JwtService, hash_password, verify_password};

#[derive(Clone)]
pub struct AuthService<R: UserRepository + 'static> {
    repo: Arc<R>,
    jwt_service: JwtService,
}

impl<R> AuthService<R>
where
    R: UserRepository + 'static,
{
    pub fn new(repo: Arc<R>, jwt_service: JwtService) -> Self {
        Self { repo, jwt_service }
    }

    pub fn jwt_service(&self) -> &JwtService {
        &self.jwt_service
    }

    #[warn(dead_code)]
    pub async fn get_user(&self, id: uuid::Uuid) -> Result<User, AppError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| DomainError::UserNotFound(format!("user {}", id)))
            .map_err(AppError::from)
    }

    #[instrument(skip(self, password), fields(username))]
    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(User, String), AppError> {
        if password.is_empty() {
            return Err(DomainError::Validation("password is empty".to_string()).into());
        }
        let hash = hash_password(&password).map_err(|err| AppError::Hash(err.to_string()))?;
        let user = User::new(username, email.to_lowercase(), hash)?;
        let user = self.repo.create(user).await.map_err(AppError::from)?;
        tracing::debug!(user_id = %user.id, "user saved to database");

        let jwt_token = self
            .jwt_service
            .generate_token(user.id, &user.username)
            .map_err(|err| AppError::Internal(err.to_string()))?;
        Ok((user, jwt_token))
    }

    #[instrument(skip(self))]
    pub async fn login(&self, username: &str, password: &str) -> Result<String, AppError> {
        let user = self
            .repo
            .find_by_username(username)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::Unauthorized)?;

        let valid =
            verify_password(password, &user.password_hash).map_err(|_| AppError::Unauthorized)?;
        if !valid {
            return Err(AppError::Unauthorized);
        }

        self.jwt_service
            .generate_token(user.id, &user.username)
            .map_err(|err| AppError::Internal(err.to_string()))
    }
}
