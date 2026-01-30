-- Create wallet_social_graph table for tracking social counts for wallet addresses without profiles
CREATE TABLE IF NOT EXISTS wallet_social_graph (
    wallet_address VARCHAR PRIMARY KEY,
    followers_count INTEGER NOT NULL DEFAULT 0,
    following_count INTEGER NOT NULL DEFAULT 0,
    blocked_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add comments for documentation
COMMENT ON TABLE wallet_social_graph IS 'Tracks social graph counts (followers, following, blocked) for wallet addresses that do not have profiles. Counts migrate to profiles table when profile is created.';
COMMENT ON COLUMN wallet_social_graph.wallet_address IS 'Blockchain wallet address (primary key)';
COMMENT ON COLUMN wallet_social_graph.followers_count IS 'Number of wallets following this address';
COMMENT ON COLUMN wallet_social_graph.following_count IS 'Number of wallets this address is following';
COMMENT ON COLUMN wallet_social_graph.blocked_count IS 'Number of wallets this address has blocked';
