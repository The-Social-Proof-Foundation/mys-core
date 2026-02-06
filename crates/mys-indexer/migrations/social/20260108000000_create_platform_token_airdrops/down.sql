-- Drop indexes
DROP INDEX IF EXISTS idx_platform_token_airdrops_platform_timestamp;
DROP INDEX IF EXISTS idx_platform_token_airdrops_executed_by;
DROP INDEX IF EXISTS idx_platform_token_airdrops_timestamp;
DROP INDEX IF EXISTS idx_platform_token_airdrops_recipient;
DROP INDEX IF EXISTS idx_platform_token_airdrops_platform_id;

-- Drop table
DROP TABLE IF EXISTS platform_token_airdrops;

