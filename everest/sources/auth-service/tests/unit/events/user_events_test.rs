// tests/domain/events/user_events_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::events::{
        DomainEvent, UserCreatedEvent, UserDeactivatedEvent, UserUpdatedEvent,
    };
    use auth_service::domain::value_objects::{Email, UserId, Username};

    fn create_test_user_id() -> UserId {
        UserId::new("test-user-id".to_string()).unwrap()
    }

    fn create_test_email() -> Email {
        Email::new("test@example.com").unwrap()
    }

    fn create_test_username() -> Username {
        Username::new("testuser").unwrap()
    }

    #[test]
    fn test_user_created_event() {
        let user_id = create_test_user_id();
        let email = create_test_email();
        let username = create_test_username();

        let event = UserCreatedEvent::new(user_id.clone(), email.clone(), username.clone());

        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), &user_id);
        assert_eq!(event.email(), &email);
        assert_eq!(event.username(), &username);
        assert_eq!(event.version(), "1.0");
        assert!(event.occurred_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_updated_event() {
        let user_id = create_test_user_id();
        let old_email = create_test_email();

        let event = UserUpdatedEvent::new(user_id.clone(), old_email.clone());

        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), &user_id);
        assert_eq!(event.old_email(), &old_email);
        assert_eq!(event.version(), "1.0");
        assert!(event.occurred_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_deactivated_event() {
        let user_id = create_test_user_id();

        let event = UserDeactivatedEvent::new(user_id.clone());

        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), &user_id);
        assert_eq!(event.version(), "1.0");
        assert!(event.occurred_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_domain_event_trait_implementation() {
        let user_id = create_test_user_id();
        let email = create_test_email();
        let username = create_test_username();

        let event = UserCreatedEvent::new(user_id, email, username);

        // Test that it implements the trait
        let domain_event: &dyn DomainEvent = &event;
        assert_eq!(domain_event.event_type(), "user.created");
        assert_eq!(domain_event.version(), "1.0");
    }
}
