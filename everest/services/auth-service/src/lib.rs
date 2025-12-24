use actix_web::{App, HttpServer, middleware::Logger, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod presentation;

use crate::core::{config::Config, database};
use crate::presentation::controllers::health_controller;
use crate::presentation::openapi::ApiDoc;

pub struct AppState {
    pub config: Config,
    pub db_pool: database::DbPool,
}

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env();

    // Initialize Database
    let db_pool = database::create_pool(&config.database_url).await?;

    let app_state = web::Data::new(AppState {
        config: config.clone(),
        db_pool,
    });

    let server_addr = config.server_addr.clone();
    let openapi = ApiDoc::openapi();

    tracing::info!("🚀 Starting server on {}", server_addr);
    tracing::info!(
        "📑 Swagger UI available at http://{}/swagger-ui/",
        server_addr
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // API Routes
            .service(web::scope("/api/v1").service(health_controller::health_check))
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
