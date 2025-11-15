-- Platform Delivery Configuration Table
-- Stores platform-specific delivery settings for APNs, FCM, and Email (Resend)

CREATE TABLE IF NOT EXISTS platform_delivery_config (
    id BIGSERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL UNIQUE,
    
    -- APNs Configuration
    apns_bundle_id TEXT,
    apns_key_id TEXT,
    apns_team_id TEXT,
    apns_key_path TEXT, -- Path to .p8 key file (stored securely)
    apns_key_content TEXT, -- Base64 encoded key content (alternative to path)
    
    -- FCM Configuration
    fcm_server_key TEXT,
    
    -- Resend Email Configuration
    resend_api_key TEXT,
    resend_from_email TEXT,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Ensure platform exists
    CONSTRAINT fk_platform FOREIGN KEY (platform_id) REFERENCES platforms(platform_id) ON DELETE CASCADE
);

-- Index for fast lookups
CREATE INDEX IF NOT EXISTS idx_platform_delivery_config_platform_id 
    ON platform_delivery_config(platform_id);

COMMENT ON TABLE platform_delivery_config IS 'Platform-specific delivery configuration for push notifications and email';
COMMENT ON COLUMN platform_delivery_config.apns_key_content IS 'Base64 encoded APNs key content (alternative to apns_key_path)';

