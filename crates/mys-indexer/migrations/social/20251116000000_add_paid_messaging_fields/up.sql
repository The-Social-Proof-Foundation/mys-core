-- Add paid messaging fields to profiles table
ALTER TABLE profiles
ADD COLUMN IF NOT EXISTS paid_messaging_enabled BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS paid_messaging_min_cost BIGINT;

-- Create index on paid_messaging_enabled for quick lookups
CREATE INDEX IF NOT EXISTS idx_profiles_paid_messaging_enabled ON profiles(paid_messaging_enabled);

