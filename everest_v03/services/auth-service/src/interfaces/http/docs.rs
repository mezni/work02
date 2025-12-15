use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = r#"
# Authentication and User Management Service

A production-ready authentication service with:
- Keycloak integration for SSO
- Role-based access control (user, admin, partner, operator)
- Multi-source users (web self-registration, internal admin-created)
- Comprehensive audit logging with geographic tracking
- Domain-driven design architecture

## Authentication

Most endpoints require a Bearer token in the Authorization header:
```
Authorization: Bearer <your_jwt_token>
```

Get your token by calling the `/api/v1/auth/login` endpoint.

## User Roles

- **user**: Self-registered users (network_id=X, station_id=X)
- **admin**: Full system access, can create/delete users
- **partner**: Requires network_id, manages network-level resources
- **operator**: Requires network_id and station_id, manages station operations

## Rate Limiting

API is rate-limited to 100 requests per minute per IP address.
        "#,
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development"),
        (url = "https://api.example.com", description = "Production")
    ),
    paths(
        crate::interfaces::http::handlers::health_check,
        crate::interfaces::http::handlers::register,
        crate::interfaces::http::handlers::login,
        crate::interfaces::http::handlers::logout,
        crate::interfaces::http::handlers::change_password,
        crate::interfaces::http::handlers::request_password_reset,
        crate::interfaces::http::handlers::create_user,
        crate::interfaces::http::handlers::get_current_user,
        crate::interfaces::http::handlers::get_user,
        crate::interfaces::http::handlers::update_user,
        crate::interfaces::http::handlers::delete_user,
        crate::interfaces::http::handlers::list_users,
        crate::interfaces::http::handlers::search_users,
        crate::interfaces::http::handlers::get_user_audit_logs,
        crate::interfaces::http::handlers::get_user_statistics,
    ),
    components(
        schemas(
            // DTOs
            crate::application::dtos::RegisterRequest,
            crate::application::dtos::LoginRequest,
            crate::application::dtos::LoginResponse,
            crate::application::dtos::ChangePasswordRequest,
            crate::application::dtos::ResetPasswordRequest,
            crate::application::dtos::CreateUserRequest,
            crate::application::dtos::UpdateUserRequest,
            crate::application::dtos::UserResponse,
            crate::application::dtos::UserDetailResponse,
            crate::application::dtos::AuditLogResponse,
            crate::application::dtos::UserStatistics,
            crate::application::dtos::PaginationParams,
            crate::application::dtos::SearchParams,
            crate::application::dtos::PaginatedResponse<crate::application::dtos::UserResponse>,
            crate::application::dtos::PaginatedResponse<crate::application::dtos::AuditLogResponse>,
            crate::application::dtos::MessageResponse,
            crate::application::dtos::HealthResponse,
            crate::application::dtos::ErrorResponse,
            
            // Domain
            crate::domain::value_objects::UserRole,
            crate::domain::value_objects::UserSource,
            crate::domain::audit::AuditAction,
            crate::domain::audit::GeoLocation,
            
            // Core
            crate::core::errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check and status endpoints"),
        (name = "Authentication", description = "User authentication and registration"),
        (name = "Users", description = "User management operations"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            let mut http = utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            );
            
            // Set description (bearer_format is a field, not a method)
            let scheme = utoipa::openapi::security::SecurityScheme::Http(http);
            
            components.add_security_scheme("bearer_auth", scheme);
        }
    }
}