-- Backfill wallet_social_graph with existing relationship data for wallet addresses without profiles
-- This migration aggregates followers_count and following_count from social_graph_relationships

-- First, aggregate followers_count and following_count for wallets without profiles
INSERT INTO wallet_social_graph (wallet_address, followers_count, following_count, created_at, updated_at)
SELECT 
    wallet_address,
    SUM(followers_count)::INTEGER as followers_count,
    SUM(following_count)::INTEGER as following_count,
    NOW() as created_at,
    NOW() as updated_at
FROM (
    -- Get followers_count for wallets without profiles
    SELECT 
        following_address as wallet_address,
        COUNT(*)::INTEGER as followers_count,
        0::INTEGER as following_count
    FROM social_graph_relationships
    WHERE following_address NOT IN (
        SELECT COALESCE(owner_address, '') FROM profiles WHERE owner_address IS NOT NULL
        UNION
        SELECT COALESCE(profile_id, '') FROM profiles WHERE profile_id IS NOT NULL
    )
    GROUP BY following_address
    
    UNION ALL
    
    -- Get following_count for wallets without profiles
    SELECT 
        follower_address as wallet_address,
        0::INTEGER as followers_count,
        COUNT(*)::INTEGER as following_count
    FROM social_graph_relationships
    WHERE follower_address NOT IN (
        SELECT COALESCE(owner_address, '') FROM profiles WHERE owner_address IS NOT NULL
        UNION
        SELECT COALESCE(profile_id, '') FROM profiles WHERE profile_id IS NOT NULL
    )
    GROUP BY follower_address
) AS counts
GROUP BY wallet_address
ON CONFLICT (wallet_address) 
DO UPDATE SET 
    followers_count = EXCLUDED.followers_count,
    following_count = EXCLUDED.following_count,
    updated_at = NOW();

-- Backfill blocked_count for wallets without profiles
-- Update existing wallet_social_graph entries
UPDATE wallet_social_graph wsg
SET blocked_count = bp_counts.cnt
FROM (
    SELECT blocker_address, COUNT(*)::INTEGER as cnt
    FROM blocked_profiles
    WHERE blocker_address NOT IN (
        SELECT COALESCE(owner_address, '') FROM profiles WHERE owner_address IS NOT NULL
    )
    GROUP BY blocker_address
) AS bp_counts
WHERE wsg.wallet_address = bp_counts.blocker_address;

-- Insert blocked_count for wallets that don't exist in wallet_social_graph yet
INSERT INTO wallet_social_graph (wallet_address, blocked_count, created_at, updated_at)
SELECT 
    blocker_address,
    COUNT(*)::INTEGER,
    NOW(),
    NOW()
FROM blocked_profiles
WHERE blocker_address NOT IN (
    SELECT COALESCE(owner_address, '') FROM profiles WHERE owner_address IS NOT NULL
)
AND blocker_address NOT IN (SELECT wallet_address FROM wallet_social_graph)
GROUP BY blocker_address
ON CONFLICT (wallet_address) DO NOTHING;
