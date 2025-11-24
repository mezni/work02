use serde::{Serialize, Deserialize};
use regex::Regex;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: &str) -> Result<Self> {
        let re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
        if re.is_match(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(anyhow!("Invalid email format"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyName(pub String);

impl CompanyName {
    pub fn new(name: &str) -> Result<Self> {
        if !name.trim().is_empty() {
            Ok(Self(name.to_string()))
        } else {
            Err(anyhow!("Company name cannot be empty"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(pub String);

impl Password {
    pub fn new(pw: &str) -> Result<Self> {
        if pw.len() >= 6 {
            Ok(Self(pw.to_string()))
        } else {
            Err(anyhow!("Password too short"))
        }
    }
}
