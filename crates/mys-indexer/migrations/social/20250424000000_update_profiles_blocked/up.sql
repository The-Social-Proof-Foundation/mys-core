-- Update the profiles_blocked table by removing the is_blocked and unblocked_at columns
-- This change is to support the new approach where we completely delete records when profiles are unblocked
-- rather than just marking them as unblocked

-- Remove the columns if they exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles_blocked' AND column_name = 'is_blocked'
    ) THEN
        ALTER TABLE profiles_blocked DROP COLUMN is_blocked;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles_blocked' AND column_name = 'unblocked_at'
    ) THEN
        ALTER TABLE profiles_blocked DROP COLUMN unblocked_at;
    END IF;
END $$;

-- Add a comment to the table explaining the new behavior
COMMENT ON TABLE profiles_blocked IS 'Records of profiles blocked by other profiles. Records are deleted when a profile is unblocked.'; 