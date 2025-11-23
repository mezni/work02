// tests/domain/value_objects/username_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::Username;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_username() {
        let username = Username::new("john_doe").unwrap();
        assert_eq!(username.as_str(), "john_doe");
    }

    #[test]
    fn test_username_is_normalized_to_lowercase() {
        let username = Username::new("JohnDoe").unwrap();
        assert_eq!(username.as_str(), "johndoe");
    }

    #[test]
    fn test_username_trim_whitespace() {
        let username = Username::new("  johndoe  ").unwrap();
        assert_eq!(username.as_str(), "johndoe");
    }

    #[test]
    fn test_username_rejects_too_short() {
        let result = Username::new("ab");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_username_rejects_too_long() {
        let long_username = "a".repeat(51);
        let result = Username::new(&long_username);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_username_allows_valid_characters() {
        let valid_usernames = vec!["user-name", "user_name", "user.name", "user123"];

        for username in valid_usernames {
            let result = Username::new(username);
            assert!(result.is_ok(), "Failed for: {}", username);
        }
    }

    #[test]
    fn test_username_rejects_invalid_characters() {
        let invalid_usernames = vec!["user@name", "user name", "user$name", "user#name"];

        for username in invalid_usernames {
            let result = Username::new(username);
            assert!(
                matches!(result, Err(DomainError::Validation(_))),
                "Should have failed for: {}",
                username
            );
        }
    }

    #[test]
    fn test_username_equality() {
        let username1 = Username::new("JohnDoe").unwrap();
        let username2 = Username::new("johndoe").unwrap();
        assert_eq!(username1, username2);
    }

    #[test]
    fn test_username_display() {
        let username = Username::new("test_user").unwrap();
        assert_eq!(format!("{}", username), "test_user");
    }
}
