-- Migration: Rename selected_badge_id back to active_badge_id
-- Version: 20251113040000
-- Purpose: Rollback - Rename selected_badge_id column back to active_badge_id

-- ============================================================================
-- 1. RENAME COLUMN BACK
-- ============================================================================

-- Rename the column back from selected_badge_id to active_badge_id
ALTER TABLE profiles RENAME COLUMN selected_badge_id TO active_badge_id;

-- ============================================================================
-- 2. UPDATE INDEX
-- ============================================================================

-- Drop new index
DROP INDEX IF EXISTS idx_profiles_selected_badge_id;

-- Recreate old index
CREATE INDEX IF NOT EXISTS idx_profiles_active_badge_id ON profiles (active_badge_id) WHERE active_badge_id IS NOT NULL;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

-- Update column comment back
COMMENT ON COLUMN profiles.active_badge_id IS 'The badge_id of the currently active badge for this profile. NULL means no active badge is set.';

