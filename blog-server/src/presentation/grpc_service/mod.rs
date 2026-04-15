use crate::application::AppError;
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain;
use crate::domain::{CreatePost, DomainError, PaginatedPosts, UpdatePost};
use blog_proto::blog_service_server::BlogService as BlogServiceServer;
use blog_proto::{
    CreatePostRequest, DeletePostRequest, DeletePostResponse, GetPostRequest,
    ListPostsRequest, ListPostsResponse, LoginRequest, LoginResponse, Post,
    RegisterRequest, UpdatePostRequest, User,
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
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<User>, Status> {
        let req = request.into_inner();

        let (new_user, _) = self
            .auth_serv
            .register(req.username, req.email, req.password)
            .await
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(User {
            id: new_user.id.to_string(),
            username: new_user.username,
            email: new_user.email,
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let access_token = self
            .auth_serv
            .login(&req.username, &req.password)
            .await
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(LoginResponse { access_token }))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let jwt_serv = self.auth_serv.jwt_service().clone();
        let token_meta = request
            .metadata()
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization header".to_string()))?
            .to_str()
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

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
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(Post::from(post)))
    }

    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<Post>, Status> {
        let req = request.into_inner();
        let post_id =
            Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;
        let post = self
            .blog_serv
            .get_post(post_id)
            .await
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(Post::from(post)))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let jwt_serv = self.auth_serv.jwt_service().clone();
        let token_meta = request
            .metadata()
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization header".to_string()))?
            .to_str()
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

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
                }
            )
            .await
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(Post::from(updated_post)))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let jwt_serv = self.auth_serv.jwt_service().clone();
        let token_meta = request
            .metadata()
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization header".to_string()))?
            .to_str()
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

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
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(DeletePostResponse {success: true}))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();

        let paginated_posts = self
            .blog_serv
            .list_posts(req.limit, req.offset)
            .await
            .map_err(|e| map_app_error(e))?;

        Ok(Response::new(ListPostsResponse::from(paginated_posts)))
    }
}

impl From<PaginatedPosts> for ListPostsResponse {
    fn from(value: PaginatedPosts) -> Self {
        Self {
            posts: value.posts.into_iter().map(|post| post.into()).collect(),
            total: value.total,
        }
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

fn map_app_error(err: AppError) -> Status {
    match err {
        AppError::Domain(DomainError::UserNotFound(msg)) => Status::not_found(msg),
        AppError::Domain(DomainError::PostNotFound(id)) => {
            Status::not_found(format!("Post not found: {}", id))
        }
        AppError::Domain(DomainError::UserAlreadyExists(name)) => {
            Status::already_exists(format!("User '{}' already exists", name))
        }
        AppError::Domain(DomainError::InvalidCredentials) => {
            Status::unauthenticated("Invalid username or password".to_string())
        }
        AppError::Domain(DomainError::Validation(msg)) => Status::invalid_argument(msg),
        AppError::Domain(DomainError::Forbidden { user_id, post_id }) => Status::permission_denied(
            format!("User {} is not author of post {}", user_id, post_id),
        ),
        AppError::Domain(DomainError::Internal(msg)) => Status::internal(msg),
        AppError::Unauthorized => Status::unauthenticated("Authentication required".to_string()),
        AppError::Database(msg) => Status::internal(format!("Database error: {}", msg)),
        AppError::Hash(msg) => Status::internal(format!("Hashing error: {}", msg)),
        AppError::Jwt(msg) => Status::internal(format!("JWT error: {}", msg)),
        AppError::Config(msg) => Status::internal(format!("Config error: {}", msg)),
        AppError::Internal(msg) => Status::internal(msg),
    }
}
