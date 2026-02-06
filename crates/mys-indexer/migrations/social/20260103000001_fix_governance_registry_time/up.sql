-- Fix all governance time triggers to handle milliseconds correctly
-- All timestamp fields from blockchain events are in milliseconds (epoch timestamp in ms),
-- but PostgreSQL's to_timestamp() expects seconds, so we need to divide by 1000

-- 1. Fix governance_registries time trigger
CREATE OR REPLACE FUNCTION update_registry_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.updated_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 2. Fix delegates time trigger
CREATE OR REPLACE FUNCTION update_delegate_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.updated_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 3. Fix nominated_delegates time trigger
CREATE OR REPLACE FUNCTION update_nominee_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.nomination_time / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 4. Fix proposals time trigger
CREATE OR REPLACE FUNCTION update_proposal_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.submission_time / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 5. Fix delegate_ratings time trigger
CREATE OR REPLACE FUNCTION update_rating_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.rated_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 6. Fix delegate_votes time trigger
CREATE OR REPLACE FUNCTION update_delegate_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.vote_time / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 7. Fix community_votes time trigger
CREATE OR REPLACE FUNCTION update_community_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.vote_time / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 8. Fix reward_distributions time trigger
CREATE OR REPLACE FUNCTION update_distribution_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.distribution_time / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 9. Fix anonymous_votes time trigger
CREATE OR REPLACE FUNCTION update_anonymous_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.submitted_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 10. Fix vote_decryption_failures time trigger
CREATE OR REPLACE FUNCTION update_decryption_failure_time()
RETURNS TRIGGER AS $$
BEGIN
    -- Convert milliseconds to seconds by dividing by 1000
    NEW.time = to_timestamp(NEW.attempted_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Fix any existing bad data in all tables
-- Only update rows where time is clearly wrong (in the far future, > year 2100)
UPDATE governance_registries 
SET time = to_timestamp(updated_at / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE delegates 
SET time = to_timestamp(updated_at / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE nominated_delegates 
SET time = to_timestamp(nomination_time / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE proposals 
SET time = to_timestamp(submission_time / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE delegate_ratings 
SET time = to_timestamp(rated_at / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE delegate_votes 
SET time = to_timestamp(vote_time / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE community_votes 
SET time = to_timestamp(vote_time / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE reward_distributions 
SET time = to_timestamp(distribution_time / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE anonymous_votes 
SET time = to_timestamp(submitted_at / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

UPDATE vote_decryption_failures 
SET time = to_timestamp(attempted_at / 1000.0)
WHERE time > '2100-01-01'::timestamptz;

