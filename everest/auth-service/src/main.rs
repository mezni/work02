use std::error::Error;
use reqwest::Client;
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Configuration ---
    const KEYCLOAK_PORT: &str = "5080"; 
    const REALM_NAME: &str = "ev-realm"; 
    const CLIENT_ID: &str = "auth-client";
    const CLIENT_SECRET: &str = "my_secret_value"; // Replace with your actual secret!

    let url = format!("http://localhost:{KEYCLOAK_PORT}");
    let token_endpoint = format!("{url}/realms/{REALM_NAME}/protocol/openid-connect/token");

    println!("--- Keycloak Connection Test ---");

    let client = Client::new();

    // The '?' operator here will automatically convert reqwest::Error into Box<dyn Error>
    let response = client
        .post(&token_endpoint)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
        ])
        .send()
        .await?; 

    let status = response.status();
    
    if status.is_success() {
        // Again, '?' converts reqwest::Error into Box<dyn Error>
        let token_response: Value = response.json().await?; 
        
        println!("\n✅ Connection and Client Authentication Successful!");
        println!("Status Code: {}", status);
        
        if let Some(access_token) = token_response.get("access_token").and_then(Value::as_str) {
            println!("  * Access Token received ({} bytes)", access_token.len());
        }
        if let Some(expires_in) = token_response.get("expires_in").and_then(Value::as_u64) {
            println!("  * Expires In: {} seconds", expires_in);
        }
        
    } else {
        // The failure path:
        // We now use .into() on a formatted String, which correctly converts
        // the String (which implements Error) to Box<dyn Error>, fixing E0308.
        let body = response.text().await.unwrap_or_else(|_| "[No response body]".to_string());
        
        println!("\n❌ Connection or Client Authentication FAILED!");
        println!("  * Status Code: {}", status);
        println!("  * Error Details: {}", body);
        
        let error_message = format!(
            "Keycloak token request failed with status: {} and details: {}", 
            status, 
            body
        );
        return Err(error_message.into()); // Using .into() on the String error
    }
    
    Ok(())
}