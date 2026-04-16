use crate::application::AppError;
use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::{LoginUser, RegisterUser};
use crate::presentation::http_handlers::{HealthResponse, request_id};
use actix_web::{HttpRequest, HttpResponse, Responder, Scope, get, post, web};
use chrono::Utc;
use tracing::info;

pub fn auth_scope() -> Scope {
    web::scope("/auth").service(register).service(login)
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[post("/register")]
pub async fn register(
    req: HttpRequest,
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<RegisterUser>,
) -> Result<impl Responder, AppError> {
    let register_data = payload.into_inner();
    let (user, jwt_token) = service
        .register(
            register_data.username,
            register_data.email,
            register_data.password,
        )
        .await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        "user registered"
    );
    Ok(HttpResponse::Created().json(serde_json::json!({
        "token": jwt_token,
        "username": user.username,
    })))
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<LoginUser>,
) -> Result<impl Responder, AppError> {
    let login_data = payload.into_inner();
    let jwt_token = service
        .login(&login_data.username, &login_data.password)
        .await?;

    info!(
        request_id = %request_id(&req),
        username = %login_data.username,
        "user logged in"
    );
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": jwt_token,
        "user": login_data.username,
    })))
}
