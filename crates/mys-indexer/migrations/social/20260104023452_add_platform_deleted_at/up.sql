-- Add deleted_at column to platforms table for soft delete support
ALTER TABLE platforms ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP;

-- Create index on deleted_at for efficient queries
CREATE INDEX IF NOT EXISTS idx_platforms_deleted_at ON platforms(deleted_at);

