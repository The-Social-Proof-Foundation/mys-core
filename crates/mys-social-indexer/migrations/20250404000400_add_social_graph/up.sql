-- Add follower/following count columns to profiles table
ALTER TABLE profiles 
ADD COLUMN IF NOT EXISTS followers_count INTEGER NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS following_count INTEGER NOT NULL DEFAULT 0;

-- Create social graph relationships table
-- Note: follower_id and following_id columns are dropped in a later migration (20250404000600)
-- So we create the table without foreign keys first, then add them conditionally
CREATE TABLE IF NOT EXISTS social_graph_relationships (
    id SERIAL PRIMARY KEY,
    follower_id INTEGER,
    following_id INTEGER,
    follower_address TEXT NOT NULL,
    following_address TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add follower_id column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' 
        AND column_name = 'follower_id'
    ) THEN
        ALTER TABLE social_graph_relationships 
        ADD COLUMN follower_id INTEGER;
    END IF;
END $$;

-- Add following_id column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' 
        AND column_name = 'following_id'
    ) THEN
        ALTER TABLE social_graph_relationships 
        ADD COLUMN following_id INTEGER;
    END IF;
END $$;

-- Add foreign key constraints only if they don't exist and columns exist
DO $$
BEGIN
    -- Check if profiles table has id column and follower_id exists
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' AND column_name = 'follower_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_relationships_follower_id_fkey'
    ) THEN
        ALTER TABLE social_graph_relationships
        ADD CONSTRAINT social_graph_relationships_follower_id_fkey
        FOREIGN KEY (follower_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' AND column_name = 'following_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_relationships_following_id_fkey'
    ) THEN
        ALTER TABLE social_graph_relationships
        ADD CONSTRAINT social_graph_relationships_following_id_fkey
        FOREIGN KEY (following_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
END $$;

-- Add unique constraint only if it doesn't exist and columns exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' AND column_name = 'follower_id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' AND column_name = 'following_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_relationships_follower_id_following_id_key'
    ) THEN
        ALTER TABLE social_graph_relationships
        ADD CONSTRAINT social_graph_relationships_follower_id_following_id_key
        UNIQUE(follower_id, following_id);
    END IF;
END $$;

-- Add indexes for faster lookups
-- Only create indexes on follower_id/following_id if columns exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' AND column_name = 'follower_id'
    ) THEN
        CREATE INDEX IF NOT EXISTS idx_social_graph_follower_id ON social_graph_relationships(follower_id);
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_relationships' AND column_name = 'following_id'
    ) THEN
        CREATE INDEX IF NOT EXISTS idx_social_graph_following_id ON social_graph_relationships(following_id);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_social_graph_follower_address ON social_graph_relationships(follower_address);
CREATE INDEX IF NOT EXISTS idx_social_graph_following_address ON social_graph_relationships(following_address);
CREATE INDEX IF NOT EXISTS idx_social_graph_created_at ON social_graph_relationships(created_at);

-- Add indexes to profile table for follower/following counts
CREATE INDEX IF NOT EXISTS idx_profiles_followers_count ON profiles(followers_count);
CREATE INDEX IF NOT EXISTS idx_profiles_following_count ON profiles(following_count);

-- Add comment to describe the purpose of the table
COMMENT ON TABLE social_graph_relationships IS 'Tracks follow relationships between profiles';