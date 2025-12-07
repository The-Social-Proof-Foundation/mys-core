-- FORCE BLOCKING SCHEMA UPDATE
-- This migration ensures the blocking system schema is correctly applied

-- First, drop ANY existing blocking tables to ensure clean state
DROP TABLE IF EXISTS blocked_profiles CASCADE;
DROP TABLE IF EXISTS blocked_events CASCADE;
DROP TABLE IF EXISTS profiles_blocked CASCADE;

-- Create blocked_events table for comprehensive audit trail
CREATE TABLE IF NOT EXISTS blocked_events (
    id SERIAL PRIMARY KEY,
    event_id TEXT UNIQUE,                    -- Blockchain event ID for deduplication
    event_type TEXT NOT NULL,                -- 'block' | 'unblock' | 'block_list_created'
    blocker_address TEXT NOT NULL,           -- Profile doing the blocking
    blocked_address TEXT,                    -- Profile being blocked (NULL for block_list_created)
    block_list_address TEXT,                 -- Block list object ID
    raw_event_data JSONB,                      -- Full blockchain event data
    processed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL             -- Blockchain timestamp
);

-- Create blocked_profiles table for current blocking state with rich profile data
CREATE TABLE IF NOT EXISTS blocked_profiles (
    id SERIAL PRIMARY KEY,
    blocker_address TEXT NOT NULL,          -- Profile doing the blocking  
    blocked_address TEXT NOT NULL,          -- Profile being blocked
    block_list_address TEXT,                -- Reference to block list object
    
    -- Rich profile data for performance (denormalized from profiles table)
    blocked_profile_id TEXT,                -- Blockchain profile ID of blocked user
    blocked_username TEXT NOT NULL,         -- Username of blocked user
    blocked_display_name TEXT,              -- Display name of blocked user  
    blocked_profile_photo TEXT,             -- Profile photo URL of blocked user
    
    -- Blocking metadata
    first_blocked_at TIMESTAMP NOT NULL,       -- When first blocked
    last_blocked_at TIMESTAMP NOT NULL,        -- Most recent block event
    total_block_count INTEGER DEFAULT 1 NOT NULL,   -- How many times blocked (if re-blocked)
    
    UNIQUE(blocker_address, blocked_address)
);

-- Create indexes for blocked_events
CREATE INDEX IF NOT EXISTS idx_blocked_events_blocker ON blocked_events(blocker_address);
CREATE INDEX IF NOT EXISTS idx_blocked_events_blocked ON blocked_events(blocked_address);
CREATE INDEX IF NOT EXISTS idx_blocked_events_type ON blocked_events(event_type);
CREATE INDEX IF NOT EXISTS idx_blocked_events_created_at ON blocked_events(created_at);
CREATE INDEX IF NOT EXISTS idx_blocked_events_event_id ON blocked_events(event_id);

-- Create indexes for blocked_profiles
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_blocker ON blocked_profiles(blocker_address, last_blocked_at DESC);
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_blocked ON blocked_profiles(blocked_address);
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_pagination ON blocked_profiles(blocker_address, id);
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_block_list ON blocked_profiles(block_list_address);
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_username ON blocked_profiles(blocked_username);
CREATE INDEX IF NOT EXISTS idx_blocked_profiles_profile_id ON blocked_profiles(blocked_profile_id);

-- Add table comments
COMMENT ON TABLE blocked_events IS 'Complete audit trail of all blocking/unblocking events from blockchain';
COMMENT ON TABLE blocked_profiles IS 'Current blocking relationships - represents live blocking state';

-- Add column comments
COMMENT ON COLUMN blocked_events.event_id IS 'Unique blockchain event identifier for deduplication';
COMMENT ON COLUMN blocked_events.event_type IS 'Type of blocking event: block, unblock, or block_list_created';
COMMENT ON COLUMN blocked_events.blocked_address IS 'NULL for block_list_created events';
COMMENT ON COLUMN blocked_profiles.blocked_profile_id IS 'Blockchain profile ID of the blocked user';
COMMENT ON COLUMN blocked_profiles.blocked_username IS 'Username of the blocked user for fast API responses';
COMMENT ON COLUMN blocked_profiles.blocked_display_name IS 'Display name of the blocked user for fast API responses';
COMMENT ON COLUMN blocked_profiles.blocked_profile_photo IS 'Profile photo URL of the blocked user for fast API responses';
COMMENT ON COLUMN blocked_profiles.first_blocked_at IS 'Timestamp when this profile was first blocked by this blocker';
COMMENT ON COLUMN blocked_profiles.last_blocked_at IS 'Most recent blocking event timestamp';
COMMENT ON COLUMN blocked_profiles.total_block_count IS 'Number of times this profile has been blocked by this blocker';
COMMENT ON COLUMN blocked_profiles.block_list_address IS 'Reference to the blockchain block list object';





