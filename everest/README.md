POST   /api/v1/users                    - Create user
GET    /api/v1/users                    - List all users
GET    /api/v1/users/{user_id}          - Get user by ID
GET    /api/v1/users/username/{username} - Get user by username
PUT    /api/v1/users/{user_id}/enable   - Enable user
PUT    /api/v1/users/{user_id}/disable  - Disable user
DELETE /api/v1/users/{user_id}          - Delete user

POST   /api/v1/users/{user_id}/roles    - Assign role to user
GET    /api/v1/users/{user_id}/roles    - Get user roles

http://localhost:3000/swagger-ui

# Create a user
curl -X POST http://localhost:3000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "password": "SecurePass123!"
  }'

# List users
curl http://localhost:3000/api/v1/users

# Get user by ID
curl http://localhost:3000/api/v1/users/{user_id}

# Assign role
curl -X POST http://localhost:3000/api/v1/users/{user_id}/roles \
  -H "Content-Type: application/json" \
  -d '{"role_name": "user-manager"}'
```

## 📁 New Files Structure
```
src/interfaces/
├── mod.rs                    # AppState definition
├── routes.rs                 # Route configuration
└── handlers/
    ├── mod.rs
    └── user_handlers.rs      # API handlers with OpenAPI annotations