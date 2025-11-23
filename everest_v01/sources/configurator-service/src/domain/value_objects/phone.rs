use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Phone(String);

impl Phone {
    pub fn new<S: AsRef<str>>(phone: S) -> Result<Self, String> {
        let phone_str = phone.as_ref().trim();
        let phone_regex = Regex::new(r"^\+?[0-9\s\-]{7,15}$").unwrap();

        if phone_regex.is_match(phone_str) {
            Ok(Self(phone_str.to_string()))
        } else {
            Err(format!("Invalid phone number: {}", phone_str))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ToString for Phone {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
