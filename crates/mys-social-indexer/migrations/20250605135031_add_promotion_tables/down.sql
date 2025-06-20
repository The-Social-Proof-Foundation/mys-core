-- Remove promotion analytics views
DROP VIEW IF EXISTS top_promoted_posts;
DROP VIEW IF EXISTS promotion_performance;
DROP VIEW IF EXISTS active_promoted_posts;

-- Drop continuous aggregates
DROP MATERIALIZED VIEW IF EXISTS promotion_spending_daily;
DROP MATERIALIZED VIEW IF EXISTS promotion_views_hourly;

-- Remove entries from the tracking table
DELETE FROM continuous_aggregate_refresh_status 
WHERE view_name IN ('promotion_views_hourly', 'promotion_spending_daily');

-- Drop all promotion triggers
DROP TRIGGER IF EXISTS set_promoted_post_time ON promoted_posts;
DROP TRIGGER IF EXISTS set_promotion_view_time ON promotion_views;
DROP TRIGGER IF EXISTS set_promotion_status_event_time ON promotion_status_events;
DROP TRIGGER IF EXISTS set_promotion_budget_event_time ON promotion_budget_events;

-- Drop trigger functions
DROP FUNCTION IF EXISTS update_promoted_post_time();
DROP FUNCTION IF EXISTS update_promotion_view_time();
DROP FUNCTION IF EXISTS update_promotion_status_event_time();
DROP FUNCTION IF EXISTS update_promotion_budget_event_time();

-- Drop promotion tables
DROP TABLE IF EXISTS promotion_budget_events;
DROP TABLE IF EXISTS promotion_status_events;
DROP TABLE IF EXISTS promotion_views;
DROP TABLE IF EXISTS promoted_posts;

-- Remove promotion_id column from posts table
ALTER TABLE posts DROP COLUMN IF EXISTS promotion_id;