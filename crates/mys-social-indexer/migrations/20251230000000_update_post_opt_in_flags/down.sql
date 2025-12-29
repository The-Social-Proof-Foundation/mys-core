-- Migration: Revert Post struct opt-in flags changes
-- Version: 20251230000000
-- Purpose: Reverse the migration - restore auto_pool_disabled, rename poc_id back to poc_badge_id, remove new columns

-- ============================================================================
-- 1. RESTORE auto_pool_disabled COLUMN
-- ============================================================================

ALTER TABLE posts ADD COLUMN auto_pool_disabled BOOLEAN NOT NULL DEFAULT false;

-- ============================================================================
-- 2. RENAME poc_id BACK TO poc_badge_id
-- ============================================================================

ALTER TABLE posts RENAME COLUMN poc_id TO poc_badge_id;

-- ============================================================================
-- 3. REMOVE NEW COLUMNS
-- ============================================================================

ALTER TABLE posts DROP COLUMN enable_spt;
ALTER TABLE posts DROP COLUMN enable_poc;
ALTER TABLE posts DROP COLUMN enable_spot;
ALTER TABLE posts DROP COLUMN spot_id;
ALTER TABLE posts DROP COLUMN spt_id;

-- ============================================================================
-- 4. RESTORE INDEXES
-- ============================================================================

ALTER INDEX idx_posts_poc_id RENAME TO idx_posts_poc_badge_id;
DROP INDEX idx_posts_enable_spt;
DROP INDEX idx_posts_enable_poc;
DROP INDEX idx_posts_enable_spot;
DROP INDEX idx_posts_spot_id;
DROP INDEX idx_posts_spt_id;
CREATE INDEX idx_posts_auto_pool_disabled ON posts(auto_pool_disabled, time) WHERE auto_pool_disabled = TRUE;
