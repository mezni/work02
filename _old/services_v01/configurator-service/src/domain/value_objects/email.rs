use serde::{Serialize, Deserialize};
use regex::Regex;

use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, String> {
        let regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

        if !regex.is_match(&value) {
            return Err(format!("Invalid email: {}", value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
