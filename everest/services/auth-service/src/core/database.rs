use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str, max_connections: u32) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await?;

    tracing::info!("Database pool created successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    tracing::info!("Migrations executed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        // This test requires a running PostgreSQL instance
        // Skip in CI/CD environments without database
        if std::env::var("DATABASE_URL").is_ok() {
            let pool = create_pool(
                &std::env::var("DATABASE_URL").unwrap(),
                5
            ).await;
            assert!(pool.is_ok());
        }
    }
}