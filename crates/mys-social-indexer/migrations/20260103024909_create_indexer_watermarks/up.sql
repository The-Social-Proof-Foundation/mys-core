-- Create indexer_watermarks table for checkpoint-based pipeline tracking
-- This table tracks ReaderWatermark (last checkpoint read) and CommitterWatermark (last checkpoint committed)

CREATE TABLE IF NOT EXISTS indexer_watermarks (
    id SERIAL PRIMARY KEY,
    checkpoint_seq BIGINT NOT NULL,
    tx_digest VARCHAR NOT NULL,
    reader_watermark BIGINT,
    committer_watermark BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tx_digest)
);

-- Create indexes for efficient watermark queries
CREATE INDEX IF NOT EXISTS idx_indexer_watermarks_checkpoint_seq ON indexer_watermarks(checkpoint_seq DESC);
CREATE INDEX IF NOT EXISTS idx_indexer_watermarks_reader_watermark ON indexer_watermarks(reader_watermark DESC) WHERE reader_watermark IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_indexer_watermarks_committer_watermark ON indexer_watermarks(committer_watermark DESC) WHERE committer_watermark IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_indexer_watermarks_tx_digest ON indexer_watermarks(tx_digest);

