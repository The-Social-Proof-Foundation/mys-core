-- Migration: Revert platform_revenue_summary view to original definition
-- Version: 20251017000000
-- Purpose: Restore original view with last_active_time column

-- Revert platform_revenue_summary view to original definition with last_active_time
CREATE OR REPLACE VIEW platform_revenue_summary AS
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
    MAX(time) AS last_active_time
FROM unified_revenue
WHERE platform_address IS NOT NULL
    AND time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address
ORDER BY total_revenue DESC;

COMMENT ON VIEW platform_revenue_summary IS 'Platform revenue analytics using direct unified_revenue queries (12-month summary)';

