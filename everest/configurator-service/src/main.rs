use actix_web::{App, HttpServer, middleware::Logger};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod interfaces;

use interfaces::http::health::health_checker_handler;
use interfaces::http::openapi::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Read PORT (defaults to 3000 if missing)
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Initialize logger using whatever RUST_LOG value exists in the environment
    env_logger::init();

    println!("Server running at http://localhost:{port}");

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_checker_handler)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
