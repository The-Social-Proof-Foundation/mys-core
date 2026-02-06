-- Rename avatar_url column to profile_photo (only if avatar_url exists and profile_photo doesn't)
DO $$
BEGIN
    -- Check if avatar_url exists and profile_photo doesn't exist
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'avatar_url'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'profile_photo'
    ) THEN
        ALTER TABLE profiles RENAME COLUMN avatar_url TO profile_photo;
    END IF;
END $$;