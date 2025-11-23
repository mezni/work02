use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub connect_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        // Provide sensible defaults for development
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            // Fallback to a default development URL
            "postgres://configurator_user:configurator_password@localhost:5432/configurator_db"
                .to_string()
        });

        Self {
            database_url,
            max_connections: 10,
            connect_timeout: Duration::from_secs(30),
        }
    }
}

pub async fn create_pool(config: DatabaseConfig) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connect_timeout)
        .connect(&config.database_url)
        .await
}

pub async fn create_pool_from_env() -> Result<PgPool, Error> {
    let config = DatabaseConfig::default();
    create_pool(config).await
}
