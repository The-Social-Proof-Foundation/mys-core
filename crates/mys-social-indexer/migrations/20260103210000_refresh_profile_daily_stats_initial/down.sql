-- This migration performs a data refresh operation
-- There is no rollback needed as this only populates the materialized view
-- The data will continue to be maintained by the automatic refresh policy
--
-- NOTE: refresh_continuous_aggregate() cannot run inside a transaction block,
-- so we commit the transaction first, then execute operations in autocommit mode

-- Commit any existing transaction
COMMIT;

-- Remove the tracking entry (optional - the refresh policy will continue to maintain the aggregate)
DELETE FROM continuous_aggregate_refresh_status 
WHERE view_name = 'profile_daily_stats' 
AND notes = 'Initial historical data refresh';

-- Note: The materialized data in the continuous aggregate will remain.
-- If you want to clear it, you would need to manually drop and recreate the aggregate,
-- but that's not recommended as it would require re-aggregating all historical data.
-- The automatic refresh policy will continue to maintain the aggregate going forward.

