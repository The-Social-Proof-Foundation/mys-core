// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

// Import diesel table macros
use diesel::allow_tables_to_appear_in_same_query;
use diesel::table;

// Define profile table with all fields including encrypted ones directly in the table
table! {
    profiles (id) {
        id -> Integer,
        owner_address -> Varchar,
        username -> Varchar,
        display_name -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        profile_photo -> Nullable<Varchar>,
        website -> Nullable<Text>,           // Website field from contract
        created_at -> Timestamp,
        updated_at -> Timestamp,
        cover_photo -> Nullable<Varchar>,
        profile_id -> Nullable<Varchar>,
        // Followers count - updated when follow/unfollow occurs
        followers_count -> Integer,
        // Following count - updated when follow/unfollow occurs
        following_count -> Integer,
        // Blocked count - number of users this profile has currently blocked
        blocked_count -> Integer,
        // Post count - updated when posts are created/deleted
        post_count -> Integer,
        // Minimum offer amount for profile sales (NULL = not for sale)
        min_offer_amount -> Nullable<BigInt>,
        // Sensitive fields (client-side encrypted)
        birthdate -> Nullable<Text>,
        current_location -> Nullable<Text>,
        raised_location -> Nullable<Text>,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        gender -> Nullable<Text>,
        political_view -> Nullable<Text>,
        religion -> Nullable<Text>,
        education -> Nullable<Text>,
        primary_language -> Nullable<Text>,
        relationship_status -> Nullable<Text>,
        x_username -> Nullable<Text>,
        mastodon_username -> Nullable<Text>,
        facebook_username -> Nullable<Text>,
        reddit_username -> Nullable<Text>,
        github_username -> Nullable<Text>,
        // Block list address
        block_list_address -> Nullable<Varchar>,
    }
}

// Define social graph relationships table
// This is a highly optimized junction table for handling follows/followers
// Now uses blockchain addresses directly to avoid database ID references
table! {
    social_graph_relationships (id) {
        id -> Integer,
        // Blockchain address for the follower
        follower_address -> Varchar,
        // Blockchain address for the followed user
        following_address -> Varchar,
        // When the relationship was created
        created_at -> Timestamp,
    }
}

// Define social graph events table for tracking all follow/unfollow actions
table! {
    social_graph_events (id) {
        id -> Integer,
        event_type -> Varchar,
        follower_address -> Varchar,
        following_address -> Varchar,
        created_at -> Timestamp,
        event_id -> Nullable<Varchar>,  // Changed from blockchain_tx_hash to event_id
        raw_event_data -> Nullable<Jsonb>,
    }
}

// Define indexer progress table
table! {
    indexer_progress (id) {
        id -> Varchar,
        last_checkpoint_processed -> Bigint,
        last_processed_at -> Timestamp,
    }
}

// Define platforms table
table! {
    platforms (id) {
        id -> Integer,
        platform_id -> Varchar,
        name -> Varchar,
        tagline -> Varchar,
        description -> Nullable<Text>,
        logo -> Nullable<Varchar>,
        developer_address -> Varchar,
        terms_of_service -> Nullable<Text>,
        privacy_policy -> Nullable<Text>,
        #[sql_name = "platforms"]
        platform_names -> Nullable<Jsonb>,
        links -> Nullable<Jsonb>,
        status -> SmallInt,
        release_date -> Nullable<Varchar>,
        shutdown_date -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_approved -> Bool,
        approval_changed_at -> Nullable<Timestamp>,
        approved_by -> Nullable<Varchar>,
    }
}

// Define platform_moderators table
table! {
    platform_moderators (id) {
        id -> Integer,
        platform_id -> Varchar,
        moderator_address -> Varchar,
        added_by -> Varchar,
        created_at -> Timestamp,
    }
}

// Define platform_blocked_profiles table
table! {
    platform_blocked_profiles (id) {
        id -> Integer,
        platform_id -> Varchar,
        profile_id -> Varchar,
        blocked_by -> Varchar,
        created_at -> Timestamp,
    }
}

// Define platform_events table
table! {
    platform_events (id) {
        id -> Integer,
        event_type -> Varchar,
        platform_id -> Varchar,
        event_data -> Jsonb,
        event_id -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

// Define platform_memberships table
table! {
    platform_memberships (id) {
        id -> Integer,
        platform_id -> Varchar,
        profile_id -> Varchar,
        joined_at -> Timestamp,
    }
}

// Note: platform_relationships table has been removed in favor of platform_memberships

// Production blocking system tables
// Blocked events table for complete audit trail
table! {
    blocked_events (id) {
        id -> Integer,
        event_id -> Nullable<Varchar>,
        event_type -> Varchar,
        blocker_address -> Varchar,
        blocked_address -> Nullable<Varchar>,
        block_list_address -> Nullable<Varchar>,
        raw_event_data -> Nullable<Jsonb>,
        processed_at -> Timestamp,
        created_at -> Timestamp,
    }
}

// Blocked profiles table for current blocking state with rich profile data
table! {
    blocked_profiles (id) {
        id -> Integer,
        blocker_address -> Varchar,
        blocked_address -> Varchar,
        block_list_address -> Nullable<Varchar>,
        // Rich profile data for performance (denormalized from profiles table)
        blocked_profile_id -> Nullable<Varchar>,
        blocked_username -> Varchar,
        blocked_display_name -> Nullable<Varchar>,
        blocked_profile_photo -> Nullable<Varchar>,
        // Blocking metadata
        first_blocked_at -> Timestamp,
        last_blocked_at -> Timestamp,
        total_block_count -> Integer,
    }
}

// Profile events table
table! {
    profile_events (id) {
        id -> Integer,
        event_type -> Varchar,
        profile_id -> Varchar,
        event_data -> Jsonb,
        event_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// ===========================================================================
// VESTING TABLES
// ===========================================================================

// Define vesting_wallets table (Regular table - reference data)
table! {
    vesting_wallets (wallet_id) {
        wallet_id -> Varchar,
        owner_address -> Varchar,
        total_amount -> BigInt,
        start_time -> BigInt,
        duration -> BigInt,
        curve_factor -> BigInt,
        claimed_amount -> BigInt,
        remaining_balance -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Varchar,
    }
}

// Define vesting_events table (TimescaleDB hypertable)
table! {
    vesting_events (id, time) {
        id -> Int4,
        wallet_id -> Varchar,
        event_type -> Varchar,
        owner_address -> Varchar,
        amount -> BigInt,
        remaining_balance -> Nullable<BigInt>,
        start_time -> Nullable<BigInt>,
        duration -> Nullable<BigInt>,
        curve_factor -> Nullable<BigInt>,
        event_time -> BigInt,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define posts table
table! {
    posts (id, time) {
        id -> Varchar,
        post_id -> Varchar,
        owner -> Varchar,
        profile_id -> Varchar,
        content -> Text,
        media_urls -> Nullable<Jsonb>,
        mentions -> Nullable<Jsonb>,
        metadata_json -> Nullable<Jsonb>,
        post_type -> Varchar,
        parent_post_id -> Nullable<Varchar>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        deleted_at -> Nullable<Int8>,
        reaction_count -> Int8,
        comment_count -> Int8,
        repost_count -> Int8,
        tips_received -> Int8,
        removed_from_platform -> Bool,
        removed_by -> Nullable<Varchar>,
        transaction_id -> Varchar,
        time -> Timestamptz,
        my_ip_id -> Nullable<Varchar>,
        revenue_recipient -> Nullable<Varchar>,
        // PoC fields
        poc_badge_id -> Nullable<Varchar>,
        revenue_redirect_to -> Nullable<Varchar>,
        revenue_redirect_percentage -> Nullable<Int8>,
        // Subscription fields
        requires_subscription -> Nullable<Bool>,
        subscription_service_id -> Nullable<Varchar>,
        subscription_price -> Nullable<Int8>,
        encrypted_content_hash -> Nullable<Varchar>,
        // Promotion fields
        promotion_id -> Nullable<Varchar>,
    }
}

// Define comments table
table! {
    comments (id, time) {
        id -> Varchar,
        comment_id -> Varchar,
        post_id -> Varchar,
        parent_comment_id -> Nullable<Varchar>,
        owner -> Varchar,
        profile_id -> Varchar,
        content -> Text,
        media_urls -> Nullable<Jsonb>,
        mentions -> Nullable<Jsonb>,
        metadata_json -> Nullable<Jsonb>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        deleted_at -> Nullable<Int8>,
        reaction_count -> Int8,
        comment_count -> Int8,
        repost_count -> Int8,
        tips_received -> Int8,
        removed_from_platform -> Bool,
        removed_by -> Nullable<Varchar>,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// Define reactions table
table! {
    reactions (id, time) {
        id -> Int4,
        object_id -> Varchar,
        user_address -> Varchar,
        reaction_text -> Varchar,
        is_post -> Bool,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define reaction_counts table
table! {
    reaction_counts (id) {
        id -> Int4,
        object_id -> Varchar,
        reaction_text -> Varchar,
        count -> Int8,
    }
}

// Define reposts table
table! {
    reposts (id, time) {
        id -> Varchar,
        repost_id -> Varchar,
        original_id -> Varchar,
        original_post_id -> Varchar,
        is_original_post -> Bool,
        owner -> Varchar,
        profile_id -> Varchar,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define tips table
table! {
    tips (id, time) {
        id -> Int4,
        tipper -> Varchar,
        recipient -> Varchar,
        object_id -> Varchar,
        amount -> Int8,
        is_post -> Bool,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define posts_reports table
table! {
    posts_reports (id, time) {
        id -> Int4,
        object_id -> Varchar,
        is_comment -> Bool,
        reporter -> Varchar,
        reason_code -> Int2,
        description -> Text,
        reported_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define posts_transfers table
table! {
    posts_transfers (id, time) {
        id -> Int4,
        object_id -> Varchar,
        previous_owner -> Varchar,
        new_owner -> Varchar,
        is_post -> Bool,
        transferred_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define posts_moderation_events table
table! {
    posts_moderation_events (id, time) {
        id -> Int4,
        object_id -> Varchar,
        platform_id -> Varchar,
        removed -> Bool,
        moderated_by -> Varchar,
        moderated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define posts_deletion_events table
table! {
    posts_deletion_events (id, time) {
        id -> Int4,
        object_id -> Varchar,
        owner -> Varchar,
        profile_id -> Varchar,
        is_post -> Bool,
        post_type -> Nullable<Varchar>,
        post_id -> Nullable<Varchar>,
        deleted_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// ===========================================================================
// MY IP DATA MARKETPLACE TABLES
// ===========================================================================

// Main data marketplace entries (Regular table - reference data)
table! {
    my_ip_data (ip_id) {
        ip_id -> Varchar,
        owner -> Varchar,
        media_type -> Varchar,
        tags -> Jsonb,
        platform_id -> Nullable<Varchar>,
        timestamp_start -> Int8,
        timestamp_end -> Nullable<Int8>,
        created_at -> Int8,
        last_updated -> Int8,
        one_time_price -> Nullable<Int8>,
        subscription_price -> Nullable<Int8>,
        subscription_duration_days -> Int8,
        geographic_region -> Nullable<Varchar>,
        data_quality -> Nullable<Varchar>,
        sample_size -> Nullable<Int8>,
        collection_method -> Nullable<Varchar>,
        is_updating -> Bool,
        update_frequency -> Nullable<Varchar>,
        version -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Purchase records (TimescaleDB hypertable)
table! {
    my_ip_purchases (id, time) {
        id -> Int4,
        ip_id -> Varchar,
        buyer -> Varchar,
        price -> Int8,
        purchase_type -> Varchar,
        purchase_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Subscription records (TimescaleDB hypertable)
table! {
    my_ip_subscriptions (id, time) {
        id -> Int4,
        ip_id -> Varchar,
        subscriber -> Varchar,
        subscription_start -> Int8,
        subscription_end -> Int8,
        price -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Revenue tracking (TimescaleDB hypertable - updated structure)
table! {
    my_ip_revenue (id, time) {
        id -> Int4,
        ip_id -> Varchar,
        from_address -> Varchar,
        to_address -> Varchar,
        amount -> Int8,
        revenue_type -> Varchar,
        revenue_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Access logs for analytics (TimescaleDB hypertable)
table! {
    my_ip_access_logs (id, time) {
        id -> Int4,
        ip_id -> Varchar,
        user_address -> Varchar,
        access_type -> Varchar,
        access_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// ===========================================================================
// SOCIAL PROOF TOKEN TABLES
// ===========================================================================

// Define social_proof_token_pools table
table! {
    social_proof_token_pools (id, time) {
        id -> Int4,
        pool_id -> Varchar,
        token_type -> Int2,
        owner -> Varchar,
        associated_id -> Varchar,
        symbol -> Varchar,
        name -> Varchar,
        circulating_supply -> Int8,
        base_price -> Int8,
        quadratic_coefficient -> Int8,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define spt_holdings table
table! {
    spt_holdings (id, time) {
        id -> Int4,
        pool_id -> Varchar,
        holder_address -> Varchar,
        amount -> Int8,
        acquired_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define spt_transactions table
table! {
    spt_transactions (id, time) {
        id -> Int4,
        pool_id -> Varchar,
        transaction_type -> Varchar,
        sender -> Varchar,
        amount -> Int8,
        mys_amount -> Int8,
        fee_amount -> Int8,
        creator_fee -> Int8,
        platform_fee -> Int8,
        treasury_fee -> Int8,
        price -> Int8,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define spt_reservation_pools table
table! {
    spt_reservation_pools (id, time) {
        id -> Int4,
        pool_id -> Varchar,
        associated_id -> Varchar,
        token_type -> Int2,
        owner -> Varchar,
        total_reserved -> Int8,
        required_threshold -> Int8,
        status -> Varchar,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define spt_reservations table
table! {
    spt_reservations (id, time) {
        id -> Int4,
        pool_id -> Varchar,
        reserver_address -> Varchar,
        amount -> Int8,
        reserved_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define spt_exchange_config table
table! {
    spt_exchange_config (id, time) {
        id -> Int4,
        updated_by -> Varchar,
        post_threshold -> Int8,
        profile_threshold -> Int8,
        max_individual_reservation_bps -> Int8,
        total_fee_bps -> Int8,
        creator_fee_bps -> Int8,
        platform_fee_bps -> Int8,
        treasury_fee_bps -> Int8,
        base_price -> Int8,
        quadratic_coefficient -> Int8,
        ecosystem_treasury -> Varchar,
        max_hold_percent_bps -> Int8,
        trading_halted -> Bool,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define spt_price_history table
table! {
    spt_price_history (id, time) {
        id -> Int4,
        pool_id -> Varchar,
        price -> Int8,
        circulating_supply -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// ===========================================================================
// GOVERNANCE TABLES
// ===========================================================================

// Define governance_registries table
table! {
    governance_registries (id) {
        id -> Int4,
        registry_type -> Int2,
        delegate_count -> Int8,
        delegate_term_epochs -> Int8,
        proposal_submission_cost -> Int8,
        min_on_chain_age_days -> Int8,
        max_votes_per_user -> Int8,
        quadratic_base_cost -> Int8,
        voting_period_epochs -> Int8,
        quorum_votes -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define delegates table
table! {
    delegates (id, time) {
        id -> Int4,
        address -> Varchar,
        profile_id -> Varchar,
        registry_type -> Int2,
        upvotes -> Int8,
        downvotes -> Int8,
        proposals_reviewed -> Int8,
        proposals_submitted -> Int8,
        sided_winning_proposals -> Int8,
        sided_losing_proposals -> Int8,
        term_start -> Int8,
        term_end -> Int8,
        is_active -> Bool,
        created_at -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define nominated_delegates table
table! {
    nominated_delegates (id, time) {
        id -> Int4,
        address -> Varchar,
        profile_id -> Varchar,
        registry_type -> Int2,
        upvotes -> Int8,
        downvotes -> Int8,
        scheduled_term_start_epoch -> Int8,
        nomination_time -> Int8,
        status -> Int2,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define proposals table
table! {
    proposals (id, time) {
        id -> Varchar,
        title -> Varchar,
        description -> Text,
        proposal_type -> Int2,
        reference_id -> Nullable<Varchar>,
        metadata_json -> Nullable<Jsonb>,
        submitter -> Varchar,
        submission_time -> Int8,
        delegate_approval_count -> Int8,
        delegate_rejection_count -> Int8,
        community_votes_for -> Int8,
        community_votes_against -> Int8,
        status -> Int2,
        voting_start_time -> Nullable<Int8>,
        voting_end_time -> Nullable<Int8>,
        reward_pool -> Int8,
        implemented_description -> Nullable<Text>,
        implementation_time -> Nullable<Int8>,
        rescind_time -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Varchar,
        // Anonymous voting fields
        anonymous_votes_for -> Nullable<Int8>,
        anonymous_votes_against -> Nullable<Int8>,
        anonymous_voters_count -> Nullable<Int8>,
        pending_anonymous_decryption -> Nullable<Bool>,
        anonymous_decryption_completed_at -> Nullable<Int8>,
    }
}

// Define delegate_ratings table
table! {
    delegate_ratings (id, time) {
        id -> Int4,
        target_address -> Varchar,
        voter_address -> Varchar,
        registry_type -> Int2,
        is_active_delegate -> Bool,
        upvote -> Bool,
        rated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define delegate_votes table
table! {
    delegate_votes (id, time) {
        id -> Int4,
        proposal_id -> Varchar,
        delegate_address -> Varchar,
        approve -> Bool,
        vote_time -> Int8,
        reason -> Nullable<Text>,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define community_votes table
table! {
    community_votes (id, time) {
        id -> Int4,
        proposal_id -> Varchar,
        voter_address -> Varchar,
        vote_weight -> Int8,
        approve -> Bool,
        vote_time -> Int8,
        vote_cost -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define reward_distributions table
table! {
    reward_distributions (id, time) {
        id -> Int4,
        proposal_id -> Varchar,
        recipient_address -> Varchar,
        amount -> Int8,
        distribution_time -> Int8,
        distribution_type -> Nullable<Varchar>,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define governance_events table for tracking governance events
table! {
    governance_events (id) {
        id -> Int4,
        event_type -> Varchar,
        registry_type -> Int2,
        event_data -> Jsonb,
        event_id -> Varchar,
        created_at -> Timestamptz,
        anonymous_voting_related -> Nullable<Bool>,
    }
}

// Define anonymous_votes table
table! {
    anonymous_votes (id, time) {
        id -> Int4,
        proposal_id -> Varchar,
        voter_address -> Varchar,
        encrypted_vote_data -> Nullable<Bytea>,
        submitted_at -> Int8,
        decrypted -> Bool,
        decrypted_at -> Nullable<Int8>,
        decrypted_vote -> Nullable<Int2>,
        decryption_status -> Int2,
        decryption_error -> Nullable<Text>,
        time -> Timestamptz,
        transaction_id -> Varchar,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

// Define vote_decryption_failures table
table! {
    vote_decryption_failures (id, time) {
        id -> Int4,
        proposal_id -> Varchar,
        voter_address -> Varchar,
        failure_reason -> Text,
        attempted_at -> Int8,
        encrypted_vote_length -> Nullable<Int4>,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// ===========================================================================
// PROOF OF CREATIVITY (POC) TABLES
// ===========================================================================

// Define poc_badges table
table! {
    poc_badges (badge_id, time) {
        badge_id -> Varchar,
        post_id -> Varchar,
        media_type -> Int2,
        issued_by -> Varchar,
        issued_at -> Int8,
        revoked -> Bool,
        revoked_at -> Nullable<Int8>,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// Define poc_revenue_redirections table
table! {
    poc_revenue_redirections (redirection_id, time) {
        redirection_id -> Varchar,
        accused_post_id -> Varchar,
        original_post_id -> Varchar,
        redirect_percentage -> Int8,
        similarity_score -> Int8,
        created_at -> Int8,
        removed -> Bool,
        removed_at -> Nullable<Int8>,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// Define poc_analysis_results table
table! {
    poc_analysis_results (post_id, time) {
        post_id -> Varchar,
        media_type -> Int2,
        similarity_detected -> Bool,
        highest_similarity_score -> Int8,
        oracle_address -> Varchar,
        original_creator -> Nullable<Varchar>,
        analysis_timestamp -> Int8,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// Define poc_disputes table
table! {
    poc_disputes (dispute_id, time) {
        dispute_id -> Varchar,
        post_id -> Varchar,
        disputer -> Varchar,
        dispute_type -> Int2,
        evidence -> Text,
        status -> Int2,
        stake_amount -> Int8,
        voting_start_epoch -> Int8,
        voting_end_epoch -> Int8,
        resolution -> Nullable<Int2>,
        winning_side -> Nullable<Int2>,
        total_winning_stake -> Nullable<Int8>,
        total_losing_stake -> Nullable<Int8>,
        submitted_at -> Int8,
        resolved_at -> Nullable<Int8>,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// Define poc_dispute_votes table
table! {
    poc_dispute_votes (dispute_id, voter, time) {
        dispute_id -> Varchar,
        voter -> Varchar,
        vote_choice -> Int2,
        stake_amount -> Int8,
        voted_at -> Int8,
        reward_claimed -> Bool,
        reward_amount -> Nullable<Int8>,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// Define poc_configuration table
table! {
    poc_configuration (id) {
        id -> Int4,
        image_threshold -> Int8,
        video_threshold -> Int8,
        audio_threshold -> Int8,
        revenue_redirect_percentage -> Int8,
        dispute_cost -> Int8,
        dispute_protocol_fee -> Int8,
        min_vote_stake -> Int8,
        max_vote_stake -> Int8,
        voting_duration_epochs -> Int8,
        updated_by -> Varchar,
        updated_at -> Int8,
        transaction_id -> Varchar,
        time -> Timestamptz,
    }
}

// ===========================================================================
// SUBSCRIPTION TABLES
// ===========================================================================

// Define profile_subscription_services table
table! {
    profile_subscription_services (service_id) {
        service_id -> Varchar,
        profile_owner -> Varchar,
        profile_id -> Varchar,
        monthly_fee -> Int8,
        active -> Bool,
        subscriber_count -> Int8,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define profile_subscriptions table
table! {
    profile_subscriptions (subscription_id, time) {
        subscription_id -> Varchar,
        service_id -> Varchar,
        subscriber -> Varchar,
        created_at -> Int8,
        expires_at -> Int8,
        auto_renew -> Bool,
        renewal_balance -> Int8,
        renewal_count -> Int8,
        cancelled_at -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Varchar,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

// Define subscription_events table
table! {
    subscription_events (event_type, time) {
        event_type -> Varchar,
        subscription_id -> Nullable<Varchar>,
        service_id -> Nullable<Varchar>,
        subscriber -> Nullable<Varchar>,
        event_data -> Jsonb,
        event_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

// Define subscription_revenue table
table! {
    subscription_revenue (service_id, time) {
        service_id -> Varchar,
        subscription_id -> Nullable<Varchar>,
        from_address -> Varchar,
        to_address -> Varchar,
        amount -> Int8,
        revenue_type -> Varchar,
        payment_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

// Define subscription_access_logs table
table! {
    subscription_access_logs (subscription_id, time) {
        subscription_id -> Varchar,
        subscriber -> Varchar,
        content_type -> Varchar,
        content_id -> Varchar,
        access_time -> Int8,
        seal_id -> Nullable<Varchar>,
        time -> Timestamptz,
        transaction_id -> Varchar,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

// ===========================================================================
// REVENUE AGGREGATION TABLES
// ===========================================================================

// SPT Revenue table
table! {
    spt_revenue (pool_id, time) {
        pool_id -> Varchar,
        transaction_type -> Varchar,
        trader -> Varchar,
        creator_address -> Varchar,
        platform_address -> Varchar,
        treasury_address -> Varchar,
        creator_fee -> Int8,
        platform_fee -> Int8,
        treasury_fee -> Int8,
        total_fee -> Int8,
        token_amount -> Int8,
        mys_amount -> Int8,
        token_price -> Int8,
        revenue_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Unified Revenue table
table! {
    unified_revenue (revenue_source, time) {
        revenue_source -> Varchar,
        revenue_type -> Varchar,
        creator_address -> Varchar,
        platform_address -> Nullable<Varchar>,
        amount -> Int8,
        currency -> Varchar,
        content_id -> Nullable<Varchar>,
        content_type -> Nullable<Varchar>,
        payer_address -> Varchar,
        recipient_address -> Varchar,
        revenue_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// ===========================================================================
// SOCIAL PROOF TOKENS KILL SWITCH TABLES
// ===========================================================================

// Define social proof tokens config table (for kill switch)
table! {
    social_proof_tokens_config (id) {
        id -> Int4,
        trading_halted -> Bool,
        admin_address -> Varchar,
        reason -> Varchar,
        timestamp_ms -> Int8,
        updated_at -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define social proof tokens events table (for kill switch event history)
table! {
    social_proof_tokens_events (id) {
        id -> Int4,
        event_type -> Varchar,
        event_data -> Jsonb,
        event_id -> Varchar,
        created_at -> Timestamptz,
    }
}

// ===========================================================================
// PROMOTION TABLES
// ===========================================================================

// Define promoted_posts table
table! {
    promoted_posts (id, time) {
        id -> Int4,
        promotion_id -> Varchar,
        post_id -> Varchar,
        owner -> Varchar,
        profile_id -> Varchar,
        payment_per_view -> Int8,
        total_budget -> Int8,
        remaining_budget -> Int8,
        active -> Bool,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define promotion_views table
table! {
    promotion_views (id, time) {
        id -> Int4,
        post_id -> Varchar,
        promotion_id -> Varchar,
        viewer -> Varchar,
        payment_amount -> Int8,
        view_duration -> Int8,
        platform_id -> Varchar,
        timestamp -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define promotion_status_events table
table! {
    promotion_status_events (id, time) {
        id -> Int4,
        post_id -> Varchar,
        promotion_id -> Varchar,
        event_type -> Varchar,
        triggered_by -> Varchar,
        new_status -> Nullable<Bool>,
        amount -> Nullable<Int8>,
        timestamp -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// Define promotion_budget_events table
table! {
    promotion_budget_events (id, time) {
        id -> Int4,
        promotion_id -> Varchar,
        post_id -> Varchar,
        event_type -> Varchar,
        amount -> Int8,
        remaining_budget -> Int8,
        timestamp -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// ===========================================================================
// SOCIAL PROOF OF TRUTH (SPoT) TABLES
// ===========================================================================

// spot_records: current state per post
table! {
    spot_records (id) {
        id -> Int4,
        post_id -> Varchar,
        status -> Int2,
        outcome -> Nullable<Int2>,
        amm_split_bps_used -> Int4,
        total_yes_escrow -> BigInt,
        total_no_escrow -> BigInt,
        created_epoch -> BigInt,
        last_resolution_epoch -> Nullable<BigInt>,
        version -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Varchar,
    }
}

// spot_bets: hypertable with time dimension
table! {
    spot_bets (id, time) {
        id -> Int4,
        post_id -> Varchar,
        user_address -> Varchar,
        is_yes -> Bool,
        escrow_amount -> BigInt,
        amm_amount -> BigInt,
        timestamp_epoch -> BigInt,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// spot_payouts: hypertable with time dimension
table! {
    spot_payouts (id, time) {
        id -> Int4,
        post_id -> Varchar,
        user_address -> Varchar,
        amount -> BigInt,
        timestamp_epoch -> BigInt,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// spot_refunds: hypertable with time dimension
table! {
    spot_refunds (id, time) {
        id -> Int4,
        post_id -> Varchar,
        user_address -> Varchar,
        amount -> BigInt,
        timestamp_epoch -> BigInt,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// spot_resolutions: resolution summaries
table! {
    spot_resolutions (id, time) {
        id -> Int4,
        post_id -> Varchar,
        outcome -> Int2,
        total_escrow -> BigInt,
        fee_taken -> BigInt,
        resolved_epoch -> BigInt,
        time -> Timestamptz,
        transaction_id -> Varchar,
    }
}

// spot_events: audit log of raw SPoT events
table! {
    spot_events (id) {
        id -> Int4,
        event_type -> Varchar,
        post_id -> Varchar,
        event_data -> Jsonb,
        event_id -> Varchar,
        created_at -> Timestamptz,
    }
}

// Unified SPoT events table (hypertable)
table! {
    social_proof_of_truth (id, time) {
        id -> Int4,
        event_type -> Varchar,
        post_id -> Varchar,
        user_address -> Nullable<Varchar>,
        is_yes -> Nullable<Bool>,
        escrow_amount -> Nullable<BigInt>,
        amm_amount -> Nullable<BigInt>,
        amount -> Nullable<BigInt>,
        outcome -> Nullable<Int2>,
        total_escrow -> Nullable<BigInt>,
        fee_taken -> Nullable<BigInt>,
        confidence_bps -> Nullable<BigInt>,
        timestamp_epoch -> BigInt,
        time -> Timestamptz,
        event_id -> Nullable<Varchar>,
        transaction_id -> Nullable<Varchar>,
        raw_event -> Nullable<Jsonb>,
    }
}

// Allow joining the tables if needed
allow_tables_to_appear_in_same_query!(
    profiles,
    social_graph_relationships,
    social_graph_events,
    indexer_progress,
    platforms,
    platform_moderators,
    platform_blocked_profiles,
    platform_events,
    platform_memberships,
    blocked_events,
    blocked_profiles,
    profile_events,
    // Vesting tables
    vesting_wallets,
    vesting_events,
    posts,
    comments,
    reactions,
    reaction_counts,
    reposts,
    tips,
    posts_reports,
    posts_transfers,
    posts_moderation_events,
    posts_deletion_events,
    // MyIP Data Marketplace tables
    my_ip_data,
    my_ip_purchases,
    my_ip_subscriptions,
    my_ip_revenue,
    my_ip_access_logs,
    // Social Proof Token tables
    social_proof_token_pools,
    spt_holdings,
    spt_transactions,
    spt_reservation_pools,
    spt_reservations,
    spt_exchange_config,
    spt_price_history,
    // Governance tables
    governance_registries,
    delegates,
    nominated_delegates,
    proposals,
    delegate_ratings,
    delegate_votes,
    community_votes,
    reward_distributions,
    governance_events,
    // PoC tables
    poc_badges,
    poc_revenue_redirections,
    poc_analysis_results,
    poc_disputes,
    poc_dispute_votes,
    poc_configuration,
    // Subscription tables
    profile_subscription_services,
    profile_subscriptions,
    subscription_events,
    subscription_revenue,
    subscription_access_logs,
    // Anonymous voting tables
    anonymous_votes,
    vote_decryption_failures,
    // Revenue aggregation tables
    spt_revenue,
    unified_revenue,
    // Social proof tokens config tables
    social_proof_tokens_config,
    social_proof_tokens_events,
    // Promotion tables
    promoted_posts,
    promotion_views,
    promotion_status_events,
    promotion_budget_events,
    // SPoT tables
    spot_records,
    spot_bets,
    spot_payouts,
    spot_refunds,
    spot_resolutions,
    spot_events,
    social_proof_of_truth,
);
