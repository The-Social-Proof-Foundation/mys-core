-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Modify primary keys to include the partition column for TimescaleDB compatibility
-- TimescaleDB requires all unique indexes (including PKs) to include the partition key

-- Drop existing primary keys and recreate as composite keys
ALTER TABLE order_updates DROP CONSTRAINT IF EXISTS order_updates_pkey;
ALTER TABLE order_updates ADD CONSTRAINT order_updates_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE order_fills DROP CONSTRAINT IF EXISTS order_fills_pkey;
ALTER TABLE order_fills ADD CONSTRAINT order_fills_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE flashloans DROP CONSTRAINT IF EXISTS flashloans_pkey;
ALTER TABLE flashloans ADD CONSTRAINT flashloans_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE pool_prices DROP CONSTRAINT IF EXISTS pool_prices_pkey;
ALTER TABLE pool_prices ADD CONSTRAINT pool_prices_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE balances DROP CONSTRAINT IF EXISTS balances_pkey;
ALTER TABLE balances ADD CONSTRAINT balances_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE trade_params_update DROP CONSTRAINT IF EXISTS trade_params_update_pkey;
ALTER TABLE trade_params_update ADD CONSTRAINT trade_params_update_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE stakes DROP CONSTRAINT IF EXISTS stakes_pkey;
ALTER TABLE stakes ADD CONSTRAINT stakes_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE proposals DROP CONSTRAINT IF EXISTS proposals_pkey;
ALTER TABLE proposals ADD CONSTRAINT proposals_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE votes DROP CONSTRAINT IF EXISTS votes_pkey;
ALTER TABLE votes ADD CONSTRAINT votes_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

ALTER TABLE rebates DROP CONSTRAINT IF EXISTS rebates_pkey;
ALTER TABLE rebates ADD CONSTRAINT rebates_pkey PRIMARY KEY (event_digest, checkpoint_timestamp_ms);

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