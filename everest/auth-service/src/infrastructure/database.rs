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
