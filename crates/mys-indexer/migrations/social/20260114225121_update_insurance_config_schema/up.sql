-- Migration: Update insurance_config schema
-- Version: 20260114225121
-- Purpose: Update insurance_config table to match new Move contract structure:
--   - Rename paused to enable_flag (invert values since semantics are opposite)
--   - Remove treasury field (treasury is now managed separately via EcosystemTreasury)

-- ============================================================================
-- 1. RENAME COLUMN AND INVERT VALUES
-- ============================================================================

-- Add new enable_flag column
ALTER TABLE insurance_config 
ADD COLUMN IF NOT EXISTS enable_flag BOOLEAN NOT NULL DEFAULT TRUE;

-- Migrate data: invert paused values (paused=true means enable_flag=false)
UPDATE insurance_config 
SET enable_flag = NOT paused;

-- Drop the old paused column
ALTER TABLE insurance_config DROP COLUMN IF EXISTS paused;

-- ============================================================================
-- 2. REMOVE TREASURY COLUMN
-- ============================================================================

-- Drop treasury column (treasury is now managed separately)
ALTER TABLE insurance_config DROP COLUMN IF EXISTS treasury;

-- ============================================================================
-- 3. ADD COMMENTS
-- ============================================================================

COMMENT ON COLUMN insurance_config.enable_flag IS 'Flag indicating if insurance is enabled (replaces paused field with inverted semantics)';
