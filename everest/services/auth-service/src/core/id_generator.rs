/// Generate a unique user ID with USR prefix
pub fn generate_user_id() -> String {
    format!("USR-{}", nanoid::nanoid!(20))
}

/// Generate a unique audit ID with AUD prefix
pub fn generate_audit_id() -> String {
    format!("AUD-{}", nanoid::nanoid!(20))
}

/// Generate a unique outbox event ID with EVT prefix
pub fn generate_event_id() -> String {
    format!("EVT-{}", nanoid::nanoid!(20))
}

/// Generate a custom ID with a given prefix
pub fn generate_id(prefix: &str, length: usize) -> String {
    format!("{}-{}", prefix, nanoid::nanoid!(length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_user_id() {
        let id = generate_user_id();
        assert!(id.starts_with("USR"));
        assert_eq!(id.len(), 32); // USR (3) + nanoid (29)
    }

    #[test]
    fn test_generate_audit_id() {
        let id = generate_audit_id();
        assert!(id.starts_with("AUD"));
        assert_eq!(id.len(), 32);
    }

    #[test]
    fn test_generate_event_id() {
        let id = generate_event_id();
        assert!(id.starts_with("EVT"));
        assert_eq!(id.len(), 32);
    }

    #[test]
    fn test_generate_custom_id() {
        let id = generate_id("TEST", 10);
        assert!(id.starts_with("TEST"));
        assert_eq!(id.len(), 14); // TEST (4) + 10
    }

    #[test]
    fn test_uniqueness() {
        let id1 = generate_user_id();
        let id2 = generate_user_id();
        assert_ne!(id1, id2);
    }
}