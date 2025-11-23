// tests/integration/user_management_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::events::DomainEvent;
    use auth_service::domain::value_objects::{Email, PhoneNumber, Username};
    use auth_service::domain::DomainError;
    #[test]
    fn test_complete_user_lifecycle() {
        // Create user
        let keycloak_id = "keycloak-123".to_string();
        let username = Username::new("johndoe").unwrap();
        let email = Email::new("john@example.com").unwrap();

        let (mut user, created_event) = User::create(
            keycloak_id,
            username,
            email,
            Some("John".to_string()),
            Some("Doe".to_string()),
        )
        .unwrap();

        assert_eq!(created_event.event_type(), "user.created");
        assert!(user.is_active());

        // Update profile
        let updated_event = user
            .update_profile(
                Some("Johnny".to_string()),
                Some("Doey".to_string()),
                Some("+1234567890".to_string()),
            )
            .unwrap();

        assert_eq!(updated_event.event_type(), "user.updated");
        assert_eq!(user.first_name(), Some("Johnny"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");

        // Mark email verified
        user.mark_email_verified();
        assert!(user.is_email_verified());

        // Record login
        user.record_login();
        assert!(user.last_login_at().is_some());

        // Deactivate
        let deactivated_event = user.deactivate().unwrap();
        assert_eq!(deactivated_event.event_type(), "user.deactivated");
        assert!(!user.is_active());

        // Reactivate
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_validation_chain() {
        // Test that invalid data is caught at the value object level
        let invalid_email_result = Email::new("invalid-email");
        assert!(matches!(
            invalid_email_result,
            Err(DomainError::Validation(_))
        ));

        let invalid_username_result = Username::new("ab");
        assert!(matches!(
            invalid_username_result,
            Err(DomainError::Validation(_))
        ));

        let invalid_phone_result = PhoneNumber::new("invalid");
        assert!(matches!(
            invalid_phone_result,
            Err(DomainError::Validation(_))
        ));

        // Valid data should work
        let valid_email = Email::new("valid@example.com").unwrap();
        let valid_username = Username::new("validuser").unwrap();

        let (user, _) = User::create(
            "keycloak-123".to_string(),
            valid_username,
            valid_email,
            Some("Valid".to_string()),
            Some("User".to_string()),
        )
        .unwrap();

        assert!(user.is_active());
    }
}
