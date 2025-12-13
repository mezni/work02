use crate::interfaces::{api_doc, handlers};
use actix_web::web;
use utoipa_swagger_ui::SwaggerUi;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(handlers::health_check))
        
        // Swagger UI
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()),
        )
        
        // API routes
        .service(
            web::scope("/api/v1")
                // Authentication routes (public)
                .service(
                    web::scope("/auth")
                        .route("/register", web::post().to(handlers::register))
                        .route("/login", web::post().to(handlers::login))
                        .route("/logout", web::post().to(handlers::logout))
                        .route("/password/change", web::post().to(handlers::change_password))
                        .route("/password/reset", web::post().to(handlers::request_password_reset)),
                )
                
                // User routes (protected)
                .service(
                    web::scope("/users")
                        .route("", web::post().to(handlers::create_user))
                        .route("", web::get().to(handlers::list_users))
                        .route("/me", web::get().to(handlers::get_current_user))
                        .route("/search", web::get().to(handlers::search_users))
                        .route("/{user_id}", web::get().to(handlers::get_user))
                        .route("/{user_id}", web::put().to(handlers::update_user))
                        .route("/{user_id}", web::delete().to(handlers::delete_user))
                        .route("/{user_id}/audit", web::get().to(handlers::get_user_audit_logs)),
                )
                
                // Organization routes (protected)
                .service(
                    web::scope("/network")
                        .route("/{network_id}/users", web::get().to(handlers::get_users_by_network)),
                )
                .service(
                    web::scope("/station")
                        .route("/{station_id}/users", web::get().to(handlers::get_users_by_station)),
                )
                
                // Role-based routes (protected)
                .service(
                    web::scope("/roles")
                        .route("/{role}/users", web::get().to(handlers::get_users_by_role)),
                ),
        );
}