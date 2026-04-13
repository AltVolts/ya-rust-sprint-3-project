use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::{web, Error, HttpMessage};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use uuid::Uuid;
use crate::infrastructure::security::JwtService;
use crate::presentation::auth::AuthenticatedUser;

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = match req.app_data::<web::Data<JwtService>>() {
        Some(service) => service,
        None => {
            let err = ErrorInternalServerError("JwtService not found");
            return Err((err.into(), req));
        }
    };

    let token = credentials.token();
    let claims = match jwt_service.verify_token(token) {
        Ok(claims) => claims,
        Err(_) => {
            let err = ErrorUnauthorized("Invalid token");
            return Err((err.into(), req));
        }
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            let err = ErrorUnauthorized("Invalid token subject");
            return Err((err.into(), req));
        }
    };

    let user = AuthenticatedUser {
        id: user_id,
        username: claims.username,
    };
    req.extensions_mut().insert(user);
    Ok(req)
}