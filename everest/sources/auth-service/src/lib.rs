// src/lib.rs
//! Authentication Service
//!
//! A domain-driven authentication service with Keycloak integration
//! and OpenAPI/Swagger documentation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(dead_code)] // Temporary during development

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;
pub mod config;

// Re-exports for common usage
pub use domain::DomainError;
pub use application::ApplicationError;
pub use infrastructure::KeycloakError;
pub use api::ApiError;

/// Application result type
pub type Result<T> = std::result::Result<T, ApplicationError>;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
    pub keycloak_client: std::sync::Arc<dyn infrastructure::keycloak::KeycloakClient>,
    pub user_repository: std::sync::Arc<dyn domain::repositories::UserRepository>,
    pub user_service: std::sync::Arc<dyn application::UserService>,
    pub event_publisher: std::sync::Arc<dyn infrastructure::event_bus::EventPublisher>,
}

impl AppState {
    /// Create new application state
    pub fn new(
        config: config::AppConfig,
        keycloak_client: std::sync::Arc<dyn infrastructure::keycloak::KeycloakClient>,
        user_repository: std::sync::Arc<dyn domain::repositories::UserRepository>,
        user_service: std::sync::Arc<dyn application::UserService>,
        event_publisher: std::sync::Arc<dyn infrastructure::event_bus::EventPublisher>,
    ) -> Self {
        Self {
            config,
            keycloak_client,
            user_repository,
            user_service,
            event_publisher,
        }
    }
}

/// Application factory for creating the web server
pub struct Application;

impl Application {
    /// Build and configure the Actix Web application
    pub fn build(config: config::AppConfig) -> std::io::Result<actix_web::HttpServer<impl Fn() -> actix_web::App>> {
        // Initialize keycloak client
        let keycloak_client = std::sync::Arc::new(
            infrastructure::keycloak::KeycloakClientImpl::new(
                config.keycloak.base_url.clone(),
                config.keycloak.realm.clone(),
                config.keycloak.client_id.clone(),
                config.keycloak.client_secret.clone(),
                config.keycloak.admin_username.clone(),
                config.keycloak.admin_password.clone(),
            )
        );

        // Initialize user repository
        let user_repository = std::sync::Arc::new(
            infrastructure::keycloak::KeycloakUserRepository::new(
                keycloak_client.clone()
            )
        );

        // Initialize user service
        let user_service = std::sync::Arc::new(
            application::UserApplicationService::new(
                user_repository.clone()
            )
        );

        // Initialize event publisher
        let event_publisher = std::sync::Arc::new(
            infrastructure::event_bus::InMemoryEventBus::new()
        );

        // Create application state
        let app_state = AppState::new(
            config.clone(), 
            keycloak_client, 
            user_repository, 
            user_service,
            event_publisher
        );

        // Build HTTP server
        let server = actix_web::HttpServer::new(move || {
            let state = actix_web::web::Data::new(app_state.clone());
            
            // Create handlers with the user service
            let user_service_data = actix_web::web::Data::new(
                app_state.user_service.clone() as std::sync::Arc<dyn application::UserService>
            );
            
            let (user_handler, admin_handler, health_handler) = api::routes::create_handlers(
                user_service_data
            );

            actix_web::App::new()
                .app_data(state)
                .app_data(actix_web::web::Data::new(user_handler))
                .app_data(actix_web::web::Data::new(admin_handler))
                .app_data(actix_web::web::Data::new(health_handler))
                .configure(api::routes::configure_routes)
                .configure(api::openapi::configure_swagger)
                .wrap(actix_web::middleware::Logger::default())
                .wrap(api::middleware::RequestLogger)
                .wrap(api::middleware::Authentication)
                .wrap(
                    actix_cors::Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600)
                )
                .default_service(
                    actix_web::web::route()
                        .to(api::not_found_handler)
                )
        })
        .workers(config.server.workers.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        }));

        Ok(server)
    }

    /// Run the application
    pub async fn run(config: config::AppConfig) -> std::io::Result<()> {
        // Initialize logging
        Self::init_logging(&config.logging);

        log::info!("Starting auth service on {}:{}", config.server.host, config.server.port);
        log::info!("Keycloak realm: {}", config.keycloak.realm);
        log::info!("Environment: {}", std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()));

        // Build and run server
        let server = Self::build(config)?;
        let bind_address = format!("{}:{}", 
            std::env::var("APP_SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            std::env::var("APP_SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
        );
        
        log::info!("Binding to: {}", bind_address);
        server.bind(&bind_address)?.run().await
    }

    fn init_logging(logging_config: &config::LoggingConfig) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", &logging_config.level);
        }

        env_logger::Builder::from_default_env()
            .format_timestamp_millis()
            .format_module_path(false)
            .format_target(false)
            .init();

        log::info!("Logging initialized with level: {}", logging_config.level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let config = config::AppConfig {
            server: config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: Some(4),
            },
            keycloak: config::KeycloakConfig {
                base_url: "http://localhost:8080".to_string(),
                realm: "master".to_string(),
                client_id: "admin-cli".to_string(),
                client_secret: "secret".to_string(),
                admin_username: "admin".to_string(),
                admin_password: "admin".to_string(),
            },
            logging: config::LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        };

        let keycloak_client = std::sync::Arc::new(
            infrastructure::keycloak::KeycloakClientImpl::new(
                config.keycloak.base_url.clone(),
                config.keycloak.realm.clone(),
                config.keycloak.client_id.clone(),
                config.keycloak.client_secret.clone(),
                config.keycloak.admin_username.clone(),
                config.keycloak.admin_password.clone(),
            )
        );

        let user_repository = std::sync::Arc::new(
            infrastructure::keycloak::KeycloakUserRepository::new(
                keycloak_client.clone()
            )
        );

        let user_service = std::sync::Arc::new(
            application::UserApplicationService::new(
                user_repository.clone()
            )
        );

        let event_publisher = std::sync::Arc::new(
            infrastructure::event_bus::InMemoryEventBus::new()
        );

        let app_state = AppState::new(
            config, 
            keycloak_client, 
            user_repository, 
            user_service,
            event_publisher
        );

        assert_eq!(app_state.config.server.port, 8080);
    }

    #[test]
    fn test_domain_error_reexport() {
        // Test that DomainError is properly re-exported
        let error = DomainError::validation("test error");
        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[test]
    fn test_application_error_reexport() {
        // Test that ApplicationError is properly re-exported
        let error = ApplicationError::validation("test error");
        assert!(matches!(error, ApplicationError::Validation(_)));
    }

    #[test]
    fn test_api_error_reexport() {
        // Test that ApiError is properly re-exported
        let error = ApiError::BadRequest("test".to_string());
        assert!(matches!(error, ApiError::BadRequest(_)));
    }

    #[test]
    fn test_keycloak_error_reexport() {
        // Test that KeycloakError is properly re-exported
        let error = KeycloakError::authentication("test");
        assert!(matches!(error, KeycloakError::Authentication(_)));
    }

    #[tokio::test]
    async fn test_application_build() {
        let config = config::AppConfig {
            server: config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: Some(1),
            },
            keycloak: config::KeycloakConfig {
                base_url: "http://localhost:8080".to_string(),
                realm: "master".to_string(),
                client_id: "admin-cli".to_string(),
                client_secret: "secret".to_string(),
                admin_username: "admin".to_string(),
                admin_password: "admin".to_string(),
            },
            logging: config::LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        };

        let result = Application::build(config);
        assert!(result.is_ok());
    }
}