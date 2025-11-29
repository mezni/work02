use crate::interfaces::handlers::{
    auth_handlers, user_handlers, organisation_handlers, audit_handlers, role_request_handlers
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Authentication routes
            .route("/auth/login", web::post().to(auth_handlers::login))
            .route("/auth/validate", web::post().to(auth_handlers::validate_token))
            .route("/auth/refresh", web::post().to(auth_handlers::refresh_token))
            .route("/auth/logout", web::post().to(auth_handlers::logout))
            
            // User routes
            .route("/users", web::post().to(user_handlers::create_user))
            .route("/users", web::get().to(user_handlers::list_users))
            .route("/users/{user_id}", web::get().to(user_handlers::get_user))
            .route("/users/username/{username}", web::get().to(user_handlers::get_user_by_username))
            .route("/users/{user_id}/enable", web::put().to(user_handlers::enable_user))
            .route("/users/{user_id}/disable", web::put().to(user_handlers::disable_user))
            .route("/users/{user_id}", web::delete().to(user_handlers::delete_user))
            // Remove the missing route: .route("/users/{user_id}/organisation", web::get().to(user_handlers::get_user_organisation))
            
            // Role routes
            .route("/users/{user_id}/roles", web::post().to(user_handlers::assign_role))
            .route("/users/{user_id}/roles", web::get().to(user_handlers::get_user_roles))
            
            // Organisation routes
            .route("/organisations", web::post().to(organisation_handlers::create_organisation))
            .route("/organisations", web::get().to(organisation_handlers::list_organisations))
            .route("/organisations/{id}", web::get().to(organisation_handlers::get_organisation))
            .route("/organisations/{id}", web::put().to(organisation_handlers::update_organisation))
            .route("/organisations/{id}", web::delete().to(organisation_handlers::delete_organisation))
            .route("/organisations/assign-user", web::post().to(organisation_handlers::assign_user_to_organisation))
            
            // Audit routes
            .route("/audit/logs", web::get().to(audit_handlers::get_audit_logs))
            .route("/audit/logs/export", web::get().to(audit_handlers::export_audit_logs))
            
            // Role request routes
            .route("/role-requests", web::post().to(role_request_handlers::create_role_request))
            .route("/role-requests", web::get().to(role_request_handlers::list_role_requests))
            .route("/role-requests/{id}/review", web::put().to(role_request_handlers::review_role_request))
            .route("/users/{user_id}/role-requests", web::get().to(role_request_handlers::get_user_role_requests)),
    );
}