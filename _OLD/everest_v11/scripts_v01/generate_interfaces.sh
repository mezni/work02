#!/bin/bash

set -e

echo "Fixing Interfaces Layer Issues..."

cd auth-service

# Fix 1: Update Cargo.toml with correct dependencies
echo "Updating Cargo.toml with correct dependencies..."

# Remove the duplicate dependencies section and add proper ones
cat > cargo_fix.toml << 'EOF'
# Swagger/OpenAPI documentation
utoipa = { version = "4.0", features = ["actix_ext", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "4.0", features = ["actix-web"] }

# HTTP Auth middleware
actix-web-httpauth = "0.8"

# CORS support
actix-cors = "0.7"

# Futures utilities
futures-util = "0.3"
EOF

# Fix 2: Update the swagger UI configuration
cat > src/interfaces/swagger/ui.rs << 'EOF'
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::interfaces::controllers::{
    user_controller, auth_controller, company_controller, audit_controller, health_controller
};
use crate::application::dtos::{
    UserDto, CreateUserDto, UpdateUserDto, LoginDto, AuthResponseDto, RefreshTokenDto,
    CompanyDto, CreateCompanyDto, UpdateCompanyDto, AuditLogDto
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        auth_controller::login,
        auth_controller::refresh_token,
        auth_controller::logout,
        auth_controller::validate_token,
        auth_controller::get_current_user_info,
        // User endpoints
        user_controller::create_user,
        user_controller::get_user_by_id,
        user_controller::get_current_user,
        user_controller::update_user,
        user_controller::delete_user,
        user_controller::list_users_by_company,
        user_controller::assign_user_to_company,
        user_controller::change_user_role,
        // Company endpoints
        company_controller::create_company,
        company_controller::get_company_by_id,
        company_controller::update_company,
        company_controller::delete_company,
        company_controller::list_companies,
        company_controller::list_user_companies,
        // Audit endpoints
        audit_controller::get_audit_logs_by_user,
        audit_controller::get_audit_logs_by_company,
        audit_controller::get_recent_audit_logs,
        // Health endpoints
        health_controller::health_check,
        health_controller::detailed_health_check,
    ),
    components(
        schemas(
            UserDto, CreateUserDto, UpdateUserDto,
            LoginDto, AuthResponseDto, RefreshTokenDto,
            CompanyDto, CreateCompanyDto, UpdateCompanyDto,
            AuditLogDto,
            user_controller::AssignToCompanyQuery,
            user_controller::ChangeRoleRequest,
            auth_controller::ValidateTokenRequest,
            auth_controller::ValidateTokenResponse,
            audit_controller::PaginationQuery,
            audit_controller::RecentLogsQuery,
            health_controller::HealthResponse,
            health_controller::DetailedHealthResponse,
            crate::application::dtos::ApiResponse<String>,
            crate::application::dtos::ErrorResponse,
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Companies", description = "Company management endpoints"),
        (name = "Audit", description = "Audit log endpoints"),
        (name = "Health", description = "Health check endpoints")
    ),
    modifiers(
        &crate::interfaces::swagger::config::SecurityAddon, 
        &crate::interfaces::swagger::config::ApiInfo
    )
)]
pub struct ApiDoc;

pub fn configure_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
EOF

# Fix 3: Update auth middleware to handle errors properly
cat > src/interfaces/middleware/auth_middleware.rs << 'EOF'
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use serde::Serialize;
use uuid::Uuid;

use crate::infrastructure::auth::JwtService;

#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<Uuid>,
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
            if claims.role != "admin" {
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

# Fix 4: Simplify logging middleware without futures_util
cat > src/interfaces/middleware/logging_middleware.rs << 'EOF'
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use tracing::{info, warn};

pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareInner {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let method = req.method().clone();
        let path = req.path().to_string();

        Box::pin(async move {
            let start = std::time::Instant::now();
            let result = service.call(req).await;
            let duration = start.elapsed();

            match &result {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        info!(
                            "{} {} {} {}ms",
                            method,
                            path,
                            status.as_u16(),
                            duration.as_millis()
                        );
                    } else if status.is_client_error() {
                        warn!(
                            "{} {} {} {}ms - Client error",
                            method,
                            path,
                            status.as_u16(),
                            duration.as_millis()
                        );
                    } else {
                        warn!(
                            "{} {} {} {}ms - Server error",
                            method,
                            path,
                            status.as_u16(),
                            duration.as_millis()
                        );
                    }
                }
                Err(_) => {
                    warn!(
                        "{} {} ERROR {}ms",
                        method,
                        path,
                        duration.as_millis()
                    );
                }
            }

            result
        })
    }
}
EOF

# Fix 5: Update error middleware
cat > src/interfaces/middleware/error_middleware.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use tracing::error;

use crate::shared::error::AppError;

pub fn json_error_handler(
    err: actix_web::Error,
    req: &actix_web::HttpRequest,
) -> HttpResponse {
    let status = err.as_response_error().status_code();
    
    // Convert Actix error to our AppError if possible
    let app_error = if let Some(app_err) = err.as_error::<AppError>() {
        app_err.clone()
    } else {
        match status {
            actix_web::http::StatusCode::NOT_FOUND => {
                AppError::NotFound("The requested resource was not found".to_string())
            }
            actix_web::http::StatusCode::UNAUTHORIZED => {
                AppError::Unauthorized("Authentication required".to_string())
            }
            actix_web::http::StatusCode::FORBIDDEN => {
                AppError::Unauthorized("Insufficient permissions".to_string())
            }
            actix_web::http::StatusCode::BAD_REQUEST => {
                AppError::ValidationError("Invalid request data".to_string())
            }
            _ => {
                error!("Unhandled error: {} - {}", req.path(), err);
                AppError::Internal
            }
        }
    };

    app_error.error_response()
}
EOF

# Fix 6: Update domain enums with proper serialization
cat > src/domain/enums.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    Admin,
    User,
    Manager,
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            "manager" => Ok(UserRole::Manager),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Manager => write!(f, "manager"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum AuditAction {
    Login,
    Logout,
    UserCreated,
    UserUpdated,
    UserDeleted,
    CompanyCreated,
    CompanyUpdated,
    CompanyDeleted,
    RoleChanged,
    PermissionChanged,
}

impl FromStr for AuditAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Login" => Ok(AuditAction::Login),
            "Logout" => Ok(AuditAction::Logout),
            "UserCreated" => Ok(AuditAction::UserCreated),
            "UserUpdated" => Ok(AuditAction::UserUpdated),
            "UserDeleted" => Ok(AuditAction::UserDeleted),
            "CompanyCreated" => Ok(AuditAction::CompanyCreated),
            "CompanyUpdated" => Ok(AuditAction::CompanyUpdated),
            "CompanyDeleted" => Ok(AuditAction::CompanyDeleted),
            "RoleChanged" => Ok(AuditAction::RoleChanged),
            "PermissionChanged" => Ok(AuditAction::PermissionChanged),
            _ => Err(format!("Invalid audit action: {}", s)),
        }
    }
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::Login => write!(f, "Login"),
            AuditAction::Logout => write!(f, "Logout"),
            AuditAction::UserCreated => write!(f, "UserCreated"),
            AuditAction::UserUpdated => write!(f, "UserUpdated"),
            AuditAction::UserDeleted => write!(f, "UserDeleted"),
            AuditAction::CompanyCreated => write!(f, "CompanyCreated"),
            AuditAction::CompanyUpdated => write!(f, "CompanyUpdated"),
            AuditAction::CompanyDeleted => write!(f, "CompanyDeleted"),
            AuditAction::RoleChanged => write!(f, "RoleChanged"),
            AuditAction::PermissionChanged => write!(f, "PermissionChanged"),
        }
    }
}
EOF

# Fix 7: Update company service implementation
cat > src/application/services/company_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::repositories::{CompanyRepository, UserRepository};
use crate::application::dtos::{CompanyDto, CreateCompanyDto, UpdateCompanyDto};
use crate::shared::error::AppError;

#[async_trait]
pub trait CompanyApplicationService: Send + Sync {
    async fn create_company(&self, create_company_dto: CreateCompanyDto, creator_id: Uuid) -> Result<CompanyDto, AppError>;
    async fn get_company_by_id(&self, company_id: Uuid) -> Result<CompanyDto, AppError>;
    async fn update_company(&self, company_id: Uuid, update_company_dto: UpdateCompanyDto, actor_id: Uuid) -> Result<CompanyDto, AppError>;
    async fn delete_company(&self, company_id: Uuid, actor_id: Uuid) -> Result<(), AppError>;
    async fn list_companies(&self) -> Result<Vec<CompanyDto>, AppError>;
    async fn list_user_companies(&self, user_id: Uuid) -> Result<Vec<CompanyDto>, AppError>;
}

pub struct CompanyApplicationServiceImpl {
    company_repository: Box<dyn CompanyRepository>,
    user_repository: Box<dyn UserRepository>,
}

impl CompanyApplicationServiceImpl {
    pub fn new(
        company_repository: Box<dyn CompanyRepository>,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            company_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl CompanyApplicationService for CompanyApplicationServiceImpl {
    async fn create_company(&self, create_company_dto: CreateCompanyDto, creator_id: Uuid) -> Result<CompanyDto, AppError> {
        // Check if company name already exists
        if self.company_repository.find_by_name(&create_company_dto.name).await?.is_some() {
            return Err(AppError::ValidationError("Company with this name already exists".to_string()));
        }

        // Verify creator exists
        let creator = self.user_repository.find_by_id(creator_id).await?
            .ok_or_else(|| AppError::NotFound("Creator user not found".to_string()))?;

        let company = Company::new(
            create_company_dto.name,
            create_company_dto.description,
            creator.id,
        );

        let saved_company = self.company_repository.create(&company).await?;
        
        Ok(CompanyDto::from_entity(&saved_company))
    }

    async fn get_company_by_id(&self, company_id: Uuid) -> Result<CompanyDto, AppError> {
        let company = self.company_repository.find_by_id(company_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;
        
        Ok(CompanyDto::from_entity(&company))
    }

    async fn update_company(&self, company_id: Uuid, update_company_dto: UpdateCompanyDto, actor_id: Uuid) -> Result<CompanyDto, AppError> {
        let mut company = self.company_repository.find_by_id(company_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if !actor.can_manage_company(company.id) {
            return Err(AppError::Unauthorized("Insufficient permissions to update this company".to_string()));
        }

        if let Some(name) = update_company_dto.name {
            company.name = name;
        }
        if let Some(description) = update_company_dto.description {
            company.description = Some(description);
        }

        let updated_company = self.company_repository.update(&company).await?;
        
        Ok(CompanyDto::from_entity(&updated_company))
    }

    async fn delete_company(&self, company_id: Uuid, actor_id: Uuid) -> Result<(), AppError> {
        let company = self.company_repository.find_by_id(company_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Company with ID {} not found", company_id)))?;

        let actor = self.user_repository.find_by_id(actor_id).await?
            .ok_or_else(|| AppError::NotFound("Actor user not found".to_string()))?;

        if !actor.can_manage_company(company.id) {
            return Err(AppError::Unauthorized("Insufficient permissions to delete this company".to_string()));
        }

        self.company_repository.delete(company_id).await?;
        
        Ok(())
    }

    async fn list_companies(&self) -> Result<Vec<CompanyDto>, AppError> {
        let companies = self.company_repository.list_all().await?;
        
        Ok(companies.iter().map(CompanyDto::from_entity).collect())
    }

    async fn list_user_companies(&self, user_id: Uuid) -> Result<Vec<CompanyDto>, AppError> {
        let companies = self.company_repository.list_by_user(user_id).await?;
        
        Ok(companies.iter().map(CompanyDto::from_entity).collect())
    }
}
EOF

# Fix 8: Update shared error handling
cat > src/shared/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::domain::errors::DomainError;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Domain error: {0}")]
    DomainError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal => HttpResponse::InternalServerError().json("Internal server error"),
            AppError::ConfigError(msg) => HttpResponse::InternalServerError().json(format!("Configuration error: {}", msg)),
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError().json(format!("Database error: {}", msg)),
            AppError::AuthError(msg) => HttpResponse::Unauthorized().json(format!("Authentication error: {}", msg)),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(format!("Validation error: {}", msg)),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(format!("Not found: {}", msg)),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(format!("Unauthorized: {}", msg)),
            AppError::DomainError(msg) => HttpResponse::BadRequest().json(format!("Domain error: {}", msg)),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        AppError::ConfigError(e.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::DatabaseError(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::AuthError(e.to_string())
    }
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        AppError::DomainError(e.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::AuthError(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Internal
    }
}

impl From<uuid::Error> for AppError {
    fn from(e: uuid::Error) -> Self {
        AppError::ValidationError("Invalid UUID format".to_string())
    }
}
EOF

# Fix 9: Create a main application configuration
cat > src/interfaces/app_config.rs << 'EOF'
use actix_web::web;
use actix_cors::Cors;

use crate::interfaces::{
    routes::*,
    middleware::*,
    swagger::ui
};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    // Configure CORS
    let cors = cors_middleware::create_cors_middleware();
    
    // Configure routes
    cfg.service(
        web::scope("")
            .wrap(cors)
            .wrap(logging_middleware::LoggingMiddleware)
            .configure(health_routes::configure_health_routes)
            .configure(auth_routes::configure_auth_routes)
            .configure(user_routes::configure_user_routes)
            .configure(company_routes::configure_company_routes)
            .configure(audit_routes::configure_audit_routes)
            .service(ui::configure_swagger_ui())
    );
}

pub fn create_app() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("").configure(configure_app)
}
EOF

# Fix 10: Update the interfaces mod file
cat > src/interfaces/mod.rs << 'EOF'
pub mod controllers;
pub mod middleware;
pub mod routes;
pub mod models;
pub mod swagger;
pub mod api_docs;
pub mod app_config;

pub use controllers::*;
pub use middleware::*;
pub use routes::*;
pub use models::*;
pub use swagger::*;
pub use api_docs::*;
pub use app_config::*;
EOF
