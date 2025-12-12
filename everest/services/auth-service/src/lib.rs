use actix_web::{web, App, HttpServer};
use anyhow::Result;
//use utoipa_swagger_ui::SwaggerUi;

pub mod core;

pub use core::{
//    api_doc::ApiDoc,
    config::{Config, KeycloakConfig},
    database::DbPool,
//    errors::{AppError, Result as AppResult},
    logging,
//    middleware::RequestLogger,
//    server::configure_routes,
//    telemetry,
//    utils,
};

pub async fn run() -> Result<()> {
    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize logging
    logging::init(&config.rust_log)?;
    
    // Initialize telemetry
//    telemetry::init("rust-api")?;
    
    tracing::info!(
        "Starting server on {}",
        config.server_address()
    );
    tracing::info!(
        "Keycloak configured at {} (realm: {})",
        config.keycloak.url,
        config.keycloak.realm
    );
    
    // Create database pool
    let pool = DbPool::new(&config.database_url).await?;
        
    let server_addr = config.server_address();
/*     
    // Start HTTP server
    HttpServer::new(move || {
        let openapi = ApiDoc::openapi();
        
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_cors::Cors::permissive())
            .wrap(RequestLogger)
            .configure(configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(&server_addr)?
    .run()
    .await?;
*/    
    Ok(())
}