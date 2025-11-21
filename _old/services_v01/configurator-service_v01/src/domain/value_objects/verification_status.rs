use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationStatusError {
    #[error("Invalid verification status: {0}")]
    InvalidStatus(String),
    #[error("Cannot transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
}

impl VerificationStatus {
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified)
    }

    pub fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected)
    }

    pub fn can_be_verified(&self) -> bool {
        matches!(self, Self::Pending)
    }

    pub fn can_be_rejected(&self) -> bool {
        matches!(self, Self::Pending)
    }

    pub fn can_be_reset(&self) -> bool {
        matches!(self, Self::Verified | Self::Rejected)
    }

    pub fn all() -> [Self; 3] {
        [Self::Pending, Self::Verified, Self::Rejected]
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pending => "PENDING",
            Self::Verified => "VERIFIED",
            Self::Rejected => "REJECTED",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for VerificationStatus {
    type Err = VerificationStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(Self::Pending),
            "VERIFIED" => Ok(Self::Verified),
            "REJECTED" => Ok(Self::Rejected),
            _ => Err(VerificationStatusError::InvalidStatus(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_status_properties() {
        let pending = VerificationStatus::Pending;
        let verified = VerificationStatus::Verified;
        let rejected = VerificationStatus::Rejected;

        assert!(pending.is_pending());
        assert!(pending.can_be_verified());
        assert!(pending.can_be_rejected());
        assert!(!pending.can_be_reset());

        assert!(verified.is_verified());
        assert!(!verified.can_be_verified());
        assert!(!verified.can_be_rejected());
        assert!(verified.can_be_reset());

        assert!(rejected.is_rejected());
        assert!(!rejected.can_be_verified());
        assert!(!rejected.can_be_rejected());
        assert!(rejected.can_be_reset());
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            "PENDING".parse::<VerificationStatus>().unwrap(),
            VerificationStatus::Pending
        );
        assert_eq!(
            "VERIFIED".parse::<VerificationStatus>().unwrap(),
            VerificationStatus::Verified
        );
        assert_eq!(
            "REJECTED".parse::<VerificationStatus>().unwrap(),
            VerificationStatus::Rejected
        );
        assert!("INVALID".parse::<VerificationStatus>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(VerificationStatus::Pending.to_string(), "PENDING");
        assert_eq!(VerificationStatus::Verified.to_string(), "VERIFIED");
        assert_eq!(VerificationStatus::Rejected.to_string(), "REJECTED");
    }

    #[test]
    fn test_serialization() {
        let status = VerificationStatus::Pending;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: VerificationStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_all_statuses() {
        let all = VerificationStatus::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&VerificationStatus::Pending));
        assert!(all.contains(&VerificationStatus::Verified));
        assert!(all.contains(&VerificationStatus::Rejected));
    }
}
