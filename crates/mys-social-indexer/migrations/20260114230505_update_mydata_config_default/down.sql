-- Migration: Revert default enable_flag to true for mydata_config and spot_config
-- Version: 20260114230505
-- Purpose: Revert the default value of enable_flag back to TRUE for mydata_config and spot_config tables

-- ============================================================================
-- 1. REVERT DEFAULT VALUES
-- ============================================================================

-- Change the default value for enable_flag column back to TRUE for mydata_config
ALTER TABLE mydata_config 
ALTER COLUMN enable_flag SET DEFAULT TRUE;

-- Change the default value for enable_flag column back to TRUE for spot_config
ALTER TABLE spot_config 
ALTER COLUMN enable_flag SET DEFAULT TRUE;
