-- Create profiles table
CREATE TABLE IF NOT EXISTS profiles (
    id SERIAL PRIMARY KEY,
    owner_address TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT,
    bio TEXT,
    avatar_url TEXT,
    website_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create an index on owner_address for faster lookups
CREATE INDEX IF NOT EXISTS idx_profiles_owner_address ON profiles(owner_address);

-- Create indexer progress table to track processing
CREATE TABLE IF NOT EXISTS indexer_progress (
    id TEXT PRIMARY KEY,
    last_checkpoint_processed BIGINT NOT NULL DEFAULT 0,
    last_processed_at TIMESTAMP NOT NULL DEFAULT NOW()
);