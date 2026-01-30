-- Migration: Fix platform_revenue_summary view to include last_active_month column
-- Version: 20251017000000
-- Purpose: Update view to match Rust code expectations for platform revenue stats endpoint

-- Drop existing views first (required when changing column names)
DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
DROP VIEW IF EXISTS spt_creator_revenue_summary CASCADE;

-- Recreate platform_revenue_summary view with last_active_month and updated column names
CREATE VIEW platform_revenue_summary AS
SELECT 
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT creator_address) AS total_creators,
    COUNT(DISTINCT payer_address) AS total_payers,
    AVG(amount) AS avg_transaction_amount,
    COUNT(DISTINCT DATE_TRUNC('month', time)) AS active_months,
    DATE_TRUNC('month', MAX(time))::DATE AS last_active_month
FROM unified_revenue
WHERE platform_address IS NOT NULL
    AND time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address
ORDER BY total_revenue DESC;

COMMENT ON VIEW platform_revenue_summary IS 'Platform revenue analytics using direct unified_revenue queries (12-month summary)';

-- Recreate spt_creator_revenue_summary view with updated column names (mydata instead of my_ip)
CREATE VIEW spt_creator_revenue_summary AS
SELECT 
    creator_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'tips' THEN amount ELSE 0 END) AS total_tips_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT payer_address) AS total_unique_payers,
    MAX(amount) AS largest_single_transaction,
    COUNT(DISTINCT DATE(time)) AS active_days,
    MAX(time) AS last_revenue_date
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;

COMMENT ON VIEW spt_creator_revenue_summary IS 'Creator revenue leaderboard using direct unified_revenue queries (30-day summary)';

