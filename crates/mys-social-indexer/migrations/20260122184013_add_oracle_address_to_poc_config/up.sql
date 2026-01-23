-- Migration: Add oracle_address to poc_configuration table
-- Version: 20260122184013
-- Purpose: Add oracle_address column to poc_configuration table to track the oracle address used for PoC analysis

-- ============================================================================
-- 1. ADD ORACLE_ADDRESS COLUMN
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'poc_configuration' AND column_name = 'oracle_address'
    ) THEN
        ALTER TABLE poc_configuration ADD COLUMN oracle_address TEXT NULL;
    END IF;
END $$;

-- ============================================================================
-- 2. CREATE INDEX FOR ORACLE_ADDRESS
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_poc_config_oracle_address ON poc_configuration (oracle_address, time DESC) WHERE oracle_address IS NOT NULL;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

COMMENT ON COLUMN poc_configuration.oracle_address IS 'Address of oracle used for PoC analysis verification';
