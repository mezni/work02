// tests/domain/value_objects/user_id_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::UserId;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_user_id() {
        let id = "123e4567-e89b-12d3-a456-426614174000".to_string();
        let user_id = UserId::new(id.clone()).unwrap();
        assert_eq!(user_id.as_str(), id);
    }

    #[test]
    fn test_user_id_rejects_empty_string() {
        let result = UserId::new("".to_string());
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_user_id_rejects_whitespace_only() {
        let result = UserId::new("   ".to_string());
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_generate_user_id() {
        let user_id = UserId::generate();
        assert!(!user_id.as_str().is_empty());
        // Should be a valid UUID
        let uuid_result = uuid::Uuid::parse_str(user_id.as_str());
        assert!(uuid_result.is_ok());
    }

    #[test]
    fn test_user_id_equality() {
        let id = "test-id".to_string();
        let user_id1 = UserId::new(id.clone()).unwrap();
        let user_id2 = UserId::new(id).unwrap();
        assert_eq!(user_id1, user_id2);
    }

    #[test]
    fn test_user_id_display() {
        let id = "test-id-123".to_string();
        let user_id = UserId::new(id.clone()).unwrap();
        assert_eq!(format!("{}", user_id), id);
    }
}