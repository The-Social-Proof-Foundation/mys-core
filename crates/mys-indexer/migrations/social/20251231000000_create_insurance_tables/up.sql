-- Migration: Create insurance module tables
-- Version: 20251231000000
-- Purpose: Create all tables for insurance module including config, vaults, policies, events, and exposure tracking

-- ============================================================================
-- 1. ENABLE TIMESCALEDB EXTENSION
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END $$;

-- ============================================================================
-- 2. INSURANCE CONFIG TABLE (hypertable)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    paused BOOLEAN NOT NULL DEFAULT FALSE,
    min_coverage_bps BIGINT NOT NULL DEFAULT 0,
    max_coverage_bps BIGINT NOT NULL DEFAULT 0,
    max_duration_ms BIGINT NOT NULL DEFAULT 0,
    fee_bps BIGINT NOT NULL DEFAULT 0,
    treasury TEXT NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Create trigger to automatically update time from timestamp_ms
CREATE OR REPLACE FUNCTION update_insurance_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_insurance_config_time ON insurance_config;
CREATE TRIGGER set_insurance_config_time 
BEFORE INSERT ON insurance_config
FOR EACH ROW
EXECUTE FUNCTION update_insurance_config_time();

-- Convert to hypertable
-- Using 3 month chunks for config to minimize chunk creation (config changes are infrequent)
SELECT create_hypertable('insurance_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '3 months');

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'insurance_config_pkey'
    ) THEN
        ALTER TABLE insurance_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_insurance_config_time ON insurance_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_insurance_config_updated_by ON insurance_config(updated_by, time);
CREATE INDEX IF NOT EXISTS idx_insurance_config_transaction_id ON insurance_config(transaction_id);

-- ============================================================================
-- 3. INSURANCE VAULTS TABLE (regular table - current state)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_vaults (
    vault_id TEXT NOT NULL PRIMARY KEY,
    underwriter TEXT NOT NULL,
    capital_balance BIGINT NOT NULL DEFAULT 0,
    reserved BIGINT NOT NULL DEFAULT 0,
    base_rate_bps_per_day BIGINT NOT NULL DEFAULT 0,
    utilization_multiplier_bps BIGINT NOT NULL DEFAULT 0,
    max_exposure_per_market BIGINT NOT NULL DEFAULT 0,
    max_exposure_per_user BIGINT NOT NULL DEFAULT 0,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_insurance_vaults_underwriter ON insurance_vaults(underwriter);
CREATE INDEX IF NOT EXISTS idx_insurance_vaults_created_at ON insurance_vaults(created_at);
CREATE INDEX IF NOT EXISTS idx_insurance_vaults_transaction_id ON insurance_vaults(transaction_id);

-- ============================================================================
-- 4. INSURANCE POLICIES TABLE (regular table - current state)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_policies (
    policy_id TEXT NOT NULL PRIMARY KEY,
    market_id TEXT NOT NULL,
    insured TEXT NOT NULL,
    option_id SMALLINT NOT NULL,
    covered_amount BIGINT NOT NULL DEFAULT 0,
    coverage_bps BIGINT NOT NULL DEFAULT 0,
    premium_paid BIGINT NOT NULL DEFAULT 0,
    start_time_ms BIGINT NOT NULL,
    expiry_time_ms BIGINT NOT NULL,
    vault_id TEXT NOT NULL,
    status SMALLINT NOT NULL DEFAULT 1, -- 1=ACTIVE, 2=CANCELLED, 3=CLAIMED, 4=EXPIRED
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_insurance_policies_market_id ON insurance_policies(market_id);
CREATE INDEX IF NOT EXISTS idx_insurance_policies_insured ON insurance_policies(insured);
CREATE INDEX IF NOT EXISTS idx_insurance_policies_vault_id ON insurance_policies(vault_id);
CREATE INDEX IF NOT EXISTS idx_insurance_policies_status ON insurance_policies(status);
CREATE INDEX IF NOT EXISTS idx_insurance_policies_expiry_time_ms ON insurance_policies(expiry_time_ms);
CREATE INDEX IF NOT EXISTS idx_insurance_policies_transaction_id ON insurance_policies(transaction_id);

-- ============================================================================
-- 5. INSURANCE EVENTS TABLE (regular table - audit log)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_events (
    id SERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    event_data JSONB NOT NULL,
    event_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_insurance_events_type ON insurance_events(event_type);
CREATE INDEX IF NOT EXISTS idx_insurance_events_event_id ON insurance_events(event_id);
CREATE INDEX IF NOT EXISTS idx_insurance_events_created_at ON insurance_events(created_at);

-- ============================================================================
-- 6. INSURANCE VAULT TRANSACTIONS TABLE (hypertable)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_vault_transactions (
    id SERIAL NOT NULL,
    vault_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL, -- 'DEPOSIT' or 'WITHDRAWAL'
    amount BIGINT NOT NULL,
    balance_after BIGINT NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Create trigger to automatically update time from timestamp_ms
CREATE OR REPLACE FUNCTION update_insurance_vault_transactions_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_insurance_vault_transactions_time ON insurance_vault_transactions;
CREATE TRIGGER set_insurance_vault_transactions_time 
BEFORE INSERT ON insurance_vault_transactions
FOR EACH ROW
EXECUTE FUNCTION update_insurance_vault_transactions_time();

-- Convert to hypertable
-- Using 30 day chunks to reduce chunk creation frequency and overhead during testing
SELECT create_hypertable('insurance_vault_transactions', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '30 days');

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'insurance_vault_transactions_pkey'
    ) THEN
        ALTER TABLE insurance_vault_transactions ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_insurance_vault_transactions_vault_id ON insurance_vault_transactions(vault_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_vault_transactions_type ON insurance_vault_transactions(transaction_type, time);
CREATE INDEX IF NOT EXISTS idx_insurance_vault_transactions_transaction_id ON insurance_vault_transactions(transaction_id);

-- ============================================================================
-- 7. INSURANCE POLICY EVENTS TABLE (hypertable)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_policy_events (
    id SERIAL NOT NULL,
    policy_id TEXT NOT NULL,
    event_type TEXT NOT NULL, -- 'PURCHASED', 'CANCELLED', 'CLAIMED', 'EXPIRED'
    market_id TEXT NOT NULL,
    insured TEXT NOT NULL,
    option_id SMALLINT NOT NULL,
    covered_amount BIGINT NOT NULL DEFAULT 0,
    coverage_bps BIGINT NOT NULL DEFAULT 0,
    premium_paid BIGINT NOT NULL DEFAULT 0,
    reserve_locked BIGINT NOT NULL DEFAULT 0,
    refunded_amount BIGINT,
    fee_paid BIGINT,
    payout BIGINT,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Create trigger to automatically update time from timestamp_ms
CREATE OR REPLACE FUNCTION update_insurance_policy_events_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_insurance_policy_events_time ON insurance_policy_events;
CREATE TRIGGER set_insurance_policy_events_time 
BEFORE INSERT ON insurance_policy_events
FOR EACH ROW
EXECUTE FUNCTION update_insurance_policy_events_time();

-- Convert to hypertable
-- Using 30 day chunks to reduce chunk creation frequency and overhead during testing
SELECT create_hypertable('insurance_policy_events', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '30 days');

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'insurance_policy_events_pkey'
    ) THEN
        ALTER TABLE insurance_policy_events ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_insurance_policy_events_policy_id ON insurance_policy_events(policy_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_policy_events_market_id ON insurance_policy_events(market_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_policy_events_insured ON insurance_policy_events(insured, time);
CREATE INDEX IF NOT EXISTS idx_insurance_policy_events_event_type ON insurance_policy_events(event_type, time);
CREATE INDEX IF NOT EXISTS idx_insurance_policy_events_transaction_id ON insurance_policy_events(transaction_id);

-- ============================================================================
-- 8. INSURANCE MARKET EXPOSURES TABLE (hypertable - analytics)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_market_exposures (
    id SERIAL NOT NULL,
    vault_id TEXT NOT NULL,
    market_id TEXT NOT NULL,
    option_id SMALLINT NOT NULL,
    reserved_amount BIGINT NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Create trigger to automatically update time from timestamp_ms
CREATE OR REPLACE FUNCTION update_insurance_market_exposures_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_insurance_market_exposures_time ON insurance_market_exposures;
CREATE TRIGGER set_insurance_market_exposures_time 
BEFORE INSERT ON insurance_market_exposures
FOR EACH ROW
EXECUTE FUNCTION update_insurance_market_exposures_time();

-- Convert to hypertable
-- Using 30 day chunks to reduce chunk creation frequency and overhead during testing
SELECT create_hypertable('insurance_market_exposures', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '30 days');

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'insurance_market_exposures_pkey'
    ) THEN
        ALTER TABLE insurance_market_exposures ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_insurance_market_exposures_vault_id ON insurance_market_exposures(vault_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_market_exposures_market_id ON insurance_market_exposures(market_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_market_exposures_option_id ON insurance_market_exposures(option_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_market_exposures_transaction_id ON insurance_market_exposures(transaction_id);

-- ============================================================================
-- 9. INSURANCE USER EXPOSURES TABLE (hypertable - analytics)
-- ============================================================================
CREATE TABLE IF NOT EXISTS insurance_user_exposures (
    id SERIAL NOT NULL,
    vault_id TEXT NOT NULL,
    insured TEXT NOT NULL,
    reserved_amount BIGINT NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Create trigger to automatically update time from timestamp_ms
CREATE OR REPLACE FUNCTION update_insurance_user_exposures_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_insurance_user_exposures_time ON insurance_user_exposures;
CREATE TRIGGER set_insurance_user_exposures_time 
BEFORE INSERT ON insurance_user_exposures
FOR EACH ROW
EXECUTE FUNCTION update_insurance_user_exposures_time();

-- Convert to hypertable
-- Using 30 day chunks to reduce chunk creation frequency and overhead during testing
SELECT create_hypertable('insurance_user_exposures', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '30 days');

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'insurance_user_exposures_pkey'
    ) THEN
        ALTER TABLE insurance_user_exposures ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_insurance_user_exposures_vault_id ON insurance_user_exposures(vault_id, time);
CREATE INDEX IF NOT EXISTS idx_insurance_user_exposures_insured ON insurance_user_exposures(insured, time);
CREATE INDEX IF NOT EXISTS idx_insurance_user_exposures_transaction_id ON insurance_user_exposures(transaction_id);

