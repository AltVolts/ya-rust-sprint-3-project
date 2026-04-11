use crate::application::AppError;
use crate::domain::DomainError;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Domain(domain_err) => match domain_err {
                DomainError::UserNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
                DomainError::UserAlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,
                DomainError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
                DomainError::PostNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
                DomainError::Forbidden { .. } => actix_web::http::StatusCode::FORBIDDEN,
                DomainError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
                DomainError::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::Database(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Hash(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Jwt(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Config(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_message = self.to_string();

        let client_message = match self {
            AppError::Database(_)
            | AppError::Hash(_)
            | AppError::Config(_)
            | AppError::Internal => "Internal server error".to_string(),
            _ => error_message,
        };

        HttpResponse::build(status).json(json!({
            "error": client_message,
            "status": status.as_u16(),
        }))
    }
}
