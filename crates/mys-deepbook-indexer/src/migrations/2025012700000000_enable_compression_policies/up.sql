-- Enable compression policies for TimescaleDB hypertables
-- This will provide 80-90% storage cost reduction for older chunks

-- Step 1: Enable compression on all hypertables first
-- We need to specify which columns to use for segmentation and ordering

-- Order fills (trades) - segment by pool, order by time descending
ALTER TABLE order_fills SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'pool_id',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Order updates - segment by pool, order by time descending  
ALTER TABLE order_updates SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'pool_id',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Pool prices - segment by target pool, order by time descending
ALTER TABLE pool_prices SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'target_pool',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Balances - segment by balance manager, order by time descending
ALTER TABLE balances SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'balance_manager_id',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Flashloans - order by time descending (no natural segment column)
ALTER TABLE flashloans SET (
    timescaledb.compress,
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Stakes - segment by balance manager, order by time descending
ALTER TABLE stakes SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'balance_manager_id',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Proposals - order by time descending (proposals are unique)
ALTER TABLE proposals SET (
    timescaledb.compress,
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Votes - segment by balance manager, order by time descending
ALTER TABLE votes SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'balance_manager_id',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Rebates - segment by balance manager, order by time descending
ALTER TABLE rebates SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'balance_manager_id',
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Trade params updates - order by time descending
ALTER TABLE trade_params_update SET (
    timescaledb.compress,
    timescaledb.compress_orderby = 'checkpoint_timestamp_ms DESC'
);

-- Step 2: Add compression policies
-- High-frequency trading data: Compress after 7 days (604800000 ms)
-- These tables have frequent writes but older data is accessed less often
-- 7 days keeps recent trading data uncompressed for fast queries

-- Order fills (trades) - most frequently accessed recent data
SELECT add_compression_policy('order_fills', 604800000::BIGINT);

-- Order updates - active orders change frequently 
SELECT add_compression_policy('order_updates', 604800000::BIGINT);

-- Medium-frequency data: Compress after 30 days (2592000000 ms)
-- These tables have moderate write frequency

-- Pool prices - price updates are important but less frequent than trades
SELECT add_compression_policy('pool_prices', 2592000000::BIGINT);

-- Balance changes - important for recent analysis but older data can be compressed
SELECT add_compression_policy('balances', 2592000000::BIGINT);

-- Lower-frequency data: Compress after 7 days (604800000 ms)
-- These tables have infrequent writes so can be compressed sooner

-- Flash loan events - relatively rare events
SELECT add_compression_policy('flashloans', 604800000::BIGINT);

-- Governance-related tables - infrequent updates
SELECT add_compression_policy('stakes', 604800000::BIGINT);
SELECT add_compression_policy('proposals', 604800000::BIGINT);
SELECT add_compression_policy('votes', 604800000::BIGINT);
SELECT add_compression_policy('rebates', 604800000::BIGINT);

-- Trade parameters - very infrequent updates
SELECT add_compression_policy('trade_params_update', 604800000::BIGINT);

-- Create a view to monitor compression status
CREATE OR REPLACE VIEW compression_status AS
SELECT 
    h.hypertable_schema,
    h.hypertable_name,
    h.compression_enabled,
    h.total_chunks,
    h.number_compressed_chunks,
    pg_size_pretty(h.before_compression_total_bytes) AS uncompressed_size,
    pg_size_pretty(h.after_compression_total_bytes) AS compressed_size,
    CASE 
        WHEN h.before_compression_total_bytes > 0 THEN 
            ROUND((1 - h.after_compression_total_bytes::numeric / h.before_compression_total_bytes::numeric) * 100, 2)
        ELSE 0 
    END AS compression_ratio_percent
FROM timescaledb_information.hypertables h
WHERE h.compression_enabled = true;

-- Log compression policy creation
DO $$
BEGIN
    RAISE NOTICE 'TimescaleDB compression policies enabled successfully';
    RAISE NOTICE 'High-frequency tables (order_fills, order_updates): 7 days compression';
    RAISE NOTICE 'Medium-frequency tables (pool_prices, balances): 30 days compression';  
    RAISE NOTICE 'Low-frequency tables (governance, flashloans): 7 days compression';
    RAISE NOTICE 'Use SELECT * FROM compression_status; to monitor compression effectiveness';
END $$; 