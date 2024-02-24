CREATE TABLE date_dim (
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
CREATE INDEX idx_date_dim_year ON date_dim (year);
CREATE INDEX idx_date_dim_quarter ON date_dim (quarter);
CREATE INDEX idx_date_dim_month ON date_dim (month);
CREATE INDEX idx_date_dim_day ON date_dim (day);

CREATE TABLE client_dim (
    key_id SERIAL PRIMARY KEY,
    client_name VARCHAR(50)
);
CREATE INDEX idx_client_dim_name ON client_dim (client_name);