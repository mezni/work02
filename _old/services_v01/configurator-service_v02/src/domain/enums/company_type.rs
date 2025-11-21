use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CompanyType {
    COMPANY,
    COOPERATIVE,
    GOVERNMENT,
}

impl fmt::Display for CompanyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for CompanyType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "COMPANY" => Ok(CompanyType::COMPANY),
            "COOPERATIVE" => Ok(CompanyType::COOPERATIVE),
            "GOVERNMENT" => Ok(CompanyType::GOVERNMENT),
            _ => Err("VariantNotFound"),
        }
    }
}
