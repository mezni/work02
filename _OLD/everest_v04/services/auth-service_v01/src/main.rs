mod config;
mod domain;
mod application;
mod infrastructure;
mod interfaces;

use actix_web::{App, HttpServer};
use interfaces::http::routes::users_routes;
use config::Config;
use infrastructure::repository_impl::UserRepositoryKeycloak;
use infrastructure::keycloak::KeycloakClient;
use application::services::UserService;
use utoipa_swagger_ui::SwaggerUi;
use interfaces::http::swagger::ApiDoc;
use utoipa::OpenApi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_env();

    // Keycloak client & repository
    let kc_client = KeycloakClient::new(
        &cfg.keycloak_url,
        &cfg.keycloak_realm,
        &cfg.keycloak_admin,
        &cfg.keycloak_admin_password,
    );
    let repo = UserRepositoryKeycloak { client: kc_client };
    let user_service = UserService::new(repo);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(user_service.clone()))
            .configure(users_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((cfg.server_host, cfg.server_port))?
    .run()
    .await
}
