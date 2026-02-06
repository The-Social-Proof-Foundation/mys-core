-- Rollback: Remove platform category columns
-- Version: 20251229074249

-- ============================================================================
-- 1. DROP INDEXES
-- ============================================================================

DROP INDEX IF EXISTS idx_platforms_secondary_category;
DROP INDEX IF EXISTS idx_platforms_primary_category;

-- ============================================================================
-- 2. DROP COLUMNS
-- ============================================================================

ALTER TABLE platforms DROP COLUMN IF EXISTS secondary_category;
ALTER TABLE platforms DROP COLUMN IF EXISTS primary_category;

