use crate::application::AppError;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::{CreatePost, GetListPosts, UpdatePost};
use crate::presentation::RequestId;

use crate::presentation::middleware::jwt::AuthenticatedUser;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
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
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post = service.get_post(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(post))
}

pub async fn list_posts(
    service: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Json<GetListPosts>,
) -> Result<HttpResponse, AppError> {
    let list_parameters = payload.into_inner();
    let limit = list_parameters.limit;
    let offset = list_parameters.offset;
    let post_list = service.list_posts(limit, offset).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "posts": post_list.posts,
        "total": post_list.total,
        "limit": limit,
        "offset": offset
    })))
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

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}
