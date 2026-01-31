-- Create function to migrate counts when profile owner_address changes (profile transfer/sale)
CREATE OR REPLACE FUNCTION migrate_profile_counts_to_wallet() 
RETURNS TRIGGER AS $$
BEGIN
    -- Only migrate if owner_address changed (profile transfer/sale)
    IF OLD.owner_address IS DISTINCT FROM NEW.owner_address THEN
        -- Migrate counts from profile to wallet_social_graph for OLD owner
        INSERT INTO wallet_social_graph (wallet_address, followers_count, following_count, blocked_count, updated_at)
        VALUES (
            OLD.owner_address,
            OLD.followers_count,
            OLD.following_count,
            OLD.blocked_count,
            NOW()
        )
        ON CONFLICT (wallet_address) 
        DO UPDATE SET 
            followers_count = wallet_social_graph.followers_count + OLD.followers_count,
            following_count = wallet_social_graph.following_count + OLD.following_count,
            blocked_count = wallet_social_graph.blocked_count + OLD.blocked_count,
            updated_at = NOW();
        
        -- Migrate counts from wallet_social_graph to profile for NEW owner (if they had wallet counts)
        IF EXISTS (SELECT 1 FROM wallet_social_graph WHERE wallet_address = NEW.owner_address) THEN
            -- Update profile using owner_address (which should be unique)
            -- This avoids issues if id column doesn't exist
            UPDATE profiles
            SET 
                followers_count = COALESCE(NEW.followers_count, 0) + (
                    SELECT followers_count FROM wallet_social_graph WHERE wallet_address = NEW.owner_address
                ),
                following_count = COALESCE(NEW.following_count, 0) + (
                    SELECT following_count FROM wallet_social_graph WHERE wallet_address = NEW.owner_address
                ),
                blocked_count = COALESCE(NEW.blocked_count, 0) + (
                    SELECT blocked_count FROM wallet_social_graph WHERE wallet_address = NEW.owner_address
                )
            WHERE owner_address = NEW.owner_address;
            
            DELETE FROM wallet_social_graph WHERE wallet_address = NEW.owner_address;
        END IF;
        
        RAISE NOTICE 'Migrated counts for profile transfer: old_owner=%, new_owner=%', OLD.owner_address, NEW.owner_address;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to migrate counts when profile owner_address changes
CREATE TRIGGER migrate_counts_on_profile_transfer
AFTER UPDATE OF owner_address ON profiles
FOR EACH ROW 
WHEN (OLD.owner_address IS DISTINCT FROM NEW.owner_address)
EXECUTE FUNCTION migrate_profile_counts_to_wallet();
