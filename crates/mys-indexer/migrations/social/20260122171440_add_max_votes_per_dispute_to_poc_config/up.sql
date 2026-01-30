-- Migration: Add max_votes_per_dispute field to poc_configuration table
-- Version: 20260122171440
-- Purpose: Add max_votes_per_dispute field to match smart contract PoCConfigUpdatedEvent

-- ============================================================================
-- ADD max_votes_per_dispute FIELD TO POC CONFIGURATION TABLE
-- ============================================================================

-- Add max_votes_per_dispute column (maximum votes allowed per dispute)
ALTER TABLE poc_configuration 
ADD COLUMN IF NOT EXISTS max_votes_per_dispute BIGINT NOT NULL DEFAULT 10000;

-- Backfill existing records with default value (in case DEFAULT wasn't applied)
UPDATE poc_configuration 
SET max_votes_per_dispute = 10000 
WHERE max_votes_per_dispute IS NULL;

-- Add comment for documentation
COMMENT ON COLUMN poc_configuration.max_votes_per_dispute IS 'Maximum number of votes allowed per PoC dispute (default: 10000)';
