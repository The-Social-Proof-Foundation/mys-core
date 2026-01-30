-- Fix double counting bug by enhancing triggers to handle both profile_id and owner_address
-- and recalculating existing counts

-- First, recalculate all counts from actual relationships to fix any existing discrepancies
-- Recalculate followers_count for all profiles (matching on both owner_address and profile_id)
UPDATE profiles p
SET followers_count = (
    SELECT COUNT(*) 
    FROM social_graph_relationships 
    WHERE following_address = p.owner_address 
       OR following_address = p.profile_id
)
WHERE p.owner_address IS NOT NULL;

-- Recalculate following_count for all profiles (matching on both owner_address and profile_id)
UPDATE profiles p
SET following_count = (
    SELECT COUNT(*) 
    FROM social_graph_relationships 
    WHERE follower_address = p.owner_address 
       OR follower_address = p.profile_id
)
WHERE p.owner_address IS NOT NULL;

-- Enhance the trigger function to match on both profile_id and owner_address
CREATE OR REPLACE FUNCTION verify_follow_counts() 
RETURNS TRIGGER AS $$
BEGIN
    -- Log whenever an update happens (can be removed in production if too verbose)
    -- RAISE NOTICE 'Follow relationship changed: %', TG_OP;
    
    -- Different actions based on operation type
    IF (TG_OP = 'INSERT') THEN
        -- Update follower's following_count (+1)
        -- Match on both owner_address and profile_id
        UPDATE profiles
        SET following_count = following_count + 1
        WHERE owner_address = NEW.follower_address 
           OR profile_id = NEW.follower_address;
        
        -- Update following's followers_count (+1)
        -- Match on both owner_address and profile_id
        UPDATE profiles
        SET followers_count = followers_count + 1
        WHERE owner_address = NEW.following_address 
           OR profile_id = NEW.following_address;
        
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        -- Update follower's following_count (-1)
        -- Match on both owner_address and profile_id
        -- Use GREATEST to prevent negative counts
        UPDATE profiles
        SET following_count = GREATEST(0, following_count - 1)
        WHERE owner_address = OLD.follower_address 
           OR profile_id = OLD.follower_address;
        
        -- Update following's followers_count (-1)
        -- Match on both owner_address and profile_id
        -- Use GREATEST to prevent negative counts
        UPDATE profiles
        SET followers_count = GREATEST(0, followers_count - 1)
        WHERE owner_address = OLD.following_address 
           OR profile_id = OLD.following_address;
        
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

