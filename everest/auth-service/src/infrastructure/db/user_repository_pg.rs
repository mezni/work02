use crate::domain::{
    models::User,
    errors::DomainError,
    repositories::UserRepository,
    value_objects::{Username, Email},
};
use async_trait::async_trait;
use sqlx::{PgPool, prelude::FromRow};
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

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    role: i32,
    organisation_id: Option<Uuid>,
    station_id: Option<Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    is_active: bool,
}

impl TryFrom<UserRow> for User {
    type Error = DomainError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.id,
            username: Username::parse(row.username)?,
            email: Email::parse(row.email)?,
            password_hash: row.password_hash,
            role: crate::domain::value_objects::Role::from_i32(row.role)?,
            organisation_id: row.organisation_id,
            station_id: row.station_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            is_active: row.is_active,
        })
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPg {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active 
               FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        row.map(|r| r.try_into()).transpose()
    }

    async fn get_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        let username_str = username.as_ref();
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active 
               FROM users WHERE username = $1"#,
            username_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        row.map(|r| r.try_into()).transpose()
    }

    async fn get_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let email_str = email.as_ref();
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active 
               FROM users WHERE email = $1"#,
            email_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        row.map(|r| r.try_into()).transpose()
    }

    async fn save(&self, user: &User) -> Result<User, DomainError> {
        let role_i32 = user.role.to_i32();
        let row = sqlx::query_as!(
            UserRow,
            r#"INSERT INTO users (id, username, email, password_hash, role, organisation_id, station_id, created_at, updated_at, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
               RETURNING id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active"#,
            user.id,
            user.username.as_ref(),
            user.email.as_ref(),
            user.password_hash,
            role_i32,
            user.organisation_id,
            user.station_id,
            user.created_at,
            user.updated_at,
            user.is_active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        row.try_into()
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        let role_i32 = user.role.to_i32();
        let row = sqlx::query_as!(
            UserRow,
            r#"UPDATE users 
               SET username = $2, email = $3, password_hash = $4, role = $5, organisation_id = $6, station_id = $7, updated_at = $8, is_active = $9
               WHERE id = $1
               RETURNING id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active"#,
            user.id,
            user.username.as_ref(),
            user.email.as_ref(),
            user.password_hash,
            role_i32,
            user.organisation_id,
            user.station_id,
            chrono::Utc::now(),
            user.is_active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        row.try_into()
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(())
    }

    async fn list_by_organisation(&self, organisation_id: Uuid) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query_as!(
            UserRow,
            r#"SELECT id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active 
               FROM users WHERE organisation_id = $1 AND is_active = true"#,
            organisation_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        rows.into_iter().map(|r| r.try_into()).collect()
    }

    async fn list_by_station(&self, station_id: Uuid) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query_as!(
            UserRow,
            r#"SELECT id, username, email, password_hash, role as "role: i32", organisation_id, station_id, created_at, updated_at, is_active 
               FROM users WHERE station_id = $1 AND is_active = true"#,
            station_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        rows.into_iter().map(|r| r.try_into()).collect()
    }
}
