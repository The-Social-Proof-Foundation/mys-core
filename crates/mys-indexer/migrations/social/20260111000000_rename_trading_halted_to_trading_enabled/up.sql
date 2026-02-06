-- Rename trading_halted to trading_enabled and invert boolean values
-- This migration updates the schema to match the smart contract's positive logic
-- Note: Diesel migrations run in transactions automatically, so BEGIN/COMMIT are not needed

-- ============================================================================
-- 0. DROP FUNCTION (must be dropped before changing return type)
-- ============================================================================

-- Drop the function first since PostgreSQL doesn't allow changing return types
-- with CREATE OR REPLACE. We'll recreate it after table migrations complete.
DROP FUNCTION IF EXISTS get_current_exchange_config() CASCADE;

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

-- Check if social_proof_tokens_config table exists
DO $$
BEGIN
    -- If table doesn't exist, create it with the correct schema (trading_enabled)
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_schema = 'public' 
        AND table_name = 'social_proof_tokens_config'
    ) THEN
        -- Create table with trading_enabled (final desired state)
        CREATE TABLE social_proof_tokens_config (
            id SERIAL PRIMARY KEY,
            trading_enabled BOOLEAN NOT NULL DEFAULT true,
            admin_address TEXT NOT NULL,
            reason TEXT NOT NULL DEFAULT 'System initialized',
            timestamp_ms BIGINT NOT NULL DEFAULT 0,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            transaction_id TEXT NOT NULL,
            -- Enhanced constraints for production
            CONSTRAINT valid_timestamp CHECK (timestamp_ms >= 0),
            CONSTRAINT valid_admin_address CHECK (length(admin_address) > 0),
            CONSTRAINT valid_reason CHECK (length(reason) > 0 AND length(reason) <= 512),
            CONSTRAINT valid_transaction_id CHECK (length(transaction_id) > 0 AND length(transaction_id) <= 255)
        );

        -- Create index for trading_enabled
        CREATE INDEX IF NOT EXISTS idx_social_proof_tokens_config_trading_enabled 
        ON social_proof_tokens_config(trading_enabled);

        -- Create index for updated_at
        CREATE INDEX IF NOT EXISTS idx_social_proof_tokens_config_updated_at 
        ON social_proof_tokens_config(updated_at DESC);
    ELSE
        -- Table exists, check if it has trading_halted column (needs migration)
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_schema = 'public' 
            AND table_name = 'social_proof_tokens_config' 
            AND column_name = 'trading_halted'
        ) THEN
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

            -- Create index for trading_enabled if it doesn't exist
            CREATE INDEX IF NOT EXISTS idx_social_proof_tokens_config_trading_enabled 
            ON social_proof_tokens_config(trading_enabled);

            -- Drop old index if it exists
            DROP INDEX IF EXISTS idx_social_proof_tokens_config_trading_halted;
        END IF;
        -- If table exists but already has trading_enabled, do nothing (already migrated)
    END IF;
END $$;

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

