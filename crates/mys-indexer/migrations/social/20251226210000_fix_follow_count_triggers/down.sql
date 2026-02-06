-- Rollback: Restore the old trigger function that only matches on owner_address

CREATE OR REPLACE FUNCTION verify_follow_counts() 
RETURNS TRIGGER AS $$
BEGIN
    -- Log whenever an update happens
    RAISE NOTICE 'Follow relationship changed: %', TG_OP;
    
    -- Different actions based on operation type
    IF (TG_OP = 'INSERT') THEN
        UPDATE profiles
        SET following_count = following_count + 1
        WHERE owner_address = NEW.follower_address;
        
        UPDATE profiles
        SET followers_count = followers_count + 1
        WHERE owner_address = NEW.following_address;
        
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        UPDATE profiles
        SET following_count = GREATEST(0, following_count - 1)
        WHERE owner_address = OLD.follower_address;
        
        UPDATE profiles
        SET followers_count = GREATEST(0, followers_count - 1)
        WHERE owner_address = OLD.following_address;
        
        RETURN OLD;
    END IF;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Triggers remain the same, just using the old function
DROP TRIGGER IF EXISTS update_follow_counts_insert ON social_graph_relationships;
CREATE TRIGGER update_follow_counts_insert
AFTER INSERT ON social_graph_relationships
FOR EACH ROW EXECUTE FUNCTION verify_follow_counts();

DROP TRIGGER IF EXISTS update_follow_counts_delete ON social_graph_relationships;
CREATE TRIGGER update_follow_counts_delete
AFTER DELETE ON social_graph_relationships  
FOR EACH ROW EXECUTE FUNCTION verify_follow_counts();

