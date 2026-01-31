-- This migration ensures the followers_count and following_count columns
-- in the profiles table are correctly updated from relationships

-- First ensure columns have proper defaults
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'followers_count'
    ) THEN
        ALTER TABLE profiles 
        ALTER COLUMN followers_count SET DEFAULT 0,
        ALTER COLUMN followers_count SET NOT NULL;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'following_count'
    ) THEN
        ALTER TABLE profiles 
        ALTER COLUMN following_count SET DEFAULT 0,
        ALTER COLUMN following_count SET NOT NULL;
    END IF;
END $$;

-- Update the counts based on actual relationships - matching on profile_id, not owner_address
-- Handle both TEXT and BYTEA profile_id types by casting appropriately
-- Use dynamic SQL to prevent parse-time type checking
DO $$
DECLARE
    profile_id_type text;
BEGIN
    -- Check if profile_id column exists and get its type
    SELECT data_type INTO profile_id_type
    FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'profile_id';
    
    -- Only proceed if profile_id column exists
    IF profile_id_type IS NOT NULL THEN
        -- Ensure profile_id is indexed for faster lookups
        EXECUTE 'CREATE INDEX IF NOT EXISTS idx_profiles_profile_id ON profiles(profile_id)';
        
        IF profile_id_type = 'bytea' THEN
            -- If BYTEA, encode to hex to match TEXT address format
            -- Use dynamic SQL to avoid parse-time type checking
            EXECUTE format('UPDATE profiles p SET followers_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE following_address = encode(p.profile_id::bytea, ''hex'')) WHERE p.profile_id IS NOT NULL');
            EXECUTE format('UPDATE profiles p SET following_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE follower_address = encode(p.profile_id::bytea, ''hex'')) WHERE p.profile_id IS NOT NULL');
        ELSE
            -- If TEXT/VARCHAR, cast to text directly
            -- Use dynamic SQL to avoid parse-time type checking
            EXECUTE format('UPDATE profiles p SET followers_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE following_address = p.profile_id::text) WHERE p.profile_id IS NOT NULL');
            EXECUTE format('UPDATE profiles p SET following_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE follower_address = p.profile_id::text) WHERE p.profile_id IS NOT NULL');
        END IF;
    END IF;
END $$;

-- Create an index on relationships to improve performance of count lookups
DROP INDEX IF EXISTS idx_social_graph_relationships_pair;
CREATE INDEX IF NOT EXISTS idx_social_graph_relationships_pair 
ON social_graph_relationships(follower_address, following_address);

-- Add details to the unique constraint for clarity
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_relationships_unique_relationship'
        AND conrelid = 'social_graph_relationships'::regclass
    ) THEN
        COMMENT ON CONSTRAINT social_graph_relationships_unique_relationship 
        ON social_graph_relationships 
        IS 'Ensures follower can only follow an account once';
    END IF;
END $$;