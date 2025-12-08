-- Migration: Add active_badge_id to Profiles
-- Version: 20251111031105
-- Purpose: Add active_badge_id field to track the currently active badge for a profile

-- ============================================================================
-- 1. ADD ACTIVE_BADGE_ID FIELD
-- ============================================================================

-- Add active_badge_id field to track the currently active badge
-- NULL means no active badge is set
-- References badge_id from profile_badges table
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS active_badge_id VARCHAR NULL;

-- ============================================================================
-- 2. INDEXING FOR PERFORMANCE
-- ============================================================================

-- Add index for querying profiles by active_badge_id (for badge lookups)
CREATE INDEX IF NOT EXISTS idx_profiles_active_badge_id ON profiles (active_badge_id) WHERE active_badge_id IS NOT NULL;

-- ============================================================================
-- 3. DOCUMENTATION
-- ============================================================================

-- Comment the column for documentation
COMMENT ON COLUMN profiles.active_badge_id IS 'The badge_id of the currently active badge for this profile. NULL means no active badge is set.';
