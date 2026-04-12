use crate::application::AppError;
use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::{LoginUser, RegisterUser};
use crate::presentation::http_handlers::HealthResponse;
use actix_web::{HttpResponse, Responder, get, post, web};
use chrono::Utc;
use tracing::info;

// pub fn scope() -> Scope {
//     web::scope("/auth").service(register)
// }

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[post("/auth/register")]
pub async fn register(
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

    info!(user_id = %user.id, username = %user.username, email = %user.email, "user registered");
    Ok(HttpResponse::Created().json(serde_json::json!({
        "token": jwt_token,
        "username": user.username,
    })))
}

#[post("/auth/login")]
pub async fn login(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<LoginUser>,
) -> Result<impl Responder, AppError> {
    let login_data = payload.into_inner();
    let jwt_token = service
        .login(&login_data.username, &login_data.password)
        .await?;
    info!(username = %login_data.username, "user logged in");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": jwt_token,
        "user": login_data.username,
    })))
}
