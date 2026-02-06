-- Reverse migration: Restore post prediction markets functionality
-- This recreates the table structure and columns

-- ============================================================================
-- 1. RESTORE PREDICTION COLUMNS IN POST_CONFIG TABLE
-- ============================================================================

-- Add prediction-related columns back to post_config table
ALTER TABLE post_config ADD COLUMN IF NOT EXISTS predictions_enabled BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE post_config ADD COLUMN IF NOT EXISTS prediction_fee_bps BIGINT NOT NULL DEFAULT 0;
ALTER TABLE post_config ADD COLUMN IF NOT EXISTS prediction_treasury TEXT NOT NULL DEFAULT '';
ALTER TABLE post_config ADD COLUMN IF NOT EXISTS max_prediction_options BIGINT NOT NULL DEFAULT 0;

-- ============================================================================
-- 2. RESTORE POST PREDICTION CONFIG TABLE
-- ============================================================================

-- Recreate the table
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

-- Convert to hypertable
SELECT create_hypertable('post_prediction_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Add primary key
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'post_prediction_config_pkey'
    ) THEN
        ALTER TABLE post_prediction_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create indexes
CREATE UNIQUE INDEX IF NOT EXISTS idx_post_prediction_config_time 
ON post_prediction_config(time DESC);

CREATE INDEX IF NOT EXISTS idx_post_prediction_config_updated_by ON post_prediction_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_post_prediction_config_predictions_enabled ON post_prediction_config(predictions_enabled, time);
CREATE INDEX IF NOT EXISTS idx_post_prediction_config_transaction_id ON post_prediction_config(transaction_id);

-- Recreate the view
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

