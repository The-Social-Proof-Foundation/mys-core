-- Migration: Add version column to post_config table
-- Version: 20260124010622
-- Purpose: Add version tracking to PostConfig following the same pattern as SpotConfig

-- ============================================================================
-- 1. ADD VERSION COLUMN
-- ============================================================================

-- Add version column to post_config table
ALTER TABLE post_config 
ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0;

-- Update existing rows to set version = 0 (initial version for existing configs)
UPDATE post_config SET version = 0 WHERE version IS NULL;

-- ============================================================================
-- 2. ADD COLUMN COMMENT
-- ============================================================================

COMMENT ON COLUMN post_config.version IS 'Version number of the PostConfig, incremented on each update';
