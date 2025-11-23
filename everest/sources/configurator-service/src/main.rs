use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use configurator_service::{
    api::{ApiDoc, config_network_routes},
    application::NetworkApplicationService,
    infrastructure::{PostgresNetworkRepository, create_pool_from_env},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    // Create database pool
    let pool = create_pool_from_env()
        .await
        .expect("Failed to create database pool");

    // Create repository and service
    let repository = PostgresNetworkRepository::new(pool);
    let service = NetworkApplicationService::new(repository);

    // Wrap service in web::Data for sharing across threads
    let service_data = web::Data::new(service);

    // OpenAPI documentation
    let openapi = ApiDoc::openapi();

    println!("Starting server at http://localhost:3000");
    println!("Swagger UI available at http://localhost:3000/swagger-ui/");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(service_data.clone())
            .wrap(cors)
            .configure(config_network_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
