-- Create platform_memberships table
CREATE TABLE IF NOT EXISTS platform_memberships (
    id SERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    joined_at TIMESTAMP NOT NULL
);

-- Add role column if it doesn't exist (may have been dropped by later migration)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_memberships' AND column_name = 'role'
    ) THEN
        ALTER TABLE platform_memberships ADD COLUMN role TEXT NOT NULL DEFAULT 'member';
    END IF;
END $$;

-- Add left_at column if it doesn't exist (may have been dropped by later migration)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_memberships' AND column_name = 'left_at'
    ) THEN
        ALTER TABLE platform_memberships ADD COLUMN left_at TIMESTAMP NULL;
    END IF;
END $$;

-- Add unique constraint if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'platform_memberships_platform_id_profile_id_key'
    ) THEN
        ALTER TABLE platform_memberships ADD CONSTRAINT platform_memberships_platform_id_profile_id_key UNIQUE (platform_id, profile_id);
    END IF;
END $$;

-- Create indexes for faster lookups (only if columns exist)
CREATE INDEX IF NOT EXISTS idx_platform_memberships_platform_id ON platform_memberships (platform_id);
CREATE INDEX IF NOT EXISTS idx_platform_memberships_profile_id ON platform_memberships (profile_id);
CREATE INDEX IF NOT EXISTS idx_platform_memberships_joined_at ON platform_memberships (joined_at);

-- Create index on role only if column exists
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_memberships' AND column_name = 'role'
    ) THEN
        CREATE INDEX IF NOT EXISTS idx_platform_memberships_role ON platform_memberships (role);
    END IF;
END $$;

-- Create index on left_at only if column exists
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platform_memberships' AND column_name = 'left_at'
    ) THEN
        CREATE INDEX IF NOT EXISTS idx_platform_memberships_left_at ON platform_memberships (left_at);
    END IF;
END $$; 