-- Migration: Rename active_badge_id to selected_badge_id
-- Version: 20251113040000
-- Purpose: Rename active_badge_id column to selected_badge_id for consistency with API naming

-- ============================================================================
-- 1. RENAME COLUMN
-- ============================================================================

-- Rename the column from active_badge_id to selected_badge_id (only if active_badge_id exists and selected_badge_id doesn't)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'active_badge_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'selected_badge_id'
    ) THEN
        ALTER TABLE profiles RENAME COLUMN active_badge_id TO selected_badge_id;
    END IF;
END $$;

-- ============================================================================
-- 2. UPDATE INDEX
-- ============================================================================

-- Drop old index
DROP INDEX IF EXISTS idx_profiles_active_badge_id;

-- Create new index with updated name
CREATE INDEX IF NOT EXISTS idx_profiles_selected_badge_id ON profiles (selected_badge_id) WHERE selected_badge_id IS NOT NULL;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

-- Update column comment
COMMENT ON COLUMN profiles.selected_badge_id IS 'The badge_id of the currently selected badge for this profile. NULL means no badge is selected. If None, the first badge in the badges vector should be displayed.';

