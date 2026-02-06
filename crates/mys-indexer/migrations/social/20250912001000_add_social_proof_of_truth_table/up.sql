-- Unified SPoT events table (timeseries)

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS social_proof_of_truth (
    id SERIAL NOT NULL,
    event_type TEXT NOT NULL,
    post_id TEXT NOT NULL,
    user_address TEXT,
    is_yes BOOLEAN,
    escrow_amount BIGINT,
    amm_amount BIGINT,
    amount BIGINT,
    outcome SMALLINT,
    total_escrow BIGINT,
    fee_taken BIGINT,
    confidence_bps BIGINT,
    timestamp_epoch BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_id TEXT,
    transaction_id TEXT,
    raw_event JSONB
);

SELECT create_hypertable('social_proof_of_truth', 'time', if_not_exists => TRUE, create_default_indexes => FALSE, chunk_time_interval => INTERVAL '30 days');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_proof_of_truth_pkey'
    ) THEN
        ALTER TABLE social_proof_of_truth ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_spot_unified_type ON social_proof_of_truth(event_type, time);
CREATE INDEX IF NOT EXISTS idx_spot_unified_post ON social_proof_of_truth(post_id, time);
CREATE INDEX IF NOT EXISTS idx_spot_unified_user ON social_proof_of_truth(user_address, time);
CREATE INDEX IF NOT EXISTS idx_spot_unified_tx ON social_proof_of_truth(transaction_id);

