-- Create function to migrate counts from wallet_social_graph to profiles when profile is created
CREATE OR REPLACE FUNCTION migrate_wallet_counts_to_profile() 
RETURNS TRIGGER AS $$
DECLARE
    wallet_counts RECORD;
BEGIN
    -- Check if wallet_social_graph has counts for this wallet
    SELECT * INTO wallet_counts
    FROM wallet_social_graph
    WHERE wallet_address = NEW.owner_address;
    
    IF FOUND THEN
        -- Migrate counts to profile using owner_address (which should be unique)
        -- This avoids issues if id column doesn't exist
        UPDATE profiles
        SET 
            followers_count = COALESCE(NEW.followers_count, 0) + wallet_counts.followers_count,
            following_count = COALESCE(NEW.following_count, 0) + wallet_counts.following_count,
            blocked_count = COALESCE(NEW.blocked_count, 0) + wallet_counts.blocked_count
        WHERE owner_address = NEW.owner_address;
        
        -- Delete from wallet_social_graph after migration
        DELETE FROM wallet_social_graph WHERE wallet_address = NEW.owner_address;
        
        RAISE NOTICE 'Migrated counts from wallet_social_graph to profile: wallet=%, followers=%, following=%, blocked=%', 
            NEW.owner_address, wallet_counts.followers_count, wallet_counts.following_count, wallet_counts.blocked_count;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to migrate counts when profile is created
CREATE TRIGGER migrate_counts_on_profile_create
AFTER INSERT ON profiles
FOR EACH ROW EXECUTE FUNCTION migrate_wallet_counts_to_profile();
