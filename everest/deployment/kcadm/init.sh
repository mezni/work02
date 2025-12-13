#!/bin/bash
set -e

# -----------------------
# Configuration
# -----------------------

# Allow environment variable overrides
KEYCLOAK_URL="${KEYCLOAK_URL:-http://keycloak:8080}"
ADMIN_USER="${KEYCLOAK_ADMIN_USER:-admin}"
ADMIN_PASS="${KEYCLOAK_ADMIN_PASSWORD:-password}"
REALM="${KEYCLOAK_REALM:-myrealm}"
KCADM="/opt/keycloak/bin/kcadm.sh"

# Clients
FRONTEND_CLIENT_ID="${FRONTEND_CLIENT_ID:-auth-client}"
BACKEND_CLIENT_ID="${BACKEND_CLIENT_ID:-backend-admin}"
BACKEND_CLIENT_SECRET="${BACKEND_CLIENT_SECRET:-backend-admin-secret}"

# Users
ADMIN_USER_USERNAME="${ADMIN_USER_USERNAME:-adminuser}"
ADMIN_USER_PASSWORD="${ADMIN_USER_PASSWORD:-password}"

# Login flags (master realm)
LOGIN_FLAGS="--server $KEYCLOAK_URL --realm master --user $ADMIN_USER --password $ADMIN_PASS"

# -----------------------
# Helper functions
# -----------------------
log_info() {
    echo "[INFO] $(date '+%Y-%m-%d %H:%M:%S') $1"
}

log_error() {
    echo "[ERROR] $(date '+%Y-%m-%d %H:%M:%S') $1" >&2
}

run_kcadm() {
    local cmd="$1"
    shift
    if ! $KCADM $cmd $LOGIN_FLAGS "$@" 2>/tmp/kcadm-error.log; then
        log_error "Failed to execute: $KCADM $cmd $LOGIN_FLAGS $@"
        log_error "Error output: $(cat /tmp/kcadm-error.log)"
        return 1
    fi
    return 0
}

get_client_id() {
    local cid="$1"
    run_kcadm get clients -r "$REALM" -q clientId="$cid" --fields id --format csv 2>/dev/null | tail -n1 | tr -d '"' || echo ""
}

get_user_id() {
    local uname="$1"
    run_kcadm get users -r "$REALM" -q username="$uname" --fields id --format csv 2>/dev/null | tail -n1 | tr -d '"' || echo ""
}

validate_config() {
    local missing_vars=()
    
    if [ -z "$KEYCLOAK_URL" ]; then
        missing_vars+=("KEYCLOAK_URL")
    fi
    if [ -z "$ADMIN_USER" ]; then
        missing_vars+=("ADMIN_USER")
    fi
    if [ -z "$ADMIN_PASS" ]; then
        missing_vars+=("ADMIN_PASS")
    fi
    
    if [ ${#missing_vars[@]} -gt 0 ]; then
        log_error "Missing required configuration variables: ${missing_vars[*]}"
        return 1
    fi
    
    log_info "Configuration validated"
    return 0
}

wait_for_keycloak() {
    local max_attempts=30
    local attempt=1
    
    log_info "Waiting for Keycloak to be ready at $KEYCLOAK_URL..."
    
    while [ $attempt -le $max_attempts ]; do
        if $KCADM get realms/master $LOGIN_FLAGS >/dev/null 2>&1; then
            log_info "Keycloak is ready!"
            return 0
        fi
        
        log_info "Attempt $attempt/$max_attempts: Keycloak not ready, waiting..."
        sleep 5
        ((attempt++))
    done
    
    log_error "Keycloak failed to start within $max_attempts attempts"
    return 1
}

wait_for_service_account() {
    local client_id="$1"
    local max_attempts=10
    local attempt=1
    
    log_info "Waiting for service account user for client: $client_id"
    
    while [ $attempt -le $max_attempts ]; do
        local sa_user_id=$(get_user_id "service-account-$client_id")
        if [ -n "$sa_user_id" ] && [ "$sa_user_id" != "null" ]; then
            log_info "Service account user created with ID: $sa_user_id"
            return 0
        fi
        sleep 2
        ((attempt++))
    done
    
    log_error "Service account user not created within $max_attempts attempts"
    return 1
}

# -----------------------
# Main execution
# -----------------------
main() {
    # Handle reset option
    if [ "$1" = "--reset" ]; then
        log_info "Resetting realm $REALM..."
        run_kcadm delete realms/$REALM 2>/dev/null || log_info "Realm $REALM did not exist or was already deleted"
    fi
    
    # Validate configuration
    validate_config
    
    # -----------------------
    # 1. Wait for Keycloak
    # -----------------------
    wait_for_keycloak || exit 1
    
    # -----------------------
    # 2. Create realm
    # -----------------------
    if ! run_kcadm get realms/$REALM >/dev/null 2>&1; then
        log_info "Creating realm $REALM"
        run_kcadm create realms \
            -s realm="$REALM" \
            -s enabled=true \
            -s attributes.userProfileEnabled=true \
            -s attributes.unmanagedAttributePolicy=ENABLED \
            -s registrationAllowed=false \
            -s rememberMe=true \
            -s resetPasswordAllowed=true \
            -s loginWithEmailAllowed=false \
            -s duplicateEmailsAllowed=false \
            -s sslRequired="external"
    else
        log_info "Realm $REALM already exists."
    fi
    
    # -----------------------
    # 3. Create roles
    # -----------------------
    log_info "Creating realm roles..."
    for ROLE in admin user operator partner; do
        if ! run_kcadm get roles -r "$REALM" -q name="$ROLE" | grep -q id; then
            run_kcadm create roles -r "$REALM" -s name="$ROLE" -s description="$ROLE role"
            log_info "Created role: $ROLE"
        else
            log_info "Role $ROLE already exists."
        fi
    done
    
    # -----------------------
    # 4. Create groups (optional)
    # -----------------------
    log_info "Creating user groups..."
    for GROUP in "Admins" "Users" "Operators" "Partners"; do
        if ! run_kcadm get groups -r "$REALM" -q name="$GROUP" | grep -q id; then
            run_kcadm create groups -r "$REALM" -s name="$GROUP"
            log_info "Created group: $GROUP"
        else
            log_info "Group $GROUP already exists."
        fi
    done
    
    # -----------------------
    # 5. Create clients
    # -----------------------
    # Frontend client
    log_info "Configuring frontend client: $FRONTEND_CLIENT_ID"
    if ! run_kcadm get clients -r "$REALM" -q clientId="$FRONTEND_CLIENT_ID" | grep -q id; then
        run_kcadm create clients -r "$REALM" \
            -s clientId="$FRONTEND_CLIENT_ID" \
            -s name="Frontend Login Client" \
            -s description="Client for frontend application authentication" \
            -s enabled=true \
            -s protocol=openid-connect \
            -s publicClient=true \
            -s standardFlowEnabled=true \
            -s implicitFlowEnabled=false \
            -s directAccessGrantsEnabled=true \
            -s serviceAccountsEnabled=false \
            -s authorizationServicesEnabled=false \
            -s bearerOnly=false \
            -s consentRequired=false \
            -s frontchannelLogout=true \
            -s attributes.'pkce.code.challenge.method'=S256 \
            -s attributes.'post.logout.redirect.uris'='http://localhost:8081/*' \
            -s attributes.'backchannel.logout.session.required'=true \
            -s attributes.'backchannel.logout.revoke.offline.tokens'=false \
            -s redirectUris='["http://localhost:8081/*", "http://localhost:3000/*"]' \
            -s webOrigins='["http://localhost:8081", "http://localhost:3000"]' \
            -s baseUrl="http://localhost:8081"
        log_info "Created frontend client: $FRONTEND_CLIENT_ID"
    else
        log_info "Frontend client $FRONTEND_CLIENT_ID already exists."
    fi
    
    # Backend client (Service Account)
    log_info "Configuring backend client: $BACKEND_CLIENT_ID"
    if ! run_kcadm get clients -r "$REALM" -q clientId="$BACKEND_CLIENT_ID" | grep -q id; then
        run_kcadm create clients -r "$REALM" \
            -s clientId="$BACKEND_CLIENT_ID" \
            -s name="Backend Service Account Admin" \
            -s description="Service account for backend administration" \
            -s enabled=true \
            -s protocol=openid-connect \
            -s publicClient=false \
            -s secret="$BACKEND_CLIENT_SECRET" \
            -s serviceAccountsEnabled=true \
            -s standardFlowEnabled=false \
            -s implicitFlowEnabled=false \
            -s directAccessGrantsEnabled=false \
            -s authorizationServicesEnabled=false \
            -s bearerOnly=false \
            -s consentRequired=false \
            -s fullScopeAllowed=false \
            -s attributes.'client.secret.creation.time'=$(date +%s) \
            -s attributes.'backchannel.logout.session.required'=true
        log_info "Created backend client: $BACKEND_CLIENT_ID"
    else
        log_info "Backend client $BACKEND_CLIENT_ID already exists."
    fi
    
    # Wait for service account user to be created
    wait_for_service_account "$BACKEND_CLIENT_ID" || log_warning "Service account user might not be available yet"
    
    # -----------------------
    # 6. Protocol mappers (frontend client)
    # -----------------------
    FRONTEND_CLIENT_UUID=$(get_client_id "$FRONTEND_CLIENT_ID")
    if [ -n "$FRONTEND_CLIENT_UUID" ]; then
        log_info "Configuring protocol mappers for frontend client..."
        
        MAPPERS=(
        '{ "name": "filtered-roles", "protocol": "openid-connect", "protocolMapper": "oidc-usermodel-realm-role-mapper", "config": { "claim.name": "roles", "jsonType.label": "String", "multivalued": "true", "userinfo.token.claim": "true", "id.token.claim": "true", "access.token.claim": "true", "user.attribute": "", "claim.value": "" } }'
        '{ "name": "network_id", "protocol": "openid-connect", "protocolMapper": "oidc-usermodel-attribute-mapper", "config": { "user.attribute": "network_id", "claim.name": "network_id", "jsonType.label": "String", "id.token.claim": "true", "access.token.claim": "true", "userinfo.token.claim": "true" } }'
        '{ "name": "station_id", "protocol": "openid-connect", "protocolMapper": "oidc-usermodel-attribute-mapper", "config": { "user.attribute": "station_id", "claim.name": "station_id", "jsonType.label": "String", "id.token.claim": "true", "access.token.claim": "true", "userinfo.token.claim": "true" } }'
        '{ "name": "email_verified", "protocol": "openid-connect", "protocolMapper": "oidc-usermodel-attribute-mapper", "config": { "user.attribute": "emailVerified", "claim.name": "email_verified", "jsonType.label": "boolean", "id.token.claim": "true", "access.token.claim": "true", "userinfo.token.claim": "true" } }'
        )
        
        for M in "${MAPPERS[@]}"; do
            NAME=$(echo $M | grep -oP '"name":\s*"\K[^"]+')
            if ! run_kcadm get clients/$FRONTEND_CLIENT_UUID/protocol-mappers/models -r "$REALM" -q name="$NAME" | grep -q id; then
                run_kcadm create clients/$FRONTEND_CLIENT_UUID/protocol-mappers/models -r "$REALM" -f - <<< "$M"
                log_info "Created protocol mapper: $NAME"
            else
                log_info "Protocol mapper $NAME already exists."
            fi
        done
    else
        log_error "Could not find frontend client UUID for $FRONTEND_CLIENT_ID"
    fi
    
    # -----------------------
    # 7. Create admin user
    # -----------------------
    log_info "Creating admin user: $ADMIN_USER_USERNAME"
    if ! run_kcadm get users -r "$REALM" -q username="$ADMIN_USER_USERNAME" | grep -q id; then
        run_kcadm create users -r "$REALM" \
            -s username="$ADMIN_USER_USERNAME" \
            -s enabled=true \
            -s email="admin@charging.com" \
            -s emailVerified=true \
            -s firstName="Admin" \
            -s lastName="User" \
            -s attributes.locale="en"
        
        # Set password
        USER_ID=$(get_user_id "$ADMIN_USER_USERNAME")
        if [ -n "$USER_ID" ]; then
            run_kcadm update users/$USER_ID/reset-password -r "$REALM" \
                -s type="password" \
                -s value="$ADMIN_USER_PASSWORD" \
                -s temporary=false
            
            # Assign admin role
            run_kcadm add-roles -r "$REALM" \
                --uid "$USER_ID" \
                --rolename admin
            
            # Add to Admins group
            ADMIN_GROUP_ID=$(run_kcadm get groups -r "$REALM" -q name="Admins" --fields id --format csv | tail -n1 | tr -d '"')
            if [ -n "$ADMIN_GROUP_ID" ]; then
                run_kcadm update users/$USER_ID/groups/$ADMIN_GROUP_ID -r "$REALM"
            fi
            
            log_info "Created admin user: $ADMIN_USER_USERNAME with password: $ADMIN_USER_PASSWORD"
        fi
    else
        log_info "Admin user $ADMIN_USER_USERNAME already exists."
    fi
    
    # -----------------------
    # 8. Assign service account roles (backend-admin)
    # -----------------------
    log_info "Assigning roles to service account..."
    SA_USER_ID=$(get_user_id "service-account-$BACKEND_CLIENT_ID")
    REALM_MGMT_CLIENT_ID=$(get_client_id "realm-management")
    
    if [ -n "$SA_USER_ID" ] && [ -n "$REALM_MGMT_CLIENT_ID" ]; then
        ROLES=(
            "manage-users" "view-users" "query-users" "query-groups" "view-groups"
            "view-realm" "manage-realm" "manage-clients" "view-clients" "query-clients"
            "view-events" "view-identity-providers" "manage-identity-providers"
            "realm-admin" "create-client" "manage-authorization" "view-authorization"
        )
        
        for ROLE in "${ROLES[@]}"; do
            if ! run_kcadm get users/$SA_USER_ID/role-mappings/clients/$REALM_MGMT_CLIENT_ID -r "$REALM" | grep -q "\"name\":\"$ROLE\""; then
                run_kcadm add-roles -r "$REALM" \
                    --uid "$SA_USER_ID" \
                    --cclientid "$REALM_MGMT_CLIENT_ID" \
                    --rolename "$ROLE"
                log_info "Assigned role $ROLE to service account"
            else
                log_info "Role $ROLE already assigned to service account"
            fi
        done
        
        # Also assign realm admin role
        run_kcadm add-roles -r "$REALM" --uid "$SA_USER_ID" --rolename admin
        log_info "Assigned realm admin role to service account"
    else
        log_warning "Could not assign roles to service account. SA_USER_ID: $SA_USER_ID, REALM_MGMT_CLIENT_ID: $REALM_MGMT_CLIENT_ID"
    fi
    
    # -----------------------
    # 9. Configure realm settings
    # -----------------------
    log_info "Configuring realm settings..."
    run_kcadm update realms/$REALM \
        -s accessTokenLifespan=300 \
        -s ssoSessionIdleTimeout=1800 \
        -s ssoSessionMaxLifespan=36000 \
        -s offlineSessionIdleTimeout=2592000 \
        -s loginTimeout=1800 \
        -s defaultSignatureAlgorithm=RS256 \
        -s bruteForceProtected=true \
        -s permanentLockout=false \
        -s maxFailureWaitSeconds=900 \
        -s waitIncrementSeconds=60 \
        -s quickLoginCheckMilliSeconds=1000 \
        -s minimumQuickLoginWaitSeconds=60 \
        -s maxDeltaTimeSeconds=43200 \
        -s failureFactor=30
    
    # -----------------------
    # 10. Summary
    # -----------------------
    echo ""
    echo "========================================"
    echo "✅ KEYCLOAK INITIALIZATION COMPLETE"
    echo "========================================"
    echo "Realm:                    $REALM"
    echo "Keycloak URL:             $KEYCLOAK_URL"
    echo ""
    echo "--- Clients ---"
    echo "Frontend Client ID:       $FRONTEND_CLIENT_ID"
    echo "Backend Client ID:        $BACKEND_CLIENT_ID"
    echo "Backend Client Secret:    $BACKEND_CLIENT_SECRET"
    echo ""
    echo "--- Users ---"
    echo "Admin Username:           $ADMIN_USER_USERNAME"
    echo "Admin Password:           $ADMIN_USER_PASSWORD"
    echo ""
    echo "--- Endpoints ---"
    echo "OpenID Configuration:     $KEYCLOAK_URL/realms/$REALM/.well-known/openid-configuration"
    echo "========================================"
    echo ""
    log_info "Initialization script completed successfully"
}

# -----------------------
# Execution with error handling
# -----------------------
trap 'log_error "Script failed at line $LINENO"; exit 1' ERR

# Check if kcadm.sh exists
if [ ! -f "$KCADM" ]; then
    log_error "Keycloak CLI not found at $KCADM"
    exit 1
fi

# Make script executable
chmod +x "$KCADM" 2>/dev/null || true

# Run main function
main "$@"