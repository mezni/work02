CREATE TABLE date_dimension (
    key_id SERIAL PRIMARY KEY,
    date_value DATE NOT NULL,
    year SMALLINT NOT NULL,
    quarter SMALLINT NOT NULL,
    month SMALLINT NOT NULL,
    day SMALLINT NOT NULL,
    day_of_week SMALLINT NOT NULL,
    day_of_year SMALLINT NOT NULL,
    week_of_year SMALLINT NOT NULL,
    fiscal_year SMALLINT NOT NULL,
    fiscal_quarter SMALLINT NOT NULL,
    fiscal_month SMALLINT NOT NULL,
    is_weekend BOOLEAN NOT NULL,
    is_holiday BOOLEAN NOT NULL    
);
CREATE INDEX idx_date_dimension_year ON date_dimension (year);
CREATE INDEX idx_date_dimension_quarter ON date_dimension (quarter);
CREATE INDEX idx_date_dimension_month ON date_dimension (month);
CREATE INDEX idx_date_dimension_day ON date_dimension (day);

CREATE TABLE client_dimension (
    key_id SERIAL PRIMARY KEY,
    name VARCHAR(50)
);
CREATE INDEX idx_client_dimension_name ON client_dimension (name);

CREATE TABLE provider_dimension (
    key_id SERIAL PRIMARY KEY,
    name VARCHAR(50)
);
CREATE INDEX idx_provider_dimension_name ON provider_dimension (name);

CREATE TABLE account_dimension (
    key_id SERIAL PRIMARY KEY,
    name VARCHAR(50),
    id VARCHAR(50)   
);
CREATE INDEX idx_account_dimension_name ON account_dimension (name);


CREATE TABLE costs (
    key_id SERIAL PRIMARY KEY,
    date_id INT REFERENCES date_dimension(key_id),
    client_id INT REFERENCES client_dimension(key_id),
    provider_id INT REFERENCES provider_dimension(key_id),
    account_id INT REFERENCES account_dimension(key_id),
    cost_usd DECIMAL(14, 6)
);

CREATE INDEX idx_costs_date_id ON costs (date_id);
CREATE INDEX idx_costs_client_id ON costs (client_id);
CREATE INDEX idx_costs_provider_id ON costs (provider_id);
CREATE INDEX idx_costs_account_id ON costs (account_id);