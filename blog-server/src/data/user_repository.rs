use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::{DomainError, user::User};

fn map_row(row: PgRow) -> Result<User, DomainError> {
    let decode_err = |e: sqlx::Error| DomainError::Internal(format!("row decode error: {}", e));

    Ok(User {
        id: row.try_get("id").map_err(decode_err)?,
        username: row.try_get("username").map_err(decode_err)?,
        email: row.try_get("email").map_err(decode_err)?,
        password_hash: row.try_get("password_hash").map_err(decode_err)?,
        created_at: row.try_get("created_at").map_err(decode_err)?,
    })
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User, DomainError> {
        const EMAIL_UNIQ: &str = "users_email_key";
        const USERNAME_UNIQ: &str = "users_username_key";

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            let db_err = e.as_database_error();
            let domain_err = match db_err.and_then(|err| err.constraint()) {
                Some(EMAIL_UNIQ) => DomainError::UserAlreadyExists("email already exists".into()),
                Some(USERNAME_UNIQ) => {
                    DomainError::UserAlreadyExists("username already exists".into())
                }
                _ => DomainError::Internal(format!("database error: {}", e)),
            };
            if matches!(domain_err, DomainError::Internal(_)) {
                error!("failed to create user: {}", e);
            }
            domain_err
        })?;

        info!(user_id = %user.id, "user created");
        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by username {}: {}", username, e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        row.map(map_row).transpose()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by id {}: {}", id, e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        row.map(map_row).transpose()
    }
}
