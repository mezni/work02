use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;
use dotenvy::dotenv;
use std::env;

mod interfaces;
mod infrastructure;
mod application;
mod domain;

use infrastructure::keycloak_client::KeycloakClient;
use infrastructure::user_repository::KeycloakUserRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env
    dotenv().ok();

    env_logger::init();

    // Read config from environment variables
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("SERVER_PORT must be a number");

    let keycloak_url = env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
    let keycloak_realm = env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set");
    let keycloak_client_id = env::var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set");
    let keycloak_client_secret = env::var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set");
    let keycloak_admin_token = env::var("KEYCLOAK_ADMIN_TOKEN").expect("KEYCLOAK_ADMIN_TOKEN must be set");

    // Initialize Keycloak client
    let keycloak_client = KeycloakClient::new(
        keycloak_url,
        keycloak_realm,
        keycloak_client_id,
        keycloak_client_secret,
        keycloak_admin_token,
    );

    // Wrap Keycloak repository in Arc for Actix Data
    let user_repo = Arc::new(KeycloakUserRepository { client: keycloak_client });

    // OpenAPI docs
    let openapi = interfaces::swagger::ApiDoc::openapi();

    println!("Server running at http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::from(user_repo.clone()))
            // Handlers
            .service(interfaces::http::health_handler::health_handler)
            .service(interfaces::http::register_handler::register_handler::<KeycloakUserRepository>)
            .service(interfaces::http::login_handler::login_handler)
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
