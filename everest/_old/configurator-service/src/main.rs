mod errors;
mod api;
mod domain;
mod repository;
mod service;

use actix_web::{App, HttpServer, middleware::Logger, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{organization_api, user_api, station_api};
use crate::service::{OrganizationService, UserService, StationService};

#[derive(OpenApi)]
#[openapi(
    paths(
        organization_api::create_org,
        user_api::create_user,
        station_api::create_station
    ),
    components(
        schemas(
            crate::domain::organization::Organization,
            crate::domain::user::User,
            crate::domain::user::Role,
            crate::domain::station::Station,
            crate::api::organization_api::CreateOrgRequest,
            crate::api::user_api::CreateUserRequest,
            crate::api::station_api::CreateStationRequest
        )
    ),
    tags(
        (name = "Organization", description = "Organization endpoints"),
        (name = "User", description = "User endpoints"),
        (name = "Station", description = "Station endpoints")
    )
)]
pub struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let org_service = OrganizationService::new();
    let user_service = UserService::new();
    let station_service = StationService::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(org_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(station_service.clone()))
            .service(
                web::scope("/org").route("/create", web::post().to(organization_api::create_org))
            )
            .service(
                web::scope("/user").route("/create", web::post().to(user_api::create_user))
            )
            .service(
                web::scope("/station").route("/create", web::post().to(station_api::create_station))
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
