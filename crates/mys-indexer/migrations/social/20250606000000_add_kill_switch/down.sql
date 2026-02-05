-- Enhanced rollback migration for emergency kill switch functionality only
-- Note: Diesel migrations run in transactions automatically, so BEGIN/COMMIT are not needed

-- Drop indexes for token exchange events
DROP INDEX IF EXISTS idx_token_exchange_events_event_id;
DROP INDEX IF EXISTS idx_token_exchange_events_created_at;
DROP INDEX IF EXISTS idx_token_exchange_events_type;

-- Drop token exchange events table
DROP TABLE IF EXISTS token_exchange_events;

-- Drop indexes for token exchange config
DROP INDEX IF EXISTS idx_token_exchange_config_trading_halted;
DROP INDEX IF EXISTS idx_token_exchange_config_updated_at;

-- Drop token exchange config table
DROP TABLE IF EXISTS token_exchange_config;