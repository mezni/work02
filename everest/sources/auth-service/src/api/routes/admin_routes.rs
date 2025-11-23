// src/api/routes/admin_routes.rs
use actix_web::web;
use crate::api::handlers::AdminHandler;

pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            .service(
                web::scope("/users")
                    .route("", web::get().to(AdminHandler::get_users_admin))
                    .route("/{id}/deactivate", web::post().to(AdminHandler::deactivate_user))
            ),
    );
}