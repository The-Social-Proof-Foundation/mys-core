-- Remove block_list_address columns (no longer needed with unified table architecture)
ALTER TABLE profiles DROP COLUMN IF EXISTS block_list_address;
DROP INDEX IF EXISTS idx_profiles_block_list_address;

ALTER TABLE blocked_profiles DROP COLUMN IF EXISTS block_list_address;
DROP INDEX IF EXISTS idx_blocked_profiles_block_list;

ALTER TABLE blocked_events DROP COLUMN IF EXISTS block_list_address;

