-- Add DAO governance fields to platforms table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'wants_dao_governance'
    ) THEN
        ALTER TABLE platforms ADD COLUMN wants_dao_governance BOOLEAN;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'governance_registry_id'
    ) THEN
        ALTER TABLE platforms ADD COLUMN governance_registry_id TEXT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'delegate_count'
    ) THEN
        ALTER TABLE platforms ADD COLUMN delegate_count BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'delegate_term_epochs'
    ) THEN
        ALTER TABLE platforms ADD COLUMN delegate_term_epochs BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'max_votes_per_user'
    ) THEN
        ALTER TABLE platforms ADD COLUMN max_votes_per_user BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'min_on_chain_age_days'
    ) THEN
        ALTER TABLE platforms ADD COLUMN min_on_chain_age_days BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'proposal_submission_cost'
    ) THEN
        ALTER TABLE platforms ADD COLUMN proposal_submission_cost BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'quadratic_base_cost'
    ) THEN
        ALTER TABLE platforms ADD COLUMN quadratic_base_cost BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'quorum_votes'
    ) THEN
        ALTER TABLE platforms ADD COLUMN quorum_votes BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'voting_period_epochs'
    ) THEN
        ALTER TABLE platforms ADD COLUMN voting_period_epochs BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'treasury'
    ) THEN
        ALTER TABLE platforms ADD COLUMN treasury BIGINT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'version'
    ) THEN
        ALTER TABLE platforms ADD COLUMN version BIGINT;
    END IF;
END $$;

-- Note: shutdown_date already exists in the schema but wasn't being populated from events
-- This migration adds all the missing DAO/governance fields

