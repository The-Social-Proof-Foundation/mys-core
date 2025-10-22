-- Rollback TimescaleDB cost optimization
-- This migration reverts compression and retention policy changes back to original settings
--
-- WARNING: This rollback will:
-- 1. Remove all retention policies (data will be kept forever again)
-- 2. Restore aggressive compression timing (7/30 days)
-- 3. Increase storage costs back to pre-optimization levels
--
-- Expected downtime: None (policies apply to new data)

-- Verify TimescaleDB extension is available
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        RAISE EXCEPTION 'TimescaleDB extension is not installed. Cannot proceed with rollback.';
    END IF;
    RAISE NOTICE 'TimescaleDB extension verified';
    RAISE WARNING 'Beginning rollback of cost optimization changes';
END $$;

-- Step 1: Remove retention policies (they didn't exist before)
DO $$
BEGIN
    RAISE NOTICE 'Step 1: Removing retention policies...';
    RAISE WARNING 'Data will no longer be automatically deleted - storage costs will increase';
END $$;

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

DO $$
BEGIN
    RAISE NOTICE 'Step 1: All retention policies removed successfully';
END $$;

-- Step 2: Remove current (optimized) compression policies
DO $$
BEGIN
    RAISE NOTICE 'Step 2: Removing optimized compression policies...';
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
    RAISE NOTICE 'Step 2: Optimized compression policies removed successfully';
END $$;

-- Step 3: Restore original compression policies from migration 2025012700000000
DO $$
BEGIN
    RAISE NOTICE 'Step 3: Restoring original compression policies...';
    RAISE WARNING 'Reverting to aggressive compression timing (7/30 days)';
END $$;

-- High-frequency trading data: Compress after 7 days (604,800,000 ms) - ORIGINAL
SELECT add_compression_policy('order_fills', 604800000);
SELECT add_compression_policy('order_updates', 604800000);

DO $$
BEGIN
    RAISE NOTICE 'Step 3a: Trading data compression restored (7 days)';
END $$;

-- Medium-frequency data: Compress after 30 days (2,592,000,000 ms) - ORIGINAL
SELECT add_compression_policy('pool_prices', 2592000000);
SELECT add_compression_policy('balances', 2592000000);

DO $$
BEGIN
    RAISE NOTICE 'Step 3b: Price/balance compression restored (30 days)';
END $$;

-- Low-frequency data: Compress after 7 days (604,800,000 ms) - ORIGINAL
SELECT add_compression_policy('flashloans', 604800000);
SELECT add_compression_policy('stakes', 604800000);
SELECT add_compression_policy('proposals', 604800000);
SELECT add_compression_policy('votes', 604800000);
SELECT add_compression_policy('rebates', 604800000);
SELECT add_compression_policy('trade_params_update', 604800000);

DO $$
BEGIN
    RAISE NOTICE 'Step 3c: Governance/low-frequency compression restored (7 days)';
    RAISE NOTICE 'Step 3: All original compression policies restored successfully';
END $$;

-- Final verification and summary
DO $$
DECLARE
    compression_count INTEGER;
    retention_count INTEGER;
BEGIN
    -- Verify compression policies were restored
    SELECT COUNT(*) INTO compression_count
    FROM timescaledb_information.jobs
    WHERE proc_name = 'policy_compression';
    
    -- Verify retention policies were removed
    SELECT COUNT(*) INTO retention_count
    FROM timescaledb_information.jobs
    WHERE proc_name = 'policy_retention';
    
    IF compression_count < 10 THEN
        RAISE WARNING 'Expected 10 compression policies but found %', compression_count;
    END IF;
    
    IF retention_count > 0 THEN
        RAISE WARNING 'Expected 0 retention policies but found %', retention_count;
    END IF;
    
    RAISE NOTICE '========================================';
    RAISE NOTICE 'TimescaleDB Optimization Rollback Complete';
    RAISE NOTICE '========================================';
    RAISE NOTICE '';
    RAISE NOTICE 'Summary of Changes:';
    RAISE NOTICE '-------------------';
    RAISE NOTICE 'Compression Policies Restored:';
    RAISE NOTICE '  ✓ Trading data (order_fills, order_updates): 7 days (reverted from 30 days)';
    RAISE NOTICE '  ✓ Price/balance data (pool_prices, balances): 30 days (reverted from 90 days)';
    RAISE NOTICE '  ✓ Governance data: 7 days (reverted from 30 days)';
    RAISE NOTICE '';
    RAISE NOTICE 'Retention Policies Removed:';
    RAISE NOTICE '  ✓ All retention policies removed';
    RAISE NOTICE '  ✓ Data will be kept indefinitely (no automatic deletion)';
    RAISE NOTICE '';
    RAISE WARNING 'Expected Impact:';
    RAISE WARNING '  • Storage costs will INCREASE back to pre-optimization levels';
    RAISE WARNING '  • Data will accumulate indefinitely without retention policies';
    RAISE WARNING '  • Compression overhead will INCREASE (more frequent operations)';
    RAISE WARNING '  • Monitor storage growth and costs closely';
    RAISE NOTICE '';
    RAISE NOTICE 'Active Policies:';
    RAISE NOTICE '  • Compression policies: % configured', compression_count;
    RAISE NOTICE '  • Retention policies: % configured (should be 0)', retention_count;
    RAISE NOTICE '';
    RAISE NOTICE 'Consider re-applying optimization when ready: Run up.sql migration';
    RAISE NOTICE '========================================';
END $$;

