-- Enhanced migration for emergency kill switch functionality only
-- Note: Diesel migrations run in transactions automatically, so BEGIN/COMMIT are not needed

-- Create specific table for token exchange configuration (kill switch)
CREATE TABLE IF NOT EXISTS token_exchange_config (
    id SERIAL PRIMARY KEY,
    trading_halted BOOLEAN NOT NULL DEFAULT false,
    admin_address TEXT NOT NULL,
    reason TEXT NOT NULL DEFAULT 'System initialized', -- Limit reason length
    timestamp_ms BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    -- Enhanced constraints for production
    CONSTRAINT valid_timestamp CHECK (timestamp_ms >= 0),
    CONSTRAINT valid_admin_address CHECK (length(admin_address) > 0),
    CONSTRAINT valid_reason CHECK (length(reason) > 0 AND length(reason) <= 512),
    CONSTRAINT valid_transaction_id CHECK (length(transaction_id) > 0 AND length(transaction_id) <= 255),
    -- Ensure only one configuration row exists
    CONSTRAINT single_config_row CHECK (id = 1)
);

-- Create index for token exchange config
CREATE INDEX IF NOT EXISTS idx_token_exchange_config_updated_at ON token_exchange_config(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_token_exchange_config_trading_halted ON token_exchange_config(trading_halted);

-- Initialize kill switch as not halted (with conflict resolution)
INSERT INTO token_exchange_config (id, trading_halted, admin_address, reason, timestamp_ms, transaction_id)
VALUES (1, false, '0x0', 'System initialized', 0, 'INIT')
ON CONFLICT (id) DO NOTHING;

-- Create table for token exchange events (for kill switch tracking)
CREATE TABLE IF NOT EXISTS token_exchange_events (
    id SERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    event_data JSONB NOT NULL DEFAULT '{}'::jsonb,
    event_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Enhanced constraints
    CONSTRAINT valid_event_type CHECK (length(event_type) > 0 AND length(event_type) <= 100),
    CONSTRAINT valid_event_id CHECK (length(event_id) > 0 AND length(event_id) <= 255),
    CONSTRAINT unique_event_id UNIQUE(event_id)
);

-- Create indexes for token exchange events
CREATE INDEX IF NOT EXISTS idx_token_exchange_events_type ON token_exchange_events(event_type);
CREATE INDEX IF NOT EXISTS idx_token_exchange_events_created_at ON token_exchange_events(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_token_exchange_events_event_id ON token_exchange_events(event_id);