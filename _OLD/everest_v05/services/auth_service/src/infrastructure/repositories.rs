use sqlx::postgres::PgPool;
use sqlx::Row;
use crate::domain::models::{User, NewUser};
use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::application::repositories::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, DomainError> {
        let user = sqlx::query!(
            r#"
            SELECT id, email, username, role, company_id, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InvalidCredentials)? // Map to appropriate error
        .map(|row| User {
            id: row.id,
            email: row.email,
            username: row.username,
            role: row.role.parse().unwrap_or(UserRole::User),
            company_id: row.company_id,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query!(
            r#"
            SELECT id, email, username, role, company_id, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InvalidCredentials)?
        .map(|row| User {
            id: row.id,
            email: row.email,
            username: row.username,
            role: row.role.parse().unwrap_or(UserRole::User),
            company_id: row.company_id,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query!(
            r#"
            SELECT id, email, username, role, company_id, is_active, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InvalidCredentials)?
        .map(|row| User {
            id: row.id,
            email: row.email,
            username: row.username,
            role: row.role.parse().unwrap_or(UserRole::User),
            company_id: row.company_id,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });

        Ok(user)
    }

    async fn create(&self, new_user: NewUser) -> Result<User, DomainError> {
        let user = sqlx::query!(
            r#"
            INSERT INTO users (email, username, role, company_id, is_active)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, username, role, company_id, is_active, created_at, updated_at
            "#,
            new_user.email,
            new_user.username,
            new_user.role.to_string(),
            new_user.company_id,
            true
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::UserAlreadyExists)?;

        Ok(User {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role.parse().unwrap_or(UserRole::User),
            company_id: user.company_id,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        todo!("Implement update")
    }

    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError> {
        todo!("Implement delete")
    }
}