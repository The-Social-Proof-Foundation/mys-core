-- Rename profile_id to wallet_address in platform_memberships table
-- This aligns with the smart contract which emits wallet_address in UserJoinedPlatformEvent

-- Rename the column
ALTER TABLE platform_memberships RENAME COLUMN profile_id TO wallet_address;

-- Update the unique constraint to use wallet_address
ALTER TABLE platform_memberships DROP CONSTRAINT IF EXISTS platform_memberships_platform_id_profile_id_key;
ALTER TABLE platform_memberships ADD CONSTRAINT platform_memberships_platform_id_wallet_address_key UNIQUE (platform_id, wallet_address);

-- Rename the index
DROP INDEX IF EXISTS idx_platform_memberships_profile_id;
CREATE INDEX IF NOT EXISTS idx_platform_memberships_wallet_address ON platform_memberships (wallet_address);

-- Update the table comment
COMMENT ON TABLE platform_memberships IS 'Records of wallet addresses joined to platforms. Records are deleted when a user leaves a platform.';

