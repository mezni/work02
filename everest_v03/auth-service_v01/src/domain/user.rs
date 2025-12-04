// src/domain/user.rs
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
}

impl User {
    pub fn new(id: String, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
            role: "USER".to_string(),
            company_name: "".to_string(),
            station_name: "".to_string(),
        }
    }
}