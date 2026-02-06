-- Remove all views created in the migration
DROP VIEW IF EXISTS post_interactions;
DROP VIEW IF EXISTS top_tipped_content;
DROP VIEW IF EXISTS popular_posts;
DROP VIEW IF EXISTS trending_posts;

-- Drop continuous aggregates
DROP MATERIALIZED VIEW IF EXISTS post_stats_daily;
DROP MATERIALIZED VIEW IF EXISTS tips_hourly;
DROP MATERIALIZED VIEW IF EXISTS reposts_hourly;
DROP MATERIALIZED VIEW IF EXISTS reactions_hourly;

-- Remove entries from the tracking table
DELETE FROM continuous_aggregate_refresh_status 
WHERE view_name IN ('reactions_hourly', 'reposts_hourly', 'tips_hourly', 'post_stats_daily');

-- Drop all triggers
DROP TRIGGER IF EXISTS check_post_reference ON comments;
DROP TRIGGER IF EXISTS set_post_time ON posts;
DROP TRIGGER IF EXISTS set_comment_time ON comments;
DROP TRIGGER IF EXISTS set_reaction_time ON reactions;
DROP TRIGGER IF EXISTS set_repost_time ON reposts;
DROP TRIGGER IF EXISTS set_tip_time ON tips;
DROP TRIGGER IF EXISTS set_report_time ON posts_reports;
DROP TRIGGER IF EXISTS set_transfer_time ON posts_transfers;
DROP TRIGGER IF EXISTS set_moderation_time ON posts_moderation_events;
DROP TRIGGER IF EXISTS set_deletion_time ON posts_deletion_events;

-- Drop trigger functions
DROP FUNCTION IF EXISTS validate_post_reference();
DROP FUNCTION IF EXISTS update_post_time();
DROP FUNCTION IF EXISTS update_comment_time();
DROP FUNCTION IF EXISTS update_reaction_time();
DROP FUNCTION IF EXISTS update_repost_time();
DROP FUNCTION IF EXISTS update_tip_time();
DROP FUNCTION IF EXISTS update_report_time();
DROP FUNCTION IF EXISTS update_transfer_time();
DROP FUNCTION IF EXISTS update_moderation_time();
DROP FUNCTION IF EXISTS update_deletion_time();

-- Drop moderation and management tables 
DROP TABLE IF EXISTS posts_deletion_events;
DROP TABLE IF EXISTS posts_moderation_events;
DROP TABLE IF EXISTS posts_transfers;
DROP TABLE IF EXISTS posts_reports;

-- Drop interaction tables
DROP TABLE IF EXISTS tips;
DROP TABLE IF EXISTS reposts;
DROP TABLE IF EXISTS reaction_counts;
DROP TABLE IF EXISTS reactions;

-- Drop content tables (order matters due to foreign keys)
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts; 