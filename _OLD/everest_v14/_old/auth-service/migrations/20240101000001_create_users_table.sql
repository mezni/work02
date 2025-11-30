-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    keycloak_id VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL UNIQUE,
    role VARCHAR(50) NOT NULL CHECK (role IN ('admin', 'partner', 'operator')),
    organisation_name VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_organisation_name ON users(organisation_name);
CREATE INDEX idx_users_created_at ON users(created_at);

-- Add constraint: operators must have an organisation
CREATE OR REPLACE FUNCTION check_operator_organisation()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.role = 'operator' AND NEW.organisation_name IS NULL THEN
        RAISE EXCEPTION 'Operators must belong to an organisation';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_check_operator_organisation
    BEFORE INSERT OR UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION check_operator_organisation();

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE users IS 'User accounts with role-based access control';
COMMENT ON COLUMN users.id IS 'Internal UUID primary key';
COMMENT ON COLUMN users.keycloak_id IS 'Keycloak user ID (sub claim from JWT)';
COMMENT ON COLUMN users.email IS 'User email address';
COMMENT ON COLUMN users.username IS 'User username';
COMMENT ON COLUMN users.role IS 'User role: admin, partner, or operator';
COMMENT ON COLUMN users.organisation_name IS 'Organisation name for partner and operator roles';
COMMENT ON COLUMN users.is_active IS 'Whether the user account is active';
COMMENT ON COLUMN users.created_at IS 'Timestamp when user was created';
COMMENT ON COLUMN users.updated_at IS 'Timestamp when user was last updated';