-- Migration: Remove auto_pool_disabled field from posts table
-- Version: 20251113230300
-- Purpose: Rollback - Remove auto_pool_disabled column

-- ============================================================================
-- 1. DROP INDEX
-- ============================================================================

-- Drop the index first
DROP INDEX IF EXISTS idx_posts_auto_pool_disabled;

-- ============================================================================
-- 2. DROP COLUMN
-- ============================================================================

-- Remove the auto_pool_disabled column
ALTER TABLE posts DROP COLUMN IF EXISTS auto_pool_disabled;

