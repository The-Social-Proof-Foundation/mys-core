-- Migration: Create mydata_config table
-- Version: 20251226175247
-- Purpose: Create TimescaleDB hypertable to track global MyData marketplace configuration changes

-- ============================================================================
-- 1. CREATE MYDATA CONFIG TABLE
-- ============================================================================

-- MyData config table - stores global MyData marketplace configuration
CREATE TABLE IF NOT EXISTS mydata_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    enable_flag BOOLEAN NOT NULL DEFAULT TRUE,
    max_tags BIGINT NOT NULL DEFAULT 0,
    max_subscription_days BIGINT NOT NULL DEFAULT 0,
    max_free_access_grants BIGINT NOT NULL DEFAULT 0,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on timestamp_ms for all rows
UPDATE mydata_config SET time = to_timestamp(timestamp_ms / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from timestamp_ms for new rows
CREATE OR REPLACE FUNCTION update_mydata_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_config_time ON mydata_config;
CREATE TRIGGER set_mydata_config_time 
BEFORE INSERT ON mydata_config
FOR EACH ROW
EXECUTE FUNCTION update_mydata_config_time();

-- Convert to hypertable first
SELECT create_hypertable('mydata_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'mydata_config_pkey'
    ) THEN
        ALTER TABLE mydata_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint for latest config (one row per time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_mydata_config_time 
ON mydata_config(time DESC);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_mydata_config_updated_by ON mydata_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_mydata_config_enable_flag ON mydata_config(enable_flag, time);
CREATE INDEX IF NOT EXISTS idx_mydata_config_transaction_id ON mydata_config(transaction_id);

