-- Restore block_list_address columns (for rollback)
ALTER TABLE profiles ADD COLUMN block_list_address TEXT;
CREATE INDEX IF NOT EXISTS idx_profiles_block_list_address ON profiles (block_list_address);

ALTER TABLE blocked_profiles ADD COLUMN block_list_address TEXT;
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_block_list ON blocked_profiles(block_list_address);

ALTER TABLE blocked_events ADD COLUMN block_list_address TEXT;

