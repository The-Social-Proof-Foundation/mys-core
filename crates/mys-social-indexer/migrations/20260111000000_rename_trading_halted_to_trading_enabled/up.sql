-- Rename trading_halted to trading_enabled and invert boolean values
-- This migration updates the schema to match the smart contract's positive logic

BEGIN;

-- ============================================================================
-- 1. UPDATE spt_exchange_config TABLE
-- ============================================================================

-- Add new column trading_enabled
ALTER TABLE spt_exchange_config 
ADD COLUMN trading_enabled BOOLEAN;

-- Invert values: trading_enabled = NOT trading_halted
UPDATE spt_exchange_config 
SET trading_enabled = NOT trading_halted;

-- Make trading_enabled NOT NULL (after data migration)
ALTER TABLE spt_exchange_config 
ALTER COLUMN trading_enabled SET NOT NULL;

-- Set default value
ALTER TABLE spt_exchange_config 
ALTER COLUMN trading_enabled SET DEFAULT true;

-- Drop old column
ALTER TABLE spt_exchange_config 
DROP COLUMN trading_halted;

-- Rename index
DROP INDEX IF EXISTS idx_token_exchange_config_trading_halted;
CREATE INDEX IF NOT EXISTS idx_token_exchange_config_trading_enabled 
ON spt_exchange_config(trading_enabled);

-- ============================================================================
-- 2. UPDATE social_proof_tokens_config TABLE
-- ============================================================================

-- Add new column trading_enabled
ALTER TABLE social_proof_tokens_config 
ADD COLUMN trading_enabled BOOLEAN;

-- Invert values: trading_enabled = NOT trading_halted
UPDATE social_proof_tokens_config 
SET trading_enabled = NOT trading_halted;

-- Make trading_enabled NOT NULL (after data migration)
ALTER TABLE social_proof_tokens_config 
ALTER COLUMN trading_enabled SET NOT NULL;

-- Set default value
ALTER TABLE social_proof_tokens_config 
ALTER COLUMN trading_enabled SET DEFAULT true;

-- Drop old column
ALTER TABLE social_proof_tokens_config 
DROP COLUMN trading_halted;

-- ============================================================================
-- 3. UPDATE get_current_exchange_config() FUNCTION
-- ============================================================================

CREATE OR REPLACE FUNCTION get_current_exchange_config()
RETURNS TABLE(
    post_threshold BIGINT,
    profile_threshold BIGINT,
    max_individual_reservation_bps BIGINT,
    trading_enabled BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.post_threshold,
        c.profile_threshold, 
        c.max_individual_reservation_bps,
        c.trading_enabled
    FROM spt_exchange_config c
    ORDER BY c.time DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

COMMIT;

