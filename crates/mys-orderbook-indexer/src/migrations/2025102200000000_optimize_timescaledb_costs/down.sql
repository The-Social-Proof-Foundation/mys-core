-- Rollback TimescaleDB compression timing optimization
-- This migration reverts compression timing back to original aggressive settings
--
-- WARNING: This rollback will restore aggressive compression (7/30 days)
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
DO $$
BEGIN
    RAISE NOTICE 'Step 2: Restoring original compression policies...';
    RAISE WARNING 'Reverting to aggressive compression timing (7/30 days)';
END $$;

-- Trading data: 7 days (original)
SELECT add_compression_policy('order_fills', INTERVAL '7 days');
SELECT add_compression_policy('order_updates', INTERVAL '7 days');

-- Price/balance data: 30 days (original)
SELECT add_compression_policy('pool_prices', INTERVAL '30 days');
SELECT add_compression_policy('balances', INTERVAL '30 days');

-- Low-frequency data: 7 days (original)
SELECT add_compression_policy('flashloans', INTERVAL '7 days');
SELECT add_compression_policy('stakes', INTERVAL '7 days');
SELECT add_compression_policy('proposals', INTERVAL '7 days');
SELECT add_compression_policy('votes', INTERVAL '7 days');
SELECT add_compression_policy('rebates', INTERVAL '7 days');
SELECT add_compression_policy('trade_params_update', INTERVAL '7 days');

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
