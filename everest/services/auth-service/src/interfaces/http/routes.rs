use crate::core::rate_limiter::RateLimiter;
use crate::interfaces::http::{docs, handlers};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Rate limiter: 100 requests per minute per IP
    let rate_limiter = RateLimiter::new(100);

    cfg
        // Health check (no auth required)
        .route("/health", web::get().to(handlers::health_check))
        
        // Swagger UI
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", docs::ApiDoc::openapi()),
        )
        
        // API v1 routes
        .service(
            web::scope("/api/v1")
                .wrap(rate_limiter)
                
                // Authentication routes (public)
                .service(
                    web::scope("/auth")
                        .route("/register", web::post().to(handlers::register))
                        .route("/login", web::post().to(handlers::login))
                        .route("/logout", web::post().to(handlers::logout))
//                        .route("/password/change", web::post().to(handlers::change_password))
//                        .route("/password/reset", web::post().to(handlers::request_password_reset))
                )
                
                // User routes (protected)
                .service(
                    web::scope("/users")
                        .route("", web::post().to(handlers::create_user))
                        .route("", web::get().to(handlers::list_users))
                        .route("/me", web::get().to(handlers::get_current_user))
                        .route("/search", web::get().to(handlers::search_users))
                        .route("/statistics", web::get().to(handlers::get_user_statistics))
                        .route("/{user_id}", web::get().to(handlers::get_user))
                        .route("/{user_id}", web::put().to(handlers::update_user))
                        .route("/{user_id}", web::delete().to(handlers::delete_user))
                        .route("/{user_id}/audit", web::get().to(handlers::get_user_audit_logs))
                )
        );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(
            App::new().route("/health", web::get().to(handlers::health_check))
        ).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}