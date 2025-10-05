-- Rollback native MYS treasury tracking

DROP TRIGGER IF EXISTS update_bridge_treasury_balances_updated_at ON bridge_treasury_balances;
DROP INDEX IF EXISTS idx_bridge_treasury_events_tx_digest;
DROP INDEX IF EXISTS idx_bridge_treasury_events_timestamp_ms;
DROP INDEX IF EXISTS idx_bridge_treasury_events_block_height;
DROP INDEX IF EXISTS idx_bridge_treasury_events_event_type;
DROP INDEX IF EXISTS idx_bridge_treasury_events_token_type;
DROP INDEX IF EXISTS idx_bridge_treasury_balances_token_id;
DROP INDEX IF EXISTS idx_bridge_treasury_balances_token_type;
DROP TABLE IF EXISTS bridge_treasury_events;
DROP TABLE IF EXISTS bridge_treasury_balances;

