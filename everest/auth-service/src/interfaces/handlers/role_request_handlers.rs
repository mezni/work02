use crate::application::dto::role_request_dto::{
    CreateRoleRequestDto, ReviewRoleRequestDto, ErrorResponse, SuccessResponse,
};
use crate::application::services::role_request_service::RoleRequestError;
use crate::interfaces::AppState;
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};
use validator::Validate;

/// Create a role request
#[utoipa::path(
    post,
    path = "/api/v1/role-requests",
    request_body = CreateRoleRequestDto,
    responses(
        (status = 201, description = "Role request created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Active role request already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Role Requests"
)]
pub async fn create_role_request(
    state: web::Data<AppState>,
    payload: web::Json<CreateRoleRequestDto>,
) -> impl Responder {
    info!("Creating role request for user");

    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    // In a real implementation, you would get the user ID from the authenticated session
    // For now, we'll require it to be passed in the request body or derive it from context
    let user_id = "current_user_id".to_string(); // This should come from auth middleware

    match state
        .role_request_service
        .create_role_request(user_id, payload.into_inner())
        .await
    {
        Ok(role_request) => {
            info!("Role request created successfully: {}", role_request.id);
            HttpResponse::Created().json(role_request)
        }
        Err(RoleRequestError::UserNotFound) => {
            error!("User not found for role request");
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: "User not found".to_string(),
            })
        }
        Err(RoleRequestError::ActiveRequestExists) => {
            error!("Active role request already exists for user");
            HttpResponse::Conflict().json(ErrorResponse {
                error: "Conflict".to_string(),
                message: "An active role request already exists for this user".to_string(),
            })
        }
        Err(RoleRequestError::InvalidRole(role)) => {
            error!("Invalid role requested: {}", role);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Role".to_string(),
                message: format!("Role '{}' is not valid", role),
            })
        }
        Err(e) => {
            error!("Failed to create role request: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// List all role requests (for admins)
#[utoipa::path(
    get,
    path = "/api/v1/role-requests",
    params(
        ("status" = Option<String>, Query, description = "Filter by status (pending, approved, denied)")
    ),
    responses(
        (status = 200, description = "List of role requests"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Role Requests"
)]
pub async fn list_role_requests(
    state: web::Data<AppState>,
    query: web::Query<ListRoleRequestsQuery>,
) -> impl Responder {
    let status_filter = query.status.clone();
    info!("Listing role requests with filter: {:?}", status_filter);

    match state
        .role_request_service
        .list_role_requests(status_filter)
        .await
    {
        Ok(requests) => {
            info!("Retrieved {} role requests", requests.len());
            HttpResponse::Ok().json(requests)
        }
        Err(e) => {
            error!("Failed to list role requests: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Review a role request (admin only)
#[utoipa::path(
    put,
    path = "/api/v1/role-requests/{id}/review",
    params(
        ("id" = i32, Path, description = "Role Request ID")
    ),
    request_body = ReviewRoleRequestDto,
    responses(
        (status = 200, description = "Role request reviewed successfully", body = SuccessResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Role request not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Role Requests"
)]
pub async fn review_role_request(
    state: web::Data<AppState>,
    request_id: web::Path<i32>,
    payload: web::Json<ReviewRoleRequestDto>,
) -> impl Responder {
    info!("Reviewing role request: {}", request_id);

    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    // In a real implementation, you would get the reviewer ID from the authenticated session
    let reviewer_id = "admin_user_id".to_string(); // This should come from auth middleware

    match state
        .role_request_service
        .review_role_request(*request_id, payload.into_inner(), &reviewer_id)
        .await
    {
        Ok(_) => {
            info!("Role request reviewed successfully: {}", request_id);
            HttpResponse::Ok().json(SuccessResponse {
                message: "Role request reviewed successfully".to_string(),
            })
        }
        Err(RoleRequestError::NotFound) => {
            error!("Role request not found: {}", request_id);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: format!("Role request with ID {} not found", request_id),
            })
        }
        Err(RoleRequestError::AlreadyProcessed) => {
            error!("Role request already processed: {}", request_id);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Already Processed".to_string(),
                message: "This role request has already been processed".to_string(),
            })
        }
        Err(RoleRequestError::InvalidStatus(status)) => {
            error!("Invalid status for review: {}", status);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Status".to_string(),
                message: format!("Status '{}' is not valid for review", status),
            })
        }
        Err(e) => {
            error!("Failed to review role request: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Get role requests for a specific user
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}/role-requests",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "List of user's role requests"),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Role Requests"
)]
pub async fn get_user_role_requests(
    state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    info!("Getting role requests for user: {}", user_id);

    match state
        .role_request_service
        .get_user_role_requests(&user_id)
        .await
    {
        Ok(requests) => {
            info!("Retrieved {} role requests for user {}", requests.len(), user_id);
            HttpResponse::Ok().json(requests)
        }
        Err(RoleRequestError::UserNotFound) => {
            error!("User not found: {}", user_id);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: format!("User with ID {} not found", user_id),
            })
        }
        Err(e) => {
            error!("Failed to get user role requests: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

// Query struct for list_role_requests
#[derive(Debug, serde::Deserialize)]
pub struct ListRoleRequestsQuery {
    pub status: Option<String>,
}