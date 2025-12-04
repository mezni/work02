use actix_web::{middleware::Logger, App, HttpServer, web};
use env_logger;
use dotenvy::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod domain;
mod infrastructure;
mod application;
mod interfaces;

use interfaces::swagger::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("SERVER_PORT").unwrap_or("8000".to_string()).parse::<u16>().unwrap();

    let keycloak_client = infrastructure::keycloak_client::KeycloakClient::new(
        env::var("KEYCLOAK_URL").unwrap(),
        env::var("KEYCLOAK_REALM").unwrap(),
        env::var("KEYCLOAK_CLIENT_ID").unwrap(),
        env::var("KEYCLOAK_CLIENT_SECRET").unwrap(),
    );

    let user_repo = web::Data::new(infrastructure::user_repository::KeycloakUserRepository {
        client: keycloak_client,
    });

    let openapi = ApiDoc::openapi();

    println!("Server running at http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(user_repo.clone())
            .service(interfaces::http::health_handler::health_handler)
            .service(interfaces::http::register_handler::register_handler)
            .service(interfaces::http::login_handler::login_handler)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
