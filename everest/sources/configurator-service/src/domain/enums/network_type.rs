use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

/// Represents the type of network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NetworkType {
    #[default]
    Individual,
    Company,
}

impl FromStr for NetworkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INDIVIDUAL" => Ok(NetworkType::Individual),
            "COMPANY" => Ok(NetworkType::Company),
            _ => Err(format!("Invalid network type: {}", s)),
        }
    }
}

impl ToString for NetworkType {
    fn to_string(&self) -> String {
        match self {
            NetworkType::Individual => "INDIVIDUAL".into(),
            NetworkType::Company => "COMPANY".into(),
        }
    }
}

impl NetworkType {
    pub fn is_individual(&self) -> bool {
        matches!(self, NetworkType::Individual)
    }

    pub fn is_company(&self) -> bool {
        matches!(self, NetworkType::Company)
    }
}
