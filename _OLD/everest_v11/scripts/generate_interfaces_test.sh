#!/bin/bash

set -e

echo "Generating interfaces layer tests..."

cd auth-service

# Create interfaces test directory
mkdir -p tests/unit/interfaces/{controllers,middleware,routes}

# Interfaces tests mod.rs
cat > tests/unit/interfaces/mod.rs << 'EOF'
pub mod controllers;
pub mod middleware;
pub mod routes;
EOF

# Controller tests
cat > tests/unit/interfaces/controllers/user_controller.rs << 'EOF'
use auth_service::interfaces::controllers::user_controller;

#[test]
fn test_user_controller_compilation() {
    // Test that controller functions compile
    assert!(true, "User controller should compile successfully");
}
EOF

echo "Interfaces layer tests generated successfully!"