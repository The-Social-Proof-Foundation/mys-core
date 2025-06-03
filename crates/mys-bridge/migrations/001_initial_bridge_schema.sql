-- Initial MySo Bridge database schema
-- This migration creates the core tables needed for bridge operations

-- Bridge events table to track cross-chain events
CREATE TABLE bridge_events (
    id SERIAL PRIMARY KEY,
    chain_id INTEGER NOT NULL,
    tx_hash VARCHAR(66) NOT NULL,
    event_index INTEGER NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    payload JSONB NOT NULL,
    processed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT unique_event UNIQUE(chain_id, tx_hash, event_index)
);

-- Bridge signatures table to store authority signatures for actions
CREATE TABLE bridge_signatures (
    id SERIAL PRIMARY KEY,
    action_hash VARCHAR(66) NOT NULL,
    authority_pubkey VARCHAR(66) NOT NULL,
    signature VARCHAR(132) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT unique_signature UNIQUE(action_hash, authority_pubkey)
);

-- Bridge state table for storing key-value configuration and state
CREATE TABLE bridge_state (
    id SERIAL PRIMARY KEY,
    key VARCHAR(100) NOT NULL UNIQUE,
    value JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Bridge authority table for tracking committee members
CREATE TABLE bridge_authorities (
    id SERIAL PRIMARY KEY,
    pubkey VARCHAR(66) NOT NULL UNIQUE,
    mys_address VARCHAR(66) NOT NULL,
    voting_power BIGINT NOT NULL DEFAULT 0,
    base_url TEXT,
    is_blocklisted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Bridge transactions table for tracking pending and completed transactions
CREATE TABLE bridge_transactions (
    id SERIAL PRIMARY KEY,
    bridge_action_digest VARCHAR(66) NOT NULL UNIQUE,
    action_type INTEGER NOT NULL,
    source_chain_id INTEGER NOT NULL,
    target_chain_id INTEGER NOT NULL,
    sequence_number BIGINT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    payload JSONB NOT NULL,
    mys_tx_digest VARCHAR(66),
    eth_tx_hash VARCHAR(66),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_bridge_events_chain_tx ON bridge_events(chain_id, tx_hash);
CREATE INDEX idx_bridge_events_processed ON bridge_events(processed);
CREATE INDEX idx_bridge_events_created_at ON bridge_events(created_at);
CREATE INDEX idx_bridge_events_chain_type ON bridge_events(chain_id, event_type);

CREATE INDEX idx_bridge_signatures_action ON bridge_signatures(action_hash);
CREATE INDEX idx_bridge_signatures_authority ON bridge_signatures(authority_pubkey);
CREATE INDEX idx_bridge_signatures_created_at ON bridge_signatures(created_at);

CREATE INDEX idx_bridge_state_key ON bridge_state(key);
CREATE INDEX idx_bridge_state_updated_at ON bridge_state(updated_at);

CREATE INDEX idx_bridge_authorities_pubkey ON bridge_authorities(pubkey);
CREATE INDEX idx_bridge_authorities_address ON bridge_authorities(mys_address);
CREATE INDEX idx_bridge_authorities_blocklist ON bridge_authorities(is_blocklisted);

CREATE INDEX idx_bridge_transactions_digest ON bridge_transactions(bridge_action_digest);
CREATE INDEX idx_bridge_transactions_status ON bridge_transactions(status);
CREATE INDEX idx_bridge_transactions_chains ON bridge_transactions(source_chain_id, target_chain_id);
CREATE INDEX idx_bridge_transactions_sequence ON bridge_transactions(source_chain_id, sequence_number);
CREATE INDEX idx_bridge_transactions_created_at ON bridge_transactions(created_at);

-- Create trigger to automatically update updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_bridge_events_updated_at BEFORE UPDATE ON bridge_events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bridge_state_updated_at BEFORE UPDATE ON bridge_state
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bridge_authorities_updated_at BEFORE UPDATE ON bridge_authorities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bridge_transactions_updated_at BEFORE UPDATE ON bridge_transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column(); 