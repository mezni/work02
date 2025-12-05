-- ============================
-- EV Charging Stations Schema
-- ============================

-- 1. Networks
CREATE TABLE networks (
    network_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(20) NOT NULL CHECK (type IN ('personal','business')),
    address VARCHAR(255),
    city VARCHAR(50),
    state VARCHAR(50),
    country VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 2. Stations
CREATE TABLE stations (
    station_id SERIAL PRIMARY KEY,
    network_id INT NOT NULL REFERENCES networks(network_id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(50),
    state VARCHAR(50),
    country VARCHAR(50) NOT NULL,
    latitude DECIMAL(9,6),
    longitude DECIMAL(9,6),
    total_ports INT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 3. Employees
CREATE TABLE employees (
    employee_id SERIAL PRIMARY KEY,
    network_id INT NOT NULL REFERENCES networks(network_id) ON DELETE CASCADE,
    station_id INT REFERENCES stations(station_id) ON DELETE SET NULL,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(20),
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 4. Chargers
CREATE TABLE chargers (
    charger_id SERIAL PRIMARY KEY,
    station_id INT NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL, -- e.g., AC, DC fast, Tesla, CCS, CHAdeMO
    power_kw DECIMAL(5,2) NOT NULL,
    port_number INT NOT NULL,
    status VARCHAR(20) DEFAULT 'available',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 5. Station Reviews
CREATE TABLE station_reviews (
    review_id SERIAL PRIMARY KEY,
    station_id INT NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    reviewer_name VARCHAR(100) NOT NULL,
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- ============================
-- Indexes (optional but recommended)
-- ============================
CREATE INDEX idx_stations_network_id ON stations(network_id);
CREATE INDEX idx_employees_network_id ON employees(network_id);
CREATE INDEX idx_employees_station_id ON employees(station_id);
CREATE INDEX idx_chargers_station_id ON chargers(station_id);
CREATE INDEX idx_reviews_station_id ON station_reviews(station_id);


-- ============================
-- Sample Data for EV Charging Stations
-- ============================

-- 1. Networks
INSERT INTO networks (name, type, address, city, state, country)
VALUES
('John Doe', 'personal', '123 Main St', 'Toronto', 'ON', 'Canada'),
('GreenCharge Inc.', 'business', '456 Greenway Blvd', 'Vancouver', 'BC', 'Canada');

-- 2. Stations
INSERT INTO stations (network_id, name, address, city, state, country, latitude, longitude, total_ports)
VALUES
(1, 'Home Charging Station', '123 Main St', 'Toronto', 'ON', 'Canada', 43.65107, -79.347015, 2),
(2, 'Downtown Charging Hub', '789 Downtown Rd', 'Vancouver', 'BC', 'Canada', 49.2827, -123.1207, 10),
(2, 'Mall Parking Station', '1010 Mall St', 'Vancouver', 'BC', 'Canada', 49.2819, -123.1234, 8);

-- 3. Employees
INSERT INTO employees (network_id, station_id, name, email, phone, password_hash, role)
VALUES
(2, NULL, 'Alice Smith', 'alice@greencharge.com', '604-555-0101', 'hash_alice', 'admin'),
(2, 2, 'Bob Johnson', 'bob@greencharge.com', '604-555-0102', 'hash_bob', 'operator'),
(2, 3, 'Carol Lee', 'carol@greencharge.com', '604-555-0103', 'hash_carol', 'operator');

-- 4. Chargers
INSERT INTO chargers (station_id, type, power_kw, port_number, status)
VALUES
(1, 'AC Level 2', 7.2, 1, 'available'),
(1, 'AC Level 2', 7.2, 2, 'available'),
(2, 'DC Fast', 50, 1, 'available'),
(2, 'DC Fast', 50, 2, 'in_use'),
(2, 'CCS', 22, 3, 'available'),
(3, 'AC Level 2', 7.2, 1, 'available');

-- 5. Station Reviews
INSERT INTO station_reviews (station_id, reviewer_name, rating, comment)
VALUES
(1, 'Emily Brown', 5, 'Very convenient and easy to use.'),
(2, 'Michael Green', 4, 'Fast chargers, but parking is limited.'),
(2, 'Sophia White', 3, 'Chargers work well, but station is crowded at peak hours.'),
(3, 'Liam Black', 4, 'Good location and clean facilities.');
