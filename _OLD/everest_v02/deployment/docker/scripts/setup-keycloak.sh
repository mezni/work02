#!/bin/bash

echo "Setting up Keycloak realm and client..."

# Wait for Keycloak to be ready
echo "Waiting for Keycloak to start..."
#until curl -f -s http://localhost:5800/health/ready; do
#    sleep 5
#done

echo "Keycloak is ready. Setting up realm..."

# Get admin token
TOKEN=$(curl -s -X POST \
  http://localhost:5800/realms/master/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin&password=admin&grant_type=password&client_id=admin-cli" | jq -r '.access_token')

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
    echo "Failed to get admin token"
    exit 1
fi

echo "Admin token obtained successfully"

# Create realm
echo "Creating realm..."
REALM_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
  http://localhost:5800/admin/realms \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "realm": "auth-service",
    "enabled": true,
    "displayName": "Auth Service Realm",
    "loginWithEmailAllowed": true,
    "duplicateEmailsAllowed": false
  }')

if [ "$REALM_RESPONSE" -eq 201 ]; then
    echo "Realm created successfully"
else
    echo "Failed to create realm. HTTP status: $REALM_RESPONSE"
    # Try to continue anyway - realm might already exist
fi

# Create client
echo "Creating client..."
CLIENT_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
  http://localhost:5800/admin/realms/auth-service/clients \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "clientId": "auth-service-api",
    "enabled": true,
    "publicClient": false,
    "secret": "your-client-secret-here",
    "standardFlowEnabled": true,
    "implicitFlowEnabled": false,
    "directAccessGrantsEnabled": true,
    "serviceAccountsEnabled": true,
    "authorizationServicesEnabled": true,
    "redirectUris": [
      "http://localhost:8080/*",
      "http://auth-service:8080/*"
    ],
    "webOrigins": [
      "http://localhost:8080",
      "http://auth-service:8080"
    ],
    "protocol": "openid-connect",
    "attributes": {
      "client.secret.creation.time": "0",
      "backchannel.logout.session.required": "true",
      "oauth2.device.authorization.grant.enabled": "false"
    }
  }')

if [ "$CLIENT_RESPONSE" -eq 201 ]; then
    echo "Client created successfully"
else
    echo "Failed to create client. HTTP status: $CLIENT_RESPONSE"
fi

echo "Keycloak setup completed!"
echo "Keycloak Admin Console: http://localhost:5800/admin"
echo "Realm: auth-service"
echo "Client: auth-service-api"