#!/bin/bash
set -e

# Use the environment variable with a default fallback
SCHEMA_NAME="${KEYCLOAK_DB_SCHEMA:-auth_domain}"

echo "Initializing database schema: $SCHEMA_NAME"

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    -- Create dedicated schema for Keycloak
    CREATE SCHEMA IF NOT EXISTS $SCHEMA_NAME;
    
    -- Grant permissions
    GRANT ALL ON SCHEMA $SCHEMA_NAME TO $POSTGRES_USER;
    
    -- Set default schema for the user
    ALTER USER $POSTGRES_USER SET search_path TO $SCHEMA_NAME;
    
    -- Create extensions if needed
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
    
    -- Verify schema creation
    SELECT 'Schema $SCHEMA_NAME created successfully' AS status;
EOSQL

echo "Database initialization completed successfully!"