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

-- Create TimescaleDB hypertable for profile offers
SELECT create_hypertable('profile_offers', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Indexes for profile offers
CREATE INDEX IF NOT EXISTS idx_profile_offers_profile_id_time ON profile_offers (profile_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_offers_offeror_time ON profile_offers (offeror_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_offers_status_time ON profile_offers (status, time DESC) WHERE status = 'pending';
CREATE UNIQUE INDEX IF NOT EXISTS idx_profile_offers_profile_offeror_unique ON profile_offers (profile_id, offeror_address, time) WHERE status = 'pending';

-- Enable compression
ALTER TABLE profile_offers SET (timescaledb.compress = true);
SELECT add_compression_policy('profile_offers', INTERVAL '30 days');

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

-- Create TimescaleDB hypertable for profile sale fees
SELECT create_hypertable('profile_sale_fees', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Indexes for profile sale fees
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_profile_id_time ON profile_sale_fees (profile_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_offeror_time ON profile_sale_fees (offeror_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_previous_owner_time ON profile_sale_fees (previous_owner_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_fee_recipient_time ON profile_sale_fees (fee_recipient_address, time DESC);

-- Enable compression
ALTER TABLE profile_sale_fees SET (timescaledb.compress = true);
SELECT add_compression_policy('profile_sale_fees', INTERVAL '30 days');

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

-- Create TimescaleDB hypertable for profile badges
SELECT create_hypertable('profile_badges', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Indexes for profile badges
CREATE INDEX IF NOT EXISTS idx_profile_badges_profile_id_time ON profile_badges (profile_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_badges_badge_id_time ON profile_badges (badge_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_badges_platform_id_time ON profile_badges (platform_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_profile_badges_active ON profile_badges (profile_id, badge_id, time DESC) WHERE revoked = FALSE;
CREATE UNIQUE INDEX IF NOT EXISTS idx_profile_badges_unique ON profile_badges (profile_id, badge_id, time) WHERE revoked = FALSE;

-- Enable compression
ALTER TABLE profile_badges SET (timescaledb.compress = true);
SELECT add_compression_policy('profile_badges', INTERVAL '30 days');

