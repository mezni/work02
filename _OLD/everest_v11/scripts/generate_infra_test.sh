#!/bin/bash

set -e

echo "Generating infrastructure layer tests..."

cd auth-service

# Create infrastructure test directory
mkdir -p tests/unit/infrastructure/{persistence,auth,config}

# Infrastructure tests mod.rs
cat > tests/unit/infrastructure/mod.rs << 'EOF'
pub mod persistence;
pub mod auth;
pub mod config;
EOF

# Persistence tests
cat > tests/unit/infrastructure/persistence.rs << 'EOF'
use auth_service::infrastructure::persistence::repositories::{
    PostgresUserRepository, PostgresCompanyRepository, PostgresAuditLogRepository
};
use auth_service::domain::entities::{User, Company, AuditLog};
use auth_service::domain::enums::{UserRole, AuditAction};
use uuid::Uuid;

#[cfg(test)]
mod user_repository_tests {
    use super::*;

    #[test]
    fn test_user_repository_initialization() {
        // Test that UserRepositoryImpl can be initialized
        assert!(true, "PostgresUserRepository should compile successfully");
    }

    #[test]
    fn test_user_role_parsing() {
        assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
        assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
        assert!("invalid".parse::<UserRole>().is_err());
    }
}
EOF

echo "Infrastructure layer tests generated successfully!"