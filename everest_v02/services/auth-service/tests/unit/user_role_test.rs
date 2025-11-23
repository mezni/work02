use auth_service::domain::entities::UserRole;
use auth_service::domain::value_objects::UserId;
use chrono::{DateTime, Utc};

#[test]
fn test_user_role_creation() {
    let user_id = UserId::new();
    let assigned_by = UserId::new();

    let user_role = UserRole::new(
        user_id,
        "admin".to_string(),
        assigned_by,
        Some("tenant1".to_string()),
    );

    assert_eq!(user_role.role_name(), "admin");
    assert_eq!(user_role.scope(), Some("tenant1"));
    assert!(user_role.assigned_at() <= Utc::now());
}

#[test]
fn test_user_role_equality() {
    let user_id = UserId::new();
    let assigned_by = UserId::new();

    let role1 = UserRole::new(
        user_id,
        "admin".to_string(),
        assigned_by,
        Some("tenant1".to_string()),
    );

    let role2 = UserRole::new(
        user_id,                     // Same user
        "admin".to_string(),         // Same role
        assigned_by,                 // Same assigner
        Some("tenant1".to_string()), // Same scope
    );

    let role3 = UserRole::new(
        user_id,
        "user".to_string(), // Different role
        assigned_by,
        Some("tenant1".to_string()),
    );

    let role4 = UserRole::new(
        user_id,
        "admin".to_string(),
        assigned_by,
        Some("tenant2".to_string()), // Different scope
    );

    // Same user, role, and scope should be equal
    assert_eq!(role1, role2);

    // Different role should not be equal
    assert_ne!(role1, role3);

    // Different scope should not be equal
    assert_ne!(role1, role4);
}

#[test]
fn test_user_role_scope_checks() {
    let user_id = UserId::new();
    let assigned_by = UserId::new();

    let scoped_role = UserRole::new(
        user_id,
        "admin".to_string(),
        assigned_by,
        Some("tenant1".to_string()),
    );

    let global_role = UserRole::new(user_id, "superadmin".to_string(), assigned_by, None);

    assert!(scoped_role.is_scoped());
    assert!(scoped_role.matches_scope("tenant1"));
    assert!(!scoped_role.matches_scope("tenant2"));
    assert!(!scoped_role.is_global());

    assert!(!global_role.is_scoped());
    assert!(!global_role.matches_scope("tenant1"));
    assert!(global_role.is_global());
}

#[test]
fn test_user_role_immutable_properties() {
    let user_id = UserId::new();
    let assigned_by = UserId::new();

    let role = UserRole::new(
        user_id,
        "admin".to_string(),
        assigned_by,
        Some("tenant1".to_string()),
    );

    let original_user_id = role.user_id();
    let original_role_name = role.role_name().to_string();
    let original_scope = role.scope().map(|s| s.to_string());
    let original_assigned_at = role.assigned_at();
    let original_assigned_by = role.assigned_by();

    // All properties should remain constant
    assert_eq!(role.user_id(), original_user_id);
    assert_eq!(role.role_name(), original_role_name);
    assert_eq!(role.scope().map(|s| s.to_string()), original_scope);
    assert_eq!(role.assigned_at(), original_assigned_at);
    assert_eq!(role.assigned_by(), original_assigned_by);
}
