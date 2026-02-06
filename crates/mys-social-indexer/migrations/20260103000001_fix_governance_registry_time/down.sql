-- Revert all trigger functions to the original (incorrect) versions
CREATE OR REPLACE FUNCTION update_registry_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_delegate_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_nominee_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.nomination_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_proposal_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submission_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_rating_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.rated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_delegate_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.vote_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_community_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.vote_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_distribution_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.distribution_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_anonymous_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submitted_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_decryption_failure_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.attempted_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

