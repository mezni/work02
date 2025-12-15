use crate::core::errors::AppResult;
use crate::domain::user::User;
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<User>;
    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;
    async fn update(&self, user: &User) -> AppResult<User>;
    async fn delete(&self, user_id: &str) -> AppResult<()>;
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn search(&self, query: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_by_network(&self, network_id: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_by_station(&self, station_id: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_by_role(&self, role: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_by_source(&self, source: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_active(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_inactive(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_verified(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn count(&self) -> AppResult<i64>;
    async fn count_active(&self) -> AppResult<i64>;
    async fn count_verified(&self) -> AppResult<i64>;
    async fn count_by_role(&self) -> AppResult<HashMap<String, i64>>;
    async fn count_by_source(&self) -> AppResult<HashMap<String, i64>>;
}

pub struct PostgresUserRepository {
    pub(crate) pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
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

    async fn find_by_source(&self, source: &str, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE source = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(source)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_active(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE is_active = TRUE ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_inactive(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE is_active = FALSE ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_verified(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let results = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE is_verified = TRUE ORDER BY created_at DESC LIMIT $1 OFFSET $2"
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

    async fn count_active(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE is_active = TRUE")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    async fn count_verified(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE is_verified = TRUE")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    async fn count_by_role(&self) -> AppResult<HashMap<String, i64>> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT role, COUNT(*) as count FROM users GROUP BY role"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut map = HashMap::new();
        for (role, count) in rows {
            map.insert(role, count);
        }

        Ok(map)
    }

    async fn count_by_source(&self) -> AppResult<HashMap<String, i64>> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT source, COUNT(*) as count FROM users GROUP BY source"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut map = HashMap::new();
        for (source, count) in rows {
            map.insert(source, count);
        }

        Ok(map)
    }
}