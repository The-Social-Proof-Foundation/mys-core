-- Migration: Rollback badge fields update
-- Version: 20251227022731
-- Purpose: Rollback changes - rename badge_media_url back to badge_image_url and remove badge_icon_url

-- ============================================================================
-- 1. REMOVE ICON_URL COLUMN
-- ============================================================================

-- Drop badge_icon_url column if it exists
ALTER TABLE profile_badges DROP COLUMN IF EXISTS badge_icon_url;

-- ============================================================================
-- 2. RENAME COLUMN BACK
-- ============================================================================

-- Rename badge_media_url back to badge_image_url if badge_media_url exists
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_media_url'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_image_url'
    ) THEN
        ALTER TABLE profile_badges RENAME COLUMN badge_media_url TO badge_image_url;
    END IF;
END $$;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION BACK
-- ============================================================================

-- Update column comment back
COMMENT ON COLUMN profile_badges.badge_image_url IS 'Image URL for the badge';

