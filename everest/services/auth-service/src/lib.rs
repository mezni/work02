pub mod core;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

use crate::core::config::Config;
use crate::core::database;
use crate::infrastructure::repositories::{
    audit_repository::PostgresAuditRepository,
    user_repository::PostgresUserRepository,
    outbox_repository::PostgresOutboxRepository,
};
use crate::infrastructure::keycloak_client::KeycloakClient;
use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<PostgresUserRepository>,
    pub audit_repo: Arc<PostgresAuditRepository>,
    pub outbox_repo: Arc<PostgresOutboxRepository>,
    pub keycloak: Arc<KeycloakClient>,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // Create database pool
        let pool = database::create_pool(&config.database.url, config.database.max_connections).await?;

        // Run migrations
        database::run_migrations(&pool).await?;

        // Initialize repositories
        let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
        let audit_repo = Arc::new(PostgresAuditRepository::new(pool.clone()));

        // Initialize Keycloak client
        let keycloak = Arc::new(KeycloakClient::new(config.keycloak.clone()));

        Ok(Self {
            user_repo,
            audit_repo,
            outbox_repo,
            keycloak,
            config,
        })
    }
}

pub async fn start_server(config: Config) -> anyhow::Result<()> {
    // Initialize logging
    crate::core::logging::init_logging(&config.logging.level)?;

    tracing::info!("🚀 Starting auth-service v{}", env!("CARGO_PKG_VERSION"));

    // Initialize application state
    let state = AppState::new(config.clone()).await?;

    // Start HTTP server
    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    tracing::info!("🌐 Server listening on {}:{}", server_host, server_port);
    tracing::info!("📚 Swagger UI: http://{}:{}/swagger-ui/", server_host, server_port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(interfaces::http::middleware::cors())
            .app_data(web::Data::new(state.clone()))
            .configure(interfaces::http::routes::configure)
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_clone() {
        // AppState should be Clone for use in Actix-web
        fn is_clone<T: Clone>() {}
        is_clone::<AppState>();
    }
}