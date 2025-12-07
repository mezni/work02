use std::sync::Arc;
use crate::domain::{NearbyStation, StationRepository};
use crate::infrastructure::error::DomainError;

pub struct StationService {
    station_repo: Arc<dyn StationRepository>,
}

impl StationService {
    pub fn new(station_repo: Arc<dyn StationRepository>) -> Self {
        Self { station_repo }
    }

    pub async fn find_nearby_stations(
        &self,
        latitude: f64,
        longitude: f64,
        radius_meters: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<NearbyStation>, DomainError> {
        self.station_repo
            .find_nearby(
                latitude,
                longitude,
                radius_meters.unwrap_or(5000),
                limit.unwrap_or(50),
            )
            .await
    }
}