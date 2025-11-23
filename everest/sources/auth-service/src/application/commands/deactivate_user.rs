// src/application/commands/deactivate_user.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeactivateUserCommand {
    pub deactivated_by: String, // User ID of the person performing the action
}

impl DeactivateUserCommand {
    pub fn new(deactivated_by: String) -> Self {
        Self { deactivated_by }
    }
}