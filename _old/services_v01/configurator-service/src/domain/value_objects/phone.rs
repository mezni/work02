use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Phone(String);

impl Phone {
    pub fn new(value: String) -> Result<Self, String> {
        if value.len() < 6 {
            return Err(format!("Invalid phone number: {}", value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
