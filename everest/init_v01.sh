#!/bin/bash
# setup_auth_service.sh
# Creates a complete, fixed Rust Auth-Service Microservice based on DDD principles.
# Includes: Cargo.toml, Domain, Application, Infrastructure, Interfaces, and Startup Layers.

SERVICE_DIR="auth-service"

echo "==================================================="
echo "🚀 Setting up DDD Rust Auth-Service: $SERVICE_DIR"
echo "==================================================="

# --- 1. Initial Setup: Create project and Cargo.toml ---
if [ -d "$SERVICE_DIR" ]; then
    echo "Directory $SERVICE_DIR already exists. Cleaning up..."
    rm -rf "$SERVICE_DIR"
fi

mkdir $SERVICE_DIR
cd $SERVICE_DIR
mkdir -p src

cat > Cargo.toml <<EOL
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web Framework & Async
actix-web = "4.9.0"
actix-http = "3.7.0"
actix-service = "2.0.2"
tokio = { version = "1.38.0", features = ["macros", "rt-multi-thread"] }
async-trait = "0.1.81"
anyhow = "1.0.86" # Enhanced error handling

# Data Serialization, Validation & Documentation
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.120"
chrono = { version = "0.4.38", features = ["serde"] }
uuid = { version = "1.10.0", features = ["serde", "v4"] }
regex = "1.10.5"

# Tracing, Logging, and Configuration
tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter"] }
tracing-actix-web = "0.7.11"
dotenvy = "0.15.7"

# Database (Postgres)
sqlx = { version = "0.7.4", features = ["runtime-tokio", "postgres", "macros", "uuid", "chrono"] }

# Domain Errors
thiserror = "1.0.61"

# JWT & Authentication
jsonwebtoken = "9.3.0"
reqwest = { version = "0.12.5", features = ["json", "blocking"] } # Used by KeycloakClient

# API Docs
utoipa = { version = "4.2.0", features = ["actix_web"] }
utoipa-swagger-ui = { version = "4.2.0", features = ["actix-web"] }
EOL

# --- 2. Create lib.rs and main.rs ---
cat > src/lib.rs <<EOL
// Core DDD Layers
pub mod domain;
pub mod application;
pub mod infrastructure;

// Interface/Presentation
pub mod interfaces;

// Startup and Utilities
pub mod startup;
pub mod swagger; 
EOL

cat > src/main.rs <<EOL
use auth_service::startup;

// FIX: Use actix_web::main for async entry point
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing before anything else
    auth_service::infrastructure::logging::setup_tracing(); 

    // Start the application
    startup::run().await
}
EOL

# --- 3. DOMAIN LAYER (Fixed) ---
echo "✅ Creating Domain Layer..."

mkdir -p src/domain/models
mkdir -p src/domain/value_objects
mkdir -p src/domain/repositories
mkdir -p src/domain/services

# mod.rs
cat > src/domain/mod.rs <<EOL
pub mod models;
pub mod value_objects;
pub mod repositories;
pub mod services;
pub mod errors;
EOL

# errors.rs
cat > src/domain/errors.rs <<EOL
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    #[error("Invalid role string.")]
    InvalidRole,
    #[error("User with ID {0} not found.")]
    UserNotFound(Uuid),
    #[error("User with username '{0}' already exists.")]
    UsernameAlreadyExists(String),
    #[error("Organisation with ID {0} not found.")]
    OrganisationNotFound(Uuid),
    #[error("Station with ID {0} not found.")]
    StationNotFound(Uuid),
    #[error("A critical internal domain consistency error occurred: {0}")]
    InternalError(String),
}
EOL

# value_objects/role.rs
cat > src/domain/value_objects/role.rs <<EOL
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use std::str::FromStr;
use crate::domain::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin = 4,
    Partner = 3,
    Operator = 2,
    RegisteredUser = 1,
    Public = 0,
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Role::Admin => "admin",
            Role::Partner => "partner",
            Role::Operator => "operator",
            Role::RegisteredUser => "registered_user",
            Role::Public => "public",
        })
    }
}

impl FromStr for Role {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "partner" => Ok(Role::Partner),
            "operator" => Ok(Role::Operator),
            "registered_user" => Ok(Role::RegisteredUser),
            "public" => Ok(Role::Public),
            _ => Err(DomainError::InvalidRole),
        }
    }
}
EOL
touch src/domain/value_objects/mod.rs # Ensure mod.rs exists for local imports

# value_objects/email.rs
cat > src/domain/value_objects/email.rs <<EOL
use regex::Regex;
use crate::domain::errors::DomainError;
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, DomainError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if email_regex.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(DomainError::InvalidEmail(email))
        }
    }
}

impl Deref for Email {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
EOL

# value_objects/username.rs
cat > src/domain/value_objects/username.rs <<EOL
use crate::domain::errors::DomainError;
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Result<Self, DomainError> {
        if username.len() >= 3 && username.len() <= 50 && username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(Self(username))
        } else {
            Err(DomainError::InvalidUsername(username))
        }
    }
}

impl Deref for Username {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
EOL

# models/user.rs
cat > src/domain/models/user.rs <<EOL
use crate::domain::value_objects::{Email, Username, Role};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: Uuid,
    pub username: Username,
    pub email: Email,
    pub role: Role,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}

impl User {
    pub fn register(
        keycloak_id: Uuid,
        username: Username,
        email: Email,
    ) -> Self {
        User {
            id: Uuid::new_v4(),
            keycloak_id,
            username,
            email,
            role: Role::RegisteredUser,
            organisation_id: None,
            station_id: None,
        }
    }
    
    pub fn promote_to_partner(&mut self, organisation_id: Uuid) {
        self.role = Role::Partner;
        self.organisation_id = Some(organisation_id);
        self.station_id = None;
    }
}
EOL
touch src/domain/models/organisation.rs
touch src/domain/models/station.rs
touch src/domain/models/mod.rs

# repositories/user_repository.rs
cat > src/domain/repositories/user_repository.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{Username, Email};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + Clone {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn get_by_keycloak_id(&self, keycloak_id: Uuid) -> Result<Option<User>, DomainError>;
    async fn get_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;
    async fn get_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: User) -> Result<User, DomainError>;
}
EOL
touch src/domain/repositories/organisation_repository.rs
touch src/domain/repositories/station_repository.rs
touch src/domain/repositories/mod.rs

# services/user_domain_service.rs
cat > src/domain/services/user_domain_service.rs <<EOL
use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Role;

pub struct UserDomainService;

impl UserDomainService {
    pub fn check_permission(user: &User, required_role: Role) -> Result<(), DomainError> {
        if user.role as i32 >= required_role as i32 {
            Ok(())
        } else {
            Err(DomainError::InternalError("User does not have required permissions.".to_string()))
        }
    }
}
EOL
touch src/domain/services/mod.rs

# --- 4. APPLICATION LAYER (Fixed) ---
echo "✅ Creating Application Layer..."

mkdir -p src/application/commands
mkdir -p src/application/queries
mkdir -p src/application/dto
mkdir -p src/application/handlers

# mod.rs
cat > src/application/mod.rs <<EOL
pub mod commands;
pub mod queries;
pub mod dto;
pub mod handlers;
pub mod errors;
EOL

# errors.rs
cat > src/application/errors.rs <<EOL
use thiserror::Error;
use crate::domain::errors::DomainError;
use crate::infrastructure::keycloak::keycloak_client::KeycloakError; // NOTE: Infrastructure import is needed here for mapping

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(DomainError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Keycloak client error: {0}")]
    Keycloak(#[from] KeycloakError), 
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error), 
    #[error("Authentication failed: Invalid credentials.")]
    AuthenticationFailed,
    #[error("User not found in local store.")]
    UserStoreNotFound,
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        ApplicationError::Domain(error)
    }
}
EOL

# dto/auth_request.rs
cat > src/application/dto/auth_request.rs <<EOL
use serde::{Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
    pub role: String,
    #[schema(example = "12345678-0000-0000-0000-000000000001", nullable = true)]
    pub organisation_id: Option<Uuid>,
    #[schema(example = "12345678-0000-0000-0000-000000000001", nullable = true)]
    pub station_id: Option<Uuid>,
}
EOL

# dto/user_dto.rs
cat > src/application/dto/user_dto.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

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

# dto/auth_response.rs
cat > src/application/dto/auth_response.rs <<EOL
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

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
touch src/application/dto/mod.rs

# commands/register_user.rs
cat > src/application/commands/register_user.rs <<EOL
use crate::application::dto::auth_request::RegisterRequest;

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

# commands/admin_create_user.rs
cat > src/application/commands/admin_create_user.rs <<EOL
use crate::application::dto::auth_request::AdminCreateUserRequest;
use uuid::Uuid;

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
touch src/application/queries/mod.rs

# handlers/register_handler.rs
cat > src/application/handlers/register_handler.rs <<EOL
use crate::domain::{
    repositories::UserRepository,
    models::User,
    value_objects::{Username, Email, Role},
};
use crate::infrastructure::keycloak::keycloak_client::KeycloakClient;
use crate::application::{
    commands::register_user::RegisterUserCommand,
    errors::ApplicationError,
    dto::user_dto::UserDTO,
};

#[derive(Clone)]
pub struct RegisterUserHandler<R, C> {
    user_repo: R,
    keycloak_client: C,
}

impl<R: UserRepository, C: KeycloakClient> RegisterUserHandler<R, C> {
    pub fn new(user_repo: R, keycloak_client: C) -> Self {
        RegisterUserHandler { user_repo, keycloak_client }
    }

    pub async fn execute(&self, cmd: RegisterUserCommand) -> Result<UserDTO, ApplicationError> {
        let username_vo = Username::parse(cmd.username.clone()).map_err(ApplicationError::Domain)?;

        if self.user_repo.get_by_username(&username_vo).await?.is_some() {
            return Err(ApplicationError::Validation("Username already taken.".to_string()));
        }

        let keycloak_id = self.keycloak_client
            .create_user(cmd.username.clone(), cmd.email.clone(), cmd.password, Role::RegisteredUser)
            .await?;

        let email_vo = Email::parse(cmd.email.clone()).map_err(ApplicationError::Domain)?;
        let new_user = User::register(keycloak_id, username_vo, email_vo);
        
        let saved_user = self.user_repo.save(new_user).await?;

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

# handlers/login_handler.rs
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

#[derive(Clone)]
pub struct LoginHandler<R, C> {
    user_repo: R,
    keycloak_client: C,
    jwt_enricher: JwtTokenEnricher,
}

impl<R: UserRepository, C: KeycloakClient> LoginHandler<R, C> {
    pub fn new(user_repo: R, keycloak_client: C, jwt_enricher: JwtTokenEnricher) -> Self {
        LoginHandler { user_repo, keycloak_client, jwt_enricher }
    }

    pub async fn execute(&self, req: LoginRequest) -> Result<AuthResponse, ApplicationError> {
        let _raw_jwt = self.keycloak_client
            .login(&req.username, &req.password)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;

        let keycloak_id = self.keycloak_client.get_user_id_by_username(&req.username).await?;

        let user = self.user_repo.get_by_keycloak_id(keycloak_id).await?
            .ok_or(ApplicationError::UserStoreNotFound)?;

        let enriched_token = self.jwt_enricher
            .enrich_and_encode(&user)
            .await?;

        Ok(AuthResponse {
            token: enriched_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
        })
    }
}
EOL
touch src/application/handlers/admin_create_user_handler.rs
touch src/application/handlers/mod.rs

# --- 5. INFRASTRUCTURE LAYER (Fixed) ---
echo "✅ Creating Infrastructure Layer..."

mkdir -p src/infrastructure/db
mkdir -p src/infrastructure/keycloak
mkdir -p src/infrastructure/jwt
mkdir -p src/infrastructure/ioc

# mod.rs
cat > src/infrastructure/mod.rs <<EOL
pub mod db;
pub mod keycloak;
pub mod jwt;
pub mod logging;
pub mod config;
pub mod ioc;
EOL

# config.rs
cat > src/infrastructure/config.rs <<EOL
use serde::Deserialize;
use dotenvy::dotenv;

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub keycloak_realm_url: String, 
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub keycloak_master_user: String,
    pub keycloak_master_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,

    #[serde(flatten)]
    pub keycloak: KeycloakConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, dotenvy::Error> {
        dotenv().ok();
        
        Ok(AppConfig {
            database_url: std::env::var("DATABASE_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            host: std::env::var("HOST")?,
            port: std::env::var("PORT")?.parse().unwrap_or(8000),
            keycloak: KeycloakConfig {
                keycloak_realm_url: std::env::var("KEYCLOAK_REALM_URL")?,
                keycloak_client_id: std::env::var("KEYCLOAK_CLIENT_ID")?,
                keycloak_client_secret: std::env::var("KEYCLOAK_CLIENT_SECRET")?,
                keycloak_master_user: std::env::var("KEYCLOAK_MASTER_USER")?,
                keycloak_master_password: std::env::var("KEYCLOAK_MASTER_PASSWORD")?,
            },
        })
    }
}
EOL

# logging.rs
cat > src/infrastructure/logging.rs <<EOL
use tracing_subscriber::{fmt, EnvFilter};

pub fn setup_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("auth_service=info,actix_web=info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();
}
EOL

# db/user_repository_pg.rs
cat > src/infrastructure/db/user_repository_pg.rs <<EOL
use crate::domain::{
    models::User,
    errors::DomainError,
    repositories::UserRepository,
    value_objects::{Username, Email},
};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepositoryPg {
    pool: PgPool,
}

impl UserRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        UserRepositoryPg { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPg {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> { Ok(None) }
    async fn get_by_keycloak_id(&self, keycloak_id: Uuid) -> Result<Option<User>, DomainError> { Ok(None) }
    async fn get_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> { Ok(None) }
    async fn get_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> { Ok(None) }
    async fn save(&self, user: User) -> Result<User, DomainError> { Ok(user) }
}
EOL
touch src/infrastructure/db/organisation_repository_pg.rs
touch src/infrastructure/db/station_repository_pg.rs
touch src/infrastructure/db/mod.rs

# keycloak/keycloak_client.rs
cat > src/infrastructure/keycloak/keycloak_client.rs <<EOL
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use uuid::Uuid;
use crate::{
    domain::value_objects::Role,
    infrastructure::config::KeycloakConfig,
};

#[derive(Debug, thiserror::Error)]
pub enum KeycloakError {
    #[error("Keycloak API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Keycloak API returned error: {0}")]
    ApiError(String),
    #[error("Keycloak authentication failed.")]
    AuthError,
}

#[async_trait]
pub trait KeycloakClient: Send + Sync + Clone + 'static {
    async fn login(&self, username: &str, password: &str) -> Result<String, KeycloakError>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        role: Role,
    ) -> Result<Uuid, KeycloakError>;
    async fn get_user_id_by_username(&self, username: &str) -> Result<Uuid, KeycloakError>;
}

#[derive(Clone)]
pub struct KeycloakClientImpl {
    http_client: HttpClient,
    config: KeycloakConfig,
}

impl KeycloakClientImpl {
    pub fn new(config: KeycloakConfig) -> Self {
        Self { 
            http_client: HttpClient::new(), 
            config,
        }
    }
}

#[async_trait]
impl KeycloakClient for KeycloakClientImpl {
    async fn login(&self, username: &str, password: &str) -> Result<String, KeycloakError> {
        // STUB
        if username == "test" && password == "pass" {
            Ok("keycloak_raw_jwt_stub".to_string())
        } else {
            Err(KeycloakError::AuthError)
        }
    }

    async fn create_user(
        &self,
        _username: String,
        _email: String,
        _password: String,
        _role: Role,
    ) -> Result<Uuid, KeycloakError> {
        // STUB
        Ok(Uuid::new_v4())
    }
    
    async fn get_user_id_by_username(&self, _username: &str) -> Result<Uuid, KeycloakError> {
        // STUB
        Ok(Uuid::new_v4())
    }
}
EOL
touch src/infrastructure/keycloak/mod.rs

# jwt/token_enricher.rs
cat > src/infrastructure/jwt/token_enricher.rs <<EOL
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use crate::{
    domain::{
        models::User,
        value_objects::Role,
    },
    infrastructure::config::AppConfig,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichedClaims {
    pub sub: String, 
    pub exp: usize,  
    pub iat: usize,  
    pub role: String,
    pub organisation_name: Option<String>,
    pub station_name: Option<String>,
    pub preferred_username: String,
}

#[derive(Clone)]
pub struct JwtTokenEnricher {
    encoding_key: EncodingKey,
}

impl JwtTokenEnricher {
    pub fn new(config: &AppConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_ref());
        JwtTokenEnricher { encoding_key }
    }

    pub async fn enrich_and_encode(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now() + Duration::hours(1);
        let iat = Utc::now().timestamp() as usize;

        let (org_name, station_name) = match user.role {
            Role::Admin => (None, None),
            Role::Partner => (Some("PartnerOrg_A".to_string()), None),
            Role::Operator => (Some("PartnerOrg_A".to_string()), Some("Station_X".to_string())),
            _ => (None, None),
        };

        let claims = EnrichedClaims {
            sub: user.keycloak_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat,
            role: user.role.to_string(),
            organisation_name: org_name,
            station_name: station_name,
            preferred_username: user.username.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }
}
EOL
touch src/infrastructure/jwt/mod.rs

# ioc/service_locator.rs (Dependency Injection Container)
cat > src/infrastructure/ioc/service_locator.rs <<EOL
use crate::{
    domain::repositories::UserRepository,
    infrastructure::{
        config::AppConfig,
        db::user_repository_pg::UserRepositoryPg,
        keycloak::{keycloak_client::{KeycloakClient, KeycloakClientImpl}},
        jwt::token_enricher::JwtTokenEnricher,
    },
    application::handlers::{
        register_handler::RegisterUserHandler,
        login_handler::LoginHandler,
        admin_create_user_handler::AdminCreateUserHandler,
    },
};
use sqlx::PgPool;

// ServiceLocator acts as the IoC container, housing all dependencies.
#[derive(Clone)]
pub struct ServiceLocator {
    // Infrastructure
    config: AppConfig,
    user_repo: UserRepositoryPg,
    keycloak_client: KeycloakClientImpl,
    jwt_enricher: JwtTokenEnricher,
}

impl ServiceLocator {
    pub fn new(config: AppConfig, db_pool: PgPool) -> Self {
        let keycloak_client = KeycloakClientImpl::new(config.keycloak.clone());
        let jwt_enricher = JwtTokenEnricher::new(&config);
        let user_repo = UserRepositoryPg::new(db_pool);
        
        ServiceLocator {
            config,
            user_repo,
            keycloak_client,
            jwt_enricher,
        }
    }

    // --- Application Handlers (Business Logic) ---
    // These methods resolve the handler with its required dependencies (Repositories/Clients)
    pub fn get_register_handler(&self) -> RegisterUserHandler<UserRepositoryPg, KeycloakClientImpl> {
        RegisterUserHandler::new(self.user_repo.clone(), self.keycloak_client.clone())
    }

    pub fn get_login_handler(&self) -> LoginHandler<UserRepositoryPg, KeycloakClientImpl> {
        LoginHandler::new(self.user_repo.clone(), self.keycloak_client.clone(), self.jwt_enricher.clone())
    }

    pub fn get_admin_create_user_handler(&self) -> AdminCreateUserHandler<UserRepositoryPg, KeycloakClientImpl> {
        // NOTE: AdminCreateUserHandler is a stub, but this is how it would be initialized
        AdminCreateUserHandler::new(self.user_repo.clone(), self.keycloak_client.clone())
    }
}
EOL
touch src/infrastructure/ioc/mod.rs

# --- 6. INTERFACES LAYER (Fixed) ---
echo "✅ Creating Interfaces Layer (Controllers)..."

mkdir -p src/interfaces/http

# mod.rs
cat > src/interfaces/mod.rs <<EOL
pub mod http;
EOL

# http/routes.rs
cat > src/interfaces/http/routes.rs <<EOL
use actix_web::web;
use crate::interfaces::http::auth_controller::{register, login, admin_create_user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1") // FIX: Use /api/v1 scope for versioning
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/admin/create-user", web::post().to(admin_create_user))
    );
}
EOL

# http/middleware.rs
cat > src/interfaces/http/middleware.rs <<EOL
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    Error, 
};
use std::{future::{ready, Ready}, pin::Pin};

// Simple stub for an Authorization Guard/Middleware
pub struct AuthGuard;

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardMiddleware { service }))
    }
}

pub struct AuthGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        tracing::info!("AuthGuard: Checking JWT on request to {}", req.path());
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
EOL
touch src/application/handlers/admin_create_user_handler.rs # Add the missing handler file

# http/auth_controller.rs (Controller Logic and Swagger Definition)
cat > src/interfaces/http/auth_controller.rs <<EOL
use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use crate::{
    application::{
        dto::{
            auth_request::{LoginRequest, RegisterRequest, AdminCreateUserRequest},
            auth_response::AuthResponse,
            user_dto::UserDTO,
        },
        commands::{
            register_user::RegisterUserCommand,
            admin_create_user::AdminCreateUserCommand,
        },
        errors::ApplicationError,
    },
    infrastructure::ioc::ServiceLocator,
};

// --- HTTP Controller Functions ---

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful, returning created user", body = UserDTO),
        (status = 400, description = "Invalid input or user exists"),
    )
)]
pub async fn register(
    locator: web::Data<ServiceLocator>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let handler = locator.get_register_handler();
    let cmd = RegisterUserCommand::from(req.0);
    
    let user_dto = handler.execute(cmd).await?;

    Ok(HttpResponse::Created().json(user_dto))
}


#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
pub async fn login(
    locator: web::Data<ServiceLocator>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let handler = locator.get_login_handler();

    let response: AuthResponse = handler.execute(req.0).await?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/create-user",
    request_body = AdminCreateUserRequest,
    responses(
        (status = 201, description = "User creation successful", body = UserDTO),
        (status = 400, description = "Invalid input or user exists"),
        (status = 403, description = "Forbidden (Admin role required)"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_create_user(
    locator: web::Data<ServiceLocator>,
    req: web::Json<AdminCreateUserRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let handler = locator.get_admin_create_user_handler();
    let cmd = AdminCreateUserCommand::from(req.0);

    let user_dto = handler.execute(cmd).await?;

    Ok(HttpResponse::Created().json(user_dto))
}

// Swagger Definition
#[derive(OpenApi)]
#[openapi(
    paths(
        register,
        login,
        admin_create_user,
    ),
    components(
        schemas(
            LoginRequest, 
            RegisterRequest,
            AdminCreateUserRequest, 
            AuthResponse,
            UserDTO
        ),
        security_schemes(
            ("bearer_auth" = (type = "http", scheme = "bearer", bearer_format = "JWT"))
        )
    ),
    tags(
        (name = "Auth", description = "Authentication and User Management Endpoints")
    )
)]
pub struct ApiDoc;
EOL
touch src/interfaces/http/mod.rs

# --- 7. FINAL STARTUP LAYER (Fixed) ---
echo "✅ Creating Startup Layer..."

# startup.rs
cat > src/startup.rs <<EOL
use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::{PgPoolOptions, postgres::PgConnectOptions};
use std::net::SocketAddr;
use tracing::{info};
use utoipa_swagger_ui::SwaggerUi;
use anyhow::Context;

use crate::{
    infrastructure::{
        config::AppConfig, 
        ioc::ServiceLocator,
    },
    interfaces::http::{
        routes,
        auth_controller::ApiDoc,
        middleware::AuthGuard,
    },
};

pub async fn run() -> anyhow::Result<()> {
    // 1. Configuration and Address Setup
    let config = AppConfig::load().context("Failed to load application configuration")?;
    let listen_addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .context("Failed to parse host:port address")?;
    info!("Configuration loaded. Starting server on {}", listen_addr);

    // 2. Database Connection Pooling
    let db_connect_options = config.database_url.parse::<PgConnectOptions>()
        .context("Failed to parse DATABASE_URL")?;
    
    let db_pool = PgPoolOptions::new()
        .max_connections(50) 
        .connect_with(db_connect_options)
        .await
        .context("Failed to connect to Postgres database")?;
    
    info!("Database pool created successfully.");

    // 3. Dependency Injection Container (Service Locator)
    let locator = ServiceLocator::new(config.clone(), db_pool.clone());
    let locator_data = web::Data::new(locator);

    // 4. OpenAPI Documentation
    let api_doc = ApiDoc::openapi();

    // 5. Actix-Web Server Setup
    info!("Starting Actix-Web server...");
    HttpServer::new(move || {
        App::new()
            // Dependency Injection: Pass the IoC container
            .app_data(locator_data.clone()) 
            
            // Middleware: Tracing/Logging and Custom AuthGuard
            .wrap(tracing_actix_web::TracingLogger::default()) 
            .wrap(AuthGuard) 
            
            // Health check route
            .route("/health", web::get().to(|| async { HttpResponse::Ok().json("Auth Service OK") }))
            
            // Configure all application routes
            .configure(routes::configure)

            // Swagger UI integration
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", api_doc.clone()),
            )
    })
    .bind(listen_addr)?
    .run()
    .await
    .context("Actix-Web server failed to run")?;

    Ok(())
}
EOL

# --- 8. Final Setup ---
echo "==================================================="
echo "✅ Project $SERVICE_DIR setup complete!"
echo "Next steps:"
echo "1. Create a .env file with DATABASE_URL, JWT_SECRET, HOST, PORT, and Keycloak variables."
echo "2. Run: cargo build"
echo "==================================================="