-- Migration: Revert Post struct opt-in flags changes
-- Version: 20251230000000
-- Purpose: Remove the new columns added in the up migration

-- ============================================================================
-- REMOVE NEW COLUMNS
-- ============================================================================

-- Drop new columns if they exist
DROP INDEX IF EXISTS idx_posts_enable_spt;
DROP INDEX IF EXISTS idx_posts_enable_poc;
DROP INDEX IF EXISTS idx_posts_enable_spot;
DROP INDEX IF EXISTS idx_posts_spot_id;
DROP INDEX IF EXISTS idx_posts_spt_id;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'enable_spt'
    ) THEN
        ALTER TABLE posts DROP COLUMN enable_spt;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'enable_poc'
    ) THEN
        ALTER TABLE posts DROP COLUMN enable_poc;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'enable_spot'
    ) THEN
        ALTER TABLE posts DROP COLUMN enable_spot;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'spot_id'
    ) THEN
        ALTER TABLE posts DROP COLUMN spot_id;
    END IF;
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'spt_id'
    ) THEN
        ALTER TABLE posts DROP COLUMN spt_id;
    END IF;
END $$;
