// use crate::domain::{User, RegisterUser, LoginUser, DomainError};
// use crate::application::AppError;
// use async_trait::async_trait;
// use uuid::Uuid;
// 
// // Интерфейс репозитория пользователей (инфраструктурный слой)
// #[async_trait]
// pub trait UserRepository: Send + Sync {
//     async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
//     async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
//     async fn create(&self, user: &User) -> Result<(), AppError>;
// }
// 
// // Интерфейс хеширования паролей
// #[async_trait]
// pub trait PasswordHasher: Send + Sync {
//     async fn hash(&self, password: &str) -> Result<String, AppError>;
//     async fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
// }
// 
// #[async_trait]
// pub trait TokenService: Send + Sync {
//     async fn generate_token(&self, user_id: Uuid, username: &str) -> Result<String, AppError>;
// }
// 
// // Сервис аутентификации
// pub struct AuthService<R, H, T> {
//     user_repo: R,
//     hasher: H,
//     token_service: T,
// }
// 
// impl<R, H, T> AuthService<R, H, T> {
//     pub fn new(user_repo: R, hasher: H, token_service: T) -> Self {
//         Self {
//             user_repo,
//             hasher,
//             token_service,
//         }
//     }
// }
// 
// impl<R, H, T> AuthService<R, H, T>
// where
//     R: UserRepository + Send + Sync,
//     H: PasswordHasher + Send + Sync,
//     T: TokenService + Send + Sync,
// {
//     /// Регистрация нового пользователя
//     pub async fn register(&self, req: RegisterUser) -> Result<String, AppError> {
//         // Проверка, не занят ли username или email
//         if self.user_repo.find_by_username(&req.username).await?.is_some() {
//             return Err(DomainError::UserAlreadyExists(format!("username: {}", req.username)).into());
//         }
//         if self.user_repo.find_by_email(&req.email).await?.is_some() {
//             return Err(DomainError::UserAlreadyExists(format!("email: {}", req.email)).into());
//         }
// 
//         // Хеширование пароля
//         let password_hash = self.hasher.hash(&req.password).await?;
// 
//         // Создание пользователя (id будет сгенерирован БД, пока 0)
//         let new_id = Uuid::now_v7();
//         let user = User {
//             id: new_id,
//             username: req.username,
//             email: req.email,
//             password_hash,
//             created_at: chrono::Utc::now(),
//         };
// 
//         // Сохранение в БД (репозиторий должен вернуть ошибку или сгенерировать id)
//         self.user_repo.create(&user).await?;
// 
//         // Генерация токена (в реальности нужно получить реальный id после вставки)
//         let token = self.token_service.generate_token(new_id, &user.username).await?;
//         Ok(token)
//     }
// 
//     /// Вход пользователя
//     pub async fn login(&self, req: LoginUser) -> Result<String, AppError> {
//         let user = self.user_repo
//             .find_by_username(&req.username)
//             .await?
//             .ok_or_else(|| DomainError::UserNotFound(req.username.clone()))?;
// 
//         let verified = self.hasher.verify(&req.password, &user.password_hash).await?;
//         if !verified {
//             return Err(DomainError::InvalidCredentials.into());
//         }
// 
//         let token = self.token_service.generate_token(user.id, &user.username).await?;
//         Ok(token)
//     }
// }