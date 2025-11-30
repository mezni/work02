use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Partner,
    Operator,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Partner => write!(f, "partner"),
            Role::Operator => write!(f, "operator"),
        }
    }
}

impl Role {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "partner" => Some(Role::Partner),
            "operator" => Some(Role::Operator),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') && email.len() > 3 {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct OrganisationName(String);

impl OrganisationName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.is_empty() {
            Err("Organisation name cannot be empty".to_string())
        } else {
            Ok(OrganisationName(name))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}