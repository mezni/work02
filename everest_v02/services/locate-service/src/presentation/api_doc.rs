use utoipa::OpenApi;
use crate::application::dto::{CreateReviewRequest, HealthCheckResponse};
use crate::domain::{Station, StationReview, StationWithReviews};
use crate::presentation::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::get_stations,
        handlers::get_station_by_id,
        handlers::get_station_with_reviews,
        handlers::get_station_reviews,
        handlers::create_review,
    ),
    components(
        schemas(
            Station,
            StationReview,
            StationWithReviews,
            CreateReviewRequest,
            HealthCheckResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Stations", description = "EV charging station management"),
        (name = "Reviews", description = "Station review management")
    ),
    info(
        title = "EV Station Service API",
        version = "1.0.0",
        description = "REST API for managing EV charging stations and reviews",
        contact(
            name = "API Support",
            email = "support@evstation.com"
        )
    )
)]
pub struct ApiDoc;