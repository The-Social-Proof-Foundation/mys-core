-- Remove index on deleted_at
DROP INDEX IF EXISTS idx_platforms_deleted_at;

-- Remove deleted_at column from platforms table
ALTER TABLE platforms DROP COLUMN IF EXISTS deleted_at;

