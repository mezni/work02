use auth_service::domain::value_objects::{UserId, UserIdError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct TestUser {
    id: UserId,
    name: String,
}

#[test]
fn test_new_user_id_generates_valid_uuid_v4() {
    let user_id = UserId::new();

    assert!(!user_id.is_nil());
    assert_eq!(user_id.as_uuid().get_version(), Some(uuid::Version::Random));

    let another_user_id = UserId::new();
    assert_ne!(user_id, another_user_id);
}

#[test]
fn test_default_implementation() {
    let default_id = UserId::default();
    assert!(!default_id.is_nil());
}

#[test]
fn test_parse_valid_uuid_string() {
    let valid_uuids = [
        "550e8400-e29b-41d4-a716-446655440000",
        "f47ac10b-58cc-4372-a567-0e02b2c3d479",
        "12345678-1234-5678-9abc-123456789abc",
        "00000000-0000-0000-0000-000000000000",
    ];

    for uuid_str in valid_uuids {
        let user_id = UserId::parse_str(uuid_str).unwrap();
        assert_eq!(user_id.as_string(), uuid_str);
        assert_eq!(user_id.to_string(), uuid_str);
    }
}

#[test]
fn test_parse_invalid_uuid_string() {
    let invalid_uuids = [
        "not-a-uuid",
        "550e8400-e29b-41d4-a716",
        "550e8400-e29b-41d4-a716-446655440000-extra",
        "",
        "12345678-1234-1234-1234-1234567890123",
        "gggggggg-gggg-gggg-gggg-gggggggggggg",
    ];

    for invalid_str in invalid_uuids {
        let result = UserId::parse_str(invalid_str);
        assert!(result.is_err(), "Expected error for: {}", invalid_str);
    }
}

#[test]
fn test_from_str_trait() {
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let user_id: UserId = uuid_str.parse().unwrap();
    assert_eq!(user_id.as_string(), uuid_str);

    let result: Result<UserId, _> = "invalid-uuid".parse();
    assert!(result.is_err());
}

#[test]
fn test_uuid_conversions() {
    let original_uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let user_id = UserId::from(original_uuid);
    assert_eq!(user_id.as_uuid(), original_uuid);

    let converted_back: Uuid = user_id.into();
    assert_eq!(converted_back, original_uuid);

    assert_eq!(user_id.as_ref(), &original_uuid);
}

#[test]
fn test_serialization_deserialization() {
    let user_id = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let json = serde_json::to_string(&user_id).unwrap();
    assert_eq!(json, "\"550e8400-e29b-41d4-a716-446655440000\"");

    let deserialized: UserId = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, user_id);
}

#[test]
fn test_serialization_with_struct() {
    let user_id = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let user = TestUser {
        id: user_id,
        name: "John Doe".to_string(),
    };

    let user_json = serde_json::to_string(&user).unwrap();
    let expected_json = r#"{"id":"550e8400-e29b-41d4-a716-446655440000","name":"John Doe"}"#;
    assert_eq!(user_json, expected_json);

    let deserialized_user: TestUser = serde_json::from_str(expected_json).unwrap();
    assert_eq!(deserialized_user.id, user_id);
    assert_eq!(deserialized_user.name, "John Doe");
}

#[test]
fn test_display_and_debug_format() {
    let user_id = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    assert_eq!(
        format!("User ID: {}", user_id),
        "User ID: 550e8400-e29b-41d4-a716-446655440000"
    );

    let debug_output = format!("{:?}", user_id);
    assert!(debug_output.contains("UserId"));
    assert!(debug_output.contains("550e8400-e29b-41d4-a716-446655440000"));
}

#[test]
fn test_equality_and_inequality() {
    let uuid1 = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid2 = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid3 = UserId::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();

    assert_eq!(uuid1, uuid1);
    assert_eq!(uuid1, uuid2);
    assert_eq!(uuid2, uuid1);
    assert_ne!(uuid1, uuid3);
    assert_ne!(uuid2, uuid3);
}

#[test]
fn test_hash_implementation() {
    let uuid1 = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid2 = UserId::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid3 = UserId::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();

    let mut map = HashMap::new();
    map.insert(uuid1, "user1");

    assert_eq!(map.get(&uuid2), Some(&"user1"));
    assert_eq!(map.get(&uuid3), None);

    let mut set = HashSet::new();
    set.insert(uuid1);
    set.insert(uuid2);
    set.insert(uuid3);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&uuid1));
    assert!(set.contains(&uuid2));
    assert!(set.contains(&uuid3));
}

#[test]
fn test_nil_uuid_behavior() {
    let nil_uuid = Uuid::nil();
    let user_id = UserId::from(nil_uuid);

    assert!(user_id.is_nil());
    assert_eq!(user_id.as_string(), "00000000-0000-0000-0000-000000000000");

    let random_user_id = UserId::new();
    assert_ne!(user_id, random_user_id);
}

#[test]
fn test_user_id_uniqueness() {
    let mut ids = HashSet::new();

    for _ in 0..1000 {
        let user_id = UserId::new();
        assert!(ids.insert(user_id), "Generated duplicate UserId");
    }
}
