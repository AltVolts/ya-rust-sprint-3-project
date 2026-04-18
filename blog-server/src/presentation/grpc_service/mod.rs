use crate::application::AppError;
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain;
use crate::domain::{CreatePost, DomainError, UpdatePost};
use blog_proto::blog_service_server::BlogService as BlogServiceServer;
use blog_proto::{
    CreatePostRequest, DeletePostRequest, DeletePostResponse, GetPostRequest, ListPostsRequest,
    ListPostsResponse, LoginRequest, LoginResponse, Post, RegisterRequest, RegisterResponse,
    UpdatePostRequest, User,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Clone)]
pub struct BlogServiceImpl {
    auth_serv: Arc<AuthService<PostgresUserRepository>>,
    blog_serv: Arc<BlogService<PostgresPostRepository>>,
}

impl BlogServiceImpl {
    pub fn new(
        auth_serv: Arc<AuthService<PostgresUserRepository>>,
        blog_serv: Arc<BlogService<PostgresPostRepository>>,
    ) -> Self {
        Self {
            auth_serv,
            blog_serv,
        }
    }
}

#[tonic::async_trait]
impl BlogServiceServer for BlogServiceImpl {
    #[tracing::instrument(skip(self, request), fields(username = request.get_ref().username))]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();

        let (new_user, token) = self
            .auth_serv
            .register(req.username, req.email, req.password)
            .await
            .map_err(map_app_error)?;

        tracing::info!(
            user_id = %new_user.id,
            "gRPC user registered"
        );
        Ok(Response::new(RegisterResponse {
            token,
            user: Some(User::from(new_user)),
        }))
    }

    #[tracing::instrument(skip(self, request), fields(username = request.get_ref().username))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let (user, token) = self
            .auth_serv
            .login(&req.username, &req.password)
            .await
            .map_err(map_app_error)?;

        tracing::info!(
            username = %&req.username,
            "gRPC user login"
        );
        Ok(Response::new(LoginResponse {
            token,
            user: Some(User::from(user)),
        }))
    }

    #[tracing::instrument(skip(self, request), fields(author_id = tracing::field::Empty))]
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let jwt_serv = self.auth_serv.jwt_service().clone();
        let token_meta = get_token_meta(&request)?;

        let token = token_meta
            .strip_prefix("Bearer ")
            .ok_or(Status::unauthenticated("Invalid authorization token"))?;

        let claims = jwt_serv
            .verify_token(token)
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

        let author_id =
            Uuid::parse_str(&claims.sub).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let req = request.into_inner();

        let post = self
            .blog_serv
            .create_post(
                CreatePost {
                    title: req.title,
                    content: req.content,
                },
                author_id,
            )
            .await
            .map_err(map_app_error)?;

        Ok(Response::new(Post::from(post)))
    }

    #[tracing::instrument(skip(self, request), fields(post_id = request.get_ref().id))]
    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<Post>, Status> {
        let req = request.into_inner();
        let post_id =
            Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;
        let post = self
            .blog_serv
            .get_post(post_id)
            .await
            .map_err(map_app_error)?;

        Ok(Response::new(Post::from(post)))
    }

    #[tracing::instrument(skip(self, request), fields(post_id = request.get_ref().id, user_id = tracing::field::Empty))]
    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let jwt_serv = self.auth_serv.jwt_service().clone();
        let token_meta = get_token_meta(&request)?;

        let token = token_meta
            .strip_prefix("Bearer ")
            .ok_or(Status::unauthenticated("Invalid authorization token"))?;

        let claims = jwt_serv
            .verify_token(token)
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

        let user_id =
            Uuid::parse_str(&claims.sub).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let req = request.into_inner();
        let post_id =
            Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let updated_post = self
            .blog_serv
            .update_post(
                post_id,
                user_id,
                UpdatePost {
                    title: req.title,
                    content: req.content,
                },
            )
            .await
            .map_err(map_app_error)?;

        Ok(Response::new(Post::from(updated_post)))
    }

    #[tracing::instrument(skip(self, request), fields(post_id = request.get_ref().id, user_id = tracing::field::Empty))]
    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let jwt_serv = self.auth_serv.jwt_service().clone();
        let token_meta = get_token_meta(&request)?;

        let token = token_meta
            .strip_prefix("Bearer ")
            .ok_or(Status::unauthenticated("Invalid authorization token"))?;

        let claims = jwt_serv
            .verify_token(token)
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

        let user_id =
            Uuid::parse_str(&claims.sub).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let req = request.into_inner();
        let post_id =
            Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;

        self.blog_serv
            .delete_post(post_id, user_id)
            .await
            .map_err(map_app_error)?;

        Ok(Response::new(DeletePostResponse { success: true }))
    }

    #[tracing::instrument(skip(self, request), fields(limit = request.get_ref().limit, offset = request.get_ref().offset))]
    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();

        let paginated_posts = self
            .blog_serv
            .list_posts(req.limit, req.offset)
            .await
            .map_err(map_app_error)?;

        Ok(Response::new(ListPostsResponse {
            posts: paginated_posts
                .posts
                .into_iter()
                .map(|post| post.into())
                .collect(),
            total: paginated_posts.total,
            limit: req.limit,
            offset: req.offset,
        }))
    }
}

impl From<domain::Post> for Post {
    fn from(value: domain::Post) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            content: value.content,
            author_id: value.author_id.to_string(),
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.timestamp(),
        }
    }
}

impl From<domain::User> for User {
    fn from(value: domain::User) -> Self {
        Self {
            id: value.id.to_string(),
            username: value.username,
            email: value.email,
        }
    }
}

fn get_token_meta<T>(request: &Request<T>) -> Result<String, Status> {
    let token_meta = request
        .metadata()
        .get("authorization")
        .ok_or_else(|| Status::unauthenticated("Missing authorization header".to_string()))?
        .to_str()
        .map_err(|e| Status::unauthenticated(e.to_string()))?;
    Ok(token_meta.to_string())
}

fn map_app_error(err: AppError) -> Status {
    use tracing::{error, warn};
    match &err {
        AppError::Domain(DomainError::UserNotFound(msg)) => {
            warn!(%msg, "User not found");
            Status::not_found(msg)
        }
        AppError::Domain(DomainError::PostNotFound(id)) => {
            warn!(%id, "Post not found");
            Status::not_found(format!("Post not found: {}", id))
        }
        AppError::Domain(DomainError::UserAlreadyExists(name)) => {
            warn!(%name, "User already exists");
            Status::already_exists(format!("User '{}' already exists", name))
        }
        AppError::Domain(DomainError::InvalidCredentials) => {
            warn!("Invalid credentials");
            Status::unauthenticated("Invalid username or password".to_string())
        }
        AppError::Unauthorized => {
            warn!("Unauthorized access");
            Status::unauthenticated("Authentication required".to_string())
        }
        AppError::Domain(DomainError::Validation(msg)) => {
            warn!(%msg, "Validation error"); // исправлено
            Status::invalid_argument(msg.clone())
        }
        AppError::Domain(DomainError::Forbidden { user_id, post_id }) => {
            warn!(%user_id, %post_id, "User is not author of post");
            Status::permission_denied(format!(
                "User {} is not author of post {}",
                user_id, post_id
            ))
        }
        AppError::Domain(DomainError::Internal(msg)) => {
            error!(%msg, "Internal domain error");
            Status::internal(msg.clone())
        }
        AppError::Database(msg) => {
            error!(%msg, "Database error");
            Status::internal(format!("Database error: {}", msg))
        }
        AppError::Hash(msg) => {
            error!(%msg, "Hashing error");
            Status::internal(format!("Hashing error: {}", msg))
        }
        AppError::Jwt(msg) => {
            error!(%msg, "JWT error");
            Status::internal(format!("JWT error: {}", msg))
        }
        AppError::Config(msg) => {
            error!(%msg, "Config error");
            Status::internal(format!("Config error: {}", msg))
        }
        AppError::Internal(msg) => {
            error!(%msg, "Internal server error");
            Status::internal(msg.clone())
        }
    }
}
