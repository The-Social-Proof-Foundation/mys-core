-- Update profile social media username fields
-- Remove mastodon_username and add twitch_username and linkedin_username
-- This matches the updated smart contract profile structure

-- Remove mastodon_username column
ALTER TABLE profiles DROP COLUMN IF EXISTS mastodon_username;

-- Add twitch_username column
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS twitch_username TEXT;

-- Add linkedin_username column
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS linkedin_username TEXT;

