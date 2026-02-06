-- Revert: Add profile_id column back (if needed)
-- Note: This will set profile_id to NULL for existing rows

ALTER TABLE delegates 
    ADD COLUMN IF NOT EXISTS profile_id TEXT;

ALTER TABLE nominated_delegates
    ADD COLUMN IF NOT EXISTS profile_id TEXT;

-- Recreate indexes if they existed
CREATE INDEX IF NOT EXISTS idx_delegates_profile_id ON delegates(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_nominees_profile_id ON nominated_delegates(profile_id, time);

