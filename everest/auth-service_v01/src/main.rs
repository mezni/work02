mod application;
mod domain;
mod infrastructure;

use crate::application::dto::user_dto::CreateUserDto;
use crate::application::services::user_service::UserService;
use crate::infrastructure::config::keycloak_config::KeycloakConfig;
use crate::infrastructure::keycloak::client::KeycloakClient;
use crate::infrastructure::persistence::keycloak_user_repository::KeycloakUserRepository;
use anyhow::Result;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Auth Service");

    // Load configuration from environment
    let config = KeycloakConfig::from_env().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;

    info!("Configuration loaded successfully");
    info!("Keycloak URL: {}", config.url);
    info!("Keycloak Realm: {}", config.realm);

    // Initialize Keycloak client
    let keycloak_client = KeycloakClient::new(config).await.map_err(|e| {
        error!("Failed to initialize Keycloak client: {}", e);
        e
    })?;

    // Initialize repository
    let user_repository = Arc::new(KeycloakUserRepository::new(Arc::new(keycloak_client)));

    // Initialize application service
    let user_service = UserService::new(user_repository);

    info!("Auth Service initialized successfully");

    // Example usage
    run_examples(&user_service).await?;

    info!("Auth Service completed successfully");
    Ok(())
}

async fn run_examples(user_service: &UserService) -> Result<()> {
    // Example 1: Create a new user
    info!("=== Example 1: Creating a new user ===");
    let create_user_dto = CreateUserDto {
        username: "testuser".to_string(),
        email: "testuser@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        password: "Test123!".to_string(),
    };

    match user_service.create_user(create_user_dto).await {
        Ok(user_id) => {
            info!("✓ User created successfully with ID: {}", user_id);

            // Example 2: Assign role to user
            info!("\n=== Example 2: Assigning role to user ===");
            match user_service.assign_role(&user_id, "user-manager").await {
                Ok(_) => {
                    info!("✓ Role assigned successfully");

                    // Example 3: Get user roles
                    info!("\n=== Example 3: Getting user roles ===");
                    match user_service.get_user_roles(&user_id).await {
                        Ok(roles) => {
                            info!("✓ User roles:");
                            for role in roles {
                                info!("  - {}", role);
                            }
                        }
                        Err(e) => error!("✗ Failed to get roles: {}", e),
                    }
                }
                Err(e) => error!("✗ Failed to assign role: {}", e),
            }

            // Example 4: Get user details
            info!("\n=== Example 4: Getting user details ===");
            match user_service.get_user(&user_id).await {
                Ok(Some(user)) => {
                    info!("✓ User details:");
                    info!("  ID: {}", user.id);
                    info!("  Username: {}", user.username);
                    info!("  Email: {}", user.email);
                    info!("  Name: {} {}", user.first_name, user.last_name);
                    info!("  Enabled: {}", user.enabled);
                }
                Ok(None) => error!("✗ User not found"),
                Err(e) => error!("✗ Failed to get user: {}", e),
            }
        }
        Err(e) => error!("✗ Failed to create user: {}", e),
    }

    // Example 5: List all users
    info!("\n=== Example 5: Listing all users ===");
    match user_service.list_users().await {
        Ok(users) => {
            info!("✓ Found {} users:", users.len());
            for user in users.iter().take(5) {
                info!("  - {} ({})", user.username, user.email);
            }
            if users.len() > 5 {
                info!("  ... and {} more", users.len() - 5);
            }
        }
        Err(e) => error!("✗ Failed to list users: {}", e),
    }

    Ok(())
}