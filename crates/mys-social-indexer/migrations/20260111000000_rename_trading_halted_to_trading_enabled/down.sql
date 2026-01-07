-- Reverse migration: Rename trading_enabled back to trading_halted and invert boolean values

BEGIN;

-- ============================================================================
-- 1. REVERT spt_exchange_config TABLE
-- ============================================================================

-- Add old column trading_halted
ALTER TABLE spt_exchange_config 
ADD COLUMN trading_halted BOOLEAN;

-- Invert values back: trading_halted = NOT trading_enabled
UPDATE spt_exchange_config 
SET trading_halted = NOT trading_enabled;

-- Make trading_halted NOT NULL (after data migration)
ALTER TABLE spt_exchange_config 
ALTER COLUMN trading_halted SET NOT NULL;

-- Set default value
ALTER TABLE spt_exchange_config 
ALTER COLUMN trading_halted SET DEFAULT false;

-- Drop new column
ALTER TABLE spt_exchange_config 
DROP COLUMN trading_enabled;

-- Restore old index
DROP INDEX IF EXISTS idx_token_exchange_config_trading_enabled;
CREATE INDEX IF NOT EXISTS idx_token_exchange_config_trading_halted 
ON spt_exchange_config(trading_halted);

-- ============================================================================
-- 2. REVERT social_proof_tokens_config TABLE
-- ============================================================================

-- Add old column trading_halted
ALTER TABLE social_proof_tokens_config 
ADD COLUMN trading_halted BOOLEAN;

-- Invert values back: trading_halted = NOT trading_enabled
UPDATE social_proof_tokens_config 
SET trading_halted = NOT trading_enabled;

-- Make trading_halted NOT NULL (after data migration)
ALTER TABLE social_proof_tokens_config 
ALTER COLUMN trading_halted SET NOT NULL;

-- Set default value
ALTER TABLE social_proof_tokens_config 
ALTER COLUMN trading_halted SET DEFAULT false;

-- Drop new column
ALTER TABLE social_proof_tokens_config 
DROP COLUMN trading_enabled;

-- ============================================================================
-- 3. REVERT get_current_exchange_config() FUNCTION
-- ============================================================================

CREATE OR REPLACE FUNCTION get_current_exchange_config()
RETURNS TABLE(
    post_threshold BIGINT,
    profile_threshold BIGINT,
    max_individual_reservation_bps BIGINT,
    trading_halted BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.post_threshold,
        c.profile_threshold, 
        c.max_individual_reservation_bps,
        c.trading_halted
    FROM spt_exchange_config c
    ORDER BY c.time DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

COMMIT;

