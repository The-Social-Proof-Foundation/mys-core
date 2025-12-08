-- Migration: Create spot_config table
-- Version: 20251115000000
-- Purpose: Create TimescaleDB hypertable to track global SPoT configuration changes

-- ============================================================================
-- 1. CREATE SPOT CONFIG TABLE
-- ============================================================================

-- Spot config table - stores global SPoT configuration
CREATE TABLE IF NOT EXISTS spot_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    enable_flag BOOLEAN NOT NULL DEFAULT TRUE,
    confidence_threshold_bps BIGINT NOT NULL DEFAULT 0,
    resolution_window_epochs BIGINT NOT NULL DEFAULT 0,
    max_resolution_window_epochs BIGINT NOT NULL DEFAULT 0,
    payout_delay_epochs BIGINT NOT NULL DEFAULT 0,
    fee_bps BIGINT NOT NULL DEFAULT 0,
    fee_split_bps_platform BIGINT NOT NULL DEFAULT 0,
    platform_treasury TEXT NOT NULL,
    chain_treasury TEXT NOT NULL,
    oracle_address TEXT NOT NULL,
    max_single_bet BIGINT NOT NULL DEFAULT 0,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on timestamp_ms for all rows
UPDATE spot_config SET time = to_timestamp(timestamp_ms / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from timestamp_ms for new rows
CREATE OR REPLACE FUNCTION update_spot_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_spot_config_time ON spot_config;
CREATE TRIGGER set_spot_config_time 
BEFORE INSERT ON spot_config
FOR EACH ROW
EXECUTE FUNCTION update_spot_config_time();

-- Convert to hypertable first
SELECT create_hypertable('spot_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'spot_config_pkey'
    ) THEN
        ALTER TABLE spot_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint for latest config (one row per time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_spot_config_time 
ON spot_config(time DESC);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_spot_config_updated_by ON spot_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_spot_config_enable_flag ON spot_config(enable_flag, time);
CREATE INDEX IF NOT EXISTS idx_spot_config_transaction_id ON spot_config(transaction_id);

