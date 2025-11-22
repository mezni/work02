CREATE TABLE networks (
    network_id        UUID PRIMARY KEY,
    name              VARCHAR(255),  -- for companies
    network_type      VARCHAR(20) NOT NULL CHECK (network_type IN ('INDIVIDUAL', 'COMPANY')),
    support_phone     VARCHAR(50),
    support_email     VARCHAR(255),

    is_verified       BOOLEAN DEFAULT FALSE,
    is_active         BOOLEAN DEFAULT TRUE,
    is_live           BOOLEAN DEFAULT TRUE,

    created_by        UUID NOT NULL,
    updated_by        UUID,
    created_at        TIMESTAMP DEFAULT NOW(),
    updated_at        TIMESTAMP
);