#!/bin/bash

KEYCLOAK_URL="http://127.0.0.1:5080"
REALM="auth-service-realm"

echo "=== Complete Keycloak Setup for Port 5080 ==="

# Wait for Keycloak to be ready
echo "1. Waiting for Keycloak to be ready..."
until curl -s http://127.0.0.1:5080 > /dev/null; do
    sleep 2
done
echo "✅ Keycloak is ready"

# Get admin token
echo "2. Getting admin token..."
ADMIN_RESPONSE=$(curl -s -X POST \
  "$KEYCLOAK_URL/realms/master/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin" \
  -d "password=password" \
  -d "grant_type=password" \
  -d "client_id=admin-cli")

# Use Python for JSON parsing
ADMIN_TOKEN=$(echo "$ADMIN_RESPONSE" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    print(data.get('access_token', ''))
except:
    print('')
")

if [ -z "$ADMIN_TOKEN" ]; then
    echo "❌ Cannot get admin token. Response:"
    echo "$ADMIN_RESPONSE"
    exit 1
fi
echo "✅ Admin token obtained"

# Check if realm exists
echo "3. Checking if realm '$REALM' exists..."
REALM_CHECK=$(curl -s -o /dev/null -w "%{http_code}" \
  "$KEYCLOAK_URL/admin/realms/$REALM" \
  -H "Authorization: Bearer $ADMIN_TOKEN")

if [ "$REALM_CHECK" -eq 200 ]; then
    echo "✅ Realm '$REALM' already exists"
else
    echo "❌ Realm '$REALM' doesn't exist or is not accessible"
    echo "Please create it manually in the Admin Console or check your Keycloak configuration"
fi

# Create user 'dali'
echo "4. Creating user 'dali'..."
USER_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM/users" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "dali",
    "email": "dali@example.com",
    "firstName": "Dali",
    "lastName": "Test",
    "enabled": true,
    "credentials": [
      {
        "type": "password",
        "value": "password",
        "temporary": false
      }
    ]
  }')

HTTP_CODE=$(echo "$USER_RESPONSE" | tail -n1)
if [ "$HTTP_CODE" -eq 201 ]; then
    echo "✅ User 'dali' created successfully"
else
    echo "❌ Failed to create user (HTTP $HTTP_CODE)"
    echo "Response: $(echo "$USER_RESPONSE" | head -n -1)"
fi

echo ""
echo "=== Setup Complete ==="
echo "Keycloak Admin Console: http://127.0.0.1:5080/admin/master/console/#/realms"
echo "Realm: $REALM"
echo "Test user: dali"
echo "Test password: password"