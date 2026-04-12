use std::cell::RefCell;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};

use actix_web::{ Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;
use crate::infrastructure::security::JwtService;
use crate::presentation::auth::AuthenticatedUser;

pub struct JwtAuthMiddleware {
    jwt_service: JwtService,
}

impl JwtAuthMiddleware {
    pub fn new(jwt_service: JwtService) -> Self {
        Self { jwt_service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthService {
            inner_service: Rc::new(RefCell::new(service)),
            jwt_service: self.jwt_service.clone(),
        }))
    }
}

pub struct JwtAuthService<S> {
    inner_service: Rc<RefCell<S>>,
    jwt_service: JwtService,
}

impl<S, B> Service<ServiceRequest> for JwtAuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner_service.borrow_mut().poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_service = self.jwt_service.clone();
        let inner_service = Rc::clone(&self.inner_service);


        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());

        Box::pin(async move {
            let header = auth_header
                .ok_or_else(|| ErrorUnauthorized("missing authorization header"))?;
            let token = header
                .strip_prefix("Bearer ")
                .ok_or_else(|| ErrorUnauthorized("invalid authorization header"))?;

            let claims = jwt_service
                .verify_token(token)
                .map_err(|_| ErrorUnauthorized("invalid token"))?;

            let user_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| ErrorUnauthorized("invalid token"))?;
            let username= claims.username;

            let user: AuthenticatedUser = AuthenticatedUser {
                id: user_id,
                username,
            };
            req.extensions_mut().insert(user);
            let fut = {
                let svc = inner_service.borrow_mut();
                svc.call(req)
            };
            let res = fut.await?;
            Ok(res)
        })
    }
}