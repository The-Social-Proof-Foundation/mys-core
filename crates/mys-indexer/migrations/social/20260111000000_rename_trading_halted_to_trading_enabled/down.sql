-- Reverse migration: Rename trading_enabled back to trading_halted and invert boolean values
-- Note: Diesel migrations run in transactions automatically, so BEGIN/COMMIT are not needed

-- ============================================================================
-- 0. DROP FUNCTION (must be dropped before changing return type)
-- ============================================================================

-- Drop the function first since PostgreSQL doesn't allow changing return types
-- with CREATE OR REPLACE. We'll recreate it after table migrations complete.
DROP FUNCTION IF EXISTS get_current_exchange_config() CASCADE;

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

-- Check if social_proof_tokens_config table exists before attempting revert
DO $$
BEGIN
    -- Only revert if table exists and has trading_enabled column
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_schema = 'public' 
        AND table_name = 'social_proof_tokens_config'
    ) THEN
        -- Check if table has trading_enabled column (was migrated)
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_schema = 'public' 
            AND table_name = 'social_proof_tokens_config' 
            AND column_name = 'trading_enabled'
        ) THEN
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

            -- Restore old index
            DROP INDEX IF EXISTS idx_social_proof_tokens_config_trading_enabled;
            CREATE INDEX IF NOT EXISTS idx_social_proof_tokens_config_trading_halted 
            ON social_proof_tokens_config(trading_halted);
        END IF;
        -- If table exists but doesn't have trading_enabled, it was created fresh (skip revert)
    END IF;
    -- If table doesn't exist, skip revert (nothing to revert)
END $$;

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

