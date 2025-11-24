import os
import json
from keycloak import KeycloakAdmin
from dotenv import load_dotenv

# Load .env file
load_dotenv('../.env')

KEYCLOAK_URL = f"http://localhost:{os.getenv('KEYCLOAK_HTTP_PORT')}"
KEYCLOAK_ADMIN = os.getenv('KEYCLOAK_ADMIN')
KEYCLOAK_ADMIN_PASSWORD = os.getenv('KEYCLOAK_ADMIN_PASSWORD')

# Connect to master realm
kc = KeycloakAdmin(
    server_url=KEYCLOAK_URL,
    username=KEYCLOAK_ADMIN,
    password=KEYCLOAK_ADMIN_PASSWORD,
    realm_name='master',
    verify=True
)

# Load config
with open("keycloak_config.json") as f:
    cfg = json.load(f)

realm_name = cfg["realm"]

# ----------------------------------------------------------
# 1. Create Realm
# ----------------------------------------------------------
realms = kc.get_realms()
existing = [r["realm"] for r in realms]

if realm_name not in existing:
    kc.create_realm(cfg)
    print(f"Realm '{realm_name}' created")
else:
    print(f"Realm '{realm_name}' already exists")

# Switch to the new realm
kc.realm_name = realm_name

# ----------------------------------------------------------
# 2. Create Roles
# ----------------------------------------------------------
if "roles" in cfg and "realm" in cfg["roles"]:
    for role in cfg["roles"]["realm"]:
        name = role["name"]
        try:
            kc.create_realm_role({"name": name})
            print(f"Role created: {name}")
        except:
            print(f"Role exists: {name}")

# ----------------------------------------------------------
# 3. Create Groups (Companies)
# ----------------------------------------------------------
if "groups" in cfg:
    for group in cfg["groups"]:
        try:
            kc.create_group(group)
            print(f"Group created: {group['name']}")
        except:
            print(f"Group exists: {group['name']}")

# ----------------------------------------------------------
# 4. Create Client + Protocol Mappers
# ----------------------------------------------------------
if "clients" in cfg:
    for c in cfg["clients"]:
        client_id = c["clientId"]

        # Create client if not exists
        try:
            kc.create_client(c)
            print(f"Client created: {client_id}")
        except:
            print(f"Client exists: {client_id}")

        # Retrieve internal id
        internal_id = kc.get_client_id(client_id)

        # Add protocol mappers
        if "protocolMappers" in c:
            for mapper in c["protocolMappers"]:
                try:
                    kc.create_client_protocol_mapper(internal_id, mapper)
                    print(f"Mapper created: {mapper['name']}")
                except:
                    print(f"Mapper exists: {mapper['name']}")

print("\n✔ Keycloak setup completed.")
