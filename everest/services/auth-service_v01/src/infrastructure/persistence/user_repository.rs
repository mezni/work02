use crate::core::errors::{AppError, AppResult};
use crate::domain::user_entity::User;
use crate::domain::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;

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
    async fn create(&self, user: &User) -> AppResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                user_id, keycloak_id, email, username, first_name, last_name,
                phone, photo, is_verified, role, network_id, station_id,
                source, is_active, created_by, updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(&user.role)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(&user.source)
        .bind(user.is_active)
        .bind(&user.created_by)
        .bind(&user.updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE keycloak_id = $1")
            .bind(keycloak_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn update(&self, user: &User) -> AppResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $2, username = $3, first_name = $4, last_name = $5,
                phone = $6, photo = $7, is_verified = $8, role = $9,
                network_id = $10, station_id = $11, is_active = $12,
                updated_at = NOW(), updated_by = $13
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(&user.role)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.is_active)
        .bind(&user.updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, user_id: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn count(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    async fn find_by_network(&self, network_id: &str, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE network_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(network_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_by_station(&self, station_id: &str, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE station_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(station_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_by_role(&self, role: &str, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE role = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(role)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn search(&self, query: &str, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let search_pattern = format!("%{}%", query);
        let results = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users 
            WHERE email ILIKE $1 
               OR username ILIKE $1 
               OR first_name ILIKE $1 
               OR last_name ILIKE $1
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(&search_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}