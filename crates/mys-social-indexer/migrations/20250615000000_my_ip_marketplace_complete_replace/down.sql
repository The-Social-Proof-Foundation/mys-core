-- REVERSE MYIP DATA MARKETPLACE - COMPLETE SYSTEM ROLLBACK
-- This down migration removes all marketplace system components

-- ============================================================================
-- 1. DROP CONTINUOUS AGGREGATE POLICIES
-- ============================================================================
SELECT remove_continuous_aggregate_policy('my_ip_daily_revenue', if_exists => true);
SELECT remove_continuous_aggregate_policy('my_ip_daily_access', if_exists => true);
SELECT remove_continuous_aggregate_policy('my_ip_popular_data', if_exists => true);

-- ============================================================================
-- 2. DROP CONTINUOUS AGGREGATES
-- ============================================================================
DROP MATERIALIZED VIEW IF EXISTS my_ip_daily_revenue CASCADE;
DROP MATERIALIZED VIEW IF EXISTS my_ip_daily_access CASCADE;
DROP MATERIALIZED VIEW IF EXISTS my_ip_popular_data CASCADE;

-- ============================================================================
-- 3. DROP VIEWS
-- ============================================================================
DROP VIEW IF EXISTS active_marketplace_data CASCADE;
DROP VIEW IF EXISTS popular_data_30d CASCADE;
DROP VIEW IF EXISTS creator_revenue_summary CASCADE;

-- ============================================================================
-- 4. DROP FUNCTIONS
-- ============================================================================
DROP FUNCTION IF EXISTS user_has_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_data_pricing(TEXT) CASCADE;

-- ============================================================================
-- 5. REMOVE COMPRESSION AND RETENTION POLICIES
-- ============================================================================
SELECT remove_compression_policy('my_ip_purchases', if_exists => true);
SELECT remove_compression_policy('my_ip_subscriptions', if_exists => true);
SELECT remove_compression_policy('my_ip_revenue', if_exists => true);
SELECT remove_compression_policy('my_ip_access_logs', if_exists => true);

SELECT remove_retention_policy('my_ip_access_logs', if_exists => true);

-- ============================================================================
-- 6. DROP MARKETPLACE TABLES
-- ============================================================================
DROP TABLE IF EXISTS my_ip_access_logs CASCADE;
DROP TABLE IF EXISTS my_ip_revenue CASCADE;
DROP TABLE IF EXISTS my_ip_subscriptions CASCADE;
DROP TABLE IF EXISTS my_ip_purchases CASCADE;
DROP TABLE IF EXISTS my_ip_data CASCADE;