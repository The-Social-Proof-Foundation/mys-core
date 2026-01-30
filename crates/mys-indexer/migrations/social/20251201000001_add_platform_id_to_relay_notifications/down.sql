DROP INDEX IF EXISTS idx_relay_notifications_user_platform;
DROP INDEX IF EXISTS idx_relay_notifications_user_platform_all;
ALTER TABLE relay_notifications DROP COLUMN IF EXISTS platform_id;

