use std::fmt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct OrganisationId(i32);

impl OrganisationId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl fmt::Display for OrganisationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for OrganisationId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<OrganisationId> for i32 {
    fn from(id: OrganisationId) -> Self {
        id.0
    }
}