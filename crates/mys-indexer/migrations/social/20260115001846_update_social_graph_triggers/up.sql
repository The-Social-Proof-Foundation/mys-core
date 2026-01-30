-- Update verify_follow_counts() function to handle both profiles and wallet_social_graph tables
CREATE OR REPLACE FUNCTION verify_follow_counts() 
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        -- Update follower's following_count
        -- Check if follower has profile
        IF EXISTS (SELECT 1 FROM profiles WHERE owner_address = NEW.follower_address OR profile_id = NEW.follower_address) THEN
            UPDATE profiles
            SET following_count = following_count + 1
            WHERE owner_address = NEW.follower_address OR profile_id = NEW.follower_address;
        ELSE
            -- Update wallet_social_graph (upsert)
            INSERT INTO wallet_social_graph (wallet_address, following_count, updated_at)
            VALUES (NEW.follower_address, 1, NOW())
            ON CONFLICT (wallet_address) 
            DO UPDATE SET 
                following_count = wallet_social_graph.following_count + 1,
                updated_at = NOW();
        END IF;
        
        -- Update following's followers_count
        IF EXISTS (SELECT 1 FROM profiles WHERE owner_address = NEW.following_address OR profile_id = NEW.following_address) THEN
            UPDATE profiles
            SET followers_count = followers_count + 1
            WHERE owner_address = NEW.following_address OR profile_id = NEW.following_address;
        ELSE
            -- Update wallet_social_graph (upsert)
            INSERT INTO wallet_social_graph (wallet_address, followers_count, updated_at)
            VALUES (NEW.following_address, 1, NOW())
            ON CONFLICT (wallet_address) 
            DO UPDATE SET 
                followers_count = wallet_social_graph.followers_count + 1,
                updated_at = NOW();
        END IF;
        
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        -- Update follower's following_count
        IF EXISTS (SELECT 1 FROM profiles WHERE owner_address = OLD.follower_address OR profile_id = OLD.follower_address) THEN
            UPDATE profiles
            SET following_count = GREATEST(0, following_count - 1)
            WHERE owner_address = OLD.follower_address OR profile_id = OLD.follower_address;
        ELSE
            UPDATE wallet_social_graph
            SET following_count = GREATEST(0, following_count - 1),
                updated_at = NOW()
            WHERE wallet_address = OLD.follower_address;
        END IF;
        
        -- Update following's followers_count
        IF EXISTS (SELECT 1 FROM profiles WHERE owner_address = OLD.following_address OR profile_id = OLD.following_address) THEN
            UPDATE profiles
            SET followers_count = GREATEST(0, followers_count - 1)
            WHERE owner_address = OLD.following_address OR profile_id = OLD.following_address;
        ELSE
            UPDATE wallet_social_graph
            SET followers_count = GREATEST(0, followers_count - 1),
                updated_at = NOW()
            WHERE wallet_address = OLD.following_address;
        END IF;
        
        RETURN OLD;
    END IF;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
