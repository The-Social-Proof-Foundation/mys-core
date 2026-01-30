-- Add platform_id column to relay_notifications table
ALTER TABLE relay_notifications
    ADD COLUMN IF NOT EXISTS platform_id TEXT;

-- Create index for platform filtering
CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_platform 
    ON relay_notifications(user_address, platform_id, created_at DESC) 
    WHERE read_at IS NULL;

-- Update existing index to include platform_id
CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_platform_all 
    ON relay_notifications(user_address, platform_id, created_at DESC);

COMMENT ON COLUMN relay_notifications.platform_id IS 'Platform ID for platform-specific notification filtering';

