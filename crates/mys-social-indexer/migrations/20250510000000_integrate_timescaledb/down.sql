-- CONSOLIDATED ROLLBACK FOR TIMESCALEDB INTEGRATION
-- Reverses the changes made in the up.sql migration
-- Rollbacks are executed in reverse order:
-- 5. Remove Refresh Continuous Aggregates
-- 4. Remove Checkpoint Processing Table
-- 3. Remove Continuous Aggregates
-- 2. Rollback Event Tables from Hypertables
-- 1. (Commented out) Drop TimescaleDB Extension

-- ============================================================================
-- 5. REMOVE REFRESH TRACKING TABLE (IF NEEDED)
-- ============================================================================
-- Remove the tracking table entries
DELETE FROM continuous_aggregate_refresh_status 
WHERE view_name IN (
    'social_graph_daily_stats',
    'profile_daily_stats',
    'platform_daily_stats',
    'checkpoint_daily_stats'
);

-- ============================================================================
-- 4. REMOVE CHECKPOINT PROCESSING TABLE
-- ============================================================================
-- Remove continuous aggregate policy 
SELECT remove_continuous_aggregate_policy('checkpoint_daily_stats', if_exists => TRUE);

-- Drop continuous aggregate
DROP MATERIALIZED VIEW IF EXISTS checkpoint_daily_stats;

-- Remove compression policy
SELECT remove_compression_policy('checkpoint_processing', if_exists => TRUE);

-- Drop chunks to clean up data (optional)
SELECT drop_chunks('checkpoint_processing', newer_than => '1970-01-01'::timestamp, older_than => NOW() + INTERVAL '1000 years');

-- Drop the hypertable (will drop all chunks as well)
DROP TABLE IF EXISTS checkpoint_processing;

-- ============================================================================
-- 3. REMOVE CONTINUOUS AGGREGATES
-- ============================================================================
-- Remove continuous aggregate policies
SELECT remove_continuous_aggregate_policy('social_graph_daily_stats', if_exists => TRUE);
SELECT remove_continuous_aggregate_policy('profile_daily_stats', if_exists => TRUE);
SELECT remove_continuous_aggregate_policy('platform_daily_stats', if_exists => TRUE);

-- Drop continuous aggregates
DROP MATERIALIZED VIEW IF EXISTS social_graph_daily_stats;
DROP MATERIALIZED VIEW IF EXISTS profile_daily_stats;
DROP MATERIALIZED VIEW IF EXISTS platform_daily_stats;

-- ============================================================================
-- 2. ROLLBACK EVENT TABLES FROM HYPERTABLES
-- ============================================================================
-- Drop compression policies
SELECT remove_compression_policy('social_graph_events', if_exists => TRUE);
SELECT remove_compression_policy('profile_events', if_exists => TRUE);
SELECT remove_compression_policy('platform_events', if_exists => TRUE);

-- Drop chunks to clean up data (optional)
SELECT drop_chunks('social_graph_events', newer_than => '1970-01-01'::timestamp, older_than => NOW() + INTERVAL '1000 years');
SELECT drop_chunks('profile_events', newer_than => '1970-01-01'::timestamp, older_than => NOW() + INTERVAL '1000 years');
SELECT drop_chunks('platform_events', newer_than => '1970-01-01'::timestamp, older_than => NOW() + INTERVAL '1000 years');

-- Note: There's no direct way to convert a hypertable back to a regular table.
-- These steps attempt to restore the original primary key structure, but the tables remain hypertables.

-- Restore original primary key for social_graph_events
ALTER TABLE social_graph_events DROP CONSTRAINT IF EXISTS social_graph_events_pkey;
ALTER TABLE social_graph_events ADD PRIMARY KEY (id);

-- Restore original primary key for profile_events
ALTER TABLE profile_events DROP CONSTRAINT IF EXISTS profile_events_pkey;
ALTER TABLE profile_events ADD PRIMARY KEY (id);

-- Restore original primary key for platform_events
ALTER TABLE platform_events DROP CONSTRAINT IF EXISTS platform_events_pkey;
ALTER TABLE platform_events ADD PRIMARY KEY (id);

-- ============================================================================
-- 1. REMOVE TIMESCALEDB EXTENSION (COMMENTED OUT FOR SAFETY)
-- ============================================================================
-- We cannot safely drop the extension if data exists in hypertables
-- This is a placeholder, manual intervention would be required to revert
-- DROP EXTENSION IF EXISTS timescaledb; 