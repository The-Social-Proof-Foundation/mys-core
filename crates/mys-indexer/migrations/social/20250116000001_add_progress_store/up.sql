-- Add enhanced progress store for detailed indexing state management
-- This extends the basic indexer_progress table with more granular tracking

-- Drop existing table if it exists with wrong schema (for clean migration)
DROP TABLE IF EXISTS progress_store CASCADE;

-- Create progress store table
CREATE TABLE progress_store (
    id SERIAL PRIMARY KEY,
    worker_id TEXT NOT NULL,
    module_name TEXT NOT NULL,
    last_processed_checkpoint BIGINT NOT NULL DEFAULT 0,
    last_processed_event_id TEXT,
    last_processed_timestamp BIGINT NOT NULL,
    processing_state TEXT NOT NULL DEFAULT 'running',
    error_count INTEGER NOT NULL DEFAULT 0,
    last_error_message TEXT,
    last_error_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create unique index for worker/module combination
CREATE UNIQUE INDEX idx_progress_store_worker_module
ON progress_store(worker_id, module_name);

-- Create index for checkpoint queries
CREATE INDEX idx_progress_store_checkpoint
ON progress_store(last_processed_checkpoint);

-- Create index for processing state queries
CREATE INDEX idx_progress_store_state
ON progress_store(processing_state);

-- Create index for error tracking
CREATE INDEX idx_progress_store_errors
ON progress_store(error_count, last_error_at) WHERE error_count > 0;
