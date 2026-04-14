use crate::application::AppError;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::{CreatePost, GetListPosts, UpdatePost};
use crate::presentation::RequestId;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::http_handlers::HealthResponse;
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, Scope, delete, get, post, put, web,
};
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

pub fn scope() -> Scope {
    web::scope("")
        .service(health_protected)
        .service(create_post)
        .service(get_post)
        .service(list_posts)
        .service(update_post)
        .service(delete_post)
}

#[get("/health_protected")]
pub async fn health_protected() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[post("/posts")]
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

#[get("/posts/{id}")]
pub async fn get_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post = service.get_post(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(post))
}

#[get("/posts")]
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

#[put("/posts/{id}")]
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

#[delete("/posts/{id}")]
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
