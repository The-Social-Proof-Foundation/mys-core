-- Refresh profile_daily_stats continuous aggregate to populate historical data
-- This backfills all historical data from profile_events into the materialized hypertable
-- The aggregate was created with WITH NO DATA and needs an initial refresh
--
-- NOTE: refresh_continuous_aggregate() cannot run inside a transaction block.
-- Diesel migrations run in transactions, so we cannot call refresh_continuous_aggregate here.
-- Instead, this migration just records that it ran. The actual refresh will be performed
-- by application startup code after migrations complete (see src/db.rs post_migration_refresh).

-- Update the tracking table to record that this migration ran
-- The actual refresh will happen in application startup code
INSERT INTO continuous_aggregate_refresh_status (view_name, last_manual_refresh, notes)
VALUES ('profile_daily_stats', NOW(), 'Migration recorded - refresh pending')
ON CONFLICT (view_name) DO UPDATE
SET last_manual_refresh = NOW(),
    notes = 'Migration recorded - refresh pending';

