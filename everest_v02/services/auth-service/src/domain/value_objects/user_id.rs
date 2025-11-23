use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("Invalid user ID format: {0}")]
    InvalidFormat(String),
}

/// User ID value object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    /// Generate a new UserId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse UserId from string
    pub fn parse_str(s: &str) -> Result<Self, UserIdError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| UserIdError::InvalidFormat(s.to_string()))
    }

    /// Get as Uuid
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Get as string
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    /// Check if UserId is nil/empty
    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl FromStr for UserId {
    type Err = UserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
