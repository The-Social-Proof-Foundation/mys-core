-- Social Proof of Truth (SPoT) schema

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END
$$;

-- Core record per post
CREATE TABLE IF NOT EXISTS spot_records (
    id SERIAL PRIMARY KEY,
    post_id VARCHAR NOT NULL UNIQUE,
    status SMALLINT NOT NULL,
    outcome SMALLINT,
    amm_split_bps_used INTEGER NOT NULL,
    total_yes_escrow BIGINT NOT NULL DEFAULT 0,
    total_no_escrow BIGINT NOT NULL DEFAULT 0,
    created_epoch BIGINT NOT NULL,
    last_resolution_epoch BIGINT,
    version BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_spot_records_post_id ON spot_records(post_id);
CREATE INDEX IF NOT EXISTS idx_spot_records_status ON spot_records(status);

-- Bets hypertable
CREATE TABLE IF NOT EXISTS spot_bets (
    id SERIAL NOT NULL,
    post_id VARCHAR NOT NULL,
    user_address VARCHAR NOT NULL,
    is_yes BOOLEAN NOT NULL,
    escrow_amount BIGINT NOT NULL,
    amm_amount BIGINT NOT NULL,
    timestamp_epoch BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);

SELECT create_hypertable('spot_bets', 'time', if_not_exists => TRUE, create_default_indexes => FALSE, chunk_time_interval => INTERVAL '7 days');
ALTER TABLE spot_bets ADD PRIMARY KEY (id, time);
CREATE INDEX IF NOT EXISTS idx_spot_bets_post_id ON spot_bets(post_id, time);
CREATE INDEX IF NOT EXISTS idx_spot_bets_user ON spot_bets(user_address, time);
CREATE INDEX IF NOT EXISTS idx_spot_bets_created_at ON spot_bets(timestamp_epoch);

-- Payouts hypertable
CREATE TABLE IF NOT EXISTS spot_payouts (
    id SERIAL NOT NULL,
    post_id VARCHAR NOT NULL,
    user_address VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    timestamp_epoch BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);
SELECT create_hypertable('spot_payouts', 'time', if_not_exists => TRUE, create_default_indexes => FALSE, chunk_time_interval => INTERVAL '7 days');
ALTER TABLE spot_payouts ADD PRIMARY KEY (id, time);
CREATE INDEX IF NOT EXISTS idx_spot_payouts_post_id ON spot_payouts(post_id, time);
CREATE INDEX IF NOT EXISTS idx_spot_payouts_user ON spot_payouts(user_address, time);

-- Refunds hypertable
CREATE TABLE IF NOT EXISTS spot_refunds (
    id SERIAL NOT NULL,
    post_id VARCHAR NOT NULL,
    user_address VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    timestamp_epoch BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);
SELECT create_hypertable('spot_refunds', 'time', if_not_exists => TRUE, create_default_indexes => FALSE, chunk_time_interval => INTERVAL '7 days');
ALTER TABLE spot_refunds ADD PRIMARY KEY (id, time);
CREATE INDEX IF NOT EXISTS idx_spot_refunds_post_id ON spot_refunds(post_id, time);
CREATE INDEX IF NOT EXISTS idx_spot_refunds_user ON spot_refunds(user_address, time);

-- Resolution summaries hypertable
CREATE TABLE IF NOT EXISTS spot_resolutions (
    id SERIAL NOT NULL,
    post_id VARCHAR NOT NULL,
    outcome SMALLINT NOT NULL,
    total_escrow BIGINT NOT NULL,
    fee_taken BIGINT NOT NULL,
    resolved_epoch BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);
SELECT create_hypertable('spot_resolutions', 'time', if_not_exists => TRUE, create_default_indexes => FALSE, chunk_time_interval => INTERVAL '30 days');
ALTER TABLE spot_resolutions ADD PRIMARY KEY (id, time);
CREATE INDEX IF NOT EXISTS idx_spot_resolutions_post_id ON spot_resolutions(post_id, time);

-- Event audit log
CREATE TABLE IF NOT EXISTS spot_events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    post_id VARCHAR NOT NULL,
    event_data JSONB NOT NULL,
    event_id VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_spot_events_type ON spot_events(event_type);
CREATE INDEX IF NOT EXISTS idx_spot_events_post ON spot_events(post_id);

