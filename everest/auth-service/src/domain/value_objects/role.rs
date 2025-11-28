use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use std::str::FromStr;
use crate::domain::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin = 4,
    Partner = 3,
    Operator = 2,
    RegisteredUser = 1,
    Public = 0,
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Role::Admin => "admin",
            Role::Partner => "partner",
            Role::Operator => "operator",
            Role::RegisteredUser => "registered_user",
            Role::Public => "public",
        })
    }
}

impl FromStr for Role {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "partner" => Ok(Role::Partner),
            "operator" => Ok(Role::Operator),
            "registered_user" => Ok(Role::RegisteredUser),
            "public" => Ok(Role::Public),
            _ => Err(DomainError::InvalidRole),
        }
    }
}

impl Role {
    pub fn from_i32(value: i32) -> Result<Self, DomainError> {
        match value {
            4 => Ok(Role::Admin),
            3 => Ok(Role::Partner),
            2 => Ok(Role::Operator),
            1 => Ok(Role::RegisteredUser),
            0 => Ok(Role::Public),
            _ => Err(DomainError::InvalidRole),
        }
    }

    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}
