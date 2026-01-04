-- Refresh profile_daily_stats continuous aggregate to populate historical data
-- This backfills all historical data from profile_events into the materialized hypertable
-- The aggregate was created with WITH NO DATA and needs an initial refresh
--
-- NOTE: refresh_continuous_aggregate() cannot run inside a transaction block,
-- so we commit the transaction first, then execute the refresh in autocommit mode

-- Commit any existing transaction to allow refresh_continuous_aggregate to run
COMMIT;

-- Refresh the continuous aggregate with all historical data
-- NULL, NULL means refresh all available data
CALL refresh_continuous_aggregate('profile_daily_stats', NULL, NULL);

-- Update the tracking table to record this refresh
INSERT INTO continuous_aggregate_refresh_status (view_name, last_manual_refresh, notes)
VALUES ('profile_daily_stats', NOW(), 'Initial historical data refresh')
ON CONFLICT (view_name) DO UPDATE
SET last_manual_refresh = NOW(),
    notes = 'Initial historical data refresh';

