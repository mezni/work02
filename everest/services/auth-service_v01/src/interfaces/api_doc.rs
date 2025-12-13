use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "Authentication and User Management Service with Keycloak Integration",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
        (url = "https://api.example.com", description = "Production server")
    ),
    paths(
        crate::interfaces::handlers::health_check,
        crate::interfaces::handlers::register,
        crate::interfaces::handlers::login,
        crate::interfaces::handlers::logout,
        crate::interfaces::handlers::change_password,
        crate::interfaces::handlers::request_password_reset,
        crate::interfaces::handlers::create_user,
        crate::interfaces::handlers::get_user,
        crate::interfaces::handlers::get_current_user,
        crate::interfaces::handlers::update_user,
        crate::interfaces::handlers::delete_user,
        crate::interfaces::handlers::list_users,
        crate::interfaces::handlers::search_users,
        crate::interfaces::handlers::get_users_by_network,
        crate::interfaces::handlers::get_users_by_station,
        crate::interfaces::handlers::get_users_by_role,
        crate::interfaces::handlers::get_user_audit_logs,
    ),
    components(
        schemas(
            crate::application::dto::RegisterRequest,
            crate::application::dto::LoginRequest,
            crate::application::dto::LoginResponse,
            crate::application::dto::CreateUserRequest,
            crate::application::dto::UpdateUserRequest,
            crate::application::dto::ChangePasswordRequest,
            crate::application::dto::UserResponse,
            crate::application::dto::AuditLogResponse,
            crate::application::dto::PaginatedResponse<crate::application::dto::UserResponse>,
            crate::application::dto::PaginatedResponse<crate::application::dto::AuditLogResponse>,
            crate::application::dto::MessageResponse,
            crate::application::dto::HealthResponse,
            crate::application::dto::PaginationParams,
            crate::application::dto::SearchParams,
            crate::domain::value_objects::UserRole,
            crate::domain::value_objects::UserSource,
            crate::domain::audit_entity::AuditAction,
            crate::domain::audit_entity::GeoLocation,
            crate::core::errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication and registration"),
        (name = "Users", description = "User management operations"),
        (name = "Network", description = "Network-based user queries"),
        (name = "Station", description = "Station-based user queries"),
        (name = "Roles", description = "Role-based user queries"),
        (name = "Audit", description = "Audit log operations")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    )
                    .bearer_format("JWT")
                    .description(Some("Enter your JWT token in the format: Bearer <token>")),
                ),
            )
        }
    }
}

// Add utoipa path annotations to handlers
#[cfg(feature = "utoipa")]
mod handler_docs {
    use super::*;
    use crate::application::dto::*;
    use crate::core::errors::ErrorResponse;
    
    // These would normally be in the handlers.rs file but shown here for reference
    
    /// Health check endpoint
    #[utoipa::path(
        get,
        path = "/health",
        tag = "Health",
        responses(
            (status = 200, description = "Service is healthy", body = HealthResponse)
        )
    )]
    pub async fn health_check() {}
    
    /// Register a new user
    #[utoipa::path(
        post,
        path = "/api/v1/auth/register",
        tag = "Authentication",
        request_body = RegisterRequest,
        responses(
            (status = 201, description = "User registered successfully", body = UserResponse),
            (status = 400, description = "Bad request", body = ErrorResponse),
            (status = 409, description = "User already exists", body = ErrorResponse)
        )
    )]
    pub async fn register() {}
    
    /// Login user
    #[utoipa::path(
        post,
        path = "/api/v1/auth/login",
        tag = "Authentication",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "Login successful", body = LoginResponse),
            (status = 401, description = "Invalid credentials", body = ErrorResponse)
        )
    )]
    pub async fn login() {}
    
    /// Logout user
    #[utoipa::path(
        post,
        path = "/api/v1/auth/logout",
        tag = "Authentication",
        security(("bearer_auth" = [])),
        responses(
            (status = 200, description = "Logged out successfully", body = MessageResponse),
            (status = 401, description = "Unauthorized", body = ErrorResponse)
        )
    )]
    pub async fn logout() {}
    
    /// Change password
    #[utoipa::path(
        post,
        path = "/api/v1/auth/password/change",
        tag = "Authentication",
        request_body = ChangePasswordRequest,
        security(("bearer_auth" = [])),
        responses(
            (status = 200, description = "Password changed successfully", body = MessageResponse),
            (status = 401, description = "Unauthorized", body = ErrorResponse)
        )
    )]
    pub async fn change_password() {}
    
    /// Request password reset
    #[utoipa::path(
        post,
        path = "/api/v1/auth/password/reset",
        tag = "Authentication",
        request_body = inline(Object),
        responses(
            (status = 200, description = "Reset email sent", body = MessageResponse),
            (status = 404, description = "User not found", body = ErrorResponse)
        )
    )]
    pub async fn request_password_reset() {}
}