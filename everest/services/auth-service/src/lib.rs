pub mod application;
pub mod core;
pub mod infrastructure;
pub mod presentation;

use crate::core::config::Config;
use crate::core::database;
use crate::infrastructure::keycloak_client::{HttpKeycloakClient, KeycloakClient};

use actix_web::{App, HttpServer, middleware::Logger, web};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    // Use Arc so we can share the trait object across threads
    pub keycloak: Arc<dyn KeycloakClient>,
}

impl AppState {
    pub fn new(config: Config, db: PgPool, keycloak: Arc<dyn KeycloakClient>) -> Self {
        Self {
            config,
            db,
            keycloak,
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();
    let server_addr = config.server_addr.clone();

    // 1. Initialize Database
    let pool = database::create_pool(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

    // 2. Initialize the concrete implementation of Keycloak
    let keycloak_impl = HttpKeycloakClient::new(
        config.keycloak_url.clone(),
        config.keycloak_realm.clone(),
        config.keycloak_backend_client_id.clone(),
        config.keycloak_backend_client_secret.clone(),
        config.keycloak_auth_client_id.clone(),
    );

    // 3. Wrap in Arc and pass to state
    let keycloak_trait_object: Arc<dyn KeycloakClient> = Arc::new(keycloak_impl);

    let app_state = web::Data::new(AppState::new(config.clone(), pool, keycloak_trait_object));

    tracing::info!("Starting Actix server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(presentation::controllers::health_controller::get_health)
            .configure(presentation::openapi::configure_openapi)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
