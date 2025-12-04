use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct KeycloakToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i32,
    token_type: String,
}

#[post("/authentication")]
async fn authentication(body: web::Json<LoginRequest>) -> impl Responder {
    println!("➡️ Received authentication request for user: {}", body.username);

    let client = Client::new();

    // Password grant request to Keycloak
    let token_resp = client
        .post("http://localhost:5080/realms/myrealm/protocol/openid-connect/token")
        .form(&[
            ("grant_type", "password"),
            ("client_id", "auth-client"), // public client
            ("username", &body.username),
            ("password", &body.password),
        ])
        .send()
        .await;

    match token_resp {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            println!("⬅️ Keycloak responded with status: {}", status);

            if !status.is_success() {
                return HttpResponse::Unauthorized()
                    .body(format!("Authentication failed: {}", text));
            }

            // Parse the token JSON
            let token: serde_json::Value = match serde_json::from_str(&text) {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Failed to parse Keycloak token: {}", e);
                    return HttpResponse::InternalServerError()
                        .body("Failed to parse token from Keycloak");
                }
            };

            HttpResponse::Ok().json(token)
        }
        Err(err) => {
            println!("❌ Error communicating with Keycloak: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Authentication backend running on http://localhost:3000");

    HttpServer::new(|| {
        App::new()
            .service(authentication)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
