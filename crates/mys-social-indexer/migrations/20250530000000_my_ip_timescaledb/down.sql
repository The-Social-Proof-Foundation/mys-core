-- DROP SCHEDULED JOBS AND COMPRESSION POLICIES
DO $$
BEGIN
    PERFORM remove_job(job_id) FROM timescaledb_information.jobs 
    WHERE proc_name = 'refresh_license_materialized_views';
END $$;

SELECT remove_compression_policy('my_ip', if_exists => true);
SELECT remove_compression_policy('my_ip_events', if_exists => true);
SELECT remove_compression_policy('my_ip_grants', if_exists => true);
SELECT remove_compression_policy('my_ip_revenue', if_exists => true);

-- DROP VIEWS AND MATERIALIZED VIEWS
DROP FUNCTION IF EXISTS refresh_license_materialized_views;

DROP INDEX IF EXISTS idx_weekly_creator_revenue_creator;
DROP INDEX IF EXISTS idx_weekly_creator_revenue_bucket;
DROP MATERIALIZED VIEW IF EXISTS weekly_creator_revenue;

DROP INDEX IF EXISTS idx_daily_license_revenue_license_id;
DROP INDEX IF EXISTS idx_daily_license_revenue_bucket;
DROP MATERIALIZED VIEW IF EXISTS daily_license_revenue;

DROP VIEW IF EXISTS popular_licenses;
DROP VIEW IF EXISTS active_licenses;

-- DROP INDEXES
DROP INDEX IF EXISTS idx_unique_license_id_time;
DROP INDEX IF EXISTS idx_my_ip_license_id;
DROP INDEX IF EXISTS idx_posts_my_ip_id;
DROP INDEX IF EXISTS idx_posts_revenue_recipient;

DROP INDEX IF EXISTS idx_my_ip_revenue_license_id;
DROP INDEX IF EXISTS idx_my_ip_revenue_post_id;
DROP INDEX IF EXISTS idx_my_ip_revenue_from_address;
DROP INDEX IF EXISTS idx_my_ip_revenue_to_address;
DROP INDEX IF EXISTS idx_my_ip_revenue_time;

DROP INDEX IF EXISTS idx_my_ip_grants_license_id;
DROP INDEX IF EXISTS idx_my_ip_grants_grantor;
DROP INDEX IF EXISTS idx_my_ip_grants_grantee;
DROP INDEX IF EXISTS idx_my_ip_grants_grant_time;

DROP INDEX IF EXISTS idx_my_ip_events_license_id;
DROP INDEX IF EXISTS idx_my_ip_events_event_type;
DROP INDEX IF EXISTS idx_my_ip_events_created_by;
DROP INDEX IF EXISTS idx_my_ip_events_created_at;

DROP INDEX IF EXISTS idx_my_ip_creator;
DROP INDEX IF EXISTS idx_my_ip_license_type;
DROP INDEX IF EXISTS idx_my_ip_license_state;
DROP INDEX IF EXISTS idx_my_ip_creation_time;
DROP INDEX IF EXISTS idx_my_ip_revenue_recipient;

-- DROP HYPERTABLES AND TABLES
DROP TABLE IF EXISTS my_ip_revenue;
DROP TABLE IF EXISTS my_ip_grants;
DROP TABLE IF EXISTS my_ip_events;
DROP TABLE IF EXISTS my_ip_permissions;
DROP TABLE IF EXISTS my_ip;

-- Remove columns from posts table
ALTER TABLE posts DROP COLUMN IF EXISTS my_ip_id;
ALTER TABLE posts DROP COLUMN IF EXISTS revenue_recipient; 