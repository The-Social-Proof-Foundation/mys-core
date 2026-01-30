-- Migration Down: Remove SPT Revenue Tracking and Unified Revenue System
-- Version: 20250622000001

-- Drop views first (depend on materialized views and tables)
DROP VIEW IF EXISTS revenue_dashboard_24h CASCADE;
DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
DROP VIEW IF EXISTS spt_creator_revenue_summary CASCADE;

-- Drop materialized views (with policies)
DROP MATERIALIZED VIEW IF EXISTS spt_hourly_analytics CASCADE;
DROP MATERIALIZED VIEW IF EXISTS revenue_realtime_metrics CASCADE;
DROP MATERIALIZED VIEW IF EXISTS revenue_monthly_platforms CASCADE;
DROP MATERIALIZED VIEW IF EXISTS revenue_daily_creators CASCADE;
DROP MATERIALIZED VIEW IF EXISTS revenue_hourly_summary CASCADE;

-- Drop hypertables
DROP TABLE IF EXISTS unified_revenue CASCADE;
DROP TABLE IF EXISTS spt_revenue CASCADE; 