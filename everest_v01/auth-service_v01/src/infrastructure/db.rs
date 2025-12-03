// db.rs
use crate::infrastructure::config::Config;
use crate::infrastructure::errors::InfrastructureError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, InfrastructureError> {
        let pool = PgPool::connect(&config.db_url).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
