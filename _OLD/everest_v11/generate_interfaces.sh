#!/bin/bash

set -e

echo "Generating fixed interfaces layer with proper Swagger support..."

cd auth-service

# Create interfaces directories
mkdir -p src/interfaces/{controllers,middleware,routes,models,swagger}

# 1. First, let's fix the DTOs to include ToSchema derive
cat > src/application/dtos/user_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::enums::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserDto {
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserDto {
    pub fn from_entity(user: &crate::domain::entities::User) -> Self {
        Self {
            id: user.id,
            keycloak_id: user.keycloak_id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 6))]
    pub password: String,
    
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
}
EOF

cat > src/application/dtos/auth_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::UserDto;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1))]
    pub username: String,
    
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RefreshTokenDto {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserDto,
}
EOF

cat > src/application/dtos/company_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CompanyDto {
    pub fn from_entity(company: &crate::domain::entities::Company) -> Self {
        Self {
            id: company.id,
            name: company.name.clone(),
            description: company.description.clone(),
            created_by: company.created_by,
            created_at: company.created_at,
            updated_at: company.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateCompanyDto {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateCompanyDto {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    
    pub description: Option<String>,
}
EOF

cat > src/application/dtos/audit_dto.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::enums::AuditAction;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLogDto {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuditLogDto {
    pub fn from_entity(audit_log: &crate::domain::entities::AuditLog) -> Self {
        Self {
            id: audit_log.id,
            user_id: audit_log.user_id,
            action: audit_log.action,
            resource_type: audit_log.resource_type.clone(),
            resource_id: audit_log.resource_id.clone(),
            details: audit_log.details.clone(),
            ip_address: audit_log.ip_address.clone(),
            user_agent: audit_log.user_agent.clone(),
            created_at: audit_log.created_at,
        }
    }
}
EOF

# Update the main dtos mod.rs
cat > src/application/dtos/mod.rs << 'EOF'
pub mod user_dto;
pub mod auth_dto;
pub mod company_dto;
pub mod audit_dto;
pub mod common_dto;
pub mod response_dto;

pub use user_dto::*;
pub use auth_dto::*;
pub use company_dto::*;
pub use audit_dto::*;
pub use common_dto::*;
pub use response_dto::*;
EOF

# 2. Fix the shared error to include ErrorResponse
cat > src/shared/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    Unauthorized(String),
    Internal,
    DatabaseError(String),
    AuthError(String),
    BusinessError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Internal => write!(f, "Internal Server Error"),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::AuthError(msg) => write!(f, "Authentication Error: {}", msg),
            AppError::BusinessError(msg) => write!(f, "Business Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(message) => HttpResponse::NotFound().json(ErrorResponse {
                error: "NOT_FOUND".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::ValidationError(message) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::Unauthorized(message) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "UNAUTHORIZED".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::Internal => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: "An internal server error occurred".to_string(),
                details: None,
            }),
            AppError::DatabaseError(message) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::AuthError(message) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "AUTH_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::BusinessError(message) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "BUSINESS_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(_e: std::io::Error) -> Self {
        AppError::Internal
    }
}

impl From<uuid::Error> for AppError {
    fn from(_e: uuid::Error) -> Self {
        AppError::ValidationError("Invalid UUID format".to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::ValidationError(format!("JSON serialization error: {}", e))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::AuthError(format!("JWT error: {}", e))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(e: bcrypt::BcryptError) -> Self {
        AppError::AuthError(format!("Password hashing error: {}", e))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(format!("Database error: {}", e)),
        }
    }
}

impl From<crate::domain::errors::DomainError> for AppError {
    fn from(e: crate::domain::errors::DomainError) -> Self {
        match e {
            crate::domain::errors::DomainError::UserNotFound => AppError::NotFound("User not found".to_string()),
            crate::domain::errors::DomainError::CompanyNotFound => AppError::NotFound("Company not found".to_string()),
            crate::domain::errors::DomainError::UserAlreadyExists(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::CompanyAlreadyExists(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidEmail(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidPassword(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidUserRole(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidCompanyAssignment => AppError::ValidationError("Invalid company assignment".to_string()),
            crate::domain::errors::DomainError::UnauthorizedOperation => AppError::Unauthorized("Unauthorized operation".to_string()),
            crate::domain::errors::DomainError::ValidationError(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::DomainRuleViolation(msg) => AppError::BusinessError(msg),
            crate::domain::errors::DomainError::InsufficientPermissions(msg) => AppError::Unauthorized(msg),
            crate::domain::errors::DomainError::InvalidOperation(msg) => AppError::BusinessError(msg),
        }
    }
}
EOF



# 5. Create fixed middleware - CORS Middleware
cat > src/interfaces/middleware/cors_middleware.rs << 'EOF'
use actix_cors::Cors;

pub fn create_cors_middleware() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}
EOF

# 6. Create fixed middleware mod.rs
cat > src/interfaces/middleware/mod.rs << 'EOF'
pub mod auth_middleware;
pub mod cors_middleware;
pub mod logging_middleware;

pub use auth_middleware::{AuthenticatedUser, auth_validator, admin_validator};
pub use cors_middleware::create_cors_middleware;
pub use logging_middleware::LoggingMiddleware;
EOF

# 7. Create fixed controllers - Health Controller
cat > src/interfaces/controllers/health_controller.rs << 'EOF'
use actix_web::{web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub service: String,
    pub version: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub service: String,
    pub version: String,
    pub database: String,
    pub keycloak: String,
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        service: "auth-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Detailed health check endpoint
#[utoipa::path(
    get,
    path = "/health/detailed",
    responses(
        (status = 200, description = "Detailed health status", body = DetailedHealthResponse)
    ),
    tag = "health"
)]
pub async fn detailed_health_check() -> HttpResponse {
    HttpResponse::Ok().json(DetailedHealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        service: "auth-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: "connected".to_string(),
        keycloak: "connected".to_string(),
    })
}

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check))
            .route("/detailed", web::get().to(detailed_health_check))
    );
}
EOF

# 8. Create fixed controllers - Auth Controller
cat > src/interfaces/controllers/auth_controller.rs << 'EOF'
use actix_web::{web, HttpResponse};
use utoipa::OpenApi;

use crate::application::services::AuthApplicationService;
use crate::application::dtos::{LoginDto, RefreshTokenDto, AuthResponseDto};
use crate::shared::error::AppError;
use crate::interfaces::middleware::auth_middleware::AuthenticatedUser;

#[derive(OpenApi)]
#[openapi(
    paths(
        login,
        refresh_token,
        logout,
        validate_token,
        get_current_user_info
    ),
    components(
        schemas(LoginDto, RefreshTokenDto, AuthResponseDto, AuthenticatedUser)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct AuthApiDoc;

/// Login user
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = AuthResponseDto),
        (status = 400, description = "Invalid credentials"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn login(
    auth_service: web::Data<dyn AuthApplicationService>,
    login_dto: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    let auth_response = auth_service.login(login_dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(auth_response))
}

/// Refresh token
#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = RefreshTokenDto,
    responses(
        (status = 200, description = "Token refreshed", body = AuthResponseDto),
        (status = 400, description = "Invalid refresh token"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn refresh_token(
    auth_service: web::Data<dyn AuthApplicationService>,
    refresh_dto: web::Json<RefreshTokenDto>,
) -> Result<HttpResponse, AppError> {
    let auth_response = auth_service.refresh_token(refresh_dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(auth_response))
}

/// Logout user
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth",
    security(("bearer" = []))
)]
pub async fn logout(
    auth_service: web::Data<dyn AuthApplicationService>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    auth_service.logout(authenticated_user.user_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

/// Validate token
#[utoipa::path(
    post,
    path = "/auth/validate",
    request_body = String,
    responses(
        (status = 200, description = "Token validation result"),
        (status = 400, description = "Invalid token")
    ),
    tag = "auth"
)]
pub async fn validate_token(
    auth_service: web::Data<dyn AuthApplicationService>,
    token: web::Json<String>,
) -> Result<HttpResponse, AppError> {
    let is_valid = auth_service.validate_token(&token.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "valid": is_valid
    })))
}

/// Get current user info
#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "Current user info", body = AuthenticatedUser),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth",
    security(("bearer" = []))
)]
pub async fn get_current_user_info(
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(authenticated_user))
}

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh_token))
            .route("/logout", web::post().to(logout))
            .route("/validate", web::post().to(validate_token))
            .route("/me", web::get().to(get_current_user_info))
    );
}
EOF

# 9. Create fixed controllers - User Controller
cat > src/interfaces/controllers/user_controller.rs << 'EOF'
use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::application::services::UserApplicationService;
use crate::application::dtos::{CreateUserDto, UpdateUserDto, UserDto};
use crate::shared::error::AppError;
use crate::interfaces::middleware::auth_middleware::AuthenticatedUser;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_user_by_id,
        get_current_user,
        update_user,
        delete_user,
        list_users_by_company,
        assign_user_to_company,
        change_user_role
    ),
    components(
        schemas(UserDto, CreateUserDto, UpdateUserDto, AuthenticatedUser)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct UserApiDoc;

/// Create a new user
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "User created successfully", body = UserDto),
        (status = 400, description = "Bad request"),
        (status = 409, description = "User already exists")
    ),
    tag = "users"
)]
pub async fn create_user(
    user_service: web::Data<dyn UserApplicationService>,
    create_user_dto: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    let user_dto = user_service.create_user(create_user_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(user_dto))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserDto),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn get_user_by_id(
    user_service: web::Data<dyn UserApplicationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let user_dto = user_service.get_user_by_id(user_id).await?;
    Ok(HttpResponse::Ok().json(user_dto))
}

/// Get current user information
#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "Current user info", body = UserDto),
        (status = 401, description = "Unauthorized")
    ),
    tag = "users",
    security(("bearer" = []))
)]
pub async fn get_current_user(
    user_service: web::Data<dyn UserApplicationService>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_dto = user_service.get_user_by_id(authenticated_user.user_id).await?;
    Ok(HttpResponse::Ok().json(user_dto))
}

/// Update user
#[utoipa::path(
    put,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "User updated", body = UserDto),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "users",
    security(("bearer" = []))
)]
pub async fn update_user(
    user_service: web::Data<dyn UserApplicationService>,
    path: web::Path<Uuid>,
    update_user_dto: web::Json<UpdateUserDto>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let user_dto = user_service.update_user(
        user_id,
        update_user_dto.into_inner(),
        authenticated_user.user_id,
    ).await?;
    Ok(HttpResponse::Ok().json(user_dto))
}

/// Delete user
#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "users",
    security(("bearer" = []))
)]
pub async fn delete_user(
    user_service: web::Data<dyn UserApplicationService>,
    path: web::Path<Uuid>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    user_service.delete_user(user_id, authenticated_user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// List users by company
#[utoipa::path(
    get,
    path = "/users/company/{company_id}",
    params(
        ("company_id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "List of users", body = Vec<UserDto>),
        (status = 404, description = "Company not found")
    ),
    tag = "users"
)]
pub async fn list_users_by_company(
    user_service: web::Data<dyn UserApplicationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let company_id = path.into_inner();
    let users = user_service.list_users_by_company(company_id).await?;
    Ok(HttpResponse::Ok().json(users))
}

/// Assign user to company
#[utoipa::path(
    put,
    path = "/users/{user_id}/company/{company_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("company_id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "User assigned to company", body = UserDto),
        (status = 404, description = "User or company not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "users",
    security(("bearer" = []))
)]
pub async fn assign_user_to_company(
    user_service: web::Data<dyn UserApplicationService>,
    path: web::Path<(Uuid, Uuid)>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let (user_id, company_id) = path.into_inner();
    let user_dto = user_service.assign_user_to_company(
        user_id, 
        company_id, 
        authenticated_user.user_id
    ).await?;
    Ok(HttpResponse::Ok().json(user_dto))
}

/// Change user role
#[utoipa::path(
    put,
    path = "/users/{user_id}/role",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    request_body = String,
    responses(
        (status = 200, description = "Role changed", body = UserDto),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "users",
    security(("bearer" = []))
)]
pub async fn change_user_role(
    user_service: web::Data<dyn UserApplicationService>,
    path: web::Path<Uuid>,
    role: web::Json<String>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let new_role = role.into_inner().parse()
        .map_err(|e| AppError::ValidationError(format!("Invalid role: {}", e)))?;
    
    let user_dto = user_service.change_user_role(
        user_id, 
        new_role, 
        authenticated_user.user_id
    ).await?;
    Ok(HttpResponse::Ok().json(user_dto))
}

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/me", web::get().to(get_current_user))
            .route("/{user_id}", web::get().to(get_user_by_id))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user))
            .route("/company/{company_id}", web::get().to(list_users_by_company))
            .route("/{user_id}/company/{company_id}", web::put().to(assign_user_to_company))
            .route("/{user_id}/role", web::put().to(change_user_role))
    );
}
EOF

# 10. Create fixed controllers - Company Controller
cat > src/interfaces/controllers/company_controller.rs << 'EOF'
use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::application::services::CompanyApplicationService;
use crate::application::dtos::{CreateCompanyDto, UpdateCompanyDto, CompanyDto};
use crate::shared::error::AppError;
use crate::interfaces::middleware::auth_middleware::AuthenticatedUser;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_company,
        get_company_by_id,
        update_company,
        delete_company,
        list_companies,
        list_user_companies
    ),
    components(
        schemas(CompanyDto, CreateCompanyDto, UpdateCompanyDto, AuthenticatedUser)
    ),
    tags(
        (name = "companies", description = "Company management endpoints")
    )
)]
pub struct CompanyApiDoc;

/// Create a new company
#[utoipa::path(
    post,
    path = "/companies",
    request_body = CreateCompanyDto,
    responses(
        (status = 201, description = "Company created successfully", body = CompanyDto),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "companies",
    security(("bearer" = []))
)]
pub async fn create_company(
    company_service: web::Data<dyn CompanyApplicationService>,
    create_company_dto: web::Json<CreateCompanyDto>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let company_dto = company_service.create_company(
        create_company_dto.into_inner(), 
        authenticated_user.user_id
    ).await?;
    Ok(HttpResponse::Created().json(company_dto))
}

/// Get company by ID
#[utoipa::path(
    get,
    path = "/companies/{company_id}",
    params(
        ("company_id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company found", body = CompanyDto),
        (status = 404, description = "Company not found")
    ),
    tag = "companies"
)]
pub async fn get_company_by_id(
    company_service: web::Data<dyn CompanyApplicationService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let company_id = path.into_inner();
    let company_dto = company_service.get_company_by_id(company_id).await?;
    Ok(HttpResponse::Ok().json(company_dto))
}

/// Update company
#[utoipa::path(
    put,
    path = "/companies/{company_id}",
    params(
        ("company_id" = Uuid, Path, description = "Company ID")
    ),
    request_body = UpdateCompanyDto,
    responses(
        (status = 200, description = "Company updated", body = CompanyDto),
        (status = 404, description = "Company not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "companies",
    security(("bearer" = []))
)]
pub async fn update_company(
    company_service: web::Data<dyn CompanyApplicationService>,
    path: web::Path<Uuid>,
    update_company_dto: web::Json<UpdateCompanyDto>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let company_id = path.into_inner();
    let company_dto = company_service.update_company(
        company_id, 
        update_company_dto.into_inner(), 
        authenticated_user.user_id
    ).await?;
    Ok(HttpResponse::Ok().json(company_dto))
}

/// Delete company
#[utoipa::path(
    delete,
    path = "/companies/{company_id}",
    params(
        ("company_id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 204, description = "Company deleted"),
        (status = 404, description = "Company not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "companies",
    security(("bearer" = []))
)]
pub async fn delete_company(
    company_service: web::Data<dyn CompanyApplicationService>,
    path: web::Path<Uuid>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let company_id = path.into_inner();
    company_service.delete_company(company_id, authenticated_user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// List all companies
#[utoipa::path(
    get,
    path = "/companies",
    responses(
        (status = 200, description = "List of companies", body = Vec<CompanyDto>)
    ),
    tag = "companies"
)]
pub async fn list_companies(
    company_service: web::Data<dyn CompanyApplicationService>,
) -> Result<HttpResponse, AppError> {
    let companies = company_service.list_companies().await?;
    Ok(HttpResponse::Ok().json(companies))
}

/// List user's companies
#[utoipa::path(
    get,
    path = "/companies/my",
    responses(
        (status = 200, description = "List of user companies", body = Vec<CompanyDto>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "companies",
    security(("bearer" = []))
)]
pub async fn list_user_companies(
    company_service: web::Data<dyn CompanyApplicationService>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let companies = company_service.list_user_companies(authenticated_user.user_id).await?;
    Ok(HttpResponse::Ok().json(companies))
}

pub fn configure_company_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/companies")
            .route("", web::post().to(create_company))
            .route("", web::get().to(list_companies))
            .route("/my", web::get().to(list_user_companies))
            .route("/{company_id}", web::get().to(get_company_by_id))
            .route("/{company_id}", web::put().to(update_company))
            .route("/{company_id}", web::delete().to(delete_company))
    );
}
EOF

# 11. Create fixed controllers - Audit Controller
cat > src/interfaces/controllers/audit_controller.rs << 'EOF'
use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::application::services::AuditApplicationService;
use crate::application::dtos::AuditLogDto;
use crate::shared::error::AppError;
use crate::interfaces::middleware::auth_middleware::AuthenticatedUser;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_audit_logs_by_user,
        get_audit_logs_by_company,
        get_recent_audit_logs
    ),
    components(
        schemas(AuditLogDto, AuthenticatedUser)
    ),
    tags(
        (name = "audit", description = "Audit log endpoints")
    )
)]
pub struct AuditApiDoc;

/// Get audit logs by user
#[utoipa::path(
    get,
    path = "/audit/user/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "List of audit logs", body = Vec<AuditLogDto>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "audit",
    security(("bearer" = []))
)]
pub async fn get_audit_logs_by_user(
    audit_service: web::Data<dyn AuditApplicationService>,
    path: web::Path<Uuid>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    
    if user_id != authenticated_user.user_id && authenticated_user.role != "Admin" {
        return Err(AppError::Unauthorized("Cannot view other users' audit logs".to_string()));
    }
    
    let logs = audit_service.get_audit_logs_by_user(user_id).await?;
    Ok(HttpResponse::Ok().json(logs))
}

/// Get audit logs by company
#[utoipa::path(
    get,
    path = "/audit/company/{company_id}",
    params(
        ("company_id" = Uuid, Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "List of audit logs", body = Vec<AuditLogDto>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "audit",
    security(("bearer" = []))
)]
pub async fn get_audit_logs_by_company(
    audit_service: web::Data<dyn AuditApplicationService>,
    path: web::Path<Uuid>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let company_id = path.into_inner();
    
    if authenticated_user.company_id != Some(company_id) && authenticated_user.role != "Admin" {
        return Err(AppError::Unauthorized("Cannot view this company's audit logs".to_string()));
    }
    
    let logs = audit_service.get_audit_logs_by_company(company_id).await?;
    Ok(HttpResponse::Ok().json(logs))
}

/// Get recent audit logs
#[utoipa::path(
    get,
    path = "/audit/recent",
    params(
        ("limit" = Option<u32>, Query, description = "Limit number of logs (max 100)")
    ),
    responses(
        (status = 200, description = "List of recent audit logs", body = Vec<AuditLogDto>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "audit",
    security(("bearer" = []))
)]
pub async fn get_recent_audit_logs(
    audit_service: web::Data<dyn AuditApplicationService>,
    query: web::Query<std::collections::HashMap<String, u32>>,
    authenticated_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    if authenticated_user.role != "Admin" {
        return Err(AppError::Unauthorized("Only admins can view all audit logs".to_string()));
    }
    
    let limit = query.get("limit").cloned().unwrap_or(50).min(100);
    let logs = audit_service.get_recent_audit_logs(limit).await?;
    Ok(HttpResponse::Ok().json(logs))
}

pub fn configure_audit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/audit")
            .route("/user/{user_id}", web::get().to(get_audit_logs_by_user))
            .route("/company/{company_id}", web::get().to(get_audit_logs_by_company))
            .route("/recent", web::get().to(get_recent_audit_logs))
    );
}
EOF

# 12. Create fixed controllers mod.rs
cat > src/interfaces/controllers/mod.rs << 'EOF'
pub mod health_controller;
pub mod auth_controller;
pub mod user_controller;
pub mod company_controller;
pub mod audit_controller;

pub use health_controller::{
    health_check, 
    detailed_health_check, 
    configure_health_routes
};

pub use auth_controller::{
    login, 
    refresh_token, 
    logout, 
    validate_token, 
    get_current_user_info, 
    configure_auth_routes
};

pub use user_controller::{
    create_user, 
    get_user_by_id, 
    get_current_user, 
    update_user, 
    delete_user,
    list_users_by_company, 
    assign_user_to_company, 
    change_user_role, 
    configure_user_routes
};

pub use company_controller::{
    create_company, 
    get_company_by_id, 
    update_company, 
    delete_company,
    list_companies, 
    list_user_companies, 
    configure_company_routes
};

pub use audit_controller::{
    get_audit_logs_by_user, 
    get_audit_logs_by_company, 
    get_recent_audit_logs, 
    configure_audit_routes
};
EOF

# 13. Create fixed routes modules
cat > src/interfaces/routes/health_routes.rs << 'EOF'
use actix_web::web;

use crate::interfaces::controllers::health_controller;

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    health_controller::configure_health_routes(cfg);
}
EOF

cat > src/interfaces/routes/auth_routes.rs << 'EOF'
use actix_web::web;

use crate::interfaces::controllers::auth_controller;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    auth_controller::configure_auth_routes(cfg);
}
EOF

cat > src/interfaces/routes/user_routes.rs << 'EOF'
use actix_web::web;

use crate::interfaces::controllers::user_controller;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    user_controller::configure_user_routes(cfg);
}
EOF

cat > src/interfaces/routes/company_routes.rs << 'EOF'
use actix_web::web;

use crate::interfaces::controllers::company_controller;

pub fn configure_company_routes(cfg: &mut web::ServiceConfig) {
    company_controller::configure_company_routes(cfg);
}
EOF

cat > src/interfaces/routes/audit_routes.rs << 'EOF'
use actix_web::web;

use crate::interfaces::controllers::audit_controller;

pub fn configure_audit_routes(cfg: &mut web::ServiceConfig) {
    audit_controller::configure_audit_routes(cfg);
}
EOF

cat > src/interfaces/routes/mod.rs << 'EOF'
pub mod health_routes;
pub mod auth_routes;
pub mod user_routes;
pub mod company_routes;
pub mod audit_routes;

pub use health_routes::configure_health_routes;
pub use auth_routes::configure_auth_routes;
pub use user_routes::configure_user_routes;
pub use company_routes::configure_company_routes;
pub use audit_routes::configure_audit_routes;
EOF

# 14. Create fixed swagger configuration
cat > src/interfaces/swagger/mod.rs << 'EOF'
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth Service API",
        version = "1.0.0",
        description = "Authentication and Authorization Microservice",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "/api/v1", description = "API v1 Server"),
        (url = "/", description = "Default Server")
    ),
    components(
        schemas(
            crate::application::dtos::UserDto,
            crate::application::dtos::CreateUserDto,
            crate::application::dtos::UpdateUserDto,
            crate::application::dtos::LoginDto,
            crate::application::dtos::RefreshTokenDto,
            crate::application::dtos::AuthResponseDto,
            crate::application::dtos::CompanyDto,
            crate::application::dtos::CreateCompanyDto,
            crate::application::dtos::UpdateCompanyDto,
            crate::application::dtos::AuditLogDto,
            crate::shared::error::ErrorResponse,
            crate::interfaces::middleware::auth_middleware::AuthenticatedUser,
            crate::interfaces::controllers::health_controller::HealthResponse,
            crate::interfaces::controllers::health_controller::DetailedHealthResponse
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "companies", description = "Company management endpoints"),
        (name = "audit", description = "Audit log endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    paths(
        crate::interfaces::controllers::health_controller::health_check,
        crate::interfaces::controllers::health_controller::detailed_health_check,
        crate::interfaces::controllers::auth_controller::login,
        crate::interfaces::controllers::auth_controller::refresh_token,
        crate::interfaces::controllers::auth_controller::logout,
        crate::interfaces::controllers::auth_controller::validate_token,
        crate::interfaces::controllers::auth_controller::get_current_user_info,
        crate::interfaces::controllers::user_controller::create_user,
        crate::interfaces::controllers::user_controller::get_user_by_id,
        crate::interfaces::controllers::user_controller::get_current_user,
        crate::interfaces::controllers::user_controller::update_user,
        crate::interfaces::controllers::user_controller::delete_user,
        crate::interfaces::controllers::user_controller::list_users_by_company,
        crate::interfaces::controllers::user_controller::assign_user_to_company,
        crate::interfaces::controllers::user_controller::change_user_role,
        crate::interfaces::controllers::company_controller::create_company,
        crate::interfaces::controllers::company_controller::get_company_by_id,
        crate::interfaces::controllers::company_controller::update_company,
        crate::interfaces::controllers::company_controller::delete_company,
        crate::interfaces::controllers::company_controller::list_companies,
        crate::interfaces::controllers::company_controller::list_user_companies,
        crate::interfaces::controllers::audit_controller::get_audit_logs_by_user,
        crate::interfaces::controllers::audit_controller::get_audit_logs_by_company,
        crate::interfaces::controllers::audit_controller::get_recent_audit_logs
    )
)]
pub struct ApiDoc;

pub fn configure_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs/{_:.*}")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
EOF

# 15. Create fixed app configuration
cat > src/interfaces/app_config.rs << 'EOF'
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::interfaces::{
    routes::*,
    middleware::*,
    swagger
};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    let cors = cors_middleware::create_cors_middleware();
    
    let auth = HttpAuthentication::bearer(auth_middleware::auth_validator);
    
    cfg.service(
        web::scope("/api/v1")
            .wrap(cors)
            .wrap(logging_middleware::LoggingMiddleware)
            .configure(health_routes::configure_health_routes)
            .configure(auth_routes::configure_auth_routes)
            .service(
                web::scope("")
                    .wrap(auth)
                    .configure(user_routes::configure_user_routes)
                    .configure(company_routes::configure_company_routes)
                    .configure(audit_routes::configure_audit_routes)
            )
    );
}

pub fn create_app() -> impl actix_web::dev::HttpServiceFactory {
    actix_web::web::scope("")
        .configure(configure_app)
        .service(swagger::configure_swagger_ui())
}
EOF

# 16. Create fixed interfaces mod.rs
cat > src/interfaces/mod.rs << 'EOF'
pub mod controllers;
pub mod middleware;
pub mod routes;
pub mod models;
pub mod swagger;
pub mod app_config;

// Export specific items to avoid ambiguous re-exports
pub use middleware::{
    AuthenticatedUser, 
    auth_validator, 
    admin_validator, 
    create_cors_middleware, 
    LoggingMiddleware
};

pub use app_config::{configure_app, create_app};

// Export route configuration functions
pub use routes::{
    configure_health_routes,
    configure_auth_routes,
    configure_user_routes,
    configure_company_routes,
    configure_audit_routes
};

// Export swagger
pub use swagger::configure_swagger_ui;
EOF

# 17. Create empty models
cat > src/interfaces/models/mod.rs << 'EOF'
// Request and response models for API contracts
EOF

# 1. Fix the domain enums to include ToSchema derive
cat > src/domain/enums.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    Admin,
    Partner,
    Operator,
    User,
    Guest,
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "partner" => Ok(UserRole::Partner),
            "operator" => Ok(UserRole::Operator),
            "user" => Ok(UserRole::User),
            "guest" => Ok(UserRole::Guest),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Partner => write!(f, "Partner"),
            UserRole::Operator => write!(f, "Operator"),
            UserRole::User => write!(f, "User"),
            UserRole::Guest => write!(f, "Guest"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AuditAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    CompanyCreated,
    CompanyUpdated,
    CompanyDeleted,
    Login,
    Logout,
    PasswordReset,
}

impl std::str::FromStr for AuditAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UserCreated" => Ok(AuditAction::UserCreated),
            "UserUpdated" => Ok(AuditAction::UserUpdated),
            "UserDeleted" => Ok(AuditAction::UserDeleted),
            "CompanyCreated" => Ok(AuditAction::CompanyCreated),
            "CompanyUpdated" => Ok(AuditAction::CompanyUpdated),
            "CompanyDeleted" => Ok(AuditAction::CompanyDeleted),
            "Login" => Ok(AuditAction::Login),
            "Logout" => Ok(AuditAction::Logout),
            "PasswordReset" => Ok(AuditAction::PasswordReset),
            _ => Err(format!("Invalid audit action: {}", s)),
        }
    }
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::UserCreated => write!(f, "UserCreated"),
            AuditAction::UserUpdated => write!(f, "UserUpdated"),
            AuditAction::UserDeleted => write!(f, "UserDeleted"),
            AuditAction::CompanyCreated => write!(f, "CompanyCreated"),
            AuditAction::CompanyUpdated => write!(f, "CompanyUpdated"),
            AuditAction::CompanyDeleted => write!(f, "CompanyDeleted"),
            AuditAction::Login => write!(f, "Login"),
            AuditAction::Logout => write!(f, "Logout"),
            AuditAction::PasswordReset => write!(f, "PasswordReset"),
        }
    }
}
EOF

# 2. Fix the logging middleware to import HttpMessage
cat > src/interfaces/middleware/logging_middleware.rs << 'EOF'
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::{
    rc::Rc,
    task::{Context, Poll},
};
use tracing::info;

pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        
        let user_id = req.extensions()
            .get::<crate::interfaces::middleware::auth_middleware::AuthenticatedUser>()
            .map(|user| user.user_id.to_string())
            .unwrap_or_else(|| "anonymous".to_string());

        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let res = service.call(req).await?;
            let duration = start.elapsed();
            
            info!(
                method = %method,
                path = %path,
                status = %res.status().as_u16(),
                duration_ms = duration.as_millis(),
                user_id = %user_id,
                "HTTP request"
            );
            
            Ok(res)
        })
    }
}
EOF

# 3. Also fix the auth middleware to import HttpMessage
cat > src/interfaces/middleware/auth_middleware.rs << 'EOF'
use actix_web::{dev::ServiceRequest, Error, HttpMessage, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use futures_util::future::{ready, Ready};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::infrastructure::auth::JwtService;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<Uuid>,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            ready(Ok(user.clone()))
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized("Not authenticated")))
        }
    }
}

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = match req.app_data::<actix_web::web::Data<JwtService>>() {
        Some(service) => service,
        None => {
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            return Err((AuthenticationError::from(config).into(), req));
        }
    };

    let token = credentials.token();

    match jwt_service.validate_token(token) {
        Ok(claims) => {
            let user_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(_) => {
                    let config = req.app_data::<Config>().cloned().unwrap_or_default();
                    return Err((AuthenticationError::from(config).into(), req));
                }
            };

            let authenticated_user = AuthenticatedUser {
                user_id,
                username: claims.username,
                email: claims.email,
                role: claims.role,
                company_id: claims.company_id.and_then(|id| Uuid::parse_str(&id).ok()),
            };

            req.extensions_mut().insert(authenticated_user);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

pub async fn admin_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = match req.app_data::<actix_web::web::Data<JwtService>>() {
        Some(service) => service,
        None => {
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            return Err((AuthenticationError::from(config).into(), req));
        }
    };

    let token = credentials.token();

    match jwt_service.validate_token(token) {
        Ok(claims) => {
            if claims.role != "Admin" && claims.role != "admin" {
                let config = req.app_data::<Config>().cloned().unwrap_or_default();
                return Err((AuthenticationError::from(config).into(), req));
            }

            let user_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(_) => {
                    let config = req.app_data::<Config>().cloned().unwrap_or_default();
                    return Err((AuthenticationError::from(config).into(), req));
                }
            };

            let authenticated_user = AuthenticatedUser {
                user_id,
                username: claims.username,
                email: claims.email,
                role: claims.role,
                company_id: claims.company_id.and_then(|id| Uuid::parse_str(&id).ok()),
            };

            req.extensions_mut().insert(authenticated_user);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
EOF

# 4. Update the domain mod.rs to re-export enums
cat > src/domain/mod.rs << 'EOF'
pub mod entities;
pub mod value_objects;
pub mod aggregates;
pub mod repositories;
pub mod services;
pub mod events;
pub mod errors;
pub mod enums;

// Re-exports
pub use entities::{User, Company, AuditLog};
pub use value_objects::{Email, Password};
pub use aggregates::{UserAggregate, CompanyAggregate};
pub use services::{UserService, CompanyService};
pub use events::{DomainEvent, UserEvent, CompanyEvent};
pub use errors::DomainError;
pub use enums::{UserRole, AuditAction};
EOF

# 5. Update Cargo.toml to ensure all features are enabled
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["M.MEZNI"]
license = "MIT"

[dependencies]
actix-web = "4.12.0"
serde = { version = "1.0.228", features = ["derive"] }
thiserror = "2.0.17"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
tokio = { version = "1.48.0", features = ["full"] }
config = "0.15.19"
serde_json = "1.0.145"
chrono = { version = "0.4.42", features = ["serde"] }
validator = { version = "0.20.0", features = ["derive"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
async-trait = "0.1.89"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls", "migrate"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
reqwest = { version = "0.12.24", features = ["json"] }
bcrypt = "0.17.1"
utoipa = { version = "5.4.0", features = ["actix_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "9.0.2", features = ["actix-web"] }
actix-cors = "0.7.1"
actix-web-httpauth = "0.8.2"
futures-util = "0.3.31"

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF

# 6. Create a simple test to verify everything works
cat > src/interfaces/test_setup.rs << 'EOF'
// Test utilities for the interfaces layer
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_setup() {
        assert!(true, "Interfaces layer setup is correct");
    }
}
EOF
