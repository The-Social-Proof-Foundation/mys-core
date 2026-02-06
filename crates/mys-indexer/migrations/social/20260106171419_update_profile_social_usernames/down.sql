-- Rollback: Restore mastodon_username and remove twitch_username and linkedin_username

-- Add back mastodon_username column
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS mastodon_username TEXT;

-- Remove twitch_username column
ALTER TABLE profiles DROP COLUMN IF EXISTS twitch_username;

-- Remove linkedin_username column
ALTER TABLE profiles DROP COLUMN IF EXISTS linkedin_username;

