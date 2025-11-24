import os
from keycloak import KeycloakAdmin
from dotenv import load_dotenv

# Load .env file
load_dotenv('../.env')

KEYCLOAK_URL = f"http://localhost:{os.getenv('KEYCLOAK_HTTP_PORT')}"
KEYCLOAK_ADMIN = os.getenv('KEYCLOAK_ADMIN')
KEYCLOAK_ADMIN_PASSWORD = os.getenv('KEYCLOAK_ADMIN_PASSWORD')

# Connect to Keycloak (master realm)
kc = KeycloakAdmin(
    server_url=KEYCLOAK_URL,
    username=KEYCLOAK_ADMIN,
    password=KEYCLOAK_ADMIN_PASSWORD,
    realm_name='master',
    verify=True
)

# Switch to your target realm
realm_name = "auth-domain"
kc.realm_name = realm_name

# ----------------------------
# 1. Create user dali
# ----------------------------
username = "dali3"
password = "Test123!"

user_payload = {
    "username": username,
    "enabled": False,
    "emailVerified": True,
    "firstName": "Dali",
    "lastName": "Ben",
    "email": "dali3@example.com",
    "credentials": [
        {
            "type": "password",
            "value": password,
            "temporary": False
        }
    ],
    "attributes": {
        "company_name": "CompanyA"
    }
}

try:
    user_id = kc.create_user(user_payload)
    print(f"User '{username}' created: {user_id}")
except Exception:
    # Get existing user ID
    users = kc.get_users(query={"username": username})
    user_id = users[0]["id"]
    print(f"User '{username}' already exists: {user_id}")

# ----------------------------
# 2. Assign user to CompanyA group
# ----------------------------
groups = kc.get_groups()
companyA_group = next((g for g in groups if g["name"] == "CompanyA"), None)

if not companyA_group:
    raise Exception("Group 'CompanyA' not found")

kc.group_user_add(user_id=user_id, group_id=companyA_group["id"])
print(f"User '{username}' added to group 'CompanyA'")

# ----------------------------
# 3. Assign role (optional)
# ----------------------------
role_name = "user"     # or admin/operator/partner/guest
role = kc.get_realm_role(role_name)

kc.assign_realm_roles(user_id, [role])
print(f"Role '{role_name}' assigned to '{username}'")

print("\n✔ User 'dali' successfully configured in CompanyA")
