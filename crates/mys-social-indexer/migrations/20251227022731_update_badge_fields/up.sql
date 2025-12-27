-- Migration: Update badge fields - rename image_url to media_url and add icon_url
-- Version: 20251227022731
-- Purpose: Update profile_badges table to match blockchain contract changes:
--          - Rename badge_image_url → badge_media_url
--          - Add badge_icon_url column

-- ============================================================================
-- 1. COPY DATA AND RENAME COLUMN
-- ============================================================================

-- First, copy existing badge_image_url values to badge_media_url if badge_media_url doesn't exist
DO $$
BEGIN
    -- Check if badge_media_url column exists
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_media_url'
    ) THEN
        -- Add badge_media_url column
        ALTER TABLE profile_badges ADD COLUMN badge_media_url TEXT;
        
        -- Copy data from badge_image_url to badge_media_url if badge_image_url exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profile_badges' 
            AND column_name = 'badge_image_url'
        ) THEN
            UPDATE profile_badges SET badge_media_url = badge_image_url WHERE badge_image_url IS NOT NULL;
        END IF;
        
        -- Rename badge_image_url to badge_media_url if it exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profile_badges' 
            AND column_name = 'badge_image_url'
        ) THEN
            ALTER TABLE profile_badges RENAME COLUMN badge_image_url TO badge_media_url;
        END IF;
    END IF;
END $$;

-- ============================================================================
-- 2. ADD ICON_URL COLUMN
-- ============================================================================

-- Add badge_icon_url column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_icon_url'
    ) THEN
        ALTER TABLE profile_badges ADD COLUMN badge_icon_url TEXT;
    END IF;
END $$;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

-- Update column comments
COMMENT ON COLUMN profile_badges.badge_media_url IS 'Media URL for the badge (can be image, video, etc.) - rich media content for detailed views';
COMMENT ON COLUMN profile_badges.badge_icon_url IS 'Icon URL for the badge - small icon displayed next to username';

