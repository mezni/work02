#!/bin/bash
# create_application_layer_fixed.sh
# Creates the Application layer structure and starter Rust files with CORRECT DDD separation

SERVICE_DIR="auth-service"

echo "Applying fixes and enhancements to the Application layer structure..."

cd $SERVICE_DIR

# --- 1. Create/Fix directories (ensuring all are present)
mkdir -p src/application/commands
mkdir -p src/application/queries
mkdir -p src/application/dto
mkdir -p src/application/handlers

# --- 2. Create mod.rs (Correct)
cat > src/application/mod.rs <<EOL
pub mod commands;
pub mod queries;
pub mod dto;
pub mod handlers;
pub mod errors;
EOL

# --- 3. Fix errors.rs (Correctly includes Infrastructure dependencies)
cat > src/application/errors.rs <<EOL
use thiserror::Error;
use crate::domain::errors::DomainError;
use crate::infrastructure::{
    keycloak::keycloak_client::KeycloakError,
    jwt::token_enricher::JwtError,
};

// FIX: Properly map SQLx, Keycloak, and JWT errors from Infrastructure
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Keycloak client error: {0}")]
    Keycloak(#[from] KeycloakError), // Map specific Infrastructure error
    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError), // Map specific Infrastructure error
    #[error("Authentication failed: Invalid credentials.")]
    AuthenticationFailed,
    #[error("User not found in local store.")]
    UserStoreNotFound,
}

// FIX: Helper to convert domain errors for use in application layer
impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        ApplicationError::Domain(error)
    }
}
EOL

# --- 4. Create ALL Request DTOs (Missing in original)
cat > src/application/dto/auth_request.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct LoginRequest {
    #[schema(example = "testuser")]
    pub username: String,
    #[schema(example = "securepassword123")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct RegisterRequest {
    #[schema(example = "newuser")]
    pub username: String,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "securepassword123")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct AdminCreateUserRequest {
    #[schema(example = "admin_created_user")]
    pub username: String,
    #[schema(example = "admin@example.com")]
    pub email: String,
    #[schema(example = "securepassword123")]
    pub password: String,
    #[schema(example = "partner")]
    pub role: String, // Target role (admin, partner, operator)
    #[schema(example = "12345678-0000-0000-0000-000000000001", nullable = true)]
    pub organisation_id: Option<uuid::Uuid>,
    #[schema(example = "12345678-0000-0000-0000-000000000001", nullable = true)]
    pub station_id: Option<uuid::Uuid>,
}
EOL

# --- 5. DTOs (Responses)
cat > src/application/dto/user_dto.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

// FIX: DTO should expose core IDs and use the domain Role string representation
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}
EOL

cat > src/application/dto/auth_response.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

// FIX: Use 'token' instead of 'access_token' for JWT consistency
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    #[schema(example = "bearer")]
    pub token_type: String,
    #[schema(example = 3600)]
    pub expires_in: i64,
}
EOL

# --- 6. Commands (Use Case Definitions)
cat > src/application/commands/register_user.rs <<EOL
use crate::application::dto::auth_request::RegisterRequest;

// Command struct defining the input for the use case
pub struct RegisterUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<RegisterRequest> for RegisterUserCommand {
    fn from(req: RegisterRequest) -> Self {
        RegisterUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}
EOL

cat > src/application/commands/admin_create_user.rs <<EOL
use crate::application::dto::auth_request::AdminCreateUserRequest;
use uuid::Uuid;

// Command struct defining the input for the admin creation use case
pub struct AdminCreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}

impl From<AdminCreateUserRequest> for AdminCreateUserCommand {
    fn from(req: AdminCreateUserRequest) -> Self {
        AdminCreateUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
            role: req.role,
            organisation_id: req.organisation_id,
            station_id: req.station_id,
        }
    }
}
EOL
touch src/application/commands/mod.rs

# --- 7. Handlers (Use Case Implementation - MUST BE PURE)
# FIX: Handlers are now pure Rust logic, accepting Commands and returning DTOs/Results.
# Actix logic must be removed.

cat > src/application/handlers/register_handler.rs <<EOL
use crate::domain::repositories::UserRepository;
use crate::infrastructure::keycloak::keycloak_client::KeycloakClient;
use crate::application::{
    commands::register_user::RegisterUserCommand,
    errors::ApplicationError,
    dto::user_dto::UserDTO,
};

pub struct RegisterUserHandler<R: UserRepository, C: KeycloakClient> {
    user_repo: R,
    keycloak_client: C,
}

impl<R: UserRepository, C: KeycloakClient> RegisterUserHandler<R, C> {
    pub fn new(user_repo: R, keycloak_client: C) -> Self {
        RegisterUserHandler { user_repo, keycloak_client }
    }

    // The execute function defines the use case orchestration
    pub async fn execute(&self, cmd: RegisterUserCommand) -> Result<UserDTO, ApplicationError> {
        // 1. Domain Validation (e.g., uniqueness check via repo)
        let username_vo = crate::domain::value_objects::Username::parse(cmd.username.clone())?;
        if self.user_repo.get_by_username(&username_vo).await?.is_some() {
            return Err(ApplicationError::Validation("Username already taken.".to_string()));
        }

        // 2. Infrastructure: Create user in Keycloak (gets Keycloak ID)
        let keycloak_id = self.keycloak_client
            .create_user(cmd.username.clone(), cmd.email.clone(), cmd.password, crate::domain::value_objects::Role::RegisteredUser)
            .await?;

        // 3. Domain: Create local user entity
        let email_vo = crate::domain::value_objects::Email::parse(cmd.email.clone())?;
        let new_user = crate::domain::models::User::register(keycloak_id, username_vo, email_vo);
        
        // 4. Infrastructure: Persist to DB
        let saved_user = self.user_repo.save(new_user).await?;

        // 5. Output mapping
        Ok(UserDTO {
            id: saved_user.id,
            username: saved_user.username.to_string(),
            email: saved_user.email.to_string(),
            role: saved_user.role.to_string(),
            organisation_id: saved_user.organisation_id,
            station_id: saved_user.station_id,
        })
    }
}
EOL

cat > src/application/handlers/login_handler.rs <<EOL
use crate::{
    domain::repositories::UserRepository,
    infrastructure::{
        keycloak::keycloak_client::KeycloakClient,
        jwt::token_enricher::JwtTokenEnricher,
    },
    application::{
        dto::{
            auth_request::LoginRequest, 
            auth_response::AuthResponse
        },
        errors::ApplicationError,
    },
};

pub struct LoginHandler<R: UserRepository, C: KeycloakClient> {
    user_repo: R,
    keycloak_client: C,
    jwt_enricher: JwtTokenEnricher,
}

impl<R: UserRepository, C: KeycloakClient> LoginHandler<R, C> {
    pub fn new(user_repo: R, keycloak_client: C, jwt_enricher: JwtTokenEnricher) -> Self {
        LoginHandler { user_repo, keycloak_client, jwt_enricher }
    }

    pub async fn execute(&self, req: LoginRequest) -> Result<AuthResponse, ApplicationError> {
        // 1. Infrastructure: Authenticate with Keycloak
        let raw_jwt = self.keycloak_client
            .login(&req.username, &req.password)
            .await?;

        // 2. Infrastructure: Get the Keycloak ID (needed to lookup local user)
        let keycloak_id = self.keycloak_client.get_user_id_by_username(&req.username).await?;

        // 3. Domain: Lookup local user entity based on Keycloak ID
        let user = self.user_repo.get_by_keycloak_id(keycloak_id).await?
            .ok_or(ApplicationError::UserStoreNotFound)?;

        // 4. Infrastructure: Enrich and re-encode the JWT with local data
        let enriched_token = self.jwt_enricher
            .enrich_and_encode(&user)
            .await?;

        // 5. Output mapping (FIX: The token fields should be hardcoded or configurable)
        Ok(AuthResponse {
            token: enriched_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600, // 1 hour expiration
        })
    }
}
EOL
touch src/application/handlers/admin_create_user_handler.rs
touch src/application/handlers/mod.rs
touch src/application/queries/mod.rs

echo "✅ Application layer structure fixed, separating use cases (Handlers) from web framework (Actix-Web)."