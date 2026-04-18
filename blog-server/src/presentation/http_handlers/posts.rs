use crate::application::AppError;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::{CreatePost, GetListPosts, UpdatePost};

use crate::presentation::http_handlers::request_id;
use crate::presentation::middleware::jwt::AuthenticatedUser;
use actix_web::{HttpRequest, HttpResponse, web};
use tracing::info;
use uuid::Uuid;

pub async fn create_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    service: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Json<CreatePost>,
) -> Result<HttpResponse, AppError> {
    let post = service.create_post(payload.into_inner(), user.id).await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %post.id,
        "post created"
    );
    Ok(HttpResponse::Created().json(post))
}

pub async fn get_post(
    req: HttpRequest,
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post = service.get_post(path.into_inner()).await?;

    info!(
        request_id = %request_id(&req),
        post_id = %post.id,
        "post retrieved via HTTP"
    );
    Ok(HttpResponse::Ok().json(post))
}

pub async fn list_posts(
    req: HttpRequest,
    service: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Query<GetListPosts>,
) -> Result<HttpResponse, AppError> {
    let list_parameters = payload.into_inner();
    let limit = list_parameters.limit;
    let offset = list_parameters.offset;
    let paginated_posts = service.list_posts(limit, offset).await?;

    info!(
        request_id = %request_id(&req),
        limit = limit,
        offset = offset,
        total = paginated_posts.total,
        "posts_list retrieved via HTTP"
    );

    Ok(HttpResponse::Ok().json(serde_json::json!(paginated_posts)))
}

pub async fn update_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdatePost>,
) -> Result<HttpResponse, AppError> {
    let post = service
        .update_post(path.into_inner(), user.id, payload.into_inner())
        .await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %post.id,
        "post updated"
    );
    Ok(HttpResponse::Ok().json(post))
}

pub async fn delete_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.delete_post(post_id, user.id).await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %post_id,
        "post deleted"
    );
    Ok(HttpResponse::NoContent().finish())
}
