// src/main.rs
//! Authentication Service Entry Point
//!
//! This is the main entry point for the authentication service
//! backed by Keycloak with Swagger documentation.

use auth_service::{Application, config::AppConfig};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file (if exists)
    dotenv().ok();

    // Load configuration from environment
    let config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
            std::process::exit(1);
        }
    };

    // Handle graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        
        log::info!("Received shutdown signal, starting graceful shutdown...");
    };

    // Run the application
    if let Err(err) = Application::run(config).await {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }

    // Wait for shutdown signal (this will never be reached in normal operation
    // as the server runs until stopped)
    shutdown_signal.await;
    
    log::info!("Auth service shutdown complete");
    Ok(())
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        // Set test environment variables
        std::env::set_var("APP_SERVER_HOST", "127.0.0.1");
        std::env::set_var("APP_SERVER_PORT", "8080");
        std::env::set_var("APP_KEYCLOAK_BASE_URL", "http://localhost:8080");
        std::env::set_var("APP_KEYCLOAK_REALM", "test-realm");
        std::env::set_var("APP_KEYCLOAK_CLIENT_ID", "test-client");
        std::env::set_var("APP_KEYCLOAK_CLIENT_SECRET", "test-secret");
        std::env::set_var("APP_KEYCLOAK_ADMIN_USERNAME", "admin");
        std::env::set_var("APP_KEYCLOAK_ADMIN_PASSWORD", "password");
        std::env::set_var("APP_LOGGING_LEVEL", "info");

        let config = AppConfig::from_env();
        assert!(config.is_ok());

        // Cleanup
        std::env::remove_var("APP_SERVER_HOST");
        std::env::remove_var("APP_SERVER_PORT");
        std::env::remove_var("APP_KEYCLOAK_BASE_URL");
        std::env::remove_var("APP_KEYCLOAK_REALM");
        std::env::remove_var("APP_KEYCLOAK_CLIENT_ID");
        std::env::remove_var("APP_KEYCLOAK_CLIENT_SECRET");
        std::env::remove_var("APP_KEYCLOAK_ADMIN_USERNAME");
        std::env::remove_var("APP_KEYCLOAK_ADMIN_PASSWORD");
        std::env::remove_var("APP_LOGGING_LEVEL");
    }
}