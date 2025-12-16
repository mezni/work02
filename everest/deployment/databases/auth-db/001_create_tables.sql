-- Users table with updated schema and soft delete
CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(32) PRIMARY KEY,           -- USR + nanoid
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    photo TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    network_id VARCHAR(32) NOT NULL DEFAULT '',
    station_id VARCHAR(32) NOT NULL DEFAULT '',
    source VARCHAR(20) NOT NULL DEFAULT 'web',   -- 'web' or 'internal'
    is_active BOOLEAN DEFAULT TRUE,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by VARCHAR(32),
    updated_by VARCHAR(32),
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'partner', 'operator')),
    CONSTRAINT valid_source CHECK (source IN ('web', 'internal')),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE) OR 
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);

-- Indexes for better query performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_is_active ON users(is_active);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_network_id ON users(network_id);
CREATE INDEX idx_users_station_id ON users(station_id);
CREATE INDEX idx_users_source ON users(source);
CREATE INDEX idx_users_deleted_at ON users(deleted_at);

CREATE UNIQUE INDEX idx_users_email_unique ON users(email)
WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX idx_users_username_unique ON users(username)
WHERE deleted_at IS NULL;

-- Add trigger to update updated_at timestamp and handle soft delete
CREATE OR REPLACE FUNCTION update_users_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    IF NEW.deleted_at IS NOT NULL THEN
        NEW.is_active = FALSE;
    END IF;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
FOR EACH ROW EXECUTE FUNCTION update_users_updated_at();