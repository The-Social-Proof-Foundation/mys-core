-- Rename columns in the profiles_blocked table
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles_blocked' 
        AND column_name = 'blocker_profile_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles_blocked' 
        AND column_name = 'blocker_wallet_address'
    ) THEN
        ALTER TABLE profiles_blocked 
        RENAME COLUMN blocker_profile_id TO blocker_wallet_address;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles_blocked' 
        AND column_name = 'blocked_profile_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles_blocked' 
        AND column_name = 'blocked_address'
    ) THEN
        ALTER TABLE profiles_blocked 
        RENAME COLUMN blocked_profile_id TO blocked_address;
    END IF;
END $$;

-- Update indices to reflect the new column names
DROP INDEX IF EXISTS idx_profiles_blocked_blocker_profile_id;
DROP INDEX IF EXISTS idx_profiles_blocked_blocked_profile_id;

CREATE INDEX IF NOT EXISTS idx_profiles_blocked_blocker_wallet_address ON profiles_blocked(blocker_wallet_address);
CREATE INDEX IF NOT EXISTS idx_profiles_blocked_blocked_address ON profiles_blocked(blocked_address); 