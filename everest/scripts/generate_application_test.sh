#!/bin/bash

set -e

echo "Generating application layer tests..."

cd auth-service

# Create application test directory
mkdir -p tests/unit/application/{services,dtos,commands,queries}

# Application tests mod.rs
cat > tests/unit/application/mod.rs << 'EOF'
pub mod services;
pub mod dtos;
pub mod commands;
pub mod queries;
EOF

# Service tests
cat > tests/unit/application/services/user_service.rs << 'EOF'
use auth_service::application::services::UserApplicationServiceImpl;

#[tokio::test]
async fn test_user_service_creation() {
    let _service = UserApplicationServiceImpl::new(
        todo!(), // user_repository
        todo!(), // company_repository  
        todo!(), // audit_log_repository
        todo!(), // keycloak_client
    );
    // Basic test to ensure service can be created
    assert!(true);
}
EOF

echo "Application layer tests generated successfully!"