use anyhow::{Result, Context};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::{info, error};
use crate::config::AppConfig;

pub type DbPool = Pool<Postgres>;

#[derive(Clone)]
pub struct Database {
    pool: DbPool,
}

impl Database {
    pub async fn connect(config: &AppConfig) -> Result<Self> {
        info!("Connecting to database at {}...", config.database.url);
        
        let pool = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&config.database.url)
            .await
            .context("Failed to connect to database")?;
        
        info!("Database connection established");
        
        Ok(Self { pool })
    }
    
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
    
    pub async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations...");
        
        match sqlx::migrate!("./migrations")
            .run(self.pool())
            .await
        {
            Ok(_) => {
                info!("Database migrations completed successfully");
                Ok(())
            }
            Err(e) => {
                error!("Database migrations failed: {}", e);
                Err(e.into())
            }
        }
    }
    
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(self.pool())
            .await
            .context("Database health check failed")?;
        
        Ok(())
    }
    
    pub async fn transaction(&self) -> Result<sqlx::Transaction<'_, Postgres>> {
        self.pool()
            .begin()
            .await
            .context("Failed to begin transaction")
    }
    
    // Optional: Connection pool metrics
    pub fn get_connection_metrics(&self) -> ConnectionMetrics {
        ConnectionMetrics {
            size: self.pool.size(),
            idle: self.pool.num_idle(),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionMetrics {
    pub size: u32,
    pub idle: usize,
}

// Repository implementations for caching or additional storage
#[derive(Clone)]
pub struct CacheRepository {
    pool: DbPool,
}

impl CacheRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    pub async fn store_session(
        &self,
        user_id: &uuid::Uuid,
        session_data: &serde_json::Value,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_sessions (user_id, session_data, expires_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE
            SET session_data = $2, expires_at = $3, updated_at = NOW()
            "#,
            user_id,
            session_data,
            expires_at,
        )
        .execute(self.pool())
        .await?;
        
        Ok(())
    }
    
    pub async fn get_session(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Option<serde_json::Value>> {
        let record = sqlx::query!(
            r#"
            SELECT session_data FROM user_sessions
            WHERE user_id = $1 AND expires_at > NOW()
            "#,
            user_id,
        )
        .fetch_optional(self.pool())
        .await?;
        
        Ok(record.map(|r| r.session_data))
    }
    
    pub async fn delete_session(&self, user_id: &uuid::Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM user_sessions WHERE user_id = $1",
            user_id,
        )
        .execute(self.pool())
        .await?;
        
        Ok(())
    }
}

// If you need to store refresh tokens in database (alternative to Redis)
#[derive(Clone)]
pub struct TokenRepository {
    pool: DbPool,
}

impl TokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    pub async fn store_refresh_token(
        &self,
        user_id: uuid::Uuid,
        token_hash: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
        device_info: Option<&str>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at, device_info)
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            token_hash,
            expires_at,
            device_info,
        )
        .execute(self.pool())
        .await?;
        
        Ok(())
    }
    
    pub async fn validate_refresh_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<uuid::Uuid>> {
        let record = sqlx::query!(
            r#"
            SELECT user_id FROM refresh_tokens
            WHERE token_hash = $1 AND expires_at > NOW() AND revoked = false
            "#,
            token_hash,
        )
        .fetch_optional(self.pool())
        .await?;
        
        Ok(record.map(|r| r.user_id))
    }
    
    pub async fn revoke_refresh_token(&self, token_hash: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE refresh_tokens SET revoked = true WHERE token_hash = $1",
            token_hash,
        )
        .execute(self.pool())
        .await?;
        
        Ok(())
    }
    
    pub async fn revoke_all_user_tokens(&self, user_id: uuid::Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE refresh_tokens SET revoked = true WHERE user_id = $1",
            user_id,
        )
        .execute(self.pool())
        .await?;
        
        Ok(())
    }
}