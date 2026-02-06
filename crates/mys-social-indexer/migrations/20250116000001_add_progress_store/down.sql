-- Remove progress store table and indexes
DROP INDEX IF EXISTS idx_progress_store_errors;
DROP INDEX IF EXISTS idx_progress_store_state;
DROP INDEX IF EXISTS idx_progress_store_checkpoint;
DROP INDEX IF EXISTS idx_progress_store_worker_module;
DROP TABLE IF EXISTS progress_store; 