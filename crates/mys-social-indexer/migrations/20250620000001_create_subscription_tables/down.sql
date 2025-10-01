-- Rollback subscription-related tables and changes
-- This reverses all changes made in up.sql

-- Remove foreign key constraints first
ALTER TABLE profile_subscriptions DROP CONSTRAINT IF EXISTS fk_profile_subscriptions_service_id;

-- Drop continuous aggregate policies
SELECT remove_continuous_aggregate_policy('subscription_daily_revenue', if_exists => true);
SELECT remove_continuous_aggregate_policy('subscription_daily_metrics', if_exists => true);
SELECT remove_continuous_aggregate_policy('subscription_health_metrics', if_exists => true);
SELECT remove_continuous_aggregate_policy('subscription_churn_analysis', if_exists => true);

-- Drop continuous aggregate views
DROP MATERIALIZED VIEW IF EXISTS subscription_daily_revenue;
DROP MATERIALIZED VIEW IF EXISTS subscription_daily_metrics;
DROP MATERIALIZED VIEW IF EXISTS subscription_health_metrics;
DROP MATERIALIZED VIEW IF EXISTS subscription_churn_analysis;

-- Remove retention policies
SELECT remove_retention_policy('subscription_access_logs', if_exists => true);

-- Remove compression policies
SELECT remove_compression_policy('profile_subscriptions', if_exists => true);
SELECT remove_compression_policy('subscription_events', if_exists => true);
SELECT remove_compression_policy('subscription_revenue', if_exists => true);

-- Drop hypertables (this will automatically drop the associated indexes)
DROP TABLE IF EXISTS subscription_access_logs;
DROP TABLE IF EXISTS subscription_revenue;
DROP TABLE IF EXISTS subscription_events;
DROP TABLE IF EXISTS profile_subscriptions;

-- Drop regular tables
DROP TABLE IF EXISTS profile_subscription_services;

-- Remove subscription-related fields from posts table
ALTER TABLE posts DROP COLUMN IF EXISTS requires_subscription;
ALTER TABLE posts DROP COLUMN IF EXISTS subscription_service_id;
ALTER TABLE posts DROP COLUMN IF EXISTS subscription_price;
ALTER TABLE posts DROP COLUMN IF EXISTS encrypted_content_hash;

-- Remove subscription service reference from profiles table
ALTER TABLE profiles DROP COLUMN IF EXISTS subscription_service_id;
ALTER TABLE profiles DROP COLUMN IF EXISTS subscription_enabled; 