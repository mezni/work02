pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
}
