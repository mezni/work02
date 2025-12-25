use actix_web::{App, HttpServer, middleware::Logger, web};
use anyhow::Context;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Internal imports
//pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
//pub mod presentation;

use crate::infrastructure::keycloak_client::HttpKeycloakClient;
use core::{config::Config, database, logging};

pub struct AppState {
    pub config: Config,
    pub db_pool: sqlx::PgPool,
    pub keycloak_client: Arc<HttpKeycloakClient>,
}

pub async fn run() -> anyhow::Result<()> {
    // 1. Load Configuration
    let config = Config::from_env();
    tracing::info!("Configuration loaded");

    // Create database pool
    let db_pool = database::create_pool(&config.database_url).await?;
    tracing::info!("Database connected");

    let keycloak_client = Arc::new(HttpKeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
        config.keycloak_auth_client_id.clone(),
    ));

    // 4. Wrap App State for Actix
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        db_pool,
        keycloak_client,
    });

    Ok(())
}
