-- Migration: Update poc_configuration table with max_reasoning_length and max_evidence_urls fields
-- Version: 20251227023815
-- Purpose: Add new configuration fields to match smart contract PoCConfig struct updates

-- ============================================================================
-- ADD NEW FIELDS TO POC CONFIGURATION TABLE
-- ============================================================================

-- Add max_reasoning_length column (maximum characters for reasoning text)
ALTER TABLE poc_configuration 
ADD COLUMN IF NOT EXISTS max_reasoning_length BIGINT NOT NULL DEFAULT 5000;

-- Add max_evidence_urls column (maximum number of evidence URLs allowed)
ALTER TABLE poc_configuration 
ADD COLUMN IF NOT EXISTS max_evidence_urls BIGINT NOT NULL DEFAULT 10;

-- Backfill existing records with default values (in case DEFAULT wasn't applied)
UPDATE poc_configuration 
SET max_reasoning_length = 5000 
WHERE max_reasoning_length IS NULL;

UPDATE poc_configuration 
SET max_evidence_urls = 10 
WHERE max_evidence_urls IS NULL;

-- Add comments for documentation
COMMENT ON COLUMN poc_configuration.max_reasoning_length IS 'Maximum characters allowed for reasoning text in PoC analysis';
COMMENT ON COLUMN poc_configuration.max_evidence_urls IS 'Maximum number of evidence URLs allowed in PoC analysis';

