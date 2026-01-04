-- Rollback for profile_daily_stats refresh migration
-- This migration only records that it ran, so rollback just removes the tracking entry
-- The materialized data in the continuous aggregate will remain (as it should)

-- Remove the tracking entry
DELETE FROM continuous_aggregate_refresh_status 
WHERE view_name = 'profile_daily_stats' 
AND notes = 'Migration recorded - refresh pending';

-- Note: The materialized data in the continuous aggregate will remain.
-- If you want to clear it, you would need to manually drop and recreate the aggregate,
-- but that's not recommended as it would require re-aggregating all historical data.
-- The automatic refresh policy will continue to maintain the aggregate going forward.

