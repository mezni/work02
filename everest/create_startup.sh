#!/bin/bash
# create_startup_layer_with_stubs.sh
# Creates the Startup layer for Auth-Service with Actix-Web server setup
SERVICE_DIR="auth-service"

echo "Creating Startup layer structure with starter code..."

cd $SERVICE_DIR

# Create directory
mkdir -p src/startup

# main startup.rs file
cat > src/startup.rs <<EOL
use actix_web::{App, HttpServer, middleware};
use actix_web::web;
use crate::interfaces::http::routes;
use crate::interfaces::http::middleware::LoggingMiddleware;
use crate::infrastructure::logging;
use crate::infrastructure::config::Settings;
use utoipa_swagger_ui::SwaggerUi;
use crate::interfaces::http::swagger::ApiDoc;

pub async fn run() -> std::io::Result<()> {
    // Initialize logging
    logging::init();

    // Load settings
    let settings = Settings::new();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(LoggingMiddleware)
            // Routes
            .configure(routes::configure)
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
            )
            // App data
            .app_data(web::Data::new(settings.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
EOL

echo "✅ Startup layer created successfully!"
echo "File created:"
echo "- src/startup.rs"
