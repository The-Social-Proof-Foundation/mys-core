-- Create ecosystem_treasury table for unified treasury tracking
-- This table tracks all treasury address updates across the ecosystem

CREATE TABLE IF NOT EXISTS ecosystem_treasury (
    id SERIAL,
    treasury_address VARCHAR NOT NULL,
    updated_by VARCHAR NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    transaction_id VARCHAR NOT NULL,
    PRIMARY KEY (id, time)
);

-- Convert to TimescaleDB hypertable
SELECT create_hypertable('ecosystem_treasury', 'time', if_not_exists => TRUE);

-- Create index for efficient time-based queries
CREATE INDEX IF NOT EXISTS idx_ecosystem_treasury_time ON ecosystem_treasury(time DESC);

-- Remove ecosystem_treasury column from spt_exchange_config table
-- Treasury is now tracked separately in ecosystem_treasury table
ALTER TABLE spt_exchange_config DROP COLUMN IF EXISTS ecosystem_treasury;

