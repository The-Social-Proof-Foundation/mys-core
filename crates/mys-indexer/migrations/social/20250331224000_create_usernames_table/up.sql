-- Create usernames table
CREATE TABLE IF NOT EXISTS usernames (
    id SERIAL PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    username TEXT NOT NULL UNIQUE,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create index on username for fast lookups
CREATE INDEX IF NOT EXISTS usernames_username_idx ON usernames(username);

-- Create index on profile_id for fast profile lookups
CREATE INDEX IF NOT EXISTS usernames_profile_id_idx ON usernames(profile_id);

-- Add history table to track username changes
CREATE TABLE IF NOT EXISTS username_history (
    id SERIAL PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    old_username TEXT NOT NULL,
    new_username TEXT NOT NULL,
    changed_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create index on profile_id for username history
CREATE INDEX IF NOT EXISTS username_history_profile_id_idx ON username_history(profile_id);

-- Add foreign key constraints only if profiles table exists with id column
DO $$
BEGIN
    -- Add foreign key constraint to usernames
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'usernames' AND column_name = 'profile_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'usernames_profile_id_fkey'
    ) THEN
        ALTER TABLE usernames
        ADD CONSTRAINT usernames_profile_id_fkey
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
    
    -- Add foreign key constraint to username_history
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'username_history' AND column_name = 'profile_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'username_history_profile_id_fkey'
    ) THEN
        ALTER TABLE username_history
        ADD CONSTRAINT username_history_profile_id_fkey
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
END $$;