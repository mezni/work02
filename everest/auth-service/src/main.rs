mod keycloak_client;
mod user_repository;

use keycloak_client::KeycloakClient;
use std::collections::HashMap;
use user_repository::UserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1️⃣ Initialize Keycloak client
    let kc_client = KeycloakClient::new(
        "http://localhost:5080",
        "myrealm",
        "backend-admin",
        "backend-admin-secret",
    );

    // 2️⃣ Initialize repository
    let user_repo = UserRepository::new(kc_client);

    // 3️⃣ Define user attributes
    let mut attrs = HashMap::new();
    attrs.insert("company_name".to_string(), vec!["ACME Corp".to_string()]);
    attrs.insert("station_name".to_string(), vec!["Station 42".to_string()]);

    // 4️⃣ Register user
    match user_repo
        .register_user("john", "John", "Doe", "Password123!", Some(attrs))
        .await
    {
        Ok(user_id) => println!("✅ User created successfully, ID: {}", user_id),
        Err(err) => eprintln!("❌ Failed to create user: {}", err),
    }

    Ok(())
}
