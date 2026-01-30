-- Create the profiles_blocked table
-- This migration is idempotent and handles cases where:
-- 1. Table doesn't exist - creates it with original schema
-- 2. Table exists with original columns - creates indexes
-- 3. Table exists with renamed columns - skips index creation (handled by later migrations)
-- 4. Table was dropped by later migrations - will recreate if migration runs again

CREATE TABLE IF NOT EXISTS profiles_blocked (
    id SERIAL PRIMARY KEY,
    blocker_profile_id TEXT NOT NULL,
    blocked_profile_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    is_blocked BOOLEAN NOT NULL DEFAULT TRUE,
    unblocked_at TIMESTAMP,
    UNIQUE(blocker_profile_id, blocked_profile_id)
);

-- Create indices for faster lookups (only if columns exist)
-- This makes the migration idempotent even if columns were renamed by later migrations
DO $$
BEGIN
    -- Only proceed if table exists and has the original column names
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'profiles_blocked'
    ) THEN
        -- Create index on blocker_profile_id if column exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profiles_blocked' 
            AND column_name = 'blocker_profile_id'
        ) THEN
            CREATE INDEX IF NOT EXISTS idx_profiles_blocked_blocker_profile_id ON profiles_blocked(blocker_profile_id);
        END IF;
        
        -- Create index on blocked_profile_id if column exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profiles_blocked' 
            AND column_name = 'blocked_profile_id'
        ) THEN
            CREATE INDEX IF NOT EXISTS idx_profiles_blocked_blocked_profile_id ON profiles_blocked(blocked_profile_id);
        END IF;
        
        -- Create index on is_blocked if column exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = 'profiles_blocked' 
            AND column_name = 'is_blocked'
        ) THEN
            CREATE INDEX IF NOT EXISTS idx_profiles_blocked_is_blocked ON profiles_blocked(is_blocked);
        END IF;
    END IF;
END $$; 