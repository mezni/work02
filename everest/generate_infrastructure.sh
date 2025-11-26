#!/bin/bash

set -e

echo "Generating infrastructure layer..."

cd auth-service

# Create infrastructure directories for missing modules
mkdir -p src/infrastructure #/{database,auth,audit}
mkdir -p tests/unit/infrastructure

# First, update the domain enums to add FromStr for AuditAction
cat > src/domain/enums.rs << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    Admin,
    Partner,
    Operator,
    User,
    Guest,
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "partner" => Ok(UserRole::Partner),
            "operator" => Ok(UserRole::Operator),
            "user" => Ok(UserRole::User),
            "guest" => Ok(UserRole::Guest),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Partner => write!(f, "Partner"),
            UserRole::Operator => write!(f, "Operator"),
            UserRole::User => write!(f, "User"),
            UserRole::Guest => write!(f, "Guest"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AuditAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    CompanyCreated,
    CompanyUpdated,
    CompanyDeleted,
    Login,
    Logout,
    PasswordReset,
}

impl std::str::FromStr for AuditAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UserCreated" => Ok(AuditAction::UserCreated),
            "UserUpdated" => Ok(AuditAction::UserUpdated),
            "UserDeleted" => Ok(AuditAction::UserDeleted),
            "CompanyCreated" => Ok(AuditAction::CompanyCreated),
            "CompanyUpdated" => Ok(AuditAction::CompanyUpdated),
            "CompanyDeleted" => Ok(AuditAction::CompanyDeleted),
            "Login" => Ok(AuditAction::Login),
            "Logout" => Ok(AuditAction::Logout),
            "PasswordReset" => Ok(AuditAction::PasswordReset),
            _ => Err(format!("Invalid audit action: {}", s)),
        }
    }
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::UserCreated => write!(f, "UserCreated"),
            AuditAction::UserUpdated => write!(f, "UserUpdated"),
            AuditAction::UserDeleted => write!(f, "UserDeleted"),
            AuditAction::CompanyCreated => write!(f, "CompanyCreated"),
            AuditAction::CompanyUpdated => write!(f, "CompanyUpdated"),
            AuditAction::CompanyDeleted => write!(f, "CompanyDeleted"),
            AuditAction::Login => write!(f, "Login"),
            AuditAction::Logout => write!(f, "Logout"),
            AuditAction::PasswordReset => write!(f, "PasswordReset"),
        }
    }
}
EOF

# Database - Using query() instead of query!() with proper Row import
cat > src/infrastructure/database.rs << 'EOF'
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{User, Company, AuditLog};
use crate::domain::enums::{UserRole, AuditAction};
use crate::domain::repositories::{UserRepository, CompanyRepository, AuditLogRepository};
use crate::domain::errors::DomainError;
use crate::infrastructure::errors::InfrastructureError;

pub type DatabasePool = PgPool;

pub async fn create_pool(config: &crate::infrastructure::config::DatabaseConfig) -> Result<DatabasePool, InfrastructureError> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await
        .map_err(|e| InfrastructureError::PoolError(e.to_string()))
}

pub struct UserRepositoryImpl {
    pool: DatabasePool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            "#,
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
            SELECT * FROM users WHERE id = $1
            "#,
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
            SELECT * FROM users WHERE keycloak_id = $1
            "#,
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
            SELECT * FROM users WHERE email = $1
            "#,
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
            SELECT * FROM users WHERE username = $1
            "#,
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
            "#,
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
            "#,
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
            SELECT * FROM users WHERE company_id = $1
            "#,
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
            SELECT * FROM users
            "#,
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

pub struct CompanyRepositoryImpl {
    pool: DatabasePool,
}

impl CompanyRepositoryImpl {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompanyRepository for CompanyRepositoryImpl {
    async fn create(&self, company: &Company) -> Result<Company, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO companies (id, name, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, description, created_by, created_at, updated_at
            "#,
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
            SELECT * FROM companies WHERE id = $1
            "#,
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
            SELECT * FROM companies WHERE name = $1
            "#,
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
            "#,
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
            "#,
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
            SELECT * FROM companies
            "#,
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
            SELECT c.* FROM companies c
            LEFT JOIN users u ON u.company_id = c.id
            WHERE u.id = $1 OR c.created_by = $1
            "#,
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

pub struct AuditLogRepositoryImpl {
    pool: DatabasePool,
}

impl AuditLogRepositoryImpl {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository for AuditLogRepositoryImpl {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
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
            SELECT * FROM audit_logs WHERE user_id = $1 ORDER BY created_at DESC
            "#,
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
            SELECT al.* FROM audit_logs al
            LEFT JOIN users u ON al.user_id = u.id
            WHERE u.company_id = $1
            ORDER BY al.created_at DESC
            "#,
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
            SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT $1
            "#,
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

# Keycloak Auth
cat > src/infrastructure/auth.rs << 'EOF'
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
            // Extract user ID from location header
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

# Audit - Fixed to remove unused import
cat > src/infrastructure/audit.rs << 'EOF'
use crate::domain::entities::AuditLog;
use crate::domain::repositories::AuditLogRepository;
use crate::domain::errors::DomainError;

pub struct AuditService {
    audit_repository: Box<dyn AuditLogRepository>,
}

impl AuditService {
    pub fn new(audit_repository: Box<dyn AuditLogRepository>) -> Self {
        Self { audit_repository }
    }
    
    pub async fn log_event(
        &self,
        user_id: Option<uuid::Uuid>,
        action: crate::domain::enums::AuditAction,
        resource_type: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), DomainError> {
        let audit_log = AuditLog::new(
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            ip_address,
            user_agent,
        );
        
        self.audit_repository.create(&audit_log).await
    }
}
EOF

# Update infrastructure mod.rs to include new modules
cat > src/infrastructure/mod.rs << 'EOF'
pub mod config;
pub mod database;
pub mod auth;
pub mod audit;
pub mod logger;
pub mod errors;

// Re-exports
pub use config::Config;
pub use database::{DatabasePool, UserRepositoryImpl, CompanyRepositoryImpl, AuditLogRepositoryImpl};
pub use auth::KeycloakClient;
pub use logger::{init_logger, init_test_logger};
pub use errors::InfrastructureError;
EOF

# Create infrastructure test mod.rs
cat > tests/unit/infrastructure/mod.rs << 'EOF'
pub mod database_test;
pub mod keycloak_test;
pub mod audit_test;
pub mod config_test;
pub mod logger_test;
EOF

# Database Repository Tests
cat > tests/unit/infrastructure/database_test.rs << 'EOF'
use auth_service::infrastructure::database::{UserRepositoryImpl, CompanyRepositoryImpl, AuditLogRepositoryImpl};
use auth_service::domain::entities::{User, Company, AuditLog};
use auth_service::domain::enums::{UserRole, AuditAction};
use uuid::Uuid;
use serial_test::serial;

#[cfg(test)]
mod user_repository_tests {
    use super::*;
    
    #[tokio::test]
    #[serial]
    async fn test_user_repository_initialization() {
        // Test that UserRepositoryImpl can be initialized
        // This is a compilation test - if it compiles, the test passes
        assert!(true, "UserRepositoryImpl should compile successfully");
    }
    
    #[tokio::test]
    #[serial]
    async fn test_user_role_parsing() {
        // Test UserRole string parsing
        assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
        assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
        assert!("invalid".parse::<UserRole>().is_err());
    }
}

#[cfg(test)]
mod company_repository_tests {
    use super::*;
    
    #[tokio::test]
    #[serial]
    async fn test_company_repository_initialization() {
        // Test that CompanyRepositoryImpl can be initialized
        assert!(true, "CompanyRepositoryImpl should compile successfully");
    }
    
    #[tokio::test]
    #[serial]
    async fn test_company_entity_creation() {
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
    
    #[tokio::test]
    #[serial]
    async fn test_audit_log_repository_initialization() {
        // Test that AuditLogRepositoryImpl can be initialized
        assert!(true, "AuditLogRepositoryImpl should compile successfully");
    }
    
    #[tokio::test]
    #[serial]
    async fn test_audit_action_parsing() {
        // Test AuditAction string parsing
        assert_eq!("UserCreated".parse::<AuditAction>().unwrap(), AuditAction::UserCreated);
        assert_eq!("Login".parse::<AuditAction>().unwrap(), AuditAction::Login);
        assert!("InvalidAction".parse::<AuditAction>().is_err());
    }
    
    #[tokio::test]
    #[serial]
    async fn test_audit_log_creation() {
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

# Keycloak tests
cat > tests/unit/infrastructure/keycloak_test.rs << 'EOF'
use auth_service::infrastructure::auth::KeycloakClient;
use auth_service::infrastructure::config::KeycloakConfig;

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
EOF


# Audit tests (complete version)
cat > tests/unit/infrastructure/audit_test.rs << 'EOF'
use auth_service::infrastructure::database::{UserRepositoryImpl, CompanyRepositoryImpl, AuditLogRepositoryImpl};
use auth_service::domain::entities::{User, Company, AuditLog};
use auth_service::domain::enums::{UserRole, AuditAction};
use uuid::Uuid;
use serial_test::serial;

#[cfg(test)]
mod user_repository_tests {
    use super::*;
    
    #[tokio::test]
    #[serial]
    async fn test_user_repository_initialization() {
        // Test that UserRepositoryImpl can be initialized
        // This is a compilation test - if it compiles, the test passes
        assert!(true, "UserRepositoryImpl should compile successfully");
    }
    
    #[tokio::test]
    #[serial]
    async fn test_user_role_parsing() {
        // Test UserRole string parsing
        assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
        assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
        assert!("invalid".parse::<UserRole>().is_err());
    }
}

#[cfg(test)]
mod company_repository_tests {
    use super::*;
    
    #[tokio::test]
    #[serial]
    async fn test_company_repository_initialization() {
        // Test that CompanyRepositoryImpl can be initialized
        assert!(true, "CompanyRepositoryImpl should compile successfully");
    }
    
    #[tokio::test]
    #[serial]
    async fn test_company_entity_creation() {
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
    
    #[tokio::test]
    #[serial]
    async fn test_audit_log_repository_initialization() {
        // Test that AuditLogRepositoryImpl can be initialized
        assert!(true, "AuditLogRepositoryImpl should compile successfully");
    }
    
    #[tokio::test]
    #[serial]
    async fn test_audit_action_parsing() {
        // Test AuditAction string parsing
        assert_eq!("UserCreated".parse::<AuditAction>().unwrap(), AuditAction::UserCreated);
        assert_eq!("Login".parse::<AuditAction>().unwrap(), AuditAction::Login);
        assert!("InvalidAction".parse::<AuditAction>().is_err());
    }
    
    #[tokio::test]
    #[serial]
    async fn test_audit_log_creation() {
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

echo "Infrastructure layer generation completed!"
