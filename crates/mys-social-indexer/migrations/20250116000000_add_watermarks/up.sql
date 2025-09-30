-- Add watermarks table for stream processing checkpoint tracking
-- This table tracks processing watermarks to ensure reliable stream processing

CREATE TABLE IF NOT EXISTS watermarks (
    id SERIAL PRIMARY KEY,
    worker_id TEXT NOT NULL,
    stream_name TEXT NOT NULL,
    watermark_timestamp BIGINT NOT NULL,
    checkpoint_sequence BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create unique index to prevent duplicate watermarks per worker/stream
CREATE UNIQUE INDEX IF NOT EXISTS idx_watermarks_worker_stream 
ON watermarks(worker_id, stream_name);

-- Create index for efficient watermark queries
CREATE INDEX IF NOT EXISTS idx_watermarks_timestamp 
ON watermarks(watermark_timestamp);

-- Create index for checkpoint sequence queries
CREATE INDEX IF NOT EXISTS idx_watermarks_checkpoint 
ON watermarks(checkpoint_sequence); 