use actix_web::{web, App, HttpServer, HttpResponse};
use std::net::SocketAddr;
use tracing::{info, error};
use utoipa_swagger_ui::SwaggerUi;
use anyhow::Context;

use crate::{
    infrastructure::{
        config::AppConfig, 
        ioc::ServiceLocator,
        logging::setup_tracing,
    },
    interfaces::http::{
        configure,
        auth_controller::ApiDoc,
        middleware::AuthGuard,
    },
};

pub async fn run() -> anyhow::Result<()> {
    // 1. Setup tracing
    setup_tracing();

    // 2. Load configuration
    let config = AppConfig::load().context("Failed to load application configuration")?;
    let listen_addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .context("Failed to parse host:port address")?;
    
    info!("Configuration loaded. Starting server on {}", listen_addr);

    // 3. Initialize Service Locator (Dependency Injection)
    let locator = ServiceLocator::new(config.clone()).await
        .context("Failed to initialize service locator")?;
    let locator_data = web::Data::new(locator);

    // 4. OpenAPI Documentation
    let api_doc = ApiDoc::openapi();

    // 5. Create database tables (in a real app, you'd use migrations)
    // For now, we'll just log that we're starting
    info!("Database connection pool created successfully.");

    // 6. Start HTTP server
    info!("Starting Actix-Web server on {}...", listen_addr);
    
    HttpServer::new(move || {
        App::new()
            // Dependency Injection
            .app_data(locator_data.clone())
            
            // Middleware
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(AuthGuard)
            
            // Health check
            .route("/health", web::get().to(|| async { 
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "ok",
                    "service": "auth-service"
                }))
            }))
            
            // Application routes
            .configure(configure)

            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", api_doc.clone()),
            )
    })
    .bind(listen_addr)
    .context("Failed to bind server address")?
    .run()
    .await
    .context("Actix-Web server failed to run")?;

    Ok(())
}
