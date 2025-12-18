CREATE TABLE IF NOT EXISTS system_info (
    id SERIAL PRIMARY KEY,
    key VARCHAR(255) UNIQUE NOT NULL,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Seed initial data
INSERT INTO system_info (key, value, description) 
VALUES ('maintenance_mode', 'false', 'Flag to disable API access');