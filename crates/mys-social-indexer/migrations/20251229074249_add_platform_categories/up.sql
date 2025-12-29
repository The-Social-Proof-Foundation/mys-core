-- Migration: Add platform category columns
-- Version: 20251229074249
-- Purpose: Add primary_category (required) and secondary_category (optional) fields to platforms table

-- ============================================================================
-- 1. ADD CATEGORY COLUMNS
-- ============================================================================

-- Add primary_category column (required, NOT NULL)
ALTER TABLE platforms ADD COLUMN primary_category VARCHAR NOT NULL DEFAULT '';

-- Add secondary_category column (optional, nullable)
ALTER TABLE platforms ADD COLUMN secondary_category VARCHAR;

-- ============================================================================
-- 2. CREATE INDEXES
-- ============================================================================

-- Create index on primary_category for efficient filtering and searching
CREATE INDEX idx_platforms_primary_category ON platforms(primary_category);

-- Create index on secondary_category for filtering by secondary category
CREATE INDEX idx_platforms_secondary_category ON platforms(secondary_category);

-- ============================================================================
-- 3. REMOVE DEFAULT VALUE (after data migration if needed)
-- ============================================================================

-- Remove the default value now that column exists
-- Note: Since we're not doing backward compatibility, we'll keep the default empty string
-- for now, but new events will always provide a valid category
ALTER TABLE platforms ALTER COLUMN primary_category DROP DEFAULT;

