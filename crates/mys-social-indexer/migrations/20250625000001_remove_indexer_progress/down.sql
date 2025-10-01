CREATE TABLE indexer_progress (
    id TEXT PRIMARY KEY,
    last_checkpoint_processed BIGINT NOT NULL,
    last_processed_at TIMESTAMP NOT NULL
);
