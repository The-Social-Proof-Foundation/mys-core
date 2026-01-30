-- Migration: Update default enable_flag to false for mydata_config and spot_config
-- Version: 20260114230505
-- Purpose: Change the default value of enable_flag from TRUE to FALSE for mydata_config and spot_config tables

-- ============================================================================
-- 1. UPDATE DEFAULT VALUES
-- ============================================================================

-- Change the default value for enable_flag column from TRUE to FALSE for mydata_config
ALTER TABLE mydata_config 
ALTER COLUMN enable_flag SET DEFAULT FALSE;

-- Change the default value for enable_flag column from TRUE to FALSE for spot_config
ALTER TABLE spot_config 
ALTER COLUMN enable_flag SET DEFAULT FALSE;

-- ============================================================================
-- 2. ADD COMMENTS
-- ============================================================================

COMMENT ON COLUMN mydata_config.enable_flag IS 'Flag indicating if MyData marketplace is enabled (default: false)';
COMMENT ON COLUMN spot_config.enable_flag IS 'Flag indicating if Social Proof of Truth (SPoT) is enabled (default: false)';
