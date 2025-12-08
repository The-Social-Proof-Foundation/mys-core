-- Update the platform_memberships table by removing the role and left_at columns
-- This change is to support the new approach where we completely delete records when users leave platforms
-- rather than just marking them with a left_at timestamp

-- Remove the columns if they exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_memberships' AND column_name = 'role'
    ) THEN
        ALTER TABLE platform_memberships DROP COLUMN role;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_memberships' AND column_name = 'left_at'
    ) THEN
        ALTER TABLE platform_memberships DROP COLUMN left_at;
    END IF;
END $$;

-- Add a comment to the table explaining the new behavior
COMMENT ON TABLE platform_memberships IS 'Records of profiles joined to platforms. Records are deleted when a user leaves a platform.'; 