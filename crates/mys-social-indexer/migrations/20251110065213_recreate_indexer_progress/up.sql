CREATE TABLE IF NOT EXISTS indexer_progress (
    id TEXT PRIMARY KEY,
    last_checkpoint_processed BIGINT NOT NULL DEFAULT 0,
    last_processed_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_indexer_progress_id ON indexer_progress(id);
