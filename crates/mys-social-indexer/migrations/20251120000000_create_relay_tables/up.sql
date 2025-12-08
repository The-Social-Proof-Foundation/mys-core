-- Relay-specific tables for notifications, messaging, and user preferences

-- Notifications table
CREATE TABLE IF NOT EXISTS relay_notifications (
    id BIGSERIAL PRIMARY KEY,
    user_address TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    data JSONB,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_unread 
    ON relay_notifications(user_address, created_at DESC) 
    WHERE read_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_all 
    ON relay_notifications(user_address, created_at DESC);

-- Messages table
CREATE TABLE IF NOT EXISTS relay_messages (
    id BIGSERIAL PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    sender_address TEXT NOT NULL,
    recipient_address TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    media_urls JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    read_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_relay_messages_conversation 
    ON relay_messages(conversation_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_relay_messages_recipient 
    ON relay_messages(recipient_address, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_relay_messages_sender 
    ON relay_messages(sender_address, created_at DESC);

-- Conversations table
CREATE TABLE IF NOT EXISTS relay_conversations (
    id BIGSERIAL PRIMARY KEY,
    conversation_id TEXT NOT NULL UNIQUE,
    participant1_address TEXT NOT NULL,
    participant2_address TEXT NOT NULL,
    last_message_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_relay_conversations_participant1 
    ON relay_conversations(participant1_address, last_message_at DESC);

CREATE INDEX IF NOT EXISTS idx_relay_conversations_participant2 
    ON relay_conversations(participant2_address, last_message_at DESC);

-- User preferences table
CREATE TABLE IF NOT EXISTS relay_user_preferences (
    user_address TEXT NOT NULL PRIMARY KEY,
    push_enabled BOOLEAN NOT NULL DEFAULT true,
    email_enabled BOOLEAN NOT NULL DEFAULT true,
    sms_enabled BOOLEAN NOT NULL DEFAULT false,
    notification_types JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Device tokens table (for push notifications)
CREATE TABLE IF NOT EXISTS relay_device_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_address TEXT NOT NULL,
    device_token TEXT NOT NULL,
    platform TEXT NOT NULL, -- 'ios', 'android', 'web'
    device_id TEXT,
    app_version TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_address, device_token)
);

CREATE INDEX IF NOT EXISTS idx_relay_device_tokens_user 
    ON relay_device_tokens(user_address);

CREATE INDEX IF NOT EXISTS idx_relay_device_tokens_platform 
    ON relay_device_tokens(platform);

-- WebSocket connections tracking
CREATE TABLE IF NOT EXISTS relay_ws_connections (
    id BIGSERIAL PRIMARY KEY,
    user_address TEXT NOT NULL,
    connection_id TEXT NOT NULL UNIQUE,
    connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_heartbeat_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    disconnected_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_relay_ws_connections_user_active 
    ON relay_ws_connections(user_address, connected_at DESC) 
    WHERE disconnected_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_relay_ws_connections_heartbeat 
    ON relay_ws_connections(last_heartbeat_at) 
    WHERE disconnected_at IS NULL;

COMMENT ON TABLE relay_notifications IS 'User notifications stored in Postgres (also cached in Redis)';
COMMENT ON TABLE relay_messages IS 'Direct messages between users';
COMMENT ON TABLE relay_conversations IS 'Conversation metadata and last message tracking';
COMMENT ON TABLE relay_user_preferences IS 'User notification and communication preferences';
COMMENT ON TABLE relay_device_tokens IS 'Device tokens for push notifications (APNs, FCM)';
COMMENT ON TABLE relay_ws_connections IS 'Active WebSocket connections tracking';

