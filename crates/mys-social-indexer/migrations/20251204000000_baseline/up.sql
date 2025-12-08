--
-- PostgreSQL database dump
--

-- Dumped from database version 16.10
-- Dumped by pg_dump version 16.11 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: timescaledb; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS timescaledb WITH SCHEMA public;

-- Verify TimescaleDB extension was created successfully
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_extension WHERE extname = 'timescaledb'
    ) THEN
        RAISE EXCEPTION 'TimescaleDB extension could not be created. Ensure TimescaleDB is installed on the PostgreSQL instance.';
    END IF;
END $$;

--
-- Name: EXTENSION timescaledb; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION timescaledb IS 'Enables scalable inserts and complex queries for time-series data (Community Edition)';


--
-- Name: calculate_vesting_claimable(bigint, bigint, bigint, bigint, bigint, bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.calculate_vesting_claimable(total_amount_param bigint, start_time_param bigint, duration_param bigint, curve_factor_param bigint, claimed_amount_param bigint, current_time_param bigint) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
    elapsed_time BIGINT;
    progress_ratio DOUBLE PRECISION;
    curve_factor_normalized DOUBLE PRECISION;
    curved_progress DOUBLE PRECISION;
    total_vested BIGINT;
    claimable BIGINT;
    precision_factor CONSTANT DOUBLE PRECISION := 1000.0;
BEGIN
    -- If vesting hasn't started yet
    IF current_time_param < start_time_param THEN
        RETURN 0;
    END IF;
    
    elapsed_time := current_time_param - start_time_param;
    
    -- If vesting period is complete
    IF elapsed_time >= duration_param THEN
        RETURN total_amount_param - claimed_amount_param;
    END IF;
    
    -- Calculate progress ratio (0.0 to 1.0)
    progress_ratio := CAST(elapsed_time AS DOUBLE PRECISION) / CAST(duration_param AS DOUBLE PRECISION);
    
    -- Normalize curve factor
    curve_factor_normalized := CAST(curve_factor_param AS DOUBLE PRECISION) / precision_factor;
    
    -- Apply curve based on curve factor
    IF curve_factor_param = 0 OR curve_factor_param = 1000 THEN
        -- Linear vesting
        curved_progress := progress_ratio;
    ELSIF curve_factor_param > 1000 THEN
        -- Exponential curve (more tokens toward end)
        -- Use quadratic approximation: progress^2
        curved_progress := progress_ratio * progress_ratio;
        -- Blend with linear based on how far curve_factor is from 1000
        DECLARE
            blend_factor DOUBLE PRECISION := LEAST((curve_factor_normalized - 1.0) * 2.0, 1.0);
        BEGIN
            curved_progress := progress_ratio * (1.0 - blend_factor) + curved_progress * blend_factor;
        END;
    ELSE
        -- Logarithmic curve (more tokens toward start)  
        -- Use square root approximation: sqrt(progress)
        curved_progress := SQRT(progress_ratio);
        -- Blend with linear based on how far curve_factor is from 1000
        DECLARE
            blend_factor DOUBLE PRECISION := LEAST((1.0 - curve_factor_normalized) * 2.0, 1.0);
        BEGIN
            curved_progress := progress_ratio * (1.0 - blend_factor) + curved_progress * blend_factor;
        END;
    END IF;
    
    -- Calculate total vested amount
    total_vested := CAST(CAST(total_amount_param AS DOUBLE PRECISION) * curved_progress AS BIGINT);
    
    -- Calculate claimable (total vested minus already claimed)
    claimable := total_vested - claimed_amount_param;
    
    -- Ensure non-negative result
    RETURN GREATEST(claimable, 0);
END;
$$;


--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: get_current_exchange_config(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.get_current_exchange_config() RETURNS TABLE(post_threshold bigint, profile_threshold bigint, max_individual_reservation_bps bigint, trading_halted boolean)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.post_threshold,
        c.profile_threshold, 
        c.max_individual_reservation_bps,
        c.trading_halted
    FROM spt_exchange_config c
    ORDER BY c.time DESC
    LIMIT 1;
END;
$$;


--
-- Name: get_data_pricing(character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.get_data_pricing(p_ip_id character varying) RETURNS TABLE(ip_id character varying, one_time_price bigint, subscription_price bigint, subscription_duration_days bigint, total_purchasers bigint, total_subscribers bigint, total_revenue bigint)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT 
        d.ip_id,
        d.one_time_price,
        d.subscription_price,
        d.subscription_duration_days,
        COALESCE(p.purchaser_count, 0) AS total_purchasers,
        COALESCE(s.subscriber_count, 0) AS total_subscribers,
        COALESCE(r.total_revenue, 0) AS total_revenue
    FROM my_ip_data d
    LEFT JOIN (
        SELECT ip_id, COUNT(DISTINCT buyer) AS purchaser_count
        FROM my_ip_purchases 
        WHERE purchase_type = 'one_time'
        GROUP BY ip_id
    ) p ON d.ip_id = p.ip_id
    LEFT JOIN (
        SELECT ip_id, COUNT(DISTINCT subscriber) AS subscriber_count
        FROM my_ip_subscriptions
        GROUP BY ip_id
    ) s ON d.ip_id = s.ip_id
    LEFT JOIN (
        SELECT ip_id, SUM(amount) AS total_revenue
        FROM my_ip_revenue
        GROUP BY ip_id
    ) r ON d.ip_id = r.ip_id
    WHERE d.ip_id = p_ip_id;
END;
$$;


--
-- Name: get_mydata_pricing(text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.get_mydata_pricing(p_mydata_id text) RETURNS TABLE(mydata_id text, one_time_price bigint, subscription_price bigint, subscription_duration_days bigint, total_purchasers bigint, total_subscribers bigint, total_revenue bigint)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT 
        d.mydata_id,
        d.one_time_price,
        d.subscription_price,
        d.subscription_duration_days,
        COALESCE(p.purchaser_count, 0) AS total_purchasers,
        COALESCE(s.subscriber_count, 0) AS total_subscribers,
        COALESCE(r.total_revenue, 0) AS total_revenue
    FROM mydata_data d
    LEFT JOIN (
        SELECT mydata_id, COUNT(DISTINCT buyer) AS purchaser_count
        FROM mydata_purchases 
        WHERE purchase_type = 'one_time'
        GROUP BY mydata_id
    ) p ON d.mydata_id = p.mydata_id
    LEFT JOIN (
        SELECT mydata_id, COUNT(DISTINCT subscriber) AS subscriber_count
        FROM mydata_subscriptions
        GROUP BY mydata_id
    ) s ON d.mydata_id = s.mydata_id
    LEFT JOIN (
        SELECT mydata_id, SUM(amount) AS total_revenue
        FROM mydata_revenue
        GROUP BY mydata_id
    ) r ON d.mydata_id = r.mydata_id
    WHERE d.mydata_id = p_mydata_id;
END;
$$;


--
-- Name: get_vesting_progress(bigint, bigint, bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.get_vesting_progress(start_time_param bigint, duration_param bigint, current_time_param bigint) RETURNS double precision
    LANGUAGE plpgsql
    AS $$
DECLARE
    elapsed_time BIGINT;
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 0.0;
    END IF;
    
    elapsed_time := current_time_param - start_time_param;
    
    IF elapsed_time >= duration_param THEN
        RETURN 100.0;
    END IF;
    
    RETURN (CAST(elapsed_time AS DOUBLE PRECISION) / CAST(duration_param AS DOUBLE PRECISION)) * 100.0;
END;
$$;


--
-- Name: get_vesting_status(bigint, bigint, bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.get_vesting_status(start_time_param bigint, duration_param bigint, current_time_param bigint) RETURNS character varying
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 'not_started';
    ELSIF current_time_param >= start_time_param + duration_param THEN
        RETURN 'completed';
    ELSE
        RETURN 'in_progress';
    END IF;
END;
$$;


--
-- Name: is_reservation_threshold_met(character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.is_reservation_threshold_met(pool_id_param character varying) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
DECLARE
    result BOOLEAN;
BEGIN
    SELECT (sp.total_reserved >= sp.required_threshold) INTO result
    FROM spt_reservation_pools sp
    WHERE sp.pool_id = pool_id_param
    ORDER BY sp.time DESC
    LIMIT 1;
    
    RETURN COALESCE(result, false);
END;
$$;


--
-- Name: prevent_social_graph_events_deletion(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.prevent_social_graph_events_deletion() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    RAISE EXCEPTION 'Deleting social_graph_events records is not allowed';
END;
$$;


--
-- Name: refresh_license_materialized_views(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.refresh_license_materialized_views() RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    REFRESH MATERIALIZED VIEW daily_license_revenue;
    REFRESH MATERIALIZED VIEW weekly_creator_revenue;
END;
$$;


--
-- Name: refresh_spt_price_daily(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.refresh_spt_price_daily() RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    REFRESH MATERIALIZED VIEW spt_price_daily;
END;
$$;


--
-- Name: refresh_spt_price_hourly(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.refresh_spt_price_hourly() RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    REFRESH MATERIALIZED VIEW spt_price_hourly;
END;
$$;


--
-- Name: update_anonymous_vote_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_anonymous_vote_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submitted_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_comment_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_comment_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_community_vote_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_community_vote_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.vote_time);
    RETURN NEW;
END;
$$;


--
-- Name: update_decryption_failure_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_decryption_failure_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.attempted_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_delegate_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_delegate_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_delegate_vote_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_delegate_vote_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.vote_time);
    RETURN NEW;
END;
$$;


--
-- Name: update_deletion_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_deletion_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.deleted_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_distribution_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_distribution_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.distribution_time);
    RETURN NEW;
END;
$$;


--
-- Name: update_moderation_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_moderation_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.moderated_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_nominee_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_nominee_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.nomination_time);
    RETURN NEW;
END;
$$;


--
-- Name: update_poc_analysis_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_poc_analysis_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.analysis_timestamp / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_poc_badge_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_poc_badge_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.issued_at / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_poc_dispute_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_poc_dispute_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submitted_at / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_poc_redirection_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_poc_redirection_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_poc_vote_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_poc_vote_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.voted_at / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_post_prediction_config_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_post_prediction_config_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_post_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_post_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_promoted_post_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_promoted_post_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_promotion_budget_event_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_promotion_budget_event_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_promotion_status_event_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_promotion_status_event_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_promotion_view_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_promotion_view_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_proposal_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_proposal_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submission_time);
    RETURN NEW;
END;
$$;


--
-- Name: update_rating_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_rating_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.rated_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_reaction_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_reaction_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_registry_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_registry_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_report_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_report_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.reported_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_repost_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_repost_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_spot_config_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_spot_config_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp_ms / 1000);
    RETURN NEW;
END;
$$;


--
-- Name: update_tip_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_tip_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$;


--
-- Name: update_transfer_time(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.update_transfer_time() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.time = to_timestamp(NEW.transferred_at);
    RETURN NEW;
END;
$$;


--
-- Name: user_has_access(character varying, character varying, bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.user_has_access(p_ip_id character varying, p_user_address character varying, p_current_time bigint DEFAULT EXTRACT(epoch FROM now())) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
DECLARE
    data_owner VARCHAR;
    subscription_end BIGINT;
    has_purchase BOOLEAN := FALSE;
BEGIN
    -- Get data owner
    SELECT owner INTO data_owner FROM my_ip_data WHERE ip_id = p_ip_id;
    
    -- Owner always has access
    IF data_owner = p_user_address THEN
        RETURN TRUE;
    END IF;
    
    -- Check for one-time purchase
    SELECT TRUE INTO has_purchase 
    FROM my_ip_purchases 
    WHERE ip_id = p_ip_id AND buyer = p_user_address AND purchase_type = 'one_time'
    LIMIT 1;
    
    IF has_purchase THEN
        RETURN TRUE;
    END IF;
    
    -- Check for active subscription
    SELECT MAX(subscription_end) INTO subscription_end
    FROM my_ip_subscriptions 
    WHERE ip_id = p_ip_id AND subscriber = p_user_address;
    
    IF subscription_end IS NOT NULL AND subscription_end >= p_current_time THEN
        RETURN TRUE;
    END IF;
    
    RETURN FALSE;
END;
$$;


--
-- Name: user_has_mydata_access(text, text, bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.user_has_mydata_access(p_mydata_id text, p_user_address text, p_current_time bigint DEFAULT EXTRACT(epoch FROM now())) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
DECLARE
    data_owner TEXT;
    subscription_end BIGINT;
    has_purchase BOOLEAN := FALSE;
BEGIN
    -- Get data owner
    SELECT owner INTO data_owner FROM mydata_data WHERE mydata_id = p_mydata_id;
    
    -- Owner always has access
    IF data_owner = p_user_address THEN
        RETURN TRUE;
    END IF;
    
    -- Check for one-time purchase
    SELECT TRUE INTO has_purchase 
    FROM mydata_purchases 
    WHERE mydata_id = p_mydata_id AND buyer = p_user_address AND purchase_type = 'one_time'
    LIMIT 1;
    
    IF has_purchase THEN
        RETURN TRUE;
    END IF;
    
    -- Check for active subscription
    SELECT MAX(subscription_end) INTO subscription_end
    FROM mydata_subscriptions 
    WHERE mydata_id = p_mydata_id AND subscriber = p_user_address;
    
    IF subscription_end IS NOT NULL AND subscription_end >= p_current_time THEN
        RETURN TRUE;
    END IF;
    
    RETURN FALSE;
END;
$$;


--
-- Name: validate_anonymous_vote_proposal(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_anonymous_vote_proposal() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_poc_dispute_reference(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_poc_dispute_reference() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM poc_disputes WHERE dispute_id = NEW.dispute_id) THEN
        RAISE EXCEPTION 'Referenced dispute_id does not exist: %', NEW.dispute_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_poc_original_post_reference(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_poc_original_post_reference() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.original_post_id) THEN
        RAISE EXCEPTION 'Referenced original_post_id does not exist: %', NEW.original_post_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_poc_post_reference(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_poc_post_reference() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.post_id) THEN
        RAISE EXCEPTION 'Referenced post_id does not exist: %', NEW.post_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_post_reference(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_post_reference() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.post_id) THEN
        RAISE EXCEPTION 'Referenced post_id does not exist: %', NEW.post_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_proposal_community_vote(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_proposal_community_vote() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_proposal_delegate_vote(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_proposal_delegate_vote() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: validate_proposal_reward(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.validate_proposal_reward() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: verify_follow_counts(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE OR REPLACE FUNCTION public.verify_follow_counts() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    -- Log whenever an update happens
    RAISE NOTICE 'Follow relationship changed: %', TG_OP;
    
    -- Different actions based on operation type
    IF (TG_OP = 'INSERT') THEN
        UPDATE profiles
        SET following_count = following_count + 1
        WHERE owner_address = NEW.follower_address;
        
        UPDATE profiles
        SET followers_count = followers_count + 1
        WHERE owner_address = NEW.following_address;
        
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        UPDATE profiles
        SET following_count = GREATEST(0, following_count - 1)
        WHERE owner_address = OLD.follower_address;
        
        UPDATE profiles
        SET followers_count = GREATEST(0, followers_count - 1)
        WHERE owner_address = OLD.following_address;
        
        RETURN OLD;
    END IF;
    
    RETURN NULL;
END;
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _compressed_hypertable_101; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_101 (
);


--
-- Name: _compressed_hypertable_103; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_103 (
);


--
-- Name: _compressed_hypertable_105; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_105 (
);


--
-- Name: _compressed_hypertable_11; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_11 (
);


--
-- Name: _compressed_hypertable_114; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_114 (
);


--
-- Name: _compressed_hypertable_115; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_115 (
);


--
-- Name: _compressed_hypertable_116; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_116 (
);


--
-- Name: _compressed_hypertable_120; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_120 (
);


--
-- Name: _compressed_hypertable_122; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_122 (
);


--
-- Name: _compressed_hypertable_126; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_126 (
);


--
-- Name: _compressed_hypertable_127; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_127 (
);


--
-- Name: _compressed_hypertable_129; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_129 (
);


--
-- Name: _compressed_hypertable_131; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_131 (
);


--
-- Name: _compressed_hypertable_133; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_133 (
);


--
-- Name: _compressed_hypertable_135; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_135 (
);


--
-- Name: _compressed_hypertable_14; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_14 (
);


--
-- Name: _compressed_hypertable_145; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_145 (
);


--
-- Name: _compressed_hypertable_147; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_147 (
);


--
-- Name: _compressed_hypertable_149; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_149 (
);


--
-- Name: _compressed_hypertable_151; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_151 (
);


--
-- Name: _compressed_hypertable_154; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_154 (
);


--
-- Name: _compressed_hypertable_156; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_156 (
);


--
-- Name: _compressed_hypertable_158; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_158 (
);


--
-- Name: _compressed_hypertable_16; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_16 (
);


--
-- Name: _compressed_hypertable_160; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_160 (
);


--
-- Name: _compressed_hypertable_18; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_18 (
);


--
-- Name: _compressed_hypertable_2; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_2 (
);


--
-- Name: _compressed_hypertable_20; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_20 (
);


--
-- Name: _compressed_hypertable_22; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_22 (
);


--
-- Name: _compressed_hypertable_24; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_24 (
);


--
-- Name: _compressed_hypertable_26; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_26 (
);


--
-- Name: _compressed_hypertable_28; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_28 (
);


--
-- Name: _compressed_hypertable_30; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_30 (
);


--
-- Name: _compressed_hypertable_36; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_36 (
);


--
-- Name: _compressed_hypertable_38; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_38 (
);


--
-- Name: _compressed_hypertable_4; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_4 (
);


--
-- Name: _compressed_hypertable_40; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_40 (
);


--
-- Name: _compressed_hypertable_42; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_42 (
);


--
-- Name: _compressed_hypertable_44; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_44 (
);


--
-- Name: _compressed_hypertable_46; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_46 (
);


--
-- Name: _compressed_hypertable_48; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_48 (
);


--
-- Name: _compressed_hypertable_6; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_6 (
);


--
-- Name: _compressed_hypertable_62; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_62 (
);


--
-- Name: _compressed_hypertable_64; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_64 (
);


--
-- Name: _compressed_hypertable_66; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_66 (
);


--
-- Name: _compressed_hypertable_72; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_72 (
);


--
-- Name: _compressed_hypertable_76; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_76 (
);


--
-- Name: _compressed_hypertable_78; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_78 (
);


--
-- Name: _compressed_hypertable_80; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_80 (
);


--
-- Name: _compressed_hypertable_82; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_82 (
);


--
-- Name: _compressed_hypertable_89; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_89 (
);


--
-- Name: _compressed_hypertable_90; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_90 (
);


--
-- Name: _compressed_hypertable_91; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_91 (
);


--
-- Name: _compressed_hypertable_92; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_92 (
);


--
-- Name: _compressed_hypertable_97; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_97 (
);


--
-- Name: _compressed_hypertable_99; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._compressed_hypertable_99 (
);


--
-- Name: poc_badges; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.poc_badges (
    badge_id character varying NOT NULL,
    post_id character varying NOT NULL,
    media_type smallint NOT NULL,
    issued_by character varying NOT NULL,
    issued_at bigint NOT NULL,
    revoked boolean DEFAULT false,
    revoked_at bigint,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE poc_badges; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.poc_badges IS 'PoC badges issued for original content verification';


--
-- Name: COLUMN poc_badges.media_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.poc_badges.media_type IS '1=image, 2=video, 3=audio';


--
-- Name: _direct_view_106; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE OR REPLACE VIEW _timescaledb_internal._direct_view_106 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    count(*) FILTER (WHERE (NOT revoked)) AS badges_issued,
    0 AS redirections_created,
    0 AS disputes_submitted,
    0 AS votes_cast
   FROM public.poc_badges
  GROUP BY (public.time_bucket('1 day'::interval, "time"));


--
-- Name: _direct_view_107; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE OR REPLACE VIEW _timescaledb_internal._direct_view_107 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    count(*) FILTER (WHERE (NOT revoked)) AS badges_issued_hourly,
    count(*) AS total_badges
   FROM public.poc_badges
  GROUP BY (public.time_bucket('01:00:00'::interval, "time"));


--
-- Name: subscription_revenue; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.subscription_revenue (
    service_id character varying NOT NULL,
    subscription_id character varying,
    from_address character varying NOT NULL,
    to_address character varying NOT NULL,
    amount bigint NOT NULL,
    revenue_type character varying NOT NULL,
    payment_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    processing_success boolean DEFAULT true NOT NULL,
    processing_error text
);


--
-- Name: _direct_view_112; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_112 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    service_id,
    to_address AS profile_owner,
    revenue_type,
    sum(amount) AS daily_revenue,
    count(*) AS transaction_count
   FROM public.subscription_revenue
  GROUP BY (public.time_bucket('1 day'::interval, "time")), service_id, to_address, revenue_type;


--
-- Name: profile_subscriptions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profile_subscriptions (
    subscription_id character varying NOT NULL,
    service_id character varying NOT NULL,
    subscriber character varying NOT NULL,
    created_at bigint NOT NULL,
    expires_at bigint NOT NULL,
    auto_renew boolean DEFAULT false NOT NULL,
    renewal_balance bigint DEFAULT 0 NOT NULL,
    renewal_count bigint DEFAULT 0 NOT NULL,
    cancelled_at bigint,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    processing_success boolean DEFAULT true NOT NULL,
    processing_error text
);


--
-- Name: _direct_view_113; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_113 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    service_id,
    count(*) FILTER (WHERE (cancelled_at IS NULL)) AS new_subscriptions,
    count(*) FILTER (WHERE (cancelled_at IS NOT NULL)) AS cancelled_subscriptions,
    count(*) FILTER (WHERE (renewal_count > 0)) AS renewed_subscriptions
   FROM public.profile_subscriptions
  GROUP BY (public.time_bucket('1 day'::interval, "time")), service_id;


--
-- Name: _direct_view_117; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_117 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    service_id,
    count(*) FILTER (WHERE (cancelled_at IS NULL)) AS active_subscriptions,
    count(*) FILTER (WHERE (cancelled_at IS NOT NULL)) AS cancelled_subscriptions,
    avg(renewal_count) AS avg_renewal_count,
    sum(renewal_balance) AS total_renewal_balance
   FROM public.profile_subscriptions
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), service_id;


--
-- Name: _direct_view_118; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_118 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    service_id,
    count(*) FILTER (WHERE (((cancelled_at)::numeric >= EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time"))) AND ((cancelled_at)::numeric <= (EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time")) + (86400)::numeric)))) AS daily_churn,
    count(*) FILTER (WHERE (((created_at)::numeric >= EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time"))) AND ((created_at)::numeric <= (EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time")) + (86400)::numeric)))) AS daily_new_subs
   FROM public.profile_subscriptions
  GROUP BY (public.time_bucket('1 day'::interval, "time")), service_id;


--
-- Name: checkpoint_processing; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.checkpoint_processing (
    id integer NOT NULL,
    checkpoint_number bigint NOT NULL,
    processing_start_time timestamp without time zone DEFAULT now() NOT NULL,
    processing_end_time timestamp without time zone,
    events_processed integer DEFAULT 0,
    profiles_created integer DEFAULT 0,
    profiles_updated integer DEFAULT 0,
    follows_created integer DEFAULT 0,
    follows_removed integer DEFAULT 0,
    platform_events integer DEFAULT 0,
    block_events integer DEFAULT 0,
    processing_status character varying(50) DEFAULT 'in_progress'::character varying,
    processing_duration_ms integer,
    error_message text
);


--
-- Name: _direct_view_12; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_12 AS
 SELECT public.time_bucket('1 day'::interval, processing_start_time) AS day,
    count(*) AS checkpoints_processed,
    sum(events_processed) AS total_events_processed,
    avg(processing_duration_ms) AS avg_processing_duration_ms,
    max(processing_duration_ms) AS max_processing_duration_ms,
    min(processing_duration_ms) AS min_processing_duration_ms
   FROM public.checkpoint_processing
  WHERE (((processing_status)::text = 'completed'::text) AND (processing_duration_ms IS NOT NULL))
  GROUP BY (public.time_bucket('1 day'::interval, processing_start_time));


--
-- Name: anonymous_votes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.anonymous_votes (
    id integer NOT NULL,
    proposal_id character varying NOT NULL,
    voter_address character varying NOT NULL,
    encrypted_vote_data bytea,
    submitted_at bigint NOT NULL,
    decrypted boolean DEFAULT false,
    decrypted_at bigint,
    decrypted_vote smallint,
    decryption_status smallint DEFAULT 0,
    decryption_error text,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    processing_success boolean DEFAULT true,
    processing_error text
);


--
-- Name: _direct_view_123; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_123 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    proposal_id,
    count(*) AS total_anonymous_votes,
    count(*) FILTER (WHERE (decrypted = true)) AS successfully_decrypted,
    count(*) FILTER (WHERE (decryption_status = 2)) AS failed_decryptions,
    count(*) FILTER (WHERE (decrypted_vote = 1)) AS anonymous_votes_for,
    count(*) FILTER (WHERE (decrypted_vote = 0)) AS anonymous_votes_against,
    count(*) FILTER (WHERE (decryption_status = 0)) AS pending_decryption
   FROM public.anonymous_votes
  GROUP BY (public.time_bucket('1 day'::interval, "time")), proposal_id;


--
-- Name: mydata_revenue; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.mydata_revenue (
    id integer NOT NULL,
    mydata_id character varying NOT NULL,
    from_address character varying NOT NULL,
    to_address character varying NOT NULL,
    amount bigint NOT NULL,
    revenue_type character varying NOT NULL,
    revenue_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT my_ip_revenue_revenue_type_check CHECK (((revenue_type)::text = ANY ((ARRAY['one_time'::character varying, 'subscription'::character varying, 'grant'::character varying])::text[])))
);


--
-- Name: TABLE mydata_revenue; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mydata_revenue IS 'Revenue distribution and tracking (TimescaleDB)';


--
-- Name: _direct_view_141; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_141 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    mydata_id,
    to_address AS creator,
    revenue_type,
    sum(amount) AS daily_revenue,
    count(*) AS transaction_count
   FROM public.mydata_revenue
  GROUP BY (public.time_bucket('1 day'::interval, "time")), mydata_id, to_address, revenue_type;


--
-- Name: mydata_access_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.mydata_access_logs (
    id integer NOT NULL,
    mydata_id character varying NOT NULL,
    user_address character varying NOT NULL,
    access_type character varying NOT NULL,
    access_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT my_ip_access_logs_access_type_check CHECK (((access_type)::text = ANY ((ARRAY['one_time'::character varying, 'subscription'::character varying, 'grant'::character varying, 'preview'::character varying])::text[])))
);


--
-- Name: TABLE mydata_access_logs; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mydata_access_logs IS 'Access pattern analytics and logs (TimescaleDB)';


--
-- Name: _direct_view_142; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_142 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    mydata_id,
    access_type,
    count(DISTINCT user_address) AS unique_users,
    count(*) AS total_accesses
   FROM public.mydata_access_logs
  GROUP BY (public.time_bucket('1 day'::interval, "time")), mydata_id, access_type;


--
-- Name: mydata_purchases; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.mydata_purchases (
    id integer NOT NULL,
    mydata_id character varying NOT NULL,
    buyer character varying NOT NULL,
    price bigint NOT NULL,
    purchase_type character varying NOT NULL,
    purchase_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT my_ip_purchases_purchase_type_check CHECK (((purchase_type)::text = ANY ((ARRAY['one_time'::character varying, 'subscription'::character varying])::text[])))
);


--
-- Name: TABLE mydata_purchases; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mydata_purchases IS 'Purchase records for one-time and subscription access (TimescaleDB)';


--
-- Name: _direct_view_143; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_143 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    mydata_id,
    count(DISTINCT buyer) AS unique_purchasers,
    sum(
        CASE
            WHEN ((purchase_type)::text = 'one_time'::text) THEN 1
            ELSE 0
        END) AS one_time_purchases,
    sum(
        CASE
            WHEN ((purchase_type)::text = 'subscription'::text) THEN 1
            ELSE 0
        END) AS subscriptions,
    sum(price) AS total_revenue
   FROM public.mydata_purchases
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), mydata_id;


--
-- Name: reactions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.reactions (
    id integer NOT NULL,
    object_id character varying NOT NULL,
    user_address character varying NOT NULL,
    reaction_text character varying NOT NULL,
    is_post boolean NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_31; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_31 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    object_id,
    reaction_text,
    count(*) AS reaction_count
   FROM public.reactions
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), object_id, reaction_text;


--
-- Name: reposts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.reposts (
    id character varying NOT NULL,
    repost_id character varying NOT NULL,
    original_id character varying NOT NULL,
    original_post_id character varying NOT NULL,
    is_original_post boolean NOT NULL,
    owner character varying NOT NULL,
    profile_id character varying NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_32; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_32 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    original_post_id,
    count(*) AS repost_count
   FROM public.reposts
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), original_post_id;


--
-- Name: tips; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.tips (
    id integer NOT NULL,
    tipper character varying NOT NULL,
    recipient character varying NOT NULL,
    object_id character varying NOT NULL,
    amount bigint NOT NULL,
    is_post boolean NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_33; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_33 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    object_id,
    is_post,
    sum(amount) AS total_amount,
    count(*) AS tip_count
   FROM public.tips
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), object_id, is_post;


--
-- Name: _direct_view_34; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_34 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS bucket,
    count(DISTINCT object_id) AS posts_with_tips,
    sum(amount) AS total_tip_amount,
    count(*) AS total_tips
   FROM public.tips
  WHERE (is_post = true)
  GROUP BY (public.time_bucket('1 day'::interval, "time"));


--
-- Name: delegate_ratings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.delegate_ratings (
    id integer NOT NULL,
    target_address character varying NOT NULL,
    voter_address character varying NOT NULL,
    registry_type smallint NOT NULL,
    is_active_delegate boolean NOT NULL,
    upvote boolean NOT NULL,
    rated_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_49; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_49 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    registry_type,
    target_address,
    sum(
        CASE
            WHEN upvote THEN 1
            ELSE 0
        END) AS upvotes,
    sum(
        CASE
            WHEN (NOT upvote) THEN 1
            ELSE 0
        END) AS downvotes,
    count(*) AS total_ratings
   FROM public.delegate_ratings
  GROUP BY (public.time_bucket('1 day'::interval, "time")), registry_type, target_address;


--
-- Name: delegate_votes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.delegate_votes (
    id integer NOT NULL,
    proposal_id character varying NOT NULL,
    delegate_address character varying NOT NULL,
    approve boolean NOT NULL,
    vote_time bigint NOT NULL,
    reason text,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_50; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_50 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    proposal_id,
    sum(
        CASE
            WHEN approve THEN 1
            ELSE 0
        END) AS approve_count,
    sum(
        CASE
            WHEN (NOT approve) THEN 1
            ELSE 0
        END) AS reject_count,
    count(*) AS total_votes
   FROM public.delegate_votes
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), proposal_id;


--
-- Name: community_votes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.community_votes (
    id integer NOT NULL,
    proposal_id character varying NOT NULL,
    voter_address character varying NOT NULL,
    vote_weight bigint NOT NULL,
    approve boolean NOT NULL,
    vote_time bigint NOT NULL,
    vote_cost bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_51; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_51 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    proposal_id,
    sum(
        CASE
            WHEN approve THEN vote_weight
            ELSE (0)::bigint
        END) AS approve_weight,
    sum(
        CASE
            WHEN (NOT approve) THEN vote_weight
            ELSE (0)::bigint
        END) AS reject_weight,
    count(*) AS total_votes
   FROM public.community_votes
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), proposal_id;


--
-- Name: reward_distributions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.reward_distributions (
    id integer NOT NULL,
    proposal_id character varying NOT NULL,
    recipient_address character varying NOT NULL,
    amount bigint NOT NULL,
    distribution_time bigint NOT NULL,
    distribution_type character varying,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_52; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_52 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    distribution_type,
    count(*) AS distribution_count,
    sum(amount) AS total_amount
   FROM public.reward_distributions
  GROUP BY (public.time_bucket('1 day'::interval, "time")), distribution_type;


--
-- Name: social_graph_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.social_graph_events (
    id integer NOT NULL,
    event_type character varying NOT NULL,
    follower_address character varying NOT NULL,
    following_address character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    raw_event_data jsonb,
    event_id character varying
);


--
-- Name: TABLE social_graph_events; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.social_graph_events IS 'Records all follow/unfollow events for audit and history';


--
-- Name: COLUMN social_graph_events.event_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.social_graph_events.event_id IS 'Blockchain event ID in the format <digest>:<event_seq>';


--
-- Name: _direct_view_7; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_7 AS
 SELECT public.time_bucket('1 day'::interval, created_at) AS day,
    event_type,
    count(*) AS event_count
   FROM public.social_graph_events
  GROUP BY (public.time_bucket('1 day'::interval, created_at)), event_type;


--
-- Name: spt_price_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_price_history (
    id integer NOT NULL,
    pool_id character varying NOT NULL,
    price bigint NOT NULL,
    circulating_supply bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_73; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_73 AS
 SELECT pool_id,
    public.time_bucket('01:00:00'::interval, "time") AS bucket,
    public.first(price, "time") AS open,
    max(price) AS high,
    min(price) AS low,
    public.last(price, "time") AS close,
    public.last(circulating_supply, "time") AS circulating_supply
   FROM public.spt_price_history
  GROUP BY pool_id, (public.time_bucket('01:00:00'::interval, "time"));


--
-- Name: _direct_view_74; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_74 AS
 SELECT pool_id,
    public.time_bucket('1 day'::interval, "time") AS bucket,
    public.first(price, "time") AS open,
    max(price) AS high,
    min(price) AS low,
    public.last(price, "time") AS close,
    public.last(circulating_supply, "time") AS circulating_supply
   FROM public.spt_price_history
  GROUP BY pool_id, (public.time_bucket('1 day'::interval, "time"));


--
-- Name: profile_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profile_events (
    id integer NOT NULL,
    event_type character varying NOT NULL,
    profile_id character varying NOT NULL,
    event_data jsonb NOT NULL,
    event_id character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


--
-- Name: _direct_view_8; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_8 AS
 SELECT public.time_bucket('1 day'::interval, created_at) AS day,
    event_type,
    count(*) AS event_count
   FROM public.profile_events
  GROUP BY (public.time_bucket('1 day'::interval, created_at)), event_type;


--
-- Name: promotion_views; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.promotion_views (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    promotion_id character varying NOT NULL,
    viewer character varying NOT NULL,
    payment_amount bigint NOT NULL,
    view_duration bigint NOT NULL,
    platform_id character varying NOT NULL,
    "timestamp" bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: _direct_view_83; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_83 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    post_id,
    promotion_id,
    platform_id,
    count(*) AS view_count,
    sum(payment_amount) AS total_payments,
    avg(view_duration) AS avg_view_duration
   FROM public.promotion_views
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), post_id, promotion_id, platform_id;


--
-- Name: _direct_view_84; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_84 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS bucket,
    count(DISTINCT promotion_id) AS active_promotions,
    sum(payment_amount) AS total_spending,
    count(*) AS total_views,
    avg(payment_amount) AS avg_payment_per_view
   FROM public.promotion_views
  GROUP BY (public.time_bucket('1 day'::interval, "time"));


--
-- Name: platform_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.platform_events (
    id integer NOT NULL,
    event_type character varying NOT NULL,
    platform_id character varying NOT NULL,
    event_data jsonb NOT NULL,
    event_id character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    reasoning text
);


--
-- Name: _direct_view_9; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._direct_view_9 AS
 SELECT public.time_bucket('1 day'::interval, created_at) AS day,
    event_type,
    count(*) AS event_count
   FROM public.platform_events
  GROUP BY (public.time_bucket('1 day'::interval, created_at)), event_type;


--
-- Name: _hyper_1_36_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_1_36_chunk (
    CONSTRAINT constraint_22 CHECK (((created_at >= '2025-11-27 00:00:00'::timestamp without time zone) AND (created_at < '2025-12-04 00:00:00'::timestamp without time zone)))
)
INHERITS (public.social_graph_events);


--
-- Name: _hyper_1_40_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_1_40_chunk (
    CONSTRAINT constraint_25 CHECK (((created_at >= '2025-12-04 00:00:00'::timestamp without time zone) AND (created_at < '2025-12-11 00:00:00'::timestamp without time zone)))
)
INHERITS (public.social_graph_events);


--
-- Name: _hyper_3_35_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_3_35_chunk (
    CONSTRAINT constraint_21 CHECK (((created_at >= '1970-01-01 00:00:00'::timestamp without time zone) AND (created_at < '1970-01-08 00:00:00'::timestamp without time zone)))
)
INHERITS (public.profile_events);


--
-- Name: _hyper_5_38_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_5_38_chunk (
    CONSTRAINT constraint_23 CHECK (((created_at >= '2025-11-27 00:00:00'::timestamp without time zone) AND (created_at < '2025-12-04 00:00:00'::timestamp without time zone)))
)
INHERITS (public.platform_events);


--
-- Name: _hyper_5_39_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_5_39_chunk (
    CONSTRAINT constraint_24 CHECK (((created_at >= '2025-12-04 00:00:00'::timestamp without time zone) AND (created_at < '2025-12-11 00:00:00'::timestamp without time zone)))
)
INHERITS (public.platform_events);


--
-- Name: _materialized_hypertable_7; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_7 (
    day timestamp without time zone NOT NULL,
    event_type character varying,
    event_count bigint
);


--
-- Name: _hyper_7_4_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_7_4_chunk (
    CONSTRAINT constraint_3 CHECK (((day >= '2025-07-31 00:00:00'::timestamp without time zone) AND (day < '2025-10-09 00:00:00'::timestamp without time zone)))
)
INHERITS (_timescaledb_internal._materialized_hypertable_7);


--
-- Name: _hyper_7_7_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_7_7_chunk (
    CONSTRAINT constraint_6 CHECK (((day >= '2025-10-09 00:00:00'::timestamp without time zone) AND (day < '2025-12-18 00:00:00'::timestamp without time zone)))
)
INHERITS (_timescaledb_internal._materialized_hypertable_7);


--
-- Name: _materialized_hypertable_8; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_8 (
    day timestamp without time zone NOT NULL,
    event_type character varying,
    event_count bigint
);


--
-- Name: _hyper_8_12_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_8_12_chunk (
    CONSTRAINT constraint_10 CHECK (((day >= '2025-10-09 00:00:00'::timestamp without time zone) AND (day < '2025-12-18 00:00:00'::timestamp without time zone)))
)
INHERITS (_timescaledb_internal._materialized_hypertable_8);


--
-- Name: _materialized_hypertable_9; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_9 (
    day timestamp without time zone NOT NULL,
    event_type character varying,
    event_count bigint
);


--
-- Name: _hyper_9_8_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._hyper_9_8_chunk (
    CONSTRAINT constraint_7 CHECK (((day >= '2025-10-09 00:00:00'::timestamp without time zone) AND (day < '2025-12-18 00:00:00'::timestamp without time zone)))
)
INHERITS (_timescaledb_internal._materialized_hypertable_9);


--
-- Name: _materialized_hypertable_106; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_106 (
    day timestamp with time zone NOT NULL,
    badges_issued bigint,
    redirections_created integer,
    disputes_submitted integer,
    votes_cast integer
);


--
-- Name: _materialized_hypertable_107; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_107 (
    hour timestamp with time zone NOT NULL,
    badges_issued_hourly bigint,
    total_badges bigint
);


--
-- Name: _materialized_hypertable_112; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_112 (
    day timestamp with time zone NOT NULL,
    service_id character varying,
    profile_owner character varying,
    revenue_type character varying,
    daily_revenue numeric,
    transaction_count bigint
);


--
-- Name: _materialized_hypertable_113; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_113 (
    day timestamp with time zone NOT NULL,
    service_id character varying,
    new_subscriptions bigint,
    cancelled_subscriptions bigint,
    renewed_subscriptions bigint
);


--
-- Name: _materialized_hypertable_117; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_117 (
    hour timestamp with time zone NOT NULL,
    service_id character varying,
    active_subscriptions bigint,
    cancelled_subscriptions bigint,
    avg_renewal_count numeric,
    total_renewal_balance numeric
);


--
-- Name: _materialized_hypertable_118; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_118 (
    day timestamp with time zone NOT NULL,
    service_id character varying,
    daily_churn bigint,
    daily_new_subs bigint
);


--
-- Name: _materialized_hypertable_12; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_12 (
    day timestamp without time zone NOT NULL,
    checkpoints_processed bigint,
    total_events_processed bigint,
    avg_processing_duration_ms numeric,
    max_processing_duration_ms integer,
    min_processing_duration_ms integer
);


--
-- Name: _materialized_hypertable_123; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_123 (
    day timestamp with time zone NOT NULL,
    proposal_id character varying,
    total_anonymous_votes bigint,
    successfully_decrypted bigint,
    failed_decryptions bigint,
    anonymous_votes_for bigint,
    anonymous_votes_against bigint,
    pending_decryption bigint
);


--
-- Name: _materialized_hypertable_141; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_141 (
    day timestamp with time zone NOT NULL,
    mydata_id character varying,
    creator character varying,
    revenue_type character varying,
    daily_revenue numeric,
    transaction_count bigint
);


--
-- Name: _materialized_hypertable_142; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_142 (
    day timestamp with time zone NOT NULL,
    mydata_id character varying,
    access_type character varying,
    unique_users bigint,
    total_accesses bigint
);


--
-- Name: _materialized_hypertable_143; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_143 (
    hour timestamp with time zone NOT NULL,
    mydata_id character varying,
    unique_purchasers bigint,
    one_time_purchases bigint,
    subscriptions bigint,
    total_revenue numeric
);


--
-- Name: _materialized_hypertable_31; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_31 (
    bucket timestamp with time zone NOT NULL,
    object_id character varying,
    reaction_text character varying,
    reaction_count bigint
);


--
-- Name: _materialized_hypertable_32; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_32 (
    bucket timestamp with time zone NOT NULL,
    original_post_id character varying,
    repost_count bigint
);


--
-- Name: _materialized_hypertable_33; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_33 (
    bucket timestamp with time zone NOT NULL,
    object_id character varying,
    is_post boolean,
    total_amount numeric,
    tip_count bigint
);


--
-- Name: _materialized_hypertable_34; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_34 (
    bucket timestamp with time zone NOT NULL,
    posts_with_tips bigint,
    total_tip_amount numeric,
    total_tips bigint
);


--
-- Name: _materialized_hypertable_49; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_49 (
    day timestamp with time zone NOT NULL,
    registry_type smallint,
    target_address character varying,
    upvotes bigint,
    downvotes bigint,
    total_ratings bigint
);


--
-- Name: _materialized_hypertable_50; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_50 (
    hour timestamp with time zone NOT NULL,
    proposal_id character varying,
    approve_count bigint,
    reject_count bigint,
    total_votes bigint
);


--
-- Name: _materialized_hypertable_51; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_51 (
    hour timestamp with time zone NOT NULL,
    proposal_id character varying,
    approve_weight numeric,
    reject_weight numeric,
    total_votes bigint
);


--
-- Name: _materialized_hypertable_52; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_52 (
    day timestamp with time zone NOT NULL,
    distribution_type character varying,
    distribution_count bigint,
    total_amount numeric
);


--
-- Name: _materialized_hypertable_73; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_73 (
    pool_id character varying,
    bucket timestamp with time zone NOT NULL,
    open bigint,
    high bigint,
    low bigint,
    close bigint,
    circulating_supply bigint
);


--
-- Name: _materialized_hypertable_74; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_74 (
    pool_id character varying,
    bucket timestamp with time zone NOT NULL,
    open bigint,
    high bigint,
    low bigint,
    close bigint,
    circulating_supply bigint
);


--
-- Name: _materialized_hypertable_83; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_83 (
    bucket timestamp with time zone NOT NULL,
    post_id character varying,
    promotion_id character varying,
    platform_id character varying,
    view_count bigint,
    total_payments numeric,
    avg_view_duration numeric
);


--
-- Name: _materialized_hypertable_84; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal._materialized_hypertable_84 (
    bucket timestamp with time zone NOT NULL,
    active_promotions bigint,
    total_spending numeric,
    total_views bigint,
    avg_payment_per_view numeric
);


--
-- Name: _partial_view_106; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_106 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    count(*) FILTER (WHERE (NOT revoked)) AS badges_issued,
    0 AS redirections_created,
    0 AS disputes_submitted,
    0 AS votes_cast
   FROM public.poc_badges
  GROUP BY (public.time_bucket('1 day'::interval, "time"));


--
-- Name: _partial_view_107; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_107 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    count(*) FILTER (WHERE (NOT revoked)) AS badges_issued_hourly,
    count(*) AS total_badges
   FROM public.poc_badges
  GROUP BY (public.time_bucket('01:00:00'::interval, "time"));


--
-- Name: _partial_view_112; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_112 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    service_id,
    to_address AS profile_owner,
    revenue_type,
    sum(amount) AS daily_revenue,
    count(*) AS transaction_count
   FROM public.subscription_revenue
  GROUP BY (public.time_bucket('1 day'::interval, "time")), service_id, to_address, revenue_type;


--
-- Name: _partial_view_113; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_113 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    service_id,
    count(*) FILTER (WHERE (cancelled_at IS NULL)) AS new_subscriptions,
    count(*) FILTER (WHERE (cancelled_at IS NOT NULL)) AS cancelled_subscriptions,
    count(*) FILTER (WHERE (renewal_count > 0)) AS renewed_subscriptions
   FROM public.profile_subscriptions
  GROUP BY (public.time_bucket('1 day'::interval, "time")), service_id;


--
-- Name: _partial_view_117; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_117 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    service_id,
    count(*) FILTER (WHERE (cancelled_at IS NULL)) AS active_subscriptions,
    count(*) FILTER (WHERE (cancelled_at IS NOT NULL)) AS cancelled_subscriptions,
    avg(renewal_count) AS avg_renewal_count,
    sum(renewal_balance) AS total_renewal_balance
   FROM public.profile_subscriptions
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), service_id;


--
-- Name: _partial_view_118; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_118 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    service_id,
    count(*) FILTER (WHERE (((cancelled_at)::numeric >= EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time"))) AND ((cancelled_at)::numeric <= (EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time")) + (86400)::numeric)))) AS daily_churn,
    count(*) FILTER (WHERE (((created_at)::numeric >= EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time"))) AND ((created_at)::numeric <= (EXTRACT(epoch FROM public.time_bucket('1 day'::interval, "time")) + (86400)::numeric)))) AS daily_new_subs
   FROM public.profile_subscriptions
  GROUP BY (public.time_bucket('1 day'::interval, "time")), service_id;


--
-- Name: _partial_view_12; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_12 AS
 SELECT public.time_bucket('1 day'::interval, processing_start_time) AS day,
    count(*) AS checkpoints_processed,
    sum(events_processed) AS total_events_processed,
    avg(processing_duration_ms) AS avg_processing_duration_ms,
    max(processing_duration_ms) AS max_processing_duration_ms,
    min(processing_duration_ms) AS min_processing_duration_ms
   FROM public.checkpoint_processing
  WHERE (((processing_status)::text = 'completed'::text) AND (processing_duration_ms IS NOT NULL))
  GROUP BY (public.time_bucket('1 day'::interval, processing_start_time));


--
-- Name: _partial_view_123; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_123 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    proposal_id,
    count(*) AS total_anonymous_votes,
    count(*) FILTER (WHERE (decrypted = true)) AS successfully_decrypted,
    count(*) FILTER (WHERE (decryption_status = 2)) AS failed_decryptions,
    count(*) FILTER (WHERE (decrypted_vote = 1)) AS anonymous_votes_for,
    count(*) FILTER (WHERE (decrypted_vote = 0)) AS anonymous_votes_against,
    count(*) FILTER (WHERE (decryption_status = 0)) AS pending_decryption
   FROM public.anonymous_votes
  GROUP BY (public.time_bucket('1 day'::interval, "time")), proposal_id;


--
-- Name: _partial_view_141; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_141 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    mydata_id,
    to_address AS creator,
    revenue_type,
    sum(amount) AS daily_revenue,
    count(*) AS transaction_count
   FROM public.mydata_revenue
  GROUP BY (public.time_bucket('1 day'::interval, "time")), mydata_id, to_address, revenue_type;


--
-- Name: _partial_view_142; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_142 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    mydata_id,
    access_type,
    count(DISTINCT user_address) AS unique_users,
    count(*) AS total_accesses
   FROM public.mydata_access_logs
  GROUP BY (public.time_bucket('1 day'::interval, "time")), mydata_id, access_type;


--
-- Name: _partial_view_143; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_143 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    mydata_id,
    count(DISTINCT buyer) AS unique_purchasers,
    sum(
        CASE
            WHEN ((purchase_type)::text = 'one_time'::text) THEN 1
            ELSE 0
        END) AS one_time_purchases,
    sum(
        CASE
            WHEN ((purchase_type)::text = 'subscription'::text) THEN 1
            ELSE 0
        END) AS subscriptions,
    sum(price) AS total_revenue
   FROM public.mydata_purchases
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), mydata_id;


--
-- Name: _partial_view_31; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_31 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    object_id,
    reaction_text,
    count(*) AS reaction_count
   FROM public.reactions
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), object_id, reaction_text;


--
-- Name: _partial_view_32; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_32 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    original_post_id,
    count(*) AS repost_count
   FROM public.reposts
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), original_post_id;


--
-- Name: _partial_view_33; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_33 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    object_id,
    is_post,
    sum(amount) AS total_amount,
    count(*) AS tip_count
   FROM public.tips
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), object_id, is_post;


--
-- Name: _partial_view_34; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_34 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS bucket,
    count(DISTINCT object_id) AS posts_with_tips,
    sum(amount) AS total_tip_amount,
    count(*) AS total_tips
   FROM public.tips
  WHERE (is_post = true)
  GROUP BY (public.time_bucket('1 day'::interval, "time"));


--
-- Name: _partial_view_49; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_49 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    registry_type,
    target_address,
    sum(
        CASE
            WHEN upvote THEN 1
            ELSE 0
        END) AS upvotes,
    sum(
        CASE
            WHEN (NOT upvote) THEN 1
            ELSE 0
        END) AS downvotes,
    count(*) AS total_ratings
   FROM public.delegate_ratings
  GROUP BY (public.time_bucket('1 day'::interval, "time")), registry_type, target_address;


--
-- Name: _partial_view_50; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_50 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    proposal_id,
    sum(
        CASE
            WHEN approve THEN 1
            ELSE 0
        END) AS approve_count,
    sum(
        CASE
            WHEN (NOT approve) THEN 1
            ELSE 0
        END) AS reject_count,
    count(*) AS total_votes
   FROM public.delegate_votes
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), proposal_id;


--
-- Name: _partial_view_51; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_51 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS hour,
    proposal_id,
    sum(
        CASE
            WHEN approve THEN vote_weight
            ELSE (0)::bigint
        END) AS approve_weight,
    sum(
        CASE
            WHEN (NOT approve) THEN vote_weight
            ELSE (0)::bigint
        END) AS reject_weight,
    count(*) AS total_votes
   FROM public.community_votes
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), proposal_id;


--
-- Name: _partial_view_52; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_52 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS day,
    distribution_type,
    count(*) AS distribution_count,
    sum(amount) AS total_amount
   FROM public.reward_distributions
  GROUP BY (public.time_bucket('1 day'::interval, "time")), distribution_type;


--
-- Name: _partial_view_7; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_7 AS
 SELECT public.time_bucket('1 day'::interval, created_at) AS day,
    event_type,
    count(*) AS event_count
   FROM public.social_graph_events
  GROUP BY (public.time_bucket('1 day'::interval, created_at)), event_type;


--
-- Name: _partial_view_73; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_73 AS
 SELECT pool_id,
    public.time_bucket('01:00:00'::interval, "time") AS bucket,
    public.first(price, "time") AS open,
    max(price) AS high,
    min(price) AS low,
    public.last(price, "time") AS close,
    public.last(circulating_supply, "time") AS circulating_supply
   FROM public.spt_price_history
  GROUP BY pool_id, (public.time_bucket('01:00:00'::interval, "time"));


--
-- Name: _partial_view_74; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_74 AS
 SELECT pool_id,
    public.time_bucket('1 day'::interval, "time") AS bucket,
    public.first(price, "time") AS open,
    max(price) AS high,
    min(price) AS low,
    public.last(price, "time") AS close,
    public.last(circulating_supply, "time") AS circulating_supply
   FROM public.spt_price_history
  GROUP BY pool_id, (public.time_bucket('1 day'::interval, "time"));


--
-- Name: _partial_view_8; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_8 AS
 SELECT public.time_bucket('1 day'::interval, created_at) AS day,
    event_type,
    count(*) AS event_count
   FROM public.profile_events
  GROUP BY (public.time_bucket('1 day'::interval, created_at)), event_type;


--
-- Name: _partial_view_83; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_83 AS
 SELECT public.time_bucket('01:00:00'::interval, "time") AS bucket,
    post_id,
    promotion_id,
    platform_id,
    count(*) AS view_count,
    sum(payment_amount) AS total_payments,
    avg(view_duration) AS avg_view_duration
   FROM public.promotion_views
  GROUP BY (public.time_bucket('01:00:00'::interval, "time")), post_id, promotion_id, platform_id;


--
-- Name: _partial_view_84; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_84 AS
 SELECT public.time_bucket('1 day'::interval, "time") AS bucket,
    count(DISTINCT promotion_id) AS active_promotions,
    sum(payment_amount) AS total_spending,
    count(*) AS total_views,
    avg(payment_amount) AS avg_payment_per_view
   FROM public.promotion_views
  GROUP BY (public.time_bucket('1 day'::interval, "time"));


--
-- Name: _partial_view_9; Type: VIEW; Schema: _timescaledb_internal; Owner: -
--

CREATE VIEW _timescaledb_internal._partial_view_9 AS
 SELECT public.time_bucket('1 day'::interval, created_at) AS day,
    event_type,
    count(*) AS event_count
   FROM public.platform_events
  GROUP BY (public.time_bucket('1 day'::interval, created_at)), event_type;


--
-- Name: compress_hyper_4_37_chunk; Type: TABLE; Schema: _timescaledb_internal; Owner: -
--

CREATE TABLE _timescaledb_internal.compress_hyper_4_37_chunk (
    _ts_meta_count integer,
    event_type character varying,
    _ts_meta_min_1 integer,
    _ts_meta_max_1 integer,
    id _timescaledb_internal.compressed_data,
    _ts_meta_min_3 character varying,
    _ts_meta_max_3 character varying,
    profile_id _timescaledb_internal.compressed_data,
    event_data _timescaledb_internal.compressed_data,
    _ts_meta_min_4 character varying,
    _ts_meta_max_4 character varying,
    event_id _timescaledb_internal.compressed_data,
    _ts_meta_min_2 timestamp without time zone,
    _ts_meta_max_2 timestamp without time zone,
    created_at _timescaledb_internal.compressed_data,
    updated_at _timescaledb_internal.compressed_data
)
WITH (toast_tuple_target='128');
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_count SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN event_type SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_min_1 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_max_1 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN id SET STATISTICS 0;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_min_3 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_min_3 SET STORAGE PLAIN;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_max_3 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_max_3 SET STORAGE PLAIN;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN profile_id SET STATISTICS 0;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN profile_id SET STORAGE EXTENDED;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN event_data SET STATISTICS 0;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN event_data SET STORAGE EXTENDED;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_min_4 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_min_4 SET STORAGE PLAIN;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_max_4 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_max_4 SET STORAGE PLAIN;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN event_id SET STATISTICS 0;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN event_id SET STORAGE EXTENDED;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_min_2 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN _ts_meta_max_2 SET STATISTICS 1000;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN created_at SET STATISTICS 0;
ALTER TABLE ONLY _timescaledb_internal.compress_hyper_4_37_chunk ALTER COLUMN updated_at SET STATISTICS 0;


--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


--
-- Name: my_ip; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.my_ip (
    id integer NOT NULL,
    license_id text NOT NULL,
    name text NOT NULL,
    description text,
    creator text NOT NULL,
    creation_time bigint NOT NULL,
    license_type smallint NOT NULL,
    permission_flags bigint NOT NULL,
    license_state smallint NOT NULL,
    proof_of_creativity_id text,
    custom_license_uri text,
    revenue_recipient text,
    transferable boolean DEFAULT false NOT NULL,
    expires_at bigint,
    version integer DEFAULT 1 NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: active_licenses; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.active_licenses AS
 SELECT id,
    license_id,
    name,
    description,
    creator,
    creation_time,
    license_type,
    permission_flags,
    license_state,
    proof_of_creativity_id,
    custom_license_uri,
    revenue_recipient,
    transferable,
    expires_at,
    version,
    "time",
    transaction_id,
    ((permission_flags & ((1 << 0))::bigint) > 0) AS commercial_use,
    ((permission_flags & ((1 << 1))::bigint) > 0) AS derivatives_allowed,
    ((permission_flags & ((1 << 2))::bigint) > 0) AS public_license,
    ((permission_flags & ((1 << 3))::bigint) > 0) AS authority_required,
    ((permission_flags & ((1 << 4))::bigint) > 0) AS share_alike,
    ((permission_flags & ((1 << 5))::bigint) > 0) AS require_attribution,
    ((permission_flags & ((1 << 6))::bigint) > 0) AS revenue_redirect,
    ((permission_flags & ((1 << 10))::bigint) > 0) AS allow_comments,
    ((permission_flags & ((1 << 11))::bigint) > 0) AS allow_reactions,
    ((permission_flags & ((1 << 12))::bigint) > 0) AS allow_reposts,
    ((permission_flags & ((1 << 13))::bigint) > 0) AS allow_quotes,
    ((permission_flags & ((1 << 14))::bigint) > 0) AS allow_tips
   FROM public.my_ip l
  WHERE ((license_state = 0) AND ((expires_at IS NULL) OR ((expires_at)::numeric > EXTRACT(epoch FROM now()))));


--
-- Name: mydata_data; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.mydata_data (
    mydata_id character varying NOT NULL,
    owner character varying NOT NULL,
    media_type character varying NOT NULL,
    tags jsonb DEFAULT '[]'::jsonb NOT NULL,
    platform_id character varying,
    timestamp_start bigint NOT NULL,
    timestamp_end bigint,
    created_at bigint NOT NULL,
    last_updated bigint NOT NULL,
    one_time_price bigint,
    subscription_price bigint,
    subscription_duration_days bigint DEFAULT 30 NOT NULL,
    geographic_region character varying,
    data_quality character varying,
    sample_size bigint,
    collection_method character varying,
    is_updating boolean DEFAULT false NOT NULL,
    update_frequency character varying,
    version bigint DEFAULT 1 NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT my_ip_data_data_quality_check CHECK (((data_quality)::text = ANY ((ARRAY['high'::character varying, 'medium'::character varying, 'low'::character varying])::text[]))),
    CONSTRAINT my_ip_data_update_frequency_check CHECK (((update_frequency)::text = ANY ((ARRAY['hourly'::character varying, 'daily'::character varying, 'weekly'::character varying, 'monthly'::character varying, 'yearly'::character varying])::text[])))
);


--
-- Name: TABLE mydata_data; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mydata_data IS 'MyData marketplace entries with metadata and pricing';


--
-- Name: active_mydata; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.active_mydata AS
 SELECT mydata_id,
    owner,
    media_type,
    tags,
    platform_id,
    timestamp_start,
    timestamp_end,
    created_at,
    last_updated,
    one_time_price,
    subscription_price,
    subscription_duration_days,
    geographic_region,
    data_quality,
    sample_size,
    collection_method,
    is_updating,
    update_frequency,
    version,
    "time",
    transaction_id,
        CASE
            WHEN ((one_time_price IS NOT NULL) AND (subscription_price IS NOT NULL)) THEN 'both'::text
            WHEN (one_time_price IS NOT NULL) THEN 'one_time'::text
            WHEN (subscription_price IS NOT NULL) THEN 'subscription'::text
            ELSE 'free'::text
        END AS pricing_model,
        CASE
            WHEN ((timestamp_end IS NOT NULL) AND ((timestamp_end)::numeric < EXTRACT(epoch FROM now()))) THEN false
            ELSE true
        END AS is_current
   FROM public.mydata_data d;


--
-- Name: posts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.posts (
    id character varying NOT NULL,
    post_id character varying NOT NULL,
    owner character varying NOT NULL,
    profile_id character varying NOT NULL,
    content text NOT NULL,
    media_urls jsonb,
    mentions jsonb,
    metadata_json jsonb,
    post_type character varying NOT NULL,
    parent_post_id character varying,
    created_at bigint NOT NULL,
    updated_at bigint,
    deleted_at bigint,
    reaction_count bigint DEFAULT 0,
    comment_count bigint DEFAULT 0,
    repost_count bigint DEFAULT 0,
    tips_received bigint DEFAULT 0,
    removed_from_platform boolean DEFAULT false,
    removed_by character varying,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    promotion_id character varying,
    poc_badge_id character varying,
    revenue_redirect_to character varying,
    revenue_redirect_percentage bigint,
    requires_subscription boolean DEFAULT false,
    subscription_service_id character varying,
    subscription_price bigint,
    encrypted_content_hash character varying,
    mydata_id character varying,
    revenue_recipient character varying,
    auto_pool_disabled boolean DEFAULT false NOT NULL,
    my_ip_id text
);


--
-- Name: COLUMN posts.auto_pool_disabled; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.auto_pool_disabled IS 'Whether auto pool creation is disabled for this post. FALSE means auto pool is enabled (default), TRUE means disabled.';


--
-- Name: promoted_posts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.promoted_posts (
    id integer NOT NULL,
    promotion_id character varying NOT NULL,
    post_id character varying NOT NULL,
    owner character varying NOT NULL,
    profile_id character varying NOT NULL,
    payment_per_view bigint NOT NULL,
    total_budget bigint NOT NULL,
    remaining_budget bigint DEFAULT 0 NOT NULL,
    active boolean DEFAULT false NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: active_promoted_posts; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.active_promoted_posts AS
 SELECT pp.promotion_id,
    pp.post_id,
    pp.owner,
    pp.profile_id,
    pp.payment_per_view,
    pp.total_budget,
    pp.remaining_budget,
    pp.created_at,
    p.content,
    p.media_urls,
    p.post_type,
    COALESCE(pv_stats.view_count, (0)::bigint) AS total_views,
    COALESCE(pv_stats.total_paid, (0)::numeric) AS total_paid,
    COALESCE(pv_stats.avg_view_duration, (0)::numeric) AS avg_view_duration
   FROM ((public.promoted_posts pp
     JOIN public.posts p ON (((pp.post_id)::text = (p.post_id)::text)))
     LEFT JOIN ( SELECT promotion_views.promotion_id,
            count(*) AS view_count,
            sum(promotion_views.payment_amount) AS total_paid,
            avg(promotion_views.view_duration) AS avg_view_duration
           FROM public.promotion_views
          GROUP BY promotion_views.promotion_id) pv_stats ON (((pp.promotion_id)::text = (pv_stats.promotion_id)::text)))
  WHERE ((pp.active = true) AND (p.deleted_at IS NULL) AND (p.removed_from_platform = false));


--
-- Name: spt_reservation_pools; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_reservation_pools (
    id integer NOT NULL,
    pool_id character varying NOT NULL,
    associated_id character varying NOT NULL,
    token_type smallint NOT NULL,
    owner character varying NOT NULL,
    total_reserved bigint DEFAULT 0 NOT NULL,
    required_threshold bigint NOT NULL,
    status character varying DEFAULT 'active'::character varying NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spt_reservations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_reservations (
    id integer NOT NULL,
    pool_id character varying NOT NULL,
    reservatior_address character varying NOT NULL,
    amount bigint NOT NULL,
    reserved_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: active_reservation_pools; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.active_reservation_pools AS
 SELECT sp.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    sp.total_reserved,
    sp.required_threshold,
    sp.status,
    sp.created_at,
    (sp.total_reserved >= sp.required_threshold) AS threshold_met,
    count(s.id) AS reservatior_count,
    COALESCE(max(s."time"), sp."time") AS last_activity
   FROM (public.spt_reservation_pools sp
     LEFT JOIN public.spt_reservations s ON (((sp.pool_id)::text = (s.pool_id)::text)))
  WHERE (sp."time" = ( SELECT max(sub."time") AS max
           FROM public.spt_reservation_pools sub
          WHERE ((sub.pool_id)::text = (sp.pool_id)::text)))
  GROUP BY sp.pool_id, sp.associated_id, sp.token_type, sp.owner, sp.total_reserved, sp.required_threshold, sp.status, sp.created_at, sp."time"
  ORDER BY sp.total_reserved DESC;


--
-- Name: social_proof_token_pools; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.social_proof_token_pools (
    id integer NOT NULL,
    pool_id character varying NOT NULL,
    token_type smallint NOT NULL,
    owner character varying NOT NULL,
    associated_id character varying NOT NULL,
    symbol character varying NOT NULL,
    name character varying NOT NULL,
    circulating_supply bigint NOT NULL,
    base_price bigint NOT NULL,
    quadratic_coefficient bigint NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: active_token_pools; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.active_token_pools AS
 SELECT p.id,
    p.pool_id,
    p.token_type,
    p.owner,
    p.associated_id,
    p.symbol,
    p.name,
    p.circulating_supply,
    p.base_price,
    p.quadratic_coefficient,
    p.created_at,
    p."time",
    p.transaction_id,
    ph.price AS current_price
   FROM (public.social_proof_token_pools p
     JOIN ( SELECT DISTINCT ON (spt_price_history.pool_id) spt_price_history.pool_id,
            spt_price_history.price
           FROM public.spt_price_history
          ORDER BY spt_price_history.pool_id, spt_price_history."time" DESC) ph ON (((p.pool_id)::text = (ph.pool_id)::text)));


--
-- Name: anonymous_votes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.anonymous_votes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: anonymous_votes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.anonymous_votes_id_seq OWNED BY public.anonymous_votes.id;


--
-- Name: anonymous_voting_daily_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.anonymous_voting_daily_stats AS
 SELECT day,
    proposal_id,
    total_anonymous_votes,
    successfully_decrypted,
    failed_decryptions,
    anonymous_votes_for,
    anonymous_votes_against,
    pending_decryption
   FROM _timescaledb_internal._materialized_hypertable_123;


--
-- Name: blocked_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.blocked_events (
    id integer NOT NULL,
    event_id character varying,
    event_type character varying NOT NULL,
    blocker_address character varying NOT NULL,
    blocked_address character varying,
    block_list_address character varying,
    raw_event_data jsonb,
    processed_at timestamp without time zone DEFAULT now() NOT NULL,
    created_at timestamp without time zone NOT NULL
);


--
-- Name: TABLE blocked_events; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.blocked_events IS 'Complete audit trail of all blocking/unblocking events from blockchain';


--
-- Name: COLUMN blocked_events.event_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_events.event_id IS 'Unique blockchain event identifier for deduplication';


--
-- Name: COLUMN blocked_events.event_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_events.event_type IS 'Type of blocking event: block, unblock, or block_list_created';


--
-- Name: COLUMN blocked_events.blocked_address; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_events.blocked_address IS 'NULL for block_list_created events';


--
-- Name: blocked_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.blocked_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: blocked_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.blocked_events_id_seq OWNED BY public.blocked_events.id;


--
-- Name: blocked_profiles; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.blocked_profiles (
    id integer NOT NULL,
    blocker_address character varying NOT NULL,
    blocked_address character varying NOT NULL,
    block_list_address character varying,
    blocked_profile_id character varying,
    blocked_username character varying NOT NULL,
    blocked_display_name character varying,
    blocked_profile_photo character varying,
    first_blocked_at timestamp without time zone NOT NULL,
    last_blocked_at timestamp without time zone NOT NULL,
    total_block_count integer DEFAULT 1 NOT NULL
);


--
-- Name: TABLE blocked_profiles; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.blocked_profiles IS 'Current blocking relationships - represents live blocking state';


--
-- Name: COLUMN blocked_profiles.block_list_address; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.block_list_address IS 'Reference to the blockchain block list object';


--
-- Name: COLUMN blocked_profiles.blocked_profile_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.blocked_profile_id IS 'Blockchain profile ID of the blocked user';


--
-- Name: COLUMN blocked_profiles.blocked_username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.blocked_username IS 'Username of the blocked user for fast API responses';


--
-- Name: COLUMN blocked_profiles.blocked_display_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.blocked_display_name IS 'Display name of the blocked user for fast API responses';


--
-- Name: COLUMN blocked_profiles.blocked_profile_photo; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.blocked_profile_photo IS 'Profile photo URL of the blocked user for fast API responses';


--
-- Name: COLUMN blocked_profiles.first_blocked_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.first_blocked_at IS 'Timestamp when this profile was first blocked by this blocker';


--
-- Name: COLUMN blocked_profiles.last_blocked_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.last_blocked_at IS 'Most recent blocking event timestamp';


--
-- Name: COLUMN blocked_profiles.total_block_count; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.blocked_profiles.total_block_count IS 'Number of times this profile has been blocked by this blocker';


--
-- Name: blocked_profiles_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.blocked_profiles_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: blocked_profiles_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.blocked_profiles_id_seq OWNED BY public.blocked_profiles.id;


--
-- Name: checkpoint_daily_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.checkpoint_daily_stats AS
 SELECT _materialized_hypertable_12.day,
    _materialized_hypertable_12.checkpoints_processed,
    _materialized_hypertable_12.total_events_processed,
    _materialized_hypertable_12.avg_processing_duration_ms,
    _materialized_hypertable_12.max_processing_duration_ms,
    _materialized_hypertable_12.min_processing_duration_ms
   FROM _timescaledb_internal._materialized_hypertable_12
  WHERE (_materialized_hypertable_12.day < COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(12)), '-infinity'::timestamp without time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, checkpoint_processing.processing_start_time) AS day,
    count(*) AS checkpoints_processed,
    sum(checkpoint_processing.events_processed) AS total_events_processed,
    avg(checkpoint_processing.processing_duration_ms) AS avg_processing_duration_ms,
    max(checkpoint_processing.processing_duration_ms) AS max_processing_duration_ms,
    min(checkpoint_processing.processing_duration_ms) AS min_processing_duration_ms
   FROM public.checkpoint_processing
  WHERE ((((checkpoint_processing.processing_status)::text = 'completed'::text) AND (checkpoint_processing.processing_duration_ms IS NOT NULL)) AND (checkpoint_processing.processing_start_time >= COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(12)), '-infinity'::timestamp without time zone)))
  GROUP BY (public.time_bucket('1 day'::interval, checkpoint_processing.processing_start_time));


--
-- Name: checkpoint_processing_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.checkpoint_processing_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: checkpoint_processing_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.checkpoint_processing_id_seq OWNED BY public.checkpoint_processing.id;


--
-- Name: comments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.comments (
    id character varying NOT NULL,
    comment_id character varying NOT NULL,
    post_id character varying NOT NULL,
    parent_comment_id character varying,
    owner character varying NOT NULL,
    profile_id character varying NOT NULL,
    content text NOT NULL,
    media_urls jsonb,
    mentions jsonb,
    metadata_json jsonb,
    created_at bigint NOT NULL,
    updated_at bigint,
    deleted_at bigint,
    reaction_count bigint DEFAULT 0,
    comment_count bigint DEFAULT 0,
    repost_count bigint DEFAULT 0,
    tips_received bigint DEFAULT 0,
    removed_from_platform boolean DEFAULT false,
    removed_by character varying,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: community_votes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.community_votes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: community_votes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.community_votes_id_seq OWNED BY public.community_votes.id;


--
-- Name: community_voting_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.community_voting_hourly AS
 SELECT _materialized_hypertable_51.hour,
    _materialized_hypertable_51.proposal_id,
    _materialized_hypertable_51.approve_weight,
    _materialized_hypertable_51.reject_weight,
    _materialized_hypertable_51.total_votes
   FROM _timescaledb_internal._materialized_hypertable_51
  WHERE (_materialized_hypertable_51.hour < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(51)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('01:00:00'::interval, community_votes."time") AS hour,
    community_votes.proposal_id,
    sum(
        CASE
            WHEN community_votes.approve THEN community_votes.vote_weight
            ELSE (0)::bigint
        END) AS approve_weight,
    sum(
        CASE
            WHEN (NOT community_votes.approve) THEN community_votes.vote_weight
            ELSE (0)::bigint
        END) AS reject_weight,
    count(*) AS total_votes
   FROM public.community_votes
  WHERE (community_votes."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(51)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('01:00:00'::interval, community_votes."time")), community_votes.proposal_id;


--
-- Name: continuous_aggregate_refresh_status; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.continuous_aggregate_refresh_status (
    view_name text NOT NULL,
    last_manual_refresh timestamp without time zone DEFAULT now(),
    notes text
);


--
-- Name: creator_mydata_revenue_summary; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.creator_mydata_revenue_summary AS
 SELECT d.owner AS creator,
    count(DISTINCT d.mydata_id) AS data_entries,
    sum(r.amount) AS total_revenue,
    count(DISTINCT r.from_address) AS unique_customers,
    max(r."time") AS last_revenue
   FROM (public.mydata_data d
     LEFT JOIN public.mydata_revenue r ON (((d.mydata_id)::text = (r.mydata_id)::text)))
  GROUP BY d.owner
  ORDER BY (sum(r.amount)) DESC NULLS LAST;


--
-- Name: my_ip_revenue; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.my_ip_revenue (
    id integer NOT NULL,
    license_id text NOT NULL,
    post_id text,
    from_address text NOT NULL,
    to_address text NOT NULL,
    amount bigint NOT NULL,
    revenue_type text NOT NULL,
    revenue_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: daily_license_revenue; Type: MATERIALIZED VIEW; Schema: public; Owner: -
--

DROP MATERIALIZED VIEW IF EXISTS public.daily_license_revenue;
CREATE MATERIALIZED VIEW public.daily_license_revenue AS
 SELECT public.time_bucket('1 day'::interval, "time") AS bucket,
    license_id,
    revenue_type,
    sum(amount) AS total_amount,
    count(*) AS transaction_count
   FROM public.my_ip_revenue
  GROUP BY (public.time_bucket('1 day'::interval, "time")), license_id, revenue_type
  WITH NO DATA;


--
-- Name: delegates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.delegates (
    id integer NOT NULL,
    address character varying NOT NULL,
    profile_id character varying NOT NULL,
    registry_type smallint NOT NULL,
    upvotes bigint DEFAULT 0 NOT NULL,
    downvotes bigint DEFAULT 0 NOT NULL,
    proposals_reviewed bigint DEFAULT 0 NOT NULL,
    proposals_submitted bigint DEFAULT 0 NOT NULL,
    sided_winning_proposals bigint DEFAULT 0 NOT NULL,
    sided_losing_proposals bigint DEFAULT 0 NOT NULL,
    term_start bigint NOT NULL,
    term_end bigint NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    created_at bigint NOT NULL,
    updated_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: delegate_performance; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.delegate_performance AS
 SELECT d.address,
    d.profile_id,
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
            WHEN (d.proposals_reviewed > 0) THEN ((d.sided_winning_proposals)::double precision / (NULLIF(d.proposals_reviewed, 0))::double precision)
            ELSE NULL::double precision
        END AS winning_rate,
    count(DISTINCT dv.proposal_id) AS recent_votes,
    sum(
        CASE
            WHEN dv.approve THEN 1
            ELSE 0
        END) AS recent_approvals,
    sum(
        CASE
            WHEN (NOT dv.approve) THEN 1
            ELSE 0
        END) AS recent_rejections
   FROM (public.delegates d
     LEFT JOIN public.delegate_votes dv ON ((((d.address)::text = (dv.delegate_address)::text) AND (dv."time" > (now() - '30 days'::interval)))))
  GROUP BY d.id, d.address, d.profile_id, d.registry_type, d.upvotes, d.downvotes, d.proposals_reviewed, d.proposals_submitted, d.sided_winning_proposals, d.sided_losing_proposals, d.term_start, d.term_end, d.is_active;


--
-- Name: delegate_ratings_daily; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.delegate_ratings_daily AS
 SELECT _materialized_hypertable_49.day,
    _materialized_hypertable_49.registry_type,
    _materialized_hypertable_49.target_address,
    _materialized_hypertable_49.upvotes,
    _materialized_hypertable_49.downvotes,
    _materialized_hypertable_49.total_ratings
   FROM _timescaledb_internal._materialized_hypertable_49
  WHERE (_materialized_hypertable_49.day < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(49)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, delegate_ratings."time") AS day,
    delegate_ratings.registry_type,
    delegate_ratings.target_address,
    sum(
        CASE
            WHEN delegate_ratings.upvote THEN 1
            ELSE 0
        END) AS upvotes,
    sum(
        CASE
            WHEN (NOT delegate_ratings.upvote) THEN 1
            ELSE 0
        END) AS downvotes,
    count(*) AS total_ratings
   FROM public.delegate_ratings
  WHERE (delegate_ratings."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(49)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('1 day'::interval, delegate_ratings."time")), delegate_ratings.registry_type, delegate_ratings.target_address;


--
-- Name: delegate_ratings_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.delegate_ratings_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: delegate_ratings_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.delegate_ratings_id_seq OWNED BY public.delegate_ratings.id;


--
-- Name: delegate_votes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.delegate_votes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: delegate_votes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.delegate_votes_id_seq OWNED BY public.delegate_votes.id;


--
-- Name: delegate_voting_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.delegate_voting_hourly AS
 SELECT _materialized_hypertable_50.hour,
    _materialized_hypertable_50.proposal_id,
    _materialized_hypertable_50.approve_count,
    _materialized_hypertable_50.reject_count,
    _materialized_hypertable_50.total_votes
   FROM _timescaledb_internal._materialized_hypertable_50
  WHERE (_materialized_hypertable_50.hour < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(50)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('01:00:00'::interval, delegate_votes."time") AS hour,
    delegate_votes.proposal_id,
    sum(
        CASE
            WHEN delegate_votes.approve THEN 1
            ELSE 0
        END) AS approve_count,
    sum(
        CASE
            WHEN (NOT delegate_votes.approve) THEN 1
            ELSE 0
        END) AS reject_count,
    count(*) AS total_votes
   FROM public.delegate_votes
  WHERE (delegate_votes."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(50)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('01:00:00'::interval, delegate_votes."time")), delegate_votes.proposal_id;


--
-- Name: delegates_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.delegates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: delegates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.delegates_id_seq OWNED BY public.delegates.id;


--
-- Name: governance_registries; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.governance_registries (
    id integer NOT NULL,
    registry_type smallint NOT NULL,
    delegate_count bigint NOT NULL,
    delegate_term_epochs bigint NOT NULL,
    proposal_submission_cost bigint NOT NULL,
    min_on_chain_age_days bigint NOT NULL,
    max_votes_per_user bigint NOT NULL,
    quadratic_base_cost bigint NOT NULL,
    voting_period_epochs bigint NOT NULL,
    quorum_votes bigint NOT NULL,
    updated_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: governance_registries_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.governance_registries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: governance_registries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.governance_registries_id_seq OWNED BY public.governance_registries.id;


--
-- Name: nominated_delegates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.nominated_delegates (
    id integer NOT NULL,
    address character varying NOT NULL,
    profile_id character varying NOT NULL,
    registry_type smallint NOT NULL,
    upvotes bigint DEFAULT 0 NOT NULL,
    downvotes bigint DEFAULT 0 NOT NULL,
    scheduled_term_start_epoch bigint NOT NULL,
    nomination_time bigint NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: proposals; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.proposals (
    id character varying NOT NULL,
    title character varying NOT NULL,
    description text NOT NULL,
    proposal_type smallint NOT NULL,
    reference_id character varying,
    metadata_json jsonb,
    submitter character varying NOT NULL,
    submission_time bigint NOT NULL,
    delegate_approval_count bigint DEFAULT 0 NOT NULL,
    delegate_rejection_count bigint DEFAULT 0 NOT NULL,
    community_votes_for bigint DEFAULT 0 NOT NULL,
    community_votes_against bigint DEFAULT 0 NOT NULL,
    status smallint NOT NULL,
    voting_start_time bigint,
    voting_end_time bigint,
    reward_pool bigint NOT NULL,
    implemented_description text,
    implementation_time bigint,
    rescind_time bigint,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    anonymous_votes_for bigint DEFAULT 0,
    anonymous_votes_against bigint DEFAULT 0,
    anonymous_voters_count bigint DEFAULT 0,
    pending_anonymous_decryption boolean DEFAULT false,
    anonymous_decryption_completed_at bigint
);


--
-- Name: governance_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.governance_stats AS
 SELECT g.registry_type,
    count(DISTINCT d.id) AS active_delegates,
    count(DISTINCT n.id) AS pending_nominees,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 0)) AS submitted_proposals,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 1)) AS in_review_proposals,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 2)) AS voting_proposals,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 3)) AS approved_proposals,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 4)) AS rejected_proposals,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 5)) AS implemented_proposals,
    count(DISTINCT p.id) FILTER (WHERE (p.status = 6)) AS rescinded_proposals
   FROM (((public.governance_registries g
     LEFT JOIN public.delegates d ON (((g.registry_type = d.registry_type) AND (d.is_active = true))))
     LEFT JOIN public.nominated_delegates n ON (((g.registry_type = n.registry_type) AND (n.status = 0))))
     LEFT JOIN public.proposals p ON ((g.registry_type = p.proposal_type)))
  GROUP BY g.registry_type;


--
-- Name: indexer_checkpoint_state; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.indexer_checkpoint_state (
    id integer NOT NULL,
    last_processed_checkpoint bigint NOT NULL,
    last_processed_timestamp timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: indexer_checkpoint_state_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.indexer_checkpoint_state_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indexer_checkpoint_state_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indexer_checkpoint_state_id_seq OWNED BY public.indexer_checkpoint_state.id;


--
-- Name: indexer_progress; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.indexer_progress (
    id text NOT NULL,
    last_checkpoint_processed bigint DEFAULT 0 NOT NULL,
    last_processed_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: post_prediction_config; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.post_prediction_config (
    id integer NOT NULL,
    updated_by text NOT NULL,
    predictions_enabled boolean DEFAULT true NOT NULL,
    fee_bps bigint DEFAULT 0 NOT NULL,
    treasury text NOT NULL,
    updated_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: TABLE post_prediction_config; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.post_prediction_config IS 'Tracks global prediction configuration changes over time. Each row represents a configuration update.';


--
-- Name: COLUMN post_prediction_config.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.post_prediction_config.updated_by IS 'Address of the account that updated the configuration';


--
-- Name: COLUMN post_prediction_config.predictions_enabled; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.post_prediction_config.predictions_enabled IS 'Whether predictions are globally enabled';


--
-- Name: COLUMN post_prediction_config.fee_bps; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.post_prediction_config.fee_bps IS 'Prediction fee in basis points (1 bps = 0.01%)';


--
-- Name: COLUMN post_prediction_config.treasury; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.post_prediction_config.treasury IS 'Treasury address that receives prediction fees';


--
-- Name: COLUMN post_prediction_config.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.post_prediction_config.updated_at IS 'Unix timestamp in milliseconds when the configuration was updated';


--
-- Name: latest_post_prediction_config; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.latest_post_prediction_config AS
 SELECT DISTINCT ON (updated_by) updated_by,
    predictions_enabled,
    fee_bps,
    treasury,
    updated_at,
    "time",
    transaction_id
   FROM public.post_prediction_config
  ORDER BY updated_by, "time" DESC;


--
-- Name: my_ip_access_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_access_logs_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_access_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_access_logs_id_seq OWNED BY public.mydata_access_logs.id;


--
-- Name: my_ip_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.my_ip_events (
    id integer NOT NULL,
    event_type text NOT NULL,
    license_id text NOT NULL,
    event_data jsonb NOT NULL,
    created_by text NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: my_ip_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_events_id_seq OWNED BY public.my_ip_events.id;


--
-- Name: my_ip_grants; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.my_ip_grants (
    id integer NOT NULL,
    license_id text NOT NULL,
    grantor text NOT NULL,
    grantee text NOT NULL,
    grant_type text NOT NULL,
    payment_amount bigint DEFAULT 0 NOT NULL,
    payment_token text,
    grant_time bigint NOT NULL,
    expiration_time bigint,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: my_ip_grants_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_grants_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_grants_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_grants_id_seq OWNED BY public.my_ip_grants.id;


--
-- Name: my_ip_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_id_seq OWNED BY public.my_ip.id;


--
-- Name: my_ip_permissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.my_ip_permissions (
    id integer NOT NULL,
    permission_name text NOT NULL,
    bit_position integer NOT NULL,
    description text NOT NULL
);


--
-- Name: my_ip_permissions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_permissions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_permissions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_permissions_id_seq OWNED BY public.my_ip_permissions.id;


--
-- Name: my_ip_purchases_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_purchases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_purchases_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_purchases_id_seq OWNED BY public.mydata_purchases.id;


--
-- Name: my_ip_revenue_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_revenue_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_revenue_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_revenue_id_seq OWNED BY public.mydata_revenue.id;


--
-- Name: my_ip_revenue_id_seq1; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_revenue_id_seq1
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_revenue_id_seq1; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_revenue_id_seq1 OWNED BY public.my_ip_revenue.id;


--
-- Name: mydata_subscriptions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.mydata_subscriptions (
    id integer NOT NULL,
    mydata_id character varying NOT NULL,
    subscriber character varying NOT NULL,
    subscription_start bigint NOT NULL,
    subscription_end bigint NOT NULL,
    price bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: TABLE mydata_subscriptions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mydata_subscriptions IS 'Active subscription tracking with expiry times (TimescaleDB)';


--
-- Name: my_ip_subscriptions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.my_ip_subscriptions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: my_ip_subscriptions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.my_ip_subscriptions_id_seq OWNED BY public.mydata_subscriptions.id;


--
-- Name: mydata_daily_access; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.mydata_daily_access AS
 SELECT day,
    mydata_id,
    access_type,
    unique_users,
    total_accesses
   FROM _timescaledb_internal._materialized_hypertable_142;


--
-- Name: mydata_daily_revenue; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.mydata_daily_revenue AS
 SELECT day,
    mydata_id,
    creator,
    revenue_type,
    daily_revenue,
    transaction_count
   FROM _timescaledb_internal._materialized_hypertable_141;


--
-- Name: mydata_popular_data; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.mydata_popular_data AS
 SELECT hour,
    mydata_id,
    unique_purchasers,
    one_time_purchases,
    subscriptions,
    total_revenue
   FROM _timescaledb_internal._materialized_hypertable_143;


--
-- Name: mydata_registry; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.mydata_registry (
    ip_id text NOT NULL,
    owner text NOT NULL,
    registered_at bigint NOT NULL,
    unregistered_at bigint,
    is_active boolean DEFAULT true NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: TABLE mydata_registry; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mydata_registry IS 'Tracks MyData IP ID to owner mappings. Records registration and unregistration events from the blockchain.';


--
-- Name: COLUMN mydata_registry.ip_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry.ip_id IS 'The IP ID (address) of the MyData object';


--
-- Name: COLUMN mydata_registry.owner; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry.owner IS 'The owner address of the MyData';


--
-- Name: COLUMN mydata_registry.registered_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry.registered_at IS 'Unix timestamp in milliseconds when the MyData was registered';


--
-- Name: COLUMN mydata_registry.unregistered_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry.unregistered_at IS 'Unix timestamp in milliseconds when the MyData was unregistered (NULL if still active)';


--
-- Name: COLUMN mydata_registry.is_active; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry.is_active IS 'Whether the MyData is currently registered (true) or unregistered (false)';


--
-- Name: COLUMN mydata_registry."time"; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry."time" IS 'PostgreSQL timestamp for the record (derived from registered_at)';


--
-- Name: COLUMN mydata_registry.transaction_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.mydata_registry.transaction_id IS 'Transaction ID from the blockchain event';


--
-- Name: nominated_delegates_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.nominated_delegates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: nominated_delegates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.nominated_delegates_id_seq OWNED BY public.nominated_delegates.id;


--
-- Name: platform_blocked_profiles; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.platform_blocked_profiles (
    id integer NOT NULL,
    platform_id character varying NOT NULL,
    profile_id character varying NOT NULL,
    blocked_by character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE platform_blocked_profiles; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.platform_blocked_profiles IS 'Records of profiles blocked by platforms. Records are deleted when a profile is unblocked.';


--
-- Name: platform_blocked_profiles_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.platform_blocked_profiles_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: platform_blocked_profiles_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.platform_blocked_profiles_id_seq OWNED BY public.platform_blocked_profiles.id;


--
-- Name: platform_daily_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.platform_daily_stats AS
 SELECT _materialized_hypertable_9.day,
    _materialized_hypertable_9.event_type,
    _materialized_hypertable_9.event_count
   FROM _timescaledb_internal._materialized_hypertable_9
  WHERE (_materialized_hypertable_9.day < COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(9)), '-infinity'::timestamp without time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, platform_events.created_at) AS day,
    platform_events.event_type,
    count(*) AS event_count
   FROM public.platform_events
  WHERE (platform_events.created_at >= COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(9)), '-infinity'::timestamp without time zone))
  GROUP BY (public.time_bucket('1 day'::interval, platform_events.created_at)), platform_events.event_type;


--
-- Name: platform_delivery_config; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.platform_delivery_config (
    id bigint NOT NULL,
    platform_id text NOT NULL,
    apns_bundle_id text,
    apns_key_id text,
    apns_team_id text,
    apns_key_path text,
    apns_key_content text,
    fcm_server_key text,
    resend_api_key text,
    resend_from_email text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE platform_delivery_config; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.platform_delivery_config IS 'Platform-specific delivery configuration for push notifications and email';


--
-- Name: COLUMN platform_delivery_config.apns_key_content; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.platform_delivery_config.apns_key_content IS 'Base64 encoded APNs key content (alternative to apns_key_path)';


--
-- Name: platform_delivery_config_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.platform_delivery_config_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: platform_delivery_config_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.platform_delivery_config_id_seq OWNED BY public.platform_delivery_config.id;


--
-- Name: platform_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.platform_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: platform_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.platform_events_id_seq OWNED BY public.platform_events.id;


--
-- Name: platform_memberships; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.platform_memberships (
    id integer NOT NULL,
    platform_id character varying NOT NULL,
    profile_id character varying NOT NULL,
    joined_at timestamp without time zone NOT NULL
);


--
-- Name: TABLE platform_memberships; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.platform_memberships IS 'Records of profiles joined to platforms. Records are deleted when a user leaves a platform.';


--
-- Name: platform_memberships_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.platform_memberships_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: platform_memberships_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.platform_memberships_id_seq OWNED BY public.platform_memberships.id;


--
-- Name: platform_moderators; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.platform_moderators (
    id integer NOT NULL,
    platform_id character varying NOT NULL,
    moderator_address character varying NOT NULL,
    added_by character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: platform_moderators_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.platform_moderators_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: platform_moderators_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.platform_moderators_id_seq OWNED BY public.platform_moderators.id;


--
-- Name: unified_revenue; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.unified_revenue (
    revenue_source character varying NOT NULL,
    revenue_type character varying NOT NULL,
    creator_address character varying NOT NULL,
    platform_address character varying,
    amount bigint NOT NULL,
    currency character varying DEFAULT 'MYSO'::character varying NOT NULL,
    content_id character varying,
    content_type character varying,
    payer_address character varying NOT NULL,
    recipient_address character varying NOT NULL,
    revenue_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT unified_revenue_revenue_source_check CHECK (((revenue_source)::text = ANY ((ARRAY['subscription'::character varying, 'mydata'::character varying, 'spt'::character varying, 'tips'::character varying, 'posts'::character varying])::text[])))
);


--
-- Name: TABLE unified_revenue; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.unified_revenue IS 'Unified revenue tracking across all MySocial revenue sources (TimescaleDB)';


--
-- Name: platform_revenue_summary; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.platform_revenue_summary AS
 SELECT platform_address,
    sum(amount) AS total_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'subscription'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_subscription_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'mydata'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_mydata_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'spt'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_spt_revenue,
    count(*) AS total_transactions,
    count(DISTINCT creator_address) AS total_creators,
    count(DISTINCT payer_address) AS total_payers,
    avg(amount) AS avg_transaction_amount,
    count(DISTINCT date_trunc('month'::text, "time")) AS active_months,
    (date_trunc('month'::text, max("time")))::date AS last_active_month
   FROM public.unified_revenue
  WHERE ((platform_address IS NOT NULL) AND ("time" >= date_trunc('month'::text, (now() - '1 year'::interval))))
  GROUP BY platform_address
  ORDER BY (sum(amount)) DESC;


--
-- Name: VIEW platform_revenue_summary; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.platform_revenue_summary IS 'Platform revenue analytics using direct unified_revenue queries (12-month summary)';


--
-- Name: platforms; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.platforms (
    id integer NOT NULL,
    platform_id character varying NOT NULL,
    name character varying NOT NULL,
    tagline character varying NOT NULL,
    description text,
    logo character varying,
    developer_address character varying NOT NULL,
    terms_of_service text,
    privacy_policy text,
    platforms jsonb,
    links jsonb,
    status smallint NOT NULL,
    release_date character varying,
    shutdown_date character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    is_approved boolean DEFAULT false NOT NULL,
    approval_changed_at timestamp without time zone,
    approved_by character varying,
    wants_dao_governance boolean,
    governance_registry_id text,
    delegate_count bigint,
    delegate_term_epochs bigint,
    max_votes_per_user bigint,
    min_on_chain_age_days bigint,
    proposal_submission_cost bigint,
    quadratic_base_cost bigint,
    quorum_votes bigint,
    voting_period_epochs bigint,
    treasury bigint,
    version bigint
);


--
-- Name: platforms_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.platforms_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: platforms_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.platforms_id_seq OWNED BY public.platforms.id;


--
-- Name: poc_analysis_results; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.poc_analysis_results (
    post_id character varying NOT NULL,
    media_type smallint NOT NULL,
    similarity_detected boolean NOT NULL,
    highest_similarity_score bigint NOT NULL,
    oracle_address character varying NOT NULL,
    original_creator character varying,
    analysis_timestamp bigint NOT NULL,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    reasoning text,
    evidence_urls jsonb
);


--
-- Name: TABLE poc_analysis_results; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.poc_analysis_results IS 'Oracle analysis results for content similarity detection';


--
-- Name: poc_configuration; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.poc_configuration (
    id integer NOT NULL,
    image_threshold bigint NOT NULL,
    video_threshold bigint NOT NULL,
    audio_threshold bigint NOT NULL,
    revenue_redirect_percentage bigint NOT NULL,
    dispute_cost bigint NOT NULL,
    dispute_protocol_fee bigint NOT NULL,
    min_vote_stake bigint NOT NULL,
    max_vote_stake bigint NOT NULL,
    voting_duration_epochs bigint NOT NULL,
    updated_by character varying NOT NULL,
    updated_at bigint NOT NULL,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE poc_configuration; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.poc_configuration IS 'System-wide PoC configuration parameters';


--
-- Name: poc_configuration_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.poc_configuration_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: poc_configuration_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.poc_configuration_id_seq OWNED BY public.poc_configuration.id;


--
-- Name: poc_daily_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.poc_daily_stats AS
 SELECT day,
    badges_issued,
    redirections_created,
    disputes_submitted,
    votes_cast
   FROM _timescaledb_internal._materialized_hypertable_106;


--
-- Name: poc_dispute_votes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.poc_dispute_votes (
    dispute_id character varying NOT NULL,
    voter character varying NOT NULL,
    vote_choice smallint NOT NULL,
    stake_amount bigint NOT NULL,
    voted_at bigint NOT NULL,
    reward_claimed boolean DEFAULT false,
    reward_amount bigint,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE poc_dispute_votes; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.poc_dispute_votes IS 'Community votes on PoC disputes';


--
-- Name: COLUMN poc_dispute_votes.vote_choice; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.poc_dispute_votes.vote_choice IS '1=uphold, 2=overturn';


--
-- Name: poc_disputes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.poc_disputes (
    dispute_id character varying NOT NULL,
    post_id character varying NOT NULL,
    disputer character varying NOT NULL,
    dispute_type smallint NOT NULL,
    evidence text NOT NULL,
    status smallint NOT NULL,
    stake_amount bigint NOT NULL,
    voting_start_epoch bigint NOT NULL,
    voting_end_epoch bigint NOT NULL,
    resolution smallint,
    winning_side smallint,
    total_winning_stake bigint,
    total_losing_stake bigint,
    submitted_at bigint NOT NULL,
    resolved_at bigint,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE poc_disputes; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.poc_disputes IS 'Community disputes challenging PoC decisions';


--
-- Name: COLUMN poc_disputes.dispute_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.poc_disputes.dispute_type IS '1=challenge badge, 2=challenge redirection';


--
-- Name: COLUMN poc_disputes.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.poc_disputes.status IS '1=voting, 2=resolved_upheld, 3=resolved_overturned';


--
-- Name: poc_hourly_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.poc_hourly_stats AS
 SELECT hour,
    badges_issued_hourly,
    total_badges
   FROM _timescaledb_internal._materialized_hypertable_107;


--
-- Name: poc_revenue_redirections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.poc_revenue_redirections (
    redirection_id character varying NOT NULL,
    accused_post_id character varying NOT NULL,
    original_post_id character varying NOT NULL,
    redirect_percentage bigint NOT NULL,
    similarity_score bigint NOT NULL,
    created_at bigint NOT NULL,
    removed boolean DEFAULT false,
    removed_at bigint,
    transaction_id character varying NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE poc_revenue_redirections; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.poc_revenue_redirections IS 'Revenue redirection records for derivative content';


--
-- Name: popular_licenses; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.popular_licenses AS
 SELECT l.license_id,
    l.name,
    l.creator,
    l.license_type,
    count(p.id) AS post_count,
    sum(p.reaction_count) AS total_reactions,
    sum(p.comment_count) AS total_comments,
    sum(p.repost_count) AS total_reposts,
    sum(p.tips_received) AS total_tips
   FROM (public.my_ip l
     JOIN public.posts p ON ((p.my_ip_id = l.license_id)))
  WHERE ((p.deleted_at IS NULL) AND (p.removed_from_platform = false))
  GROUP BY l.license_id, l.name, l.creator, l.license_type
  ORDER BY (count(p.id)) DESC, (sum(p.reaction_count)) DESC;


--
-- Name: popular_mydata_30d; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.popular_mydata_30d AS
 SELECT d.mydata_id,
    d.owner,
    d.media_type,
    d.tags,
    count(DISTINCT p.buyer) AS unique_purchasers,
    sum(p.price) AS total_revenue,
    count(p.id) AS total_purchases,
    max(p."time") AS last_purchase
   FROM (public.mydata_data d
     LEFT JOIN public.mydata_purchases p ON ((((d.mydata_id)::text = (p.mydata_id)::text) AND (p."time" >= (now() - '30 days'::interval)))))
  GROUP BY d.mydata_id, d.owner, d.media_type, d.tags
  ORDER BY (count(DISTINCT p.buyer)) DESC, (sum(p.price)) DESC;


--
-- Name: popular_posts; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.popular_posts AS
 SELECT id,
    post_id,
    owner,
    profile_id,
    content,
    media_urls,
    post_type,
    created_at,
    reaction_count,
    comment_count,
    repost_count,
    tips_received
   FROM public.posts p
  WHERE ((deleted_at IS NULL) AND (removed_from_platform = false) AND ((EXTRACT(epoch FROM now()) - (created_at)::numeric) < (2592000)::numeric))
  ORDER BY reaction_count DESC, created_at DESC;


--
-- Name: spt_transactions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_transactions (
    id integer NOT NULL,
    pool_id character varying NOT NULL,
    transaction_type character varying NOT NULL,
    sender character varying NOT NULL,
    amount bigint NOT NULL,
    mys_amount bigint NOT NULL,
    fee_amount bigint NOT NULL,
    creator_fee bigint NOT NULL,
    platform_fee bigint NOT NULL,
    treasury_fee bigint NOT NULL,
    price bigint NOT NULL,
    created_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: popular_token_pools; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.popular_token_pools AS
 SELECT p.pool_id,
    p.token_type,
    p.owner,
    p.associated_id,
    p.symbol,
    p.name,
    p.circulating_supply,
    count(t.id) AS transaction_count,
    sum(
        CASE
            WHEN ((t.transaction_type)::text = 'BUY'::text) THEN t.mys_amount
            ELSE (0)::bigint
        END) AS buy_volume,
    sum(
        CASE
            WHEN ((t.transaction_type)::text = 'SELL'::text) THEN t.mys_amount
            ELSE (0)::bigint
        END) AS sell_volume,
    sum(t.mys_amount) AS total_volume,
    COALESCE(ph.price, p.base_price) AS current_price
   FROM ((public.social_proof_token_pools p
     JOIN public.spt_transactions t ON (((p.pool_id)::text = (t.pool_id)::text)))
     LEFT JOIN ( SELECT DISTINCT ON (spt_price_history.pool_id) spt_price_history.pool_id,
            spt_price_history.price
           FROM public.spt_price_history
          ORDER BY spt_price_history.pool_id, spt_price_history."time" DESC) ph ON (((p.pool_id)::text = (ph.pool_id)::text)))
  WHERE ((t."time" > (now() - '7 days'::interval)) AND (p."time" = ( SELECT max(sub."time") AS max
           FROM public.social_proof_token_pools sub
          WHERE ((sub.pool_id)::text = (p.pool_id)::text))))
  GROUP BY p.pool_id, p.token_type, p.owner, p.associated_id, p.symbol, p.name, p.circulating_supply, p.base_price, ph.price
  ORDER BY (sum(t.mys_amount)) DESC;


--
-- Name: post_interactions; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.post_interactions AS
 SELECT p.post_id,
    p.owner AS post_owner,
    p.profile_id AS post_profile_id,
    p.content AS post_content,
    p.created_at AS post_created_at,
    c.comment_id,
    c.owner AS comment_owner,
    c.profile_id AS comment_profile_id,
    c.content AS comment_content,
    c.created_at AS comment_created_at,
    r.reaction_text,
    r.user_address AS reaction_user,
    r.created_at AS reaction_created_at
   FROM ((public.posts p
     LEFT JOIN public.comments c ON (((p.post_id)::text = (c.post_id)::text)))
     LEFT JOIN public.reactions r ON (((((r.object_id)::text = (p.post_id)::text) AND (r.is_post = true)) OR (((r.object_id)::text = (c.comment_id)::text) AND (r.is_post = false)))))
  WHERE ((p.deleted_at IS NULL) AND ((c.deleted_at IS NULL) OR (c.deleted_at IS NOT NULL)));


--
-- Name: post_prediction_config_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.post_prediction_config_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: post_prediction_config_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.post_prediction_config_id_seq OWNED BY public.post_prediction_config.id;


--
-- Name: post_stats_daily; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.post_stats_daily AS
 SELECT _materialized_hypertable_34.bucket,
    _materialized_hypertable_34.posts_with_tips,
    _materialized_hypertable_34.total_tip_amount,
    _materialized_hypertable_34.total_tips
   FROM _timescaledb_internal._materialized_hypertable_34
  WHERE (_materialized_hypertable_34.bucket < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(34)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, tips."time") AS bucket,
    count(DISTINCT tips.object_id) AS posts_with_tips,
    sum(tips.amount) AS total_tip_amount,
    count(*) AS total_tips
   FROM public.tips
  WHERE ((tips.is_post = true) AND (tips."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(34)), '-infinity'::timestamp with time zone)))
  GROUP BY (public.time_bucket('1 day'::interval, tips."time"));


--
-- Name: posts_deletion_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.posts_deletion_events (
    id integer NOT NULL,
    object_id character varying NOT NULL,
    owner character varying NOT NULL,
    profile_id character varying NOT NULL,
    is_post boolean NOT NULL,
    post_type character varying,
    post_id character varying,
    deleted_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: posts_deletion_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.posts_deletion_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: posts_deletion_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.posts_deletion_events_id_seq OWNED BY public.posts_deletion_events.id;


--
-- Name: posts_moderation_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.posts_moderation_events (
    id integer NOT NULL,
    object_id character varying NOT NULL,
    platform_id character varying NOT NULL,
    removed boolean NOT NULL,
    moderated_by character varying NOT NULL,
    moderated_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: posts_moderation_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.posts_moderation_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: posts_moderation_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.posts_moderation_events_id_seq OWNED BY public.posts_moderation_events.id;


--
-- Name: posts_reports; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.posts_reports (
    id integer NOT NULL,
    object_id character varying NOT NULL,
    is_comment boolean NOT NULL,
    reporter character varying NOT NULL,
    reason_code smallint NOT NULL,
    description text NOT NULL,
    reported_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: posts_reports_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.posts_reports_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: posts_reports_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.posts_reports_id_seq OWNED BY public.posts_reports.id;


--
-- Name: posts_transfers; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.posts_transfers (
    id integer NOT NULL,
    object_id character varying NOT NULL,
    previous_owner character varying NOT NULL,
    new_owner character varying NOT NULL,
    is_post boolean NOT NULL,
    transferred_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: posts_transfers_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.posts_transfers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: posts_transfers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.posts_transfers_id_seq OWNED BY public.posts_transfers.id;


--
-- Name: profile_badges; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profile_badges (
    id integer NOT NULL,
    profile_id text NOT NULL,
    badge_id text NOT NULL,
    badge_name text NOT NULL,
    badge_description text,
    badge_image_url text,
    platform_id text NOT NULL,
    assigned_by text NOT NULL,
    assigned_at bigint NOT NULL,
    revoked boolean DEFAULT false,
    revoked_at bigint,
    revoked_by text,
    badge_type smallint NOT NULL,
    transaction_id text NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: profile_badges_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.profile_badges_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: profile_badges_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.profile_badges_id_seq OWNED BY public.profile_badges.id;


--
-- Name: profile_daily_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.profile_daily_stats AS
 SELECT _materialized_hypertable_8.day,
    _materialized_hypertable_8.event_type,
    _materialized_hypertable_8.event_count
   FROM _timescaledb_internal._materialized_hypertable_8
  WHERE (_materialized_hypertable_8.day < COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(8)), '-infinity'::timestamp without time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, profile_events.created_at) AS day,
    profile_events.event_type,
    count(*) AS event_count
   FROM public.profile_events
  WHERE (profile_events.created_at >= COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(8)), '-infinity'::timestamp without time zone))
  GROUP BY (public.time_bucket('1 day'::interval, profile_events.created_at)), profile_events.event_type;


--
-- Name: profile_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.profile_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: profile_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.profile_events_id_seq OWNED BY public.profile_events.id;


--
-- Name: profile_offers; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profile_offers (
    id integer NOT NULL,
    profile_id text NOT NULL,
    offeror_address text NOT NULL,
    amount bigint NOT NULL,
    status text DEFAULT 'pending'::text NOT NULL,
    created_at bigint NOT NULL,
    updated_at bigint NOT NULL,
    resolved_at bigint,
    transaction_id text NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: profile_offers_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.profile_offers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: profile_offers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.profile_offers_id_seq OWNED BY public.profile_offers.id;


--
-- Name: profile_sale_fees; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profile_sale_fees (
    id integer NOT NULL,
    profile_id text NOT NULL,
    offeror_address text NOT NULL,
    previous_owner_address text NOT NULL,
    sale_amount bigint NOT NULL,
    fee_amount bigint NOT NULL,
    fee_recipient_address text NOT NULL,
    "timestamp" bigint NOT NULL,
    transaction_id text NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: profile_sale_fees_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.profile_sale_fees_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: profile_sale_fees_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.profile_sale_fees_id_seq OWNED BY public.profile_sale_fees.id;


--
-- Name: profile_subscription_services; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profile_subscription_services (
    service_id character varying NOT NULL,
    profile_owner character varying NOT NULL,
    profile_id character varying NOT NULL,
    monthly_fee bigint NOT NULL,
    active boolean DEFAULT true NOT NULL,
    subscriber_count bigint DEFAULT 0 NOT NULL,
    created_at bigint NOT NULL,
    updated_at bigint,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: profiles; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profiles (
    id integer NOT NULL,
    owner_address character varying(255) NOT NULL,
    username character varying(100) NOT NULL,
    display_name character varying(255),
    bio text,
    profile_photo character varying(255),
    website character varying(255),
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    post_count integer DEFAULT 0 NOT NULL,
    min_offer_amount bigint,
    cover_photo character varying,
    profile_id character varying,
    birthdate text,
    current_location text,
    raised_location text,
    phone text,
    email text,
    gender text,
    political_view text,
    religion text,
    education text,
    primary_language text,
    relationship_status text,
    x_username text,
    mastodon_username text,
    facebook_username text,
    reddit_username text,
    github_username text,
    followers_count integer DEFAULT 0 NOT NULL,
    following_count integer DEFAULT 0 NOT NULL,
    block_list_address character varying,
    subscription_service_id character varying,
    subscription_enabled boolean DEFAULT false,
    blocked_count integer DEFAULT 0 NOT NULL,
    social_proof_token_address character varying,
    instagram_username text,
    selected_badge_id character varying,
    reservation_pool_address character varying,
    paid_messaging_enabled boolean DEFAULT false NOT NULL,
    paid_messaging_min_cost bigint,
    sensitive_data_updated_at timestamp without time zone,
    CONSTRAINT chk_profiles_post_count_non_negative CHECK ((post_count >= 0))
);


--
-- Name: COLUMN profiles.post_count; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.profiles.post_count IS 'Number of top-level, non-deleted posts created by this profile. Updated synchronously with post creation/deletion events.';


--
-- Name: COLUMN profiles.min_offer_amount; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.profiles.min_offer_amount IS 'Minimum MYSO token amount the profile owner will accept as an offer for profile sale. NULL means no minimum is set and profile is not explicitly for sale.';


--
-- Name: COLUMN profiles.block_list_address; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.profiles.block_list_address IS 'The Blockchain address of the profile''s BlockList object';


--
-- Name: COLUMN profiles.selected_badge_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.profiles.selected_badge_id IS 'The badge_id of the currently selected badge for this profile. NULL means no badge is selected. If None, the first badge in the badges vector should be displayed.';


--
-- Name: profiles_blocked; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.profiles_blocked (
    id integer NOT NULL,
    blocker_wallet_address text NOT NULL,
    blocked_address text NOT NULL,
    created_at timestamp without time zone NOT NULL
);


--
-- Name: TABLE profiles_blocked; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.profiles_blocked IS 'Records of profiles blocked by other profiles. Records are deleted when a profile is unblocked.';


--
-- Name: profiles_blocked_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.profiles_blocked_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: profiles_blocked_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.profiles_blocked_id_seq OWNED BY public.profiles_blocked.id;


--
-- Name: profiles_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.profiles_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: profiles_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.profiles_id_seq OWNED BY public.profiles.id;


--
-- Name: progress_store; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.progress_store (
    id integer NOT NULL,
    worker_id character varying NOT NULL,
    module_name character varying NOT NULL,
    last_processed_checkpoint bigint DEFAULT 0 NOT NULL,
    last_processed_event_id character varying,
    last_processed_timestamp bigint NOT NULL,
    processing_state character varying DEFAULT 'running'::character varying NOT NULL,
    error_count integer DEFAULT 0 NOT NULL,
    last_error_message text,
    last_error_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: progress_store_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.progress_store_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: progress_store_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.progress_store_id_seq OWNED BY public.progress_store.id;


--
-- Name: promoted_posts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.promoted_posts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: promoted_posts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.promoted_posts_id_seq OWNED BY public.promoted_posts.id;


--
-- Name: promotion_budget_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.promotion_budget_events (
    id integer NOT NULL,
    promotion_id character varying NOT NULL,
    post_id character varying NOT NULL,
    event_type character varying NOT NULL,
    amount bigint NOT NULL,
    remaining_budget bigint NOT NULL,
    "timestamp" bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: promotion_budget_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.promotion_budget_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: promotion_budget_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.promotion_budget_events_id_seq OWNED BY public.promotion_budget_events.id;


--
-- Name: promotion_performance; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.promotion_performance AS
 WITH promotion_stats AS (
         SELECT pp.promotion_id,
            pp.post_id,
            pp.owner,
            pp.payment_per_view,
            pp.total_budget,
            pp.remaining_budget,
            pp.created_at,
            COALESCE(pv.view_count, (0)::bigint) AS total_views,
            COALESCE(pv.total_paid, (0)::numeric) AS total_paid,
            COALESCE(pv.unique_viewers, (0)::bigint) AS unique_viewers
           FROM (public.promoted_posts pp
             LEFT JOIN ( SELECT promotion_views.promotion_id,
                    count(*) AS view_count,
                    sum(promotion_views.payment_amount) AS total_paid,
                    count(DISTINCT promotion_views.viewer) AS unique_viewers
                   FROM public.promotion_views
                  GROUP BY promotion_views.promotion_id) pv ON (((pp.promotion_id)::text = (pv.promotion_id)::text)))
        )
 SELECT promotion_id,
    post_id,
    owner,
    payment_per_view,
    total_budget,
    remaining_budget,
    created_at,
    total_views,
    total_paid,
    unique_viewers,
        CASE
            WHEN (total_budget > 0) THEN (((total_paid)::double precision / (total_budget)::double precision) * (100)::double precision)
            ELSE (0)::double precision
        END AS budget_utilization_percent,
        CASE
            WHEN (total_views > 0) THEN ((total_paid)::double precision / (total_views)::double precision)
            ELSE (0)::double precision
        END AS actual_cost_per_view,
        CASE
            WHEN ((remaining_budget > 0) AND (payment_per_view > 0)) THEN (remaining_budget / payment_per_view)
            ELSE (0)::bigint
        END AS estimated_remaining_views
   FROM promotion_stats ps;


--
-- Name: promotion_spending_daily; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.promotion_spending_daily AS
 SELECT _materialized_hypertable_84.bucket,
    _materialized_hypertable_84.active_promotions,
    _materialized_hypertable_84.total_spending,
    _materialized_hypertable_84.total_views,
    _materialized_hypertable_84.avg_payment_per_view
   FROM _timescaledb_internal._materialized_hypertable_84
  WHERE (_materialized_hypertable_84.bucket < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(84)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, promotion_views."time") AS bucket,
    count(DISTINCT promotion_views.promotion_id) AS active_promotions,
    sum(promotion_views.payment_amount) AS total_spending,
    count(*) AS total_views,
    avg(promotion_views.payment_amount) AS avg_payment_per_view
   FROM public.promotion_views
  WHERE (promotion_views."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(84)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('1 day'::interval, promotion_views."time"));


--
-- Name: promotion_status_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.promotion_status_events (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    promotion_id character varying NOT NULL,
    event_type character varying NOT NULL,
    triggered_by character varying NOT NULL,
    new_status boolean,
    amount bigint,
    "timestamp" bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: promotion_status_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.promotion_status_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: promotion_status_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.promotion_status_events_id_seq OWNED BY public.promotion_status_events.id;


--
-- Name: promotion_views_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.promotion_views_hourly AS
 SELECT _materialized_hypertable_83.bucket,
    _materialized_hypertable_83.post_id,
    _materialized_hypertable_83.promotion_id,
    _materialized_hypertable_83.platform_id,
    _materialized_hypertable_83.view_count,
    _materialized_hypertable_83.total_payments,
    _materialized_hypertable_83.avg_view_duration
   FROM _timescaledb_internal._materialized_hypertable_83
  WHERE (_materialized_hypertable_83.bucket < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(83)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('01:00:00'::interval, promotion_views."time") AS bucket,
    promotion_views.post_id,
    promotion_views.promotion_id,
    promotion_views.platform_id,
    count(*) AS view_count,
    sum(promotion_views.payment_amount) AS total_payments,
    avg(promotion_views.view_duration) AS avg_view_duration
   FROM public.promotion_views
  WHERE (promotion_views."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(83)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('01:00:00'::interval, promotion_views."time")), promotion_views.post_id, promotion_views.promotion_id, promotion_views.platform_id;


--
-- Name: promotion_views_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.promotion_views_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: promotion_views_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.promotion_views_id_seq OWNED BY public.promotion_views.id;


--
-- Name: proposal_voting_status; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.proposal_voting_status AS
 SELECT p.id AS proposal_id,
    p.title,
    p.proposal_type,
    p.status,
    p.submitter,
    p.delegate_approval_count,
    p.delegate_rejection_count,
    p.community_votes_for,
    p.community_votes_against,
    p.voting_start_time,
    p.voting_end_time,
    COALESCE(dv_counts.total_delegate_votes, (0)::bigint) AS confirmed_delegate_votes,
    COALESCE(cv_counts.total_community_voters, (0)::bigint) AS confirmed_community_voters,
    COALESCE(cv_counts.total_community_weight, (0)::numeric) AS confirmed_community_weight,
        CASE
            WHEN ((p.status = 2) AND ((p.voting_end_time)::numeric > EXTRACT(epoch FROM now()))) THEN (((p.voting_end_time)::numeric - EXTRACT(epoch FROM now())) / 86400.0)
            ELSE NULL::numeric
        END AS days_remaining
   FROM ((public.proposals p
     LEFT JOIN ( SELECT delegate_votes.proposal_id,
            count(*) AS total_delegate_votes
           FROM public.delegate_votes
          GROUP BY delegate_votes.proposal_id) dv_counts ON (((p.id)::text = (dv_counts.proposal_id)::text)))
     LEFT JOIN ( SELECT community_votes.proposal_id,
            count(*) AS total_community_voters,
            sum(community_votes.vote_weight) AS total_community_weight
           FROM public.community_votes
          GROUP BY community_votes.proposal_id) cv_counts ON (((p.id)::text = (cv_counts.proposal_id)::text)));


--
-- Name: reaction_counts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.reaction_counts (
    id integer NOT NULL,
    object_id character varying NOT NULL,
    reaction_text character varying NOT NULL,
    count bigint DEFAULT 0 NOT NULL
);


--
-- Name: reaction_counts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.reaction_counts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: reaction_counts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.reaction_counts_id_seq OWNED BY public.reaction_counts.id;


--
-- Name: reactions_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.reactions_hourly AS
 SELECT _materialized_hypertable_31.bucket,
    _materialized_hypertable_31.object_id,
    _materialized_hypertable_31.reaction_text,
    _materialized_hypertable_31.reaction_count
   FROM _timescaledb_internal._materialized_hypertable_31
  WHERE (_materialized_hypertable_31.bucket < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(31)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('01:00:00'::interval, reactions."time") AS bucket,
    reactions.object_id,
    reactions.reaction_text,
    count(*) AS reaction_count
   FROM public.reactions
  WHERE (reactions."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(31)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('01:00:00'::interval, reactions."time")), reactions.object_id, reactions.reaction_text;


--
-- Name: reactions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.reactions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: reactions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.reactions_id_seq OWNED BY public.reactions.id;


--
-- Name: relay_conversations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_conversations (
    id bigint NOT NULL,
    conversation_id text NOT NULL,
    participant1_address text NOT NULL,
    participant2_address text NOT NULL,
    last_message_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE relay_conversations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_conversations IS 'Conversation metadata and last message tracking';


--
-- Name: relay_conversations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.relay_conversations_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: relay_conversations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.relay_conversations_id_seq OWNED BY public.relay_conversations.id;


--
-- Name: relay_device_tokens; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_device_tokens (
    id bigint NOT NULL,
    user_address text NOT NULL,
    device_token text NOT NULL,
    platform text NOT NULL,
    device_id text,
    app_version text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    last_used_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE relay_device_tokens; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_device_tokens IS 'Device tokens for push notifications (APNs, FCM)';


--
-- Name: relay_device_tokens_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.relay_device_tokens_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: relay_device_tokens_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.relay_device_tokens_id_seq OWNED BY public.relay_device_tokens.id;


--
-- Name: relay_messages; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_messages (
    id bigint NOT NULL,
    conversation_id text NOT NULL,
    sender_address text NOT NULL,
    recipient_address text NOT NULL,
    content_type text DEFAULT 'text'::text NOT NULL,
    media_urls jsonb,
    metadata jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    delivered_at timestamp with time zone,
    read_at timestamp with time zone,
    content bytea
);


--
-- Name: TABLE relay_messages; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_messages IS 'Direct messages between users';


--
-- Name: COLUMN relay_messages.content; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.relay_messages.content IS 'Encrypted message content (AES-256-GCM, base64 encoded)';


--
-- Name: relay_messages_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.relay_messages_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: relay_messages_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.relay_messages_id_seq OWNED BY public.relay_messages.id;


--
-- Name: relay_notifications; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_notifications (
    id bigint NOT NULL,
    user_address text NOT NULL,
    notification_type text NOT NULL,
    title text NOT NULL,
    body text NOT NULL,
    data jsonb,
    read_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    platform_id text
);


--
-- Name: TABLE relay_notifications; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_notifications IS 'User notifications stored in Postgres (also cached in Redis)';


--
-- Name: COLUMN relay_notifications.platform_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.relay_notifications.platform_id IS 'Platform ID for platform-specific notification filtering';


--
-- Name: relay_notifications_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.relay_notifications_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: relay_notifications_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.relay_notifications_id_seq OWNED BY public.relay_notifications.id;


--
-- Name: relay_outbox; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_outbox (
    id bigint NOT NULL,
    event_type text NOT NULL,
    event_data jsonb NOT NULL,
    event_id text,
    transaction_id text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    processed_at timestamp with time zone,
    published_at timestamp with time zone,
    retry_count integer DEFAULT 0 NOT NULL,
    error_message text
);


--
-- Name: TABLE relay_outbox; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_outbox IS 'Outbox table for CDC - indexer writes events here, relay polls and publishes to Redpanda';


--
-- Name: relay_outbox_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.relay_outbox_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: relay_outbox_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.relay_outbox_id_seq OWNED BY public.relay_outbox.id;


--
-- Name: relay_user_preferences; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_user_preferences (
    user_address text NOT NULL,
    push_enabled boolean DEFAULT true NOT NULL,
    email_enabled boolean DEFAULT true NOT NULL,
    sms_enabled boolean DEFAULT false NOT NULL,
    notification_types jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE relay_user_preferences; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_user_preferences IS 'User notification and communication preferences';


--
-- Name: relay_ws_connections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.relay_ws_connections (
    id bigint NOT NULL,
    user_address text NOT NULL,
    connection_id text NOT NULL,
    connected_at timestamp with time zone DEFAULT now() NOT NULL,
    last_heartbeat_at timestamp with time zone DEFAULT now() NOT NULL,
    disconnected_at timestamp with time zone
);


--
-- Name: TABLE relay_ws_connections; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.relay_ws_connections IS 'Active WebSocket connections tracking';


--
-- Name: relay_ws_connections_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.relay_ws_connections_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: relay_ws_connections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.relay_ws_connections_id_seq OWNED BY public.relay_ws_connections.id;


--
-- Name: reposts_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.reposts_hourly AS
 SELECT _materialized_hypertable_32.bucket,
    _materialized_hypertable_32.original_post_id,
    _materialized_hypertable_32.repost_count
   FROM _timescaledb_internal._materialized_hypertable_32
  WHERE (_materialized_hypertable_32.bucket < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(32)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('01:00:00'::interval, reposts."time") AS bucket,
    reposts.original_post_id,
    count(*) AS repost_count
   FROM public.reposts
  WHERE (reposts."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(32)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('01:00:00'::interval, reposts."time")), reposts.original_post_id;


--
-- Name: revenue_dashboard_24h; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.revenue_dashboard_24h AS
 SELECT revenue_source,
    sum(amount) AS total_revenue_24h,
    count(*) AS total_transactions_24h,
    count(DISTINCT creator_address) AS unique_creators_24h,
    count(DISTINCT payer_address) AS unique_payers_24h,
    max(amount) AS largest_transaction_24h,
    avg(amount) AS avg_transaction_amount
   FROM public.unified_revenue
  WHERE ("time" >= (now() - '24:00:00'::interval))
  GROUP BY revenue_source
  ORDER BY (sum(amount)) DESC;


--
-- Name: VIEW revenue_dashboard_24h; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.revenue_dashboard_24h IS 'Real-time dashboard metrics using direct unified_revenue queries (24-hour summary)';


--
-- Name: reward_distributions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.reward_distributions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: reward_distributions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.reward_distributions_id_seq OWNED BY public.reward_distributions.id;


--
-- Name: rewards_daily; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.rewards_daily AS
 SELECT _materialized_hypertable_52.day,
    _materialized_hypertable_52.distribution_type,
    _materialized_hypertable_52.distribution_count,
    _materialized_hypertable_52.total_amount
   FROM _timescaledb_internal._materialized_hypertable_52
  WHERE (_materialized_hypertable_52.day < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(52)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, reward_distributions."time") AS day,
    reward_distributions.distribution_type,
    count(*) AS distribution_count,
    sum(reward_distributions.amount) AS total_amount
   FROM public.reward_distributions
  WHERE (reward_distributions."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(52)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('1 day'::interval, reward_distributions."time")), reward_distributions.distribution_type;


--
-- Name: social_graph_daily_stats; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.social_graph_daily_stats AS
 SELECT _materialized_hypertable_7.day,
    _materialized_hypertable_7.event_type,
    _materialized_hypertable_7.event_count
   FROM _timescaledb_internal._materialized_hypertable_7
  WHERE (_materialized_hypertable_7.day < COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(7)), '-infinity'::timestamp without time zone))
UNION ALL
 SELECT public.time_bucket('1 day'::interval, social_graph_events.created_at) AS day,
    social_graph_events.event_type,
    count(*) AS event_count
   FROM public.social_graph_events
  WHERE (social_graph_events.created_at >= COALESCE(_timescaledb_functions.to_timestamp_without_timezone(_timescaledb_functions.cagg_watermark(7)), '-infinity'::timestamp without time zone))
  GROUP BY (public.time_bucket('1 day'::interval, social_graph_events.created_at)), social_graph_events.event_type;


--
-- Name: social_graph_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.social_graph_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: social_graph_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.social_graph_events_id_seq OWNED BY public.social_graph_events.id;


--
-- Name: social_graph_relationships; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.social_graph_relationships (
    id integer NOT NULL,
    follower_address character varying NOT NULL,
    following_address character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE social_graph_relationships; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.social_graph_relationships IS 'Tracks follow relationships between profiles';


--
-- Name: social_graph_relationships_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.social_graph_relationships_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: social_graph_relationships_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.social_graph_relationships_id_seq OWNED BY public.social_graph_relationships.id;


--
-- Name: social_proof_of_truth; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.social_proof_of_truth (
    id integer NOT NULL,
    event_type character varying NOT NULL,
    post_id character varying NOT NULL,
    user_address character varying,
    is_yes boolean,
    escrow_amount bigint,
    amm_amount bigint,
    amount bigint,
    outcome smallint,
    total_escrow bigint,
    fee_taken bigint,
    confidence_bps bigint,
    timestamp_epoch bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    event_id character varying,
    transaction_id character varying,
    raw_event jsonb
);


--
-- Name: social_proof_of_truth_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.social_proof_of_truth_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: social_proof_of_truth_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.social_proof_of_truth_id_seq OWNED BY public.social_proof_of_truth.id;


--
-- Name: social_proof_token_pools_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.social_proof_token_pools_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: social_proof_token_pools_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.social_proof_token_pools_id_seq OWNED BY public.social_proof_token_pools.id;


--
-- Name: spot_bets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_bets (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    user_address character varying NOT NULL,
    is_yes boolean NOT NULL,
    escrow_amount bigint NOT NULL,
    amm_amount bigint NOT NULL,
    timestamp_epoch bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spot_bets_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_bets_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_bets_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_bets_id_seq OWNED BY public.spot_bets.id;


--
-- Name: spot_config; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_config (
    id integer NOT NULL,
    updated_by text NOT NULL,
    enable_flag boolean DEFAULT true NOT NULL,
    confidence_threshold_bps bigint DEFAULT 0 NOT NULL,
    resolution_window_epochs bigint DEFAULT 0 NOT NULL,
    max_resolution_window_epochs bigint DEFAULT 0 NOT NULL,
    payout_delay_epochs bigint DEFAULT 0 NOT NULL,
    fee_bps bigint DEFAULT 0 NOT NULL,
    fee_split_bps_platform bigint DEFAULT 0 NOT NULL,
    platform_treasury text NOT NULL,
    chain_treasury text NOT NULL,
    oracle_address text NOT NULL,
    max_single_bet bigint DEFAULT 0 NOT NULL,
    timestamp_ms bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id text NOT NULL
);


--
-- Name: spot_config_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_config_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_config_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_config_id_seq OWNED BY public.spot_config.id;


--
-- Name: spot_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_events (
    id integer NOT NULL,
    event_type character varying NOT NULL,
    post_id character varying NOT NULL,
    event_data jsonb NOT NULL,
    event_id character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: spot_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_events_id_seq OWNED BY public.spot_events.id;


--
-- Name: spot_payouts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_payouts (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    user_address character varying NOT NULL,
    amount bigint NOT NULL,
    timestamp_epoch bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spot_payouts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_payouts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_payouts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_payouts_id_seq OWNED BY public.spot_payouts.id;


--
-- Name: spot_records; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_records (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    status smallint NOT NULL,
    outcome smallint,
    amm_split_bps_used integer NOT NULL,
    total_yes_escrow bigint DEFAULT 0 NOT NULL,
    total_no_escrow bigint DEFAULT 0 NOT NULL,
    created_epoch bigint NOT NULL,
    last_resolution_epoch bigint,
    version bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spot_records_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_records_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_records_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_records_id_seq OWNED BY public.spot_records.id;


--
-- Name: spot_refunds; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_refunds (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    user_address character varying NOT NULL,
    amount bigint NOT NULL,
    timestamp_epoch bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spot_refunds_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_refunds_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_refunds_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_refunds_id_seq OWNED BY public.spot_refunds.id;


--
-- Name: spot_resolutions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spot_resolutions (
    id integer NOT NULL,
    post_id character varying NOT NULL,
    outcome smallint NOT NULL,
    total_escrow bigint NOT NULL,
    fee_taken bigint NOT NULL,
    resolved_epoch bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    reasoning text NOT NULL,
    evidence_urls jsonb NOT NULL
);


--
-- Name: spot_resolutions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spot_resolutions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spot_resolutions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spot_resolutions_id_seq OWNED BY public.spot_resolutions.id;


--
-- Name: spt_creator_revenue_summary; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.spt_creator_revenue_summary AS
 SELECT creator_address,
    sum(amount) AS total_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'subscription'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_subscription_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'mydata'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_mydata_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'spt'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_spt_revenue,
    sum(
        CASE
            WHEN ((revenue_source)::text = 'tips'::text) THEN amount
            ELSE (0)::bigint
        END) AS total_tips_revenue,
    count(*) AS total_transactions,
    count(DISTINCT payer_address) AS total_unique_payers,
    max(amount) AS largest_single_transaction,
    count(DISTINCT date("time")) AS active_days,
    max("time") AS last_revenue_date
   FROM public.unified_revenue
  WHERE ("time" >= (now() - '30 days'::interval))
  GROUP BY creator_address
  ORDER BY (sum(amount)) DESC;


--
-- Name: VIEW spt_creator_revenue_summary; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.spt_creator_revenue_summary IS 'Creator revenue leaderboard using direct unified_revenue queries (30-day summary)';


--
-- Name: spt_exchange_config; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_exchange_config (
    id integer NOT NULL,
    updated_by character varying NOT NULL,
    post_threshold bigint NOT NULL,
    profile_threshold bigint NOT NULL,
    max_individual_reservation_bps bigint NOT NULL,
    total_fee_bps bigint NOT NULL,
    creator_fee_bps bigint NOT NULL,
    platform_fee_bps bigint NOT NULL,
    treasury_fee_bps bigint NOT NULL,
    base_price bigint NOT NULL,
    quadratic_coefficient bigint NOT NULL,
    ecosystem_treasury character varying NOT NULL,
    max_hold_percent_bps bigint NOT NULL,
    trading_halted boolean DEFAULT false NOT NULL,
    updated_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spt_exchange_config_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spt_exchange_config_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spt_exchange_config_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spt_exchange_config_id_seq OWNED BY public.spt_exchange_config.id;


--
-- Name: spt_holdings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_holdings (
    id integer NOT NULL,
    pool_id character varying NOT NULL,
    holder_address character varying NOT NULL,
    amount bigint NOT NULL,
    acquired_at bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: spt_holdings_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spt_holdings_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spt_holdings_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spt_holdings_id_seq OWNED BY public.spt_holdings.id;


--
-- Name: spt_price_daily; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.spt_price_daily AS
 SELECT pool_id,
    bucket,
    open,
    high,
    low,
    close,
    circulating_supply
   FROM _timescaledb_internal._materialized_hypertable_74;


--
-- Name: spt_price_history_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spt_price_history_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spt_price_history_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spt_price_history_id_seq OWNED BY public.spt_price_history.id;


--
-- Name: spt_price_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.spt_price_hourly AS
 SELECT pool_id,
    bucket,
    open,
    high,
    low,
    close,
    circulating_supply
   FROM _timescaledb_internal._materialized_hypertable_73;


--
-- Name: spt_reservation_pools_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spt_reservation_pools_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spt_reservation_pools_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spt_reservation_pools_id_seq OWNED BY public.spt_reservation_pools.id;


--
-- Name: spt_reservations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spt_reservations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spt_reservations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spt_reservations_id_seq OWNED BY public.spt_reservations.id;


--
-- Name: spt_revenue; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.spt_revenue (
    pool_id character varying NOT NULL,
    transaction_type character varying NOT NULL,
    trader character varying NOT NULL,
    creator_address character varying NOT NULL,
    platform_address character varying NOT NULL,
    treasury_address character varying NOT NULL,
    creator_fee bigint NOT NULL,
    platform_fee bigint NOT NULL,
    treasury_fee bigint NOT NULL,
    total_fee bigint NOT NULL,
    token_amount bigint NOT NULL,
    mys_amount bigint NOT NULL,
    token_price bigint NOT NULL,
    revenue_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT spt_revenue_transaction_type_check CHECK (((transaction_type)::text = ANY ((ARRAY['buy'::character varying, 'sell'::character varying])::text[])))
);


--
-- Name: TABLE spt_revenue; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.spt_revenue IS 'SPT swap fee revenue tracking with real-time analytics (TimescaleDB)';


--
-- Name: spt_transactions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.spt_transactions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: spt_transactions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.spt_transactions_id_seq OWNED BY public.spt_transactions.id;


--
-- Name: subscription_access_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.subscription_access_logs (
    subscription_id character varying NOT NULL,
    subscriber character varying NOT NULL,
    content_type character varying NOT NULL,
    content_id character varying NOT NULL,
    access_time bigint NOT NULL,
    seal_id character varying,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    processing_success boolean DEFAULT true NOT NULL,
    processing_error text
);


--
-- Name: subscription_churn_analysis; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.subscription_churn_analysis AS
 SELECT day,
    service_id,
    daily_churn,
    daily_new_subs
   FROM _timescaledb_internal._materialized_hypertable_118;


--
-- Name: subscription_daily_metrics; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.subscription_daily_metrics AS
 SELECT day,
    service_id,
    new_subscriptions,
    cancelled_subscriptions,
    renewed_subscriptions
   FROM _timescaledb_internal._materialized_hypertable_113;


--
-- Name: subscription_daily_revenue; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.subscription_daily_revenue AS
 SELECT day,
    service_id,
    profile_owner,
    revenue_type,
    daily_revenue,
    transaction_count
   FROM _timescaledb_internal._materialized_hypertable_112;


--
-- Name: subscription_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.subscription_events (
    event_type character varying NOT NULL,
    subscription_id character varying,
    service_id character varying,
    subscriber character varying,
    event_data jsonb NOT NULL,
    event_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    processing_success boolean DEFAULT true NOT NULL,
    processing_error text
);


--
-- Name: subscription_health_metrics; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.subscription_health_metrics AS
 SELECT hour,
    service_id,
    active_subscriptions,
    cancelled_subscriptions,
    avg_renewal_count,
    total_renewal_balance
   FROM _timescaledb_internal._materialized_hypertable_117;


--
-- Name: tips_hourly; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.tips_hourly AS
 SELECT _materialized_hypertable_33.bucket,
    _materialized_hypertable_33.object_id,
    _materialized_hypertable_33.is_post,
    _materialized_hypertable_33.total_amount,
    _materialized_hypertable_33.tip_count
   FROM _timescaledb_internal._materialized_hypertable_33
  WHERE (_materialized_hypertable_33.bucket < COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(33)), '-infinity'::timestamp with time zone))
UNION ALL
 SELECT public.time_bucket('01:00:00'::interval, tips."time") AS bucket,
    tips.object_id,
    tips.is_post,
    sum(tips.amount) AS total_amount,
    count(*) AS tip_count
   FROM public.tips
  WHERE (tips."time" >= COALESCE(_timescaledb_functions.to_timestamp(_timescaledb_functions.cagg_watermark(33)), '-infinity'::timestamp with time zone))
  GROUP BY (public.time_bucket('01:00:00'::interval, tips."time")), tips.object_id, tips.is_post;


--
-- Name: tips_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.tips_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: tips_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.tips_id_seq OWNED BY public.tips.id;


--
-- Name: token_exchange_config; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.token_exchange_config (
    id integer NOT NULL,
    trading_halted boolean DEFAULT false NOT NULL,
    admin_address character varying NOT NULL,
    reason character varying(512) DEFAULT 'System initialized'::character varying NOT NULL,
    timestamp_ms bigint DEFAULT 0 NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL,
    CONSTRAINT single_config_row CHECK ((id = 1)),
    CONSTRAINT valid_admin_address CHECK ((length((admin_address)::text) > 0)),
    CONSTRAINT valid_reason CHECK (((length((reason)::text) > 0) AND (length((reason)::text) <= 512))),
    CONSTRAINT valid_timestamp CHECK ((timestamp_ms >= 0)),
    CONSTRAINT valid_transaction_id CHECK (((length((transaction_id)::text) > 0) AND (length((transaction_id)::text) <= 255)))
);


--
-- Name: token_exchange_config_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.token_exchange_config_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: token_exchange_config_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.token_exchange_config_id_seq OWNED BY public.token_exchange_config.id;


--
-- Name: token_exchange_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.token_exchange_events (
    id integer NOT NULL,
    event_type character varying NOT NULL,
    event_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    event_id character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT valid_event_id CHECK (((length((event_id)::text) > 0) AND (length((event_id)::text) <= 255))),
    CONSTRAINT valid_event_type CHECK (((length((event_type)::text) > 0) AND (length((event_type)::text) <= 100)))
);


--
-- Name: token_exchange_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.token_exchange_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: token_exchange_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.token_exchange_events_id_seq OWNED BY public.token_exchange_events.id;


--
-- Name: top_promoted_posts; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.top_promoted_posts AS
 SELECT pp.promotion_id,
    pp.post_id,
    pp.owner,
    pp.profile_id,
    p.content,
    p.created_at,
    pv_stats.view_count,
    pv_stats.total_paid,
    pv_stats.unique_viewers,
    pv_stats.avg_view_duration,
    (((pv_stats.view_count)::double precision / (EXTRACT(epoch FROM (now() - to_timestamp(((pp.created_at / 1000))::double precision))))::double precision) * (3600)::double precision) AS views_per_hour
   FROM ((public.promoted_posts pp
     JOIN public.posts p ON (((pp.post_id)::text = (p.post_id)::text)))
     LEFT JOIN ( SELECT promotion_views.promotion_id,
            count(*) AS view_count,
            sum(promotion_views.payment_amount) AS total_paid,
            count(DISTINCT promotion_views.viewer) AS unique_viewers,
            avg(promotion_views.view_duration) AS avg_view_duration
           FROM public.promotion_views
          WHERE (promotion_views."time" >= (now() - '7 days'::interval))
          GROUP BY promotion_views.promotion_id) pv_stats ON (((pp.promotion_id)::text = (pv_stats.promotion_id)::text)))
  WHERE ((p.deleted_at IS NULL) AND (p.removed_from_platform = false) AND (pv_stats.view_count > 0))
  ORDER BY (((pv_stats.view_count)::double precision / (EXTRACT(epoch FROM (now() - to_timestamp(((pp.created_at / 1000))::double precision))))::double precision) * (3600)::double precision) DESC;


--
-- Name: top_tipped_content; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.top_tipped_content AS
 WITH tipped_content AS (
         SELECT t.object_id,
            t.is_post,
            sum(t.amount) AS total_tips,
            count(*) AS tip_count
           FROM public.tips t
          WHERE ((EXTRACT(epoch FROM now()) - (t.created_at)::numeric) < (2592000)::numeric)
          GROUP BY t.object_id, t.is_post
        )
 SELECT tc.object_id,
        CASE
            WHEN tc.is_post THEN 'post'::text
            ELSE 'comment'::text
        END AS content_type,
        CASE
            WHEN tc.is_post THEN p.owner
            ELSE c.owner
        END AS owner,
        CASE
            WHEN tc.is_post THEN p.profile_id
            ELSE c.profile_id
        END AS profile_id,
        CASE
            WHEN tc.is_post THEN p.content
            ELSE c.content
        END AS content,
    tc.total_tips,
    tc.tip_count
   FROM ((tipped_content tc
     LEFT JOIN public.posts p ON ((tc.is_post AND ((tc.object_id)::text = (p.post_id)::text))))
     LEFT JOIN public.comments c ON (((NOT tc.is_post) AND ((tc.object_id)::text = (c.comment_id)::text))))
  WHERE ((tc.is_post AND (p.deleted_at IS NULL) AND (p.removed_from_platform = false)) OR ((NOT tc.is_post) AND (c.deleted_at IS NULL) AND (c.removed_from_platform = false)))
  ORDER BY tc.total_tips DESC;


--
-- Name: trending_posts; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.trending_posts AS
 SELECT id,
    post_id,
    owner,
    profile_id,
    content,
    media_urls,
    post_type,
    created_at,
    reaction_count,
    comment_count,
    repost_count,
    tips_received,
    ((((reaction_count * 1) + (comment_count * 2)) + (repost_count * 3)) + tips_received) AS engagement_score,
    (EXTRACT(epoch FROM now()) - (created_at)::numeric) AS age_seconds,
    (((((((reaction_count * 1) + (comment_count * 2)) + (repost_count * 3)) + tips_received))::numeric / ((EXTRACT(epoch FROM now()) - (created_at)::numeric) + (3600)::numeric)) * (10000)::numeric) AS trending_score
   FROM public.posts p
  WHERE ((deleted_at IS NULL) AND (removed_from_platform = false) AND ((EXTRACT(epoch FROM now()) - (created_at)::numeric) < (604800)::numeric))
  ORDER BY (((((((reaction_count * 1) + (comment_count * 2)) + (repost_count * 3)) + tips_received))::numeric / ((EXTRACT(epoch FROM now()) - (created_at)::numeric) + (3600)::numeric)) * (10000)::numeric) DESC;


--
-- Name: user_reservation_holdings; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.user_reservation_holdings AS
 SELECT s.reservatior_address,
    s.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    s.amount,
    s.reserved_at,
    sp.total_reserved,
    sp.required_threshold,
    (sp.total_reserved >= sp.required_threshold) AS threshold_met,
    sp.status AS pool_status
   FROM (public.spt_reservations s
     JOIN public.spt_reservation_pools sp ON (((s.pool_id)::text = (sp.pool_id)::text)))
  WHERE ((s."time" = ( SELECT max(sub."time") AS max
           FROM public.spt_reservations sub
          WHERE (((sub.pool_id)::text = (s.pool_id)::text) AND ((sub.reservatior_address)::text = (s.reservatior_address)::text)))) AND (sp."time" = ( SELECT max(sub."time") AS max
           FROM public.spt_reservation_pools sub
          WHERE ((sub.pool_id)::text = (sp.pool_id)::text))) AND (s.amount > 0))
  ORDER BY s.amount DESC;


--
-- Name: vesting_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.vesting_events (
    id integer NOT NULL,
    wallet_id character varying NOT NULL,
    event_type character varying NOT NULL,
    owner_address character varying NOT NULL,
    amount bigint NOT NULL,
    remaining_balance bigint,
    start_time bigint,
    duration bigint,
    curve_factor bigint,
    event_time bigint NOT NULL,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: vesting_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.vesting_events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: vesting_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.vesting_events_id_seq OWNED BY public.vesting_events.id;


--
-- Name: vesting_wallets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.vesting_wallets (
    wallet_id character varying NOT NULL,
    owner_address character varying NOT NULL,
    total_amount bigint NOT NULL,
    start_time bigint NOT NULL,
    duration bigint NOT NULL,
    curve_factor bigint NOT NULL,
    claimed_amount bigint DEFAULT 0 NOT NULL,
    remaining_balance bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: vote_decryption_failures; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.vote_decryption_failures (
    id integer NOT NULL,
    proposal_id character varying NOT NULL,
    voter_address character varying NOT NULL,
    failure_reason text NOT NULL,
    attempted_at bigint NOT NULL,
    encrypted_vote_length integer,
    "time" timestamp with time zone DEFAULT now() NOT NULL,
    transaction_id character varying NOT NULL
);


--
-- Name: vote_decryption_failures_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.vote_decryption_failures_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: vote_decryption_failures_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.vote_decryption_failures_id_seq OWNED BY public.vote_decryption_failures.id;


--
-- Name: voting_activity; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.voting_activity AS
 SELECT 'delegate'::text AS voter_type,
    delegate_voting_hourly.hour,
    delegate_voting_hourly.proposal_id,
    delegate_voting_hourly.approve_count,
    delegate_voting_hourly.reject_count,
    delegate_voting_hourly.total_votes,
    0 AS approve_weight,
    0 AS reject_weight
   FROM public.delegate_voting_hourly
UNION ALL
 SELECT 'community'::text AS voter_type,
    community_voting_hourly.hour,
    community_voting_hourly.proposal_id,
    0 AS approve_count,
    0 AS reject_count,
    community_voting_hourly.total_votes,
    community_voting_hourly.approve_weight,
    community_voting_hourly.reject_weight
   FROM public.community_voting_hourly;


--
-- Name: watermarks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE IF NOT EXISTS public.watermarks (
    id integer NOT NULL,
    worker_id character varying NOT NULL,
    stream_name character varying NOT NULL,
    watermark_timestamp bigint NOT NULL,
    checkpoint_sequence bigint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: watermarks_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE IF NOT EXISTS public.watermarks_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: watermarks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.watermarks_id_seq OWNED BY public.watermarks.id;


--
-- Name: weekly_creator_revenue; Type: MATERIALIZED VIEW; Schema: public; Owner: -
--

DROP MATERIALIZED VIEW IF EXISTS public.weekly_creator_revenue;
CREATE MATERIALIZED VIEW public.weekly_creator_revenue AS
 SELECT public.time_bucket('7 days'::interval, r."time") AS bucket,
    l.creator,
    sum(r.amount) AS total_amount,
    count(*) AS transaction_count
   FROM (public.my_ip_revenue r
     JOIN public.my_ip l ON ((r.license_id = l.license_id)))
  GROUP BY (public.time_bucket('7 days'::interval, r."time")), l.creator
  WITH NO DATA;


--
-- Name: _hyper_1_36_chunk id; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_1_36_chunk ALTER COLUMN id SET DEFAULT nextval('public.social_graph_events_id_seq'::regclass);


--
-- Name: _hyper_1_36_chunk created_at; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_1_36_chunk ALTER COLUMN created_at SET DEFAULT now();


--
-- Name: _hyper_1_40_chunk id; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_1_40_chunk ALTER COLUMN id SET DEFAULT nextval('public.social_graph_events_id_seq'::regclass);


--
-- Name: _hyper_1_40_chunk created_at; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_1_40_chunk ALTER COLUMN created_at SET DEFAULT now();


--
-- Name: _hyper_3_35_chunk id; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_3_35_chunk ALTER COLUMN id SET DEFAULT nextval('public.profile_events_id_seq'::regclass);


--
-- Name: _hyper_5_38_chunk id; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_5_38_chunk ALTER COLUMN id SET DEFAULT nextval('public.platform_events_id_seq'::regclass);


--
-- Name: _hyper_5_38_chunk created_at; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_5_38_chunk ALTER COLUMN created_at SET DEFAULT now();


--
-- Name: _hyper_5_39_chunk id; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_5_39_chunk ALTER COLUMN id SET DEFAULT nextval('public.platform_events_id_seq'::regclass);


--
-- Name: _hyper_5_39_chunk created_at; Type: DEFAULT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_5_39_chunk ALTER COLUMN created_at SET DEFAULT now();


--
-- Name: anonymous_votes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.anonymous_votes ALTER COLUMN id SET DEFAULT nextval('public.anonymous_votes_id_seq'::regclass);


--
-- Name: blocked_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocked_events ALTER COLUMN id SET DEFAULT nextval('public.blocked_events_id_seq'::regclass);


--
-- Name: blocked_profiles id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocked_profiles ALTER COLUMN id SET DEFAULT nextval('public.blocked_profiles_id_seq'::regclass);


--
-- Name: checkpoint_processing id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.checkpoint_processing ALTER COLUMN id SET DEFAULT nextval('public.checkpoint_processing_id_seq'::regclass);


--
-- Name: community_votes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.community_votes ALTER COLUMN id SET DEFAULT nextval('public.community_votes_id_seq'::regclass);


--
-- Name: delegate_ratings id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.delegate_ratings ALTER COLUMN id SET DEFAULT nextval('public.delegate_ratings_id_seq'::regclass);


--
-- Name: delegate_votes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.delegate_votes ALTER COLUMN id SET DEFAULT nextval('public.delegate_votes_id_seq'::regclass);


--
-- Name: delegates id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.delegates ALTER COLUMN id SET DEFAULT nextval('public.delegates_id_seq'::regclass);


--
-- Name: governance_registries id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.governance_registries ALTER COLUMN id SET DEFAULT nextval('public.governance_registries_id_seq'::regclass);


--
-- Name: indexer_checkpoint_state id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indexer_checkpoint_state ALTER COLUMN id SET DEFAULT nextval('public.indexer_checkpoint_state_id_seq'::regclass);


--
-- Name: my_ip id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip ALTER COLUMN id SET DEFAULT nextval('public.my_ip_id_seq'::regclass);


--
-- Name: my_ip_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_events ALTER COLUMN id SET DEFAULT nextval('public.my_ip_events_id_seq'::regclass);


--
-- Name: my_ip_grants id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_grants ALTER COLUMN id SET DEFAULT nextval('public.my_ip_grants_id_seq'::regclass);


--
-- Name: my_ip_permissions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_permissions ALTER COLUMN id SET DEFAULT nextval('public.my_ip_permissions_id_seq'::regclass);


--
-- Name: my_ip_revenue id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_revenue ALTER COLUMN id SET DEFAULT nextval('public.my_ip_revenue_id_seq1'::regclass);


--
-- Name: mydata_access_logs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_access_logs ALTER COLUMN id SET DEFAULT nextval('public.my_ip_access_logs_id_seq'::regclass);


--
-- Name: mydata_purchases id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_purchases ALTER COLUMN id SET DEFAULT nextval('public.my_ip_purchases_id_seq'::regclass);


--
-- Name: mydata_revenue id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_revenue ALTER COLUMN id SET DEFAULT nextval('public.my_ip_revenue_id_seq'::regclass);


--
-- Name: mydata_subscriptions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_subscriptions ALTER COLUMN id SET DEFAULT nextval('public.my_ip_subscriptions_id_seq'::regclass);


--
-- Name: nominated_delegates id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.nominated_delegates ALTER COLUMN id SET DEFAULT nextval('public.nominated_delegates_id_seq'::regclass);


--
-- Name: platform_blocked_profiles id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_blocked_profiles ALTER COLUMN id SET DEFAULT nextval('public.platform_blocked_profiles_id_seq'::regclass);


--
-- Name: platform_delivery_config id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_delivery_config ALTER COLUMN id SET DEFAULT nextval('public.platform_delivery_config_id_seq'::regclass);


--
-- Name: platform_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_events ALTER COLUMN id SET DEFAULT nextval('public.platform_events_id_seq'::regclass);


--
-- Name: platform_memberships id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_memberships ALTER COLUMN id SET DEFAULT nextval('public.platform_memberships_id_seq'::regclass);


--
-- Name: platform_moderators id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_moderators ALTER COLUMN id SET DEFAULT nextval('public.platform_moderators_id_seq'::regclass);


--
-- Name: platforms id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platforms ALTER COLUMN id SET DEFAULT nextval('public.platforms_id_seq'::regclass);


--
-- Name: poc_configuration id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_configuration ALTER COLUMN id SET DEFAULT nextval('public.poc_configuration_id_seq'::regclass);


--
-- Name: post_prediction_config id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_prediction_config ALTER COLUMN id SET DEFAULT nextval('public.post_prediction_config_id_seq'::regclass);


--
-- Name: posts_deletion_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_deletion_events ALTER COLUMN id SET DEFAULT nextval('public.posts_deletion_events_id_seq'::regclass);


--
-- Name: posts_moderation_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_moderation_events ALTER COLUMN id SET DEFAULT nextval('public.posts_moderation_events_id_seq'::regclass);


--
-- Name: posts_reports id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_reports ALTER COLUMN id SET DEFAULT nextval('public.posts_reports_id_seq'::regclass);


--
-- Name: posts_transfers id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_transfers ALTER COLUMN id SET DEFAULT nextval('public.posts_transfers_id_seq'::regclass);


--
-- Name: profile_badges id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_badges ALTER COLUMN id SET DEFAULT nextval('public.profile_badges_id_seq'::regclass);


--
-- Name: profile_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_events ALTER COLUMN id SET DEFAULT nextval('public.profile_events_id_seq'::regclass);


--
-- Name: profile_offers id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_offers ALTER COLUMN id SET DEFAULT nextval('public.profile_offers_id_seq'::regclass);


--
-- Name: profile_sale_fees id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_sale_fees ALTER COLUMN id SET DEFAULT nextval('public.profile_sale_fees_id_seq'::regclass);


--
-- Name: profiles id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profiles ALTER COLUMN id SET DEFAULT nextval('public.profiles_id_seq'::regclass);


--
-- Name: profiles_blocked id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profiles_blocked ALTER COLUMN id SET DEFAULT nextval('public.profiles_blocked_id_seq'::regclass);


--
-- Name: progress_store id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.progress_store ALTER COLUMN id SET DEFAULT nextval('public.progress_store_id_seq'::regclass);


--
-- Name: promoted_posts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promoted_posts ALTER COLUMN id SET DEFAULT nextval('public.promoted_posts_id_seq'::regclass);


--
-- Name: promotion_budget_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promotion_budget_events ALTER COLUMN id SET DEFAULT nextval('public.promotion_budget_events_id_seq'::regclass);


--
-- Name: promotion_status_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promotion_status_events ALTER COLUMN id SET DEFAULT nextval('public.promotion_status_events_id_seq'::regclass);


--
-- Name: promotion_views id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promotion_views ALTER COLUMN id SET DEFAULT nextval('public.promotion_views_id_seq'::regclass);


--
-- Name: reaction_counts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reaction_counts ALTER COLUMN id SET DEFAULT nextval('public.reaction_counts_id_seq'::regclass);


--
-- Name: reactions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reactions ALTER COLUMN id SET DEFAULT nextval('public.reactions_id_seq'::regclass);


--
-- Name: relay_conversations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_conversations ALTER COLUMN id SET DEFAULT nextval('public.relay_conversations_id_seq'::regclass);


--
-- Name: relay_device_tokens id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_device_tokens ALTER COLUMN id SET DEFAULT nextval('public.relay_device_tokens_id_seq'::regclass);


--
-- Name: relay_messages id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_messages ALTER COLUMN id SET DEFAULT nextval('public.relay_messages_id_seq'::regclass);


--
-- Name: relay_notifications id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_notifications ALTER COLUMN id SET DEFAULT nextval('public.relay_notifications_id_seq'::regclass);


--
-- Name: relay_outbox id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_outbox ALTER COLUMN id SET DEFAULT nextval('public.relay_outbox_id_seq'::regclass);


--
-- Name: relay_ws_connections id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_ws_connections ALTER COLUMN id SET DEFAULT nextval('public.relay_ws_connections_id_seq'::regclass);


--
-- Name: reward_distributions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reward_distributions ALTER COLUMN id SET DEFAULT nextval('public.reward_distributions_id_seq'::regclass);


--
-- Name: social_graph_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_graph_events ALTER COLUMN id SET DEFAULT nextval('public.social_graph_events_id_seq'::regclass);


--
-- Name: social_graph_relationships id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_graph_relationships ALTER COLUMN id SET DEFAULT nextval('public.social_graph_relationships_id_seq'::regclass);


--
-- Name: social_proof_of_truth id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_proof_of_truth ALTER COLUMN id SET DEFAULT nextval('public.social_proof_of_truth_id_seq'::regclass);


--
-- Name: social_proof_token_pools id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_proof_token_pools ALTER COLUMN id SET DEFAULT nextval('public.social_proof_token_pools_id_seq'::regclass);


--
-- Name: spot_bets id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_bets ALTER COLUMN id SET DEFAULT nextval('public.spot_bets_id_seq'::regclass);


--
-- Name: spot_config id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_config ALTER COLUMN id SET DEFAULT nextval('public.spot_config_id_seq'::regclass);


--
-- Name: spot_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_events ALTER COLUMN id SET DEFAULT nextval('public.spot_events_id_seq'::regclass);


--
-- Name: spot_payouts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_payouts ALTER COLUMN id SET DEFAULT nextval('public.spot_payouts_id_seq'::regclass);


--
-- Name: spot_records id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_records ALTER COLUMN id SET DEFAULT nextval('public.spot_records_id_seq'::regclass);


--
-- Name: spot_refunds id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_refunds ALTER COLUMN id SET DEFAULT nextval('public.spot_refunds_id_seq'::regclass);


--
-- Name: spot_resolutions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_resolutions ALTER COLUMN id SET DEFAULT nextval('public.spot_resolutions_id_seq'::regclass);


--
-- Name: spt_exchange_config id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_exchange_config ALTER COLUMN id SET DEFAULT nextval('public.spt_exchange_config_id_seq'::regclass);


--
-- Name: spt_holdings id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_holdings ALTER COLUMN id SET DEFAULT nextval('public.spt_holdings_id_seq'::regclass);


--
-- Name: spt_price_history id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_price_history ALTER COLUMN id SET DEFAULT nextval('public.spt_price_history_id_seq'::regclass);


--
-- Name: spt_reservation_pools id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_reservation_pools ALTER COLUMN id SET DEFAULT nextval('public.spt_reservation_pools_id_seq'::regclass);


--
-- Name: spt_reservations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_reservations ALTER COLUMN id SET DEFAULT nextval('public.spt_reservations_id_seq'::regclass);


--
-- Name: spt_transactions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_transactions ALTER COLUMN id SET DEFAULT nextval('public.spt_transactions_id_seq'::regclass);


--
-- Name: tips id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tips ALTER COLUMN id SET DEFAULT nextval('public.tips_id_seq'::regclass);


--
-- Name: token_exchange_config id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.token_exchange_config ALTER COLUMN id SET DEFAULT nextval('public.token_exchange_config_id_seq'::regclass);


--
-- Name: token_exchange_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.token_exchange_events ALTER COLUMN id SET DEFAULT nextval('public.token_exchange_events_id_seq'::regclass);


--
-- Name: vesting_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vesting_events ALTER COLUMN id SET DEFAULT nextval('public.vesting_events_id_seq'::regclass);


--
-- Name: vote_decryption_failures id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vote_decryption_failures ALTER COLUMN id SET DEFAULT nextval('public.vote_decryption_failures_id_seq'::regclass);


--
-- Name: watermarks id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.watermarks ALTER COLUMN id SET DEFAULT nextval('public.watermarks_id_seq'::regclass);


--
-- Name: _hyper_3_35_chunk 35_105_profile_events_pkey; Type: CONSTRAINT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_3_35_chunk
    ADD CONSTRAINT "35_105_profile_events_pkey" PRIMARY KEY (id, created_at);


--
-- Name: _hyper_1_36_chunk 36_103_social_graph_events_pkey; Type: CONSTRAINT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_1_36_chunk
    ADD CONSTRAINT "36_103_social_graph_events_pkey" PRIMARY KEY (id, created_at);


--
-- Name: _hyper_5_38_chunk 38_106_platform_events_pkey; Type: CONSTRAINT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_5_38_chunk
    ADD CONSTRAINT "38_106_platform_events_pkey" PRIMARY KEY (id, created_at);


--
-- Name: _hyper_5_39_chunk 39_107_platform_events_pkey; Type: CONSTRAINT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_5_39_chunk
    ADD CONSTRAINT "39_107_platform_events_pkey" PRIMARY KEY (id, created_at);


--
-- Name: _hyper_1_40_chunk 40_104_social_graph_events_pkey; Type: CONSTRAINT; Schema: _timescaledb_internal; Owner: -
--

ALTER TABLE ONLY _timescaledb_internal._hyper_1_40_chunk
    ADD CONSTRAINT "40_104_social_graph_events_pkey" PRIMARY KEY (id, created_at);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = '__diesel_schema_migrations_pkey' 
        AND conrelid = 'public.__diesel_schema_migrations'::regclass
    ) THEN
        ALTER TABLE ONLY public.__diesel_schema_migrations
            ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);
    END IF;
END $$;


--
-- Name: anonymous_votes anonymous_votes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.anonymous_votes
    ADD CONSTRAINT anonymous_votes_pkey PRIMARY KEY (id, "time");


--
-- Name: blocked_events blocked_events_event_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocked_events
    ADD CONSTRAINT blocked_events_event_id_key UNIQUE (event_id);


--
-- Name: blocked_events blocked_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocked_events
    ADD CONSTRAINT blocked_events_pkey PRIMARY KEY (id);


--
-- Name: blocked_profiles blocked_profiles_blocker_address_blocked_address_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocked_profiles
    ADD CONSTRAINT blocked_profiles_blocker_address_blocked_address_key UNIQUE (blocker_address, blocked_address);


--
-- Name: blocked_profiles blocked_profiles_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.blocked_profiles
    ADD CONSTRAINT blocked_profiles_pkey PRIMARY KEY (id);


--
-- Name: checkpoint_processing checkpoint_processing_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.checkpoint_processing
    ADD CONSTRAINT checkpoint_processing_pkey PRIMARY KEY (id, processing_start_time);


--
-- Name: comments comments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_pkey PRIMARY KEY (id, "time");


--
-- Name: community_votes community_votes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.community_votes
    ADD CONSTRAINT community_votes_pkey PRIMARY KEY (id, "time");


--
-- Name: continuous_aggregate_refresh_status continuous_aggregate_refresh_status_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.continuous_aggregate_refresh_status
    ADD CONSTRAINT continuous_aggregate_refresh_status_pkey PRIMARY KEY (view_name);


--
-- Name: delegate_ratings delegate_ratings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.delegate_ratings
    ADD CONSTRAINT delegate_ratings_pkey PRIMARY KEY (id, "time");


--
-- Name: delegate_votes delegate_votes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.delegate_votes
    ADD CONSTRAINT delegate_votes_pkey PRIMARY KEY (id, "time");


--
-- Name: delegates delegates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.delegates
    ADD CONSTRAINT delegates_pkey PRIMARY KEY (id, "time");


--
-- Name: governance_registries governance_registries_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.governance_registries
    ADD CONSTRAINT governance_registries_pkey PRIMARY KEY (id);


--
-- Name: indexer_checkpoint_state indexer_checkpoint_state_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indexer_checkpoint_state
    ADD CONSTRAINT indexer_checkpoint_state_pkey PRIMARY KEY (id);


--
-- Name: indexer_progress indexer_progress_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indexer_progress
    ADD CONSTRAINT indexer_progress_pkey PRIMARY KEY (id);


--
-- Name: my_ip_permissions my_ip_permissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_permissions
    ADD CONSTRAINT my_ip_permissions_pkey PRIMARY KEY (id);


--
-- Name: mydata_data mydata_data_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_data
    ADD CONSTRAINT mydata_data_pkey PRIMARY KEY (mydata_id);


--
-- Name: mydata_registry mydata_registry_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_registry
    ADD CONSTRAINT mydata_registry_pkey PRIMARY KEY (ip_id);


--
-- Name: nominated_delegates nominated_delegates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.nominated_delegates
    ADD CONSTRAINT nominated_delegates_pkey PRIMARY KEY (id, "time");


--
-- Name: my_ip pk_my_ip; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip
    ADD CONSTRAINT pk_my_ip PRIMARY KEY (id, "time");


--
-- Name: my_ip_events pk_my_ip_events; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_events
    ADD CONSTRAINT pk_my_ip_events PRIMARY KEY (id, "time");


--
-- Name: my_ip_grants pk_my_ip_grants; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_grants
    ADD CONSTRAINT pk_my_ip_grants PRIMARY KEY (id, "time");


--
-- Name: my_ip_revenue pk_my_ip_revenue; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.my_ip_revenue
    ADD CONSTRAINT pk_my_ip_revenue PRIMARY KEY (id, "time");


--
-- Name: mydata_access_logs pk_mydata_access_logs; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_access_logs
    ADD CONSTRAINT pk_mydata_access_logs PRIMARY KEY (id, "time");


--
-- Name: mydata_purchases pk_mydata_purchases; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_purchases
    ADD CONSTRAINT pk_mydata_purchases PRIMARY KEY (id, "time");


--
-- Name: mydata_revenue pk_mydata_revenue; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_revenue
    ADD CONSTRAINT pk_mydata_revenue PRIMARY KEY (id, "time");


--
-- Name: mydata_subscriptions pk_mydata_subscriptions; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mydata_subscriptions
    ADD CONSTRAINT pk_mydata_subscriptions PRIMARY KEY (id, "time");


--
-- Name: profile_badges pk_profile_badges; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_badges
    ADD CONSTRAINT pk_profile_badges PRIMARY KEY (id, "time");


--
-- Name: profile_offers pk_profile_offers; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_offers
    ADD CONSTRAINT pk_profile_offers PRIMARY KEY (id, "time");


--
-- Name: profile_sale_fees pk_profile_sale_fees; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_sale_fees
    ADD CONSTRAINT pk_profile_sale_fees PRIMARY KEY (id, "time");


--
-- Name: social_proof_token_pools pk_social_proof_token_pools; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_proof_token_pools
    ADD CONSTRAINT pk_social_proof_token_pools PRIMARY KEY (id, "time");


--
-- Name: spt_exchange_config pk_spt_exchange_config; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_exchange_config
    ADD CONSTRAINT pk_spt_exchange_config PRIMARY KEY (id, "time");


--
-- Name: spt_holdings pk_spt_holdings; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_holdings
    ADD CONSTRAINT pk_spt_holdings PRIMARY KEY (id, "time");


--
-- Name: spt_price_history pk_spt_price_history; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_price_history
    ADD CONSTRAINT pk_spt_price_history PRIMARY KEY (id, "time");


--
-- Name: spt_reservation_pools pk_spt_reservation_pools; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_reservation_pools
    ADD CONSTRAINT pk_spt_reservation_pools PRIMARY KEY (id, "time");


--
-- Name: spt_reservations pk_spt_reservations; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_reservations
    ADD CONSTRAINT pk_spt_reservations PRIMARY KEY (id, "time");


--
-- Name: spt_transactions pk_spt_transactions; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spt_transactions
    ADD CONSTRAINT pk_spt_transactions PRIMARY KEY (id, "time");


--
-- Name: vesting_events pk_vesting_events; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vesting_events
    ADD CONSTRAINT pk_vesting_events PRIMARY KEY (id, "time");


--
-- Name: platform_blocked_profiles platform_blocked_profiles_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_blocked_profiles
    ADD CONSTRAINT platform_blocked_profiles_pkey PRIMARY KEY (id);


--
-- Name: platform_blocked_profiles platform_blocked_profiles_platform_id_profile_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_blocked_profiles
    ADD CONSTRAINT platform_blocked_profiles_platform_id_profile_id_key UNIQUE (platform_id, profile_id);


--
-- Name: platform_delivery_config platform_delivery_config_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_delivery_config
    ADD CONSTRAINT platform_delivery_config_pkey PRIMARY KEY (id);


--
-- Name: platform_delivery_config platform_delivery_config_platform_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_delivery_config
    ADD CONSTRAINT platform_delivery_config_platform_id_key UNIQUE (platform_id);


--
-- Name: platform_events platform_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_events
    ADD CONSTRAINT platform_events_pkey PRIMARY KEY (id, created_at);


--
-- Name: platform_memberships platform_memberships_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_memberships
    ADD CONSTRAINT platform_memberships_pkey PRIMARY KEY (id);


--
-- Name: platform_memberships platform_memberships_platform_id_profile_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_memberships
    ADD CONSTRAINT platform_memberships_platform_id_profile_id_key UNIQUE (platform_id, profile_id);


--
-- Name: platform_moderators platform_moderators_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_moderators
    ADD CONSTRAINT platform_moderators_pkey PRIMARY KEY (id);


--
-- Name: platform_moderators platform_moderators_platform_id_moderator_address_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_moderators
    ADD CONSTRAINT platform_moderators_platform_id_moderator_address_key UNIQUE (platform_id, moderator_address);


--
-- Name: platforms platforms_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platforms
    ADD CONSTRAINT platforms_pkey PRIMARY KEY (id);


--
-- Name: platforms platforms_platform_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platforms
    ADD CONSTRAINT platforms_platform_id_key UNIQUE (platform_id);


--
-- Name: poc_analysis_results poc_analysis_results_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_analysis_results
    ADD CONSTRAINT poc_analysis_results_pkey PRIMARY KEY (post_id, "time");


--
-- Name: poc_badges poc_badges_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_badges
    ADD CONSTRAINT poc_badges_pkey PRIMARY KEY (badge_id, "time");


--
-- Name: poc_configuration poc_configuration_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_configuration
    ADD CONSTRAINT poc_configuration_pkey PRIMARY KEY (id);


--
-- Name: poc_dispute_votes poc_dispute_votes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_dispute_votes
    ADD CONSTRAINT poc_dispute_votes_pkey PRIMARY KEY (dispute_id, voter, "time");


--
-- Name: poc_disputes poc_disputes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_disputes
    ADD CONSTRAINT poc_disputes_pkey PRIMARY KEY (dispute_id, "time");


--
-- Name: poc_revenue_redirections poc_revenue_redirections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.poc_revenue_redirections
    ADD CONSTRAINT poc_revenue_redirections_pkey PRIMARY KEY (redirection_id, "time");


--
-- Name: post_prediction_config post_prediction_config_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_prediction_config
    ADD CONSTRAINT post_prediction_config_pkey PRIMARY KEY (id, "time");


--
-- Name: posts_deletion_events posts_deletion_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_deletion_events
    ADD CONSTRAINT posts_deletion_events_pkey PRIMARY KEY (id, "time");


--
-- Name: posts_moderation_events posts_moderation_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_moderation_events
    ADD CONSTRAINT posts_moderation_events_pkey PRIMARY KEY (id, "time");


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id, "time");


--
-- Name: posts_reports posts_reports_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_reports
    ADD CONSTRAINT posts_reports_pkey PRIMARY KEY (id, "time");


--
-- Name: posts_transfers posts_transfers_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts_transfers
    ADD CONSTRAINT posts_transfers_pkey PRIMARY KEY (id, "time");


--
-- Name: profile_events profile_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_events
    ADD CONSTRAINT profile_events_pkey PRIMARY KEY (id, created_at);


--
-- Name: profile_subscription_services profile_subscription_services_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_subscription_services
    ADD CONSTRAINT profile_subscription_services_pkey PRIMARY KEY (service_id);


--
-- Name: profiles_blocked profiles_blocked_blocker_profile_id_blocked_profile_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profiles_blocked
    ADD CONSTRAINT profiles_blocked_blocker_profile_id_blocked_profile_id_key UNIQUE (blocker_wallet_address, blocked_address);


--
-- Name: profiles_blocked profiles_blocked_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profiles_blocked
    ADD CONSTRAINT profiles_blocked_pkey PRIMARY KEY (id);


--
-- Name: profiles profiles_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profiles
    ADD CONSTRAINT profiles_pkey PRIMARY KEY (id);


--
-- Name: progress_store progress_store_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.progress_store
    ADD CONSTRAINT progress_store_pkey PRIMARY KEY (id);


--
-- Name: promoted_posts promoted_posts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promoted_posts
    ADD CONSTRAINT promoted_posts_pkey PRIMARY KEY (id, "time");


--
-- Name: promotion_budget_events promotion_budget_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promotion_budget_events
    ADD CONSTRAINT promotion_budget_events_pkey PRIMARY KEY (id, "time");


--
-- Name: promotion_status_events promotion_status_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promotion_status_events
    ADD CONSTRAINT promotion_status_events_pkey PRIMARY KEY (id, "time");


--
-- Name: promotion_views promotion_views_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.promotion_views
    ADD CONSTRAINT promotion_views_pkey PRIMARY KEY (id, "time");


--
-- Name: proposals proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.proposals
    ADD CONSTRAINT proposals_pkey PRIMARY KEY (id, "time");


--
-- Name: reaction_counts reaction_counts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reaction_counts
    ADD CONSTRAINT reaction_counts_pkey PRIMARY KEY (id);


--
-- Name: reactions reactions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reactions
    ADD CONSTRAINT reactions_pkey PRIMARY KEY (id, "time");


--
-- Name: relay_conversations relay_conversations_conversation_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_conversations
    ADD CONSTRAINT relay_conversations_conversation_id_key UNIQUE (conversation_id);


--
-- Name: relay_conversations relay_conversations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_conversations
    ADD CONSTRAINT relay_conversations_pkey PRIMARY KEY (id);


--
-- Name: relay_device_tokens relay_device_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_device_tokens
    ADD CONSTRAINT relay_device_tokens_pkey PRIMARY KEY (id);


--
-- Name: relay_device_tokens relay_device_tokens_user_address_device_token_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_device_tokens
    ADD CONSTRAINT relay_device_tokens_user_address_device_token_key UNIQUE (user_address, device_token);


--
-- Name: relay_messages relay_messages_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_messages
    ADD CONSTRAINT relay_messages_pkey PRIMARY KEY (id);


--
-- Name: relay_notifications relay_notifications_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_notifications
    ADD CONSTRAINT relay_notifications_pkey PRIMARY KEY (id);


--
-- Name: relay_outbox relay_outbox_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_outbox
    ADD CONSTRAINT relay_outbox_pkey PRIMARY KEY (id);


--
-- Name: relay_user_preferences relay_user_preferences_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_user_preferences
    ADD CONSTRAINT relay_user_preferences_pkey PRIMARY KEY (user_address);


--
-- Name: relay_ws_connections relay_ws_connections_connection_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_ws_connections
    ADD CONSTRAINT relay_ws_connections_connection_id_key UNIQUE (connection_id);


--
-- Name: relay_ws_connections relay_ws_connections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.relay_ws_connections
    ADD CONSTRAINT relay_ws_connections_pkey PRIMARY KEY (id);


--
-- Name: reposts reposts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reposts
    ADD CONSTRAINT reposts_pkey PRIMARY KEY (id, "time");


--
-- Name: reward_distributions reward_distributions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reward_distributions
    ADD CONSTRAINT reward_distributions_pkey PRIMARY KEY (id, "time");


--
-- Name: social_graph_events social_graph_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_graph_events
    ADD CONSTRAINT social_graph_events_pkey PRIMARY KEY (id, created_at);


--
-- Name: social_graph_relationships social_graph_relationships_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_graph_relationships
    ADD CONSTRAINT social_graph_relationships_pkey PRIMARY KEY (id);


--
-- Name: social_graph_relationships social_graph_relationships_unique_relationship; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_graph_relationships
    ADD CONSTRAINT social_graph_relationships_unique_relationship UNIQUE (follower_address, following_address);


--
-- Name: CONSTRAINT social_graph_relationships_unique_relationship ON social_graph_relationships; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON CONSTRAINT social_graph_relationships_unique_relationship ON public.social_graph_relationships IS 'Ensures follower can only follow an account once';


--
-- Name: social_proof_of_truth social_proof_of_truth_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.social_proof_of_truth
    ADD CONSTRAINT social_proof_of_truth_pkey PRIMARY KEY (id, "time");


--
-- Name: spot_bets spot_bets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_bets
    ADD CONSTRAINT spot_bets_pkey PRIMARY KEY (id, "time");


--
-- Name: spot_config spot_config_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_config
    ADD CONSTRAINT spot_config_pkey PRIMARY KEY (id, "time");


--
-- Name: spot_events spot_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_events
    ADD CONSTRAINT spot_events_pkey PRIMARY KEY (id);


--
-- Name: spot_payouts spot_payouts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_payouts
    ADD CONSTRAINT spot_payouts_pkey PRIMARY KEY (id, "time");


--
-- Name: spot_records spot_records_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_records
    ADD CONSTRAINT spot_records_pkey PRIMARY KEY (id);


--
-- Name: spot_records spot_records_post_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_records
    ADD CONSTRAINT spot_records_post_id_key UNIQUE (post_id);


--
-- Name: spot_refunds spot_refunds_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_refunds
    ADD CONSTRAINT spot_refunds_pkey PRIMARY KEY (id, "time");


--
-- Name: spot_resolutions spot_resolutions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.spot_resolutions
    ADD CONSTRAINT spot_resolutions_pkey PRIMARY KEY (id, "time");


--
-- Name: tips tips_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tips
    ADD CONSTRAINT tips_pkey PRIMARY KEY (id, "time");


--
-- Name: token_exchange_config token_exchange_config_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.token_exchange_config
    ADD CONSTRAINT token_exchange_config_pkey PRIMARY KEY (id);


--
-- Name: token_exchange_events token_exchange_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.token_exchange_events
    ADD CONSTRAINT token_exchange_events_pkey PRIMARY KEY (id);


--
-- Name: token_exchange_events unique_event_id; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.token_exchange_events
    ADD CONSTRAINT unique_event_id UNIQUE (event_id);


--
-- Name: vesting_wallets vesting_wallets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vesting_wallets
    ADD CONSTRAINT vesting_wallets_pkey PRIMARY KEY (wallet_id);


--
-- Name: vote_decryption_failures vote_decryption_failures_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vote_decryption_failures
    ADD CONSTRAINT vote_decryption_failures_pkey PRIMARY KEY (id, "time");


--
-- Name: watermarks watermarks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.watermarks
    ADD CONSTRAINT watermarks_pkey PRIMARY KEY (id);


--
-- Name: _hyper_1_36_chunk_idx_social_graph_events_created_at; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_1_36_chunk_idx_social_graph_events_created_at ON _timescaledb_internal._hyper_1_36_chunk USING btree (created_at);


--
-- Name: _hyper_1_36_chunk_idx_social_graph_events_event_id; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_1_36_chunk_idx_social_graph_events_event_id ON _timescaledb_internal._hyper_1_36_chunk USING btree (event_id);


--
-- Name: _hyper_1_36_chunk_idx_social_graph_events_event_type; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_1_36_chunk_idx_social_graph_events_event_type ON _timescaledb_internal._hyper_1_36_chunk USING btree (event_type);


--
-- Name: _hyper_1_40_chunk_idx_social_graph_events_created_at; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_1_40_chunk_idx_social_graph_events_created_at ON _timescaledb_internal._hyper_1_40_chunk USING btree (created_at);


--
-- Name: _hyper_1_40_chunk_idx_social_graph_events_event_id; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_1_40_chunk_idx_social_graph_events_event_id ON _timescaledb_internal._hyper_1_40_chunk USING btree (event_id);


--
-- Name: _hyper_1_40_chunk_idx_social_graph_events_event_type; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_1_40_chunk_idx_social_graph_events_event_type ON _timescaledb_internal._hyper_1_40_chunk USING btree (event_type);


--
-- Name: _hyper_3_35_chunk_idx_profile_events_created_at; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_3_35_chunk_idx_profile_events_created_at ON _timescaledb_internal._hyper_3_35_chunk USING btree (created_at);


--
-- Name: _hyper_3_35_chunk_idx_profile_events_event_id; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_3_35_chunk_idx_profile_events_event_id ON _timescaledb_internal._hyper_3_35_chunk USING btree (event_id);


--
-- Name: _hyper_3_35_chunk_idx_profile_events_event_type; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_3_35_chunk_idx_profile_events_event_type ON _timescaledb_internal._hyper_3_35_chunk USING btree (event_type);


--
-- Name: _hyper_3_35_chunk_idx_profile_events_profile_id; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_3_35_chunk_idx_profile_events_profile_id ON _timescaledb_internal._hyper_3_35_chunk USING btree (profile_id);


--
-- Name: _hyper_5_38_chunk_idx_platform_events_created_at; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_5_38_chunk_idx_platform_events_created_at ON _timescaledb_internal._hyper_5_38_chunk USING btree (created_at);


--
-- Name: _hyper_5_38_chunk_idx_platform_events_platform_id; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_5_38_chunk_idx_platform_events_platform_id ON _timescaledb_internal._hyper_5_38_chunk USING btree (platform_id);


--
-- Name: _hyper_5_38_chunk_idx_platform_events_reasoning; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_5_38_chunk_idx_platform_events_reasoning ON _timescaledb_internal._hyper_5_38_chunk USING gin (to_tsvector('english'::regconfig, reasoning)) WHERE (reasoning IS NOT NULL);


--
-- Name: _hyper_5_39_chunk_idx_platform_events_created_at; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_5_39_chunk_idx_platform_events_created_at ON _timescaledb_internal._hyper_5_39_chunk USING btree (created_at);


--
-- Name: _hyper_5_39_chunk_idx_platform_events_platform_id; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_5_39_chunk_idx_platform_events_platform_id ON _timescaledb_internal._hyper_5_39_chunk USING btree (platform_id);


--
-- Name: _hyper_5_39_chunk_idx_platform_events_reasoning; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_5_39_chunk_idx_platform_events_reasoning ON _timescaledb_internal._hyper_5_39_chunk USING gin (to_tsvector('english'::regconfig, reasoning)) WHERE (reasoning IS NOT NULL);


--
-- Name: _hyper_7_4_chunk__materialized_hypertable_7_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_7_4_chunk__materialized_hypertable_7_day_idx ON _timescaledb_internal._hyper_7_4_chunk USING btree (day DESC);


--
-- Name: _hyper_7_4_chunk__materialized_hypertable_7_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_7_4_chunk__materialized_hypertable_7_event_type_day_idx ON _timescaledb_internal._hyper_7_4_chunk USING btree (event_type, day DESC);


--
-- Name: _hyper_7_7_chunk__materialized_hypertable_7_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_7_7_chunk__materialized_hypertable_7_day_idx ON _timescaledb_internal._hyper_7_7_chunk USING btree (day DESC);


--
-- Name: _hyper_7_7_chunk__materialized_hypertable_7_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_7_7_chunk__materialized_hypertable_7_event_type_day_idx ON _timescaledb_internal._hyper_7_7_chunk USING btree (event_type, day DESC);


--
-- Name: _hyper_8_12_chunk__materialized_hypertable_8_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_8_12_chunk__materialized_hypertable_8_day_idx ON _timescaledb_internal._hyper_8_12_chunk USING btree (day DESC);


--
-- Name: _hyper_8_12_chunk__materialized_hypertable_8_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_8_12_chunk__materialized_hypertable_8_event_type_day_idx ON _timescaledb_internal._hyper_8_12_chunk USING btree (event_type, day DESC);


--
-- Name: _hyper_9_8_chunk__materialized_hypertable_9_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_9_8_chunk__materialized_hypertable_9_day_idx ON _timescaledb_internal._hyper_9_8_chunk USING btree (day DESC);


--
-- Name: _hyper_9_8_chunk__materialized_hypertable_9_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _hyper_9_8_chunk__materialized_hypertable_9_event_type_day_idx ON _timescaledb_internal._hyper_9_8_chunk USING btree (event_type, day DESC);


--
-- Name: _materialized_hypertable_106_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_106_day_idx ON _timescaledb_internal._materialized_hypertable_106 USING btree (day DESC);


--
-- Name: _materialized_hypertable_107_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_107_hour_idx ON _timescaledb_internal._materialized_hypertable_107 USING btree (hour DESC);


--
-- Name: _materialized_hypertable_112_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_112_day_idx ON _timescaledb_internal._materialized_hypertable_112 USING btree (day DESC);


--
-- Name: _materialized_hypertable_112_profile_owner_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_112_profile_owner_day_idx ON _timescaledb_internal._materialized_hypertable_112 USING btree (profile_owner, day DESC);


--
-- Name: _materialized_hypertable_112_revenue_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_112_revenue_type_day_idx ON _timescaledb_internal._materialized_hypertable_112 USING btree (revenue_type, day DESC);


--
-- Name: _materialized_hypertable_112_service_id_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_112_service_id_day_idx ON _timescaledb_internal._materialized_hypertable_112 USING btree (service_id, day DESC);


--
-- Name: _materialized_hypertable_113_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_113_day_idx ON _timescaledb_internal._materialized_hypertable_113 USING btree (day DESC);


--
-- Name: _materialized_hypertable_113_service_id_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_113_service_id_day_idx ON _timescaledb_internal._materialized_hypertable_113 USING btree (service_id, day DESC);


--
-- Name: _materialized_hypertable_117_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_117_hour_idx ON _timescaledb_internal._materialized_hypertable_117 USING btree (hour DESC);


--
-- Name: _materialized_hypertable_117_service_id_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_117_service_id_hour_idx ON _timescaledb_internal._materialized_hypertable_117 USING btree (service_id, hour DESC);


--
-- Name: _materialized_hypertable_118_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_118_day_idx ON _timescaledb_internal._materialized_hypertable_118 USING btree (day DESC);


--
-- Name: _materialized_hypertable_118_service_id_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_118_service_id_day_idx ON _timescaledb_internal._materialized_hypertable_118 USING btree (service_id, day DESC);


--
-- Name: _materialized_hypertable_123_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_123_day_idx ON _timescaledb_internal._materialized_hypertable_123 USING btree (day DESC);


--
-- Name: _materialized_hypertable_123_proposal_id_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_123_proposal_id_day_idx ON _timescaledb_internal._materialized_hypertable_123 USING btree (proposal_id, day DESC);


--
-- Name: _materialized_hypertable_12_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_12_day_idx ON _timescaledb_internal._materialized_hypertable_12 USING btree (day DESC);


--
-- Name: _materialized_hypertable_141_creator_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_141_creator_day_idx ON _timescaledb_internal._materialized_hypertable_141 USING btree (creator, day DESC);


--
-- Name: _materialized_hypertable_141_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_141_day_idx ON _timescaledb_internal._materialized_hypertable_141 USING btree (day DESC);


--
-- Name: _materialized_hypertable_141_mydata_id_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_141_mydata_id_day_idx ON _timescaledb_internal._materialized_hypertable_141 USING btree (mydata_id, day DESC);


--
-- Name: _materialized_hypertable_141_revenue_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_141_revenue_type_day_idx ON _timescaledb_internal._materialized_hypertable_141 USING btree (revenue_type, day DESC);


--
-- Name: _materialized_hypertable_142_access_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_142_access_type_day_idx ON _timescaledb_internal._materialized_hypertable_142 USING btree (access_type, day DESC);


--
-- Name: _materialized_hypertable_142_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_142_day_idx ON _timescaledb_internal._materialized_hypertable_142 USING btree (day DESC);


--
-- Name: _materialized_hypertable_142_mydata_id_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_142_mydata_id_day_idx ON _timescaledb_internal._materialized_hypertable_142 USING btree (mydata_id, day DESC);


--
-- Name: _materialized_hypertable_143_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_143_hour_idx ON _timescaledb_internal._materialized_hypertable_143 USING btree (hour DESC);


--
-- Name: _materialized_hypertable_143_mydata_id_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_143_mydata_id_hour_idx ON _timescaledb_internal._materialized_hypertable_143 USING btree (mydata_id, hour DESC);


--
-- Name: _materialized_hypertable_31_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_31_bucket_idx ON _timescaledb_internal._materialized_hypertable_31 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_31_object_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_31_object_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_31 USING btree (object_id, bucket DESC);


--
-- Name: _materialized_hypertable_31_reaction_text_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_31_reaction_text_bucket_idx ON _timescaledb_internal._materialized_hypertable_31 USING btree (reaction_text, bucket DESC);


--
-- Name: _materialized_hypertable_32_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_32_bucket_idx ON _timescaledb_internal._materialized_hypertable_32 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_32_original_post_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_32_original_post_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_32 USING btree (original_post_id, bucket DESC);


--
-- Name: _materialized_hypertable_33_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_33_bucket_idx ON _timescaledb_internal._materialized_hypertable_33 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_33_is_post_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_33_is_post_bucket_idx ON _timescaledb_internal._materialized_hypertable_33 USING btree (is_post, bucket DESC);


--
-- Name: _materialized_hypertable_33_object_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_33_object_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_33 USING btree (object_id, bucket DESC);


--
-- Name: _materialized_hypertable_34_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_34_bucket_idx ON _timescaledb_internal._materialized_hypertable_34 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_49_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_49_day_idx ON _timescaledb_internal._materialized_hypertable_49 USING btree (day DESC);


--
-- Name: _materialized_hypertable_49_registry_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_49_registry_type_day_idx ON _timescaledb_internal._materialized_hypertable_49 USING btree (registry_type, day DESC);


--
-- Name: _materialized_hypertable_49_target_address_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_49_target_address_day_idx ON _timescaledb_internal._materialized_hypertable_49 USING btree (target_address, day DESC);


--
-- Name: _materialized_hypertable_50_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_50_hour_idx ON _timescaledb_internal._materialized_hypertable_50 USING btree (hour DESC);


--
-- Name: _materialized_hypertable_50_proposal_id_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_50_proposal_id_hour_idx ON _timescaledb_internal._materialized_hypertable_50 USING btree (proposal_id, hour DESC);


--
-- Name: _materialized_hypertable_51_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_51_hour_idx ON _timescaledb_internal._materialized_hypertable_51 USING btree (hour DESC);


--
-- Name: _materialized_hypertable_51_proposal_id_hour_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_51_proposal_id_hour_idx ON _timescaledb_internal._materialized_hypertable_51 USING btree (proposal_id, hour DESC);


--
-- Name: _materialized_hypertable_52_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_52_day_idx ON _timescaledb_internal._materialized_hypertable_52 USING btree (day DESC);


--
-- Name: _materialized_hypertable_52_distribution_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_52_distribution_type_day_idx ON _timescaledb_internal._materialized_hypertable_52 USING btree (distribution_type, day DESC);


--
-- Name: _materialized_hypertable_73_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_73_bucket_idx ON _timescaledb_internal._materialized_hypertable_73 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_73_pool_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_73_pool_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_73 USING btree (pool_id, bucket DESC);


--
-- Name: _materialized_hypertable_74_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_74_bucket_idx ON _timescaledb_internal._materialized_hypertable_74 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_74_pool_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_74_pool_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_74 USING btree (pool_id, bucket DESC);


--
-- Name: _materialized_hypertable_7_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_7_day_idx ON _timescaledb_internal._materialized_hypertable_7 USING btree (day DESC);


--
-- Name: _materialized_hypertable_7_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_7_event_type_day_idx ON _timescaledb_internal._materialized_hypertable_7 USING btree (event_type, day DESC);


--
-- Name: _materialized_hypertable_83_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_83_bucket_idx ON _timescaledb_internal._materialized_hypertable_83 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_83_platform_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_83_platform_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_83 USING btree (platform_id, bucket DESC);


--
-- Name: _materialized_hypertable_83_post_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_83_post_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_83 USING btree (post_id, bucket DESC);


--
-- Name: _materialized_hypertable_83_promotion_id_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_83_promotion_id_bucket_idx ON _timescaledb_internal._materialized_hypertable_83 USING btree (promotion_id, bucket DESC);


--
-- Name: _materialized_hypertable_84_bucket_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_84_bucket_idx ON _timescaledb_internal._materialized_hypertable_84 USING btree (bucket DESC);


--
-- Name: _materialized_hypertable_8_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_8_day_idx ON _timescaledb_internal._materialized_hypertable_8 USING btree (day DESC);


--
-- Name: _materialized_hypertable_8_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_8_event_type_day_idx ON _timescaledb_internal._materialized_hypertable_8 USING btree (event_type, day DESC);


--
-- Name: _materialized_hypertable_9_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_9_day_idx ON _timescaledb_internal._materialized_hypertable_9 USING btree (day DESC);


--
-- Name: _materialized_hypertable_9_event_type_day_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS _materialized_hypertable_9_event_type_day_idx ON _timescaledb_internal._materialized_hypertable_9 USING btree (event_type, day DESC);


--
-- Name: compress_hyper_4_37_chunk_event_type__ts_meta_min_1__ts_met_idx; Type: INDEX; Schema: _timescaledb_internal; Owner: -
--

CREATE INDEX IF NOT EXISTS compress_hyper_4_37_chunk_event_type__ts_meta_min_1__ts_met_idx ON _timescaledb_internal.compress_hyper_4_37_chunk USING btree (event_type, _ts_meta_min_1, _ts_meta_max_1, _ts_meta_min_2 DESC, _ts_meta_max_2 DESC, _ts_meta_min_3, _ts_meta_max_3, _ts_meta_min_4, _ts_meta_max_4);


--
-- Name: idx_anonymous_votes_decrypted_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_anonymous_votes_decrypted_time ON public.anonymous_votes USING btree (decrypted, "time" DESC) WHERE (decrypted = true);


--
-- Name: idx_anonymous_votes_proposal_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_anonymous_votes_proposal_time ON public.anonymous_votes USING btree (proposal_id, "time" DESC);


--
-- Name: idx_anonymous_votes_status_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_anonymous_votes_status_time ON public.anonymous_votes USING btree (decryption_status, "time" DESC);


--
-- Name: idx_anonymous_votes_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_anonymous_votes_transaction_id ON public.anonymous_votes USING btree (transaction_id);


--
-- Name: idx_anonymous_votes_unique_vote; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_anonymous_votes_unique_vote ON public.anonymous_votes USING btree (proposal_id, voter_address, "time");


--
-- Name: idx_anonymous_votes_voter_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_anonymous_votes_voter_time ON public.anonymous_votes USING btree (voter_address, "time" DESC);


--
-- Name: idx_blocked_events_blocked; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_events_blocked ON public.blocked_events USING btree (blocked_address);


--
-- Name: idx_blocked_events_blocker; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_events_blocker ON public.blocked_events USING btree (blocker_address);


--
-- Name: idx_blocked_events_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_events_created_at ON public.blocked_events USING btree (created_at);


--
-- Name: idx_blocked_events_event_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_events_event_id ON public.blocked_events USING btree (event_id);


--
-- Name: idx_blocked_events_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_events_type ON public.blocked_events USING btree (event_type);


--
-- Name: idx_blocked_profiles_block_list; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_profiles_block_list ON public.blocked_profiles USING btree (block_list_address);


--
-- Name: idx_blocked_profiles_blocked; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_profiles_blocked ON public.blocked_profiles USING btree (blocked_address);


--
-- Name: idx_blocked_profiles_blocker; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_profiles_blocker ON public.blocked_profiles USING btree (blocker_address, last_blocked_at DESC);


--
-- Name: idx_blocked_profiles_pagination; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_profiles_pagination ON public.blocked_profiles USING btree (blocker_address, id);


--
-- Name: idx_blocked_profiles_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_profiles_profile_id ON public.blocked_profiles USING btree (blocked_profile_id);


--
-- Name: idx_blocked_profiles_username; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_blocked_profiles_username ON public.blocked_profiles USING btree (blocked_username);


--
-- Name: idx_checkpoint_processing_start_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_checkpoint_processing_start_time ON public.checkpoint_processing USING btree (processing_start_time);


--
-- Name: idx_comments_comment_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_comments_comment_id_time ON public.comments USING btree (comment_id, "time");


--
-- Name: idx_comments_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_created_at ON public.comments USING btree (created_at);


--
-- Name: idx_comments_deleted_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_deleted_at ON public.comments USING btree (deleted_at);


--
-- Name: idx_comments_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_owner ON public.comments USING btree (owner, "time");


--
-- Name: idx_comments_parent_comment_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_parent_comment_id ON public.comments USING btree (parent_comment_id, "time");


--
-- Name: idx_comments_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_post_id ON public.comments USING btree (post_id, "time");


--
-- Name: idx_comments_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_profile_id ON public.comments USING btree (profile_id, "time");


--
-- Name: idx_comments_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_comments_transaction_id ON public.comments USING btree (transaction_id);


--
-- Name: idx_community_votes_approve; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_community_votes_approve ON public.community_votes USING btree (approve, "time");


--
-- Name: idx_community_votes_proposal_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_community_votes_proposal_id ON public.community_votes USING btree (proposal_id, "time");


--
-- Name: idx_community_votes_proposal_voter_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_community_votes_proposal_voter_time ON public.community_votes USING btree (proposal_id, voter_address, "time");


--
-- Name: idx_community_votes_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_community_votes_transaction_id ON public.community_votes USING btree (transaction_id);


--
-- Name: idx_community_votes_vote_weight; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_community_votes_vote_weight ON public.community_votes USING btree (vote_weight, "time");


--
-- Name: idx_community_votes_voter_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_community_votes_voter_address ON public.community_votes USING btree (voter_address, "time");


--
-- Name: idx_daily_license_revenue_bucket; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_daily_license_revenue_bucket ON public.daily_license_revenue USING btree (bucket);


--
-- Name: idx_daily_license_revenue_license_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_daily_license_revenue_license_id ON public.daily_license_revenue USING btree (license_id);


--
-- Name: idx_decryption_failures_proposal_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_decryption_failures_proposal_time ON public.vote_decryption_failures USING btree (proposal_id, "time" DESC);


--
-- Name: idx_decryption_failures_reason_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_decryption_failures_reason_time ON public.vote_decryption_failures USING btree (failure_reason, "time" DESC);


--
-- Name: idx_decryption_failures_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_decryption_failures_transaction_id ON public.vote_decryption_failures USING btree (transaction_id);


--
-- Name: idx_decryption_failures_voter_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_decryption_failures_voter_time ON public.vote_decryption_failures USING btree (voter_address, "time" DESC);


--
-- Name: idx_delegate_votes_approve; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegate_votes_approve ON public.delegate_votes USING btree (approve, "time");


--
-- Name: idx_delegate_votes_delegate_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegate_votes_delegate_address ON public.delegate_votes USING btree (delegate_address, "time");


--
-- Name: idx_delegate_votes_proposal_delegate_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_delegate_votes_proposal_delegate_time ON public.delegate_votes USING btree (proposal_id, delegate_address, "time");


--
-- Name: idx_delegate_votes_proposal_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegate_votes_proposal_id ON public.delegate_votes USING btree (proposal_id, "time");


--
-- Name: idx_delegate_votes_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegate_votes_transaction_id ON public.delegate_votes USING btree (transaction_id);


--
-- Name: idx_delegates_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegates_address ON public.delegates USING btree (address, "time");


--
-- Name: idx_delegates_address_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_delegates_address_type_time ON public.delegates USING btree (address, registry_type, "time");


--
-- Name: idx_delegates_is_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegates_is_active ON public.delegates USING btree (is_active, "time");


--
-- Name: idx_delegates_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegates_profile_id ON public.delegates USING btree (profile_id, "time");


--
-- Name: idx_delegates_registry_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegates_registry_type ON public.delegates USING btree (registry_type, "time");


--
-- Name: idx_delegates_term_end; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegates_term_end ON public.delegates USING btree (term_end, "time");


--
-- Name: idx_delegates_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_delegates_transaction_id ON public.delegates USING btree (transaction_id);


--
-- Name: idx_deletion_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_deletion_object_id ON public.posts_deletion_events USING btree (object_id, "time");


--
-- Name: idx_deletion_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_deletion_owner ON public.posts_deletion_events USING btree (owner, "time");


--
-- Name: idx_deletion_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_deletion_transaction_id ON public.posts_deletion_events USING btree (transaction_id);


--
-- Name: idx_governance_registries_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_governance_registries_transaction_id ON public.governance_registries USING btree (transaction_id);


--
-- Name: idx_governance_registries_type; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_governance_registries_type ON public.governance_registries USING btree (registry_type);


--
-- Name: idx_governance_registries_updated_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_governance_registries_updated_at ON public.governance_registries USING btree (updated_at);


--
-- Name: idx_indexer_progress_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_indexer_progress_id ON public.indexer_progress USING btree (id);


--
-- Name: idx_moderation_moderated_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_moderation_moderated_by ON public.posts_moderation_events USING btree (moderated_by, "time");


--
-- Name: idx_moderation_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_moderation_object_id ON public.posts_moderation_events USING btree (object_id, "time");


--
-- Name: idx_moderation_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_moderation_platform_id ON public.posts_moderation_events USING btree (platform_id, "time");


--
-- Name: idx_moderation_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_moderation_transaction_id ON public.posts_moderation_events USING btree (transaction_id);


--
-- Name: idx_my_ip_creation_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_creation_time ON public.my_ip USING btree (creation_time);


--
-- Name: idx_my_ip_creator; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_creator ON public.my_ip USING btree (creator);


--
-- Name: idx_my_ip_events_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_events_created_at ON public.my_ip_events USING btree (created_at);


--
-- Name: idx_my_ip_events_created_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_events_created_by ON public.my_ip_events USING btree (created_by);


--
-- Name: idx_my_ip_events_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_events_event_type ON public.my_ip_events USING btree (event_type);


--
-- Name: idx_my_ip_events_license_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_events_license_id ON public.my_ip_events USING btree (license_id);


--
-- Name: idx_my_ip_grants_grant_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_grants_grant_time ON public.my_ip_grants USING btree (grant_time);


--
-- Name: idx_my_ip_grants_grantee; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_grants_grantee ON public.my_ip_grants USING btree (grantee);


--
-- Name: idx_my_ip_grants_grantor; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_grants_grantor ON public.my_ip_grants USING btree (grantor);


--
-- Name: idx_my_ip_grants_license_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_grants_license_id ON public.my_ip_grants USING btree (license_id);


--
-- Name: idx_my_ip_license_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_license_id ON public.my_ip USING btree (license_id);


--
-- Name: idx_my_ip_license_state; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_license_state ON public.my_ip USING btree (license_state);


--
-- Name: idx_my_ip_license_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_license_type ON public.my_ip USING btree (license_type);


--
-- Name: idx_my_ip_revenue_from_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_from_address ON public.my_ip_revenue USING btree (from_address);


--
-- Name: idx_my_ip_revenue_license_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_license_id ON public.my_ip_revenue USING btree (license_id);


--
-- Name: idx_my_ip_revenue_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_post_id ON public.my_ip_revenue USING btree (post_id);


--
-- Name: idx_my_ip_revenue_recipient; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_recipient ON public.my_ip USING btree (revenue_recipient);


--
-- Name: idx_my_ip_revenue_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_time ON public.my_ip_revenue USING btree (revenue_time);


--
-- Name: idx_my_ip_revenue_to_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_to_address ON public.my_ip_revenue USING btree (to_address);


--
-- Name: idx_mydata_access_time_mydata; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_access_time_mydata ON public.mydata_access_logs USING btree ("time" DESC, mydata_id);


--
-- Name: idx_mydata_access_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_access_type_time ON public.mydata_access_logs USING btree (access_type, "time" DESC);


--
-- Name: idx_mydata_access_user_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_access_user_time ON public.mydata_access_logs USING btree (user_address, "time" DESC);


--
-- Name: idx_mydata_data_geographic; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_geographic ON public.mydata_data USING btree (geographic_region) WHERE (geographic_region IS NOT NULL);


--
-- Name: idx_mydata_data_media_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_media_type ON public.mydata_data USING btree (media_type);


--
-- Name: idx_mydata_data_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_owner ON public.mydata_data USING btree (owner);


--
-- Name: idx_mydata_data_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_platform_id ON public.mydata_data USING btree (platform_id) WHERE (platform_id IS NOT NULL);


--
-- Name: idx_mydata_data_pricing; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_pricing ON public.mydata_data USING btree (one_time_price, subscription_price) WHERE ((one_time_price IS NOT NULL) OR (subscription_price IS NOT NULL));


--
-- Name: idx_mydata_data_quality; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_quality ON public.mydata_data USING btree (data_quality) WHERE (data_quality IS NOT NULL);


--
-- Name: idx_mydata_data_tags; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_tags ON public.mydata_data USING gin (tags);


--
-- Name: idx_mydata_data_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_data_time ON public.mydata_data USING btree ("time" DESC);


--
-- Name: idx_mydata_purchases_buyer_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_purchases_buyer_time ON public.mydata_purchases USING btree (buyer, "time" DESC);


--
-- Name: idx_mydata_purchases_mydata_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_purchases_mydata_time ON public.mydata_purchases USING btree (mydata_id, "time" DESC);


--
-- Name: idx_mydata_purchases_time_mydata; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_purchases_time_mydata ON public.mydata_purchases USING btree ("time" DESC, mydata_id);


--
-- Name: idx_mydata_purchases_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_purchases_type_time ON public.mydata_purchases USING btree (purchase_type, "time" DESC);


--
-- Name: idx_mydata_registry_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_registry_active ON public.mydata_registry USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_mydata_registry_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_registry_owner ON public.mydata_registry USING btree (owner);


--
-- Name: idx_mydata_registry_registered_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_registry_registered_at ON public.mydata_registry USING btree (registered_at DESC);


--
-- Name: idx_mydata_registry_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_registry_transaction_id ON public.mydata_registry USING btree (transaction_id);


--
-- Name: idx_mydata_revenue_mydata_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_revenue_mydata_time ON public.mydata_revenue USING btree (mydata_id, "time" DESC);


--
-- Name: idx_mydata_revenue_time_mydata; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_revenue_time_mydata ON public.mydata_revenue USING btree ("time" DESC, mydata_id);


--
-- Name: idx_mydata_revenue_to_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_revenue_to_time ON public.mydata_revenue USING btree (to_address, "time" DESC);


--
-- Name: idx_mydata_revenue_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_revenue_type_time ON public.mydata_revenue USING btree (revenue_type, "time" DESC);


--
-- Name: idx_mydata_subscriptions_end_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_end_time ON public.mydata_subscriptions USING btree (subscription_end, "time" DESC);


--
-- Name: idx_mydata_subscriptions_mydata_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_mydata_time ON public.mydata_subscriptions USING btree (mydata_id, "time" DESC);


--
-- Name: idx_mydata_subscriptions_subscriber_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_subscriber_time ON public.mydata_subscriptions USING btree (subscriber, "time" DESC);


--
-- Name: idx_mydata_subscriptions_time_mydata; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_mydata_subscriptions_time_mydata ON public.mydata_subscriptions USING btree ("time" DESC, mydata_id);


--
-- Name: idx_nominees_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_nominees_address ON public.nominated_delegates USING btree (address, "time");


--
-- Name: idx_nominees_address_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_nominees_address_type_time ON public.nominated_delegates USING btree (address, registry_type, "time");


--
-- Name: idx_nominees_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_nominees_profile_id ON public.nominated_delegates USING btree (profile_id, "time");


--
-- Name: idx_nominees_registry_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_nominees_registry_type ON public.nominated_delegates USING btree (registry_type, "time");


--
-- Name: idx_nominees_scheduled_term; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_nominees_scheduled_term ON public.nominated_delegates USING btree (scheduled_term_start_epoch, "time");


--
-- Name: idx_nominees_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_nominees_status ON public.nominated_delegates USING btree (status, "time");


--
-- Name: idx_nominees_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_nominees_transaction_id ON public.nominated_delegates USING btree (transaction_id);


--
-- Name: idx_platform_blocked_profiles_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_blocked_profiles_platform_id ON public.platform_blocked_profiles USING btree (platform_id);


--
-- Name: idx_platform_blocked_profiles_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_blocked_profiles_profile_id ON public.platform_blocked_profiles USING btree (profile_id);


--
-- Name: idx_platform_delivery_config_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_delivery_config_platform_id ON public.platform_delivery_config USING btree (platform_id);


--
-- Name: idx_platform_events_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_events_created_at ON public.platform_events USING btree (created_at);


--
-- Name: idx_platform_events_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_events_platform_id ON public.platform_events USING btree (platform_id);


--
-- Name: idx_platform_events_reasoning; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_events_reasoning ON public.platform_events USING gin (to_tsvector('english'::regconfig, reasoning)) WHERE (reasoning IS NOT NULL);


--
-- Name: idx_platform_memberships_joined_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_memberships_joined_at ON public.platform_memberships USING btree (joined_at);


--
-- Name: idx_platform_memberships_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_memberships_platform_id ON public.platform_memberships USING btree (platform_id);


--
-- Name: idx_platform_memberships_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_memberships_profile_id ON public.platform_memberships USING btree (profile_id);


--
-- Name: idx_platform_moderators_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platform_moderators_platform_id ON public.platform_moderators USING btree (platform_id);


--
-- Name: idx_platforms_name; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platforms_name ON public.platforms USING btree (name);


--
-- Name: idx_platforms_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_platforms_platform_id ON public.platforms USING btree (platform_id);


--
-- Name: idx_poc_analysis_creator_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_analysis_creator_time ON public.poc_analysis_results USING btree (original_creator, "time" DESC) WHERE (original_creator IS NOT NULL);


--
-- Name: idx_poc_analysis_oracle_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_analysis_oracle_time ON public.poc_analysis_results USING btree (oracle_address, "time" DESC);


--
-- Name: idx_poc_analysis_reasoning; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_analysis_reasoning ON public.poc_analysis_results USING gin (to_tsvector('english'::regconfig, reasoning)) WHERE (reasoning IS NOT NULL);


--
-- Name: idx_poc_analysis_time_post; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_analysis_time_post ON public.poc_analysis_results USING btree ("time" DESC, post_id);


--
-- Name: idx_poc_badges_badge_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_poc_badges_badge_id ON public.poc_badges USING btree (badge_id, "time");


--
-- Name: idx_poc_badges_issued_by_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_badges_issued_by_time ON public.poc_badges USING btree (issued_by, "time" DESC);


--
-- Name: idx_poc_badges_time_post; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_badges_time_post ON public.poc_badges USING btree ("time" DESC, post_id);


--
-- Name: idx_poc_config_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_config_time ON public.poc_configuration USING btree ("time" DESC);


--
-- Name: idx_poc_disputes_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_poc_disputes_id ON public.poc_disputes USING btree (dispute_id, "time");


--
-- Name: idx_poc_disputes_post_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_disputes_post_time ON public.poc_disputes USING btree (post_id, "time" DESC);


--
-- Name: idx_poc_disputes_time_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_disputes_time_status ON public.poc_disputes USING btree ("time" DESC, status);


--
-- Name: idx_poc_redirections_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_poc_redirections_id ON public.poc_revenue_redirections USING btree (redirection_id, "time");


--
-- Name: idx_poc_redirections_original_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_redirections_original_time ON public.poc_revenue_redirections USING btree (original_post_id, "time" DESC);


--
-- Name: idx_poc_redirections_time_accused; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_redirections_time_accused ON public.poc_revenue_redirections USING btree ("time" DESC, accused_post_id);


--
-- Name: idx_poc_votes_dispute_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_votes_dispute_time ON public.poc_dispute_votes USING btree (dispute_id, "time" DESC);


--
-- Name: idx_poc_votes_dispute_voter; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_poc_votes_dispute_voter ON public.poc_dispute_votes USING btree (dispute_id, voter, "time");


--
-- Name: idx_poc_votes_time_voter; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_poc_votes_time_voter ON public.poc_dispute_votes USING btree ("time" DESC, voter);


--
-- Name: idx_post_prediction_config_predictions_enabled; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_post_prediction_config_predictions_enabled ON public.post_prediction_config USING btree (predictions_enabled, "time");


--
-- Name: idx_post_prediction_config_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_post_prediction_config_time ON public.post_prediction_config USING btree ("time" DESC);


--
-- Name: idx_post_prediction_config_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_post_prediction_config_transaction_id ON public.post_prediction_config USING btree (transaction_id);


--
-- Name: idx_post_prediction_config_updated_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_post_prediction_config_updated_by ON public.post_prediction_config USING btree (updated_by, "time");


--
-- Name: idx_posts_auto_pool_disabled; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_auto_pool_disabled ON public.posts USING btree (auto_pool_disabled, "time") WHERE (auto_pool_disabled = true);


--
-- Name: idx_posts_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_created_at ON public.posts USING btree (created_at);


--
-- Name: idx_posts_deleted_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_deleted_at ON public.posts USING btree (deleted_at);


--
-- Name: idx_posts_my_ip_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_my_ip_id ON public.posts USING btree (my_ip_id);


--
-- Name: idx_posts_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_owner ON public.posts USING btree (owner, "time");


--
-- Name: idx_posts_poc_badge_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_poc_badge_id ON public.posts USING btree (poc_badge_id, "time") WHERE (poc_badge_id IS NOT NULL);


--
-- Name: idx_posts_post_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_posts_post_id_time ON public.posts USING btree (post_id, "time");


--
-- Name: idx_posts_post_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_post_type ON public.posts USING btree (post_type, "time");


--
-- Name: idx_posts_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_profile_id ON public.posts USING btree (profile_id, "time");


--
-- Name: idx_posts_promotion_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_promotion_id ON public.posts USING btree (promotion_id, "time");


--
-- Name: idx_posts_revenue_recipient; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_revenue_recipient ON public.posts USING btree (revenue_recipient);


--
-- Name: idx_posts_revenue_redirect_to; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_revenue_redirect_to ON public.posts USING btree (revenue_redirect_to, "time") WHERE (revenue_redirect_to IS NOT NULL);


--
-- Name: idx_posts_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_posts_transaction_id ON public.posts USING btree (transaction_id);


--
-- Name: idx_profile_badges_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_badges_active ON public.profile_badges USING btree (profile_id, badge_id, "time" DESC) WHERE (revoked = false);


--
-- Name: idx_profile_badges_badge_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_badges_badge_id_time ON public.profile_badges USING btree (badge_id, "time" DESC);


--
-- Name: idx_profile_badges_platform_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_badges_platform_id_time ON public.profile_badges USING btree (platform_id, "time" DESC);


--
-- Name: idx_profile_badges_profile_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_badges_profile_id_time ON public.profile_badges USING btree (profile_id, "time" DESC);


--
-- Name: idx_profile_badges_unique; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_profile_badges_unique ON public.profile_badges USING btree (profile_id, badge_id, "time") WHERE (revoked = false);


--
-- Name: idx_profile_events_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_events_created_at ON public.profile_events USING btree (created_at);


--
-- Name: idx_profile_events_event_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_events_event_id ON public.profile_events USING btree (event_id);


--
-- Name: idx_profile_events_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_events_event_type ON public.profile_events USING btree (event_type);


--
-- Name: idx_profile_events_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_events_profile_id ON public.profile_events USING btree (profile_id);


--
-- Name: idx_profile_offers_offeror_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_offers_offeror_time ON public.profile_offers USING btree (offeror_address, "time" DESC);


--
-- Name: idx_profile_offers_profile_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_offers_profile_id_time ON public.profile_offers USING btree (profile_id, "time" DESC);


--
-- Name: idx_profile_offers_profile_offeror_unique; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_profile_offers_profile_offeror_unique ON public.profile_offers USING btree (profile_id, offeror_address, "time") WHERE (status = 'pending'::text);


--
-- Name: idx_profile_offers_status_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_offers_status_time ON public.profile_offers USING btree (status, "time" DESC) WHERE (status = 'pending'::text);


--
-- Name: idx_profile_sale_fees_fee_recipient_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_fee_recipient_time ON public.profile_sale_fees USING btree (fee_recipient_address, "time" DESC);


--
-- Name: idx_profile_sale_fees_offeror_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_offeror_time ON public.profile_sale_fees USING btree (offeror_address, "time" DESC);


--
-- Name: idx_profile_sale_fees_previous_owner_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_previous_owner_time ON public.profile_sale_fees USING btree (previous_owner_address, "time" DESC);


--
-- Name: idx_profile_sale_fees_profile_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_sale_fees_profile_id_time ON public.profile_sale_fees USING btree (profile_id, "time" DESC);


--
-- Name: idx_profile_services_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_services_active ON public.profile_subscription_services USING btree (active) WHERE (active = true);


--
-- Name: idx_profile_services_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_services_owner ON public.profile_subscription_services USING btree (profile_owner);


--
-- Name: idx_profile_services_profile; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_services_profile ON public.profile_subscription_services USING btree (profile_id);


--
-- Name: idx_profile_subscriptions_expires; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_expires ON public.profile_subscriptions USING btree (expires_at) WHERE (cancelled_at IS NULL);


--
-- Name: idx_profile_subscriptions_expires_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_expires_at ON public.profile_subscriptions USING btree (expires_at);


--
-- Name: idx_profile_subscriptions_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_profile_subscriptions_id ON public.profile_subscriptions USING btree (subscription_id, "time");


--
-- Name: idx_profile_subscriptions_service_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_service_id ON public.profile_subscriptions USING btree (service_id);


--
-- Name: idx_profile_subscriptions_subscriber; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_subscriber ON public.profile_subscriptions USING btree (subscriber);


--
-- Name: idx_profile_subscriptions_subscriber_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_subscriber_time ON public.profile_subscriptions USING btree (subscriber, "time" DESC);


--
-- Name: idx_profile_subscriptions_time_service; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profile_subscriptions_time_service ON public.profile_subscriptions USING btree ("time" DESC, service_id);


--
-- Name: idx_profiles_block_list_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_block_list_address ON public.profiles USING btree (block_list_address);


--
-- Name: idx_profiles_blocked_blocked_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_blocked_blocked_address ON public.profiles_blocked USING btree (blocked_address);


--
-- Name: idx_profiles_blocked_blocker_wallet_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_blocked_blocker_wallet_address ON public.profiles_blocked USING btree (blocker_wallet_address);


--
-- Name: idx_profiles_followers_count; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_followers_count ON public.profiles USING btree (followers_count);


--
-- Name: idx_profiles_following_count; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_following_count ON public.profiles USING btree (following_count);


--
-- Name: idx_profiles_instagram_username; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_instagram_username ON public.profiles USING btree (instagram_username) WHERE (instagram_username IS NOT NULL);


--
-- Name: idx_profiles_min_offer_amount; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_min_offer_amount ON public.profiles USING btree (min_offer_amount) WHERE (min_offer_amount IS NOT NULL);


--
-- Name: idx_profiles_owner_address; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_profiles_owner_address ON public.profiles USING btree (owner_address);


--
-- Name: idx_profiles_owner_min_offer; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_owner_min_offer ON public.profiles USING btree (owner_address, min_offer_amount) WHERE (min_offer_amount IS NOT NULL);


--
-- Name: idx_profiles_owner_post_count; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_owner_post_count ON public.profiles USING btree (owner_address, post_count DESC);


--
-- Name: idx_profiles_paid_messaging_enabled; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_paid_messaging_enabled ON public.profiles USING btree (paid_messaging_enabled);


--
-- Name: idx_profiles_post_count; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_post_count ON public.profiles USING btree (post_count DESC);


--
-- Name: idx_profiles_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_profile_id ON public.profiles USING btree (profile_id);


--
-- Name: idx_profiles_reservation_pool_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_reservation_pool_address ON public.profiles USING btree (reservation_pool_address) WHERE (reservation_pool_address IS NOT NULL);


--
-- Name: idx_profiles_selected_badge_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_selected_badge_id ON public.profiles USING btree (selected_badge_id) WHERE (selected_badge_id IS NOT NULL);


--
-- Name: idx_profiles_social_proof_token_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_profiles_social_proof_token_address ON public.profiles USING btree (social_proof_token_address) WHERE (social_proof_token_address IS NOT NULL);


--
-- Name: idx_profiles_username; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_profiles_username ON public.profiles USING btree (username);


--
-- Name: idx_progress_store_checkpoint; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_progress_store_checkpoint ON public.progress_store USING btree (last_processed_checkpoint);


--
-- Name: idx_progress_store_errors; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_progress_store_errors ON public.progress_store USING btree (error_count, last_error_at) WHERE (error_count > 0);


--
-- Name: idx_progress_store_state; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_progress_store_state ON public.progress_store USING btree (processing_state);


--
-- Name: idx_progress_store_worker_module; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_progress_store_worker_module ON public.progress_store USING btree (worker_id, module_name);


--
-- Name: idx_promoted_posts_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promoted_posts_active ON public.promoted_posts USING btree (active, "time");


--
-- Name: idx_promoted_posts_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promoted_posts_owner ON public.promoted_posts USING btree (owner, "time");


--
-- Name: idx_promoted_posts_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promoted_posts_post_id ON public.promoted_posts USING btree (post_id, "time");


--
-- Name: idx_promoted_posts_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promoted_posts_profile_id ON public.promoted_posts USING btree (profile_id, "time");


--
-- Name: idx_promoted_posts_promotion_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_promoted_posts_promotion_id_time ON public.promoted_posts USING btree (promotion_id, "time");


--
-- Name: idx_promoted_posts_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promoted_posts_transaction_id ON public.promoted_posts USING btree (transaction_id);


--
-- Name: idx_promotion_budget_events_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_event_type ON public.promotion_budget_events USING btree (event_type, "time");


--
-- Name: idx_promotion_budget_events_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_post_id ON public.promotion_budget_events USING btree (post_id, "time");


--
-- Name: idx_promotion_budget_events_promotion_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_promotion_id ON public.promotion_budget_events USING btree (promotion_id, "time");


--
-- Name: idx_promotion_budget_events_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_transaction_id ON public.promotion_budget_events USING btree (transaction_id);


--
-- Name: idx_promotion_status_events_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_status_events_event_type ON public.promotion_status_events USING btree (event_type, "time");


--
-- Name: idx_promotion_status_events_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_status_events_post_id ON public.promotion_status_events USING btree (post_id, "time");


--
-- Name: idx_promotion_status_events_promotion_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_status_events_promotion_id ON public.promotion_status_events USING btree (promotion_id, "time");


--
-- Name: idx_promotion_status_events_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_status_events_transaction_id ON public.promotion_status_events USING btree (transaction_id);


--
-- Name: idx_promotion_status_events_triggered_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_status_events_triggered_by ON public.promotion_status_events USING btree (triggered_by, "time");


--
-- Name: idx_promotion_views_platform_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_views_platform_id ON public.promotion_views USING btree (platform_id, "time");


--
-- Name: idx_promotion_views_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_views_post_id ON public.promotion_views USING btree (post_id, "time");


--
-- Name: idx_promotion_views_post_viewer_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_promotion_views_post_viewer_time ON public.promotion_views USING btree (post_id, viewer, "time");


--
-- Name: idx_promotion_views_promotion_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_views_promotion_id ON public.promotion_views USING btree (promotion_id, "time");


--
-- Name: idx_promotion_views_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_views_transaction_id ON public.promotion_views USING btree (transaction_id);


--
-- Name: idx_promotion_views_viewer; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_promotion_views_viewer ON public.promotion_views USING btree (viewer, "time");


--
-- Name: idx_proposals_anonymous_votes; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_anonymous_votes ON public.proposals USING btree (anonymous_votes_for, anonymous_votes_against, "time");


--
-- Name: idx_proposals_pending_decryption; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_pending_decryption ON public.proposals USING btree (pending_anonymous_decryption, "time") WHERE (pending_anonymous_decryption = true);


--
-- Name: idx_proposals_proposal_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_proposal_type ON public.proposals USING btree (proposal_type, "time");


--
-- Name: idx_proposals_reference_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_reference_id ON public.proposals USING btree (reference_id, "time") WHERE (reference_id IS NOT NULL);


--
-- Name: idx_proposals_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_status ON public.proposals USING btree (status, "time");


--
-- Name: idx_proposals_submitter; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_submitter ON public.proposals USING btree (submitter, "time");


--
-- Name: idx_proposals_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_transaction_id ON public.proposals USING btree (transaction_id);


--
-- Name: idx_proposals_voting_end_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_proposals_voting_end_time ON public.proposals USING btree (voting_end_time, "time") WHERE (voting_end_time IS NOT NULL);


--
-- Name: idx_ratings_registry_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_ratings_registry_type ON public.delegate_ratings USING btree (registry_type, "time");


--
-- Name: idx_ratings_target_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_ratings_target_address ON public.delegate_ratings USING btree (target_address, "time");


--
-- Name: idx_ratings_target_voter_registry_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_ratings_target_voter_registry_time ON public.delegate_ratings USING btree (target_address, voter_address, registry_type, "time");


--
-- Name: idx_ratings_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_ratings_transaction_id ON public.delegate_ratings USING btree (transaction_id);


--
-- Name: idx_ratings_voter_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_ratings_voter_address ON public.delegate_ratings USING btree (voter_address, "time");


--
-- Name: idx_reaction_counts_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reaction_counts_object_id ON public.reaction_counts USING btree (object_id);


--
-- Name: idx_reaction_counts_object_reaction; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_reaction_counts_object_reaction ON public.reaction_counts USING btree (object_id, reaction_text);


--
-- Name: idx_reaction_counts_reaction_text; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reaction_counts_reaction_text ON public.reaction_counts USING btree (reaction_text);


--
-- Name: idx_reactions_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reactions_object_id ON public.reactions USING btree (object_id, "time");


--
-- Name: idx_reactions_object_user_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_reactions_object_user_time ON public.reactions USING btree (object_id, user_address, "time");


--
-- Name: idx_reactions_reaction_text; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reactions_reaction_text ON public.reactions USING btree (reaction_text);


--
-- Name: idx_reactions_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reactions_transaction_id ON public.reactions USING btree (transaction_id);


--
-- Name: idx_reactions_user_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reactions_user_address ON public.reactions USING btree (user_address, "time");


--
-- Name: idx_relay_conversations_participant1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_conversations_participant1 ON public.relay_conversations USING btree (participant1_address, last_message_at DESC);


--
-- Name: idx_relay_conversations_participant2; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_conversations_participant2 ON public.relay_conversations USING btree (participant2_address, last_message_at DESC);


--
-- Name: idx_relay_device_tokens_platform; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_device_tokens_platform ON public.relay_device_tokens USING btree (platform);


--
-- Name: idx_relay_device_tokens_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_device_tokens_user ON public.relay_device_tokens USING btree (user_address);


--
-- Name: idx_relay_messages_conversation; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_messages_conversation ON public.relay_messages USING btree (conversation_id, created_at DESC);


--
-- Name: idx_relay_messages_recipient; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_messages_recipient ON public.relay_messages USING btree (recipient_address, created_at DESC);


--
-- Name: idx_relay_messages_sender; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_messages_sender ON public.relay_messages USING btree (sender_address, created_at DESC);


--
-- Name: idx_relay_notifications_user_all; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_all ON public.relay_notifications USING btree (user_address, created_at DESC);


--
-- Name: idx_relay_notifications_user_platform; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_platform ON public.relay_notifications USING btree (user_address, platform_id, created_at DESC) WHERE (read_at IS NULL);


--
-- Name: idx_relay_notifications_user_platform_all; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_platform_all ON public.relay_notifications USING btree (user_address, platform_id, created_at DESC);


--
-- Name: idx_relay_notifications_user_unread; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_notifications_user_unread ON public.relay_notifications USING btree (user_address, created_at DESC) WHERE (read_at IS NULL);


--
-- Name: idx_relay_outbox_event_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_outbox_event_id ON public.relay_outbox USING btree (event_id) WHERE (event_id IS NOT NULL);


--
-- Name: idx_relay_outbox_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_outbox_transaction_id ON public.relay_outbox USING btree (transaction_id) WHERE (transaction_id IS NOT NULL);


--
-- Name: idx_relay_outbox_unprocessed; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_outbox_unprocessed ON public.relay_outbox USING btree (created_at) WHERE (processed_at IS NULL);


--
-- Name: idx_relay_ws_connections_heartbeat; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_ws_connections_heartbeat ON public.relay_ws_connections USING btree (last_heartbeat_at) WHERE (disconnected_at IS NULL);


--
-- Name: idx_relay_ws_connections_user_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_relay_ws_connections_user_active ON public.relay_ws_connections USING btree (user_address, connected_at DESC) WHERE (disconnected_at IS NULL);


--
-- Name: idx_reports_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reports_object_id ON public.posts_reports USING btree (object_id, "time");


--
-- Name: idx_reports_reporter; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reports_reporter ON public.posts_reports USING btree (reporter, "time");


--
-- Name: idx_reports_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reports_transaction_id ON public.posts_reports USING btree (transaction_id);


--
-- Name: idx_reposts_original_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reposts_original_id ON public.reposts USING btree (original_id, "time");


--
-- Name: idx_reposts_original_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reposts_original_post_id ON public.reposts USING btree (original_post_id, "time");


--
-- Name: idx_reposts_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reposts_owner ON public.reposts USING btree (owner, "time");


--
-- Name: idx_reposts_profile_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reposts_profile_id ON public.reposts USING btree (profile_id, "time");


--
-- Name: idx_reposts_repost_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_reposts_repost_id_time ON public.reposts USING btree (repost_id, "time");


--
-- Name: idx_reposts_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reposts_transaction_id ON public.reposts USING btree (transaction_id);


--
-- Name: idx_reward_amount; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reward_amount ON public.reward_distributions USING btree (amount, "time");


--
-- Name: idx_reward_distribution_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reward_distribution_type ON public.reward_distributions USING btree (distribution_type, "time");


--
-- Name: idx_reward_proposal_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reward_proposal_id ON public.reward_distributions USING btree (proposal_id, "time");


--
-- Name: idx_reward_recipient_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reward_recipient_address ON public.reward_distributions USING btree (recipient_address, "time");


--
-- Name: idx_reward_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_reward_transaction_id ON public.reward_distributions USING btree (transaction_id);


--
-- Name: idx_social_graph_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_created_at ON public.social_graph_relationships USING btree (created_at);


--
-- Name: idx_social_graph_events_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_events_created_at ON public.social_graph_events USING btree (created_at);


--
-- Name: idx_social_graph_events_event_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_events_event_id ON public.social_graph_events USING btree (event_id);


--
-- Name: idx_social_graph_events_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_events_event_type ON public.social_graph_events USING btree (event_type);


--
-- Name: idx_social_graph_follower_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_follower_address ON public.social_graph_relationships USING btree (follower_address);


--
-- Name: idx_social_graph_following_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_following_address ON public.social_graph_relationships USING btree (following_address);


--
-- Name: idx_social_graph_relationships_follower_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_relationships_follower_address ON public.social_graph_relationships USING btree (follower_address);


--
-- Name: idx_social_graph_relationships_following_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_relationships_following_address ON public.social_graph_relationships USING btree (following_address);


--
-- Name: idx_social_graph_relationships_pair; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_social_graph_relationships_pair ON public.social_graph_relationships USING btree (follower_address, following_address);


--
-- Name: idx_spot_bets_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_bets_created_at ON public.spot_bets USING btree (timestamp_epoch);


--
-- Name: idx_spot_bets_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_bets_post_id ON public.spot_bets USING btree (post_id, "time");


--
-- Name: idx_spot_bets_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_bets_user ON public.spot_bets USING btree (user_address, "time");


--
-- Name: idx_spot_config_enable_flag; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_config_enable_flag ON public.spot_config USING btree (enable_flag, "time");


--
-- Name: idx_spot_config_time; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_spot_config_time ON public.spot_config USING btree ("time" DESC);


--
-- Name: idx_spot_config_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_config_transaction_id ON public.spot_config USING btree (transaction_id);


--
-- Name: idx_spot_config_updated_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_config_updated_by ON public.spot_config USING btree (updated_by, "time");


--
-- Name: idx_spot_events_post; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_events_post ON public.spot_events USING btree (post_id);


--
-- Name: idx_spot_events_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_events_type ON public.spot_events USING btree (event_type);


--
-- Name: idx_spot_payouts_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_payouts_post_id ON public.spot_payouts USING btree (post_id, "time");


--
-- Name: idx_spot_payouts_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_payouts_user ON public.spot_payouts USING btree (user_address, "time");


--
-- Name: idx_spot_records_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_records_post_id ON public.spot_records USING btree (post_id);


--
-- Name: idx_spot_records_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_records_status ON public.spot_records USING btree (status);


--
-- Name: idx_spot_refunds_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_refunds_post_id ON public.spot_refunds USING btree (post_id, "time");


--
-- Name: idx_spot_refunds_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_refunds_user ON public.spot_refunds USING btree (user_address, "time");


--
-- Name: idx_spot_resolutions_post_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_resolutions_post_id ON public.spot_resolutions USING btree (post_id, "time");


--
-- Name: idx_spot_resolutions_reasoning; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_resolutions_reasoning ON public.spot_resolutions USING gin (to_tsvector('english'::regconfig, reasoning)) WHERE (reasoning <> ''::text);


--
-- Name: idx_spot_unified_post; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_unified_post ON public.social_proof_of_truth USING btree (post_id, "time");


--
-- Name: idx_spot_unified_tx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_unified_tx ON public.social_proof_of_truth USING btree (transaction_id);


--
-- Name: idx_spot_unified_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_unified_type ON public.social_proof_of_truth USING btree (event_type, "time");


--
-- Name: idx_spot_unified_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spot_unified_user ON public.social_proof_of_truth USING btree (user_address, "time");


--
-- Name: idx_spt_exchange_config_updated_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_exchange_config_updated_by ON public.spt_exchange_config USING btree (updated_by);


--
-- Name: idx_spt_holdings_holder_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_holdings_holder_address ON public.spt_holdings USING btree (holder_address);


--
-- Name: idx_spt_holdings_pool_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_holdings_pool_id ON public.spt_holdings USING btree (pool_id);


--
-- Name: idx_spt_pools_associated_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_pools_associated_id ON public.social_proof_token_pools USING btree (associated_id);


--
-- Name: idx_spt_pools_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_pools_owner ON public.social_proof_token_pools USING btree (owner);


--
-- Name: idx_spt_pools_pool_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_pools_pool_id ON public.social_proof_token_pools USING btree (pool_id);


--
-- Name: idx_spt_pools_token_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_pools_token_type ON public.social_proof_token_pools USING btree (token_type);


--
-- Name: idx_spt_price_history_pool_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_price_history_pool_id ON public.spt_price_history USING btree (pool_id);


--
-- Name: idx_spt_reservation_pools_associated_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_associated_id ON public.spt_reservation_pools USING btree (associated_id);


--
-- Name: idx_spt_reservation_pools_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_owner ON public.spt_reservation_pools USING btree (owner);


--
-- Name: idx_spt_reservation_pools_pool_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_pool_id ON public.spt_reservation_pools USING btree (pool_id);


--
-- Name: idx_spt_reservation_pools_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_status ON public.spt_reservation_pools USING btree (status);


--
-- Name: idx_spt_reservation_pools_token_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_token_type ON public.spt_reservation_pools USING btree (token_type);


--
-- Name: idx_spt_reservations_pool_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservations_pool_id ON public.spt_reservations USING btree (pool_id);


--
-- Name: idx_spt_reservations_reservatior_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_reservations_reservatior_address ON public.spt_reservations USING btree (reservatior_address);


--
-- Name: idx_spt_revenue_creator_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_revenue_creator_time ON public.spt_revenue USING btree (creator_address, "time" DESC);


--
-- Name: idx_spt_revenue_platform_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_revenue_platform_time ON public.spt_revenue USING btree (platform_address, "time" DESC);


--
-- Name: idx_spt_revenue_pool_time_fees; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_revenue_pool_time_fees ON public.spt_revenue USING btree (pool_id, "time" DESC, total_fee DESC);


--
-- Name: idx_spt_revenue_time_pool; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_revenue_time_pool ON public.spt_revenue USING btree ("time" DESC, pool_id);


--
-- Name: idx_spt_revenue_trader_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_revenue_trader_time ON public.spt_revenue USING btree (trader, "time" DESC);


--
-- Name: idx_spt_revenue_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_revenue_type_time ON public.spt_revenue USING btree (transaction_type, "time" DESC);


--
-- Name: idx_spt_transactions_pool_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_transactions_pool_id ON public.spt_transactions USING btree (pool_id);


--
-- Name: idx_spt_transactions_sender; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_transactions_sender ON public.spt_transactions USING btree (sender);


--
-- Name: idx_spt_transactions_transaction_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_spt_transactions_transaction_type ON public.spt_transactions USING btree (transaction_type);


--
-- Name: idx_subscription_access_content_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_access_content_time ON public.subscription_access_logs USING btree (content_id, "time" DESC);


--
-- Name: idx_subscription_access_subscriber_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_access_subscriber_time ON public.subscription_access_logs USING btree (subscriber, "time" DESC);


--
-- Name: idx_subscription_access_time_sub; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_access_time_sub ON public.subscription_access_logs USING btree ("time" DESC, subscription_id);


--
-- Name: idx_subscription_events_service_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_events_service_time ON public.subscription_events USING btree (service_id, "time" DESC) WHERE (service_id IS NOT NULL);


--
-- Name: idx_subscription_events_subscription_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_events_subscription_time ON public.subscription_events USING btree (subscription_id, "time" DESC) WHERE (subscription_id IS NOT NULL);


--
-- Name: idx_subscription_events_time_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_events_time_type ON public.subscription_events USING btree ("time" DESC, event_type);


--
-- Name: idx_subscription_revenue_service_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_revenue_service_id ON public.subscription_revenue USING btree (service_id);


--
-- Name: idx_subscription_revenue_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_revenue_time ON public.subscription_revenue USING btree ("time");


--
-- Name: idx_subscription_revenue_time_service; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_revenue_time_service ON public.subscription_revenue USING btree ("time" DESC, service_id);


--
-- Name: idx_subscription_revenue_to_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_revenue_to_time ON public.subscription_revenue USING btree (to_address, "time" DESC);


--
-- Name: idx_subscription_revenue_type_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_subscription_revenue_type_time ON public.subscription_revenue USING btree (revenue_type, "time" DESC);


--
-- Name: idx_tips_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_tips_object_id ON public.tips USING btree (object_id, "time");


--
-- Name: idx_tips_recipient; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_tips_recipient ON public.tips USING btree (recipient, "time");


--
-- Name: idx_tips_tipper; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_tips_tipper ON public.tips USING btree (tipper, "time");


--
-- Name: idx_tips_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_tips_transaction_id ON public.tips USING btree (transaction_id);


--
-- Name: idx_token_exchange_config_trading_halted; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_token_exchange_config_trading_halted ON public.token_exchange_config USING btree (trading_halted);


--
-- Name: idx_token_exchange_config_updated_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_token_exchange_config_updated_at ON public.token_exchange_config USING btree (updated_at DESC);


--
-- Name: idx_token_exchange_events_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_token_exchange_events_created_at ON public.token_exchange_events USING btree (created_at DESC);


--
-- Name: idx_token_exchange_events_event_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_token_exchange_events_event_id ON public.token_exchange_events USING btree (event_id);


--
-- Name: idx_token_exchange_events_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_token_exchange_events_type ON public.token_exchange_events USING btree (event_type);


--
-- Name: idx_transfers_new_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_transfers_new_owner ON public.posts_transfers USING btree (new_owner, "time");


--
-- Name: idx_transfers_object_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_transfers_object_id ON public.posts_transfers USING btree (object_id, "time");


--
-- Name: idx_transfers_previous_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_transfers_previous_owner ON public.posts_transfers USING btree (previous_owner, "time");


--
-- Name: idx_transfers_transaction_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_transfers_transaction_id ON public.posts_transfers USING btree (transaction_id);


--
-- Name: idx_unified_revenue_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_content ON public.unified_revenue USING btree (content_id, content_type, "time" DESC) WHERE (content_id IS NOT NULL);


--
-- Name: idx_unified_revenue_creator_source_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_creator_source_time ON public.unified_revenue USING btree (creator_address, revenue_source, "time" DESC);


--
-- Name: idx_unified_revenue_creator_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_creator_time ON public.unified_revenue USING btree (creator_address, "time" DESC);


--
-- Name: idx_unified_revenue_payer_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_payer_time ON public.unified_revenue USING btree (payer_address, "time" DESC);


--
-- Name: idx_unified_revenue_platform_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_platform_time ON public.unified_revenue USING btree (platform_address, "time" DESC) WHERE (platform_address IS NOT NULL);


--
-- Name: idx_unified_revenue_source_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_source_type ON public.unified_revenue USING btree (revenue_source, revenue_type, "time" DESC);


--
-- Name: idx_unified_revenue_time_amount; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_time_amount ON public.unified_revenue USING btree ("time" DESC, amount DESC);


--
-- Name: idx_unified_revenue_time_source; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unified_revenue_time_source ON public.unified_revenue USING btree ("time" DESC, revenue_source);


--
-- Name: idx_unique_license_id_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_unique_license_id_time ON public.my_ip USING btree (license_id, "time");


--
-- Name: idx_vesting_events_event_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_events_event_time ON public.vesting_events USING btree (event_time);


--
-- Name: idx_vesting_events_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_events_event_type ON public.vesting_events USING btree (event_type);


--
-- Name: idx_vesting_events_owner_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_events_owner_address ON public.vesting_events USING btree (owner_address);


--
-- Name: idx_vesting_events_wallet_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_events_wallet_id ON public.vesting_events USING btree (wallet_id);


--
-- Name: idx_vesting_wallets_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_wallets_created_at ON public.vesting_wallets USING btree (created_at);


--
-- Name: idx_vesting_wallets_curve_factor; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_wallets_curve_factor ON public.vesting_wallets USING btree (curve_factor);


--
-- Name: idx_vesting_wallets_owner_address; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_wallets_owner_address ON public.vesting_wallets USING btree (owner_address);


--
-- Name: idx_vesting_wallets_start_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_vesting_wallets_start_time ON public.vesting_wallets USING btree (start_time);


--
-- Name: idx_watermarks_checkpoint; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_watermarks_checkpoint ON public.watermarks USING btree (checkpoint_sequence);


--
-- Name: idx_watermarks_timestamp; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_watermarks_timestamp ON public.watermarks USING btree (watermark_timestamp);


--
-- Name: idx_watermarks_worker_stream; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_watermarks_worker_stream ON public.watermarks USING btree (worker_id, stream_name);


--
-- Name: idx_weekly_creator_revenue_bucket; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_weekly_creator_revenue_bucket ON public.weekly_creator_revenue USING btree (bucket);


--
-- Name: idx_weekly_creator_revenue_creator; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS idx_weekly_creator_revenue_creator ON public.weekly_creator_revenue USING btree (creator);


--
-- Name: my_ip_access_logs_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_access_logs_time_idx ON public.mydata_access_logs USING btree ("time" DESC);


--
-- Name: my_ip_events_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_events_time_idx ON public.my_ip_events USING btree ("time" DESC);


--
-- Name: my_ip_grants_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_grants_time_idx ON public.my_ip_grants USING btree ("time" DESC);


--
-- Name: my_ip_purchases_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_purchases_time_idx ON public.mydata_purchases USING btree ("time" DESC);


--
-- Name: my_ip_revenue_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_revenue_time_idx ON public.mydata_revenue USING btree ("time" DESC);


--
-- Name: my_ip_revenue_time_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_revenue_time_idx1 ON public.my_ip_revenue USING btree ("time" DESC);


--
-- Name: my_ip_subscriptions_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_subscriptions_time_idx ON public.mydata_subscriptions USING btree ("time" DESC);


--
-- Name: my_ip_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS my_ip_time_idx ON public.my_ip USING btree ("time" DESC);


--
-- Name: profile_badges_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS profile_badges_time_idx ON public.profile_badges USING btree ("time" DESC);


--
-- Name: profile_offers_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS profile_offers_time_idx ON public.profile_offers USING btree ("time" DESC);


--
-- Name: profile_sale_fees_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS profile_sale_fees_time_idx ON public.profile_sale_fees USING btree ("time" DESC);


--
-- Name: profile_subscriptions_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS profile_subscriptions_time_idx ON public.profile_subscriptions USING btree ("time" DESC);


--
-- Name: social_proof_token_pools_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS social_proof_token_pools_time_idx ON public.social_proof_token_pools USING btree ("time" DESC);


--
-- Name: spt_exchange_config_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_exchange_config_time_idx ON public.spt_exchange_config USING btree ("time" DESC);


--
-- Name: spt_holdings_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_holdings_time_idx ON public.spt_holdings USING btree ("time" DESC);


--
-- Name: spt_price_history_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_price_history_time_idx ON public.spt_price_history USING btree ("time" DESC);


--
-- Name: spt_reservation_pools_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_reservation_pools_time_idx ON public.spt_reservation_pools USING btree ("time" DESC);


--
-- Name: spt_reservations_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_reservations_time_idx ON public.spt_reservations USING btree ("time" DESC);


--
-- Name: spt_revenue_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_revenue_time_idx ON public.spt_revenue USING btree ("time" DESC);


--
-- Name: spt_transactions_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS spt_transactions_time_idx ON public.spt_transactions USING btree ("time" DESC);


--
-- Name: subscription_access_logs_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS subscription_access_logs_time_idx ON public.subscription_access_logs USING btree ("time" DESC);


--
-- Name: subscription_events_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS subscription_events_time_idx ON public.subscription_events USING btree ("time" DESC);


--
-- Name: subscription_revenue_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS subscription_revenue_time_idx ON public.subscription_revenue USING btree ("time" DESC);


--
-- Name: unified_revenue_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS unified_revenue_time_idx ON public.unified_revenue USING btree ("time" DESC);


--
-- Name: vesting_events_time_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX IF NOT EXISTS vesting_events_time_idx ON public.vesting_events USING btree ("time" DESC);


--
-- Name: _hyper_1_36_chunk no_delete_social_graph_events; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER no_delete_social_graph_events BEFORE DELETE ON _timescaledb_internal._hyper_1_36_chunk FOR EACH ROW EXECUTE FUNCTION public.prevent_social_graph_events_deletion();


--
-- Name: _hyper_1_40_chunk no_delete_social_graph_events; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER no_delete_social_graph_events BEFORE DELETE ON _timescaledb_internal._hyper_1_40_chunk FOR EACH ROW EXECUTE FUNCTION public.prevent_social_graph_events_deletion();


--
-- Name: _hyper_1_36_chunk ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON _timescaledb_internal._hyper_1_36_chunk;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON _timescaledb_internal._hyper_1_36_chunk FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('1');
    END IF;
END $$;


--
-- Name: _hyper_1_40_chunk ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON _timescaledb_internal._hyper_1_40_chunk;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON _timescaledb_internal._hyper_1_40_chunk FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('1');
    END IF;
END $$;


--
-- Name: _hyper_3_35_chunk ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON _timescaledb_internal._hyper_3_35_chunk;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON _timescaledb_internal._hyper_3_35_chunk FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('3');
    END IF;
END $$;


--
-- Name: _hyper_5_38_chunk ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON _timescaledb_internal._hyper_5_38_chunk;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON _timescaledb_internal._hyper_5_38_chunk FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('5');
    END IF;
END $$;


--
-- Name: _hyper_5_39_chunk ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON _timescaledb_internal._hyper_5_39_chunk;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON _timescaledb_internal._hyper_5_39_chunk FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('5');
    END IF;
END $$;


--
-- Name: _compressed_hypertable_101 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_101;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_101 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_103 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_103;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_103 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_105 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_105;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_105 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_11 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_11;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_11 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_114 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_114;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_114 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_115 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_115;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_115 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_116 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_116;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_116 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_120 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_120;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_120 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_122 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_122;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_122 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_126 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_126;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_126 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_127 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_127;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_127 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_129 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_129;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_129 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_131 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_131;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_131 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_133 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_133;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_133 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_135 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_135;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_135 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_14 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_14;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_14 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_145 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_145;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_145 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_147 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_147;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_147 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_149 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_149;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_149 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_151 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_151;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_151 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_154 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_154;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_154 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_156 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_156;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_156 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_158 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_158;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_158 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_16 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_16;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_16 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_160 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_160;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_160 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_18 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_18;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_18 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_2 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_2;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_2 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_20 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_20;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_20 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_22 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_22;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_22 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_24 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_24;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_24 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_26 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_26;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_26 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_28 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_28;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_28 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_30 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_30;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_30 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_36 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_36;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_36 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_38 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_38;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_38 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_4 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_4;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_4 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_40 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_40;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_40 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_42 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_42;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_42 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_44 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_44;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_44 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_46 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_46;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_46 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_48 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_48;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_48 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_6 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_6;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_6 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_62 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_62;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_62 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_64 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_64;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_64 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_66 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_66;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_66 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_72 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_72;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_72 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_76 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_76;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_76 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_78 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_78;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_78 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_80 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_80;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_80 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_82 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_82;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_82 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_89 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_89;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_89 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_90 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_90;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_90 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_91 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_91;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_91 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_92 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_92;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_92 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_97 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_97;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_97 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _compressed_hypertable_99 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

DROP TRIGGER IF EXISTS ts_insert_blocker ON _timescaledb_internal._compressed_hypertable_99;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'insert_blocker') THEN
        CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._compressed_hypertable_99 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();
    END IF;
END $$;


--
-- Name: _materialized_hypertable_106 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_106 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_107 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_107 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_112 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_112 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_113 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_113 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_117 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_117 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_118 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_118 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_12 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_12 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_123 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_123 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_141 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_141 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_142 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_142 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_143 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_143 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_31 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_31 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_32 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_32 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_33 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_33 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_34 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_34 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_49 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_49 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_50 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_50 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_51 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_51 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_52 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_52 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_7 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_7 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_73 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_73 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_74 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_74 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_8 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_8 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_83 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_83 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_84 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_84 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: _materialized_hypertable_9 ts_insert_blocker; Type: TRIGGER; Schema: _timescaledb_internal; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON _timescaledb_internal._materialized_hypertable_9 FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: anonymous_votes check_anonymous_vote_proposal; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_anonymous_vote_proposal BEFORE INSERT OR UPDATE ON public.anonymous_votes FOR EACH ROW EXECUTE FUNCTION public.validate_anonymous_vote_proposal();


--
-- Name: poc_analysis_results check_poc_analysis_post_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_poc_analysis_post_reference BEFORE INSERT OR UPDATE ON public.poc_analysis_results FOR EACH ROW EXECUTE FUNCTION public.validate_poc_post_reference();


--
-- Name: poc_badges check_poc_badge_post_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_poc_badge_post_reference BEFORE INSERT OR UPDATE ON public.poc_badges FOR EACH ROW EXECUTE FUNCTION public.validate_poc_post_reference();


--
-- Name: poc_disputes check_poc_dispute_post_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_poc_dispute_post_reference BEFORE INSERT OR UPDATE ON public.poc_disputes FOR EACH ROW EXECUTE FUNCTION public.validate_poc_post_reference();


--
-- Name: poc_revenue_redirections check_poc_redirection_accused_post_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_poc_redirection_accused_post_reference BEFORE INSERT OR UPDATE ON public.poc_revenue_redirections FOR EACH ROW EXECUTE FUNCTION public.validate_poc_post_reference();


--
-- Name: poc_revenue_redirections check_poc_redirection_original_post_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_poc_redirection_original_post_reference BEFORE INSERT OR UPDATE ON public.poc_revenue_redirections FOR EACH ROW EXECUTE FUNCTION public.validate_poc_original_post_reference();


--
-- Name: poc_dispute_votes check_poc_vote_dispute_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_poc_vote_dispute_reference BEFORE INSERT OR UPDATE ON public.poc_dispute_votes FOR EACH ROW EXECUTE FUNCTION public.validate_poc_dispute_reference();


--
-- Name: comments check_post_reference; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_post_reference BEFORE INSERT OR UPDATE ON public.comments FOR EACH ROW EXECUTE FUNCTION public.validate_post_reference();


--
-- Name: community_votes check_proposal_community_vote; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_proposal_community_vote BEFORE INSERT OR UPDATE ON public.community_votes FOR EACH ROW EXECUTE FUNCTION public.validate_proposal_community_vote();


--
-- Name: delegate_votes check_proposal_delegate_vote; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_proposal_delegate_vote BEFORE INSERT OR UPDATE ON public.delegate_votes FOR EACH ROW EXECUTE FUNCTION public.validate_proposal_delegate_vote();


--
-- Name: reward_distributions check_proposal_reward; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER check_proposal_reward BEFORE INSERT OR UPDATE ON public.reward_distributions FOR EACH ROW EXECUTE FUNCTION public.validate_proposal_reward();


--
-- Name: social_graph_events no_delete_social_graph_events; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER no_delete_social_graph_events BEFORE DELETE ON public.social_graph_events FOR EACH ROW EXECUTE FUNCTION public.prevent_social_graph_events_deletion();


--
-- Name: anonymous_votes set_anonymous_vote_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_anonymous_vote_time BEFORE INSERT OR UPDATE ON public.anonymous_votes FOR EACH ROW EXECUTE FUNCTION public.update_anonymous_vote_time();


--
-- Name: comments set_comment_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_comment_time BEFORE INSERT ON public.comments FOR EACH ROW EXECUTE FUNCTION public.update_comment_time();


--
-- Name: community_votes set_community_vote_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_community_vote_time BEFORE INSERT OR UPDATE ON public.community_votes FOR EACH ROW EXECUTE FUNCTION public.update_community_vote_time();


--
-- Name: vote_decryption_failures set_decryption_failure_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_decryption_failure_time BEFORE INSERT OR UPDATE ON public.vote_decryption_failures FOR EACH ROW EXECUTE FUNCTION public.update_decryption_failure_time();


--
-- Name: delegates set_delegate_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_delegate_time BEFORE INSERT OR UPDATE ON public.delegates FOR EACH ROW EXECUTE FUNCTION public.update_delegate_time();


--
-- Name: delegate_votes set_delegate_vote_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_delegate_vote_time BEFORE INSERT OR UPDATE ON public.delegate_votes FOR EACH ROW EXECUTE FUNCTION public.update_delegate_vote_time();


--
-- Name: posts_deletion_events set_deletion_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_deletion_time BEFORE INSERT ON public.posts_deletion_events FOR EACH ROW EXECUTE FUNCTION public.update_deletion_time();


--
-- Name: reward_distributions set_distribution_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_distribution_time BEFORE INSERT OR UPDATE ON public.reward_distributions FOR EACH ROW EXECUTE FUNCTION public.update_distribution_time();


--
-- Name: posts_moderation_events set_moderation_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_moderation_time BEFORE INSERT ON public.posts_moderation_events FOR EACH ROW EXECUTE FUNCTION public.update_moderation_time();


--
-- Name: nominated_delegates set_nominee_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_nominee_time BEFORE INSERT OR UPDATE ON public.nominated_delegates FOR EACH ROW EXECUTE FUNCTION public.update_nominee_time();


--
-- Name: poc_analysis_results set_poc_analysis_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_poc_analysis_time BEFORE INSERT ON public.poc_analysis_results FOR EACH ROW EXECUTE FUNCTION public.update_poc_analysis_time();


--
-- Name: poc_badges set_poc_badge_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_poc_badge_time BEFORE INSERT ON public.poc_badges FOR EACH ROW EXECUTE FUNCTION public.update_poc_badge_time();


--
-- Name: poc_disputes set_poc_dispute_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_poc_dispute_time BEFORE INSERT ON public.poc_disputes FOR EACH ROW EXECUTE FUNCTION public.update_poc_dispute_time();


--
-- Name: poc_revenue_redirections set_poc_redirection_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_poc_redirection_time BEFORE INSERT ON public.poc_revenue_redirections FOR EACH ROW EXECUTE FUNCTION public.update_poc_redirection_time();


--
-- Name: poc_dispute_votes set_poc_vote_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_poc_vote_time BEFORE INSERT ON public.poc_dispute_votes FOR EACH ROW EXECUTE FUNCTION public.update_poc_vote_time();


--
-- Name: post_prediction_config set_post_prediction_config_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_post_prediction_config_time BEFORE INSERT ON public.post_prediction_config FOR EACH ROW EXECUTE FUNCTION public.update_post_prediction_config_time();


--
-- Name: posts set_post_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_post_time BEFORE INSERT ON public.posts FOR EACH ROW EXECUTE FUNCTION public.update_post_time();


--
-- Name: promoted_posts set_promoted_post_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_promoted_post_time BEFORE INSERT ON public.promoted_posts FOR EACH ROW EXECUTE FUNCTION public.update_promoted_post_time();


--
-- Name: promotion_budget_events set_promotion_budget_event_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_promotion_budget_event_time BEFORE INSERT ON public.promotion_budget_events FOR EACH ROW EXECUTE FUNCTION public.update_promotion_budget_event_time();


--
-- Name: promotion_status_events set_promotion_status_event_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_promotion_status_event_time BEFORE INSERT ON public.promotion_status_events FOR EACH ROW EXECUTE FUNCTION public.update_promotion_status_event_time();


--
-- Name: promotion_views set_promotion_view_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_promotion_view_time BEFORE INSERT ON public.promotion_views FOR EACH ROW EXECUTE FUNCTION public.update_promotion_view_time();


--
-- Name: proposals set_proposal_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_proposal_time BEFORE INSERT OR UPDATE ON public.proposals FOR EACH ROW EXECUTE FUNCTION public.update_proposal_time();


--
-- Name: delegate_ratings set_rating_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_rating_time BEFORE INSERT OR UPDATE ON public.delegate_ratings FOR EACH ROW EXECUTE FUNCTION public.update_rating_time();


--
-- Name: reactions set_reaction_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_reaction_time BEFORE INSERT ON public.reactions FOR EACH ROW EXECUTE FUNCTION public.update_reaction_time();


--
-- Name: governance_registries set_registry_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_registry_time BEFORE INSERT OR UPDATE ON public.governance_registries FOR EACH ROW EXECUTE FUNCTION public.update_registry_time();


--
-- Name: posts_reports set_report_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_report_time BEFORE INSERT ON public.posts_reports FOR EACH ROW EXECUTE FUNCTION public.update_report_time();


--
-- Name: reposts set_repost_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_repost_time BEFORE INSERT ON public.reposts FOR EACH ROW EXECUTE FUNCTION public.update_repost_time();


--
-- Name: spot_config set_spot_config_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_spot_config_time BEFORE INSERT ON public.spot_config FOR EACH ROW EXECUTE FUNCTION public.update_spot_config_time();


--
-- Name: tips set_tip_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_tip_time BEFORE INSERT ON public.tips FOR EACH ROW EXECUTE FUNCTION public.update_tip_time();


--
-- Name: posts_transfers set_transfer_time; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_transfer_time BEFORE INSERT ON public.posts_transfers FOR EACH ROW EXECUTE FUNCTION public.update_transfer_time();


--
-- Name: anonymous_votes ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.anonymous_votes;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.anonymous_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('119');
    END IF;
END $$;


--
-- Name: checkpoint_processing ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.checkpoint_processing;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.checkpoint_processing FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('10');
    END IF;
END $$;


--
-- Name: community_votes ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.community_votes;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.community_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('45');
    END IF;
END $$;


--
-- Name: delegate_ratings ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.delegate_ratings;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.delegate_ratings FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('41');
    END IF;
END $$;


--
-- Name: delegate_votes ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.delegate_votes;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.delegate_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('43');
    END IF;
END $$;


--
-- Name: mydata_access_logs ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.mydata_access_logs;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.mydata_access_logs FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('88');
    END IF;
END $$;


--
-- Name: mydata_purchases ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.mydata_purchases;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.mydata_purchases FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('85');
    END IF;
END $$;


--
-- Name: mydata_revenue ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.mydata_revenue;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.mydata_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('87');
    END IF;
END $$;


--
-- Name: platform_events ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.platform_events;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.platform_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('5');
    END IF;
END $$;


--
-- Name: poc_badges ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.poc_badges;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.poc_badges FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('96');
    END IF;
END $$;


--
-- Name: profile_events ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.profile_events;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.profile_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('3');
    END IF;
END $$;


--
-- Name: profile_subscriptions ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.profile_subscriptions;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.profile_subscriptions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('108');
    END IF;
END $$;


--
-- Name: promotion_views ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.promotion_views;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.promotion_views FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('77');
    END IF;
END $$;


--
-- Name: reactions ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.reactions;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.reactions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('17');
    END IF;
END $$;


--
-- Name: reposts ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.reposts;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.reposts FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('19');
    END IF;
END $$;


--
-- Name: reward_distributions ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.reward_distributions;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.reward_distributions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('47');
    END IF;
END $$;


--
-- Name: social_graph_events ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.social_graph_events;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.social_graph_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('1');
    END IF;
END $$;


--
-- Name: spt_price_history ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.spt_price_history;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.spt_price_history FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('71');
    END IF;
END $$;


--
-- Name: subscription_revenue ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.subscription_revenue;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.subscription_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('110');
    END IF;
END $$;


--
-- Name: tips ts_cagg_invalidation_trigger; Type: TRIGGER; Schema: public; Owner: -
--

DROP TRIGGER IF EXISTS ts_cagg_invalidation_trigger ON public.tips;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_proc p JOIN pg_namespace n ON p.pronamespace = n.oid WHERE n.nspname = '_timescaledb_functions' AND p.proname = 'continuous_agg_invalidation_trigger') THEN
        CREATE TRIGGER ts_cagg_invalidation_trigger AFTER INSERT OR DELETE OR UPDATE ON public.tips FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.continuous_agg_invalidation_trigger('21');
    END IF;
END $$;


--
-- Name: anonymous_votes ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.anonymous_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: checkpoint_processing ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.checkpoint_processing FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: comments ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.comments FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: community_votes ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.community_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: delegate_ratings ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.delegate_ratings FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: delegate_votes ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.delegate_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: delegates ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.delegates FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: my_ip ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.my_ip FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: my_ip_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.my_ip_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: my_ip_grants ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.my_ip_grants FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: my_ip_revenue ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.my_ip_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: mydata_access_logs ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.mydata_access_logs FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: mydata_purchases ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.mydata_purchases FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: mydata_revenue ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.mydata_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: mydata_subscriptions ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.mydata_subscriptions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: nominated_delegates ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.nominated_delegates FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: platform_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.platform_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: poc_analysis_results ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.poc_analysis_results FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: poc_badges ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.poc_badges FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: poc_dispute_votes ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.poc_dispute_votes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: poc_disputes ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.poc_disputes FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: poc_revenue_redirections ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.poc_revenue_redirections FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: post_prediction_config ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.post_prediction_config FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: posts ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.posts FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: posts_deletion_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.posts_deletion_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: posts_moderation_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.posts_moderation_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: posts_reports ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.posts_reports FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: posts_transfers ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.posts_transfers FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: profile_badges ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.profile_badges FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: profile_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.profile_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: profile_offers ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.profile_offers FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: profile_sale_fees ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.profile_sale_fees FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: profile_subscriptions ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.profile_subscriptions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: promoted_posts ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.promoted_posts FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: promotion_budget_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.promotion_budget_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: promotion_status_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.promotion_status_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: promotion_views ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.promotion_views FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: proposals ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.proposals FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: reactions ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.reactions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: reposts ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.reposts FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: reward_distributions ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.reward_distributions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: social_graph_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.social_graph_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: social_proof_of_truth ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.social_proof_of_truth FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: social_proof_token_pools ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.social_proof_token_pools FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spot_bets ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spot_bets FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spot_config ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spot_config FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spot_payouts ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spot_payouts FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spot_refunds ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spot_refunds FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spot_resolutions ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spot_resolutions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_exchange_config ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_exchange_config FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_holdings ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_holdings FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_price_history ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_price_history FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_reservation_pools ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_reservation_pools FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_reservations ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_reservations FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_revenue ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: spt_transactions ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.spt_transactions FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: subscription_access_logs ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.subscription_access_logs FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: subscription_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.subscription_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: subscription_revenue ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.subscription_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: tips ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.tips FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: unified_revenue ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.unified_revenue FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: vesting_events ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.vesting_events FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: vote_decryption_failures ts_insert_blocker; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER ts_insert_blocker BEFORE INSERT ON public.vote_decryption_failures FOR EACH ROW EXECUTE FUNCTION _timescaledb_functions.insert_blocker();


--
-- Name: social_graph_relationships update_follow_counts_delete; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_follow_counts_delete AFTER DELETE ON public.social_graph_relationships FOR EACH ROW EXECUTE FUNCTION public.verify_follow_counts();


--
-- Name: social_graph_relationships update_follow_counts_insert; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_follow_counts_insert AFTER INSERT ON public.social_graph_relationships FOR EACH ROW EXECUTE FUNCTION public.verify_follow_counts();


--
-- Name: platform_delivery_config fk_platform; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.platform_delivery_config
    ADD CONSTRAINT fk_platform FOREIGN KEY (platform_id) REFERENCES public.platforms(platform_id) ON DELETE CASCADE;


--
-- Name: profile_subscriptions fk_profile_subscriptions_service_id; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.profile_subscriptions
    ADD CONSTRAINT fk_profile_subscriptions_service_id FOREIGN KEY (service_id) REFERENCES public.profile_subscription_services(service_id);


--
-- PostgreSQL database dump complete
--
