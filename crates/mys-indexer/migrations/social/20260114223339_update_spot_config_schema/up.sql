-- Migration: Update spot_config schema
-- Version: 20260114223339
-- Purpose: Update spot_config table to match new Move contract structure:
--   - Change payout_delay_epochs to payout_delay_ms (milliseconds instead of epochs)
--   - Remove platform_treasury and chain_treasury fields
--   - Add version field

-- ============================================================================
-- 1. ADD NEW COLUMNS
-- ============================================================================

-- Add payout_delay_ms column (milliseconds)
ALTER TABLE spot_config 
ADD COLUMN IF NOT EXISTS payout_delay_ms BIGINT NOT NULL DEFAULT 0;

-- Add version column
ALTER TABLE spot_config 
ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0;

-- ============================================================================
-- 2. MIGRATE EXISTING DATA
-- ============================================================================

-- Convert payout_delay_epochs to payout_delay_ms
-- NOTE: This conversion assumes 1 epoch = 86400000 milliseconds (24 hours)
-- Adjust the multiplier if your epoch duration differs
-- Sui epochs are typically ~24 hours, but verify with your network configuration
UPDATE spot_config 
SET payout_delay_ms = payout_delay_epochs * 86400000
WHERE payout_delay_epochs > 0;

-- For rows where payout_delay_epochs is 0 or NULL, payout_delay_ms already defaults to 0

-- ============================================================================
-- 3. DROP OLD COLUMNS
-- ============================================================================

-- Drop payout_delay_epochs column
ALTER TABLE spot_config DROP COLUMN IF EXISTS payout_delay_epochs;

-- Drop platform_treasury column
ALTER TABLE spot_config DROP COLUMN IF EXISTS platform_treasury;

-- Drop chain_treasury column
ALTER TABLE spot_config DROP COLUMN IF EXISTS chain_treasury;

-- ============================================================================
-- 4. ADD COMMENTS
-- ============================================================================

COMMENT ON COLUMN spot_config.payout_delay_ms IS 'Payout delay in milliseconds (replaces payout_delay_epochs)';
COMMENT ON COLUMN spot_config.version IS 'Configuration version number';
