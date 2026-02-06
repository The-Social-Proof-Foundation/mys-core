-- Create profiles table (only if it doesn't exist)
CREATE TABLE IF NOT EXISTS profiles (
    id SERIAL PRIMARY KEY,
    owner_address TEXT NOT NULL,
    username TEXT NOT NULL,
    display_name TEXT,
    bio TEXT,
    avatar_url TEXT,
    website_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Ensure owner_address column exists (in case table was created without it)
DO $$ 
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'owner_address'
    ) THEN
        -- Add column, handling NOT NULL constraint based on whether table has data
        IF EXISTS (SELECT 1 FROM profiles LIMIT 1) THEN
            -- Table has data: add as nullable, populate, then make NOT NULL
            ALTER TABLE profiles ADD COLUMN owner_address TEXT;
            UPDATE profiles SET owner_address = '' WHERE owner_address IS NULL;
            ALTER TABLE profiles ALTER COLUMN owner_address SET NOT NULL;
        ELSE
            -- Table is empty: can add as NOT NULL directly
            ALTER TABLE profiles ADD COLUMN owner_address TEXT NOT NULL;
        END IF;
    END IF;
END $$;

-- Ensure username column exists (in case table was created without it)
DO $$ 
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'username'
    ) THEN
        -- Add column, handling NOT NULL constraint based on whether table has data
        IF EXISTS (SELECT 1 FROM profiles LIMIT 1) THEN
            -- Table has data: add as nullable, populate, then make NOT NULL
            ALTER TABLE profiles ADD COLUMN username TEXT;
            UPDATE profiles SET username = '' WHERE username IS NULL;
            ALTER TABLE profiles ALTER COLUMN username SET NOT NULL;
        ELSE
            -- Table is empty: can add as NOT NULL directly
            ALTER TABLE profiles ADD COLUMN username TEXT NOT NULL;
        END IF;
    END IF;
END $$;

-- Create unique indexes (only if they don't exist and column exists)
DO $$ 
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'owner_address'
    ) THEN
        CREATE UNIQUE INDEX IF NOT EXISTS idx_profiles_owner_address ON profiles(owner_address);
    END IF;
END $$;

DO $$ 
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'username'
    ) THEN
        CREATE UNIQUE INDEX IF NOT EXISTS idx_profiles_username ON profiles(username);
    END IF;
END $$;

-- Create indexer checkpoint state table (only if it doesn't exist)
CREATE TABLE IF NOT EXISTS indexer_checkpoint_state (
    id SERIAL PRIMARY KEY,
    last_processed_checkpoint BIGINT NOT NULL,
    last_processed_timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Insert initial indexer checkpoint state (only if it doesn't exist)
INSERT INTO indexer_checkpoint_state (id, last_processed_checkpoint, last_processed_timestamp)
VALUES (1, 0, NOW())
ON CONFLICT (id) DO NOTHING;