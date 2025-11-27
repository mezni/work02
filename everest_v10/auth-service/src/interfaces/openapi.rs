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
