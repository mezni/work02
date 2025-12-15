use crate::application::dtos::*;
use crate::application::user_commands::UserCommandHandler;
use crate::application::user_queries::UserQueryHandler;
use crate::core::errors::AppError;
use crate::domain::value_objects::UserRole;
use crate::interfaces::http::middleware::{extract_geo_info, extract_user_agent, AuthenticatedUser};
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;

// ============================================================================
// Health & Status
// ============================================================================

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health_check() -> HttpResponse {
    // TODO: Add uptime tracking
    let response = HealthResponse::healthy(0);
    HttpResponse::Ok().json(response)
}

// ============================================================================
// Authentication Handlers
// ============================================================================

/// Register a new user (public)
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = UserResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 409, description = "Email or username already exists", body = ErrorResponse)
    )
)]
pub async fn register(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let cmd_handler = UserCommandHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
        state.outbox_repo.clone(),
        state.keycloak.clone(),
    );

    let create_req = CreateUserRequest {
        email: body.email.clone(),
        username: body.username.clone(),
        password: body.password.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        phone: body.phone.clone(),
        photo: None,
        role: UserRole::User,
        network_id: Some("X".to_string()),
        station_id: Some("X".to_string()),
    };

    let user = cmd_handler
        .create_user(create_req, "system", geo, user_agent)
        .await?;

    Ok(HttpResponse::Created().json(user))
}

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
pub async fn login(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    // Authenticate with Keycloak
    let token_response = state
        .keycloak
        .authenticate(&body.email, &body.password)
        .await?;

    // Get user from database
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let user = query_handler.get_user_by_email(&body.email).await?;

    // TODO: Log audit for login
    
    let response = LoginResponse {
        access_token: token_response.access_token,
        token_type: token_response.token_type,
        expires_in: token_response.expires_in,
        refresh_token: token_response.refresh_token,
        user,
    };

    Ok(HttpResponse::Ok().json(response))
}

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
pub async fn logout(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    body: Option<web::Json<serde_json::Value>>,
) -> Result<HttpResponse, AppError> {
    // Extract refresh token if provided
    if let Some(json) = body {
        if let Some(refresh_token) = json.get("refresh_token").and_then(|v| v.as_str()) {
            let _ = state.keycloak.logout(refresh_token).await;
        }
    }

    // TODO: Log audit for logout

    Ok(HttpResponse::Ok().json(MessageResponse::success("Logged out successfully")))
}

/// Change password
#[utoipa::path(
    post,
    path = "/api/v1/auth/password/change",
    tag = "Authentication",
    request_body = ChangePasswordRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Password changed", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn change_password(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    // Verify current password by attempting authentication
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let user = query_handler.get_user_by_email(&body.email).await?;
    
    // Change password in Keycloak
    state
        .keycloak
        .authenticate(&user.email, &body.current_password)
        .await?;

    if let Some(ref keycloak_id) = user.keycloak_id {
        state
            .keycloak
            .change_password(keycloak_id, &body.new_password)
            .await?;
    }

    Ok(HttpResponse::Ok().json(MessageResponse::success("Password changed successfully")))
}

/// Request password reset
#[utoipa::path(
    post,
    path = "/api/v1/auth/password/reset",
    tag = "Authentication",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Reset email sent", body = MessageResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    )
)]
pub async fn request_password_reset(
    state: web::Data<AppState>,
    body: web::Json<ResetPasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let user = query_handler.get_user_by_email(&body.email).await?;

    if let Some(ref keycloak_id) = user.keycloak_id {
        state
            .keycloak
            .send_password_reset_email(keycloak_id)
            .await?;
    }

    Ok(HttpResponse::Ok().json(MessageResponse::success("Password reset email sent")))
}

// ============================================================================
// User Management Handlers
// ============================================================================

/// Create user (admin only)
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body = CreateUserRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    )
)]
pub async fn create_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let role: UserRole = auth.role.parse()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    if !role.can_create_users() {
        return Err(AppError::Forbidden("Only admins can create users".to_string()));
    }

    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let cmd_handler = UserCommandHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
        state.outbox_repo.clone(),
        state.keycloak.clone(),
    );

    let user = cmd_handler
        .create_user(body.into_inner(), &auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::Created().json(user))
}

/// Get current user
#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "Users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Current user", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_current_user(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let user = query_handler.get_user(&auth.user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}",
    tag = "Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    )
)]
pub async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let user = query_handler.get_user(&user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// Update user
#[utoipa::path(
    put,
    path = "/api/v1/users/{user_id}",
    tag = "Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    )
)]
pub async fn update_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    auth: AuthenticatedUser,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let role: UserRole = auth.role.parse()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    // Users can only update themselves unless they're admin
    if auth.user_id != user_id && !role.is_admin() {
        return Err(AppError::Forbidden("You can only update your own profile".to_string()));
    }

    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let cmd_handler = UserCommandHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
        state.outbox_repo.clone(),
        state.keycloak.clone(),
    );

    let user = cmd_handler
        .update_user(&user_id, body.into_inner(), &auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::Ok().json(user))
}

/// Delete user (admin only)
#[utoipa::path(
    delete,
    path = "/api/v1/users/{user_id}",
    tag = "Users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "User deleted"),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    )
)]
pub async fn delete_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let role: UserRole = auth.role.parse()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    if !role.can_delete_users() {
        return Err(AppError::Forbidden("Only admins can delete users".to_string()));
    }

    let user_id = path.into_inner();
    let geo = extract_geo_info(&req);
    let user_agent = extract_user_agent(&req);

    let cmd_handler = UserCommandHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
        state.outbox_repo.clone(),
        state.keycloak.clone(),
    );

    cmd_handler
        .delete_user(&user_id, &auth.user_id, geo, user_agent)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// List users
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    params(
        ("limit" = Option<i64>, Query, description = "Page size"),
        ("offset" = Option<i64>, Query, description = "Page offset")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Users list", body = PaginatedResponse<UserResponse>)
    )
)]
pub async fn list_users(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let users = query_handler.list_users(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(users))
}

/// Search users
#[utoipa::path(
    get,
    path = "/api/v1/users/search",
    tag = "Users",
    params(
        ("query" = String, Query, description = "Search query"),
        ("limit" = Option<i64>, Query, description = "Page size"),
        ("offset" = Option<i64>, Query, description = "Page offset")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Search results", body = PaginatedResponse<UserResponse>)
    )
)]
pub async fn search_users(
    state: web::Data<AppState>,
    query: web::Query<SearchParams>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let users = query_handler.search_users(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(users))
}

/// Get user audit logs
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}/audit",
    tag = "Users",
    params(
        ("user_id" = String, Path, description = "User ID"),
        ("limit" = Option<i64>, Query, description = "Page size"),
        ("offset" = Option<i64>, Query, description = "Page offset")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Audit logs", body = PaginatedResponse<AuditLogResponse>)
    )
)]
pub async fn get_user_audit_logs(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let role: UserRole = auth.role.parse()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    // Users can only view their own audit logs unless they're admin
    if auth.user_id != user_id && !role.is_admin() {
        return Err(AppError::Forbidden("You can only view your own audit logs".to_string()));
    }

    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let logs = query_handler
        .get_user_audit_logs(&user_id, query.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(logs))
}

/// Get user statistics (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/users/statistics",
    tag = "Users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User statistics", body = UserStatistics)
    )
)]
pub async fn get_user_statistics(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let role: UserRole = auth.role.parse()
        .map_err(|_| AppError::Unauthorized("Invalid role".to_string()))?;

    if !role.is_admin() {
        return Err(AppError::Forbidden("Only admins can view statistics".to_string()));
    }

    let query_handler = UserQueryHandler::new(
        state.user_repo.clone(),
        state.audit_repo.clone(),
    );

    let stats = query_handler.get_user_statistics().await?;
    Ok(HttpResponse::Ok().json(stats))
}