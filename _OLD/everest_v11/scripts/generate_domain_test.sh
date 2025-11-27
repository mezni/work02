#!/bin/bash

set -e

echo "Generating domain layer tests..."

cd auth-service

# Create domain test directory
mkdir -p tests/unit/domain/{entities,value_objects,aggregates,services,repositories,events,enums,errors}

# Domain tests mod.rs
cat > tests/unit/domain/mod.rs << 'EOF'
pub mod entities;
pub mod value_objects;
pub mod aggregates;
pub mod services;
pub mod repositories;
pub mod events;
pub mod enums;
pub mod errors;
EOF

# Value Objects tests
cat > tests/unit/domain/value_objects.rs << 'EOF'
use auth_service::domain::value_objects::{Email, Password};

#[test]
fn test_email_validation() {
    // Valid email
    let email = Email::new("test@example.com".to_string());
    assert!(email.is_ok());
    assert_eq!(email.unwrap().value(), "test@example.com");
    
    // Invalid email
    let email = Email::new("invalid-email".to_string());
    assert!(email.is_err());
}

#[test]
fn test_password_validation() {
    // Valid password
    let password = Password::new("password123".to_string());
    assert!(password.is_ok());
    
    // Too short password
    let password = Password::new("short".to_string());
    assert!(password.is_err());
}
EOF

cat > tests/unit/mod.rs << 'EOF'
pub mod domain;
EOF

echo "Domain layer tests generated successfully!"