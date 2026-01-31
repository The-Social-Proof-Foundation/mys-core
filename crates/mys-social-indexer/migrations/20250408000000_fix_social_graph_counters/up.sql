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
-- Handle all combinations of TEXT and BYTEA types for both address columns and profile_id
-- Use dynamic SQL to prevent parse-time type checking
DO $$
DECLARE
    profile_id_type text;
    following_address_type text;
    follower_address_type text;
    following_compare_sql text;
    follower_compare_sql text;
BEGIN
    -- Check if profile_id column exists and get its type
    SELECT data_type INTO profile_id_type
    FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'profile_id';
    
    -- Check address column types in social_graph_relationships table
    SELECT data_type INTO following_address_type
    FROM information_schema.columns
    WHERE table_name = 'social_graph_relationships' AND column_name = 'following_address';
    
    SELECT data_type INTO follower_address_type
    FROM information_schema.columns
    WHERE table_name = 'social_graph_relationships' AND column_name = 'follower_address';
    
    -- Only proceed if profile_id column exists
    IF profile_id_type IS NOT NULL AND following_address_type IS NOT NULL AND follower_address_type IS NOT NULL THEN
        -- Ensure profile_id is indexed for faster lookups
        EXECUTE 'CREATE INDEX IF NOT EXISTS idx_profiles_profile_id ON profiles(profile_id)';
        
        -- Build comparison SQL for following_address based on type combination
        IF following_address_type = 'bytea' AND profile_id_type = 'bytea' THEN
            -- Both BYTEA: direct comparison
            following_compare_sql := 'following_address = p.profile_id';
        ELSIF following_address_type = 'bytea' AND profile_id_type != 'bytea' THEN
            -- Address BYTEA, profile_id TEXT: decode TEXT to BYTEA
            following_compare_sql := 'following_address = decode(p.profile_id::text, ''hex'')::bytea';
        ELSIF following_address_type != 'bytea' AND profile_id_type = 'bytea' THEN
            -- Address TEXT, profile_id BYTEA: encode BYTEA to TEXT
            following_compare_sql := 'following_address = encode(p.profile_id::bytea, ''hex'')';
        ELSE
            -- Both TEXT: direct comparison with text cast
            following_compare_sql := 'following_address = p.profile_id::text';
        END IF;
        
        -- Build comparison SQL for follower_address based on type combination
        IF follower_address_type = 'bytea' AND profile_id_type = 'bytea' THEN
            -- Both BYTEA: direct comparison
            follower_compare_sql := 'follower_address = p.profile_id';
        ELSIF follower_address_type = 'bytea' AND profile_id_type != 'bytea' THEN
            -- Address BYTEA, profile_id TEXT: decode TEXT to BYTEA
            follower_compare_sql := 'follower_address = decode(p.profile_id::text, ''hex'')::bytea';
        ELSIF follower_address_type != 'bytea' AND profile_id_type = 'bytea' THEN
            -- Address TEXT, profile_id BYTEA: encode BYTEA to TEXT
            follower_compare_sql := 'follower_address = encode(p.profile_id::bytea, ''hex'')';
        ELSE
            -- Both TEXT: direct comparison with text cast
            follower_compare_sql := 'follower_address = p.profile_id::text';
        END IF;
        
        -- Execute UPDATE statements with dynamically built comparisons
        EXECUTE format('UPDATE profiles p SET followers_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE %s) WHERE p.profile_id IS NOT NULL', following_compare_sql);
        EXECUTE format('UPDATE profiles p SET following_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE %s) WHERE p.profile_id IS NOT NULL', follower_compare_sql);
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