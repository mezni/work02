use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        let query = r#"
            INSERT INTO users (id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#;

        sqlx::query(query)
            .bind(user.id)
            .bind(&user.keycloak_id)
            .bind(&user.username)
            .bind(user.email.value())
            .bind(user.role.to_string())
            .bind(user.company_id)
            .bind(user.email_verified)
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to create user: {}", e);
                DomainError::Validation(format!("Failed to create user: {}", e))
            })?;

        info!("User created successfully: {}", user.id);
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let query = r#"
            UPDATE users 
            SET username = $2, email = $3, role = $4, company_id = $5, email_verified = $6, updated_at = $7
            WHERE id = $1
        "#;

        let rows_affected = sqlx::query(query)
            .bind(user.id)
            .bind(&user.username)
            .bind(user.email.value())
            .bind(user.role.to_string())
            .bind(user.company_id)
            .bind(user.email_verified)
            .bind(user.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to update user: {}", e);
                DomainError::Validation(format!("Failed to update user: {}", e))
            })?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::UserNotFound(user.id.to_string()));
        }

        info!("User updated successfully: {}", user.id);
        Ok(())
    }

    async fn delete(&self, user_id: &Uuid) -> Result<(), DomainError> {
        let query = "DELETE FROM users WHERE id = $1";

        let rows_affected = sqlx::query(query)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete user: {}", e);
                DomainError::Validation(format!("Failed to delete user: {}", e))
            })?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::UserNotFound(user_id.to_string()));
        }

        info!("User deleted successfully: {}", user_id);
        Ok(())
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, DomainError> {
        let query = r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users 
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by ID: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => {
                let email_str: String = r.get("email");
                let email =
                    Email::new(email_str).map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

                let role_str: String = r.get("role");
                let role = UserRole::from_str(&role_str)
                    .ok_or_else(|| DomainError::InvalidRole(role_str))?;

                Ok(Some(User {
                    id: r.get("id"),
                    keycloak_id: r.get("keycloak_id"),
                    username: r.get("username"),
                    email,
                    role,
                    company_id: r.get("company_id"),
                    email_verified: r.get("email_verified"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        let query = r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users 
            WHERE keycloak_id = $1
        "#;

        let row = sqlx::query(query)
            .bind(keycloak_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by Keycloak ID: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => {
                let email_str: String = r.get("email");
                let email =
                    Email::new(email_str).map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

                let role_str: String = r.get("role");
                let role = UserRole::from_str(&role_str)
                    .ok_or_else(|| DomainError::InvalidRole(role_str))?;

                Ok(Some(User {
                    id: r.get("id"),
                    keycloak_id: r.get("keycloak_id"),
                    username: r.get("username"),
                    email,
                    role,
                    company_id: r.get("company_id"),
                    email_verified: r.get("email_verified"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let query = r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users 
            WHERE email = $1
        "#;

        let row = sqlx::query(query)
            .bind(email.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by email: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => {
                let email_str: String = r.get("email");
                let email =
                    Email::new(email_str).map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

                let role_str: String = r.get("role");
                let role = UserRole::from_str(&role_str)
                    .ok_or_else(|| DomainError::InvalidRole(role_str))?;

                Ok(Some(User {
                    id: r.get("id"),
                    keycloak_id: r.get("keycloak_id"),
                    username: r.get("username"),
                    email,
                    role,
                    company_id: r.get("company_id"),
                    email_verified: r.get("email_verified"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let query = r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users 
            WHERE username = $1
        "#;

        let row = sqlx::query(query)
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by username: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => {
                let email_str: String = r.get("email");
                let email =
                    Email::new(email_str).map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

                let role_str: String = r.get("role");
                let role = UserRole::from_str(&role_str)
                    .ok_or_else(|| DomainError::InvalidRole(role_str))?;

                Ok(Some(User {
                    id: r.get("id"),
                    keycloak_id: r.get("keycloak_id"),
                    username: r.get("username"),
                    email,
                    role,
                    company_id: r.get("company_id"),
                    email_verified: r.get("email_verified"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_company(&self, company_id: &Uuid) -> Result<Vec<User>, DomainError> {
        let query = r#"
            SELECT id, keycloak_id, username, email, role, company_id, email_verified, created_at, updated_at
            FROM users 
            WHERE company_id = $1
        "#;

        let rows = sqlx::query(query)
            .bind(company_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find users by company: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let mut users = Vec::new();
        for r in rows {
            let email_str: String = r.get("email");
            let email =
                Email::new(email_str).map_err(|e| DomainError::InvalidEmail(e.to_string()))?;

            let role_str: String = r.get("role");
            let role =
                UserRole::from_str(&role_str).ok_or_else(|| DomainError::InvalidRole(role_str))?;

            users.push(User {
                id: r.get("id"),
                keycloak_id: r.get("keycloak_id"),
                username: r.get("username"),
                email,
                role,
                company_id: r.get("company_id"),
                email_verified: r.get("email_verified"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            });
        }

        Ok(users)
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError> {
        let query = "SELECT 1 FROM users WHERE email = $1";

        let result = sqlx::query(query)
            .bind(email.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to check email existence: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        Ok(result.is_some())
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool, DomainError> {
        let query = "SELECT 1 FROM users WHERE username = $1";

        let result = sqlx::query(query)
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to check username existence: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        Ok(result.is_some())
    }
}
