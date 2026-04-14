use crate::infrastructure::security::JwtService;
use actix_web::dev::{Payload, ServiceRequest};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, HttpMessage, web, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{ready, Ready};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<AuthenticatedUser>() {
            Some(user) => ready(Ok(user.clone())),
            None => ready(Err(ErrorUnauthorized("missing authenticated user"))),
        }
    }
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = match req.app_data::<web::Data<JwtService>>() {
        Some(service) => service,
        None => {
            let err = ErrorInternalServerError("JwtService not found");
            return Err((err, req));
        }
    };

    let token = credentials.token();
    let claims = match jwt_service.verify_token(token) {
        Ok(claims) => claims,
        Err(_) => {
            let err = ErrorUnauthorized("Invalid token");
            return Err((err, req));
        }
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            let err = ErrorUnauthorized("Invalid token subject");
            return Err((err, req));
        }
    };

    let user = AuthenticatedUser {
        id: user_id,
        username: claims.username,
    };
    req.extensions_mut().insert(user);
    Ok(req)
}
