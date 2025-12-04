mod keycloak_client;
mod user_dto;
mod user_entity;
mod user_handler;
mod user_repository;
mod user_service;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use keycloak_client::{KeycloakClient, KeycloakConfig};
use user_handler::{authenticate_handler, create_user_handler, UserHandler};
use user_repository::UserRepository;
use user_service::UserService;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "keycloak-auth"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .unwrap_or(8081);

    // Initialize Keycloak configuration
    let keycloak_config = match KeycloakConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!("Please ensure all required environment variables are set:");
            eprintln!("  - KEYCLOAK_URL");
            eprintln!("  - KEYCLOAK_REALM");
            eprintln!("  - KEYCLOAK_ADMIN_CLIENT_ID");
            eprintln!("  - KEYCLOAK_ADMIN_CLIENT_SECRET");
            eprintln!("  - KEYCLOAK_AUTH_CLIENT_ID");
            std::process::exit(1);
        }
    };

    println!("=================================");
    println!("Starting Keycloak Auth Service...");
    println!("=================================");
    println!("Server URL: http://127.0.0.1:{}", port);
    println!("Keycloak URL: {}", keycloak_config.url);
    println!("Realm: {}", keycloak_config.realm);
    println!("Default role: user");
    println!("=================================");

    // Setup layers
    let keycloak_client = KeycloakClient::new(keycloak_config);
    let user_repository = UserRepository::new(keycloak_client);
    let user_service = UserService::new(user_repository);
    let user_handler = web::Data::new(UserHandler::new(user_service));

    let http_client = web::Data::new(reqwest::Client::new());

    HttpServer::new(move || {
        App::new()
            .app_data(user_handler.clone())
            .app_data(http_client.clone())
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/api/v1/register").route(web::post().to(create_user_handler)))
            .service(web::resource("/api/v1/auth").route(web::post().to(authenticate_handler)))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}