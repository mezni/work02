use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId {
    value: Uuid,
}

impl UserId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(value: Uuid) -> Self {
        Self { value }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        UserId::from_uuid(value)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Uuid {
        user_id.value
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.value
    }
}
