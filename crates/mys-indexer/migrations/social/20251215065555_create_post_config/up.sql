-- Migration: Create post_config table
-- Version: 20251215065555
-- Purpose: Create TimescaleDB hypertable to track global PostConfig changes

-- ============================================================================
-- 1. CREATE POST CONFIG TABLE
-- ============================================================================

-- Post config table - stores global post configuration (PostAdminCap settings)
CREATE TABLE IF NOT EXISTS post_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    predictions_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    prediction_fee_bps BIGINT NOT NULL DEFAULT 0,
    prediction_treasury TEXT NOT NULL,
    max_content_length BIGINT NOT NULL DEFAULT 0,
    max_media_urls BIGINT NOT NULL DEFAULT 0,
    max_mentions BIGINT NOT NULL DEFAULT 0,
    max_metadata_size BIGINT NOT NULL DEFAULT 0,
    max_description_length BIGINT NOT NULL DEFAULT 0,
    max_reaction_length BIGINT NOT NULL DEFAULT 0,
    commenter_tip_percentage BIGINT NOT NULL DEFAULT 0,
    repost_tip_percentage BIGINT NOT NULL DEFAULT 0,
    max_prediction_options BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on updated_at for all rows
UPDATE post_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from updated_at for new rows
CREATE OR REPLACE FUNCTION update_post_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_post_config_time ON post_config;
CREATE TRIGGER set_post_config_time 
BEFORE INSERT ON post_config
FOR EACH ROW
EXECUTE FUNCTION update_post_config_time();

-- Convert to hypertable first
SELECT create_hypertable('post_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'post_config_pkey'
    ) THEN
        ALTER TABLE post_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create index for latest config lookup
CREATE INDEX IF NOT EXISTS idx_post_config_time 
ON post_config(time DESC);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_post_config_updated_by ON post_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_post_config_transaction_id ON post_config(transaction_id);

-- Enable compression on post_config table
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'post_config'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE post_config SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'post_config'
    ) THEN
        PERFORM add_compression_policy('post_config', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 2. ADD TABLE COMMENTS
-- ============================================================================

COMMENT ON TABLE post_config IS 'Tracks global post configuration changes over time (PostAdminCap settings). Each row represents a configuration update.';
COMMENT ON COLUMN post_config.updated_by IS 'Address of the account that updated the configuration';
COMMENT ON COLUMN post_config.predictions_enabled IS 'Whether predictions are globally enabled';
COMMENT ON COLUMN post_config.prediction_fee_bps IS 'Prediction fee in basis points (1 bps = 0.01%)';
COMMENT ON COLUMN post_config.prediction_treasury IS 'Treasury address that receives prediction fees';
COMMENT ON COLUMN post_config.max_content_length IS 'Maximum character length for post content';
COMMENT ON COLUMN post_config.max_media_urls IS 'Maximum number of media URLs per post';
COMMENT ON COLUMN post_config.max_mentions IS 'Maximum number of mentions in a post';
COMMENT ON COLUMN post_config.max_metadata_size IS 'Maximum size for post metadata in bytes';
COMMENT ON COLUMN post_config.max_description_length IS 'Maximum length for report descriptions';
COMMENT ON COLUMN post_config.max_reaction_length IS 'Maximum length for reactions';
COMMENT ON COLUMN post_config.commenter_tip_percentage IS 'Percentage of tip that goes to commenter (remainder to post owner)';
COMMENT ON COLUMN post_config.repost_tip_percentage IS 'Percentage of tip that goes to reposter (remainder to original post owner)';
COMMENT ON COLUMN post_config.max_prediction_options IS 'Maximum number of prediction options';

