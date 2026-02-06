-- Migration to add native MYS treasury balance tracking
-- This tracks the locked native MYS tokens in the bridge treasury for bidirectional bridging

-- Table for current native treasury balances
CREATE TABLE bridge_treasury_balances (
    id SERIAL PRIMARY KEY,
    token_type VARCHAR(255) NOT NULL UNIQUE,
    token_id INTEGER NOT NULL UNIQUE,
    total_locked BIGINT NOT NULL DEFAULT 0,
    total_unlocked BIGINT NOT NULL DEFAULT 0,
    net_balance BIGINT NOT NULL DEFAULT 0,
    last_updated_block BIGINT NOT NULL,
    last_updated_timestamp BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Table for lock/unlock event history
CREATE TABLE bridge_treasury_events (
    id SERIAL PRIMARY KEY,
    token_type VARCHAR(255) NOT NULL,
    token_id INTEGER NOT NULL,
    event_type VARCHAR(20) NOT NULL CHECK (event_type IN ('lock', 'unlock')),
    amount BIGINT NOT NULL,
    tx_digest VARCHAR(66) NOT NULL,
    block_height BIGINT NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    sender_address BYTEA,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for efficient querying
CREATE INDEX idx_bridge_treasury_balances_token_type ON bridge_treasury_balances(token_type);
CREATE INDEX idx_bridge_treasury_balances_token_id ON bridge_treasury_balances(token_id);
CREATE INDEX idx_bridge_treasury_events_token_type ON bridge_treasury_events(token_type);
CREATE INDEX idx_bridge_treasury_events_event_type ON bridge_treasury_events(event_type);
CREATE INDEX idx_bridge_treasury_events_block_height ON bridge_treasury_events(block_height);
CREATE INDEX idx_bridge_treasury_events_timestamp_ms ON bridge_treasury_events(timestamp_ms);
CREATE INDEX idx_bridge_treasury_events_tx_digest ON bridge_treasury_events(tx_digest);

-- Create function to automatically update updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to update updated_at timestamp
CREATE TRIGGER update_bridge_treasury_balances_updated_at 
    BEFORE UPDATE ON bridge_treasury_balances
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Initialize native MYS entry (will be populated when bootstrap event is detected)
INSERT INTO bridge_treasury_balances (
    token_type, 
    token_id, 
    total_locked, 
    total_unlocked, 
    net_balance, 
    last_updated_block, 
    last_updated_timestamp
) VALUES (
    '0x0000000000000000000000000000000000000000000000000000000000000002::mys::MYS',
    0,
    0,
    0,
    0,
    0,
    0
) ON CONFLICT (token_type) DO NOTHING;

