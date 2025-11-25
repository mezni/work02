use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use actix_web::web;

use crate::application::dto::{
    UserDto, UserProfileDto, CreateUserRequest, UpdateUserRequest, ChangeUserRoleRequest,
    AssignUserToCompanyRequest, UpdateProfileRequest, ChangePasswordRequest, UserListResponse,
    CompanyDto, CreateCompanyRequest, UpdateCompanyRequest, CompanyListResponse, CompanyUsersResponse,
    RegisterRequest, LoginRequest, RefreshTokenRequest, ForgotPasswordRequest, ResetPasswordRequest,
    AuthResponse, UserAuthDto, TokenValidationResponse,
    AuditLogDto, AuditLogListResponse, AuditSearchRequest
};
use crate::domain::enums::{UserRole, AuditAction};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth routes
        crate::interfaces::routes::auth_routes::register,
        crate::interfaces::routes::auth_routes::login,
        crate::interfaces::routes::auth_routes::refresh_token,
        crate::interfaces::routes::auth_routes::logout,
        crate::interfaces::routes::auth_routes::forgot_password,
        crate::interfaces::routes::auth_routes::reset_password,
        crate::interfaces::routes::auth_routes::get_current_user,
        
        // User routes
        crate::interfaces::routes::user_routes::create_user,
        crate::interfaces::routes::user_routes::get_user,
        crate::interfaces::routes::user_routes::list_users,
        crate::interfaces::routes::user_routes::update_user,
        crate::interfaces::routes::user_routes::delete_user,
        crate::interfaces::routes::user_routes::change_user_role,
        crate::interfaces::routes::user_routes::assign_user_to_company,
        crate::interfaces::routes::user_routes::remove_user_from_company,
        crate::interfaces::routes::user_routes::update_profile,
        crate::interfaces::routes::user_routes::change_password,
        
        // Company routes
        crate::interfaces::routes::company_routes::create_company,
        crate::interfaces::routes::company_routes::get_company,
        crate::interfaces::routes::company_routes::list_companies,
        crate::interfaces::routes::company_routes::update_company,
        crate::interfaces::routes::company_routes::delete_company,
        crate::interfaces::routes::company_routes::get_company_users,
        
        // Audit routes
        crate::interfaces::routes::audit_routes::get_audit_logs,
        crate::interfaces::routes::audit_routes::get_user_audit_logs,
        crate::interfaces::routes::audit_routes::get_company_audit_logs,
    ),
    components(
        schemas(
            // User schemas
            UserDto, UserProfileDto, CreateUserRequest, UpdateUserRequest, 
            ChangeUserRoleRequest, AssignUserToCompanyRequest, UpdateProfileRequest,
            ChangePasswordRequest, UserListResponse,
            
            // Company schemas
            CompanyDto, CreateCompanyRequest, UpdateCompanyRequest, CompanyListResponse,
            CompanyUsersResponse,
            
            // Auth schemas
            RegisterRequest, LoginRequest, RefreshTokenRequest, ForgotPasswordRequest,
            ResetPasswordRequest, AuthResponse, UserAuthDto, TokenValidationResponse,
            
            // Audit schemas
            AuditLogDto, AuditLogListResponse, AuditSearchRequest,
            
            // Enum schemas
            UserRole, AuditAction,
            
            // Common schemas
            crate::common::response::ApiResponse,
            crate::interfaces::errors::ErrorResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "companies", description = "Company management endpoints"),
        (name = "audit", description = "Audit log endpoints")
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
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                )
            );
        }
    }
}

pub fn configure_swagger() -> SwaggerUi {
    SwaggerUi::new("/api/docs/{_:.*}")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}