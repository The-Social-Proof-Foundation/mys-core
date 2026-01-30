-- Migration: Update SPoT tables for multi-option betting system
-- Replaces binary YES/NO with multi-option system (option_id) and JSONB escrow tracking

-- Step 1: Add option_id column to spot_bets table (nullable initially)
ALTER TABLE spot_bets ADD COLUMN IF NOT EXISTS option_id SMALLINT;

-- Step 2: Migrate existing data: is_yes = true -> option_id = 0, is_yes = false -> option_id = 1
UPDATE spot_bets SET option_id = CASE WHEN is_yes THEN 0 ELSE 1 END WHERE option_id IS NULL;

-- Step 3: Make option_id NOT NULL
ALTER TABLE spot_bets ALTER COLUMN option_id SET NOT NULL;

-- Step 4: Drop is_yes column from spot_bets
ALTER TABLE spot_bets DROP COLUMN IF EXISTS is_yes;

-- Step 5: Add new columns to spot_records table
ALTER TABLE spot_records ADD COLUMN IF NOT EXISTS betting_options JSONB;
ALTER TABLE spot_records ADD COLUMN IF NOT EXISTS option_escrow JSONB;
ALTER TABLE spot_records ADD COLUMN IF NOT EXISTS resolution_window_epochs BIGINT;
ALTER TABLE spot_records ADD COLUMN IF NOT EXISTS max_resolution_window_epochs BIGINT;

-- Step 6: Migrate existing data in spot_records
-- Initialize betting_options to ["Yes", "No"] for existing records
UPDATE spot_records 
SET betting_options = '["Yes", "No"]'::jsonb 
WHERE betting_options IS NULL;

-- Convert total_yes_escrow/total_no_escrow to option_escrow JSONB format
UPDATE spot_records 
SET option_escrow = jsonb_build_object(
    '0', COALESCE(total_yes_escrow, 0),
    '1', COALESCE(total_no_escrow, 0)
)
WHERE option_escrow IS NULL;

-- Step 7: Make betting_options and option_escrow NOT NULL (after migration)
ALTER TABLE spot_records ALTER COLUMN betting_options SET NOT NULL;
ALTER TABLE spot_records ALTER COLUMN option_escrow SET NOT NULL;

-- Step 8: Drop old escrow columns from spot_records
ALTER TABLE spot_records DROP COLUMN IF EXISTS total_yes_escrow;
ALTER TABLE spot_records DROP COLUMN IF EXISTS total_no_escrow;

-- Step 9: Add option_id column to social_proof_of_truth table (nullable initially)
ALTER TABLE social_proof_of_truth ADD COLUMN IF NOT EXISTS option_id SMALLINT;

-- Step 10: Migrate existing data in social_proof_of_truth
UPDATE social_proof_of_truth 
SET option_id = CASE 
    WHEN is_yes = true THEN 0 
    WHEN is_yes = false THEN 1 
    ELSE NULL 
END 
WHERE option_id IS NULL;

-- Step 11: Drop is_yes column from social_proof_of_truth
ALTER TABLE social_proof_of_truth DROP COLUMN IF EXISTS is_yes;

-- Step 12: Create spot_bet_withdrawals hypertable
CREATE TABLE IF NOT EXISTS spot_bet_withdrawals (
    id SERIAL NOT NULL,
    post_id TEXT NOT NULL,
    user_address TEXT NOT NULL,
    option_id SMALLINT NOT NULL,
    amount BIGINT NOT NULL,
    fee_taken BIGINT NOT NULL,
    timestamp_epoch BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Convert to hypertable if TimescaleDB is available
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM create_hypertable('spot_bet_withdrawals', 'time', if_not_exists => TRUE, create_default_indexes => FALSE, chunk_time_interval => INTERVAL '7 days');
    END IF;
END $$;

-- Drop existing primary key if it exists
ALTER TABLE spot_bet_withdrawals DROP CONSTRAINT IF EXISTS spot_bet_withdrawals_pkey CASCADE;

-- Add primary key
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'spot_bet_withdrawals_pkey'
    ) THEN
        ALTER TABLE spot_bet_withdrawals ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_spot_bet_withdrawals_post_id ON spot_bet_withdrawals(post_id, time);
CREATE INDEX IF NOT EXISTS idx_spot_bet_withdrawals_user ON spot_bet_withdrawals(user_address, time);
CREATE INDEX IF NOT EXISTS idx_spot_bet_withdrawals_option_id ON spot_bet_withdrawals(option_id, time);
CREATE INDEX IF NOT EXISTS idx_spot_bet_withdrawals_created_at ON spot_bet_withdrawals(timestamp_epoch);

