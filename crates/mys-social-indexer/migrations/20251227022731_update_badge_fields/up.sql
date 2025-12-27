-- Migration: Update badge fields - rename image_url to media_url and add icon_url
-- Version: 20251227022731
-- Purpose: Update profile_badges table to match blockchain contract changes:
--          - Rename badge_image_url → badge_media_url
--          - Add badge_icon_url column

-- ============================================================================
-- 1. COPY DATA AND RENAME COLUMN
-- ============================================================================

-- Handle badge_image_url → badge_media_url rename/copy
DO $$
BEGIN
    -- Check if badge_media_url already exists
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_media_url'
    ) THEN
        -- badge_media_url already exists, just ensure data is copied from badge_image_url if it exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profile_badges' 
            AND column_name = 'badge_image_url'
        ) THEN
            -- Copy any missing data from badge_image_url to badge_media_url
            UPDATE profile_badges 
            SET badge_media_url = badge_image_url 
            WHERE badge_image_url IS NOT NULL 
            AND (badge_media_url IS NULL OR badge_media_url = '');
        END IF;
    ELSE
        -- badge_media_url doesn't exist, check if badge_image_url exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profile_badges' 
            AND column_name = 'badge_image_url'
        ) THEN
            -- Rename badge_image_url to badge_media_url (this preserves data)
            ALTER TABLE profile_badges RENAME COLUMN badge_image_url TO badge_media_url;
        ELSE
            -- Neither column exists, add badge_media_url
            ALTER TABLE profile_badges ADD COLUMN badge_media_url TEXT;
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

-- Update column comments (only if columns exist)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_media_url'
    ) THEN
        COMMENT ON COLUMN profile_badges.badge_media_url IS 'Media URL for the badge (can be image, video, etc.) - rich media content for detailed views';
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_badges' 
        AND column_name = 'badge_icon_url'
    ) THEN
        COMMENT ON COLUMN profile_badges.badge_icon_url IS 'Icon URL for the badge - small icon displayed next to username';
    END IF;
END $$;

