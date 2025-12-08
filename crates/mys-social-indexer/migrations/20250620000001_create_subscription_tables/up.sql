-- Create subscription-related tables following the SUBSCRIPTION_SOCIAL_INDEXER_PLAN
-- Phase 1: Database Schema Design

-- 1. Profile Subscription Services Table (Regular Table)
-- Services don't change frequently, so regular table is sufficient
CREATE TABLE IF NOT EXISTS profile_subscription_services (
    service_id TEXT NOT NULL PRIMARY KEY,
    profile_owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    monthly_fee BIGINT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    subscriber_count BIGINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL,
    updated_at BIGINT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Optimized indexes for service lookups
CREATE INDEX IF NOT EXISTS idx_profile_services_owner ON profile_subscription_services (profile_owner);
CREATE INDEX IF NOT EXISTS idx_profile_services_profile ON profile_subscription_services (profile_id);
CREATE INDEX IF NOT EXISTS idx_profile_services_active ON profile_subscription_services (active) WHERE active = true;

-- 2. Profile Subscriptions Table (TimescaleDB Hypertable)
-- Individual subscription records with time partitioning for analytics
CREATE TABLE IF NOT EXISTS profile_subscriptions (
    subscription_id TEXT NOT NULL,
    service_id TEXT NOT NULL,
    subscriber TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    auto_renew BOOLEAN NOT NULL DEFAULT false,
    renewal_balance BIGINT NOT NULL DEFAULT 0,
    renewal_count BIGINT NOT NULL DEFAULT 0,
    cancelled_at BIGINT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    processing_success BOOLEAN NOT NULL DEFAULT true,
    processing_error TEXT
);

-- Convert to TimescaleDB hypertable for subscription analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'profile_subscriptions'
    ) THEN
        PERFORM create_hypertable('profile_subscriptions'::regclass, 'time'::name, chunk_time_interval => INTERVAL '14 days');
    END IF;
END $$;

-- Optimized indexes for subscription tracking
-- TimescaleDB requires unique indexes to include partitioning column (time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_profile_subscriptions_id ON profile_subscriptions (subscription_id, time);
CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_time_service ON profile_subscriptions (time DESC, service_id);
CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_subscriber_time ON profile_subscriptions (subscriber, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_expires ON profile_subscriptions (expires_at) WHERE cancelled_at IS NULL;

-- 3. Subscription Events Table (TimescaleDB Hypertable)
-- All subscription-related events for audit trail and compliance
CREATE TABLE IF NOT EXISTS subscription_events (
    event_type TEXT NOT NULL,
    subscription_id TEXT,
    service_id TEXT,
    subscriber TEXT,
    event_data JSONB NOT NULL,
    event_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    processing_success BOOLEAN NOT NULL DEFAULT true,
    processing_error TEXT
);

-- Convert to TimescaleDB hypertable for event analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'subscription_events'
    ) THEN
        PERFORM create_hypertable('subscription_events'::regclass, 'time'::name, chunk_time_interval => INTERVAL '7 days');
    END IF;
END $$;

-- Optimized indexes for event tracking
CREATE INDEX IF NOT EXISTS idx_subscription_events_time_type ON subscription_events (time DESC, event_type);
CREATE INDEX IF NOT EXISTS idx_subscription_events_subscription_time ON subscription_events (subscription_id, time DESC) WHERE subscription_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_subscription_events_service_time ON subscription_events (service_id, time DESC) WHERE service_id IS NOT NULL;

-- 4. Subscription Revenue Table (TimescaleDB Hypertable)
-- Revenue analytics for subscriptions with time partitioning
CREATE TABLE IF NOT EXISTS subscription_revenue (
    service_id TEXT NOT NULL,
    subscription_id TEXT,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    revenue_type TEXT NOT NULL, -- 'subscription', 'renewal', 'refund'
    payment_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    processing_success BOOLEAN NOT NULL DEFAULT true,
    processing_error TEXT
);

-- Convert to TimescaleDB hypertable for revenue analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'subscription_revenue'
    ) THEN
        PERFORM create_hypertable('subscription_revenue'::regclass, 'time'::name, chunk_time_interval => INTERVAL '7 days');
    END IF;
END $$;

-- Optimized indexes for revenue queries
CREATE INDEX IF NOT EXISTS idx_subscription_revenue_time_service ON subscription_revenue (time DESC, service_id);
CREATE INDEX IF NOT EXISTS idx_subscription_revenue_to_time ON subscription_revenue (to_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_subscription_revenue_type_time ON subscription_revenue (revenue_type, time DESC);

-- 5. Subscription Access Logs Table (TimescaleDB Hypertable)
-- Track content access for analytics with shorter retention
CREATE TABLE IF NOT EXISTS subscription_access_logs (
    subscription_id TEXT NOT NULL,
    subscriber TEXT NOT NULL,
    content_type TEXT NOT NULL, -- 'profile', 'post'
    content_id TEXT NOT NULL,
    access_time BIGINT NOT NULL,
    seal_id TEXT, -- For encrypted content access
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    processing_success BOOLEAN NOT NULL DEFAULT true,
    processing_error TEXT
);

-- Convert to TimescaleDB hypertable with daily chunks for access logs
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'subscription_access_logs'
    ) THEN
        PERFORM create_hypertable('subscription_access_logs'::regclass, 'time'::name, chunk_time_interval => INTERVAL '1 day');
    END IF;
END $$;

-- Optimized indexes for access pattern analysis
CREATE INDEX IF NOT EXISTS idx_subscription_access_time_sub ON subscription_access_logs (time DESC, subscription_id);
CREATE INDEX IF NOT EXISTS idx_subscription_access_content_time ON subscription_access_logs (content_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_subscription_access_subscriber_time ON subscription_access_logs (subscriber, time DESC);

-- Add data retention policy for access logs (keep 90 days)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'subscription_access_logs'
    ) THEN
        PERFORM add_retention_policy('subscription_access_logs', INTERVAL '90 days');
    END IF;
END $$;

-- 6. TimescaleDB Continuous Aggregates for Analytics
-- Daily subscription revenue aggregate
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'subscription_daily_revenue'
    ) THEN
        CREATE MATERIALIZED VIEW subscription_daily_revenue
        WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', time) AS day,
    service_id,
    to_address AS profile_owner,
    revenue_type,
    SUM(amount) AS daily_revenue,
    COUNT(*) AS transaction_count
FROM subscription_revenue
GROUP BY time_bucket('1 day', time), service_id, to_address, revenue_type
WITH NO DATA;

        -- Enable automatic refresh (window: 3 days - 1 hour = ~71 hours, chunk: 7 days = 168 hours)
        -- Fix: Make refresh window larger than chunk interval
        PERFORM add_continuous_aggregate_policy('subscription_daily_revenue',
            start_offset => INTERVAL '8 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Daily subscription metrics aggregate
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'subscription_daily_metrics'
    ) THEN
        CREATE MATERIALIZED VIEW subscription_daily_metrics
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', time) AS day,
    service_id,
    COUNT(*) FILTER (WHERE cancelled_at IS NULL) AS new_subscriptions,
    COUNT(*) FILTER (WHERE cancelled_at IS NOT NULL) AS cancelled_subscriptions,
    COUNT(*) FILTER (WHERE renewal_count > 0) AS renewed_subscriptions
FROM profile_subscriptions
GROUP BY time_bucket('1 day', time), service_id
WITH NO DATA;

        -- Enable automatic refresh (window must be > 14 days for profile_subscriptions chunk interval)
        PERFORM add_continuous_aggregate_policy('subscription_daily_metrics',
            start_offset => INTERVAL '15 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Schema Updates: Add subscription-related fields to posts table
ALTER TABLE posts ADD COLUMN IF NOT EXISTS requires_subscription BOOLEAN DEFAULT false;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS subscription_service_id TEXT;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS subscription_price BIGINT;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS encrypted_content_hash TEXT;

-- Schema Updates: Add subscription service reference to profiles table  
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS subscription_service_id TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS subscription_enabled BOOLEAN DEFAULT false;

-- Advanced TimescaleDB Features Implementation
-- Enable compression on hypertables first, then add policies
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'profile_subscriptions'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE profile_subscriptions SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'subscription_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE subscription_events SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'subscription_revenue'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE subscription_revenue SET (timescaledb.compress = true);
    END IF;
    
    -- Add compression policies if they don't exist
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'profile_subscriptions'
    ) THEN
        PERFORM add_compression_policy('profile_subscriptions', INTERVAL '60 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'subscription_events'
    ) THEN
        PERFORM add_compression_policy('subscription_events', INTERVAL '30 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'subscription_revenue'
    ) THEN
        PERFORM add_compression_policy('subscription_revenue', INTERVAL '30 days');
    END IF;
END $$;

-- Real-time subscription health monitoring
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'subscription_health_metrics'
    ) THEN
        CREATE MATERIALIZED VIEW subscription_health_metrics
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 hour', time) AS hour,
            service_id,
            COUNT(*) FILTER (WHERE cancelled_at IS NULL) AS active_subscriptions,
            COUNT(*) FILTER (WHERE cancelled_at IS NOT NULL) AS cancelled_subscriptions,
            AVG(renewal_count) AS avg_renewal_count,
            SUM(renewal_balance) AS total_renewal_balance
        FROM profile_subscriptions
        GROUP BY time_bucket('1 hour', time), service_id
        WITH NO DATA;

        -- Enable automatic refresh for health metrics (window must be > 14 days for profile_subscriptions)
        PERFORM add_continuous_aggregate_policy('subscription_health_metrics',
            start_offset => INTERVAL '15 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Churn analysis aggregate
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'subscription_churn_analysis'
    ) THEN
        CREATE MATERIALIZED VIEW subscription_churn_analysis
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 day', time) AS day,
            service_id,
            COUNT(*) FILTER (WHERE cancelled_at BETWEEN extract(epoch from time_bucket('1 day', time)) 
                             AND extract(epoch from time_bucket('1 day', time)) + 86400) AS daily_churn,
            COUNT(*) FILTER (WHERE created_at BETWEEN extract(epoch from time_bucket('1 day', time)) 
                             AND extract(epoch from time_bucket('1 day', time)) + 86400) AS daily_new_subs
        FROM profile_subscriptions
        GROUP BY time_bucket('1 day', time), service_id
        WITH NO DATA;

        -- Enable automatic refresh for churn analysis (window must be > 14 days for profile_subscriptions)
        PERFORM add_continuous_aggregate_policy('subscription_churn_analysis',
            start_offset => INTERVAL '15 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '2 hours');
    END IF;
END $$;

-- Add foreign key constraints
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'fk_profile_subscriptions_service_id'
        AND conrelid = 'profile_subscriptions'::regclass
    ) THEN
        ALTER TABLE profile_subscriptions 
        ADD CONSTRAINT fk_profile_subscriptions_service_id 
        FOREIGN KEY (service_id) REFERENCES profile_subscription_services(service_id);
    END IF;
END $$;

-- Additional indexes for performance
CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_service_id ON profile_subscriptions(service_id);
CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_subscriber ON profile_subscriptions(subscriber);
CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_expires_at ON profile_subscriptions(expires_at);
CREATE INDEX IF NOT EXISTS idx_subscription_revenue_service_id ON subscription_revenue(service_id);
CREATE INDEX IF NOT EXISTS idx_subscription_revenue_time ON subscription_revenue(time); 