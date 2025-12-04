use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize)]
struct Credential {
    #[serde(rename = "type")]
    cred_type: String,
    value: String,
    temporary: bool,
}

#[derive(Serialize)]
struct CreateUserDto {
    username: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    enabled: bool,
    credentials: Vec<Credential>,
    attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
struct KeycloakToken {
    access_token: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct KeycloakUser {
    id: String,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct RoleMapping {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // 1️⃣ Get admin token via client credentials
    let resp = client
        .post("http://localhost:5080/realms/myrealm/protocol/openid-connect/token")
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", "backend-admin"),
            ("client_secret", "backend-admin-secret"),
        ])
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        eprintln!("❌ Failed to get admin token: {}", text);
        return Ok(());
    }
    let token: KeycloakToken = serde_json::from_str(&text)?;
    println!("✅ Got access token");

    // 2️⃣ Create a new user with attributes
    let mut attributes = HashMap::new();
    attributes.insert("company_name".to_string(), vec!["ACME Corp".to_string()]);
    attributes.insert("station_name".to_string(), vec!["Station 42".to_string()]);

    let new_user = CreateUserDto {
        username: "john".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        enabled: true,
        credentials: vec![Credential {
            cred_type: "password".to_string(),
            value: "Password123!".to_string(),
            temporary: false,
        }],
        attributes: Some(attributes),
    };

    let create_resp = client
        .post("http://localhost:5080/admin/realms/myrealm/users")
        .bearer_auth(&token.access_token)
        .json(&new_user)
        .send()
        .await?;

    if create_resp.status().is_success() {
        println!("✅ User created successfully");
    } else {
        let text = create_resp.text().await?;
        eprintln!("❌ Failed to create user: {}", text);
        return Ok(());
    }

    // 3️⃣ Fetch the created user's ID
    let users_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/users")
        .bearer_auth(&token.access_token)
        .query(&[("username", "john")])
        .send()
        .await?;

    let users_status = users_resp.status();
    let users_text = users_resp.text().await?;
    if !users_status.is_success() {
        eprintln!("❌ Failed to fetch user ID: {}", users_text);
        return Ok(());
    }

    let users: Vec<KeycloakUser> = serde_json::from_str(&users_text)?;
    if users.is_empty() {
        eprintln!("❌ User not found after creation");
        return Ok(());
    }

    let user_id = &users[0].id;
    println!("✅ User ID: {}", user_id);

    // 4️⃣ Fetch the role ID for "user"
    let roles_resp = client
        .get("http://localhost:5080/admin/realms/myrealm/roles/user")
        .bearer_auth(&token.access_token)
        .send()
        .await?;

    let roles_status = roles_resp.status();
    let roles_text = roles_resp.text().await?;
    if !roles_status.is_success() {
        eprintln!("❌ Failed to fetch role ID: {}", roles_text);
        return Ok(());
    }

    let role: RoleMapping = serde_json::from_str(&roles_text)?;

    // 5️⃣ Assign default realm role to the created user
    let assign_resp = client
        .post(format!(
            "http://localhost:5080/admin/realms/myrealm/users/{}/role-mappings/realm",
            user_id
        ))
        .bearer_auth(&token.access_token)
        .json(&vec![role])
        .send()
        .await?;

    if assign_resp.status().is_success() {
        println!("✅ Default role 'user' assigned successfully");
    } else {
        let assign_text = assign_resp.text().await?;
        eprintln!("❌ Failed to assign role: {}", assign_text);
    }

    // 6️⃣ Fetch the user again and print all attributes
    let user_resp = client
        .get(format!("http://localhost:5080/admin/realms/myrealm/users/{}", user_id))
        .bearer_auth(&token.access_token)
        .send()
        .await?;

    let user_text = user_resp.text().await?;
    println!("👤 Full user JSON:\n{}", user_text);

    Ok(())
}
