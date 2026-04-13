use crate::presentation::http_handlers::HealthResponse;
use actix_web::{HttpResponse, Responder, Scope, get, web, post, HttpRequest, HttpMessage};
use chrono::Utc;
use tracing::info;
use crate::application::AppError;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::CreatePost;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::RequestId;

pub fn scope() -> Scope {
    web::scope("")
        .service(health_protected)
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
    payload: web::Json<CreatePost>
) -> Result<HttpResponse, AppError> {
    let post = service.create_post(payload.into_inner(), user.id).await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = %post.id,
        "account created"
    );

    Ok(HttpResponse::Created().json(post))
}

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}