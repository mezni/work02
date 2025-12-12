
#[cfg(feature = "database")]
pub mod db {
    use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
    use std::time::Duration;
    
    pub type DbPool = Pool<Postgres>;
    
    pub async fn create_pool(
        database_url: &str,
        max_connections: u32,
        min_connections: u32,
    ) -> Result<DbPool, sqlx::Error> {
        tracing::info!("Creating database connection pool");
        
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await?;
        
        tracing::info!("Database connection pool created successfully");
        
        Ok(pool)
    }
    
    pub async fn health_check(pool: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .fetch_one(pool)
            .await?;
        Ok(())
    }
}
