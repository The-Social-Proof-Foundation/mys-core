-- Create platforms table
CREATE TABLE IF NOT EXISTS platforms (
    id SERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    tagline TEXT NOT NULL,
    description TEXT,
    logo TEXT,
    developer_address TEXT NOT NULL,
    terms_of_service TEXT,
    privacy_policy TEXT,
    platforms JSONB,                  -- Array of platform names (Twitter, Instagram, etc.)
    links JSONB,                      -- Array of platform URLs
    status SMALLINT NOT NULL,         -- Platform status (0=dev, 1=alpha, 2=beta, 3=live, etc.)
    release_date TEXT,
    shutdown_date TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create index on platform_id
CREATE INDEX IF NOT EXISTS idx_platforms_platform_id ON platforms(platform_id);

-- Create index on name for quick lookups
CREATE INDEX IF NOT EXISTS idx_platforms_name ON platforms(name);

-- Create moderators table
CREATE TABLE IF NOT EXISTS platform_moderators (
    id SERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL,
    moderator_address TEXT NOT NULL,
    added_by TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(platform_id, moderator_address)
);

-- Create index on platform_id for moderators
CREATE INDEX IF NOT EXISTS idx_platform_moderators_platform_id ON platform_moderators(platform_id);

-- Create blocked profiles table
CREATE TABLE IF NOT EXISTS platform_blocked_profiles (
    id SERIAL PRIMARY KEY,
    platform_id TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    blocked_by TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    is_blocked BOOLEAN NOT NULL DEFAULT TRUE,
    unblocked_at TIMESTAMP,
    unblocked_by TEXT,
    UNIQUE(platform_id, profile_id)
);

-- Create index on platform_id for blocked profiles
CREATE INDEX IF NOT EXISTS idx_platform_blocked_profiles_platform_id ON platform_blocked_profiles(platform_id);

-- Create index on profile_id for blocked profiles to quickly check if a profile is blocked
CREATE INDEX IF NOT EXISTS idx_platform_blocked_profiles_profile_id ON platform_blocked_profiles(profile_id);

-- Create a table for platform events
CREATE TABLE IF NOT EXISTS platform_events (
    id SERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    platform_id TEXT NOT NULL,
    event_data JSONB NOT NULL,
    event_id TEXT,  
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create index on platform_id for events
CREATE INDEX IF NOT EXISTS idx_platform_events_platform_id ON platform_events(platform_id);