CREATE TABLE date_dim
  (
     key_id         SERIAL PRIMARY KEY,
     date_value     DATE NOT NULL,
     year           SMALLINT NOT NULL,
     quarter        SMALLINT NOT NULL,
     month          SMALLINT NOT NULL,
     day            SMALLINT NOT NULL,
     day_of_week    SMALLINT NOT NULL,
     day_of_year    SMALLINT NOT NULL,
     week_of_year   SMALLINT NOT NULL,
     fiscal_year    SMALLINT NOT NULL,
     fiscal_quarter SMALLINT NOT NULL,
     fiscal_month   SMALLINT NOT NULL,
     is_weekend     BOOLEAN NOT NULL,
     is_holiday     BOOLEAN NOT NULL
  );

CREATE INDEX idx_date_dim_year
  ON date_dim (year);

CREATE INDEX idx_date_dim_quarter
  ON date_dim (quarter);

CREATE INDEX idx_date_dim_month
  ON date_dim (month);

CREATE INDEX idx_date_dim_day
  ON date_dim (day);

CREATE TABLE org_dim
  (
     key_id   SERIAL PRIMARY KEY,
     org_name VARCHAR(50)
  );

CREATE INDEX idx_org_dim_name
  ON org_dim (org_name);

CREATE TABLE provider_dim
  (
     key_id        SERIAL PRIMARY KEY,
     provider_name VARCHAR(50)
  );

CREATE INDEX idx_provider_dim_provider_name
  ON provider_dim (provider_name);

CREATE TABLE account_dim
  (
     key_id       SERIAL PRIMARY KEY,
     account_name VARCHAR(50),
     account_id   VARCHAR(50)
  );

CREATE INDEX idx_account_dim_account_name
  ON account_dim (account_name);

CREATE INDEX idx_account_dim_account_id
  ON account_dim (account_id);

CREATE TABLE service_dim
  (
     key_id       SERIAL PRIMARY KEY,
     service_name VARCHAR(50),
     service_id   VARCHAR(50)
  );

CREATE INDEX idx_service_dim_service_name
  ON service_dim (service_name);

CREATE INDEX idx_service_dim_service_id
  ON service_dim (service_id);

CREATE TABLE costs
  (
     key_id      SERIAL PRIMARY KEY,
     date_id     INT REFERENCES date_dim(key_id),
     org_id      INT REFERENCES org_dim(key_id),
     provider_id INT REFERENCES provider_dim(key_id),
     account_id  INT REFERENCES account_dim(key_id),
     service_id  INT REFERENCES service_dim(key_id),
     cost_usd    DECIMAL(14, 6)
  );

CREATE INDEX idx_costs_date_id
  ON costs (date_id);

CREATE INDEX idx_costs_org_id
  ON costs (org_id);

CREATE INDEX idx_costs_provider_id
  ON costs (provider_id);

CREATE INDEX idx_costs_account_id
  ON costs (account_id);

CREATE INDEX idx_costs_service_id
  ON costs (service_id);

CREATE OR replace VIEW costs_vw
AS
  SELECT od.org_name,
         ad,
         account_name,
         ad.account_id,
         c.cost_usd
  FROM   costs c,
         org_dim od,
         account_dim ad
  WHERE  1 = 1
         AND c.org_id = od.key_id
         AND c.account_id = ad.key_id; 