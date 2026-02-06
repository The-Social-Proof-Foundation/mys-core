-- Create platform_token_airdrops table
CREATE TABLE IF NOT EXISTS platform_token_airdrops (
    id SERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL,
    recipient TEXT NOT NULL,
    amount BIGINT NOT NULL,
    reason_code SMALLINT NOT NULL,
    executed_by TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    event_id TEXT
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_platform_token_airdrops_platform_id ON platform_token_airdrops (platform_id);
CREATE INDEX IF NOT EXISTS idx_platform_token_airdrops_recipient ON platform_token_airdrops (recipient);
CREATE INDEX IF NOT EXISTS idx_platform_token_airdrops_timestamp ON platform_token_airdrops (timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_platform_token_airdrops_executed_by ON platform_token_airdrops (executed_by);
CREATE INDEX IF NOT EXISTS idx_platform_token_airdrops_platform_timestamp ON platform_token_airdrops (platform_id, timestamp DESC);

