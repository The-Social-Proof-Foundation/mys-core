-- Migration: Add auto_pool_disabled field to posts table
-- Version: 20251113230300
-- Purpose: Add auto_pool_disabled boolean field to track per-post auto pool disabled status

-- ============================================================================
-- 1. ADD COLUMN
-- ============================================================================

-- Add auto_pool_disabled column to posts table
-- Defaults to FALSE (auto pool enabled by default)
ALTER TABLE posts ADD COLUMN IF NOT EXISTS auto_pool_disabled BOOLEAN NOT NULL DEFAULT FALSE;

-- ============================================================================
-- 2. CREATE INDEX
-- ============================================================================

-- Create index for filtering posts by auto_pool_disabled status
CREATE INDEX IF NOT EXISTS idx_posts_auto_pool_disabled ON posts(auto_pool_disabled, time) WHERE auto_pool_disabled = TRUE;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

-- Add column comment
COMMENT ON COLUMN posts.auto_pool_disabled IS 'Whether auto pool creation is disabled for this post. FALSE means auto pool is enabled (default), TRUE means disabled.';

