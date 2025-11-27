#!/bin/bash

set -e

echo "Creating complete infrastructure layer with SQL migrations..."

cd auth-service

# Create infrastructure directories
mkdir -p src/infrastructure/{config,persistence,external,auth,errors}
mkdir -p src/infrastructure/persistence/{database,repositories}
mkdir -p migrations


# 1. Create the missing KeycloakClient in infrastructure/auth
cat > src/infrastructure/auth/keycloak.rs << 'EOF'
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::infrastructure::config::KeycloakConfig;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUserInfo {
    pub sub: String,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub token_type: String,
}

#[derive(Clone)]
pub struct KeycloakClient {
    client: Client,
    config: KeycloakConfig,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("username", username),
            ("password", password),
            ("grant_type", "password"),
            ("scope", "openid"),
        ];

        let response = self.client
            .post(&self.config.token_url())
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InfrastructureError::ExternalServiceError(
                format!("Keycloak login failed: {}", response.status())
            ));
        }

        let token_response: KeycloakTokenResponse = response.json().await?;
        Ok(token_response)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = self.client
            .post(&self.config.token_url())
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InfrastructureError::ExternalServiceError(
                format!("Keycloak token refresh failed: {}", response.status())
            ));
        }

        let token_response: KeycloakTokenResponse = response.json().await?;
        Ok(token_response)
    }

    pub async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError> {
        let response = self.client
            .get(&self.config.user_info_url())
            .bearer_auth(access_token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InfrastructureError::ExternalServiceError(
                format!("Keycloak user info failed: {}", response.status())
            ));
        }

        let user_info: KeycloakUserInfo = response.json().await?;
        Ok(user_info)
    }

    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String, InfrastructureError> {
        // First get admin token
        let admin_token = self.get_admin_token().await?;

        // Create user in Keycloak
        let user_representation = serde_json::json!({
            "username": username,
            "email": email,
            "enabled": true,
            "emailVerified": false,
            "credentials": [{
                "type": "password",
                "value": password,
                "temporary": false
            }]
        });

        let response = self.client
            .post(&self.config.admin_users_url())
            .bearer_auth(&admin_token)
            .json(&user_representation)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InfrastructureError::ExternalServiceError(
                format!("Keycloak user creation failed: {}", response.status())
            ));
        }

        // Extract user ID from location header
        if let Some(location) = response.headers().get("Location") {
            if let Ok(location_str) = location.to_str() {
                if let Some(user_id) = location_str.split('/').last() {
                    return Ok(user_id.to_string());
                }
            }
        }

        Err(InfrastructureError::ExternalServiceError(
            "Failed to extract user ID from Keycloak response".to_string()
        ))
    }

    async fn get_admin_token(&self) -> Result<String, InfrastructureError> {
        let params = [
            ("client_id", "admin-cli"),
            ("username", &self.config.admin_username),
            ("password", &self.config.admin_password),
            ("grant_type", "password"),
        ];

        let response = self.client
            .post(&self.config.admin_token_url())
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InfrastructureError::ExternalServiceError(
                format!("Admin token request failed: {}", response.status())
            ));
        }

        let token_response: KeycloakTokenResponse = response.json().await?;
        Ok(token_response.access_token)
    }
}
EOF

# 3. Fix the JWT service to remove the extra parameter
cat > src/infrastructure/auth/jwt.rs << 'EOF'
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::infrastructure::config::JwtConfig;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<String>,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        let expiration_seconds = config.expiration_days as u64 * 24 * 3600; // Convert days to seconds
        
        Self {
            encoding_key: EncodingKey::from_secret(config.secret.as_ref()),
            decoding_key: DecodingKey::from_secret(config.secret.as_ref()),
            expiration_seconds,
        }
    }

    pub fn generate_token(
        &self, 
        user_id: Uuid,
        username: &str, 
        email: &str, 
        role: &str,
        company_id: Option<Uuid>
    ) -> Result<String, InfrastructureError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| InfrastructureError::JwtError(e.to_string()))?
            .as_secs() as usize;

        let expiration = now + self.expiration_seconds as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: now,
            username: username.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            company_id: company_id.map(|id| id.to_string()),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, InfrastructureError> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn extract_user_id(&self, token: &str) -> Result<Uuid, InfrastructureError> {
        let claims = self.validate_token(token)?;
        Uuid::parse_str(&claims.sub)
            .map_err(|e| InfrastructureError::JwtError(format!("Invalid user ID in token: {}", e)))
    }
}
EOF

# 2. Fix the shared error.rs to remove unused variables
cat > src/shared/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    Unauthorized(String),
    Internal,
    DatabaseError(String),
    AuthError(String),
    BusinessError(String),
    ConfigurationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Internal => write!(f, "Internal Server Error"),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::AuthError(msg) => write!(f, "Authentication Error: {}", msg),
            AppError::BusinessError(msg) => write!(f, "Business Error: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(message) => HttpResponse::NotFound().json(ErrorResponse {
                error: "NOT_FOUND".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::ValidationError(message) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::Unauthorized(message) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "UNAUTHORIZED".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::Internal => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: "An internal server error occurred".to_string(),
                details: None,
            }),
            AppError::DatabaseError(message) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::AuthError(message) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "AUTH_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::BusinessError(message) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "BUSINESS_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
            AppError::ConfigurationError(message) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "CONFIGURATION_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(_e: std::io::Error) -> Self {
        AppError::Internal
    }
}

impl From<uuid::Error> for AppError {
    fn from(_e: uuid::Error) -> Self {
        AppError::ValidationError("Invalid UUID format".to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::ValidationError(format!("JSON serialization error: {}", e))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::AuthError(format!("JWT error: {}", e))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(e: bcrypt::BcryptError) -> Self {
        AppError::AuthError(format!("Password hashing error: {}", e))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(format!("Database error: {}", e)),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        AppError::ConfigurationError(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_e: reqwest::Error) -> Self {
        AppError::Internal
    }
}

impl From<crate::infrastructure::errors::InfrastructureError> for AppError {
    fn from(e: crate::infrastructure::errors::InfrastructureError) -> Self {
        match e {
            crate::infrastructure::errors::InfrastructureError::DatabaseError(msg) => AppError::DatabaseError(msg),
            crate::infrastructure::errors::InfrastructureError::JwtError(msg) => AppError::AuthError(msg),
            crate::infrastructure::errors::InfrastructureError::ConfigError(msg) => AppError::ConfigurationError(msg),
            crate::infrastructure::errors::InfrastructureError::ExternalServiceError(_msg) => AppError::Internal,
            crate::infrastructure::errors::InfrastructureError::KeycloakError(msg) => AppError::AuthError(msg),
            crate::infrastructure::errors::InfrastructureError::MigrationError(msg) => AppError::DatabaseError(msg),
            crate::infrastructure::errors::InfrastructureError::IoError(_e) => AppError::Internal,
        }
    }
}

impl From<crate::domain::errors::DomainError> for AppError {
    fn from(e: crate::domain::errors::DomainError) -> Self {
        match e {
            crate::domain::errors::DomainError::UserNotFound => AppError::NotFound("User not found".to_string()),
            crate::domain::errors::DomainError::CompanyNotFound => AppError::NotFound("Company not found".to_string()),
            crate::domain::errors::DomainError::UserAlreadyExists(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::CompanyAlreadyExists(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidEmail(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidPassword(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidUserRole(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::InvalidCompanyAssignment => AppError::ValidationError("Invalid company assignment".to_string()),
            crate::domain::errors::DomainError::UnauthorizedOperation => AppError::Unauthorized("Unauthorized operation".to_string()),
            crate::domain::errors::DomainError::ValidationError(msg) => AppError::ValidationError(msg),
            crate::domain::errors::DomainError::DomainRuleViolation(msg) => AppError::BusinessError(msg),
            crate::domain::errors::DomainError::InsufficientPermissions(msg) => AppError::Unauthorized(msg),
            crate::domain::errors::DomainError::InvalidOperation(msg) => AppError::BusinessError(msg),
        }
    }
}
EOF


# 2. Create infrastructure/errors.rs
cat > src/infrastructure/errors.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("JWT error: {0}")]
    JwtError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Keycloak error: {0}")]
    KeycloakError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(error: sqlx::Error) -> Self {
        InfrastructureError::DatabaseError(error.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for InfrastructureError {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        InfrastructureError::MigrationError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for InfrastructureError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        InfrastructureError::JwtError(error.to_string())
    }
}

impl From<config::ConfigError> for InfrastructureError {
    fn from(error: config::ConfigError) -> Self {
        InfrastructureError::ConfigError(error.to_string())
    }
}

impl From<reqwest::Error> for InfrastructureError {
    fn from(error: reqwest::Error) -> Self {
        InfrastructureError::ExternalServiceError(error.to_string())
    }
}

impl From<serde_json::Error> for InfrastructureError {
    fn from(error: serde_json::Error) -> Self {
        InfrastructureError::ExternalServiceError(format!("JSON error: {}", error))
    }
}
EOF

# 1. Fix the KeycloakConfig to include admin_token_url method
cat > src/infrastructure/config.rs << 'EOF'
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl KeycloakConfig {
    pub fn token_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/token", self.server_url, self.realm)
    }
    
    pub fn user_info_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/userinfo", self.server_url, self.realm)
    }
    
    pub fn admin_users_url(&self) -> String {
        format!("{}/admin/realms/{}/users", self.server_url, self.realm)
    }
    
    pub fn admin_token_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/token", self.server_url, self.realm)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration_days: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
    pub jwt: JwtConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn from_shared_config(shared_config: &crate::shared::config::AppConfig) -> Self {
        Self {
            database: DatabaseConfig {
                host: shared_config.database.host.clone(),
                port: shared_config.database.port,
                username: shared_config.database.username.clone(),
                password: shared_config.database.password.clone(),
                database_name: shared_config.database.database_name.clone(),
                max_connections: shared_config.database.max_connections,
            },
            keycloak: KeycloakConfig {
                server_url: shared_config.keycloak.server_url.clone(),
                realm: shared_config.keycloak.realm.clone(),
                client_id: shared_config.keycloak.client_id.clone(),
                client_secret: shared_config.keycloak.client_secret.clone(),
                admin_username: shared_config.keycloak.admin_username.clone(),
                admin_password: shared_config.keycloak.admin_password.clone(),
            },
            jwt: JwtConfig {
                secret: shared_config.jwt.secret.clone(),
                issuer: shared_config.jwt.issuer.clone(),
                audience: shared_config.jwt.audience.clone(),
                expiration_days: shared_config.jwt.expiration_days,
            },
            logging: LoggingConfig {
                level: shared_config.logging.level.clone(),
            },
        }
    }
}
EOF

# 2. Also update the shared config to match
cat > src/shared/config.rs << 'EOF'
use serde::Deserialize;
use config::{Config, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
    pub jwt: JwtConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl KeycloakConfig {
    pub fn token_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/token", self.server_url, self.realm)
    }
    
    pub fn user_info_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/userinfo", self.server_url, self.realm)
    }
    
    pub fn admin_users_url(&self) -> String {
        format!("{}/admin/realms/{}/users", self.server_url, self.realm)
    }
    
    pub fn admin_token_url(&self) -> String {
        format!("{}/realms/{}/protocol/openid-connect/token", self.server_url, self.realm)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration_days: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .build()?;
        config.try_deserialize()
    }
}
EOF



# 3. Create infrastructure/persistence/database.rs
cat > src/infrastructure/persistence/database.rs << 'EOF'
use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::infrastructure::config::DatabaseConfig;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await?;
    
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Run SQL migrations from migrations directory
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    Ok(())
}
EOF

# 4. Create SQL migrations
mkdir -p migrations

# Initial migration
cat > migrations/001_initial_schema.sql << 'EOF'
-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    company_id UUID,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create companies table
CREATE TABLE IF NOT EXISTS companies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create audit_logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id VARCHAR(255),
    details JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add foreign key constraint for users.company_id
ALTER TABLE users 
ADD CONSTRAINT users_company_id_fkey 
FOREIGN KEY (company_id) REFERENCES companies(id);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_company_id ON users(company_id);
CREATE INDEX IF NOT EXISTS idx_companies_created_by ON companies(created_by);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
EOF

# 5. Create infrastructure/persistence/repositories/user_repository.rs
cat > src/infrastructure/persistence/repositories/user_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            "#
        )
        .bind(user.id)
        .bind(&user.keycloak_id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(user.role.to_string())
        .bind(&user.company_id)
        .bind(user.email_verified)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(User {
            id: row.get("id"),
            keycloak_id: row.get("keycloak_id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: row.get("company_id"),
            email_verified: row.get("email_verified"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: r.get("company_id"),
            email_verified: r.get("email_verified"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users WHERE keycloak_id = $1
            "#
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: r.get("company_id"),
            email_verified: r.get("email_verified"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: r.get("company_id"),
            email_verified: r.get("email_verified"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users WHERE username = $1
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: r.get("company_id"),
            email_verified: r.get("email_verified"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        let row = sqlx::query(
            r#"
            UPDATE users 
            SET username = $2, email = $3, role = $4, company_id = $5, email_verified = $6, updated_at = $7
            WHERE id = $1
            RETURNING id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            "#
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(user.role.to_string())
        .bind(&user.company_id)
        .bind(user.email_verified)
        .bind(chrono::Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(User {
            id: row.get("id"),
            keycloak_id: row.get("keycloak_id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: row.get("company_id"),
            email_verified: row.get("email_verified"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM users WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(())
    }

    async fn list_by_company(&self, company_id: Uuid) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users WHERE company_id = $1
            "#
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: r.get("company_id"),
            email_verified: r.get("email_verified"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| User {
            id: r.get("id"),
            keycloak_id: r.get("keycloak_id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get::<String, _>("role").parse().unwrap_or(UserRole::User),
            company_id: r.get("company_id"),
            email_verified: r.get("email_verified"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }
}
EOF

# 6. Create infrastructure/persistence/repositories/company_repository.rs
cat > src/infrastructure/persistence/repositories/company_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::errors::DomainError;
use crate::domain::repositories::CompanyRepository;

#[derive(Clone)]
pub struct PostgresCompanyRepository {
    pool: PgPool,
}

impl PostgresCompanyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompanyRepository for PostgresCompanyRepository {
    async fn create(&self, company: &Company) -> Result<Company, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO companies (id, name, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, description, created_by, created_at, updated_at
            "#
        )
        .bind(company.id)
        .bind(&company.name)
        .bind(&company.description)
        .bind(company.created_by)
        .bind(company.created_at)
        .bind(company.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(Company {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Company>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(row.map(|r| Company {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies WHERE name = $1
            "#
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(row.map(|r| Company {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn update(&self, company: &Company) -> Result<Company, DomainError> {
        let row = sqlx::query(
            r#"
            UPDATE companies 
            SET name = $2, description = $3, updated_at = $4
            WHERE id = $1
            RETURNING id, name, description, created_by, created_at, updated_at
            "#
        )
        .bind(company.id)
        .bind(&company.name)
        .bind(&company.description)
        .bind(chrono::Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(Company {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM companies WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<Company>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| Company {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }

    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Company>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT c.id, c.name, c.description, c.created_by, c.created_at, c.updated_at
            FROM companies c
            LEFT JOIN users u ON u.company_id = c.id
            WHERE u.id = $1 OR c.created_by = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| Company {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }
}
EOF

# 7. Create infrastructure/persistence/repositories/audit_log_repository.rs
cat > src/infrastructure/persistence/repositories/audit_log_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AuditLogRepository;

#[derive(Clone)]
pub struct PostgresAuditLogRepository {
    pool: PgPool,
}

impl PostgresAuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository for PostgresAuditLogRepository {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(audit_log.id)
        .bind(&audit_log.user_id)
        .bind(audit_log.action.to_string())
        .bind(&audit_log.resource_type)
        .bind(&audit_log.resource_id)
        .bind(&audit_log.details)
        .bind(&audit_log.ip_address)
        .bind(&audit_log.user_agent)
        .bind(audit_log.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<AuditLog>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at
            FROM audit_logs WHERE user_id = $1 ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| AuditLog {
            id: r.get("id"),
            user_id: r.get("user_id"),
            action: r.get::<String, _>("action").parse().unwrap_or(AuditAction::Login),
            resource_type: r.get("resource_type"),
            resource_id: r.get("resource_id"),
            details: r.get("details"),
            ip_address: r.get("ip_address"),
            user_agent: r.get("user_agent"),
            created_at: r.get("created_at"),
        }).collect())
    }

    async fn find_by_company(&self, company_id: Uuid) -> Result<Vec<AuditLog>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT al.id, al.user_id, al.action, al.resource_type, al.resource_id, al.details, al.ip_address, al.user_agent, al.created_at
            FROM audit_logs al
            LEFT JOIN users u ON al.user_id = u.id
            WHERE u.company_id = $1
            ORDER BY al.created_at DESC
            "#
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| AuditLog {
            id: r.get("id"),
            user_id: r.get("user_id"),
            action: r.get::<String, _>("action").parse().unwrap_or(AuditAction::Login),
            resource_type: r.get("resource_type"),
            resource_id: r.get("resource_id"),
            details: r.get("details"),
            ip_address: r.get("ip_address"),
            user_agent: r.get("user_agent"),
            created_at: r.get("created_at"),
        }).collect())
    }

    async fn list_recent(&self, limit: u32) -> Result<Vec<AuditLog>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at
            FROM audit_logs ORDER BY created_at DESC LIMIT $1
            "#
        )
        .bind(limit as i32)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| AuditLog {
            id: r.get("id"),
            user_id: r.get("user_id"),
            action: r.get::<String, _>("action").parse().unwrap_or(AuditAction::Login),
            resource_type: r.get("resource_type"),
            resource_id: r.get("resource_id"),
            details: r.get("details"),
            ip_address: r.get("ip_address"),
            user_agent: r.get("user_agent"),
            created_at: r.get("created_at"),
        }).collect())
    }
}
EOF

# 8. Create infrastructure/persistence/repositories/mod.rs
cat > src/infrastructure/persistence/repositories/mod.rs << 'EOF'
pub mod user_repository;
pub mod company_repository;
pub mod audit_log_repository;

pub use user_repository::PostgresUserRepository;
pub use company_repository::PostgresCompanyRepository;
pub use audit_log_repository::PostgresAuditLogRepository;
EOF

# 9. Create infrastructure/persistence/mod.rs
cat > src/infrastructure/persistence/mod.rs << 'EOF'
pub mod database;
pub mod repositories;

pub use database::{create_pool, run_migrations};
pub use repositories::*;
EOF

# 2. Update infrastructure/auth/mod.rs to include KeycloakClient
cat > src/infrastructure/auth/mod.rs << 'EOF'
pub mod jwt;
pub mod keycloak;

pub use jwt::JwtService;
pub use keycloak::KeycloakClient;
EOF

# 12. Create infrastructure/external/mod.rs
cat > src/infrastructure/external/mod.rs << 'EOF'
use reqwest::Client;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
EOF

# 13. Create infrastructure/mod.rs
cat > src/infrastructure/mod.rs << 'EOF'
pub mod config;
pub mod persistence;
pub mod auth;
pub mod external;
pub mod errors;

pub use config::Config;
pub use persistence::*;
pub use auth::*;
pub use external::HttpClient;
pub use errors::InfrastructureError;
EOF

# 14. Update Cargo.toml to include sqlx migration feature
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice with Keycloak"
authors = ["M.MEZNI"]
license = "MIT"

[dependencies]
actix-web = "4.12.0"
serde = { version = "1.0.228", features = ["derive"] }
thiserror = "2.0.17"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
tokio = { version = "1.48.0", features = ["full"] }
config = "0.15.19"
serde_json = "1.0.145"
chrono = { version = "0.4.42", features = ["serde"] }
validator = { version = "0.20.0", features = ["derive"] }
uuid = { version = "1.18.1", features = ["v4", "serde"] }
async-trait = "0.1.89"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
reqwest = { version = "0.12.24", features = ["json"] }
bcrypt = "0.17.1"

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF


cat > config/default.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 3000

[database]
host = "localhost"
port = 5433
username = "auth_user"
password = "password"
database_name = "auth_db"
max_connections = 10

[keycloak]
server_url = "http://localhost:5080"
realm = "ev-realm"
client_id = "auth-service"
client_secret = "your-client-secret"
admin_username = "admin"
admin_password = "admin"

[jwt]
secret = "your-jwt-secret"
issuer = "auth-service"
audience = "auth-service-users"
expiration_days = 7

[logging]
level = "info"
EOF


echo "Complete infrastructure layer created with SQL migrations!"
echo "SQL migrations are located in the migrations/ directory"
echo "Run 'cargo check' to verify everything compiles"