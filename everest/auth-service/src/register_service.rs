use crate::user_repository::UserRepository;
use std::collections::HashMap;
use std::error::Error;

pub struct RegisterService {
    user_repo: UserRepository,
}

impl RegisterService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    /// Register a user with optional attributes
    pub async fn register_user(
        &self,
        username: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
        attributes: Option<HashMap<String, Vec<String>>>,
    ) -> Result<String, Box<dyn Error>> {
        self.user_repo
            .create_user(username, first_name, last_name, password, attributes)
            .await
    }
}
