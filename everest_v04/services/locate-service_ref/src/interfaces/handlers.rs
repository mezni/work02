use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validate;

use crate::application::dto::*;
use crate::application::{StationService, ReviewService};
use crate::config::Config;
use crate::domain::{NearbyStation, UserReview};
use crate::infrastructure::{DomainError, PostgresStationRepository, PostgresReviewRepository};
use crate::middleware::Claims;

/// Health check
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Healthy", body = HealthCheckResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Find nearby stations (PUBLIC - no auth required)
#[utoipa::path(
    get,
    path = "/api/v1/stations/nearby",
    params(
        ("latitude" = f64, Query, description = "Latitude"),
        ("longitude" = f64, Query, description = "Longitude"),
        ("radius_meters" = Option<i32>, Query, description = "Search radius in meters (default: 5000)"),
        ("limit" = Option<i32>, Query, description = "Max results (default: 50)")
    ),
    responses(
        (status = 200, description = "Nearby stations", body = Vec<NearbyStation>),
        (status = 400, description = "Invalid parameters")
    ),
    tag = "Stations"
)]
pub async fn find_nearby_stations(
    pool: web::Data<PgPool>,
    query: web::Query<NearbyStationsQuery>,
) -> Result<impl Responder, DomainError> {
    let station_repo = Arc::new(PostgresStationRepository::new(pool.get_ref().clone()));
    let service = StationService::new(station_repo);

    let stations = service
        .find_nearby_stations(
            query.latitude,
            query.longitude,
            query.radius_meters,
            query.limit,
        )
        .await?;

    Ok(HttpResponse::Ok().json(stations))
}

/// Create review (requires authentication)
#[utoipa::path(
    post,
    path = "/api/v1/reviews",
    request_body = CreateReviewRequest,
    responses(
        (status = 201, description = "Review created", body = UserReview),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reviews",
    security(("bearer_auth" = []))
)]
pub async fn create_review(
    pool: web::Data<PgPool>,
    request: web::Json<CreateReviewRequest>,
    claims: Claims,
) -> Result<impl Responder, DomainError> {
    request.validate().map_err(|e| {
        DomainError::ValidationError(format!("Validation failed: {}", e))
    })?;

    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo);

    let review = service.create_review(request.into_inner(), &claims).await?;

    Ok(HttpResponse::Created().json(review))
}

/// Get reviews for a station (PUBLIC)
#[utoipa::path(
    get,
    path = "/api/v1/stations/{station_id}/reviews",
    params(
        ("station_id" = String, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "Station reviews", body = Vec<UserReview>)
    ),
    tag = "Reviews"
)]
pub async fn get_station_reviews(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<impl Responder, DomainError> {
    let station_id = path.into_inner();

    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo);

    let reviews = service.get_station_reviews(&station_id).await?;

    Ok(HttpResponse::Ok().json(reviews))
}

/// Get user's reviews (requires authentication)
#[utoipa::path(
    get,
    path = "/api/v1/reviews/my",
    responses(
        (status = 200, description = "User reviews", body = Vec<UserReview>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reviews",
    security(("bearer_auth" = []))
)]
pub async fn get_my_reviews(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<impl Responder, DomainError> {
    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo);

    let reviews = service.get_user_reviews(claims.get_user_id()).await?;

    Ok(HttpResponse::Ok().json(reviews))
}

/// Update review (requires authentication)
#[utoipa::path(
    put,
    path = "/api/v1/reviews/{review_id}",
    params(
        ("review_id" = String, Path, description = "Review ID")
    ),
    request_body = UpdateReviewRequest,
    responses(
        (status = 200, description = "Review updated", body = UserReview),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reviews",
    security(("bearer_auth" = []))
)]
pub async fn update_review(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    request: web::Json<UpdateReviewRequest>,
    claims: Claims,
) -> Result<impl Responder, DomainError> {
    let review_id = path.into_inner();

    request.validate().map_err(|e| {
        DomainError::ValidationError(format!("Validation failed: {}", e))
    })?;

    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo);

    let review = service.update_review(&review_id, request.into_inner(), &claims).await?;

    Ok(HttpResponse::Ok().json(review))
}

/// Delete review (requires authentication)
#[utoipa::path(
    delete,
    path = "/api/v1/reviews/{review_id}",
    params(
        ("review_id" = String, Path, description = "Review ID")
    ),
    responses(
        (status = 204, description = "Review deleted"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reviews",
    security(("bearer_auth" = []))
)]
pub async fn delete_review(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    claims: Claims,
) -> Result<impl Responder, DomainError> {
    let review_id = path.into_inner();

    let review_repo = Arc::new(PostgresReviewRepository::new(pool.get_ref().clone()));
    let service = ReviewService::new(review_repo);

    service.delete_review(&review_id, &claims).await?;

    Ok(HttpResponse::NoContent().finish())
}