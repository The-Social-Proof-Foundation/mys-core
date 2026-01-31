-- Recreate the username and username_history tables

-- Recreate usernames table
CREATE TABLE IF NOT EXISTS usernames (
    id SERIAL PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add unique index to enforce uniqueness of usernames
CREATE UNIQUE INDEX IF NOT EXISTS idx_usernames_username ON usernames(username);

-- Recreate username_history table
CREATE TABLE IF NOT EXISTS username_history (
    id SERIAL PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    old_username TEXT NOT NULL,
    new_username TEXT NOT NULL,
    changed_at TIMESTAMP NOT NULL DEFAULT NOW()
);

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

-- Populate the usernames table with existing usernames from profiles
-- Only populate if profiles table has id column
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'usernames'
    ) THEN
        INSERT INTO usernames (profile_id, username, registered_at, updated_at)
        SELECT id, username, created_at, updated_at
        FROM profiles;
    END IF;
END $$;