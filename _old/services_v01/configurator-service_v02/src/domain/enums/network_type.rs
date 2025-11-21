use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NetworkType {
    INDIVIDUAL,
    COMPANY,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self).map(|_| ())
    }
}

impl FromStr for NetworkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INDIVIDUAL" => Ok(NetworkType::INDIVIDUAL),
            "COMPANY" => Ok(NetworkType::COMPANY),
            _ => Err("VariantNotFound"),
        }
    }
}
