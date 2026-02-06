-- Revert wallet_address back to profile_id in platform_blocked_profiles table

-- Rename the column back
ALTER TABLE platform_blocked_profiles RENAME COLUMN wallet_address TO profile_id;

-- Update the unique constraint back to use profile_id
ALTER TABLE platform_blocked_profiles DROP CONSTRAINT IF EXISTS platform_blocked_profiles_platform_id_wallet_address_key;
ALTER TABLE platform_blocked_profiles ADD CONSTRAINT platform_blocked_profiles_platform_id_profile_id_key UNIQUE (platform_id, profile_id);

-- Rename the index back
DROP INDEX IF EXISTS idx_platform_blocked_profiles_wallet_address;
CREATE INDEX IF NOT EXISTS idx_platform_blocked_profiles_profile_id ON platform_blocked_profiles (profile_id);

-- Revert the table comment
COMMENT ON TABLE platform_blocked_profiles IS 'Records of profiles blocked by platforms.';
