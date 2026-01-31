-- Ensure helper function exists to convert profile_id to text format matching addresses
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

-- Restore the previous version of verify_follow_counts() that only updates profiles
CREATE OR REPLACE FUNCTION verify_follow_counts() 
RETURNS TRIGGER AS $$
BEGIN
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
