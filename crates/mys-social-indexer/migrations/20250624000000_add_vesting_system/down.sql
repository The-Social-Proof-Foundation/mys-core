-- ROLLBACK TOKEN VESTING SYSTEM
-- Remove vesting tables, indexes, functions, and compression policies

-- ============================================================================
-- 1. DROP VESTING UTILITY FUNCTIONS
-- ============================================================================

DROP FUNCTION IF EXISTS calculate_vesting_claimable(BIGINT, BIGINT, BIGINT, BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_vesting_status(BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_vesting_progress(BIGINT, BIGINT, BIGINT) CASCADE;

-- ============================================================================
-- 2. REMOVE COMPRESSION POLICIES
-- ============================================================================

-- Remove compression policy for vesting events
SELECT remove_compression_policy('vesting_events', if_exists => true);

-- ============================================================================
-- 3. DROP VESTING EVENTS TABLE AND INDEXES
-- ============================================================================

-- Drop vesting events indexes
DROP INDEX IF EXISTS idx_vesting_events_wallet_id;
DROP INDEX IF EXISTS idx_vesting_events_owner_address;
DROP INDEX IF EXISTS idx_vesting_events_event_type;
DROP INDEX IF EXISTS idx_vesting_events_event_time;

-- Drop vesting events table (TimescaleDB hypertable)
DROP TABLE IF EXISTS vesting_events CASCADE;

-- ============================================================================
-- 4. DROP VESTING WALLETS TABLE AND INDEXES  
-- ============================================================================

-- Drop vesting wallets indexes
DROP INDEX IF EXISTS idx_vesting_wallets_owner_address;
DROP INDEX IF EXISTS idx_vesting_wallets_start_time;
DROP INDEX IF EXISTS idx_vesting_wallets_curve_factor;
DROP INDEX IF EXISTS idx_vesting_wallets_created_at;

-- Drop vesting wallets table
DROP TABLE IF EXISTS vesting_wallets CASCADE;