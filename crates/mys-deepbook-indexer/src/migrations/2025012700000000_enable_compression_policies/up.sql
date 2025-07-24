-- Enable compression policies for TimescaleDB hypertables
-- This will provide 80-90% storage cost reduction for older chunks

-- High-frequency trading data: Compress after 24 hours
-- These tables have frequent writes but older data is accessed less often
-- 24 hours keeps recent trading data uncompressed for fast queries

-- Order fills (trades) - most frequently accessed recent data
SELECT add_compression_policy('order_fills', INTERVAL '24 hours');

-- Order updates - active orders change frequently 
SELECT add_compression_policy('order_updates', INTERVAL '24 hours');

-- Medium-frequency data: Compress after 48 hours
-- These tables have moderate write frequency

-- Pool prices - price updates are important but less frequent than trades
SELECT add_compression_policy('pool_prices', INTERVAL '48 hours');

-- Balance changes - important for recent analysis but older data can be compressed
SELECT add_compression_policy('balances', INTERVAL '48 hours');

-- Lower-frequency data: Compress after 24 hours  
-- These tables have infrequent writes so can be compressed sooner

-- Flash loan events - relatively rare events
SELECT add_compression_policy('flashloans', INTERVAL '24 hours');

-- Governance-related tables - infrequent updates
SELECT add_compression_policy('stakes', INTERVAL '24 hours');
SELECT add_compression_policy('proposals', INTERVAL '24 hours');
SELECT add_compression_policy('votes', INTERVAL '24 hours');
SELECT add_compression_policy('rebates', INTERVAL '24 hours');

-- Trade parameters - very infrequent updates
SELECT add_compression_policy('trade_params_update', INTERVAL '24 hours');

-- Create a view to monitor compression status
CREATE OR REPLACE VIEW compression_status AS
SELECT 
    hypertable_name,
    compression_enabled,
    compress_after,
    total_chunks,
    number_compressed_chunks,
    compressed_heap_size,
    uncompressed_heap_size,
    CASE 
        WHEN uncompressed_heap_size > 0 THEN 
            ROUND((1 - compressed_heap_size::numeric / uncompressed_heap_size::numeric) * 100, 2)
        ELSE 0 
    END AS compression_ratio_percent
FROM timescaledb_information.compression_settings cs
JOIN timescaledb_information.hypertables h ON cs.hypertable_name = h.hypertable_name
LEFT JOIN timescaledb_information.chunks c ON h.hypertable_name = c.hypertable_name
GROUP BY 
    cs.hypertable_name, 
    compression_enabled, 
    compress_after,
    compressed_heap_size,
    uncompressed_heap_size,
    total_chunks,
    number_compressed_chunks;

-- Log compression policy creation
DO $$
BEGIN
    RAISE NOTICE 'TimescaleDB compression policies enabled successfully';
    RAISE NOTICE 'High-frequency tables (order_fills, order_updates): 24h compression';
    RAISE NOTICE 'Medium-frequency tables (pool_prices, balances): 48h compression';  
    RAISE NOTICE 'Low-frequency tables (governance, flashloans): 24h compression';
    RAISE NOTICE 'Use SELECT * FROM compression_status; to monitor compression effectiveness';
END $$; 