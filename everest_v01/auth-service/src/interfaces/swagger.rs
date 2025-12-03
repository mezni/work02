use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use crate::interfaces::dtos::*;
use crate::application::commands::*;
use crate::application::queries::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        crate::interfaces::handlers::AuthHandlerImpl::login,
        crate::interfaces::handlers::AuthHandlerImpl::register,
        crate::interfaces::handlers::AuthHandlerImpl::refresh_token,
        crate::interfaces::handlers::AuthHandlerImpl::logout,
        crate::interfaces::handlers::AuthHandlerImpl::validate_token,
        // User endpoints
        crate::interfaces::handlers::UserHandlerImpl::get_current_user,
        crate::interfaces::handlers::UserHandlerImpl::update_current_user,
        crate::interfaces::handlers::UserHandlerImpl::change_password,
        // Health endpoints
        crate::interfaces::handlers::HealthHandlerImpl::health_check,
        crate::interfaces::handlers::HealthHandlerImpl::detailed_health_check,
        // Admin endpoints
        crate::interfaces::handlers::AdminHandlerImpl::list_users,
        crate::interfaces::handlers::AdminHandlerImpl::get_user,
    ),
    components(
        schemas(
            // Request DTOs
            LoginRequest, RegisterRequest, RefreshTokenRequest, LogoutRequest,
            ChangePasswordRequest, UpdateProfileRequest, ResetPasswordRequest,
            ConfirmPasswordResetRequest, VerifyEmailRequest, ResendVerificationRequest,
            ValidateTokenRequest, UpdateUserRoleRequest,
            // Response DTOs
            AuthResponse, RegistrationResponse, UserResponse, UserListResponse,
            HealthResponse, DetailedHealthResponse, ErrorResponse, SuccessResponse,
            TokenInfoResponse, UserStatistics, AuthStatistics,
            // Query DTOs
            PaginationQuery, UserFilterQuery,
            // Command/Query schemas
            LoginCommand, RegisterCommand, RefreshTokenCommand, LogoutCommand,
            ChangePasswordCommand, UpdateProfileCommand, ResetPasswordCommand,
            ConfirmPasswordResetCommand, VerifyEmailCommand, ResendVerificationCommand,
            GetUserQuery, ValidateTokenQuery, ListUsersQuery,
            // Domain types
            crate::domain::user::User,
            crate::domain::token::Token,
            crate::domain::value_objects::UserRole,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "admin", description = "Administrator endpoints"),
        (name = "health", description = "Health check endpoints"),
    ),
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "Authentication and authorization service with Keycloak integration",
        contact(
            name = "Auth Service Team",
            email = "auth@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "/api/v1", description = "API v1"),
        (url = "http://localhost:8080/api/v1", description = "Local development"),
    )
)]
pub struct ApiDoc;

pub fn configure_swagger(cfg: &mut ServiceConfig) {
    let openapi = ApiDoc::openapi();
    
    cfg.service(
        web::resource("/openapi.json")
            .route(web::get().to(move || {
                let json = serde_json::to_value(&openapi).unwrap();
                HttpResponse::Ok().json(json)
            }))
    );
    
    // Swagger UI is configured in http_routes.rs
}

// Helper function to get swagger config with customizations
pub fn get_swagger_config() -> Config<'static> {
    let mut config = Config::new(["/api-docs/openapi.json"]);
    
    // Customize swagger UI
    config.default_models_expand_depth = 2;
    config.default_model_expand_depth = 2;
    config.doc_expansion = "list".to_string();
    config.filter = true;
    config.show_extensions = true;
    config.show_common_extensions = true;
    config.oauth = Some(
        utoipa_swagger_ui::OAuthConfig::new()
            .client_id("auth-service")
            .app_name("Auth Service")
            .use_pkce_with_authorization_code_grant(true),
    );
    
    config
}