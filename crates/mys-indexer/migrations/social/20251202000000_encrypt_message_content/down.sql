-- Rollback migration: convert BYTEA back to TEXT
-- Note: This will lose encryption, converting encrypted content back to text representation

-- Add temporary TEXT column
ALTER TABLE relay_messages
ADD COLUMN IF NOT EXISTS content_text TEXT;

-- Convert BYTEA to TEXT (this will show base64-encoded content)
UPDATE relay_messages
SET content_text = encode(content, 'base64')
WHERE content IS NOT NULL;

-- Drop BYTEA column
ALTER TABLE relay_messages
DROP COLUMN IF EXISTS content;

-- Rename content_text to content
ALTER TABLE relay_messages
RENAME COLUMN content_text TO content;

-- Change column type back to TEXT
ALTER TABLE relay_messages
ALTER COLUMN content TYPE TEXT;

