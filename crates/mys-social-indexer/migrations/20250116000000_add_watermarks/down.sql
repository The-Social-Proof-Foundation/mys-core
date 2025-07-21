-- Remove watermarks table and indexes
DROP INDEX IF EXISTS idx_watermarks_checkpoint;
DROP INDEX IF EXISTS idx_watermarks_timestamp;
DROP INDEX IF EXISTS idx_watermarks_worker_stream;
DROP TABLE IF EXISTS watermarks; 