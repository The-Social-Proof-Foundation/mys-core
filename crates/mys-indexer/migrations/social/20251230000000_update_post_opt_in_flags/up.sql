-- Migration: Update Post struct to use opt-in flags and linked object IDs
-- Version: 20251230000000
-- Purpose: Replace disable_auto_pool with three opt-in flags (enable_spt, enable_poc, enable_spot)
--          and add linked object address fields (spot_id, spt_id, poc_id)
--          Also rename poc_badge_id to poc_id

-- ============================================================================
-- 1. ADD NEW COLUMNS
-- ============================================================================

-- Add opt-in flag columns (default to false - opt-out by default)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'enable_spt'
    ) THEN
        ALTER TABLE posts ADD COLUMN enable_spt BOOLEAN NOT NULL DEFAULT false;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'enable_poc'
    ) THEN
        ALTER TABLE posts ADD COLUMN enable_poc BOOLEAN NOT NULL DEFAULT false;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'enable_spot'
    ) THEN
        ALTER TABLE posts ADD COLUMN enable_spot BOOLEAN NOT NULL DEFAULT false;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'spot_id'
    ) THEN
        ALTER TABLE posts ADD COLUMN spot_id TEXT NULL;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'spt_id'
    ) THEN
        ALTER TABLE posts ADD COLUMN spt_id TEXT NULL;
    END IF;
END $$;

-- ============================================================================
-- 2. RENAME poc_badge_id TO poc_id
-- ============================================================================

-- Rename poc_badge_id to poc_id if it exists
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'poc_badge_id'
    ) THEN
        ALTER TABLE posts RENAME COLUMN poc_badge_id TO poc_id;
    END IF;
END $$;

-- ============================================================================
-- 3. REMOVE OLD COLUMN
-- ============================================================================

-- Drop auto_pool_disabled column if it exists
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'auto_pool_disabled'
    ) THEN
        ALTER TABLE posts DROP COLUMN auto_pool_disabled;
    END IF;
END $$;

-- ============================================================================
-- 4. UPDATE INDEXES
-- ============================================================================

-- Rename poc_badge_id index to poc_id (if it exists)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE indexname = 'idx_posts_poc_badge_id'
    ) THEN
        ALTER INDEX idx_posts_poc_badge_id RENAME TO idx_posts_poc_id;
    END IF;
END $$;

-- Create index on poc_id if it doesn't exist (in case rename didn't happen)
CREATE INDEX IF NOT EXISTS idx_posts_poc_id ON posts(poc_id, time) WHERE poc_id IS NOT NULL;

-- Create indexes for new opt-in flags (partial indexes for efficiency)
CREATE INDEX IF NOT EXISTS idx_posts_enable_spt ON posts(enable_spt, time) WHERE enable_spt = true;
CREATE INDEX IF NOT EXISTS idx_posts_enable_poc ON posts(enable_poc, time) WHERE enable_poc = true;
CREATE INDEX IF NOT EXISTS idx_posts_enable_spot ON posts(enable_spot, time) WHERE enable_spot = true;

-- Create indexes for linked object IDs
CREATE INDEX IF NOT EXISTS idx_posts_spot_id ON posts(spot_id, time) WHERE spot_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_posts_spt_id ON posts(spt_id, time) WHERE spt_id IS NOT NULL;

-- Drop old auto_pool_disabled index (if it exists)
DROP INDEX IF EXISTS idx_posts_auto_pool_disabled;

-- ============================================================================
-- 5. UPDATE DOCUMENTATION
-- ============================================================================

COMMENT ON COLUMN posts.enable_spt IS 'Opt-in flag for Social Proof Tokens (SPT) auto-pool initialization';
COMMENT ON COLUMN posts.enable_poc IS 'Opt-in flag for Proof of Creativity (PoC) analysis and badges';
COMMENT ON COLUMN posts.enable_spot IS 'Opt-in flag for Social Proof of Truth (SPoT) prediction markets';
COMMENT ON COLUMN posts.spot_id IS 'Address of the SpotRecord object (set when a SPoT record is created)';
COMMENT ON COLUMN posts.spt_id IS 'Address of the TokenPool object (set when an SPT pool is created)';
COMMENT ON COLUMN posts.poc_id IS 'Address of the PoC record/badge for original content (renamed from poc_badge_id)';
