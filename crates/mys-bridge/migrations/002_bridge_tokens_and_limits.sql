-- Bridge tokens and limits configuration
-- This migration adds tables for token management and bridge limits

-- Bridge supported tokens table
CREATE TABLE bridge_tokens (
    id SERIAL PRIMARY KEY,
    token_id INTEGER NOT NULL UNIQUE,
    token_type VARCHAR(255) NOT NULL,
    chain_id INTEGER NOT NULL,
    contract_address VARCHAR(66),
    native_token BOOLEAN DEFAULT FALSE,
    decimal_multiplier BIGINT NOT NULL DEFAULT 1,
    notional_value BIGINT NOT NULL DEFAULT 0,
    mys_decimals INTEGER DEFAULT 9,
    evm_decimals INTEGER DEFAULT 18,
    symbol VARCHAR(10),
    name VARCHAR(100),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Bridge chain limits table
CREATE TABLE bridge_chain_limits (
    id SERIAL PRIMARY KEY,
    source_chain_id INTEGER NOT NULL,
    target_chain_id INTEGER NOT NULL,
    daily_limit_usd BIGINT NOT NULL DEFAULT 0,
    hourly_limit_usd BIGINT NOT NULL DEFAULT 0,
    current_daily_volume_usd BIGINT DEFAULT 0,
    current_hourly_volume_usd BIGINT DEFAULT 0,
    last_daily_reset TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_hourly_reset TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT unique_chain_route UNIQUE(source_chain_id, target_chain_id)
);

-- Bridge transfer history for volume tracking
CREATE TABLE bridge_transfer_history (
    id SERIAL PRIMARY KEY,
    bridge_action_digest VARCHAR(66) NOT NULL,
    source_chain_id INTEGER NOT NULL,
    target_chain_id INTEGER NOT NULL,
    token_id INTEGER NOT NULL,
    amount_native BIGINT NOT NULL,
    amount_usd BIGINT NOT NULL,
    sender_address VARCHAR(66) NOT NULL,
    recipient_address VARCHAR(66) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Bridge committee configuration
CREATE TABLE bridge_committee_config (
    id SERIAL PRIMARY KEY,
    key VARCHAR(100) NOT NULL UNIQUE,
    value JSONB NOT NULL,
    description TEXT,
    updated_by VARCHAR(66),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Bridge price feeds for token valuations
CREATE TABLE bridge_price_feeds (
    id SERIAL PRIMARY KEY,
    token_id INTEGER NOT NULL,
    price_usd BIGINT NOT NULL,
    source VARCHAR(50) NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Bridge emergency actions log
CREATE TABLE bridge_emergency_actions (
    id SERIAL PRIMARY KEY,
    action_type VARCHAR(50) NOT NULL,
    chain_id INTEGER NOT NULL,
    initiated_by VARCHAR(66) NOT NULL,
    reason TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    executed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for bridge tokens
CREATE INDEX idx_bridge_tokens_id ON bridge_tokens(token_id);
CREATE INDEX idx_bridge_tokens_chain ON bridge_tokens(chain_id);
CREATE INDEX idx_bridge_tokens_address ON bridge_tokens(contract_address);
CREATE INDEX idx_bridge_tokens_active ON bridge_tokens(is_active);
CREATE INDEX idx_bridge_tokens_symbol ON bridge_tokens(symbol);

-- Create indexes for chain limits
CREATE INDEX idx_bridge_chain_limits_source ON bridge_chain_limits(source_chain_id);
CREATE INDEX idx_bridge_chain_limits_target ON bridge_chain_limits(target_chain_id);
CREATE INDEX idx_bridge_chain_limits_active ON bridge_chain_limits(is_active);
CREATE INDEX idx_bridge_chain_limits_daily_reset ON bridge_chain_limits(last_daily_reset);
CREATE INDEX idx_bridge_chain_limits_hourly_reset ON bridge_chain_limits(last_hourly_reset);

-- Create indexes for transfer history
CREATE INDEX idx_bridge_transfer_history_digest ON bridge_transfer_history(bridge_action_digest);
CREATE INDEX idx_bridge_transfer_history_chains ON bridge_transfer_history(source_chain_id, target_chain_id);
CREATE INDEX idx_bridge_transfer_history_token ON bridge_transfer_history(token_id);
CREATE INDEX idx_bridge_transfer_history_status ON bridge_transfer_history(status);
CREATE INDEX idx_bridge_transfer_history_sender ON bridge_transfer_history(sender_address);
CREATE INDEX idx_bridge_transfer_history_recipient ON bridge_transfer_history(recipient_address);
CREATE INDEX idx_bridge_transfer_history_created_at ON bridge_transfer_history(created_at);

-- Create indexes for committee config
CREATE INDEX idx_bridge_committee_config_key ON bridge_committee_config(key);
CREATE INDEX idx_bridge_committee_config_updated_at ON bridge_committee_config(updated_at);

-- Create indexes for price feeds
CREATE INDEX idx_bridge_price_feeds_token ON bridge_price_feeds(token_id);
CREATE INDEX idx_bridge_price_feeds_updated ON bridge_price_feeds(last_updated);
CREATE INDEX idx_bridge_price_feeds_active ON bridge_price_feeds(is_active);

-- Create indexes for emergency actions
CREATE INDEX idx_bridge_emergency_actions_type ON bridge_emergency_actions(action_type);
CREATE INDEX idx_bridge_emergency_actions_chain ON bridge_emergency_actions(chain_id);
CREATE INDEX idx_bridge_emergency_actions_status ON bridge_emergency_actions(status);
CREATE INDEX idx_bridge_emergency_actions_created_at ON bridge_emergency_actions(created_at);

-- Add triggers for updated_at timestamps
CREATE TRIGGER update_bridge_tokens_updated_at BEFORE UPDATE ON bridge_tokens
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bridge_chain_limits_updated_at BEFORE UPDATE ON bridge_chain_limits
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bridge_committee_config_updated_at BEFORE UPDATE ON bridge_committee_config
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert initial supported tokens (Base Sepolia USDC)
INSERT INTO bridge_tokens (token_id, token_type, chain_id, contract_address, native_token, decimal_multiplier, notional_value, mys_decimals, evm_decimals, symbol, name) VALUES
(3, 'USDC', 84532, '0x0000000000000000000000000000000000000003', false, 1000000, 100000000, 6, 6, 'USDC', 'USD Coin'),
(0, 'ETH', 84532, '0x0000000000000000000000000000000000000000', true, 1000000000, 100000000, 9, 18, 'ETH', 'Ethereum');

-- Insert initial chain limits (Base Sepolia <-> Mys)
INSERT INTO bridge_chain_limits (source_chain_id, target_chain_id, daily_limit_usd, hourly_limit_usd) VALUES
(84532, 1, 100000000000, 10000000000),  -- Base Sepolia to Mys: $10M daily, $1M hourly
(1, 84532, 100000000000, 10000000000);  -- Mys to Base Sepolia: $10M daily, $1M hourly

-- Insert initial committee configuration
INSERT INTO bridge_committee_config (key, value, description) VALUES
('min_committee_stake_required', '5001', 'Minimum stake required for committee operations'),
('validity_threshold', '6667', 'Minimum stake percentage required for action approval'),
('emergency_pause_enabled', 'true', 'Whether emergency pause functionality is enabled'),
('transfer_cooldown_seconds', '300', 'Cooldown period between transfers for same address');

-- Insert initial price feeds
INSERT INTO bridge_price_feeds (token_id, price_usd, source) VALUES
(0, 100000000, 'manual'),  -- ETH: $1.00 (8 decimal places)
(3, 100000000, 'manual');  -- USDC: $1.00 (8 decimal places) 