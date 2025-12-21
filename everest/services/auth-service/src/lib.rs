pub mod core;

use crate::core::config::Config;
use crate::core::database;
use actix_web::{App, HttpServer, middleware::Logger, web};
use sqlx::PgPool;

pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    pub http_client: reqwest::Client, // Added for Keycloak/API calls
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        Self {
            config,
            db,
            http_client: reqwest::Client::new(),
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();
    let server_addr = config.server_addr.clone();

    // Initialize Database Pool
    let pool = database::create_pool(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create pool: {}", e))?;

    let app_state = web::Data::new(AppState::new(config.clone(), pool));

    tracing::info!("Starting Actix server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
