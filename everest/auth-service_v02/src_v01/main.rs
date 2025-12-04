mod keycloak_client;
mod register_service;
mod user_repository;

use keycloak_client::KeycloakClient;
use register_service::RegisterService;
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Keycloak client
    let kc_client = KeycloakClient::new(
        "http://localhost:5080",
        "myrealm",
        "backend-admin",
        "backend-admin-secret",
    );

    let user_repo = user_repository::UserRepository::new(kc_client);
    let register_service = RegisterService::new(user_repo);

    // Optional attributes
    let mut attributes = HashMap::new();
    attributes.insert("company_name".to_string(), vec!["ACME Corp".to_string()]);
    attributes.insert("station_name".to_string(), vec!["Station 42".to_string()]);

    // Register user via service
    match register_service
        .register_user("john", "John", "Doe", "Password123!", Some(attributes))
        .await
    {
        Ok(user_id) => println!("✅ User created with ID: {}", user_id),
        Err(err) => eprintln!("❌ Failed to create user: {}", err),
    }

    Ok(())
}
