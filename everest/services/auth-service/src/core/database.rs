use crate::core::constants::MAX_DB_CONNECTIONS;
use crate::core::errors::AppError;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
