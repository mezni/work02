pub mod application;
pub mod core;
pub mod interfaces;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::core::config::Config;
use crate::core::database::create_pool;
use crate::interfaces::http::routes::configure_routes;
use crate::interfaces::http::swagger::ApiDoc;

/// Global Application State
pub struct AppState {
    pub db: sqlx::PgPool,
    pub env: Config,
}

pub async fn run() -> anyhow::Result<()> {
    // 1. Load config and DB
    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await?;

    // 2. Wrap state in web::Data (Arc)
    let app_state = web::Data::new(AppState {
        db: pool,
        env: config.clone(),
    });

    let server_addr = config.server_addr.clone();

    tracing::info!("Starting server on http://{}", server_addr);
    tracing::info!("Swagger UI: http://{}/swagger-ui/", server_addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // State MUST be registered before routes/middleware that use it
            .app_data(app_state.clone())
            // Middlewares
            .wrap(TracingLogger::default())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            // 3. Swagger UI (Register at root to avoid prefix conflicts)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // 4. Configure Application Routes (uses API_PREFIX internally)
            .configure(configure_routes)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
