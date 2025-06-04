-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Convert time-sensitive tables to hypertables
-- These tables contain time-series data that will benefit from TimescaleDB's optimizations

-- Order fills (trades) - most frequently queried time-series data
-- Using smaller chunk intervals for better query performance on recent data
SELECT create_hypertable('order_fills', 'checkpoint_timestamp_ms', 
    chunk_time_interval => 3600000, -- 1 hour chunks (in milliseconds) for high-frequency trading data
    if_not_exists => TRUE
);

-- Order updates - order state changes over time
SELECT create_hypertable('order_updates', 'checkpoint_timestamp_ms',
    chunk_time_interval => 3600000, -- 1 hour chunks for active order tracking
    if_not_exists => TRUE
);

-- Pool prices - price updates over time
SELECT create_hypertable('pool_prices', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks (price updates are less frequent)
    if_not_exists => TRUE
);

-- Balance changes over time
SELECT create_hypertable('balances', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Flash loan events
SELECT create_hypertable('flashloans', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Stakes events
SELECT create_hypertable('stakes', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Proposals events
SELECT create_hypertable('proposals', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Votes events
SELECT create_hypertable('votes', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Rebates events
SELECT create_hypertable('rebates', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Trade params updates
SELECT create_hypertable('trade_params_update', 'checkpoint_timestamp_ms',
    chunk_time_interval => 86400000, -- 1 day chunks
    if_not_exists => TRUE
);

-- Create optimized indexes for order book queries
-- These indexes are critical for high-performance order book operations

-- Index on pool_id and time for order_fills (most common query pattern)
-- This index enables fast queries for specific trading pairs by time
CREATE INDEX IF NOT EXISTS idx_order_fills_pool_time 
ON order_fills (pool_id, checkpoint_timestamp_ms DESC);

-- Index for balance manager queries on order fills
CREATE INDEX IF NOT EXISTS idx_order_fills_maker_time
ON order_fills (maker_balance_manager_id, checkpoint_timestamp_ms DESC);

CREATE INDEX IF NOT EXISTS idx_order_fills_taker_time
ON order_fills (taker_balance_manager_id, checkpoint_timestamp_ms DESC);

-- Index on pool_id and time for order_updates
CREATE INDEX IF NOT EXISTS idx_order_updates_pool_time 
ON order_updates (pool_id, checkpoint_timestamp_ms DESC);

-- Index for balance manager queries on order updates
CREATE INDEX IF NOT EXISTS idx_order_updates_manager_time
ON order_updates (balance_manager_id, checkpoint_timestamp_ms DESC);

-- Index on balance_manager_id and time for balances
CREATE INDEX IF NOT EXISTS idx_balances_manager_time 
ON balances (balance_manager_id, checkpoint_timestamp_ms DESC);

-- Index on asset and time for balances (for asset-specific queries)
CREATE INDEX IF NOT EXISTS idx_balances_asset_time 
ON balances (asset, checkpoint_timestamp_ms DESC);

-- Index for price queries by pool
CREATE INDEX IF NOT EXISTS idx_pool_prices_target_time
ON pool_prices (target_pool, checkpoint_timestamp_ms DESC);

-- Enable compression for older chunks (data older than 7 days)
-- This significantly reduces storage costs while preserving all data forever
-- Compression ratio typically 90%+ for time-series data
SELECT add_compression_policy('order_fills', INTERVAL '7 days');
SELECT add_compression_policy('order_updates', INTERVAL '7 days');
SELECT add_compression_policy('pool_prices', INTERVAL '7 days');
SELECT add_compression_policy('balances', INTERVAL '7 days');
SELECT add_compression_policy('flashloans', INTERVAL '7 days');
SELECT add_compression_policy('stakes', INTERVAL '7 days');
SELECT add_compression_policy('proposals', INTERVAL '7 days');
SELECT add_compression_policy('votes', INTERVAL '7 days');
SELECT add_compression_policy('rebates', INTERVAL '7 days');
SELECT add_compression_policy('trade_params_update', INTERVAL '7 days');

-- NO RETENTION POLICIES - Data is kept forever for complete order book history
-- Order book data is critical financial data that must be preserved indefinitely 