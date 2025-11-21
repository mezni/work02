use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

use super::company_type::CompanyType; // Add this import

#[derive(Debug, Error)]
pub enum NetworkTypeError {
    #[error("Invalid network type: {0}")]
    InvalidType(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkType {
    Individual,
    Company,
}

impl NetworkType {
    pub fn is_individual(&self) -> bool {
        matches!(self, Self::Individual)
    }

    pub fn is_company(&self) -> bool {
        matches!(self, Self::Company)
    }

    pub fn requires_business_registration(&self) -> bool {
        matches!(self, Self::Company)
    }

    pub fn allowed_company_types(&self) -> Vec<CompanyType> {
        match self {
            Self::Individual => vec![],
            Self::Company => CompanyType::all().to_vec(),
        }
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Individual => "individual",
            Self::Company => "company",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for NetworkType {
    type Err = NetworkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "individual" => Ok(Self::Individual),
            "company" => Ok(Self::Company),
            _ => Err(NetworkTypeError::InvalidType(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_type_properties() {
        assert!(NetworkType::Individual.is_individual());
        assert!(NetworkType::Company.is_company());
        assert!(!NetworkType::Individual.requires_business_registration());
        assert!(NetworkType::Company.requires_business_registration());
    }

    #[test]
    fn test_allowed_company_types() {
        assert!(NetworkType::Individual.allowed_company_types().is_empty());
        assert!(!NetworkType::Company.allowed_company_types().is_empty());
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            "individual".parse::<NetworkType>().unwrap(),
            NetworkType::Individual
        );
        assert_eq!(
            "company".parse::<NetworkType>().unwrap(),
            NetworkType::Company
        );
        assert!("invalid".parse::<NetworkType>().is_err());
    }
}
