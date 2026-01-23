-- Migration: Remove oracle_address from poc_configuration table
-- Version: 20260122184013
-- Purpose: Rollback addition of oracle_address column

-- ============================================================================
-- 1. DROP INDEX
-- ============================================================================

DROP INDEX IF EXISTS idx_poc_config_oracle_address;

-- ============================================================================
-- 2. DROP COLUMN
-- ============================================================================

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'poc_configuration' AND column_name = 'oracle_address'
    ) THEN
        ALTER TABLE poc_configuration DROP COLUMN oracle_address;
    END IF;
END $$;
