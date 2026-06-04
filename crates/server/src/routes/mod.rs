use actix_web::web;

use crate::handlers::{health, users};

/// Register all application routes under their respective scopes.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Infrastructure
        .service(health::health)
        // API v1
        .service(
            web::scope("/api/v1")
                .service(users::create_user)
                .service(users::list_users)
                .service(users::get_user),
        );
}
