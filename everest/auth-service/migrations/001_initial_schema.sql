-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create organisations table
CREATE TABLE organisations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Create stations table
CREATE TABLE stations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    organisation_id UUID NOT NULL REFERENCES organisations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE(name, organisation_id)
);

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role INTEGER NOT NULL DEFAULT 1, -- 0=Public, 1=RegisteredUser, 2=Operator, 3=Partner, 4=Admin
    organisation_id UUID REFERENCES organisations(id) ON DELETE SET NULL,
    station_id UUID REFERENCES stations(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Create indexes for better performance
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_organisation_id ON users(organisation_id);
CREATE INDEX idx_users_station_id ON users(station_id);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_is_active ON users(is_active);

CREATE INDEX idx_stations_organisation_id ON stations(organisation_id);
CREATE INDEX idx_organisations_is_active ON organisations(is_active);

-- Insert sample data
INSERT INTO organisations (id, name, description) VALUES 
    ('11111111-1111-1111-1111-111111111111', 'Main Organisation', 'Primary organisation for the system'),
    ('22222222-2222-2222-2222-222222222222', 'Partner Org A', 'First partner organisation'),
    ('33333333-3333-3333-3333-333333333333', 'Partner Org B', 'Second partner organisation');

INSERT INTO stations (id, name, description, organisation_id) VALUES 
    ('44444444-4444-4444-4444-444444444444', 'Station Alpha', 'Main station', '11111111-1111-1111-1111-111111111111'),
    ('55555555-5555-5555-5555-555555555555', 'Station Beta', 'Secondary station', '11111111-1111-1111-1111-111111111111'),
    ('66666666-6666-6666-6666-666666666666', 'Station Gamma', 'Partner station', '22222222-2222-2222-2222-222222222222');

-- Create admin user (password: admin123)
INSERT INTO users (id, username, email, password_hash, role) VALUES 
    ('77777777-7777-7777-7777-777777777777', 'admin', 'admin@system.com', '$2b$12$r9CUH.ObWr6Qe6aR.1qQ.O9gokNZ5.Nq.y7.KoI1Q2.7Q2QZzQZJu', 4);
