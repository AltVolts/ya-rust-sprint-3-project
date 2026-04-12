use actix_web::{web, Scope};

mod error;
pub mod http_handlers;
mod middleware;
pub mod auth;

pub use error::*;
pub use middleware::*;
use crate::presentation::http_handlers::{public_scope, protected_scope};



pub fn configure_handlers_scopes(jwt_middleware: JwtAuthMiddleware) -> Scope {
    let pub_scope = public_scope();
    let protected_scope = protected_scope();

    let scope: Scope = web::scope("/api")
        .service(pub_scope)
        .service(
            web::scope("")
                .wrap(jwt_middleware)
                .service(protected_scope)
        );
    scope
}

// pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
//     let pub_scope = public_scope();
//     let protected_scope = protected_scope();
//     cfg
//         .service(
//             web::scope("/api")
//                 .service(pub_scope)
//                 .service(
//                     web::scope("")
//                         .wrap(jwt_middleware)
//                         .service(protected_scope)
//         )
//
//     ;
// }