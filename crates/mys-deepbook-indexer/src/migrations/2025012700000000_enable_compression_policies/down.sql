-- Remove compression policies for TimescaleDB hypertables
-- This rollback will disable compression but preserve existing compressed chunks

-- Remove compression monitoring view
DROP VIEW IF EXISTS compression_status;

-- Remove compression policies for all hypertables
-- Note: This stops future compression but doesn't decompress existing chunks
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

-- Log rollback completion
DO $$
BEGIN
    RAISE NOTICE 'TimescaleDB compression policies removed successfully';
    RAISE NOTICE 'Existing compressed chunks remain compressed';
    RAISE NOTICE 'To decompress existing chunks, run: SELECT decompress_chunk(chunk_name);';
END $$; 