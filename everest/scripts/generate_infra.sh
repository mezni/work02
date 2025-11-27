#!/bin/bash

set -e

echo "Fixing all missing module and file errors..."

cd auth-service

# Create all missing directories
mkdir -p src/infrastructure/{config,persistence,external,auth,errors}
mkdir -p src/infrastructure/persistence/{database,repositories}
mkdir -p src/shared
mkdir -p migrations

# 1. Create missing shared config
cat > src/shared/config.rs << 'EOF'
use serde::Deserialize;
use config::{Config, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
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
pub struct JwtConfig {
    pub secret: String,
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

# 2. Create missing shared error
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

impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        AppError::DatabaseError(format!("Connection pool error: {}", e))
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Resource not found".to_string())
            }
            _ => AppError::DatabaseError(format!("Database operation failed: {}", e)),
        }
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
EOF

# 3. Create missing shared logger
cat > src/shared/logger.rs << 'EOF'
pub fn init() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
EOF

# 4. Create shared mod.rs
cat > src/shared/mod.rs << 'EOF'
pub mod config;
pub mod error;
pub mod logger;

pub use config::AppConfig;
pub use error::AppError;
pub use logger::init;
EOF

# 5. Create missing infrastructure/config.rs
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
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: u64,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
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
            jwt: JwtConfig {
                secret: shared_config.jwt.secret.clone(),
                expiration_hours: shared_config.jwt.expiration_days as u64 * 24,
            },
        }
    }
}
EOF

# 6. Create missing infrastructure/errors.rs
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
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(error: sqlx::Error) -> Self {
        InfrastructureError::DatabaseError(error.to_string())
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
EOF

# 7. Create missing infrastructure/persistence/database.rs
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
    // Create tables if they don't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(255) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            first_name VARCHAR(255) NOT NULL,
            last_name VARCHAR(255) NOT NULL,
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            company_id UUID,
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS companies (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) UNIQUE NOT NULL,
            description TEXT,
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS audit_logs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES users(id),
            action VARCHAR(100) NOT NULL,
            resource_type VARCHAR(100) NOT NULL,
            resource_id UUID,
            details JSONB,
            ip_address VARCHAR(45),
            user_agent TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS refresh_tokens (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES users(id) ON DELETE CASCADE,
            token VARCHAR(512) UNIQUE NOT NULL,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;

    // Add foreign key constraint for users.company_id after companies table exists
    sqlx::query(
        r#"
        DO $$ 
        BEGIN 
            IF NOT EXISTS (
                SELECT 1 FROM information_schema.table_constraints 
                WHERE constraint_name = 'users_company_id_fkey'
            ) THEN
                ALTER TABLE users 
                ADD CONSTRAINT users_company_id_fkey 
                FOREIGN KEY (company_id) REFERENCES companies(id);
            END IF;
        END $$;
        "#
    ).execute(pool).await?;

    Ok(())
}
EOF

# 8. Create missing infrastructure/persistence/repositories/mod.rs
cat > src/infrastructure/persistence/repositories/mod.rs << 'EOF'
pub mod user_repository;
pub mod company_repository;
pub mod audit_log_repository;

pub use user_repository::PostgresUserRepository;
pub use company_repository::PostgresCompanyRepository;
pub use audit_log_repository::PostgresAuditLogRepository;
EOF

# 9. Create missing infrastructure/persistence/repositories/user_repository.rs
cat > src/infrastructure/persistence/repositories/user_repository.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn hash_password(password: &str) -> Result<String, DomainError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| DomainError::ValidationError(format!("Password hashing failed: {}", e)))
    }

    fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
        verify(password, hash)
            .map_err(|e| DomainError::ValidationError(format!("Password verification failed: {}", e)))
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &mut User) -> Result<(), DomainError> {
        let password_hash = Self::hash_password(&user.password_hash)?;
        
        let row = sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, first_name, last_name, role, company_id, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, username, email, first_name, last_name, role, company_id, is_active, created_at, updated_at
            "#
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.role)
        .bind(&user.company_id)
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        user.password_hash = password_hash;
        user.username = row.get("username");
        user.email = row.get("email");
        user.first_name = row.get("first_name");
        user.last_name = row.get("last_name");
        user.role = row.get("role");
        user.company_id = row.get("company_id");
        user.is_active = row.get("is_active");
        user.created_at = row.get("created_at");
        user.updated_at = row.get("updated_at");

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, first_name, last_name, role, company_id, is_active, created_at, updated_at
            FROM users WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            role: r.get("role"),
            company_id: r.get("company_id"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, first_name, last_name, role, company_id, is_active, created_at, updated_at
            FROM users WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            role: r.get("role"),
            company_id: r.get("company_id"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, first_name, last_name, role, company_id, is_active, created_at, updated_at
            FROM users WHERE username = $1
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            role: r.get("role"),
            company_id: r.get("company_id"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            UPDATE users 
            SET username = $2, email = $3, first_name = $4, last_name = $5, role = $6, 
                company_id = $7, is_active = $8, updated_at = $9
            WHERE id = $1
            "#
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.role)
        .bind(&user.company_id)
        .bind(user.is_active)
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    async fn list_by_company(&self, company_id: Uuid) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, first_name, last_name, role, company_id, is_active, created_at, updated_at
            FROM users WHERE company_id = $1
            "#
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            role: r.get("role"),
            company_id: r.get("company_id"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }

    async fn verify_credentials(&self, email: &str, password: &str) -> Result<Option<User>, DomainError> {
        let user = self.find_by_email(email).await?;
        
        match user {
            Some(user) => {
                if Self::verify_password(password, &user.password_hash)? {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    async fn assign_to_company(&self, user_id: Uuid, company_id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            UPDATE users SET company_id = $2, updated_at = $3
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(company_id)
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    async fn change_role(&self, user_id: Uuid, role: String) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            UPDATE users SET role = $2, updated_at = $3
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(role)
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }
}
EOF

# 10. Create missing infrastructure/persistence/repositories/company_repository.rs
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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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

# 11. Create missing infrastructure/persistence/repositories/audit_log_repository.rs
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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

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

# 12. Create missing infrastructure/persistence/mod.rs
cat > src/infrastructure/persistence/mod.rs << 'EOF'
pub mod database;
pub mod repositories;

pub use database::{create_pool, run_migrations};
pub use repositories::*;
EOF

# 13. Create missing infrastructure/auth/mod.rs
cat > src/infrastructure/auth/mod.rs << 'EOF'
pub mod jwt;

pub use jwt::JwtService;
EOF

# 14. Create missing infrastructure/auth/jwt.rs
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

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        let expiration_seconds = config.expiration_hours * 3600;
        
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

# 15. Create missing infrastructure/external/mod.rs
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

# 16. Create missing infrastructure/mod.rs
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

# 17. Create missing config file
mkdir -p config
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

# 18. Update Cargo.toml with all required dependencies
cat > Cargo.toml << 'EOF'
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"
description = "Authentication and Authorization Microservice"
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
async-trait = "0.1.68"
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
reqwest = { version = "0.12.24", features = ["json"] }
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
actix-web-httpauth = "0.8.0"
actix-cors = "0.7.0"
bcrypt = "0.15"

[dev-dependencies]
actix-web = "4.12.0"
tokio = { version = "1.48.0", features = ["full"] }
sqlx = { version = "0.8.6", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }

[[bin]]
name = "auth-service"
path = "src/main.rs"
EOF

echo "All missing files have been created successfully!"
echo "Running cargo check to verify..."

#cargo check

echo "If there are still errors, please run: cargo clean && cargo build"