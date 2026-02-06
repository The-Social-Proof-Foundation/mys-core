-- Migration: Remove max_votes_per_dispute field from poc_configuration table
-- Version: 20260122171440
-- Purpose: Rollback addition of max_votes_per_dispute field

-- ============================================================================
-- REMOVE max_votes_per_dispute FIELD FROM POC CONFIGURATION TABLE
-- ============================================================================

-- Remove max_votes_per_dispute column
ALTER TABLE poc_configuration 
DROP COLUMN IF EXISTS max_votes_per_dispute;
