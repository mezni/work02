use crate::domain::value_objects::{Email, UserId};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<UserId>,
    pub username: String,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub enabled: bool,
}

impl User {
    pub fn new(
        username: String,
        email: Email,
        first_name: String,
        last_name: String,
    ) -> Self {
        Self {
            id: None,
            username,
            email,
            first_name,
            last_name,
            enabled: true,
        }
    }

    pub fn with_id(mut self, id: UserId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}