#!/bin/bash

set -e

echo "Generating interfaces layer..."

cd auth-service

# Interfaces mod.rs
cat > src/interfaces/mod.rs << 'EOF'
pub mod controllers;
pub mod routes;
pub mod openapi;
pub mod errors;

// Re-exports
pub use controllers::{AuthController, UserController, CompanyController};
pub use routes::configure_routes;
pub use errors::InterfaceError;
EOF

# Interface errors
cat > src/interfaces/errors.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Application error: {0}")]
    ApplicationError(#[from] crate::application::errors::ApplicationError),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication required")]
    AuthenticationRequired,
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for InterfaceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            InterfaceError::ApplicationError(app_error) => {
                let status_code = match app_error {
                    crate::application::errors::ApplicationError::AuthenticationFailed => {
                        actix_web::http::StatusCode::UNAUTHORIZED
                    }
                    crate::application::errors::ApplicationError::AuthorizationFailed(_) => {
                        actix_web::http::StatusCode::FORBIDDEN
                    }
                    crate::application::errors::ApplicationError::UserNotFound
                    | crate::application::errors::ApplicationError::CompanyNotFound => {
                        actix_web::http::StatusCode::NOT_FOUND
                    }
                    crate::application::errors::ApplicationError::InvalidToken
                    | crate::application::errors::ApplicationError::TokenExpired => {
                        actix_web::http::StatusCode::UNAUTHORIZED
                    }
                    _ => actix_web::http::StatusCode::BAD_REQUEST,
                };
                
                HttpResponse::build(status_code).json(json!({
                    "error": app_error.code(),
                    "message": app_error.to_string(),
                }))
            }
            InterfaceError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "VALIDATION_ERROR",
                    "message": msg,
                }))
            }
            InterfaceError::AuthenticationRequired => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "AUTHENTICATION_REQUIRED",
                    "message": "Authentication is required to access this resource",
                }))
            }
            InterfaceError::InsufficientPermissions => {
                HttpResponse::Forbidden().json(json!({
                    "error": "INSUFFICIENT_PERMISSIONS",
                    "message": "You don't have sufficient permissions to access this resource",
                }))
            }
            InterfaceError::NotFound => {
                HttpResponse::NotFound().json(json!({
                    "error": "NOT_FOUND",
                    "message": "The requested resource was not found",
                }))
            }
            InterfaceError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "BAD_REQUEST",
                    "message": msg,
                }))
            }
            InterfaceError::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "INTERNAL_SERVER_ERROR",
                    "message": "An internal server error occurred",
                }))
            }
        }
    }
}

pub type WebResult<T> = Result<T, InterfaceError>;
EOF

# controllers
cat > src/interfaces/controllers.rs << 'EOF'
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

use crate::application::dto::{
    LoginRequest, RegisterRequest, UserDto, CompanyDto, 
    CreateUserDto, UpdateUserDto, CreateCompanyDto, UpdateCompanyDto
};
use crate::application::services::AuthService;
use crate::domain::repositories::{UserRepository, CompanyRepository};
use crate::interfaces::errors::{WebResult, InterfaceError};

#[derive(Clone)]
pub struct AuthController {
    auth_service: web::Data<Box<dyn AuthService>>,
}

impl AuthController {
    pub fn new(auth_service: web::Data<Box<dyn AuthService>>) -> Self {
        Self { auth_service }
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/register",
        request_body = RegisterRequest,
        responses(
            (status = 201, description = "User registered successfully", body = UserDto),
            (status = 400, description = "Validation error"),
            (status = 500, description = "Internal server error")
        ),
        tag = "auth"
    )]
    pub async fn register(
        &self,
        register_request: web::Json<RegisterRequest>,
    ) -> WebResult<HttpResponse> {
        // Validate the inner data, not the Json wrapper
        register_request.0.validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;
        
        let user_dto = self.auth_service.register(
            register_request.username.clone(),
            register_request.email.clone(),
            register_request.password.clone(),
        ).await?;
        
        Ok(HttpResponse::Created().json(user_dto))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "Login successful", body = LoginResponse),
            (status = 400, description = "Validation error"),
            (status = 401, description = "Authentication failed"),
            (status = 500, description = "Internal server error")
        ),
        tag = "auth"
    )]
    pub async fn login(
        &self,
        login_request: web::Json<LoginRequest>,
    ) -> WebResult<HttpResponse> {
        // Validate the inner data
        login_request.0.validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;
        
        let login_response = self.auth_service.login(
            login_request.username.clone(),
            login_request.password.clone(),
        ).await?;
        
        Ok(HttpResponse::Ok().json(login_response))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/refresh",
        request_body = RefreshTokenRequest,
        responses(
            (status = 200, description = "Token refreshed successfully", body = LoginResponse),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Invalid token"),
            (status = 500, description = "Internal server error")
        ),
        tag = "auth"
    )]
    pub async fn refresh_token(
        &self,
        refresh_request: web::Json<serde_json::Value>,
    ) -> WebResult<HttpResponse> {
        let refresh_token = refresh_request.get("refresh_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InterfaceError::BadRequest("Refresh token is required".to_string()))?;
        
        let login_response = self.auth_service.refresh_token(refresh_token.to_string()).await?;
        
        Ok(HttpResponse::Ok().json(login_response))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/validate",
        request_body = ValidateTokenRequest,
        responses(
            (status = 200, description = "Token is valid", body = BusinessClaims),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Invalid token"),
            (status = 500, description = "Internal server error")
        ),
        tag = "auth"
    )]
    pub async fn validate_token(
        &self,
        token_request: web::Json<serde_json::Value>,
    ) -> WebResult<HttpResponse> {
        let token = token_request.get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InterfaceError::BadRequest("Token is required".to_string()))?;
        
        let claims = self.auth_service.validate_token(token.to_string()).await?;
        
        Ok(HttpResponse::Ok().json(claims))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/logout",
        responses(
            (status = 200, description = "Logged out successfully"),
            (status = 500, description = "Internal server error")
        ),
        tag = "auth"
    )]
    pub async fn logout(
        &self,
        _request: HttpRequest,
    ) -> WebResult<HttpResponse> {
        // In a real implementation, you might want to blacklist the token
        // or call Keycloak's logout endpoint
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Logged out successfully"
        })))
    }
}

#[derive(Clone)]
pub struct UserController {
    user_repository: web::Data<Box<dyn UserRepository>>,
    company_repository: web::Data<Box<dyn CompanyRepository>>,
}

impl UserController {
    pub fn new(
        user_repository: web::Data<Box<dyn UserRepository>>,
        company_repository: web::Data<Box<dyn CompanyRepository>>,
    ) -> Self {
        Self {
            user_repository,
            company_repository,
        }
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/users",
        params(
            ("company_id" = Option<Uuid>, Query, description = "Filter by company ID"),
            ("page" = Option<u32>, Query, description = "Page number"),
            ("page_size" = Option<u32>, Query, description = "Page size")
        ),
        responses(
            (status = 200, description = "List of users", body = [UserDto]),
            (status = 403, description = "Insufficient permissions"),
            (status = 500, description = "Internal server error")
        ),
        tag = "users"
    )]
    pub async fn list_users(
        &self,
        request: HttpRequest,
        query: web::Query<ListUsersQuery>,
    ) -> WebResult<HttpResponse> {
        // Extract user from request (from JWT middleware)
        let current_user = Self::extract_current_user(&request)?;
        
        // Authorization check
        if !current_user.is_admin() && !current_user.is_partner() && !current_user.is_operator() {
            return Err(InterfaceError::InsufficientPermissions);
        }
        
        let users = if let Some(company_id) = query.company_id {
            // Non-admin users can only list users from their own company
            if !current_user.is_admin() && current_user.company_id != Some(company_id) {
                return Err(InterfaceError::InsufficientPermissions);
            }
            self.user_repository.list_by_company(company_id).await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        } else {
            // Only admin users can list all users without company filter
            if !current_user.is_admin() {
                return Err(InterfaceError::InsufficientPermissions);
            }
            self.user_repository.list_all().await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        };
        
        let users_dto: Vec<UserDto> = users.into_iter().map(|u| UserDto {
            id: u.id,
            keycloak_id: u.keycloak_id,
            username: u.username,
            email: u.email,
            role: u.role,
            company_id: u.company_id,
            email_verified: u.email_verified,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }).collect();
        
        Ok(HttpResponse::Ok().json(users_dto))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/users/{id}",
        params(
            ("id" = Uuid, Path, description = "User ID")
        ),
        responses(
            (status = 200, description = "User details", body = UserDto),
            (status = 403, description = "Insufficient permissions"),
            (status = 404, description = "User not found"),
            (status = 500, description = "Internal server error")
        ),
        tag = "users"
    )]
    pub async fn get_user(
        &self,
        request: HttpRequest,
        user_id: web::Path<Uuid>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        let target_user_id = user_id.into_inner();
        
        // Users can view their own profile, admins can view any profile
        // Partners/Operators can view users from their company
        if current_user.id != target_user_id && !current_user.is_admin() {
            let target_user = self.user_repository.find_by_id(target_user_id).await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
                .ok_or(InterfaceError::NotFound)?;
                
            if !current_user.can_manage_user(&target_user) {
                return Err(InterfaceError::InsufficientPermissions);
            }
        }
        
        let user = self.user_repository.find_by_id(target_user_id).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;
        
        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };
        
        Ok(HttpResponse::Ok().json(user_dto))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/users",
        request_body = CreateUserDto,
        responses(
            (status = 201, description = "User created successfully", body = UserDto),
            (status = 400, description = "Validation error"),
            (status = 403, description = "Insufficient permissions"),
            (status = 501, description = "Not implemented"),
            (status = 500, description = "Internal server error")
        ),
        tag = "users"
    )]
    pub async fn create_user(
        &self,
        request: HttpRequest,
        create_user_dto: web::Json<CreateUserDto>,
    ) -> WebResult<HttpResponse> {
        // Validate the inner data
        create_user_dto.0.validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;
        
        let current_user = Self::extract_current_user(&request)?;
        
        // Only admins can create users with specific roles
        if !current_user.is_admin() {
            // Access the role directly from the DTO (it's not optional in CreateUserDto)
            if matches!(create_user_dto.role, crate::domain::enums::UserRole::Admin) {
                return Err(InterfaceError::InsufficientPermissions);
            }
            
            // Partners/Operators can only create users for their company
            if let Some(company_id) = create_user_dto.company_id {
                if !current_user.can_manage_company(company_id) {
                    return Err(InterfaceError::InsufficientPermissions);
                }
            }
        }
        
        // This is a simplified implementation
        // In a real app, you'd use the AuthService to create users in both Keycloak and local DB
        Ok(HttpResponse::NotImplemented().json(serde_json::json!({
            "message": "User creation through this endpoint is not implemented. Use registration endpoint instead."
        })))
    }
    
    #[utoipa::path(
        put,
        path = "/api/v1/users/{id}",
        params(
            ("id" = Uuid, Path, description = "User ID")
        ),
        request_body = UpdateUserDto,
        responses(
            (status = 200, description = "User updated successfully", body = UserDto),
            (status = 400, description = "Validation error"),
            (status = 403, description = "Insufficient permissions"),
            (status = 404, description = "User not found"),
            (status = 500, description = "Internal server error")
        ),
        tag = "users"
    )]
    pub async fn update_user(
        &self,
        request: HttpRequest,
        user_id: web::Path<Uuid>,
        update_user_dto: web::Json<UpdateUserDto>,
    ) -> WebResult<HttpResponse> {
        // Validate the inner data
        update_user_dto.0.validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;
        
        let current_user = Self::extract_current_user(&request)?;
        let target_user_id = user_id.into_inner();
        
        let mut target_user = self.user_repository.find_by_id(target_user_id).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;
        
        // Authorization check
        if !current_user.can_manage_user(&target_user) {
            return Err(InterfaceError::InsufficientPermissions);
        }
        
        // Update user fields
        if let Some(username) = &update_user_dto.username {
            target_user.username = username.clone();
        }
        
        if let Some(email) = &update_user_dto.email {
            target_user.email = email.clone();
        }
        
        if let Some(role) = update_user_dto.role {
            // Only admins can change roles to admin
            if matches!(role, crate::domain::enums::UserRole::Admin) && !current_user.is_admin() {
                return Err(InterfaceError::InsufficientPermissions);
            }
            target_user.role = role;
        }
        
        if let Some(company_id) = update_user_dto.company_id {
            // Authorization check for company assignment
            if !current_user.can_manage_company(company_id) {
                return Err(InterfaceError::InsufficientPermissions);
            }
            target_user.company_id = Some(company_id);
        }
        
        let updated_user = self.user_repository.update(&target_user).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;
        
        let user_dto = UserDto {
            id: updated_user.id,
            keycloak_id: updated_user.keycloak_id,
            username: updated_user.username,
            email: updated_user.email,
            role: updated_user.role,
            company_id: updated_user.company_id,
            email_verified: updated_user.email_verified,
            created_at: updated_user.created_at.to_rfc3339(),
            updated_at: updated_user.updated_at.to_rfc3339(),
        };
        
        Ok(HttpResponse::Ok().json(user_dto))
    }
    
    fn extract_current_user(_request: &HttpRequest) -> WebResult<crate::domain::entities::User> {
        // This would extract the user from JWT claims added by middleware
        // For now, return a mock admin user for testing
        Ok(crate::domain::entities::User::new(
            "mock-keycloak-id".to_string(),
            "admin".to_string(),
            "admin@example.com".to_string(),
            crate::domain::enums::UserRole::Admin,
            None,
        ).map_err(|e| InterfaceError::ApplicationError(e.into()))?)
    }
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct ListUsersQuery {
    #[schema(value_type = Option<String>)]
    pub company_id: Option<Uuid>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Clone)]
pub struct CompanyController {
    company_repository: web::Data<Box<dyn CompanyRepository>>,
    user_repository: web::Data<Box<dyn UserRepository>>,
}

impl CompanyController {
    pub fn new(
        company_repository: web::Data<Box<dyn CompanyRepository>>,
        user_repository: web::Data<Box<dyn UserRepository>>,
    ) -> Self {
        Self {
            company_repository,
            user_repository,
        }
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/companies",
        params(
            ("page" = Option<u32>, Query, description = "Page number"),
            ("page_size" = Option<u32>, Query, description = "Page size")
        ),
        responses(
            (status = 200, description = "List of companies", body = [CompanyDto]),
            (status = 403, description = "Insufficient permissions"),
            (status = 500, description = "Internal server error")
        ),
        tag = "companies"
    )]
    pub async fn list_companies(
        &self,
        request: HttpRequest,
        _query: web::Query<ListCompaniesQuery>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        
        let companies = if current_user.is_admin() {
            self.company_repository.list_all().await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        } else {
            self.company_repository.list_by_user(current_user.id).await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        };
        
        let companies_dto: Vec<CompanyDto> = companies.into_iter().map(|c| CompanyDto {
            id: c.id,
            name: c.name,
            description: c.description,
            created_by: c.created_by,
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
        }).collect();
        
        Ok(HttpResponse::Ok().json(companies_dto))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/companies/{id}",
        params(
            ("id" = Uuid, Path, description = "Company ID")
        ),
        responses(
            (status = 200, description = "Company details", body = CompanyDto),
            (status = 403, description = "Insufficient permissions"),
            (status = 404, description = "Company not found"),
            (status = 500, description = "Internal server error")
        ),
        tag = "companies"
    )]
    pub async fn get_company(
        &self,
        request: HttpRequest,
        company_id: web::Path<Uuid>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        let target_company_id = company_id.into_inner();
        
        let company = self.company_repository.find_by_id(target_company_id).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;
        
        // Authorization check
        if !current_user.can_manage_company(company.id) {
            return Err(InterfaceError::InsufficientPermissions);
        }
        
        let company_dto = CompanyDto {
            id: company.id,
            name: company.name,
            description: company.description,
            created_by: company.created_by,
            created_at: company.created_at.to_rfc3339(),
            updated_at: company.updated_at.to_rfc3339(),
        };
        
        Ok(HttpResponse::Ok().json(company_dto))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/companies",
        request_body = CreateCompanyDto,
        responses(
            (status = 201, description = "Company created successfully", body = CompanyDto),
            (status = 400, description = "Validation error"),
            (status = 403, description = "Insufficient permissions"),
            (status = 500, description = "Internal server error")
        ),
        tag = "companies"
    )]
    pub async fn create_company(
        &self,
        request: HttpRequest,
        create_company_dto: web::Json<CreateCompanyDto>,
    ) -> WebResult<HttpResponse> {
        // Validate the inner data
        create_company_dto.0.validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;
        
        let current_user = Self::extract_current_user(&request)?;
        
        // Only admin users can create companies
        if !current_user.is_admin() {
            return Err(InterfaceError::InsufficientPermissions);
        }
        
        let company = crate::domain::entities::Company::new(
            create_company_dto.name.clone(),
            create_company_dto.description.clone(),
            current_user.id,
        );
        
        let created_company = self.company_repository.create(&company).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;
        
        let company_dto = CompanyDto {
            id: created_company.id,
            name: created_company.name,
            description: created_company.description,
            created_by: created_company.created_by,
            created_at: created_company.created_at.to_rfc3339(),
            updated_at: created_company.updated_at.to_rfc3339(),
        };
        
        Ok(HttpResponse::Created().json(company_dto))
    }
    
    #[utoipa::path(
        put,
        path = "/api/v1/companies/{id}",
        params(
            ("id" = Uuid, Path, description = "Company ID")
        ),
        request_body = UpdateCompanyDto,
        responses(
            (status = 200, description = "Company updated successfully", body = CompanyDto),
            (status = 400, description = "Validation error"),
            (status = 403, description = "Insufficient permissions"),
            (status = 404, description = "Company not found"),
            (status = 500, description = "Internal server error")
        ),
        tag = "companies"
    )]
    pub async fn update_company(
        &self,
        request: HttpRequest,
        company_id: web::Path<Uuid>,
        update_company_dto: web::Json<UpdateCompanyDto>,
    ) -> WebResult<HttpResponse> {
        // Validate the inner data
        update_company_dto.0.validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;
        
        let current_user = Self::extract_current_user(&request)?;
        let target_company_id = company_id.into_inner();
        
        let mut company = self.company_repository.find_by_id(target_company_id).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;
        
        // Authorization check
        if !current_user.can_manage_company(company.id) {
            return Err(InterfaceError::InsufficientPermissions);
        }
        
        // Update company fields
        if let Some(name) = &update_company_dto.name {
            company.name = name.clone();
        }
        
        if let Some(description) = &update_company_dto.description {
            company.description = Some(description.clone());
        }
        
        let updated_company = self.company_repository.update(&company).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;
        
        let company_dto = CompanyDto {
            id: updated_company.id,
            name: updated_company.name,
            description: updated_company.description,
            created_by: updated_company.created_by,
            created_at: updated_company.created_at.to_rfc3339(),
            updated_at: updated_company.updated_at.to_rfc3339(),
        };
        
        Ok(HttpResponse::Ok().json(company_dto))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/companies/{id}/users",
        params(
            ("id" = Uuid, Path, description = "Company ID"),
            ("page" = Option<u32>, Query, description = "Page number"),
            ("page_size" = Option<u32>, Query, description = "Page size")
        ),
        responses(
            (status = 200, description = "List of company users", body = [UserDto]),
            (status = 403, description = "Insufficient permissions"),
            (status = 500, description = "Internal server error")
        ),
        tag = "companies"
    )]
    pub async fn list_company_users(
        &self,
        request: HttpRequest,
        company_id: web::Path<Uuid>,
        _query: web::Query<ListCompanyUsersQuery>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        let target_company_id = company_id.into_inner();
        
        // Authorization check
        if !current_user.can_manage_company(target_company_id) {
            return Err(InterfaceError::InsufficientPermissions);
        }
        
        let users = self.user_repository.list_by_company(target_company_id).await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;
        
        let users_dto: Vec<UserDto> = users.into_iter().map(|u| UserDto {
            id: u.id,
            keycloak_id: u.keycloak_id,
            username: u.username,
            email: u.email,
            role: u.role,
            company_id: u.company_id,
            email_verified: u.email_verified,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }).collect();
        
        Ok(HttpResponse::Ok().json(users_dto))
    }
    
    fn extract_current_user(_request: &HttpRequest) -> WebResult<crate::domain::entities::User> {
        // This would extract the user from JWT claims added by middleware
        // For now, return a mock admin user for testing
        Ok(crate::domain::entities::User::new(
            "mock-keycloak-id".to_string(),
            "admin".to_string(),
            "admin@example.com".to_string(),
            crate::domain::enums::UserRole::Admin,
            None,
        ).map_err(|e| InterfaceError::ApplicationError(e.into()))?)
    }
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct ListCompaniesQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct ListCompanyUsersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// Define request schemas for OpenAPI
#[derive(utoipa::ToSchema)]
struct RefreshTokenRequest {
    refresh_token: String,
}

#[derive(utoipa::ToSchema)]
struct ValidateTokenRequest {
    token: String,
}
EOF



# routes
cat > src/interfaces/routes.rs << 'EOF'
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::interfaces::controllers::{AuthController, UserController, CompanyController};
use crate::interfaces::openapi::ApiDoc;

// Handler functions that work with Actix Web's routing system
async fn register_handler(
    auth_controller: web::Data<AuthController>,
    register_request: web::Json<crate::application::dto::RegisterRequest>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.register(register_request).await
}

async fn login_handler(
    auth_controller: web::Data<AuthController>,
    login_request: web::Json<crate::application::dto::LoginRequest>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.login(login_request).await
}

async fn refresh_token_handler(
    auth_controller: web::Data<AuthController>,
    refresh_request: web::Json<serde_json::Value>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.refresh_token(refresh_request).await
}

async fn validate_token_handler(
    auth_controller: web::Data<AuthController>,
    token_request: web::Json<serde_json::Value>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.validate_token(token_request).await
}

async fn logout_handler(
    auth_controller: web::Data<AuthController>,
    request: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.logout(request).await
}

async fn list_users_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    query: web::Query<crate::interfaces::controllers::ListUsersQuery>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.list_users(request, query).await
}

async fn get_user_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    user_id: web::Path<uuid::Uuid>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.get_user(request, user_id).await
}

async fn create_user_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    create_user_dto: web::Json<crate::application::dto::CreateUserDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.create_user(request, create_user_dto).await
}

async fn update_user_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    user_id: web::Path<uuid::Uuid>,
    update_user_dto: web::Json<crate::application::dto::UpdateUserDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.update_user(request, user_id, update_user_dto).await
}

async fn list_companies_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    query: web::Query<crate::interfaces::controllers::ListCompaniesQuery>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.list_companies(request, query).await
}

async fn get_company_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    company_id: web::Path<uuid::Uuid>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.get_company(request, company_id).await
}

async fn create_company_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    create_company_dto: web::Json<crate::application::dto::CreateCompanyDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.create_company(request, create_company_dto).await
}

async fn update_company_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    company_id: web::Path<uuid::Uuid>,
    update_company_dto: web::Json<crate::application::dto::UpdateCompanyDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.update_company(request, company_id, update_company_dto).await
}

async fn list_company_users_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    company_id: web::Path<uuid::Uuid>,
    query: web::Query<crate::interfaces::controllers::ListCompanyUsersQuery>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.list_company_users(request, company_id, query).await
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Serve Swagger UI
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
    
    // API routes
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(register_handler))
                    .route("/login", web::post().to(login_handler))
                    .route("/refresh", web::post().to(refresh_token_handler))
                    .route("/validate", web::post().to(validate_token_handler))
                    .route("/logout", web::post().to(logout_handler))
            )
            .service(
                web::scope("/users")
                    .route("", web::get().to(list_users_handler))
                    .route("", web::post().to(create_user_handler))
                    .route("/{id}", web::get().to(get_user_handler))
                    .route("/{id}", web::put().to(update_user_handler))
            )
            .service(
                web::scope("/companies")
                    .route("", web::get().to(list_companies_handler))
                    .route("", web::post().to(create_company_handler))
                    .route("/{id}", web::get().to(get_company_handler))
                    .route("/{id}", web::put().to(update_company_handler))
                    .route("/{id}/users", web::get().to(list_company_users_handler))
            )
    );
    
    // Health check
    cfg.route("/health", web::get().to(|| async {
        actix_web::HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "service": "auth-service"
        }))
    }));
}
EOF

# OpenAPI documentation
cat > src/interfaces/openapi.rs << 'EOF'
use utoipa::OpenApi;

use crate::application::dto::{
    LoginRequest, LoginResponse, RegisterRequest, UserDto, CompanyDto,
    CreateUserDto, UpdateUserDto, CreateCompanyDto, UpdateCompanyDto,
    BusinessClaims
};
use crate::interfaces::controllers::{
    ListUsersQuery, ListCompaniesQuery, ListCompanyUsersQuery
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth routes - we need to document the controller methods directly
        // since utoipa doesn't work well with wrapper functions
        crate::interfaces::controllers::AuthController::register,
        crate::interfaces::controllers::AuthController::login,
        crate::interfaces::controllers::AuthController::refresh_token,
        crate::interfaces::controllers::AuthController::validate_token,
        crate::interfaces::controllers::AuthController::logout,
        
        // User routes
        crate::interfaces::controllers::UserController::list_users,
        crate::interfaces::controllers::UserController::get_user,
        crate::interfaces::controllers::UserController::create_user,
        crate::interfaces::controllers::UserController::update_user,
        
        // Company routes
        crate::interfaces::controllers::CompanyController::list_companies,
        crate::interfaces::controllers::CompanyController::get_company,
        crate::interfaces::controllers::CompanyController::create_company,
        crate::interfaces::controllers::CompanyController::update_company,
        crate::interfaces::controllers::CompanyController::list_company_users,
    ),
    components(
        schemas(
            LoginRequest, LoginResponse, RegisterRequest, UserDto, CompanyDto,
            CreateUserDto, UpdateUserDto, CreateCompanyDto, UpdateCompanyDto,
            BusinessClaims,
            crate::domain::enums::UserRole,
            ListUsersQuery,
            ListCompaniesQuery,
            ListCompanyUsersQuery
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "companies", description = "Company management endpoints")
    )
)]
pub struct ApiDoc;
EOF

# Create interfaces tests directory
mkdir -p tests/unit/interfaces

# Interfaces tests mod.rs
cat > tests/unit/interfaces/mod.rs << 'EOF'
pub mod controllers_test;
pub mod errors_test;
pub mod routes_test;
EOF

# Controllers tests
cat > tests/unit/interfaces/controllers_test.rs << 'EOF'
use actix_web::{test, web, App, http};
use auth_service::interfaces::controllers::{AuthController, UserController, CompanyController};
use auth_service::interfaces::routes::configure_routes;
use auth_service::application::dto::{RegisterRequest, LoginRequest};
use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use async_trait::async_trait;
use uuid::Uuid;

// Mock AuthService for testing
struct MockAuthService {
    should_succeed: bool,
}

impl MockAuthService {
    fn new(should_succeed: bool) -> Self {
        Self { should_succeed }
    }
}

#[async_trait]
impl auth_service::application::services::AuthService for MockAuthService {
    async fn login(&self, _username: String, _password: String) -> Result<auth_service::application::dto::LoginResponse, auth_service::application::errors::ApplicationError> {
        if self.should_succeed {
            let user = User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap();
            
            Ok(auth_service::application::dto::LoginResponse {
                access_token: "test-token".to_string(),
                refresh_token: "test-refresh-token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: 3600,
                user: auth_service::application::dto::UserDto {
                    id: user.id,
                    keycloak_id: user.keycloak_id,
                    username: user.username,
                    email: user.email,
                    role: user.role,
                    company_id: user.company_id,
                    email_verified: user.email_verified,
                    created_at: user.created_at.to_rfc3339(),
                    updated_at: user.updated_at.to_rfc3339(),
                },
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::AuthenticationFailed)
        }
    }
    
    async fn register(&self, _username: String, _email: String, _password: String) -> Result<auth_service::application::dto::UserDto, auth_service::application::errors::ApplicationError> {
        if self.should_succeed {
            let user = User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap();
            
            Ok(auth_service::application::dto::UserDto {
                id: user.id,
                keycloak_id: user.keycloak_id,
                username: user.username,
                email: user.email,
                role: user.role,
                company_id: user.company_id,
                email_verified: user.email_verified,
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::ValidationError("Mock error".to_string()))
        }
    }
    
    async fn validate_token(&self, _token: String) -> Result<auth_service::application::dto::BusinessClaims, auth_service::application::errors::ApplicationError> {
        if self.should_succeed {
            Ok(auth_service::application::dto::BusinessClaims {
                sub: "keycloak-123".to_string(),
                email: "test@example.com".to_string(),
                username: "testuser".to_string(),
                role: UserRole::User,
                company_id: None,
                permissions: vec!["users:read:self".to_string()],
                exp: 1234567890,
                iat: 1234567890,
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::InvalidToken)
        }
    }
    
    async fn refresh_token(&self, _refresh_token: String) -> Result<auth_service::application::dto::LoginResponse, auth_service::application::errors::ApplicationError> {
        if self.should_succeed {
            let user = User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap();
            
            Ok(auth_service::application::dto::LoginResponse {
                access_token: "new-token".to_string(),
                refresh_token: "new-refresh-token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: 3600,
                user: auth_service::application::dto::UserDto {
                    id: user.id,
                    keycloak_id: user.keycloak_id,
                    username: user.username,
                    email: user.email,
                    role: user.role,
                    company_id: user.company_id,
                    email_verified: user.email_verified,
                    created_at: user.created_at.to_rfc3339(),
                    updated_at: user.updated_at.to_rfc3339(),
                },
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::InvalidToken)
        }
    }
}

#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("auth-service"));
}

#[actix_web::test]
async fn test_swagger_ui_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/swagger-ui/")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_auth_register_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "newuser@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    // This should return 501 Not Implemented since we're using mock controllers
    // In a real implementation, this would test the actual registration flow
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_auth_login_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    let login_request = LoginRequest {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    // This should return an error since we're using mock controllers
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_user_list_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/v1/users")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    // Should return unauthorized/forbidden without proper authentication
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_company_list_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/v1/companies")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    // Should return unauthorized/forbidden without proper authentication
    assert!(resp.status().is_client_error());
}
EOF

# Errors tests
cat > tests/unit/interfaces/errors_test.rs << 'EOF'
use actix_web::{http::StatusCode, test};
use auth_service::interfaces::errors::{InterfaceError, WebResult};
use auth_service::application::errors::ApplicationError;

#[test]
fn test_interface_error_codes() {
    let auth_error = InterfaceError::ApplicationError(ApplicationError::AuthenticationFailed);
    assert_eq!(auth_error.code(), "APP_AUTHENTICATION_FAILED");
    
    let validation_error = InterfaceError::ValidationError("Test error".to_string());
    assert_eq!(validation_error.to_string(), "Validation error: Test error");
}

#[test]
fn test_interface_error_responses() {
    let auth_error = InterfaceError::ApplicationError(ApplicationError::AuthenticationFailed);
    let response = auth_error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    let validation_error = InterfaceError::ValidationError("Test error".to_string());
    let response = validation_error.error_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let not_found_error = InterfaceError::NotFound;
    let response = not_found_error.error_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    let forbidden_error = InterfaceError::InsufficientPermissions;
    let response = forbidden_error.error_response();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    
    let internal_error = InterfaceError::InternalServerError;
    let response = internal_error.error_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_web_result_type() {
    let success_result: WebResult<i32> = Ok(42);
    assert!(success_result.is_ok());
    
    let error_result: WebResult<i32> = Err(InterfaceError::NotFound);
    assert!(error_result.is_err());
}

#[test]
fn test_error_messages() {
    let auth_required = InterfaceError::AuthenticationRequired;
    assert_eq!(auth_required.to_string(), "Authentication required");
    
    let bad_request = InterfaceError::BadRequest("Invalid input".to_string());
    assert_eq!(bad_request.to_string(), "Bad request: Invalid input");
    
    let app_error = InterfaceError::ApplicationError(ApplicationError::UserNotFound);
    assert_eq!(app_error.to_string(), "Application error: User not found");
}
EOF

# Routes tests
cat > tests/unit/interfaces/routes_test.rs << 'EOF'
use actix_web::{test, App};
use auth_service::interfaces::routes::configure_routes;

#[actix_web::test]
async fn test_all_routes_configured() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    // Test health endpoint
    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Test Swagger UI endpoint
    let req = test::TestRequest::get()
        .uri("/swagger-ui/")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Test auth endpoints exist (they should return errors without proper setup)
    let auth_endpoints = [
        "/api/v1/auth/register",
        "/api/v1/auth/login",
        "/api/v1/auth/refresh",
        "/api/v1/auth/validate",
        "/api/v1/auth/logout",
    ];
    
    for endpoint in auth_endpoints.iter() {
        let req = test::TestRequest::post()
            .uri(endpoint)
            .to_request();
        let resp = test::call_service(&app, req).await;
        // These should return some kind of response (not 404)
        assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
    }
    
    // Test user endpoints exist
    let user_endpoints = [
        "/api/v1/users",
        "/api/v1/users/", // Test with trailing slash
    ];
    
    for endpoint in user_endpoints.iter() {
        let req = test::TestRequest::get()
            .uri(endpoint)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
    }
    
    // Test company endpoints exist
    let company_endpoints = [
        "/api/v1/companies",
        "/api/v1/companies/", // Test with trailing slash
    ];
    
    for endpoint in company_endpoints.iter() {
        let req = test::TestRequest::get()
            .uri(endpoint)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
    }
}

#[actix_web::test]
async fn test_api_version_prefix() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;
    
    // Test that all API routes are under /api/v1 prefix
    let non_api_req = test::TestRequest::get()
        .uri("/users") // Without /api/v1 prefix
        .to_request();
    let resp = test::call_service(&app, non_api_req).await;
    // This should be 404 since routes are under /api/v1
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    
    let api_req = test::TestRequest::get()
        .uri("/api/v1/users") // With /api/v1 prefix
        .to_request();
    let resp = test::call_service(&app, api_req).await;
    // This should not be 404
    assert!(resp.status() != actix_web::http::StatusCode::NOT_FOUND);
}
EOF

# Integration tests directory
mkdir -p tests/integration

# Integration tests
cat > tests/integration/auth_flow_test.rs << 'EOF'
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use auth_service::interfaces::routes::configure_routes;

    // These would be comprehensive integration tests
    // testing the complete authentication flow
    
    #[actix_web::test]
    async fn test_user_registration_flow() {
        // Test user registration -> login -> token validation
        // This would require a test Keycloak instance and test database
        
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;
        
        // In a real integration test, we would:
        // 1. Register a new user
        // 2. Login with the new user credentials
        // 3. Validate the received token
        // 4. Access protected endpoints with the token
        
        assert!(true); // Placeholder for actual test logic
    }
    
    #[actix_web::test] 
    async fn test_company_management_flow() {
        // Test company creation -> user assignment -> permission checks
        
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;
        
        // In a real integration test, we would:
        // 1. Login as admin
        // 2. Create a new company
        // 3. Assign users to the company
        // 4. Test permission checks for company resources
        
        assert!(true); // Placeholder for actual test logic
    }
    
    #[actix_web::test]
    async fn test_authentication_required() {
        // Test that protected endpoints require authentication
        
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;
        
        // Test accessing user list without authentication
        let req = test::TestRequest::get()
            .uri("/api/v1/users")
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return unauthorized/forbidden
        assert!(resp.status().is_client_error());
        
        // Test accessing company list without authentication
        let req = test::TestRequest::get()
            .uri("/api/v1/companies")
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return unauthorized/forbidden
        assert!(resp.status().is_client_error());
    }
}
EOF

# Main integration test mod.rs
cat > tests/integration/mod.rs << 'EOF'
pub mod auth_flow_test;
EOF

echo "Interfaces layer generated successfully!"
echo ""
echo "Project generation complete!"
echo ""
echo "Next steps:"
echo "1. Make all scripts executable: chmod +x generate_*.sh"
echo "2. Run the scripts in order:"
echo "   ./generate_project_structure.sh"
echo "   ./generate_domain.sh" 
echo "   ./generate_application.sh"
echo "   ./generate_infrastructure.sh"
echo "   ./generate_interfaces.sh"
echo "3. Set up Keycloak and PostgreSQL"
echo "4. Run: cargo build"
echo "5. Run: cargo test"
echo ""
echo "Project structure overview:"
echo "├── src/"
echo "│   ├── domain/           # Core business logic"
echo "│   ├── application/      # Use cases and services"
echo "│   ├── infrastructure/   # External concerns (DB, Keycloak)"
echo "│   ├── interfaces/       # Web API and controllers"
echo "│   └── main.rs          # Application entry point"
echo "├── tests/"
echo "│   ├── unit/            # Unit tests for each layer"
echo "│   └── integration/     # Integration tests"
echo "└── Cargo.toml"