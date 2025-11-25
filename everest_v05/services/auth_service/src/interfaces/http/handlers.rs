use actix_web::{web, HttpResponse, Result, HttpRequest};
use uuid::Uuid;
use crate::application::services::{UserService, CreateUserCommand};
use crate::interfaces::http::dto::{
    CreateUserRequest, LoginRequest, UserResponse, LoginResponse, ErrorResponse
};
use crate::domain::models::{NewUser, UserCredentials};
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::interfaces::http::auth_middleware::{get_authenticated_user, has_required_role};

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_user(
    user_service: web::Data<UserService<impl crate::application::repositories::UserRepository>>,
    request: web::Json<CreateUserRequest>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    // Check if user is authenticated and has admin role
    let current_user = get_authenticated_user(&req)
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "Authentication required".to_string(),
            })
        })?;

    let is_admin = has_required_role(current_user, "admin");
    
    let new_user = NewUser {
        email: request.email.clone(),
        username: request.username.clone(),
        password: request.password.clone(),
        role: request.role.clone().unwrap_or(UserRole::User),
        company_id: request.company_id,
    };

    match user_service.create_user(new_user, is_admin).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                role: user.role,
                company_id: user.company_id,
                is_active: user.is_active,
                created_at: user.created_at,
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            let status = match e {
                DomainError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
                DomainError::UserAlreadyExists => actix_web::http::StatusCode::CONFLICT,
                DomainError::CompanyAssignmentNotAllowed => actix_web::http::StatusCode::BAD_REQUEST,
                _ => actix_web::http::StatusCode::BAD_REQUEST,
            };
            Ok(HttpResponse::build(status).json(ErrorResponse {
                error: e.to_string(),
                message: "Failed to create user".to_string(),
            }))
        }
    }
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    )
)]
pub async fn login(
    user_service: web::Data<UserService<impl crate::application::repositories::UserRepository>>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let credentials = UserCredentials {
        username: request.username.clone(),
        password: request.password.clone(),
    };

    match user_service.authenticate_user(&credentials).await {
        Ok((user, token)) => {
            let response = LoginResponse {
                user: UserResponse {
                    id: user.id,
                    email: user.email,
                    username: user.username,
                    role: user.role,
                    company_id: user.company_id,
                    is_active: user.is_active,
                    created_at: user.created_at,
                },
                token,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid credentials".to_string(),
                message: e.to_string(),
            }))
        }
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user(
    user_service: web::Data<UserService<impl crate::application::repositories::UserRepository>>,
    user_id: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    // Check authentication
    let current_user = get_authenticated_user(&req)
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "Authentication required".to_string(),
            })
        })?;

    // Users can only view their own profile unless they're admin
    let requested_user_id = *user_id;
    let is_admin = has_required_role(current_user, "admin");
    let is_own_profile = current_user.id == requested_user_id.to_string();

    if !is_admin && !is_own_profile {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            error: "Forbidden".to_string(),
            message: "You can only view your own profile".to_string(),
        }));
    }

    match user_service.get_user_by_id(requested_user_id).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
                role: user.role,
                company_id: user.company_id,
                is_active: user.is_active,
                created_at: user.created_at,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(DomainError::UserNotFound) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "User not found".to_string(),
                message: "The requested user does not exist".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                message: e.to_string(),
            }))
        }
    }
}