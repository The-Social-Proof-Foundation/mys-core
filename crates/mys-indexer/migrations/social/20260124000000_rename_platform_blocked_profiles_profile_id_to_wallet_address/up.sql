-- Rename profile_id to wallet_address in platform_blocked_profiles table
-- This aligns with the smart contract which uses wallet-level blocking for platform blocking

-- Rename the column
ALTER TABLE platform_blocked_profiles RENAME COLUMN profile_id TO wallet_address;

-- Update the unique constraint to use wallet_address
ALTER TABLE platform_blocked_profiles DROP CONSTRAINT IF EXISTS platform_blocked_profiles_platform_id_profile_id_key;
ALTER TABLE platform_blocked_profiles ADD CONSTRAINT platform_blocked_profiles_platform_id_wallet_address_key UNIQUE (platform_id, wallet_address);

-- Rename the index
DROP INDEX IF EXISTS idx_platform_blocked_profiles_profile_id;
CREATE INDEX IF NOT EXISTS idx_platform_blocked_profiles_wallet_address ON platform_blocked_profiles (wallet_address);

-- Update the table comment
COMMENT ON TABLE platform_blocked_profiles IS 'Records of wallet addresses blocked by platforms. Uses wallet-level blocking as per the smart contract.';
