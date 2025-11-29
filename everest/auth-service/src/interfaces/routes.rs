// src/interfaces/routes.rs
use crate::interfaces::handlers::{auth_handlers, user_handlers};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Authentication routes (public)
            .route("/auth/login", web::post().to(auth_handlers::login))
            .route(
                "/auth/validate",
                web::post().to(auth_handlers::validate_token),
            )
            .route(
                "/auth/refresh",
                web::post().to(auth_handlers::refresh_token),
            )
            .route("/auth/logout", web::post().to(auth_handlers::logout))
            // User routes (protected)
            .route("/users", web::post().to(user_handlers::create_user))
            .route("/users", web::get().to(user_handlers::list_users))
            .route("/users/{user_id}", web::get().to(user_handlers::get_user))
            .route(
                "/users/username/{username}",
                web::get().to(user_handlers::get_user_by_username),
            )
            .route(
                "/users/{user_id}/enable",
                web::put().to(user_handlers::enable_user),
            )
            .route(
                "/users/{user_id}/disable",
                web::put().to(user_handlers::disable_user),
            )
            .route(
                "/users/{user_id}",
                web::delete().to(user_handlers::delete_user),
            )
            // Role routes (protected)
            .route(
                "/users/{user_id}/roles",
                web::post().to(user_handlers::assign_role),
            )
            .route(
                "/users/{user_id}/roles",
                web::get().to(user_handlers::get_user_roles),
            ),
    );
}
