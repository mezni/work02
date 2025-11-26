import os
from keycloak import KeycloakAdmin
from dotenv import load_dotenv

# Load .env
load_dotenv('../.env')

# Build Keycloak URL dynamically
KEYCLOAK_URL = f"http://localhost:{os.getenv('KEYCLOAK_HTTP_PORT')}"
KEYCLOAK_ADMIN = os.getenv('KEYCLOAK_ADMIN')
KEYCLOAK_ADMIN_PASSWORD = os.getenv('KEYCLOAK_ADMIN_PASSWORD')
REALM_NAME = os.getenv("REALM_NAME")
CLIENT_NAME = os.getenv("CLIENT_NAME")
ADMIN_USERNAME = os.getenv("ADMIN_USERNAME")
ADMIN_EMAIL = os.getenv("ADMIN_EMAIL")
ADMIN_PASSWORD = os.getenv("ADMIN_PASSWORD")

# Connect to master realm
kc = KeycloakAdmin(
    server_url=KEYCLOAK_URL,
    username=KEYCLOAK_ADMIN,
    password=KEYCLOAK_ADMIN_PASSWORD,
    realm_name='master',
    verify=True
)

# 1️⃣ Create realm if not exists
realms = [r['realm'] for r in kc.get_realms()]
if REALM_NAME not in realms:
    kc.create_realm({"realm": REALM_NAME, "enabled": True})
    print(f"Realm '{REALM_NAME}' created.")
else:
    print(f"Realm '{REALM_NAME}' already exists.")

# Switch to the new realm
kc.realm_name = REALM_NAME

# 2️⃣ Create client
clients = kc.get_clients()
client_ids = [c['clientId'] for c in clients]
if CLIENT_NAME not in client_ids:
    client_payload = {
        "clientId": CLIENT_NAME,
        "enabled": True,
        "directAccessGrantsEnabled": True,
        "publicClient": False,
        "protocol": "openid-connect"
    }
    kc.create_client(client_payload)
    print(f"Client '{CLIENT_NAME}' created.")
else:
    print(f"Client '{CLIENT_NAME}' already exists.")

# 3️⃣ Create client roles
client_id = kc.get_client_id(CLIENT_NAME)
roles = ["admin", "operator", "partner", "user", "guest"]
existing_roles = [r['name'] for r in kc.get_client_roles(client_id)]
for role in roles:
    if role not in existing_roles:
        kc.create_client_role(client_id, {"name": role})
        print(f"Role '{role}' created in client '{CLIENT_NAME}'.")
    else:
        print(f"Role '{role}' already exists in client '{CLIENT_NAME}'.")

# 4️⃣ Create initial admin user
users = kc.get_users()
if not any(u['username'] == ADMIN_USERNAME for u in users):
    user_payload = {
        "username": ADMIN_USERNAME,
        "email": ADMIN_EMAIL,
        "enabled": True,
        "credentials": [{"value": ADMIN_PASSWORD, "type": "password", "temporary": False}]
    }
    user_id = kc.create_user(user_payload)
    print(f"Admin user '{ADMIN_USERNAME}' created.")

    # Assign client role 'admin'
    kc.assign_client_role(user_id, client_id, [{"name": "admin"}])
    print(f"Role 'admin' assigned to user '{ADMIN_USERNAME}'.")
else:
    print(f"Admin user '{ADMIN_USERNAME}' already exists.")

#
# TEST user and company
#
COMPANIES=["neutrino"]
existing_groups = [g['name'] for g in kc.get_groups()]
for company in COMPANIES:
    company = company.strip()
    if company and company not in existing_groups:
        kc.create_group({"name": company})
        print(f"Company group '{company}' created.")
    elif company:
        print(f"Company group '{company}' already exists.")

def get_group_id_by_name(kc: KeycloakAdmin, group_name: str):
    groups = kc.get_groups()
    for group in groups:
        if group['name'] == group_name:
            return group['id']
    return None

# Example: assign user 'admin' to company 'neutrino'
user_id = kc.get_user_id("admin")
company_group_id = get_group_id_by_name(kc, "neutrino")

if company_group_id:
    kc.group_user_add(user_id, company_group_id)
    print("User 'admin' assigned to neutrino")
else:
    print("Company group 'neutrino' not found")

