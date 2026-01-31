-- ============================================================================
-- ENABLE TIMESCALEDB EXTENSION (if not already enabled)
-- ============================================================================
-- Ensure TimescaleDB extension is available before creating hypertables
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END
$$;

-- Add Instagram username support to profiles table
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS instagram_username TEXT;

-- Create index for instagram_username lookups
CREATE INDEX IF NOT EXISTS idx_profiles_instagram_username ON profiles(instagram_username) WHERE instagram_username IS NOT NULL;

-- ============================================================================
-- PROFILE OFFERS TABLE (TimescaleDB Hypertable)
-- ============================================================================
CREATE TABLE IF NOT EXISTS profile_offers (
    id SERIAL NOT NULL,
    profile_id TEXT NOT NULL,
    offeror_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'accepted', 'rejected', 'revoked'
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    resolved_at BIGINT,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_profile_offers PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for profile offers (only if not already a hypertable)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_schema = 'public' AND hypertable_name = 'profile_offers'
    ) THEN
        PERFORM create_hypertable('profile_offers'::regclass, 'time'::name);
    END IF;
END $$;

-- Indexes for profile offers
CREATE INDEX IF NOT EXISTS idx_profile_offers_profile_id_time ON profile_offers (profile_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_offers_offeror_time ON profile_offers (offeror_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_offers_status_time ON profile_offers (status, time DESC) WHERE status = 'pending';
CREATE UNIQUE INDEX IF NOT EXISTS idx_profile_offers_profile_offeror_unique ON profile_offers (profile_id, offeror_address, time) WHERE status = 'pending';

-- Enable compression (only if not already set)
DO $$
BEGIN
    -- Check if compression is enabled on the table
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
        AND c.relname = 'profile_offers'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE profile_offers SET (timescaledb.compress = true);
    END IF;

    -- Add compression policy if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
        AND hypertable_schema = 'public'
        AND hypertable_name = 'profile_offers'
    ) THEN
        PERFORM add_compression_policy('profile_offers', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- PROFILE SALE FEES TABLE (TimescaleDB Hypertable)
-- ============================================================================
CREATE TABLE IF NOT EXISTS profile_sale_fees (
    id SERIAL NOT NULL,
    profile_id TEXT NOT NULL,
    offeror_address TEXT NOT NULL,
    previous_owner_address TEXT NOT NULL,
    sale_amount BIGINT NOT NULL,
    fee_amount BIGINT NOT NULL,
    fee_recipient_address TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_profile_sale_fees PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for profile sale fees (only if not already a hypertable)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_schema = 'public' AND hypertable_name = 'profile_sale_fees'
    ) THEN
        PERFORM create_hypertable('profile_sale_fees'::regclass, 'time'::name);
    END IF;
END $$;

-- Indexes for profile sale fees
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_profile_id_time ON profile_sale_fees (profile_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_offeror_time ON profile_sale_fees (offeror_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_previous_owner_time ON profile_sale_fees (previous_owner_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_fee_recipient_time ON profile_sale_fees (fee_recipient_address, time DESC);

-- Enable compression (only if not already set)
DO $$
BEGIN
    -- Check if compression is enabled on the table
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
        AND c.relname = 'profile_sale_fees'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE profile_sale_fees SET (timescaledb.compress = true);
    END IF;

    -- Add compression policy if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
        AND hypertable_schema = 'public'
        AND hypertable_name = 'profile_sale_fees'
    ) THEN
        PERFORM add_compression_policy('profile_sale_fees', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- PROFILE BADGES TABLE (TimescaleDB Hypertable)
-- ============================================================================
CREATE TABLE IF NOT EXISTS profile_badges (
    id SERIAL NOT NULL,
    profile_id TEXT NOT NULL,
    badge_id TEXT NOT NULL,
    badge_name TEXT NOT NULL,
    badge_description TEXT,
    badge_image_url TEXT,
    platform_id TEXT NOT NULL,
    assigned_by TEXT NOT NULL,
    assigned_at BIGINT NOT NULL,
    revoked BOOLEAN DEFAULT FALSE,
    revoked_at BIGINT,
    revoked_by TEXT,
    badge_type SMALLINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_profile_badges PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for profile badges (only if not already a hypertable)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables
        WHERE hypertable_schema = 'public' AND hypertable_name = 'profile_badges'
    ) THEN
        PERFORM create_hypertable('profile_badges'::regclass, 'time'::name);
    END IF;
END $$;

-- Indexes for profile badges
CREATE INDEX IF NOT EXISTS idx_profile_badges_profile_id_time ON profile_badges (profile_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_badges_badge_id_time ON profile_badges (badge_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_badges_platform_id_time ON profile_badges (platform_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_badges_active ON profile_badges (profile_id, badge_id, time DESC) WHERE revoked = FALSE;
CREATE UNIQUE INDEX IF NOT EXISTS idx_profile_badges_unique ON profile_badges (profile_id, badge_id, time) WHERE revoked = FALSE;

-- Enable compression (only if not already set)
DO $$
BEGIN
    -- Check if compression is enabled on the table
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
        AND c.relname = 'profile_badges'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE profile_badges SET (timescaledb.compress = true);
    END IF;

    -- Add compression policy if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
        AND hypertable_schema = 'public'
        AND hypertable_name = 'profile_badges'
    ) THEN
        PERFORM add_compression_policy('profile_badges', INTERVAL '30 days');
    END IF;
END $$;
