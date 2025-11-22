use regex::Regex;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Email(String);

impl Email {
    pub fn new<S: AsRef<str>>(email: S) -> Result<Self, String> {
        let email_str = email.as_ref().trim();
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

        if email_regex.is_match(email_str) {
            Ok(Self(email_str.to_string()))
        } else {
            Err(format!("Invalid email address: {}", email_str))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
