-- Create function to update blocked_count in both profiles and wallet_social_graph tables
CREATE OR REPLACE FUNCTION update_blocked_count() 
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        -- Increment blocker's blocked_count
        IF EXISTS (SELECT 1 FROM profiles WHERE owner_address = NEW.blocker_address) THEN
            UPDATE profiles
            SET blocked_count = blocked_count + 1
            WHERE owner_address = NEW.blocker_address;
        ELSE
            INSERT INTO wallet_social_graph (wallet_address, blocked_count, updated_at)
            VALUES (NEW.blocker_address, 1, NOW())
            ON CONFLICT (wallet_address) 
            DO UPDATE SET 
                blocked_count = wallet_social_graph.blocked_count + 1,
                updated_at = NOW();
        END IF;
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        -- Decrement blocker's blocked_count
        IF EXISTS (SELECT 1 FROM profiles WHERE owner_address = OLD.blocker_address) THEN
            UPDATE profiles
            SET blocked_count = GREATEST(0, blocked_count - 1)
            WHERE owner_address = OLD.blocker_address;
        ELSE
            UPDATE wallet_social_graph
            SET blocked_count = GREATEST(0, blocked_count - 1),
                updated_at = NOW()
            WHERE wallet_address = OLD.blocker_address;
        END IF;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for blocked_profiles table
CREATE TRIGGER update_blocked_count_on_block
AFTER INSERT ON blocked_profiles
FOR EACH ROW EXECUTE FUNCTION update_blocked_count();

CREATE TRIGGER update_blocked_count_on_unblock
AFTER DELETE ON blocked_profiles
FOR EACH ROW EXECUTE FUNCTION update_blocked_count();
