use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::infrastructure::config::AppConfig;

pub async fn get_db_pool(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
}
