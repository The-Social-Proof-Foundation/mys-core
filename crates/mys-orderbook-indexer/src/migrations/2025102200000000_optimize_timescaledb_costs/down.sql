-- Rollback TimescaleDB compression timing optimization
-- Reverts compression timing back to original aggressive settings (7/30 days)
--
-- NOTE: Our hypertables use BIGINT timestamps (checkpoint_timestamp_ms in milliseconds)
-- Therefore compression policies must use INTEGER values, not INTERVAL
--
-- Expected downtime: None

-- Verify TimescaleDB extension is available
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        RAISE EXCEPTION 'TimescaleDB extension is not installed. Cannot proceed.';
    END IF;
    RAISE NOTICE 'TimescaleDB extension verified';
    RAISE WARNING 'Beginning rollback of compression optimization';
END $$;

-- Step 1: Remove optimized compression policies
DO $$
BEGIN
    RAISE NOTICE 'Step 1: Removing optimized compression policies...';
END $$;

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

DO $$
BEGIN
    RAISE NOTICE 'Step 1: Optimized compression policies removed';
END $$;

-- Step 2: Restore original aggressive compression timing
-- Using INTEGER milliseconds for BIGINT timestamp columns
-- 7 days = 604,800,000 ms
-- 30 days = 2,592,000,000 ms
DO $$
BEGIN
    RAISE NOTICE 'Step 2: Restoring original compression policies...';
    RAISE WARNING 'Reverting to aggressive compression timing (7/30 days)';
END $$;

-- Trading data: 7 days (604,800,000 ms) - original
SELECT add_compression_policy('order_fills', BIGINT '604800000');
SELECT add_compression_policy('order_updates', BIGINT '604800000');

-- Price/balance data: 30 days (2,592,000,000 ms) - original
SELECT add_compression_policy('pool_prices', BIGINT '2592000000');
SELECT add_compression_policy('balances', BIGINT '2592000000');

-- Low-frequency data: 7 days (604,800,000 ms) - original
SELECT add_compression_policy('flashloans', BIGINT '604800000');
SELECT add_compression_policy('stakes', BIGINT '604800000');
SELECT add_compression_policy('proposals', BIGINT '604800000');
SELECT add_compression_policy('votes', BIGINT '604800000');
SELECT add_compression_policy('rebates', BIGINT '604800000');
SELECT add_compression_policy('trade_params_update', BIGINT '604800000');

-- Final summary
DO $$
DECLARE
    compression_count INTEGER;
BEGIN
    SELECT COUNT(*) INTO compression_count
    FROM timescaledb_information.jobs
    WHERE proc_name = 'policy_compression';
    
    RAISE NOTICE '========================================';
    RAISE NOTICE 'TimescaleDB Optimization Rollback Complete';
    RAISE NOTICE '========================================';
    RAISE NOTICE '';
    RAISE NOTICE 'Compression Policies Restored:';
    RAISE NOTICE '  ✓ Trading data: 7 days (reverted from 30 days)';
    RAISE NOTICE '  ✓ Price/balance data: 30 days (reverted from 90 days)';
    RAISE NOTICE '  ✓ Governance data: 7 days (reverted from 30 days)';
    RAISE NOTICE '';
    RAISE WARNING 'Storage costs will increase back to original levels';
    RAISE NOTICE 'Active compression policies: %', compression_count;
    RAISE NOTICE '========================================';
END $$;
