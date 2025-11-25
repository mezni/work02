use auth_service::domain::events::{
    Event, UserAssignedToCompanyEvent, UserLoggedInEvent, UserRegisteredEvent,
    UserRoleChangedEvent, UserVerifiedEvent,
};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_user_registered_event() {
    let user_id = Uuid::new_v4().to_string();
    let event = UserRegisteredEvent {
        user_id: user_id.clone(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        role: "user".to_string(),
        registered_at: Utc::now(),
    };

    assert_eq!(event.event_type(), "user.registered");
    assert_eq!(event.metadata().user_id, Some(user_id));
    assert_eq!(event.metadata().source, "auth-service");

    let event_data = event.event_data();
    assert!(event_data.is_object());
}

#[test]
fn test_user_verified_event() {
    let user_id = Uuid::new_v4().to_string();
    let event = UserVerifiedEvent {
        user_id: user_id.clone(),
        verified_at: Utc::now(),
    };

    assert_eq!(event.event_type(), "user.verified");
    assert_eq!(event.metadata().user_id, Some(user_id));
}

#[test]
fn test_user_role_changed_event() {
    let user_id = Uuid::new_v4().to_string();
    let changed_by = Uuid::new_v4().to_string();
    let event = UserRoleChangedEvent {
        user_id: user_id.clone(),
        old_role: "user".to_string(),
        new_role: "admin".to_string(),
        changed_by: changed_by.clone(),
        changed_at: Utc::now(),
    };

    assert_eq!(event.event_type(), "user.role_changed");
    assert_eq!(event.metadata().user_id, Some(user_id));

    let event_data = event.event_data();
    assert_eq!(event_data["old_role"], "user");
    assert_eq!(event_data["new_role"], "admin");
}

#[test]
fn test_user_assigned_to_company_event() {
    let user_id = Uuid::new_v4().to_string();
    let company_id = Uuid::new_v4().to_string();
    let assigned_by = Uuid::new_v4().to_string();

    let event = UserAssignedToCompanyEvent {
        user_id: user_id.clone(),
        company_id: company_id.clone(),
        assigned_by: assigned_by.clone(),
        assigned_at: Utc::now(),
    };

    assert_eq!(event.event_type(), "user.assigned_to_company");
    assert_eq!(event.metadata().user_id, Some(user_id));

    let event_data = event.event_data();
    assert_eq!(event_data["company_id"], company_id);
    assert_eq!(event_data["assigned_by"], assigned_by);
}

#[test]
fn test_user_logged_in_event() {
    let user_id = Uuid::new_v4().to_string();
    let event = UserLoggedInEvent {
        user_id: user_id.clone(),
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        logged_in_at: Utc::now(),
    };

    assert_eq!(event.event_type(), "user.logged_in");
    assert_eq!(event.metadata().user_id, Some(user_id));

    let event_data = event.event_data();
    assert_eq!(event_data["ip_address"], "192.168.1.1");
    assert_eq!(event_data["user_agent"], "Mozilla/5.0");
}

#[test]
fn test_event_serialization() {
    let event = UserRegisteredEvent {
        user_id: "user-123".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        role: "user".to_string(),
        registered_at: Utc::now(),
    };

    let event_data = event.event_data();

    // Verify serialization works
    let serialized = serde_json::to_string(&event_data).unwrap();
    let deserialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized["user_id"], "user-123");
    assert_eq!(deserialized["username"], "testuser");
    assert_eq!(deserialized["email"], "test@example.com");
}
