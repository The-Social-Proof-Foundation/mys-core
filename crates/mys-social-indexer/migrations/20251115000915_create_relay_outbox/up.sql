-- Relay Outbox Table for Change Data Capture
-- This table is written to by the indexer and polled by the relay server

CREATE TABLE IF NOT EXISTS relay_outbox (
    id BIGSERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    event_data JSONB NOT NULL,
    event_id TEXT,
    transaction_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    published_at TIMESTAMPTZ,
    retry_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT
);

-- Index for efficient polling (unprocessed events)
CREATE INDEX IF NOT EXISTS idx_relay_outbox_unprocessed 
    ON relay_outbox(created_at) 
    WHERE processed_at IS NULL;

-- Index for event_id deduplication
CREATE INDEX IF NOT EXISTS idx_relay_outbox_event_id 
    ON relay_outbox(event_id) 
    WHERE event_id IS NOT NULL;

-- Index for transaction_id
CREATE INDEX IF NOT EXISTS idx_relay_outbox_transaction_id 
    ON relay_outbox(transaction_id) 
    WHERE transaction_id IS NOT NULL;

COMMENT ON TABLE relay_outbox IS 'Outbox table for CDC - indexer writes events here, relay polls and publishes to Redpanda';

