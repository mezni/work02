// tests/unit/entities/user_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::events::{
        DomainEvent, UserCreatedEvent, UserDeactivatedEvent, UserUpdatedEvent,
    };
    use auth_service::domain::value_objects::{Email, PhoneNumber, Username};
    use auth_service::domain::DomainError;
    use chrono::{DateTime, TimeZone, Utc};

    fn create_test_user() -> (User, UserCreatedEvent) {
        let keycloak_id = "test-keycloak-id".to_string();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();

        User::create(
            keycloak_id,
            username,
            email,
            Some("John".to_string()),
            Some("Doe".to_string()),
        )
        .unwrap()
    }

    fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }

    #[test]
    fn test_user_creation() {
        let (user, event) = create_test_user();

        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(user.phone_number().is_empty()); // Should be empty by default

        // Verify event
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.email().as_str(), "test@example.com");
        assert_eq!(event.username().as_str(), "testuser");
    }

    #[test]
    fn test_user_creation_with_minimal_data() {
        let keycloak_id = "minimal-keycloak-id".to_string();
        let username = Username::new("minimaluser").unwrap();
        let email = Email::new("minimal@example.com").unwrap();

        let (user, event) = User::create(keycloak_id, username, email, None, None).unwrap();

        assert_eq!(user.keycloak_id(), "minimal-keycloak-id");
        assert_eq!(user.username().as_str(), "minimaluser");
        assert_eq!(user.email().as_str(), "minimal@example.com");
        assert_eq!(user.first_name(), None);
        assert_eq!(user.last_name(), None);
        assert!(user.is_active());
        assert!(!user.is_email_verified());

        // Verify event
        assert_eq!(event.event_type(), "user.created");
    }

    #[test]
    fn test_user_update_profile() {
        let (mut user, _) = create_test_user();
        let old_email = user.email().clone();

        let event = user
            .update_profile(
                Some("Jane".to_string()),
                Some("Smith".to_string()),
                Some("+1234567890".to_string()),
            )
            .unwrap();

        assert_eq!(user.first_name(), Some("Jane"));
        assert_eq!(user.last_name(), Some("Smith"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");

        // Verify event
        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.old_email(), &old_email);
    }

    #[test]
    fn test_user_update_profile_partial() {
        let (mut user, _) = create_test_user();
        let original_first_name = user.first_name().map(|s| s.to_string());

        // Update only last name and phone
        let event = user
            .update_profile(
                None,                            // Keep existing first name
                Some("UpdatedLast".to_string()), // Update last name
                Some("+1987654321".to_string()), // Update phone
            )
            .unwrap();

        // First name should remain unchanged
        assert_eq!(user.first_name(), original_first_name.as_deref());
        assert_eq!(user.last_name(), Some("UpdatedLast"));
        assert_eq!(user.phone_number().as_str(), "+1987654321");

        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_empty_phone() {
        let (mut user, _) = create_test_user();

        // Set a phone number first
        user.update_profile(None, None, Some("+1234567890".to_string()))
            .unwrap();
        assert!(!user.phone_number().is_empty());

        // Update with empty phone (should clear it)
        let event = user
            .update_profile(None, None, Some("".to_string()))
            .unwrap();

        assert!(user.phone_number().is_empty());
        assert_eq!(event.event_type(), "user.updated");
    }

    // tests/unit/entities/user_test.rs - update the failing test
    #[test]
    fn test_user_update_profile_with_invalid_phone() {
        let (mut user, _) = create_test_user();

        let result = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("invalid-phone".to_string()),
        );

        // Should fail with validation error
        assert!(matches!(result, Err(DomainError::Validation(_))));

        // Ensure NO fields were changed due to validation error
        // The validation happens before any fields are updated
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.phone_number().is_empty()); // Should still be empty
    }

    #[test]
    fn test_user_update_profile_preserves_existing_values() {
        let (mut user, _) = create_test_user();

        // Set initial phone number
        user.update_profile(None, None, Some("+1234567890".to_string()))
            .unwrap();

        // Update only first name
        user.update_profile(
            Some("OnlyFirst".to_string()),
            None, // Keep existing last name
            None, // Keep existing phone number
        )
        .unwrap();

        assert_eq!(user.first_name(), Some("OnlyFirst"));
        assert_eq!(user.last_name(), Some("Doe")); // Should remain unchanged
        assert_eq!(user.phone_number().as_str(), "+1234567890"); // Should remain unchanged
    }

    #[test]
    fn test_user_mark_email_verified() {
        let (mut user, _) = create_test_user();

        assert!(!user.is_email_verified());
        user.mark_email_verified();
        assert!(user.is_email_verified());

        // Verify updated_at changed
        let updated_at_after = user.updated_at();
        assert!(updated_at_after > user.created_at());
    }

    #[test]
    fn test_user_record_login() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();

        assert!(user.last_login_at().is_none());
        user.record_login();

        assert!(user.last_login_at().is_some());
        assert!(user.updated_at() > original_updated_at);

        // Record another login
        let second_login_time = user.last_login_at().unwrap();
        user.record_login();
        assert!(user.last_login_at().unwrap() > second_login_time);
    }

    #[test]
    fn test_user_deactivate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();

        assert!(user.is_active());
        let event = user.deactivate().unwrap();
        assert!(!user.is_active());
        assert!(user.updated_at() > original_updated_at);

        // Verify event
        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), user.id());
    }

    #[test]
    fn test_user_cannot_deactivate_already_deactivated() {
        let (mut user, _) = create_test_user();

        user.deactivate().unwrap();
        let result = user.deactivate();

        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        assert!(!user.is_active()); // Should still be deactivated
    }

    #[test]
    fn test_user_activate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();

        user.deactivate().unwrap();
        assert!(!user.is_active());

        user.activate();
        assert!(user.is_active());
        assert!(user.updated_at() > original_updated_at);
    }

    #[test]
    fn test_user_getters() {
        let (user, _) = create_test_user();

        // Test all getters
        assert!(!user.id().as_str().is_empty());
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.phone_number().is_empty()); // Should be empty by default
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(user.created_at() <= chrono::Utc::now());
        assert!(user.updated_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_id_generation() {
        let (user1, _) = create_test_user();
        let (user2, _) = create_test_user();

        // Each user should have a unique ID
        assert_ne!(user1.id().as_str(), user2.id().as_str());

        // IDs should be valid UUIDs
        let uuid_result1 = uuid::Uuid::parse_str(user1.id().as_str());
        let uuid_result2 = uuid::Uuid::parse_str(user2.id().as_str());

        assert!(uuid_result1.is_ok());
        assert!(uuid_result2.is_ok());
    }

    #[test]
    fn test_user_created_at_and_updated_at() {
        let (user, _) = create_test_user();

        // Created and updated should be initially the same
        assert_eq!(user.created_at(), user.updated_at());

        // Both should be in the past (or very close to now)
        let now = chrono::Utc::now();
        assert!(user.created_at() <= now);
        assert!(user.updated_at() <= now);
    }

    #[test]
    fn test_user_phone_number_defaults_to_empty() {
        let keycloak_id = "test-keycloak".to_string();
        let username = Username::new("newuser").unwrap();
        let email = Email::new("new@example.com").unwrap();

        let (user, _) = User::create(keycloak_id, username, email, None, None).unwrap();

        assert!(user.phone_number().is_empty());
    }

    #[test]
    fn test_user_immutability_via_getters() {
        let (user, _) = create_test_user();

        // All getters should return immutable references or copies
        let id = user.id();
        let username = user.username();
        let email = user.email();

        // These should all compile and work
        assert_eq!(id.as_str(), user.id().as_str());
        assert_eq!(username.as_str(), user.username().as_str());
        assert_eq!(email.as_str(), user.email().as_str());
    }

    #[test]
    fn test_user_lifecycle_events() {
        // Create user
        let keycloak_id = "lifecycle-user".to_string();
        let username = Username::new("lifecycle").unwrap();
        let email = Email::new("lifecycle@example.com").unwrap();

        let (mut user, created_event) = User::create(
            keycloak_id,
            username,
            email,
            Some("Life".to_string()),
            Some("Cycle".to_string()),
        )
        .unwrap();

        assert_eq!(created_event.event_type(), "user.created");

        // Update profile
        let updated_event = user
            .update_profile(
                Some("UpdatedLife".to_string()),
                Some("UpdatedCycle".to_string()),
                Some("+1111111111".to_string()),
            )
            .unwrap();

        assert_eq!(updated_event.event_type(), "user.updated");

        // Deactivate
        let deactivated_event = user.deactivate().unwrap();
        assert_eq!(deactivated_event.event_type(), "user.deactivated");

        // Reactivate (no event for reactivation in current design)
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_business_rules() {
        let (mut user, _) = create_test_user();

        // Test that business rules are enforced
        user.deactivate().unwrap();

        // Try to deactivate again - should fail
        let result = user.deactivate();
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));

        // Error message should be meaningful
        if let Err(DomainError::BusinessRule(msg)) = result {
            assert!(
                msg.contains("already deactivated") || msg.contains("User already deactivated")
            );
        }
    }

    #[test]
    fn test_user_clone() {
        let (user, _) = create_test_user();
        let cloned_user = user.clone();

        // Cloned user should have same values
        assert_eq!(user.id().as_str(), cloned_user.id().as_str());
        assert_eq!(user.username().as_str(), cloned_user.username().as_str());
        assert_eq!(user.email().as_str(), cloned_user.email().as_str());
    }

    #[test]
    fn test_user_debug_format() {
        let (user, _) = create_test_user();

        // Debug format should work without panicking
        let debug_output = format!("{:?}", user);
        assert!(!debug_output.is_empty());
        // Should contain some identifiable information
        assert!(debug_output.contains("testuser") || debug_output.contains("test@example.com"));
    }
}
