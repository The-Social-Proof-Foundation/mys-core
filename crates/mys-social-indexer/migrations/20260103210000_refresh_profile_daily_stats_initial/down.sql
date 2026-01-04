-- This migration performs a data refresh operation
-- There is no rollback needed as this only populates the materialized view
-- The data will continue to be maintained by the automatic refresh policy

-- Note: Rolling back this migration would require manually clearing the materialized data,
-- which is not recommended as it would require re-aggregating all historical data.
-- If needed, the continuous aggregate can be manually refreshed again using:
-- CALL refresh_continuous_aggregate('profile_daily_stats', NULL, NULL);

