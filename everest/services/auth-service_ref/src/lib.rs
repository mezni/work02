pub mod core;
/*
pub mod domain;
pub mod infrastructure;
pub mod application;
*/
pub mod interfaces;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use core::config::Config;
use core::database::create_pool; //, run_migrations};
/*
use infrastructure::keycloak::service::KeycloakService;
use infrastructure::persistence::{audit_repository::PostgresAuditRepository, user_repository::PostgresUserRepository};
use application::services::{auth_service::AuthService, user_service::UserService};
use interfaces::routes;
*/
use crate::interfaces::api_doc::ApiDoc;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    /*
        pub user_service: Arc<UserService>,
        pub auth_service: Arc<AuthService>,
    */
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // Create database pool
        let pool = create_pool(&config.database.url, config.database.max_connections).await?;
        /*
                // Run migrations
                run_migrations(&pool).await?;

                // Initialize repositories
                let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
                let audit_repo = Arc::new(PostgresAuditRepository::new(pool.clone()));

                // Initialize Keycloak service
                let keycloak = Arc::new(KeycloakService::new(config.keycloak.clone()));

                // Initialize application services
                let user_service = Arc::new(UserService::new(
                    user_repo.clone(),
                    audit_repo.clone(),
                    keycloak.clone(),
                ));

                let auth_service = Arc::new(AuthService::new(
                    user_repo.clone(),
                    audit_repo.clone(),
                    keycloak.clone(),
                ));
        */
        Ok(Self {
            //            user_service,
            //            auth_service,
            config: config.clone(),
        })
    }
}

pub async fn start_server(config: Config) -> anyhow::Result<()> {
    // Initialize logging
    core::logging::init_logging(&config.logging.level)?;

    tracing::info!("Starting auth-service...");

    // Initialize application state
    let state = AppState::new(config.clone()).await?;

    // Start HTTP server
    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    tracing::info!("Starting HTTP server at {}:{}", server_host, server_port);
    tracing::info!(
        "Swagger UI available at http://{}:{}/swagger-ui/",
        server_host,
        server_port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(web::Data::new(state.clone()))
            //            .configure(routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_creation() {
        std::env::set_var(
            "DATABASE_URL",
            "postgresql://postgres:password@localhost:6200/test_db",
        );
        std::env::set_var("KEYCLOAK_URL", "http://localhost:8080");
        std::env::set_var("KEYCLOAK_REALM", "test");
        std::env::set_var("KEYCLOAK_AUTH_CLIENT_ID", "test-client");
        std::env::set_var("KEYCLOAK_BACKEND_CLIENT_ID", "test-backend");
        std::env::set_var("KEYCLOAK_BACKEND_CLIENT_SECRET", "secret");

        // This test will fail without a running database
        // but demonstrates the structure
    }
}
