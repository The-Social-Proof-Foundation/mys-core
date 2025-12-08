-- Update the platform_blocked_profiles table by removing the is_blocked, unblocked_at, and unblocked_by columns
-- This change is to support the new approach where we completely delete records when profiles are unblocked
-- rather than just marking them as unblocked

-- Remove the columns if they exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_blocked_profiles' AND column_name = 'is_blocked'
    ) THEN
        ALTER TABLE platform_blocked_profiles DROP COLUMN is_blocked;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_blocked_profiles' AND column_name = 'unblocked_at'
    ) THEN
        ALTER TABLE platform_blocked_profiles DROP COLUMN unblocked_at;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_blocked_profiles' AND column_name = 'unblocked_by'
    ) THEN
        ALTER TABLE platform_blocked_profiles DROP COLUMN unblocked_by;
    END IF;
END $$;

-- Add a comment to the table explaining the new behavior
COMMENT ON TABLE platform_blocked_profiles IS 'Records of profiles blocked by platforms. Records are deleted when a profile is unblocked.'; 