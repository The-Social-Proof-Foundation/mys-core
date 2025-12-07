-- Migration: Update Profiles - Remove sensitive_data_updated_at and Add post_count
-- Version: 20250330000001
-- Purpose: Clean up profiles table by removing unused sensitive_data_updated_at field and adding post_count tracking

-- ============================================================================
-- 1. REMOVE SENSITIVE_DATA_UPDATED_AT FIELD
-- ============================================================================

-- Drop the sensitive_data_updated_at column as it's no longer needed
ALTER TABLE profiles DROP COLUMN IF EXISTS sensitive_data_updated_at;

-- ============================================================================
-- 2. ADD POST_COUNT FIELD
-- ============================================================================

-- Add post_count field to track number of top-level, non-deleted posts per profile
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS post_count INTEGER NOT NULL DEFAULT 0;

-- Add index on post_count for efficient sorting and filtering
CREATE INDEX IF NOT EXISTS idx_profiles_post_count ON profiles (post_count DESC);

-- Add composite index for common queries (owner_address + post_count)
CREATE INDEX IF NOT EXISTS idx_profiles_owner_post_count ON profiles (owner_address, post_count DESC);

-- ============================================================================
-- 3. VALIDATION AND CONSTRAINTS
-- ============================================================================

-- Add constraint to ensure post_count is never negative (only if it doesn't exist)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'chk_profiles_post_count_non_negative'
        AND conrelid = 'profiles'::regclass
    ) THEN
        ALTER TABLE profiles ADD CONSTRAINT chk_profiles_post_count_non_negative CHECK (post_count >= 0);
    END IF;
END $$;

-- Comment the column for documentation
COMMENT ON COLUMN profiles.post_count IS 'Number of top-level, non-deleted posts created by this profile. Updated synchronously with post creation/deletion events.'; 