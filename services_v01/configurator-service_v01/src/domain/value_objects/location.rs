use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocationError {
    #[error("Invalid latitude: {0} (must be -90 to 90)")]
    InvalidLatitude(f64),
    #[error("Invalid longitude: {0} (must be -180 to 180)")]
    InvalidLongitude(f64),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl Location {
    const MIN_LAT: f64 = -90.0;
    const MAX_LAT: f64 = 90.0;
    const MIN_LON: f64 = -180.0;
    const MAX_LON: f64 = 180.0;

    pub fn new(lat: f64, lon: f64) -> Result<Self, LocationError> {
        if !(Self::MIN_LAT..=Self::MAX_LAT).contains(&lat) {
            return Err(LocationError::InvalidLatitude(lat));
        }
        if !(Self::MIN_LON..=Self::MAX_LON).contains(&lon) {
            return Err(LocationError::InvalidLongitude(lon));
        }
        Ok(Self {
            latitude: lat,
            longitude: lon,
        })
    }

    pub fn lat(&self) -> f64 {
        self.latitude
    }
    pub fn lon(&self) -> f64 {
        self.longitude
    }

    pub fn distance_to(&self, other: &Location) -> f64 {
        let r = 6371.0; // Earth radius in km
        let d_lat = (other.lat() - self.lat()).to_radians();
        let d_lon = (other.lon() - self.lon()).to_radians();
        let a = (d_lat / 2.0).sin().powi(2)
            + self.lat().to_radians().cos()
                * other.lat().to_radians().cos()
                * (d_lon / 2.0).sin().powi(2);
        r * 2.0 * a.sqrt().atan2((1.0 - a).sqrt())
    }

    pub fn is_within_radius(&self, other: &Location, radius_km: f64) -> bool {
        self.distance_to(other) <= radius_km
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6},{:.6}", self.latitude, self.longitude)
    }
}

impl FromStr for Location {
    type Err = LocationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(LocationError::InvalidFormat(s.into()));
        }
        let lat = parts[0]
            .parse()
            .map_err(|_| LocationError::InvalidFormat(s.into()))?;
        let lon = parts[1]
            .parse()
            .map_err(|_| LocationError::InvalidFormat(s.into()))?;
        Self::new(lat, lon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_locations() {
        let locations = [
            (40.7128, -74.0060), // NYC
            (51.5074, -0.1278),  // London
            (0.0, 0.0),          // Null Island
        ];

        for (lat, lon) in locations {
            assert!(Location::new(lat, lon).is_ok());
        }
    }

    #[test]
    fn test_invalid_locations() {
        assert!(Location::new(91.0, 0.0).is_err());
        assert!(Location::new(0.0, 181.0).is_err());
    }

    #[test]
    fn test_distance() {
        let ny = Location::new(40.7128, -74.0060).unwrap();
        let la = Location::new(34.0522, -118.2437).unwrap();
        let distance = ny.distance_to(&la);
        assert!(distance > 3900.0 && distance < 4000.0);
    }

    #[test]
    fn test_parsing() {
        let loc: Location = "40.7128,-74.0060".parse().unwrap();
        assert_eq!(loc.lat(), 40.7128);
        assert_eq!(loc.lon(), -74.0060);
    }

    #[test]
    fn test_display() {
        let loc = Location::new(40.7128, -74.0060).unwrap();
        assert_eq!(loc.to_string(), "40.712800,-74.006000");
    }
}
