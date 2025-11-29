use crate::application::dto::user_dto::{
    AssignRoleDto, CreateUserDto, CreateUserResponse, ErrorResponse, SuccessResponse, UserDto,
    UserRolesDto,
};
use crate::interfaces::AppState;
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};
use validator::Validate;

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "User created successfully", body = CreateUserResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn create_user(
    state: web::Data<AppState>,
    payload: web::Json<CreateUserDto>,
) -> impl Responder {
    info!("Creating user: {}", payload.username);

    // Validate input
    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    match state.user_service.create_user(payload.into_inner()).await {
        Ok(user_id) => {
            info!("User created successfully: {}", user_id);
            HttpResponse::Created().json(CreateUserResponse {
                id: user_id,
                message: "User created successfully".to_string(),
            })
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Creation Failed".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserDto),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_user(state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    info!("Getting user: {}", user_id);

    match state.user_service.get_user(&user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Not Found".to_string(),
            message: format!("User with ID {} not found", user_id),
        }),
        Err(e) => {
            error!("Failed to get user: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Get user by username
#[utoipa::path(
    get,
    path = "/api/v1/users/username/{username}",
    params(
        ("username" = String, Path, description = "Username")
    ),
    responses(
        (status = 200, description = "User found", body = UserDto),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_user_by_username(
    state: web::Data<AppState>,
    username: web::Path<String>,
) -> impl Responder {
    info!("Getting user by username: {}", username);

    match state.user_service.get_user_by_username(&username).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Not Found".to_string(),
            message: format!("User with username {} not found", username),
        }),
        Err(e) => {
            error!("Failed to get user by username: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// List all users
#[utoipa::path(
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserDto>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn list_users(state: web::Data<AppState>) -> impl Responder {
    info!("Listing all users");

    match state.user_service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            error!("Failed to list users: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Enable a user
#[utoipa::path(
    put,
    path = "/api/v1/users/{user_id}/enable",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User enabled successfully", body = SuccessResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn enable_user(state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    info!("Enabling user: {}", user_id);

    match state.user_service.enable_user(&user_id).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: "User enabled successfully".to_string(),
        }),
        Err(e) => {
            error!("Failed to enable user: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Disable a user
#[utoipa::path(
    put,
    path = "/api/v1/users/{user_id}/disable",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User disabled successfully", body = SuccessResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn disable_user(
    state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    info!("Disabling user: {}", user_id);

    match state.user_service.disable_user(&user_id).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: "User disabled successfully".to_string(),
        }),
        Err(e) => {
            error!("Failed to disable user: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = SuccessResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    info!("Deleting user: {}", user_id);

    match state.user_service.delete_user(&user_id).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: "User deleted successfully".to_string(),
        }),
        Err(e) => {
            error!("Failed to delete user: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Assign role to user
#[utoipa::path(
    post,
    path = "/api/v1/users/{user_id}/roles",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = AssignRoleDto,
    responses(
        (status = 200, description = "Role assigned successfully", body = SuccessResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn assign_role(
    state: web::Data<AppState>,
    user_id: web::Path<String>,
    payload: web::Json<AssignRoleDto>,
) -> impl Responder {
    info!(
        "Assigning role '{}' to user: {}",
        payload.role_name, user_id
    );

    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    match state
        .user_service
        .assign_role(&user_id, &payload.role_name)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: format!("Role '{}' assigned successfully", payload.role_name),
        }),
        Err(e) => {
            error!("Failed to assign role: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Get user roles
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}/roles",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User roles", body = UserRolesDto),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn get_user_roles(
    state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    info!("Getting roles for user: {}", user_id);

    match state.user_service.get_user_roles(&user_id).await {
        Ok(roles) => HttpResponse::Ok().json(UserRolesDto {
            user_id: user_id.to_string(),
            roles,
        }),
        Err(e) => {
            error!("Failed to get user roles: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}
