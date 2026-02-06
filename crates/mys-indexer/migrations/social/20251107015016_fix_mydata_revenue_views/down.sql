-- Migration Down: Revert mydata revenue views fix
-- Version: 20251107015016

-- Revert data changes
UPDATE unified_revenue 
SET revenue_source = 'my_ip' 
WHERE revenue_source = 'mydata';

-- Drop the new CHECK constraint
DO $$
DECLARE
    constraint_name TEXT;
BEGIN
    -- Find the CHECK constraint on revenue_source column
    SELECT conname INTO constraint_name
    FROM pg_constraint
    WHERE conrelid = 'unified_revenue'::regclass
        AND contype = 'c'
        AND conname LIKE '%revenue_source%';
    
    -- Drop the constraint if found
    IF constraint_name IS NOT NULL THEN
        EXECUTE format('ALTER TABLE unified_revenue DROP CONSTRAINT %I', constraint_name);
    END IF;
END $$;

-- Add back the old CHECK constraint
ALTER TABLE unified_revenue ADD CONSTRAINT unified_revenue_revenue_source_check 
    CHECK (revenue_source IN ('subscription', 'my_ip', 'spt', 'tips', 'posts'));

-- Drop views
DROP VIEW IF EXISTS revenue_dashboard_24h CASCADE;
DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
DROP VIEW IF EXISTS spt_creator_revenue_summary CASCADE;

-- Recreate views with old column names (my_ip)
CREATE OR REPLACE VIEW revenue_dashboard_24h AS
SELECT 
    revenue_source,
    SUM(amount) AS total_revenue_24h,
    COUNT(*) AS total_transactions_24h,
    COUNT(DISTINCT creator_address) AS unique_creators_24h,
    COUNT(DISTINCT payer_address) AS unique_payers_24h,
    MAX(amount) AS largest_transaction_24h,
    AVG(amount) AS avg_transaction_amount
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '24 hours'
GROUP BY revenue_source
ORDER BY total_revenue_24h DESC;

CREATE OR REPLACE VIEW platform_revenue_summary AS
SELECT 
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'my_ip' THEN amount ELSE 0 END) AS total_myip_revenue,
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

CREATE OR REPLACE VIEW spt_creator_revenue_summary AS
SELECT 
    creator_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'my_ip' THEN amount ELSE 0 END) AS total_myip_revenue,
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

