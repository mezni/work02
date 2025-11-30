cd ev-charging-platform

# Remove the existing file
rm config/default.toml

# Create it fresh with exact content
cat > config/default.toml << 'EOF'
[server]
host = "0.0.0.0"
port = 8080
log_level = "info"

[database]
host = "localhost"
port = 5432
username = "ev_charging"
password = "password"
database_name = "ev_charging_configurator"
max_connections = 20

[auth]
keycloak_url = "http://localhost:8080"
realm = "ev-charging"
client_id = "configurator-service"
jwt_leeway = 60

[cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
allowed_headers = ["*"]
EOF