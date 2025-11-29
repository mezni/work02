// src/interfaces/handlers/auth_handlers.rs
use crate::application::dto::auth_dto::{
    LoginRequest, LoginResponse, LogoutResponse, RefreshTokenRequest, ValidateTokenResponse,
};
use crate::application::dto::user_dto::ErrorResponse;
use crate::application::services::auth_service::AuthError;
use crate::interfaces::AppState;
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};
use validator::Validate;

/// User login
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn login(state: web::Data<AppState>, payload: web::Json<LoginRequest>) -> impl Responder {
    info!("Login attempt for user: {}", payload.username);

    // Validate input
    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    let username = payload.username.clone(); // Clone username before moving payload

    match state.auth_service.login(payload.into_inner()).await {
        Ok(response) => {
            info!("Login successful for user: {}", response.username);
            HttpResponse::Ok().json(response)
        }
        Err(AuthError::InvalidCredentials | AuthError::UserNotFound) => {
            error!("Invalid credentials for user: {}", username);
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Authentication Failed".to_string(),
                message: "Invalid username or password".to_string(),
            })
        }
        Err(e) => {
            error!("Login failed: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Validate token
#[utoipa::path(
    post,
    path = "/api/v1/auth/validate",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token validation result", body = ValidateTokenResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn validate_token(
    state: web::Data<AppState>,
    payload: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    info!("Validating token");

    match state.auth_service.validate_token(&payload.token) {
        Ok(claims) => {
            info!("Token validated for user: {}", claims.username);
            HttpResponse::Ok().json(ValidateTokenResponse {
                valid: true,
                user_id: Some(claims.sub),
                username: Some(claims.username),
            })
        }
        Err(_) => {
            info!("Token validation failed");
            HttpResponse::Ok().json(ValidateTokenResponse {
                valid: false,
                user_id: None,
                username: None,
            })
        }
    }
}

/// Refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Invalid token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    state: web::Data<AppState>,
    payload: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    info!("Refreshing token");

    match state.auth_service.refresh_token(&payload.token).await {
        Ok(response) => {
            info!("Token refreshed for user: {}", response.username);
            HttpResponse::Ok().json(response)
        }
        Err(AuthError::TokenValidation(_)) => {
            error!("Invalid token for refresh");
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid Token".to_string(),
                message: "Token is invalid or expired".to_string(),
            })
        }
        Err(e) => {
            error!("Token refresh failed: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// User logout
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn logout() -> impl Responder {
    info!("User logout");

    HttpResponse::Ok().json(LogoutResponse {
        message: "Logout successful".to_string(),
    })
}
