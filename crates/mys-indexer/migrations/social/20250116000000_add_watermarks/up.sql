-- Add watermarks table for stream processing checkpoint tracking
-- This table tracks processing watermarks to ensure reliable stream processing
--
-- NOTE: The main indexer already creates a watermarks table with a 'pipeline' column.
-- This migration only creates the table if it doesn't exist, and does NOT drop
-- the existing main indexer watermarks table.

-- Only create the table if it doesn't exist AND doesn't have the main indexer schema
DO $$
BEGIN
    -- Check if watermarks table exists with the main indexer schema (has 'pipeline' column)
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'watermarks'
        AND column_name = 'pipeline'
        AND table_schema = 'public'
    ) THEN
        -- Main indexer's watermarks table already exists, skip creating social version
        RAISE NOTICE 'watermarks table already exists with main indexer schema (pipeline column), skipping social migration';
    ELSE
        -- Table doesn't exist or has different schema, create it
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
    END IF;
END $$;
