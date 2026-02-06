-- Migration: Create post_prediction_config table
-- Version: 20251113230301
-- Purpose: Create TimescaleDB hypertable to track global prediction configuration changes

-- ============================================================================
-- 1. CREATE POST PREDICTION CONFIG TABLE
-- ============================================================================

-- Post prediction config table - stores global prediction configuration
CREATE TABLE IF NOT EXISTS post_prediction_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    predictions_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    fee_bps BIGINT NOT NULL DEFAULT 0,
    treasury TEXT NOT NULL,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on updated_at for all rows
UPDATE post_prediction_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from updated_at for new rows
CREATE OR REPLACE FUNCTION update_post_prediction_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_post_prediction_config_time ON post_prediction_config;
CREATE TRIGGER set_post_prediction_config_time 
BEFORE INSERT ON post_prediction_config
FOR EACH ROW
EXECUTE FUNCTION update_post_prediction_config_time();

-- Convert to hypertable first
SELECT create_hypertable('post_prediction_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'post_prediction_config_pkey'
    ) THEN
        ALTER TABLE post_prediction_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint for latest config (one row per time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_post_prediction_config_time 
ON post_prediction_config(time DESC);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_post_prediction_config_updated_by ON post_prediction_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_post_prediction_config_predictions_enabled ON post_prediction_config(predictions_enabled, time);
CREATE INDEX IF NOT EXISTS idx_post_prediction_config_transaction_id ON post_prediction_config(transaction_id);

-- Enable compression on post_prediction_config table
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'post_prediction_config'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE post_prediction_config SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'post_prediction_config'
    ) THEN
        PERFORM add_compression_policy('post_prediction_config', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 2. CREATE VIEW FOR LATEST CONFIG
-- ============================================================================

-- Create a view to get the latest prediction configuration
CREATE OR REPLACE VIEW latest_post_prediction_config AS
SELECT DISTINCT ON (1)
    updated_by,
    predictions_enabled,
    fee_bps,
    treasury,
    updated_at,
    time,
    transaction_id
FROM post_prediction_config
ORDER BY 1, time DESC;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

-- Add table comment
COMMENT ON TABLE post_prediction_config IS 'Tracks global prediction configuration changes over time. Each row represents a configuration update.';

-- Add column comments
COMMENT ON COLUMN post_prediction_config.updated_by IS 'Address of the account that updated the configuration';
COMMENT ON COLUMN post_prediction_config.predictions_enabled IS 'Whether predictions are globally enabled';
COMMENT ON COLUMN post_prediction_config.fee_bps IS 'Prediction fee in basis points (1 bps = 0.01%)';
COMMENT ON COLUMN post_prediction_config.treasury IS 'Treasury address that receives prediction fees';
COMMENT ON COLUMN post_prediction_config.updated_at IS 'Unix timestamp in milliseconds when the configuration was updated';

