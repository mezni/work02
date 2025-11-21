use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocationError {
    #[error("Latitude must be between -90.0 and 90.0")]
    InvalidLatitude,
    #[error("Longitude must be between -180.0 and 180.0")]
    InvalidLongitude,
}

/// Geographic location value object.
/// Latitude and longitude are required; address fields optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

impl Location {
    /// Constructor with validation.
    pub fn new(
        latitude: f64,
        longitude: f64,
        address_line1: Option<String>,
        address_line2: Option<String>,
        city: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
    ) -> Result<Self, LocationError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(LocationError::InvalidLatitude);
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(LocationError::InvalidLongitude);
        }

        Ok(Self {
            latitude,
            longitude,
            address_line1,
            address_line2,
            city,
            state,
            postal_code,
            country,
        })
    }

    /// Update latitude immutably
    pub fn with_latitude(&self, lat: f64) -> Result<Self, LocationError> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(LocationError::InvalidLatitude);
        }

        let mut new = self.clone();
        new.latitude = lat;
        Ok(new)
    }

    /// Update longitude immutably
    pub fn with_longitude(&self, lon: f64) -> Result<Self, LocationError> {
        if !(-180.0..=180.0).contains(&lon) {
            return Err(LocationError::InvalidLongitude);
        }

        let mut new = self.clone();
        new.longitude = lon;
        Ok(new)
    }
}
