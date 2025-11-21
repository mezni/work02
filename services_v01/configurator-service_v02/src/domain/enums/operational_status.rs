use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OperationalStatus {
    ACTIVE,
    MAINTENANCE,
    OUT_OF_SERVICE,
    COMMISSIONING,
}

impl fmt::Display for OperationalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for OperationalStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACTIVE" => Ok(OperationalStatus::ACTIVE),
            "MAINTENANCE" => Ok(OperationalStatus::MAINTENANCE),
            "OUT_OF_SERVICE" => Ok(OperationalStatus::OUT_OF_SERVICE),
            "COMMISSIONING" => Ok(OperationalStatus::COMMISSIONING),
            _ => Err("VariantNotFound"),
        }
    }
}
