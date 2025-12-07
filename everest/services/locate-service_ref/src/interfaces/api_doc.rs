use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::handlers::health_check,
        crate::interfaces::handlers::find_nearby_stations,
        crate::interfaces::handlers::create_review,
        crate::interfaces::handlers::get_station_reviews,
        crate::interfaces::handlers::get_my_reviews,
        crate::interfaces::handlers::update_review,
        crate::interfaces::handlers::delete_review,
    ),
    components(
        schemas(
            crate::domain::NearbyStation,
            crate::domain::UserReview,
            crate::application::dto::NearbyStationsQuery,
            crate::application::dto::CreateReviewRequest,
            crate::application::dto::UpdateReviewRequest,
            crate::application::dto::HealthCheckResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check"),
        (name = "Stations", description = "Find nearby stations (public)"),
        (name = "Reviews", description = "User reviews (auth required)")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}