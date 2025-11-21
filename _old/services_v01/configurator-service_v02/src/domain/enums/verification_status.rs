use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VerificationStatus {
    PENDING,
    VERIFIED,
    REJECTED,
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for VerificationStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(VerificationStatus::PENDING),
            "VERIFIED" => Ok(VerificationStatus::VERIFIED),
            "REJECTED" => Ok(VerificationStatus::REJECTED),
            _ => Err("VariantNotFound"),
        }
    }
}
