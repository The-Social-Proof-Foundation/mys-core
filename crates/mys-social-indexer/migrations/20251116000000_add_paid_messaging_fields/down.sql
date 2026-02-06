-- Remove paid messaging fields from profiles table
DROP INDEX IF EXISTS idx_profiles_paid_messaging_enabled;
ALTER TABLE profiles
DROP COLUMN IF EXISTS paid_messaging_enabled,
DROP COLUMN IF EXISTS paid_messaging_min_cost;

