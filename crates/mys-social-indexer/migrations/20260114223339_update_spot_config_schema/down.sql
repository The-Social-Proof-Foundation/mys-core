-- Migration: Revert spot_config schema changes
-- Version: 20260114223339
-- Purpose: Revert spot_config table to previous structure

-- ============================================================================
-- 1. ADD BACK OLD COLUMNS
-- ============================================================================

-- Add back payout_delay_epochs column
ALTER TABLE spot_config 
ADD COLUMN IF NOT EXISTS payout_delay_epochs BIGINT NOT NULL DEFAULT 0;

-- Add back platform_treasury column
ALTER TABLE spot_config 
ADD COLUMN IF NOT EXISTS platform_treasury TEXT NOT NULL DEFAULT '';

-- Add back chain_treasury column
ALTER TABLE spot_config 
ADD COLUMN IF NOT EXISTS chain_treasury TEXT NOT NULL DEFAULT '';

-- ============================================================================
-- 2. MIGRATE DATA BACK
-- ============================================================================

-- Convert payout_delay_ms back to payout_delay_epochs
-- NOTE: This conversion assumes 1 epoch = 86400000 milliseconds (24 hours)
-- Adjust the divisor if your epoch duration differs
UPDATE spot_config 
SET payout_delay_epochs = payout_delay_ms / 86400000
WHERE payout_delay_ms > 0;

-- For rows where payout_delay_ms is 0, payout_delay_epochs already defaults to 0

-- ============================================================================
-- 3. DROP NEW COLUMNS
-- ============================================================================

-- Drop payout_delay_ms column
ALTER TABLE spot_config DROP COLUMN IF EXISTS payout_delay_ms;

-- Drop version column
ALTER TABLE spot_config DROP COLUMN IF EXISTS version;
