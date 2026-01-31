-- Fix double counting bug by enhancing triggers to handle both profile_id and owner_address
-- and recalculating existing counts

-- First, recalculate all counts from actual relationships to fix any existing discrepancies
-- Recalculate followers_count for all profiles (matching on both owner_address and profile_id)
-- Handle all combinations of TEXT and BYTEA types for both address columns and profile_id
-- Use dynamic SQL to prevent parse-time type checking
DO $$
DECLARE
    profile_id_type text;
    following_address_type text;
    follower_address_type text;
    owner_address_type text;
    following_profile_compare_sql text;
    follower_profile_compare_sql text;
BEGIN
    -- Check the actual column types
    SELECT data_type INTO profile_id_type
    FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'profile_id';
    
    SELECT data_type INTO owner_address_type
    FROM information_schema.columns
    WHERE table_name = 'profiles' AND column_name = 'owner_address';
    
    SELECT data_type INTO following_address_type
    FROM information_schema.columns
    WHERE table_name = 'social_graph_relationships' AND column_name = 'following_address';
    
    SELECT data_type INTO follower_address_type
    FROM information_schema.columns
    WHERE table_name = 'social_graph_relationships' AND column_name = 'follower_address';
    
    -- Only proceed if all required columns exist
    IF profile_id_type IS NOT NULL AND owner_address_type IS NOT NULL 
       AND following_address_type IS NOT NULL AND follower_address_type IS NOT NULL THEN
        
        -- Build comparison SQL for following_address with profile_id based on type combination
        IF following_address_type = 'bytea' AND profile_id_type = 'bytea' THEN
            -- Both BYTEA: direct comparison
            following_profile_compare_sql := 'following_address = p.profile_id';
        ELSIF following_address_type = 'bytea' AND profile_id_type != 'bytea' THEN
            -- Address BYTEA, profile_id TEXT: decode TEXT to BYTEA
            following_profile_compare_sql := 'following_address = decode(p.profile_id::text, ''hex'')::bytea';
        ELSIF following_address_type != 'bytea' AND profile_id_type = 'bytea' THEN
            -- Address TEXT, profile_id BYTEA: encode BYTEA to TEXT
            following_profile_compare_sql := 'following_address = encode(p.profile_id::bytea, ''hex'')';
        ELSE
            -- Both TEXT: direct comparison with text cast
            following_profile_compare_sql := 'following_address = p.profile_id::text';
        END IF;
        
        -- Build comparison SQL for follower_address with profile_id based on type combination
        IF follower_address_type = 'bytea' AND profile_id_type = 'bytea' THEN
            -- Both BYTEA: direct comparison
            follower_profile_compare_sql := 'follower_address = p.profile_id';
        ELSIF follower_address_type = 'bytea' AND profile_id_type != 'bytea' THEN
            -- Address BYTEA, profile_id TEXT: decode TEXT to BYTEA
            follower_profile_compare_sql := 'follower_address = decode(p.profile_id::text, ''hex'')::bytea';
        ELSIF follower_address_type != 'bytea' AND profile_id_type = 'bytea' THEN
            -- Address TEXT, profile_id BYTEA: encode BYTEA to TEXT
            follower_profile_compare_sql := 'follower_address = encode(p.profile_id::bytea, ''hex'')';
        ELSE
            -- Both TEXT: direct comparison with text cast
            follower_profile_compare_sql := 'follower_address = p.profile_id::text';
        END IF;
        
        -- Execute UPDATE statements with dynamically built comparisons
        -- Note: owner_address comparison is always direct since it's always TEXT
        EXECUTE format('UPDATE profiles p SET followers_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE following_address = p.owner_address OR %s) WHERE p.owner_address IS NOT NULL', following_profile_compare_sql);
        EXECUTE format('UPDATE profiles p SET following_count = (SELECT COUNT(*) FROM social_graph_relationships WHERE follower_address = p.owner_address OR %s) WHERE p.owner_address IS NOT NULL', follower_profile_compare_sql);
    END IF;
END $$;

-- Create helper function to convert profile_id to text format matching addresses
-- Handles both BYTEA and TEXT types, as well as NULL values
CREATE OR REPLACE FUNCTION profile_id_to_text(p_profile_id anyelement) 
RETURNS text AS $$
BEGIN
    -- Handle NULL case
    IF p_profile_id IS NULL THEN
        RETURN '';
    END IF;
    
    -- Check if profile_id is BYTEA by attempting to encode it
    -- If it fails, it's TEXT and we can cast directly
    BEGIN
        RETURN encode(p_profile_id::bytea, 'hex');
    EXCEPTION WHEN OTHERS THEN
        RETURN p_profile_id::text;
    END;
END;
$$ LANGUAGE plpgsql;

-- Enhance the trigger function to match on both profile_id and owner_address
CREATE OR REPLACE FUNCTION verify_follow_counts() 
RETURNS TRIGGER AS $$
BEGIN
    -- Log whenever an update happens (can be removed in production if too verbose)
    -- RAISE NOTICE 'Follow relationship changed: %', TG_OP;
    
    -- Different actions based on operation type
    IF (TG_OP = 'INSERT') THEN
        -- Update follower's following_count (+1)
        -- Match on both owner_address and profile_id (handle BYTEA/TEXT types)
        UPDATE profiles
        SET following_count = following_count + 1
        WHERE owner_address = NEW.follower_address 
           OR profile_id_to_text(profile_id) = NEW.follower_address;
        
        -- Update following's followers_count (+1)
        -- Match on both owner_address and profile_id (handle BYTEA/TEXT types)
        UPDATE profiles
        SET followers_count = followers_count + 1
        WHERE owner_address = NEW.following_address 
           OR profile_id_to_text(profile_id) = NEW.following_address;
        
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        -- Update follower's following_count (-1)
        -- Match on both owner_address and profile_id (handle BYTEA/TEXT types)
        -- Use GREATEST to prevent negative counts
        UPDATE profiles
        SET following_count = GREATEST(0, following_count - 1)
        WHERE owner_address = OLD.follower_address 
           OR profile_id_to_text(profile_id) = OLD.follower_address;
        
        -- Update following's followers_count (-1)
        -- Match on both owner_address and profile_id (handle BYTEA/TEXT types)
        -- Use GREATEST to prevent negative counts
        UPDATE profiles
        SET followers_count = GREATEST(0, followers_count - 1)
        WHERE owner_address = OLD.following_address 
           OR profile_id_to_text(profile_id) = OLD.following_address;
        
        RETURN OLD;
    END IF;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Triggers are already created, but ensure they exist with the updated function
DROP TRIGGER IF EXISTS update_follow_counts_insert ON social_graph_relationships;
CREATE TRIGGER update_follow_counts_insert
AFTER INSERT ON social_graph_relationships
FOR EACH ROW EXECUTE FUNCTION verify_follow_counts();

DROP TRIGGER IF EXISTS update_follow_counts_delete ON social_graph_relationships;
CREATE TRIGGER update_follow_counts_delete
AFTER DELETE ON social_graph_relationships  
FOR EACH ROW EXECUTE FUNCTION verify_follow_counts();

