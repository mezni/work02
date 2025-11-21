use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RoleType {
    ADMIN,
    BILLING,
    TECHNICAL,
    OPERATIONS,
    GENERAL,
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for RoleType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADMIN" => Ok(RoleType::ADMIN),
            "BILLING" => Ok(RoleType::BILLING),
            "TECHNICAL" => Ok(RoleType::TECHNICAL),
            "OPERATIONS" => Ok(RoleType::OPERATIONS),
            "GENERAL" => Ok(RoleType::GENERAL),
            _ => Err("VariantNotFound"),
        }
    }
}
