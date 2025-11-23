CREATE TABLE IF NOT EXISTS networks (
    network_id UUID PRIMARY KEY,
    name VARCHAR(255),
    network_type VARCHAR(50) NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_live BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL,
    updated_by UUID,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ,
    
    -- Constraint for valid network types
    CONSTRAINT valid_network_type CHECK (network_type IN ('INDIVIDUAL', 'COMPANY'))
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_networks_network_type ON networks(network_type);