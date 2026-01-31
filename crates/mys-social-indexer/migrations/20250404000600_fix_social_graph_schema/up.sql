-- Update social_graph_relationships table to use blockchain addresses as primary key fields
-- This ensures we don't rely on database IDs, which can be duplicated

ALTER TABLE social_graph_relationships 
DROP COLUMN IF EXISTS follower_id,
DROP COLUMN IF EXISTS following_id;

-- Update the unique constraint to prevent duplicate relationships using addresses
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_relationships_unique_relationship'
        AND conrelid = 'social_graph_relationships'::regclass
    ) THEN
        ALTER TABLE social_graph_relationships
        ADD CONSTRAINT social_graph_relationships_unique_relationship 
        UNIQUE (follower_address, following_address);
    END IF;
END $$;

-- Add indexes on addresses for fast lookups
CREATE INDEX IF NOT EXISTS idx_social_graph_relationships_follower_address ON social_graph_relationships(follower_address);
CREATE INDEX IF NOT EXISTS idx_social_graph_relationships_following_address ON social_graph_relationships(following_address);

-- Similarly update the social_graph_events table to remove db IDs
ALTER TABLE social_graph_events
DROP COLUMN IF EXISTS follower_id,
DROP COLUMN IF EXISTS following_id;

-- Add raw blockchain IDs to profiles table for better tracking
ALTER TABLE profiles
ADD COLUMN IF NOT EXISTS blockchain_tx_id TEXT NULL;

-- Delete placeholder profiles (rows 3 and 4) that were automatically created
-- Only delete if profiles table has id column
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) THEN
        DELETE FROM profiles WHERE id IN (3, 4);
    END IF;
END $$;

-- Update counter values for existing profiles
UPDATE profiles 
SET followers_count = (
    SELECT COUNT(*) FROM social_graph_relationships 
    WHERE following_address = profiles.owner_address
),
following_count = (
    SELECT COUNT(*) FROM social_graph_relationships 
    WHERE follower_address = profiles.owner_address
);