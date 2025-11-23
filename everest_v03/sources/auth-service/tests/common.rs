// tests/common.rs
pub mod test_utils {
    use chrono::{DateTime, TimeZone, Utc};

    pub fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }

    pub fn create_valid_email() -> String {
        "test@example.com".to_string()
    }

    pub fn create_valid_username() -> String {
        "testuser".to_string()
    }

    pub fn create_valid_phone() -> String {
        "+1234567890".to_string()
    }

    pub fn create_valid_keycloak_id() -> String {
        "keycloak-123".to_string()
    }
}
