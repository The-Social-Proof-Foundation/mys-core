-- Remove profile_id column from governance tables
-- Address is the primary identifier for governance, not profile_id

-- Drop views that reference profile_id first
DROP VIEW IF EXISTS delegate_performance CASCADE;

-- Drop indexes that reference profile_id
DROP INDEX IF EXISTS idx_delegates_profile_id;
DROP INDEX IF EXISTS idx_nominees_profile_id;

-- Drop profile_id column from delegates table
ALTER TABLE delegates 
    DROP COLUMN IF EXISTS profile_id CASCADE;

-- Drop profile_id column from nominated_delegates table  
ALTER TABLE nominated_delegates
    DROP COLUMN IF EXISTS profile_id CASCADE;

-- Recreate delegate_performance view without profile_id
CREATE OR REPLACE VIEW delegate_performance AS
SELECT
    d.address,
    d.registry_type,
    d.upvotes,
    d.downvotes,
    d.proposals_reviewed,
    d.proposals_submitted,
    d.sided_winning_proposals,
    d.sided_losing_proposals,
    d.term_start,
    d.term_end,
    d.is_active,
    CASE 
        WHEN d.proposals_reviewed > 0 THEN 
            d.sided_winning_proposals::FLOAT / NULLIF(d.proposals_reviewed, 0) 
        ELSE NULL 
    END AS winning_rate,
    COUNT(DISTINCT dv.proposal_id) AS recent_votes,
    SUM(CASE WHEN dv.approve THEN 1 ELSE 0 END) AS recent_approvals,
    SUM(CASE WHEN NOT dv.approve THEN 1 ELSE 0 END) AS recent_rejections
FROM
    delegates d
LEFT JOIN
    delegate_votes dv ON d.address = dv.delegate_address 
                      AND dv.time > NOW() - INTERVAL '30 days'
GROUP BY
    d.id, d.address, d.registry_type, d.upvotes, d.downvotes, 
    d.proposals_reviewed, d.proposals_submitted, d.sided_winning_proposals, 
    d.sided_losing_proposals, d.term_start, d.term_end, d.is_active;

