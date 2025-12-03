use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;
use crate::{
    application::{
        AuthService, RegistrationService, TokenService,
        service_traits::{AuthServiceTrait, RegistrationServiceTrait, TokenServiceTrait},
    },
    interfaces::{
        handlers::{AuthHandlerImpl, UserHandlerImpl, HealthHandlerImpl, AdminHandlerImpl},
        swagger::ApiDoc,
        middleware::{AuthMiddleware, ErrorHandler},
    },
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Create shared services
    let auth_service = Arc::new(AuthService::new(
        // These would be injected from infrastructure
        todo!("Inject UserRepository"),
        todo!("Inject AuthRepository"),
        todo!("Inject TokenRepository"),
    ));
    
    let registration_service = Arc::new(RegistrationService::new(
        todo!("Inject UserRepository"),
        todo!("Inject RegistrationRepository"),
    ));
    
    let token_service = Arc::new(TokenService::new(
        todo!("Inject UserRepository"),
        todo!("Inject TokenRepository"),
    ));
    
    // Create handlers
    let auth_handler = AuthHandlerImpl::new(
        auth_service.clone(),
        registration_service.clone(),
        token_service.clone(),
    );
    
    let user_handler = UserHandlerImpl::new(
        todo!("Inject UserService"),
        token_service.clone(),
    );
    
    let health_handler = HealthHandlerImpl::new(
        todo!("Inject HealthChecker"),
    );
    
    let admin_handler = AdminHandlerImpl::new(
        todo!("Inject UserService"),
    );
    
    // API v1 routes
    cfg.service(
        web::scope("/api/v1")
            .wrap(ErrorHandler)
            // Auth routes (no auth required)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(AuthHandlerImpl::login))
                    .route("/register", web::post().to(AuthHandlerImpl::register))
                    .route("/refresh", web::post().to(AuthHandlerImpl::refresh_token))
                    .route("/logout", web::post().to(AuthHandlerImpl::logout))
                    .route("/verify-email", web::post().to(AuthHandlerImpl::verify_email))
                    .route("/resend-verification", web::post().to(AuthHandlerImpl::resend_verification))
                    .route("/reset-password", web::post().to(AuthHandlerImpl::reset_password))
                    .route("/confirm-password-reset", web::post().to(AuthHandlerImpl::confirm_password_reset))
                    .route("/validate", web::post().to(AuthHandlerImpl::validate_token))
            )
            // User routes (auth required)
            .service(
                web::scope("/users")
                    .wrap(AuthMiddleware)
                    .route("/me", web::get().to(UserHandlerImpl::get_current_user))
                    .route("/me", web::patch().to(UserHandlerImpl::update_current_user))
                    .route("/me/password", web::post().to(UserHandlerImpl::change_password))
            )
            // Admin routes (admin auth required)
            .service(
                web::scope("/admin")
                    .wrap(AuthMiddleware::admin())
                    .route("/users", web::get().to(AdminHandlerImpl::list_users))
                    .route("/users/{user_id}", web::get().to(AdminHandlerImpl::get_user))
                    .route("/users/{user_id}/role", web::put().to(AdminHandlerImpl::update_user_role))
                    .route("/users/{user_id}/activate", web::put().to(AdminHandlerImpl::activate_user))
                    .route("/users/{user_id}/deactivate", web::put().to(AdminHandlerImpl::deactivate_user))
            )
            // Health routes
            .service(
                web::scope("/health")
                    .route("", web::get().to(HealthHandlerImpl::health_check))
                    .route("/detailed", web::get().to(HealthHandlerImpl::detailed_health_check))
            )
    );
    
    // Swagger UI
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
    
    // Root/health endpoint
    cfg.route("/health", web::get().to(|| async {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "service": "auth-service",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }));
    
    // 404 handler
    cfg.default_service(
        web::route().to(|| async {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Not Found",
                "message": "Endpoint not found",
                "code": "NOT_FOUND"
            }))
        })
    );
}