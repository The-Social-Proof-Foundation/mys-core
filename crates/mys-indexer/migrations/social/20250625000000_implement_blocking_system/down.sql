-- ROLLBACK PRODUCTION BLOCKING SYSTEM
-- Drop new tables and recreate old profiles_blocked table

-- Drop new blocking system tables
DROP TABLE IF EXISTS blocked_profiles;
DROP TABLE IF EXISTS blocked_events;

-- Recreate old profiles_blocked table for rollback
CREATE TABLE profiles_blocked (
    id SERIAL PRIMARY KEY,
    blocker_wallet_address TEXT NOT NULL,
    blocked_address TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    UNIQUE(blocker_wallet_address, blocked_address)
);

-- Create indexes for old table
CREATE INDEX idx_profiles_blocked_blocker ON profiles_blocked(blocker_wallet_address);
CREATE INDEX idx_profiles_blocked_blocked ON profiles_blocked(blocked_address);
