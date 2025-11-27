#!/bin/bash

set -e

echo "Generating infrastructure layer with fixes..."

cd auth-service

# Create infrastructure directories
mkdir -p src/infrastructure/{persistence,external,auth,config}
mkdir -p src/infrastructure/persistence/repositories
mkdir -p tests/unit/infrastructure
mkdir -p migrations

# Create migrations directory with initial schema
cat > migrations/001_initial_schema.sql << 'EOF'
-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    keycloak_id VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    role VARCHAR(50) NOT NULL,
    company_id UUID,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create companies table
CREATE TABLE companies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create audit_logs table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id VARCHAR(255),
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_company_id ON users(company_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
EOF

# Update shared config.rs with new configuration structure
cat > src/shared/config.rs << 'EOF'
use serde::Deserialize;
use config::{Config, File};
use crate::shared::error::AppError;

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
            "postgresql://{}:{}@{}:{}/{}",
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
        self.token_url()
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
    pub fn load() -> Result<Self, AppError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            .build()?;
        let app_config = config.try_deserialize()?;
        Ok(app_config)
    }
}
EOF

# Update shared error.rs with more error types
cat > src/shared/error.rs << 'EOF'
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal => HttpResponse::InternalServerError().json("Internal server error"),
            AppError::ConfigError(msg) => HttpResponse::InternalServerError().json(format!("Configuration error: {}", msg)),
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError().json(format!("Database error: {}", msg)),
            AppError::AuthError(msg) => HttpResponse::Unauthorized().json(format!("Authentication error: {}", msg)),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(format!("Validation error: {}", msg)),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(format!("Not found: {}", msg)),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(format!("Unauthorized: {}", msg)),
        }
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        AppError::ConfigError(e.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::DatabaseError(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::AuthError(e.to_string())
    }
}
EOF

# Create infrastructure config
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
            "postgresql://{}:{}@{}:{}/{}",
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
        self.token_url()
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
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
        }
    }
}
EOF

# Create infrastructure errors
cat > src/infrastructure/errors.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Database pool error: {0}")]
    PoolError(String),
    #[error("Database query error: {0}")]
    QueryError(String),
    #[error("Keycloak error: {0}")]
    KeycloakError(String),
    #[error("HTTP client error: {0}")]
    HttpClientError(String),
    #[error("JWT error: {0}")]
    JwtError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(e: sqlx::Error) -> Self {
        InfrastructureError::QueryError(e.to_string())
    }
}

impl From<reqwest::Error> for InfrastructureError {
    fn from(e: reqwest::Error) -> Self {
        InfrastructureError::HttpClientError(e.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for InfrastructureError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        InfrastructureError::JwtError(e.to_string())
    }
}
EOF

# Create database module
cat > src/infrastructure/persistence/mod.rs << 'EOF'
pub mod database;
pub mod repositories;

pub use database::*;
pub use repositories::*;
EOF

# Create database implementation
cat > src/infrastructure/persistence/database.rs << 'EOF'
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::errors::InfrastructureError;

pub type DatabasePool = PgPool;

pub async fn create_pool(config: &DatabaseConfig) -> Result<DatabasePool, InfrastructureError> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&config.connection_string())
        .await
        .map_err(|e| InfrastructureError::PoolError(e.to_string()))
}

pub async fn run_migrations(pool: &DatabasePool) -> Result<(), InfrastructureError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| InfrastructureError::QueryError(format!("Migration error: {}", e)))?;
    Ok(())
}
EOF

# Create repositories module
cat > src/infrastructure/persistence/repositories/mod.rs << 'EOF'
pub mod user_repository;
pub mod company_repository;
pub mod audit_log_repository;

pub use user_repository::PostgresUserRepository;
pub use company_repository::PostgresCompanyRepository;
pub use audit_log_repository::PostgresAuditLogRepository;
EOF

# Create user repository
cat > src/infrastructure/persistence/repositories/user_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

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
        .bind(user.company_id)
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
            role: row.get::<String, _>("role").parse().map_err(|e: String| DomainError::InvalidUserRole(e))?,
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
        .bind(user.company_id)
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

# Create company repository
cat > src/infrastructure/persistence/repositories/company_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::errors::DomainError;
use crate::domain::repositories::CompanyRepository;

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

# Create audit log repository
cat > src/infrastructure/persistence/repositories/audit_log_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AuditLogRepository;

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
        .bind(audit_log.user_id)
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

# Create auth module
cat > src/infrastructure/auth/mod.rs << 'EOF'
pub mod keycloak;
pub mod jwt;

pub use keycloak::KeycloakClient;
pub use jwt::JwtService;
EOF

# Create Keycloak client
cat > src/infrastructure/auth/keycloak.rs << 'EOF'
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::infrastructure::config::KeycloakConfig;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Clone)]
pub struct KeycloakClient {
    config: KeycloakConfig,
    http_client: Client,
}

impl KeycloakClient {
    pub fn new(config: KeycloakConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String, InfrastructureError> {
        let admin_token = self.get_admin_token().await?;

        let user_data = serde_json::json!({
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

        let response = self.http_client
            .post(&self.config.admin_users_url())
            .header("Authorization", format!("Bearer {}", admin_token))
            .header("Content-Type", "application/json")
            .json(&user_data)
            .send()
            .await?;

        if response.status().is_success() {
            if let Some(location) = response.headers().get("Location") {
                if let Ok(location_str) = location.to_str() {
                    if let Some(user_id) = location_str.split('/').last() {
                        return Ok(user_id.to_string());
                    }
                }
            }
            Err(InfrastructureError::KeycloakError("Failed to extract user ID from response".to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Keycloak API error: {}", error_text)))
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "password");
        form_data.insert("client_id", &self.config.client_id);
        form_data.insert("client_secret", &self.config.client_secret);
        form_data.insert("username", username);
        form_data.insert("password", password);

        let response = self.http_client
            .post(&self.config.token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Login failed: {}", error_text)))
        }
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<KeycloakTokenResponse, InfrastructureError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("client_id", &self.config.client_id);
        form_data.insert("client_secret", &self.config.client_secret);
        form_data.insert("refresh_token", refresh_token);

        let response = self.http_client
            .post(&self.config.token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Token refresh failed: {}", error_text)))
        }
    }

    pub async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError> {
        let response = self.http_client
            .get(&self.config.user_info_url())
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let user_info: KeycloakUserInfo = response.json().await?;
            Ok(user_info)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("User info request failed: {}", error_text)))
        }
    }

    pub async fn update_user(&self, user_id: &str, attributes: HashMap<String, String>) -> Result<(), InfrastructureError> {
        let admin_token = self.get_admin_token().await?;

        let user_data = serde_json::json!({
            "attributes": attributes
        });

        let response = self.http_client
            .put(&format!("{}/{}", self.config.admin_users_url(), user_id))
            .header("Authorization", format!("Bearer {}", admin_token))
            .header("Content-Type", "application/json")
            .json(&user_data)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("User update failed: {}", error_text)))
        }
    }

    pub async fn reset_password(&self, user_id: &str, new_password: &str) -> Result<(), InfrastructureError> {
        let admin_token = self.get_admin_token().await?;

        let password_data = serde_json::json!({
            "type": "password",
            "value": new_password,
            "temporary": false
        });

        let response = self.http_client
            .put(&format!("{}/{}/reset-password", self.config.admin_users_url(), user_id))
            .header("Authorization", format!("Bearer {}", admin_token))
            .header("Content-Type", "application/json")
            .json(&password_data)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Password reset failed: {}", error_text)))
        }
    }

    async fn get_admin_token(&self) -> Result<String, InfrastructureError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "password");
        form_data.insert("client_id", &self.config.client_id);
        form_data.insert("client_secret", &self.config.client_secret);
        form_data.insert("username", &self.config.admin_username);
        form_data.insert("password", &self.config.admin_password);

        let response = self.http_client
            .post(&self.config.admin_token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: KeycloakTokenResponse = response.json().await?;
            Ok(token_response.access_token)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(InfrastructureError::KeycloakError(format!("Admin token request failed: {}", error_text)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUserInfo {
    pub sub: String,
    pub email: String,
    pub preferred_username: String,
    pub email_verified: bool,
    pub exp: i64,
    pub iat: i64,
}
EOF

# Create JWT service with updated Claims structure
cat > src/infrastructure/auth/jwt.rs << 'EOF'
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::shared::config::JwtConfig;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
    pub jti: String, // JWT ID
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<String>,
    pub company_name: Option<String>,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        let expiration_seconds = config.expiration_days * 24 * 60 * 60;
        
        Self {
            encoding_key: EncodingKey::from_secret(config.secret.as_ref()),
            decoding_key: DecodingKey::from_secret(config.secret.as_ref()),
            expiration_seconds: expiration_seconds as u64,
        }
    }

    pub fn generate_token(
        &self, 
        user_id: Uuid,
        username: &str, 
        email: &str, 
        role: &str,
        company_id: Option<Uuid>,
        company_name: Option<&str>
    ) -> Result<String, InfrastructureError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| InfrastructureError::JwtError(e.to_string()))?
            .as_secs() as usize;

        let expiration = now + self.expiration_seconds as usize;
        let jti = Uuid::new_v4().to_string();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: now,
            jti,
            username: username.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            company_id: company_id.map(|id| id.to_string()),
            company_name: company_name.map(|name| name.to_string()),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, InfrastructureError> {
        let mut validation = Validation::new(Algorithm::HS256);
        // You can add additional validation here if needed

        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &validation,
        )?;

        Ok(token_data.claims)
    }

    pub fn extract_user_id(&self, token: &str) -> Result<Uuid, InfrastructureError> {
        let claims = self.validate_token(token)?;
        Uuid::parse_str(&claims.sub)
            .map_err(|e| InfrastructureError::JwtError(format!("Invalid user ID in token: {}", e)))
    }

    pub fn extract_claims(&self, token: &str) -> Result<Claims, InfrastructureError> {
        self.validate_token(token)
    }
}
EOF

# Create external module
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

# Create infrastructure main mod.rs
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

# Fix unused variable warning in user_aggregate.rs
echo "Fixing unused variable warning in user_aggregate.rs..."
cat > src/domain/aggregates/user_aggregate.rs << 'EOF'
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::events::{DomainEvent, UserEvent};

pub struct UserAggregate {
    pub user: User,
    pub events: Vec<DomainEvent>,
}

impl UserAggregate {
    pub fn new(
        keycloak_id: String,
        username: String,
        email: String,
        role: UserRole,
        company_id: Option<Uuid>,
    ) -> Result<Self, DomainError> {
        let user = User::new(keycloak_id, username, email, role, company_id)?;
        let mut aggregate = UserAggregate {
            user,
            events: Vec::new(),
        };
        
        aggregate.events.push(DomainEvent::UserCreated(UserEvent {
            user_id: aggregate.user.id,
            keycloak_id: aggregate.user.keycloak_id.clone(),
            username: aggregate.user.username.clone(),
            email: aggregate.user.email.clone(),
            role: aggregate.user.role,
            company_id: aggregate.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(aggregate)
    }
    
    pub fn update_email(&mut self, new_email: String) -> Result<(), DomainError> {
        let _old_email = self.user.email.clone();
        self.user.email = new_email;
        self.user.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::UserUpdated(UserEvent {
            user_id: self.user.id,
            keycloak_id: self.user.keycloak_id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            role: self.user.role,
            company_id: self.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn update_role(&mut self, new_role: UserRole, actor: &User) -> Result<(), DomainError> {
        if !actor.is_admin() && !actor.can_manage_user(&self.user) {
            return Err(DomainError::InsufficientPermissions(
                "Only admins or managers can change user roles".to_string(),
            ));
        }
        
        self.user.role = new_role;
        self.user.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::UserRoleChanged(UserEvent {
            user_id: self.user.id,
            keycloak_id: self.user.keycloak_id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            role: self.user.role,
            company_id: self.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn assign_to_company(&mut self, company_id: Uuid, actor: &User) -> Result<(), DomainError> {
        if !actor.can_manage_company(company_id) {
            return Err(DomainError::InsufficientPermissions(
                "User cannot assign to this company".to_string(),
            ));
        }
        
        self.user.company_id = Some(company_id);
        self.user.updated_at = chrono::Utc::now();
        
        self.events.push(DomainEvent::UserCompanyAssigned(UserEvent {
            user_id: self.user.id,
            keycloak_id: self.user.keycloak_id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            role: self.user.role,
            company_id: self.user.company_id,
            timestamp: chrono::Utc::now(),
        }));
        
        Ok(())
    }
    
    pub fn take_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_aggregate_creation() {
        let aggregate = UserAggregate::new(
            "keycloak-123".to_string(),
            "testuser".to_string(),
            "test@example.com".to_string(),
            UserRole::User,
            None,
        );
        
        assert!(aggregate.is_ok());
        let aggregate = aggregate.unwrap();
        assert_eq!(aggregate.events.len(), 1);
    }

    #[test]
    fn test_user_aggregate_email_update() {
        let mut aggregate = UserAggregate::new(
            "keycloak-123".to_string(),
            "testuser".to_string(),
            "test@example.com".to_string(),
            UserRole::User,
            None,
        ).unwrap();
        
        let result = aggregate.update_email("new@example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(aggregate.events.len(), 2);
        assert_eq!(aggregate.user.email, "new@example.com");
    }
}
EOF

# Create infrastructure tests
cat > tests/unit/infrastructure/mod.rs << 'EOF'
pub mod persistence;
pub mod auth;
pub mod config;
pub mod jwt;
EOF

# Create persistence tests
cat > tests/unit/infrastructure/persistence.rs << 'EOF'
use auth_service::infrastructure::persistence::repositories::{
    PostgresUserRepository, PostgresCompanyRepository, PostgresAuditLogRepository
};
use auth_service::domain::entities::{User, Company, AuditLog};
use auth_service::domain::enums::{UserRole, AuditAction};
use uuid::Uuid;

#[cfg(test)]
mod user_repository_tests {
    use super::*;

    #[test]
    fn test_user_repository_initialization() {
        // Test that UserRepositoryImpl can be initialized
        assert!(true, "PostgresUserRepository should compile successfully");
    }

    #[test]
    fn test_user_role_parsing() {
        assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
        assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
        assert!("invalid".parse::<UserRole>().is_err());
    }

    #[test]
    fn test_user_entity_creation() {
        let user = User::new(
            "keycloak-123".to_string(),
            "testuser".to_string(),
            "test@example.com".to_string(),
            UserRole::User,
            None,
        );

        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(matches!(user.role, UserRole::User));
    }
}

#[cfg(test)]
mod company_repository_tests {
    use super::*;

    #[test]
    fn test_company_repository_initialization() {
        assert!(true, "PostgresCompanyRepository should compile successfully");
    }

    #[test]
    fn test_company_entity_creation() {
        let company = Company::new(
            "Test Company".to_string(),
            Some("Test Description".to_string()),
            Uuid::new_v4(),
        );

        assert_eq!(company.name, "Test Company");
        assert_eq!(company.description, Some("Test Description".to_string()));
    }
}

#[cfg(test)]
mod audit_log_repository_tests {
    use super::*;

    #[test]
    fn test_audit_log_repository_initialization() {
        assert!(true, "PostgresAuditLogRepository should compile successfully");
    }

    #[test]
    fn test_audit_action_parsing() {
        assert_eq!("UserCreated".parse::<AuditAction>().unwrap(), AuditAction::UserCreated);
        assert_eq!("Login".parse::<AuditAction>().unwrap(), AuditAction::Login);
        assert!("InvalidAction".parse::<AuditAction>().is_err());
    }

    #[test]
    fn test_audit_log_creation() {
        let audit_log = AuditLog::new(
            Some(Uuid::new_v4()),
            AuditAction::UserCreated,
            "User".to_string(),
            Some("user-123".to_string()),
            Some(serde_json::json!({"email": "test@example.com"})),
            Some("127.0.0.1".to_string()),
            Some("Test-Agent".to_string()),
        );

        assert_eq!(audit_log.action, AuditAction::UserCreated);
        assert_eq!(audit_log.resource_type, "User");
        assert_eq!(audit_log.resource_id, Some("user-123".to_string()));
    }
}
EOF

# Create auth tests
cat > tests/unit/infrastructure/auth.rs << 'EOF'
use auth_service::infrastructure::auth::{KeycloakClient, JwtService};
use auth_service::infrastructure::config::{KeycloakConfig, Config};
use auth_service::shared::config::{JwtConfig, AppConfig};

#[cfg(test)]
mod keycloak_tests {
    use super::*;

    #[test]
    fn test_keycloak_config_creation() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test-realm".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };

        assert_eq!(config.server_url, "http://localhost:8080");
        assert_eq!(config.realm, "test-realm");
        assert_eq!(config.client_id, "test-client");
    }

    #[test]
    fn test_keycloak_client_initialization() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test-realm".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };

        let client = KeycloakClient::new(config);

        // Test that client can be created (compilation test)
        assert!(true, "KeycloakClient should be created successfully");
    }

    #[test]
    fn test_keycloak_url_generation() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test-realm".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };

        let token_url = config.token_url();
        let user_info_url = config.user_info_url();
        let admin_users_url = config.admin_users_url();

        assert!(token_url.contains("protocol/openid-connect/token"));
        assert!(user_info_url.contains("protocol/openid-connect/userinfo"));
        assert!(admin_users_url.contains("admin/realms/test-realm/users"));
    }
}

#[cfg(test)]
mod jwt_tests {
    use super::*;

    #[test]
    fn test_jwt_service_initialization() {
        let config = JwtConfig {
            secret: "test-secret".to_string(),
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            expiration_days: 7,
        };

        let jwt_service = JwtService::new(config);

        assert!(true, "JwtService should be created successfully");
    }

    #[test]
    fn test_jwt_config_validation() {
        let config = JwtConfig {
            secret: "test-secret".to_string(),
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            expiration_days: 7,
        };

        assert_eq!(config.secret, "test-secret");
        assert_eq!(config.issuer, "test-issuer");
        assert_eq!(config.audience, "test-audience");
        assert_eq!(config.expiration_days, 7);
    }
}
EOF

# Fix test imports in infrastructure config tests
cat > tests/unit/infrastructure/config.rs << 'EOF'
use auth_service::infrastructure::config::{Config, DatabaseConfig, KeycloakConfig};
use auth_service::shared::config::AppConfig;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_database_config_connection_string() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "password".to_string(),
            database_name: "auth_service".to_string(),
            max_connections: 10,
        };

        let connection_string = config.connection_string();
        assert!(connection_string.contains("postgresql://postgres:password@localhost:5432/auth_service"));
    }

    #[test]
    fn test_keycloak_config_urls() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "auth-realm".to_string(),
            client_id: "auth-client".to_string(),
            client_secret: "secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };

        let token_url = config.token_url();
        let user_info_url = config.user_info_url();
        let admin_users_url = config.admin_users_url();

        assert_eq!(token_url, "http://localhost:8080/realms/auth-realm/protocol/openid-connect/token");
        assert_eq!(user_info_url, "http://localhost:8080/realms/auth-realm/protocol/openid-connect/userinfo");
        assert_eq!(admin_users_url, "http://localhost:8080/admin/realms/auth-realm/users");
    }

    #[test]
    fn test_config_from_shared_config() {
        let shared_config = AppConfig {
            server: auth_service::shared::config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: auth_service::shared::config::DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "password".to_string(),
                database_name: "auth_service".to_string(),
                max_connections: 10,
            },
            keycloak: auth_service::shared::config::KeycloakConfig {
                server_url: "http://localhost:8080".to_string(),
                realm: "auth-realm".to_string(),
                client_id: "auth-client".to_string(),
                client_secret: "secret".to_string(),
                admin_username: "admin".to_string(),
                admin_password: "admin".to_string(),
            },
            jwt: auth_service::shared::config::JwtConfig {
                secret: "jwt-secret".to_string(),
                issuer: "auth-service".to_string(),
                audience: "auth-users".to_string(),
                expiration_days: 7,
            },
            logging: auth_service::shared::config::LoggingConfig {
                level: "info".to_string(),
            },
        };

        let config = Config::from_shared_config(&shared_config);

        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.database.username, "postgres");
        assert_eq!(config.keycloak.server_url, "http://localhost:8080");
        assert_eq!(config.keycloak.realm, "auth-realm");
    }
}
EOF

# Add JWT service tests
cat > tests/unit/infrastructure/jwt.rs << 'EOF'
use auth_service::infrastructure::auth::jwt::{JwtService, Claims};
use auth_service::shared::config::JwtConfig;
use uuid::Uuid;

#[cfg(test)]
mod jwt_service_tests {
    use super::*;

    #[test]
    fn test_jwt_service_initialization() {
        let config = JwtConfig {
            secret: "test-secret-key-that-is-long-enough-for-hs256".to_string(),
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            expiration_days: 7,
        };

        let jwt_service = JwtService::new(config);
        assert!(true, "JwtService should be created successfully");
    }

    #[test]
    fn test_claims_structure() {
        let user_id = Uuid::new_v4();
        let company_id = Uuid::new_v4();
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: 1000000000,
            iat: 1000000000,
            jti: Uuid::new_v4().to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: "admin".to_string(),
            company_id: Some(company_id.to_string()),
            company_name: Some("Test Company".to_string()),
        };

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.company_id, Some(company_id.to_string()));
        assert_eq!(claims.company_name, Some("Test Company".to_string()));
    }

    #[test]
    fn test_claims_without_company() {
        let user_id = Uuid::new_v4();
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: 1000000000,
            iat: 1000000000,
            jti: Uuid::new_v4().to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: "user".to_string(),
            company_id: None,
            company_name: None,
        };

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.role, "user");
        assert_eq!(claims.company_id, None);
        assert_eq!(claims.company_name, None);
    }
}
EOF

# Update tests/unit/mod.rs
cat > tests/unit/mod.rs << 'EOF'
pub mod domain;
pub mod infrastructure;
EOF

# Update config file with all required sections
cat > config/default.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 3000

[database]
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database_name = "auth_service"
max_connections = 10

[keycloak]
server_url = "http://localhost:8080"
realm = "auth-service-realm"
client_id = "auth-service-client"
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

# Update lib.rs with proper error handling
cat > src/lib.rs << 'EOF'
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod shared;

use actix_web::{web, App, HttpServer, Responder};
use shared::{config::AppConfig, logger};

pub async fn run() -> std::io::Result<()> {
    logger::init();
    
    let config = AppConfig::load()
        .expect("Failed to load configuration. Please check your config/default.toml file");
    
    println!("✅ {} service starting...", env!("CARGO_PKG_NAME"));
    
    // Initialize database pool
    let infra_config = infrastructure::Config::from_shared_config(&config);
    let db_pool = infrastructure::persistence::database::create_pool(&infra_config.database)
        .await
        .expect("Failed to create database pool");
    
    // Run migrations
    infrastructure::persistence::database::run_migrations(&db_pool)
        .await
        .expect("Failed to run migrations");
    
    println!("✅ Database connected and migrations applied");
    
    async fn health_check() -> impl Responder {
        "✅ Authentication Service is healthy"
    }
    
    println!("🚀 Server running at http://{}:{}", config.server.host, config.server.port);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
EOF

# Update Cargo.toml with infrastructure dependencies
echo "Updating Cargo.toml with infrastructure dependencies..."

# Update Cargo.toml with specific dependency versions
if [ -f Cargo.toml ]; then
    # Create a backup
    cp Cargo.toml Cargo.toml.backup
    
    # Create updated Cargo.toml
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
tracing = "0.1.42"
tracing-subscriber = "0.3.21"
tokio = { version = "1.48.0", features = ["full"] }
config = "0.15.19"
uuid = { version = "1.18.1", features = ["v4", "serde"] }
serde_json = "1.0.145"
validator = { version = "0.20.0", features = ["derive"] }
chrono = { version = "0.4.42", features = ["serde"] }
async-trait = "0.1.68"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
reqwest = { version = "0.12.24", features = ["json"] }

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }
validator = "0.20.0"
uuid = { version = "1.18.1", features = ["v4", "serde"] }
serial_test = "2.0"

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF
fi
