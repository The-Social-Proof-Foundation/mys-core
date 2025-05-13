-- SOCIAL PROOF TOKEN SYSTEM - DOWN MIGRATION
-- Drop all objects created in the up migration

-- ============================================================================
-- 1. DROP REFRESH POLICIES AND FUNCTIONS FOR MATERIALIZED VIEWS
-- ============================================================================

-- Try to drop any scheduled jobs first
SELECT remove_job(job_id) FROM timescaledb_information.jobs 
WHERE proc_name = 'refresh_spt_price_hourly' OR proc_name = 'refresh_spt_price_daily';

-- Drop the refresh functions
DROP FUNCTION IF EXISTS refresh_spt_price_hourly() CASCADE;
DROP FUNCTION IF EXISTS refresh_spt_price_daily() CASCADE;

-- ============================================================================
-- 2. DROP COMPRESSION POLICIES
-- ============================================================================

-- Remove compression policies
SELECT remove_compression_policy('social_proof_token_pools', if_exists => true);
SELECT remove_compression_policy('spt_holdings', if_exists => true);
SELECT remove_compression_policy('spt_transactions', if_exists => true);
SELECT remove_compression_policy('spt_auction_pools', if_exists => true);
SELECT remove_compression_policy('spt_auction_contributions', if_exists => true);
SELECT remove_compression_policy('spt_price_history', if_exists => true);

-- ============================================================================
-- 3. DROP VIEWS AND MATERIALIZED VIEWS
-- ============================================================================

-- Drop regular views
DROP VIEW IF EXISTS active_token_pools CASCADE;
DROP VIEW IF EXISTS popular_token_pools CASCADE;

-- Drop materialized views
DROP MATERIALIZED VIEW IF EXISTS spt_price_hourly CASCADE;
DROP MATERIALIZED VIEW IF EXISTS spt_price_daily CASCADE;

-- ============================================================================
-- 4. DROP TABLES
-- ============================================================================

-- Drop price history table and indexes
DROP INDEX IF EXISTS idx_spt_price_history_pool_id;
DROP TABLE IF EXISTS spt_price_history CASCADE;

-- Drop auction contributions table and indexes
DROP INDEX IF EXISTS idx_spt_auction_contributions_auction_id;
DROP INDEX IF EXISTS idx_spt_auction_contributions_contributor_address;
DROP TABLE IF EXISTS spt_auction_contributions CASCADE;

-- Drop auction pools table and indexes
DROP INDEX IF EXISTS idx_spt_auction_pools_auction_id;
DROP INDEX IF EXISTS idx_spt_auction_pools_associated_id;
DROP INDEX IF EXISTS idx_spt_auction_pools_owner;
DROP INDEX IF EXISTS idx_spt_auction_pools_status;
DROP TABLE IF EXISTS spt_auction_pools CASCADE;

-- Drop transactions table and indexes
DROP INDEX IF EXISTS idx_spt_transactions_pool_id;
DROP INDEX IF EXISTS idx_spt_transactions_sender;
DROP INDEX IF EXISTS idx_spt_transactions_transaction_type;
DROP TABLE IF EXISTS spt_transactions CASCADE;

-- Drop holdings table and indexes
DROP INDEX IF EXISTS idx_spt_holdings_pool_id;
DROP INDEX IF EXISTS idx_spt_holdings_holder_address;
DROP TABLE IF EXISTS spt_holdings CASCADE;

-- Drop token pools table and indexes
DROP INDEX IF EXISTS idx_spt_pools_pool_id;
DROP INDEX IF EXISTS idx_spt_pools_owner;
DROP INDEX IF EXISTS idx_spt_pools_token_type;
DROP INDEX IF EXISTS idx_spt_pools_associated_id;
DROP TABLE IF EXISTS social_proof_token_pools CASCADE; 