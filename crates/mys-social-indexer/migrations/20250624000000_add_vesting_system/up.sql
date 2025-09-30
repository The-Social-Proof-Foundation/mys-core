-- ADD TOKEN VESTING SYSTEM
-- Production-ready implementation for MYS token vesting with configurable release curves

-- ============================================================================
-- 1. CREATE VESTING WALLET TABLE (Regular Table)
-- ============================================================================

-- Vesting Wallets table - Core vesting wallet configurations and current balances
CREATE TABLE IF NOT EXISTS vesting_wallets (
    wallet_id TEXT PRIMARY KEY,
    owner_address TEXT NOT NULL,
    total_amount BIGINT NOT NULL,
    start_time BIGINT NOT NULL,
    duration BIGINT NOT NULL,
    curve_factor BIGINT NOT NULL,
    claimed_amount BIGINT NOT NULL DEFAULT 0,
    remaining_balance BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- ============================================================================
-- 2. CREATE VESTING EVENTS TABLE (TimescaleDB Hypertable)
-- ============================================================================

-- Vesting Events table - Complete event history for all vesting actions
CREATE TABLE IF NOT EXISTS vesting_events (
    id SERIAL NOT NULL,
    wallet_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    owner_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    remaining_balance BIGINT,
    start_time BIGINT,
    duration BIGINT,
    curve_factor BIGINT,
    event_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_vesting_events PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for vesting events
SELECT create_hypertable('vesting_events', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on vesting events table
ALTER TABLE vesting_events SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'wallet_id,owner_address,event_type',
    timescaledb.compress_orderby = 'time DESC'
);

-- ============================================================================
-- 3. CREATE INDEXES
-- ============================================================================

-- Vesting wallets indexes
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_owner_address ON vesting_wallets(owner_address);
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_start_time ON vesting_wallets(start_time);
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_curve_factor ON vesting_wallets(curve_factor);
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_created_at ON vesting_wallets(created_at);

-- Vesting events indexes
CREATE INDEX IF NOT EXISTS idx_vesting_events_wallet_id ON vesting_events(wallet_id);
CREATE INDEX IF NOT EXISTS idx_vesting_events_owner_address ON vesting_events(owner_address);
CREATE INDEX IF NOT EXISTS idx_vesting_events_event_type ON vesting_events(event_type);
CREATE INDEX IF NOT EXISTS idx_vesting_events_event_time ON vesting_events(event_time);

-- ============================================================================
-- 4. SET UP AUTOMATIC COMPRESSION POLICIES
-- ============================================================================

-- Add compression policy to compress chunks after 7 days
SELECT add_compression_policy('vesting_events', INTERVAL '7 days');

-- ============================================================================
-- 5. CREATE VESTING UTILITY FUNCTIONS
-- ============================================================================

-- Function to calculate claimable amount based on vesting curve
CREATE OR REPLACE FUNCTION calculate_vesting_claimable(
    total_amount_param BIGINT,
    start_time_param BIGINT,
    duration_param BIGINT,
    curve_factor_param BIGINT,
    claimed_amount_param BIGINT,
    current_time_param BIGINT
)
RETURNS BIGINT AS $$
DECLARE
    elapsed_time BIGINT;
    progress_ratio DOUBLE PRECISION;
    curve_factor_normalized DOUBLE PRECISION;
    curved_progress DOUBLE PRECISION;
    total_vested BIGINT;
    claimable BIGINT;
    precision_factor CONSTANT DOUBLE PRECISION := 1000.0;
BEGIN
    -- If vesting hasn't started yet
    IF current_time_param < start_time_param THEN
        RETURN 0;
    END IF;
    
    elapsed_time := current_time_param - start_time_param;
    
    -- If vesting period is complete
    IF elapsed_time >= duration_param THEN
        RETURN total_amount_param - claimed_amount_param;
    END IF;
    
    -- Calculate progress ratio (0.0 to 1.0)
    progress_ratio := CAST(elapsed_time AS DOUBLE PRECISION) / CAST(duration_param AS DOUBLE PRECISION);
    
    -- Normalize curve factor
    curve_factor_normalized := CAST(curve_factor_param AS DOUBLE PRECISION) / precision_factor;
    
    -- Apply curve based on curve factor
    IF curve_factor_param = 0 OR curve_factor_param = 1000 THEN
        -- Linear vesting
        curved_progress := progress_ratio;
    ELSIF curve_factor_param > 1000 THEN
        -- Exponential curve (more tokens toward end)
        -- Use quadratic approximation: progress^2
        curved_progress := progress_ratio * progress_ratio;
        -- Blend with linear based on how far curve_factor is from 1000
        DECLARE
            blend_factor DOUBLE PRECISION := LEAST((curve_factor_normalized - 1.0) * 2.0, 1.0);
        BEGIN
            curved_progress := progress_ratio * (1.0 - blend_factor) + curved_progress * blend_factor;
        END;
    ELSE
        -- Logarithmic curve (more tokens toward start)  
        -- Use square root approximation: sqrt(progress)
        curved_progress := SQRT(progress_ratio);
        -- Blend with linear based on how far curve_factor is from 1000
        DECLARE
            blend_factor DOUBLE PRECISION := LEAST((1.0 - curve_factor_normalized) * 2.0, 1.0);
        BEGIN
            curved_progress := progress_ratio * (1.0 - blend_factor) + curved_progress * blend_factor;
        END;
    END IF;
    
    -- Calculate total vested amount
    total_vested := CAST(CAST(total_amount_param AS DOUBLE PRECISION) * curved_progress AS BIGINT);
    
    -- Calculate claimable (total vested minus already claimed)
    claimable := total_vested - claimed_amount_param;
    
    -- Ensure non-negative result
    RETURN GREATEST(claimable, 0);
END;
$$ LANGUAGE plpgsql;

-- Function to get vesting status
CREATE OR REPLACE FUNCTION get_vesting_status(
    start_time_param BIGINT,
    duration_param BIGINT,
    current_time_param BIGINT
)
RETURNS TEXT AS $$
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 'not_started';
    ELSIF current_time_param >= start_time_param + duration_param THEN
        RETURN 'completed';
    ELSE
        RETURN 'in_progress';
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate vesting progress percentage
CREATE OR REPLACE FUNCTION get_vesting_progress(
    start_time_param BIGINT,
    duration_param BIGINT,
    current_time_param BIGINT
)
RETURNS DOUBLE PRECISION AS $$
DECLARE
    elapsed_time BIGINT;
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 0.0;
    END IF;
    
    elapsed_time := current_time_param - start_time_param;
    
    IF elapsed_time >= duration_param THEN
        RETURN 100.0;
    END IF;
    
    RETURN (CAST(elapsed_time AS DOUBLE PRECISION) / CAST(duration_param AS DOUBLE PRECISION)) * 100.0;
END;
$$ LANGUAGE plpgsql;