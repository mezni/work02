mod domain;
mod application;
mod infrastructure;
mod api;

use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::infrastructure::keycloak::client::KeycloakAuthClient;
use crate::application::services::user_app_service::UserAppService;
use crate::api::handlers::user_handler::create_user;

// Manual OpenAPI configuration
#[derive(OpenApi)]
#[openapi(
    paths(
        api::handlers::user_handler::create_user,
    ),
    components(
        schemas(
            crate::application::dtos::requests::CreateUserRequest,
            crate::domain::models::user::User
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let keycloak_client = KeycloakAuthClient::new(
        "http://localhost:5600".to_string(),
        "myrealm".to_string(),
        "admin".to_string(),
        "admin".to_string(),
    );
    
    let user_service = UserAppService::new(keycloak_client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(create_user))
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
            )
    })
    .bind("127.0.0.1:3200")?
    .run()
    .await
}