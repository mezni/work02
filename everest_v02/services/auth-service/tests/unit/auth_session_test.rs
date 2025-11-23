use auth_service::domain::entities::AuthSession;
use auth_service::domain::value_objects::UserId;
use chrono::{DateTime, Duration, Utc};

#[test]
fn test_auth_session_creation() {
    let user_id = UserId::new();
    let session = AuthSession::new(
        user_id,
        "Chrome on Windows".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(24),
    );

    assert_eq!(session.user_id(), user_id);
    assert!(!session.session_id().is_empty());
    assert!(!session.access_token().is_empty());
    assert!(!session.refresh_token().is_empty());
    assert!(!session.revoked());
    assert_eq!(session.device_info(), "Chrome on Windows");
    assert_eq!(session.ip_address(), "192.168.1.100");
    assert!(session.is_valid());
}

#[test]
fn test_auth_session_identity_equality() {
    let user_id = UserId::new();
    let session1 = AuthSession::new(
        user_id,
        "Device1".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(24),
    );

    let session2 = AuthSession::new(
        user_id,
        "Device2".to_string(),       // Different device
        "192.168.1.101".to_string(), // Different IP
        Duration::hours(12),         // Different duration
    );

    // Different sessions should not be equal
    assert_ne!(session1, session2);

    // Same session should be equal to itself
    assert_eq!(session1, session1);
}

#[test]
fn test_auth_session_refresh() {
    let user_id = UserId::new();
    let mut session = AuthSession::new(
        user_id,
        "Chrome on Windows".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(1),
    );

    let original_access_token = session.access_token().to_string();
    let original_expires_at = session.expires_at();
    let original_last_used = session.last_used_at();

    // Wait a moment to ensure time difference
    std::thread::sleep(std::time::Duration::from_millis(10));

    session.refresh(Duration::hours(2));

    // Access token should change
    assert_ne!(session.access_token(), original_access_token);
    // Expiry should be extended
    assert!(session.expires_at() > original_expires_at);
    // Last used should be updated
    assert!(session.last_used_at() > original_last_used);
}

#[test]
fn test_auth_session_revocation() {
    let user_id = UserId::new();
    let mut session = AuthSession::new(
        user_id,
        "Chrome on Windows".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(24),
    );

    assert!(!session.revoked());
    assert!(session.is_valid());

    session.revoke();

    assert!(session.revoked());
    assert!(!session.is_valid());
}

#[test]
fn test_auth_session_usage_update() {
    let user_id = UserId::new();
    let mut session = AuthSession::new(
        user_id,
        "Chrome on Windows".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(24),
    );

    let original_last_used = session.last_used_at();

    // Wait a moment to ensure time difference
    std::thread::sleep(std::time::Duration::from_millis(10));

    session.update_usage();

    assert!(session.last_used_at() > original_last_used);
}

#[test]
fn test_auth_session_expiry_validation() {
    let user_id = UserId::new();

    // Session with future expiry should be valid
    let future_session = AuthSession::new(
        user_id,
        "Device".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(1),
    );
    assert!(future_session.is_valid());
    assert!(!future_session.is_expired());
    assert!(future_session.time_until_expiry() > Duration::zero());

    // Session with past expiry should be invalid
    let past_session = AuthSession::new(
        user_id,
        "Device".to_string(),
        "192.168.1.100".to_string(),
        Duration::seconds(-1), // Already expired
    );
    assert!(!past_session.is_valid());
    assert!(past_session.is_expired());
    assert!(past_session.time_until_expiry() < Duration::zero());
}

#[test]
fn test_auth_session_revoked_validation() {
    let user_id = UserId::new();
    let mut session = AuthSession::new(
        user_id,
        "Device".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(24),
    );

    assert!(session.is_valid());

    session.revoke();

    assert!(!session.is_valid());
    assert!(!session.is_expired()); // Not expired, but revoked
}

#[test]
fn test_auth_session_token_generation() {
    let user_id = UserId::new();
    let session1 = AuthSession::new(
        user_id,
        "Device1".to_string(),
        "192.168.1.100".to_string(),
        Duration::hours(24),
    );

    let session2 = AuthSession::new(
        user_id,
        "Device2".to_string(),
        "192.168.1.101".to_string(),
        Duration::hours(24),
    );

    // Different sessions should have different tokens
    assert_ne!(session1.session_id(), session2.session_id());
    assert_ne!(session1.access_token(), session2.access_token());
    assert_ne!(session1.refresh_token(), session2.refresh_token());
}
