use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use keycloak::types::UserRepresentation;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Module-level constants
const ROLE_NAME: &str = "user";
const COMPANY_NAME: &str = "X";
const STATION_NAME: &str = "X";

#[derive(Serialize, Deserialize)]
struct CreateUserDto {
    username: String,
    email: String,
    firstName: String,
    lastName: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthDto {
    username: String,
    password: String,
}

async fn create_user(
    user_dto: web::Json<CreateUserDto>,
    client: web::Data<reqwest::Client>,
) -> impl Responder {
    println!("=== CREATE USER REQUEST ===");
    println!("Username: {}", user_dto.username);

    let url = "http://localhost:5080/auth";
    let realm_name = "master";

    // Get admin token
    let admin_token = match KeycloakAdminToken::acquire(url, "admin", "admin", &client).await {
        Ok(token) => {
            println!("✓ Admin token acquired");
            token
        }
        Err(e) => {
            eprintln!("✗ Failed to get admin token: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to authenticate as admin");
        }
    };

    let admin = KeycloakAdmin::new(url, admin_token, client.as_ref().clone());

    // Create attributes HashMap with empty strings
    let mut attributes = HashMap::new();
    attributes.insert("company_name".to_string(), vec![COMPANY_NAME.to_string()]);
    attributes.insert("station_name".to_string(), vec![STATION_NAME.to_string()]);

    // Create user with required fields and attributes
    let user_rep = UserRepresentation {
        username: Some(user_dto.username.clone()),
        email: Some(user_dto.email.clone()),
        first_name: Some(user_dto.firstName.clone()),
        last_name: Some(user_dto.lastName.clone()),
        enabled: Some(true),
        email_verified: Some(true),
        realm_roles: Some(vec![ROLE_NAME.to_string()]),
        attributes: Some(attributes),
        credentials: Some(vec![keycloak::types::CredentialRepresentation {
            r#type_: Some("password".to_string()),
            value: Some(user_dto.password.clone()),
            temporary: Some(false),
            ..Default::default()
        }]),
        ..Default::default()
    };

    match admin.realm_users_post(realm_name, user_rep).await {
        Ok(response) => {
            let user_id_option = response.to_id().map(|id| id.to_string());

            match user_id_option {
                Some(user_id) => {
                    println!("✓ User created with ID: {}", user_id);
                    println!("✓ Role '{}' assigned", ROLE_NAME);
                    println!("✓ Empty attributes added for company_name and station_name");

                    HttpResponse::Created().json(serde_json::json!({
                        "message": format!("User created successfully with '{}' role", ROLE_NAME),
                        "userId": user_id,
                        "role": ROLE_NAME,
                        "attributes": {
                            "company_name": COMPANY_NAME,
                            "station_name": STATION_NAME
                        }
                    }))
                }
                None => {
                    println!("✓ User created (no ID returned)");
                    println!("✓ Role '{}' assigned", ROLE_NAME);

                    HttpResponse::Created().json(serde_json::json!({
                        "message": format!("User created successfully with '{}' role", ROLE_NAME),
                        "role": ROLE_NAME,
                        "attributes": {
                            "company_name": COMPANY_NAME,
                            "station_name": STATION_NAME
                        }
                    }))
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to create user: {:?}", e))
        }
    }
}

async fn authenticate_user(
    auth_dto: web::Json<AuthDto>,
    client: web::Data<reqwest::Client>,
) -> impl Responder {
    println!("=== AUTHENTICATE USER REQUEST ===");

    let token_url = "http://localhost:5080/auth/realms/master/protocol/openid-connect/token";

    let res = client
        .post(token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=password&username={}&password={}&client_id=admin-cli",
            auth_dto.username, auth_dto.password
        ))
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            match response.text().await {
                Ok(body) => {
                    if status.is_success() {
                        match serde_json::from_str::<serde_json::Value>(&body) {
                            Ok(token_response) => HttpResponse::Ok().json(token_response),
                            Err(_) => {
                                HttpResponse::InternalServerError().json("Invalid response format")
                            }
                        }
                    } else {
                        HttpResponse::Unauthorized().json("Invalid credentials")
                    }
                }
                Err(_) => HttpResponse::InternalServerError().json("Failed to read response"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to connect to auth server"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Auth Service...");
    println!("Server will run on: http://127.0.0.1:8081");
    println!("Keycloak URL: http://localhost:5080/auth");
    println!("Default role: {}", ROLE_NAME);

    let client = web::Data::new(reqwest::Client::new());

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::resource("/api/v1/register").route(web::post().to(create_user)))
            .service(web::resource("/api/v1/auth").route(web::post().to(authenticate_user)))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
