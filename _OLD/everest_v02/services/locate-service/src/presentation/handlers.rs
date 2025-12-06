use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;

use crate::application::{
    dto::{CreateReviewRequest, HealthCheckResponse},
    StationService, ReviewService,
};
use crate::domain::{Station, StationReview, StationWithReviews};
use crate::infrastructure::{PostgresStationRepository, PostgresReviewRepository, DomainError};
use std::sync::Arc;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthCheckResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Get all stations
#[utoipa::path(
    get,
    path = "/api/v1/stations",
    params(
        ("city" = Option<String>, Query, description = "Filter stations by city")
    ),
    responses(
        (status = 200, description = "List of stations", body = Vec<Station>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Stations"
)]
pub async fn get_stations(
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<impl Responder, DomainError> {
    let station_repo = Arc::new(PostgresStationRepository::new(pool.get_ref().clone()));
    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = StationService::new(station_repo, review_repo);

    let stations = if let Some(city) = query.get("city") {
        service.get_stations_by_city(city).await?
    } else {
        service.get_all_stations().await?
    };

    Ok(HttpResponse::Ok().json(stations))
}

/// Get station by ID
#[utoipa::path(
    get,
    path = "/api/v1/stations/{station_id}",
    params(
        ("station_id" = i32, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "Station details", body = Station),
        (status = 404, description = "Station not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Stations"
)]
pub async fn get_station_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, DomainError> {
    let station_id = path.into_inner();
    let station_repo = Arc::new(PostgresStationRepository::new(pool.get_ref().clone()));
    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = StationService::new(station_repo, review_repo);

    let station = service.get_station_by_id(station_id).await?;
    Ok(HttpResponse::Ok().json(station))
}

/// Get station with reviews
#[utoipa::path(
    get,
    path = "/api/v1/stations/{station_id}/details",
    params(
        ("station_id" = i32, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "Station with reviews and average rating", body = StationWithReviews),
        (status = 404, description = "Station not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Stations"
)]
pub async fn get_station_with_reviews(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, DomainError> {
    let station_id = path.into_inner();
    let station_repo = Arc::new(PostgresStationRepository::new(pool.get_ref().clone()));
    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = StationService::new(station_repo, review_repo);

    let result = service.get_station_with_reviews(station_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get reviews for a station
#[utoipa::path(
    get,
    path = "/api/v1/stations/{station_id}/reviews",
    params(
        ("station_id" = i32, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "List of reviews", body = Vec<StationReview>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Reviews"
)]
pub async fn get_station_reviews(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, DomainError> {
    let station_id = path.into_inner();
    let station_repo = Arc::new(PostgresStationRepository::new(pool.get_ref().clone()));
    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo, station_repo);

    let reviews = service.get_reviews_for_station(station_id).await?;
    Ok(HttpResponse::Ok().json(reviews))
}

/// Create a review for a station
#[utoipa::path(
    post,
    path = "/api/v1/stations/{station_id}/reviews",
    params(
        ("station_id" = i32, Path, description = "Station ID")
    ),
    request_body = CreateReviewRequest,
    responses(
        (status = 201, description = "Review created successfully", body = StationReview),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Station not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Reviews"
)]
pub async fn create_review(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    request: web::Json<CreateReviewRequest>,
) -> Result<impl Responder, DomainError> {
    let station_id = path.into_inner();
    
    // Validate request
    request.validate().map_err(|e| {
        DomainError::ValidationError(format!("Validation failed: {}", e))
    })?;

    let station_repo = Arc::new(PostgresStationRepository::new(pool.get_ref().clone()));
    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo, station_repo);

    let review = service.create_review(station_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(review))
}