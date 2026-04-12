use crate::application::AppError;
use crate::domain::DomainError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError, http};
use serde_json::json;

impl ResponseError for AppError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            AppError::Domain(domain_err) => match domain_err {
                DomainError::UserNotFound(_) => StatusCode::NOT_FOUND,
                DomainError::UserAlreadyExists(_) => StatusCode::CONFLICT,
                DomainError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                DomainError::PostNotFound(_) => StatusCode::NOT_FOUND,
                DomainError::Forbidden { .. } => StatusCode::FORBIDDEN,
                DomainError::Validation(_) => StatusCode::BAD_REQUEST,
                DomainError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Hash(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Jwt(_) => StatusCode::UNAUTHORIZED,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            &AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_message = self.to_string();

        let client_message = match self {
            AppError::Database(_)
            | AppError::Hash(_)
            | AppError::Config(_)
            | AppError::Internal(_) => "Internal server error".to_string(),
            _ => error_message,
        };

        HttpResponse::build(status).json(json!({
            "error": client_message,
            "status": status.as_u16(),
        }))
    }
}
