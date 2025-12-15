-- Create users table
CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(32) PRIMARY KEY,
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    photo TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    network_id VARCHAR(32) NOT NULL DEFAULT 'X',
    station_id VARCHAR(32) NOT NULL DEFAULT 'X',
    source VARCHAR(20) NOT NULL DEFAULT 'web',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by VARCHAR(32),
    updated_by VARCHAR(32),
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'partner', 'operator')),
    CONSTRAINT valid_source CHECK (source IN ('web', 'internal'))
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_network_id ON users(network_id);
CREATE INDEX idx_users_station_id ON users(station_id);
CREATE INDEX idx_users_source ON users(source);
CREATE INDEX idx_users_created_at ON users(created_at DESC);