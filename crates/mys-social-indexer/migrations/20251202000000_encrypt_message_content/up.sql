-- Migrate message content to BYTEA for encryption support
-- This migration changes the content column from TEXT to BYTEA
-- Existing plain text messages will be converted to BYTEA format

-- Add new encrypted_content column as BYTEA
ALTER TABLE relay_messages
ADD COLUMN IF NOT EXISTS encrypted_content BYTEA;

-- Migrate existing TEXT content to BYTEA (if any exists)
-- Convert existing plain text to BYTEA (this is a one-time migration)
UPDATE relay_messages
SET encrypted_content = content::BYTEA
WHERE encrypted_content IS NULL AND content IS NOT NULL;

-- Drop the old TEXT content column
ALTER TABLE relay_messages
DROP COLUMN IF EXISTS content;

-- Rename encrypted_content to content
ALTER TABLE relay_messages
RENAME COLUMN encrypted_content TO content;

-- Add comment
COMMENT ON COLUMN relay_messages.content IS 'Encrypted message content (AES-256-GCM, base64 encoded)';

