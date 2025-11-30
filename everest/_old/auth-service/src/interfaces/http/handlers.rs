use actix_web::{web, HttpResponse};
use std::sync::Arc;
use validator::Validate;
use crate::{
    application::{
        dto::{CreateUserRequest, LoginRequest, UserResponse, EnrichedTokenResponse, LoginResponse, PublicTokenResponse},
        commands::CreateUserCommand,
        queries::{GetUserQuery, GetUserByKeycloakIdQuery, ListUsersByOrganisationQuery},
        handlers::{UserCommandHandler, UserQueryHandler},
    },
    domain::repositories::UserRepository,
    infrastructure::keycloak::client::KeycloakClient,
};
use super::errors::HttpError;
use crate::interfaces::middleware::auth::AuthenticatedUser;

pub struct UserHandlers<R: UserRepository> {
    command_handler: Arc<UserCommandHandler<R>>,
    query_handler: Arc<UserQueryHandler<R>>,
    keycloak_client: Arc<KeycloakClient>,
}

impl<R: UserRepository> UserHandlers<R> {
    pub fn new(
        command_handler: Arc<UserCommandHandler<R>>,
        query_handler: Arc<UserQueryHandler<R>>,
        keycloak_client: Arc<KeycloakClient>,
    ) -> Self {
        Self {
            command_handler,
            query_handler,
            keycloak_client,
        }
    }
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_user<R: UserRepository + 'static>(
    req: web::Json<CreateUserRequest>,
    auth_user: AuthenticatedUser,
    handlers: web::Data<UserHandlers<R>>,
) -> Result<HttpResponse, HttpError> {
    // Validate request
    req.validate()?;

    // Create user in Keycloak first
    let keycloak_id = handlers.keycloak_client
        .create_user(
            &req.username,
            &req.email,
            &req.password,
            &req.role.to_string(),
            req.organisation_name.as_deref(),
        )
        .await
        .map_err(|e| HttpError::ExternalServiceError(e.to_string()))?;

    // Create command with the real keycloak_id
    let command = CreateUserCommand {
        email: req.email.clone(),
        username: req.username.clone(),
        password: req.password.clone(), // Note: You might not want to store this in your DB
        role: req.role.clone(),
        organisation_name: req.organisation_name.clone(),
        requester_role: auth_user.role,
        keycloak_id: Some(keycloak_id.clone()), // Add keycloak_id to the command
    };

    // Execute command
    let user = handlers.command_handler.handle_create_user(command).await?;
    
    let response: UserResponse = user.into();
    Ok(HttpResponse::Created().json(response))
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = EnrichedTokenResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login<R: UserRepository + 'static>(
    req: web::Json<LoginRequest>,
    handlers: web::Data<UserHandlers<R>>,
) -> Result<HttpResponse, HttpError> {
    // Authenticate with Keycloak
    let token_response = handlers.keycloak_client
        .authenticate(&req.username, &req.password)
        .await
        .map_err(|e| HttpError::Unauthorized(e.to_string()))?;

    // Convert to public response
    let public_token = PublicTokenResponse::from(&token_response);

    // Decode token to get keycloak_id (sub claim)
    let claims = crate::infrastructure::jwt::decode_token(
        &public_token.access_token,
        "PLACEHOLDER_PUBLIC_KEY"
    )
    .map_err(|e| {
        log::warn!("Failed to decode token: {}", e);
        HttpError::Unauthorized("Invalid token".to_string())
    })?;

    // Find user in our database using keycloak_id from claims
    let user = handlers.query_handler
        .handle_get_user_by_keycloak_id(GetUserByKeycloakIdQuery {
            keycloak_id: claims.sub, // Use the subject from JWT claims
        })
        .await?;

    // Build response based on whether user was found
    let response = if let Some(user) = user {
        // User found - return enriched response
        EnrichedTokenResponse {
            access_token: public_token.access_token,
            refresh_token: public_token.refresh_token,
            expires_in: public_token.expires_in,
            token_type: public_token.token_type,
            user: user.into(),
        }
    } else {
        // User not found in our DB - return basic token response
        // You might want to create the user in your DB here, or handle this case differently
        return Ok(HttpResponse::Ok().json(LoginResponse {
            access_token: public_token.access_token,
            refresh_token: public_token.refresh_token,
            expires_in: public_token.expires_in,
            token_type: public_token.token_type,
        }));
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user<R: UserRepository + 'static>(
    user_id: web::Path<uuid::Uuid>,
    _auth_user: AuthenticatedUser,
    handlers: web::Data<UserHandlers<R>>,
) -> Result<HttpResponse, HttpError> {
    let user = handlers.query_handler
        .handle_get_user(GetUserQuery { user_id: *user_id })
        .await?
        .ok_or(HttpError::NotFound("User not found".to_string()))?;

    let response: UserResponse = user.into();
    Ok(HttpResponse::Ok().json(response))
}

/// List users by organisation
#[utoipa::path(
    get,
    path = "/api/v1/organisations/{org_name}/users",
    responses(
        (status = 200, description = "Users found", body = Vec<UserResponse>),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("org_name" = String, Path, description = "Organisation name")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users_by_organisation<R: UserRepository + 'static>(
    org_name: web::Path<String>,
    auth_user: AuthenticatedUser,
    handlers: web::Data<UserHandlers<R>>,
) -> Result<HttpResponse, HttpError> {
    let users = handlers.query_handler
        .handle_list_by_organisation(ListUsersByOrganisationQuery {
            organisation_name: org_name.into_inner(),
            requester_id: auth_user.user_id,
        })
        .await?;

    let response: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
    Ok(HttpResponse::Ok().json(response))
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service"
    }))
}