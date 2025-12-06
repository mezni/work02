-- ============================================================================
-- EV Charging Station - Full SQL Schema (Corrected)
-- ============================================================================

-- =========================
-- 1. NETWORKS
-- =========================
CREATE TABLE networks (
    network_id        VARCHAR(32) PRIMARY KEY,
    name              VARCHAR(255) NOT NULL,
    network_type      VARCHAR(20) NOT NULL CHECK (network_type IN ('INDIVIDUAL', 'COMPANY')),
    support_phone     VARCHAR(50),
    support_email     VARCHAR(255),

    is_verified       BOOLEAN DEFAULT FALSE,
    is_active         BOOLEAN DEFAULT TRUE,
    is_live           BOOLEAN DEFAULT TRUE,

    created_by        VARCHAR(32) NOT NULL,
    updated_by        VARCHAR(32),
    created_at        TIMESTAMPTZ DEFAULT NOW(),
    updated_at        TIMESTAMPTZ
);

CREATE INDEX idx_networks_name ON networks (name);

-- =========================
-- 2. STATIONS
-- =========================
CREATE TABLE stations (
    station_id          VARCHAR(32) PRIMARY KEY,
    network_id          VARCHAR(32) NOT NULL REFERENCES networks(network_id) ON DELETE CASCADE,

    name                VARCHAR(255) NOT NULL,
    address             TEXT,
    latitude            DOUBLE PRECISION,
    longitude           DOUBLE PRECISION,
    tags                JSONB,

    operational_status  VARCHAR(50) NOT NULL CHECK (
                            operational_status IN ('ACTIVE', 'MAINTENANCE', 'OUT_OF_SERVICE', 'COMMISSIONING')
                        ),
    verification_status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (
                            verification_status IN ('PENDING', 'VERIFIED', 'REJECTED')
                        ),

    is_live             BOOLEAN DEFAULT TRUE,
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

CREATE INDEX idx_stations_network ON stations (network_id);
CREATE INDEX idx_stations_name ON stations (name);

-- =========================
-- 3. CHARGER MODELS
-- =========================
CREATE TABLE charger_models (
    model_id            VARCHAR(32) PRIMARY KEY,
    manufacturer        VARCHAR(255),
    model_name          VARCHAR(255),
    capabilities        JSONB,        -- { "max_kw": 150, "dc": true }
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

-- =========================
-- 4. CHARGERS
-- =========================
CREATE TABLE chargers (
    charger_id          VARCHAR(32) PRIMARY KEY,
    station_id          VARCHAR(32) NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    serial_number       VARCHAR(255) UNIQUE,
    model_id            VARCHAR(32) REFERENCES charger_models(model_id) ON DELETE SET NULL,
    max_power_kw        NUMERIC(7,3),
    status              VARCHAR(50) NOT NULL DEFAULT 'AVAILABLE' CHECK (
                            status IN ('AVAILABLE', 'IN_SERVICE', 'FAULTED', 'OFFLINE', 'MAINTENANCE')
                        ),
    last_seen_at        TIMESTAMPTZ,
    tags                JSONB,

    is_live             BOOLEAN DEFAULT TRUE,
    is_active           BOOLEAN DEFAULT TRUE,
    is_verified         BOOLEAN DEFAULT FALSE,

    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

CREATE INDEX idx_chargers_station ON chargers (station_id);
CREATE INDEX idx_chargers_status ON chargers (status);

-- =========================
-- 5. CONNECTOR TYPES
-- =========================
CREATE TABLE connector_types (
    connector_type_id   INT PRIMARY KEY,
    name                VARCHAR(50) NOT NULL UNIQUE,
    description         TEXT,

    is_live             BOOLEAN DEFAULT TRUE,
    is_active           BOOLEAN DEFAULT TRUE,
    is_verified         BOOLEAN DEFAULT FALSE,

    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

-- =========================
-- 6. CONNECTORS
-- =========================
CREATE TABLE connectors (
    connector_id        VARCHAR(32) PRIMARY KEY,
    charger_id          VARCHAR(32) NOT NULL REFERENCES chargers(charger_id) ON DELETE CASCADE,
    station_id          VARCHAR(32) NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    connector_type_id   INT NOT NULL REFERENCES connector_types(connector_type_id),

    connector_index     INT NOT NULL,
    capacity_kw         NUMERIC(7,3),
    max_current_a       INT,

    operational_status  VARCHAR(50) NOT NULL CHECK (
                            operational_status IN ('AVAILABLE', 'CHARGING', 'FAULTY', 'RESERVED', 'OFFLINE')
                        ),
    verification_status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (
                            verification_status IN ('PENDING', 'VERIFIED', 'REJECTED')
                        ),

    tags                JSONB,

    is_live             BOOLEAN DEFAULT TRUE,
    is_active           BOOLEAN DEFAULT TRUE,
    is_verified         BOOLEAN DEFAULT FALSE,

    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ,

    CONSTRAINT uq_connector_per_charger UNIQUE (charger_id, connector_index)
);

-- =========================
-- 7. USERS
-- =========================
CREATE TABLE users (
    user_id             VARCHAR(32) PRIMARY KEY,
    network_id          VARCHAR(32) REFERENCES networks(network_id) ON DELETE SET NULL,
    station_id          VARCHAR(32) REFERENCES stations(station_id) ON DELETE SET NULL,
    username            TEXT,
    email               TEXT,
    created_by          VARCHAR(32),
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users (email);

-- =========================
-- 8. VEHICLES
-- =========================
CREATE TABLE vehicles (
    vehicle_id          VARCHAR(32) PRIMARY KEY,
    user_id             VARCHAR(32) REFERENCES users(user_id) ON DELETE CASCADE,
    make                VARCHAR(100),
    model               VARCHAR(100),
    year                SMALLINT,
    preferred_connector VARCHAR(50),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

CREATE INDEX idx_vehicles_user ON vehicles (user_id);

-- =========================
-- 9. USER REVIEWS
-- =========================
CREATE TABLE user_reviews (
    review_id          VARCHAR(32) PRIMARY KEY,
    user_id            VARCHAR(32) NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    station_id         VARCHAR(32) NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    connector_id       VARCHAR(32) REFERENCES connectors(connector_id) ON DELETE SET NULL,

    rating             SMALLINT NOT NULL CHECK (rating BETWEEN 1 AND 5),
    comment            TEXT,
    images             JSONB,
    wait_time_minutes  SMALLINT,
    session_success    BOOLEAN,

    is_verified        BOOLEAN DEFAULT FALSE,
    is_live            BOOLEAN DEFAULT TRUE,

    created_by         VARCHAR(32) NOT NULL,
    updated_by         VARCHAR(32),
    created_at         TIMESTAMPTZ DEFAULT NOW(),
    updated_at         TIMESTAMPTZ
);

INSERT INTO connector_types (connector_type_id, name, description, is_live, is_active, is_verified, created_by)
VALUES
    (1, 'TYPE2', 'Standard AC connector widely used in Europe', TRUE, TRUE, TRUE, 'system'),
    (2, 'CCS', 'Combined Charging System (DC fast charging)', TRUE, TRUE, TRUE, 'system'),
    (3, 'CHADEMO', 'DC fast charging protocol used by Japanese EVs', TRUE, TRUE, TRUE, 'system'),
    (4, 'NEMA1450', 'North American AC connector', TRUE, TRUE, FALSE, 'system'),
    (5, 'TESLA', 'Tesla proprietary connector', TRUE, TRUE, FALSE, 'system');


INSERT INTO charger_models (model_id, manufacturer, model_name, capabilities, created_by)
VALUES
    ('CM1001', 'ABB', 'Terra 54', '{"max_kw": 50, "dc": true}'::jsonb, 'system'),
    ('CM1002', 'Siemens', 'VersiCharge AC', '{"max_kw": 22, "ac": true}'::jsonb, 'system'),
    ('CM1003', 'Delta', 'DC Wallbox', '{"max_kw": 25, "dc": true}'::jsonb, 'system'),
    ('CM1004', 'EVBox', 'Troniq 100', '{"max_kw": 100, "dc": true}'::jsonb, 'system'),
    ('CM1005', 'Tesla', 'Supercharger V3', '{"max_kw": 250, "dc": true}'::jsonb, 'system');

INSERT INTO vehicles (vehicle_id, user_id, make, model, year, preferred_connector)
VALUES
    ('V1001', NULL, 'Tesla', 'Model 3', 2024, 'TESLA'),
    ('V1002', NULL, 'Nissan', 'Leaf', 2023, 'CHADEMO'),
    ('V1003', NULL, 'Hyundai', 'Ioniq 5', 2024, 'CCS'),
    ('V1004', NULL, 'Volkswagen', 'ID.4', 2024, 'CCS'),
    ('V1005', NULL, 'BMW', 'i4', 2024, 'CCS');
