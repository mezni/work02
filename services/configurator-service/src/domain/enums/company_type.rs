use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompanyTypeError {
    #[error("Invalid company type: {0}")]
    InvalidType(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    Company,
    Cooperative,
    Government,
}

impl CompanyType {
    pub fn is_private(&self) -> bool {
        matches!(self, Self::Company)
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Self::Cooperative | Self::Government)
    }

    pub fn requires_tax_id(&self) -> bool {
        matches!(self, Self::Company | Self::Cooperative)
    }

    pub fn has_shareholders(&self) -> bool {
        matches!(self, Self::Company)
    }

    pub fn has_members(&self) -> bool {
        matches!(self, Self::Cooperative)
    }

    pub fn all() -> [Self; 3] {
        [Self::Company, Self::Cooperative, Self::Government]
    }
}

impl fmt::Display for CompanyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Company => "company",
            Self::Cooperative => "cooperative",
            Self::Government => "government",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for CompanyType {
    type Err = CompanyTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "company" => Ok(Self::Company),
            "cooperative" => Ok(Self::Cooperative),
            "government" => Ok(Self::Government),
            _ => Err(CompanyTypeError::InvalidType(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type_properties() {
        assert!(CompanyType::Company.is_private());
        assert!(CompanyType::Government.is_public());
        assert!(CompanyType::Company.requires_tax_id());
        assert!(!CompanyType::Government.requires_tax_id());
        assert!(CompanyType::Company.has_shareholders());
        assert!(CompanyType::Cooperative.has_members());
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            "company".parse::<CompanyType>().unwrap(),
            CompanyType::Company
        );
        assert_eq!(
            "cooperative".parse::<CompanyType>().unwrap(),
            CompanyType::Cooperative
        );
        assert_eq!(
            "government".parse::<CompanyType>().unwrap(),
            CompanyType::Government
        );
        assert!("invalid".parse::<CompanyType>().is_err());
    }
}
