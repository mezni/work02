use crate::infrastructure::config::Settings;
use crate::infrastructure::errors::InfrastructureError;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{error, info};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, InfrastructureError> {
        info!("Connecting to database...");

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to database: {}", e);
                InfrastructureError::Database(e)
            })?;

        info!("Database connection established successfully");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> Result<(), InfrastructureError> {
        info!("Running database migrations...");

        sqlx::migrate!("./migrations")
            .run(self.pool())
            .await
            .map_err(|e| {
                error!("Migration failed: {}", e);
                InfrastructureError::Database(e.into()) // Add .into() here
            })?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool, InfrastructureError> {
        sqlx::query("SELECT 1")
            .execute(self.pool())
            .await
            .map(|_| true)
            .map_err(InfrastructureError::Database)
    }
}

pub async fn create_database(settings: &Settings) -> Result<Database, InfrastructureError> {
    Database::new(&settings.database.url).await
}
