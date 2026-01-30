-- Revert the rename: wallet_address back to profile_id

-- Rename the column back
ALTER TABLE platform_memberships RENAME COLUMN wallet_address TO profile_id;

-- Update the unique constraint back
ALTER TABLE platform_memberships DROP CONSTRAINT IF EXISTS platform_memberships_platform_id_wallet_address_key;
ALTER TABLE platform_memberships ADD CONSTRAINT platform_memberships_platform_id_profile_id_key UNIQUE (platform_id, profile_id);

-- Rename the index back
DROP INDEX IF EXISTS idx_platform_memberships_wallet_address;
CREATE INDEX IF NOT EXISTS idx_platform_memberships_profile_id ON platform_memberships (profile_id);

-- Revert the table comment
COMMENT ON TABLE platform_memberships IS 'Records of profiles joined to platforms. Records are deleted when a user leaves a platform.';

