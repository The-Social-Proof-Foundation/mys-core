-- Remove compression policies
SELECT remove_compression_policy('order_fills', if_exists => true);
SELECT remove_compression_policy('order_updates', if_exists => true);
SELECT remove_compression_policy('pool_prices', if_exists => true);
SELECT remove_compression_policy('balances', if_exists => true);
SELECT remove_compression_policy('flashloans', if_exists => true);
SELECT remove_compression_policy('stakes', if_exists => true);
SELECT remove_compression_policy('proposals', if_exists => true);
SELECT remove_compression_policy('votes', if_exists => true);
SELECT remove_compression_policy('rebates', if_exists => true);
SELECT remove_compression_policy('trade_params_update', if_exists => true);

-- Remove retention policies (if they were enabled)
SELECT remove_retention_policy('order_fills', if_exists => true);
SELECT remove_retention_policy('order_updates', if_exists => true);
SELECT remove_retention_policy('pool_prices', if_exists => true);
SELECT remove_retention_policy('balances', if_exists => true);
SELECT remove_retention_policy('flashloans', if_exists => true);
SELECT remove_retention_policy('stakes', if_exists => true);
SELECT remove_retention_policy('proposals', if_exists => true);
SELECT remove_retention_policy('votes', if_exists => true);
SELECT remove_retention_policy('rebates', if_exists => true);
SELECT remove_retention_policy('trade_params_update', if_exists => true);

-- Drop time-series specific indexes
DROP INDEX IF EXISTS idx_order_fills_pool_time;
DROP INDEX IF EXISTS idx_order_updates_pool_time;
DROP INDEX IF EXISTS idx_balances_manager_time;
DROP INDEX IF EXISTS idx_balances_asset_time;

-- Convert hypertables back to regular tables
-- Note: This will preserve data but lose time-series optimizations
SELECT drop_chunks('order_fills', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('order_updates', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('pool_prices', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('balances', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('flashloans', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('stakes', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('proposals', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('votes', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('rebates', older_than => INTERVAL '0 seconds', cascade => true);
SELECT drop_chunks('trade_params_update', older_than => INTERVAL '0 seconds', cascade => true);

-- Disable TimescaleDB extension (optional - comment out if other schemas use it)
-- DROP EXTENSION IF EXISTS timescaledb CASCADE; 