-- organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- stations table
CREATE TABLE IF NOT EXISTS stations (
    id UUID PRIMARY KEY,
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    location TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    org_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    station_id UUID REFERENCES stations(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
