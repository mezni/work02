use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperationalStatusError {
    #[error("Invalid operational status: {0}")]
    InvalidStatus(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationalStatus {
    Active,
    Maintenance,
    OutOfService,
    Commissioning,
}

impl OperationalStatus {
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Active | Self::Commissioning)
    }

    pub fn is_available(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn requires_maintenance(&self) -> bool {
        matches!(self, Self::Maintenance)
    }

    pub fn is_offline(&self) -> bool {
        matches!(self, Self::OutOfService)
    }

    pub fn can_accept_charges(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn can_be_configured(&self) -> bool {
        matches!(self, Self::Commissioning | Self::OutOfService)
    }

    pub fn all() -> [Self; 4] {
        [
            Self::Active,
            Self::Maintenance,
            Self::OutOfService,
            Self::Commissioning,
        ]
    }
}

impl fmt::Display for OperationalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Active => "ACTIVE",
            Self::Maintenance => "MAINTENANCE",
            Self::OutOfService => "OUT_OF_SERVICE",
            Self::Commissioning => "COMMISSIONING",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for OperationalStatus {
    type Err = OperationalStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACTIVE" => Ok(Self::Active),
            "MAINTENANCE" => Ok(Self::Maintenance),
            "OUT_OF_SERVICE" | "OUT-OF-SERVICE" => Ok(Self::OutOfService),
            "COMMISSIONING" => Ok(Self::Commissioning),
            _ => Err(OperationalStatusError::InvalidStatus(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operational_status_properties() {
        assert!(OperationalStatus::Active.is_operational());
        assert!(OperationalStatus::Active.is_available());
        assert!(OperationalStatus::Active.can_accept_charges());

        assert!(OperationalStatus::Commissioning.is_operational());
        assert!(!OperationalStatus::Commissioning.is_available());
        assert!(!OperationalStatus::Commissioning.can_accept_charges());
        assert!(OperationalStatus::Commissioning.can_be_configured());

        assert!(!OperationalStatus::Maintenance.is_operational());
        assert!(OperationalStatus::Maintenance.requires_maintenance());
        assert!(!OperationalStatus::Maintenance.can_accept_charges());

        assert!(!OperationalStatus::OutOfService.is_operational());
        assert!(OperationalStatus::OutOfService.is_offline());
        assert!(OperationalStatus::OutOfService.can_be_configured());
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            "ACTIVE".parse::<OperationalStatus>().unwrap(),
            OperationalStatus::Active
        );
        assert_eq!(
            "MAINTENANCE".parse::<OperationalStatus>().unwrap(),
            OperationalStatus::Maintenance
        );
        assert_eq!(
            "OUT_OF_SERVICE".parse::<OperationalStatus>().unwrap(),
            OperationalStatus::OutOfService
        );
        assert_eq!(
            "OUT-OF-SERVICE".parse::<OperationalStatus>().unwrap(),
            OperationalStatus::OutOfService
        );
        assert_eq!(
            "COMMISSIONING".parse::<OperationalStatus>().unwrap(),
            OperationalStatus::Commissioning
        );
        assert!("INVALID".parse::<OperationalStatus>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(OperationalStatus::Active.to_string(), "ACTIVE");
        assert_eq!(OperationalStatus::Maintenance.to_string(), "MAINTENANCE");
        assert_eq!(
            OperationalStatus::OutOfService.to_string(),
            "OUT_OF_SERVICE"
        );
        assert_eq!(
            OperationalStatus::Commissioning.to_string(),
            "COMMISSIONING"
        );
    }

    #[test]
    fn test_serialization() {
        let status = OperationalStatus::Active;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: OperationalStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_all_statuses() {
        let all = OperationalStatus::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&OperationalStatus::Active));
        assert!(all.contains(&OperationalStatus::Maintenance));
        assert!(all.contains(&OperationalStatus::OutOfService));
        assert!(all.contains(&OperationalStatus::Commissioning));
    }
}
