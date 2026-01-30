// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{routing::get, Router};
use std::sync::Arc;

use crate::social::db::Database;

// Import all handlers
use crate::social::api::handlers::blocking::{
    check_platform_blocked, check_profile_blocked, get_blocked_platforms, get_blocked_profiles,
};
use crate::social::api::handlers::governance::{
    get_anonymous_voting_trends, get_delegate_by_address, get_delegate_proposals,
    get_delegate_ratings, get_proposal_anonymous_stats, get_proposal_anonymous_votes,
    get_proposal_by_id, get_proposal_community_votes, get_proposal_decryption_failures,
    get_registry_by_type, list_delegates, list_governance_events, list_nominees, list_proposals,
    list_registries,
};
use crate::social::api::handlers::health::health_check;
use crate::social::api::handlers::mydata::{
    get_creator_mydata, get_mydata_access_analytics, get_mydata_access_logs, get_mydata_by_id,
    get_mydata_configuration, get_mydata_purchases, get_mydata_revenue, get_mydata_revenue_timeline,
    get_mydata_stats, get_mydata_subscriptions, get_popular_mydata, list_mydata,
};
use crate::social::api::handlers::platforms::{
    check_platform_membership, get_approved_platforms, get_platform_approval_status,
    get_platform_blocked_profiles, get_platform_by_id, get_platform_events,
    get_platform_members, get_platform_moderators, get_platforms, get_profile_platforms,
};
use crate::social::api::handlers::posts::{
    get_post_by_id, get_post_comments, get_post_configuration, get_post_promotion, get_post_reactions, get_post_reposts,
    get_profile_posts, get_promoted_posts, get_promotion_hourly_stats,
    get_promotion_spending_trends, get_promotion_stats, get_promotion_time_analytics,
    get_promotion_views, get_top_performing_promotions, get_trending_posts, list_posts,
};
use crate::social::api::handlers::profile_events::{
    get_blocking_history, get_platform_memberships, get_profile_events,
};
use crate::social::api::handlers::profiles::{
    check_username_availability, get_badges, get_profile_badge_by_id, get_profile_badges,
    get_profile_by_address, get_profile_by_username, latest_profiles,
};
use crate::social::api::handlers::social_graph::{
    check_following, get_follow_stats, get_followers, get_following,
    get_social_graph_chart_data,
};
// Import social proof token handlers
use crate::social::api::handlers::social_proof_token::{
    get_creator_revenue_streams, get_market_sentiment, get_popular_tokens, get_spt_configuration,
    get_spt_holdings, get_spt_pool_by_associated_id, get_spt_pool_by_id, get_spt_price_history,
    get_spt_reservation_pool_by_id, get_spt_reservation_pools, get_spt_reservations_by_pool,
    get_spt_transactions, get_token_liquidity_profile, get_top_performing_tokens,
    get_user_portfolio_performance, get_user_spt_holdings, list_spt_pools,
};
// Import search handler
use crate::social::api::handlers::search::global_search;
// Import PoC handlers
use crate::social::api::handlers::poc::{
    get_dispute_votes, get_poc_analysis_results, get_poc_analytics, get_poc_badge_by_id,
    get_poc_badges, get_poc_configuration, get_poc_dispute_by_id, get_poc_disputes,
    get_post_poc_badges, get_post_revenue_redirections, get_revenue_redirections,
};
// Import subscription handlers
use crate::social::api::handlers::subscriptions::{
    check_subscription_access, get_service_performance, get_subscriber_summary,
    get_subscription_analytics, get_subscription_revenue, get_subscription_services,
    get_subscription_status, get_subscriptions,
};
// Import vesting handlers
use crate::social::api::handlers::vesting::{
    get_active_vesting_wallets, get_user_vesting_wallets, get_vesting_analytics, get_vesting_events, get_vesting_leaderboard,
    get_vesting_wallet_by_id, get_vesting_wallet_claimable, get_vesting_wallet_events,
    get_vesting_wallets,
};
// Import revenue handlers
use crate::social::api::handlers::revenue::{
    get_creator_revenue_stats, get_platform_revenue_stats, get_revenue_chart_data,
    get_revenue_dashboard, get_revenue_leaderboard, get_spt_pool_revenue, get_unified_revenue,
};
// Import stats handlers
use crate::social::api::handlers::spot::{
    get_spot_configuration, get_spot_record, list_spot_bets, list_spot_payouts, list_spot_refunds,
};
use crate::social::api::handlers::stats::get_system_stats;
// Import insurance handlers
use crate::social::api::handlers::insurance::{
    get_insurance_configuration, get_policy, get_vault, get_vault_exposures, list_market_policies,
    list_policies, list_vault_transactions, list_vaults,
};
// Import treasury handlers
use crate::social::api::handlers::treasury::{get_current_treasury, get_treasury_history_endpoint};

/// Build the application router with all API routes
pub fn build_router(db: Arc<Database>) -> Router {
    // Extract the pool from the Database wrapper
    let pool = (*db.pool).clone();

    // Create router with standard endpoints
    let main_router = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Stats endpoints
        .route("/stats/system", get(get_system_stats))
        // Profile endpoints
        .route("/profiles", get(latest_profiles))
        .route("/profiles/address/:address", get(get_profile_by_address))
        .route("/profiles/username/:username", get(get_profile_by_username))
        .route(
            "/profiles/username/:username/availability",
            get(check_username_availability),
        )
        .route("/profiles/:id/posts", get(get_profile_posts))
        .route("/profiles/:id/events", get(get_profile_events))
        .route("/profiles/:id/platforms", get(get_platform_memberships))
        .route("/profiles/:id/platform-memberships", get(get_profile_platforms))
        .route("/profiles/:id/blocking", get(get_blocking_history))
        // Profile Badge endpoints
        .route("/profiles/:id/badges", get(get_profile_badges))
        .route("/badges/:badge_id", get(get_profile_badge_by_id))
        .route("/badges", get(get_badges))
        // Social Graph endpoints
        .route("/profiles/:id/following", get(get_following))
        .route("/profiles/:id/followers", get(get_followers))
        .route("/profiles/:id/stats", get(get_follow_stats))
        .route(
            "/social-graph/check/:follower/:following",
            get(check_following),
        )
        .route("/social-graph/chart-data", get(get_social_graph_chart_data))
        // Blocking endpoints
        .route("/profiles/:id/blocked", get(get_blocked_profiles))
        .route(
            "/profiles/:id/blocked-platforms",
            get(get_blocked_platforms),
        )
        .route(
            "/blocklist/check/profile/:blocker/:blocked",
            get(check_profile_blocked),
        )
        .route(
            "/blocklist/check/platform/:profile/:platform",
            get(check_platform_blocked),
        )
        // Platform endpoints
        .route("/platforms", get(get_platforms))
        .route("/platforms/approved", get(get_approved_platforms))
        .route("/platforms/:id", get(get_platform_by_id))
        .route("/platforms/:id/moderators", get(get_platform_moderators))
        .route("/platforms/:id/approval", get(get_platform_approval_status))
        .route("/platforms/:id/blocked", get(get_platform_blocked_profiles))
        .route("/platforms/:id/members", get(get_platform_members))
        .route(
            "/platforms/:id/membership/:profile_id",
            get(check_platform_membership),
        )
        .route("/platforms/:id/events", get(get_platform_events))
        // Post endpoints (using TimescaleDB)
        .route("/posts", get(list_posts))
        .route("/posts/configuration", get(get_post_configuration))
        .route("/posts/trending", get(get_trending_posts))
        .route("/posts/:id", get(get_post_by_id))
        .route("/posts/:id/comments", get(get_post_comments))
        .route("/posts/:id/reactions", get(get_post_reactions))
        .route("/posts/:id/reposts", get(get_post_reposts))
        // Promotion endpoints
        .route("/promotions", get(get_promoted_posts))
        .route("/posts/:id/promotion", get(get_post_promotion))
        .route("/promotions/:id/views", get(get_promotion_views))
        .route("/promotions/:id/stats", get(get_promotion_stats))
        // TimescaleDB-optimized promotion analytics endpoints
        .route(
            "/promotions/:id/analytics/time-series",
            get(get_promotion_time_analytics),
        )
        .route(
            "/promotions/:id/analytics/hourly",
            get(get_promotion_hourly_stats),
        )
        .route(
            "/promotions/analytics/top-performing",
            get(get_top_performing_promotions),
        )
        .route(
            "/promotions/analytics/spending-trends",
            get(get_promotion_spending_trends),
        )
        // PoC endpoints (using TimescaleDB)
        .route("/poc/badges", get(get_poc_badges))
        .route("/poc/badges/:id", get(get_poc_badge_by_id))
        .route("/poc/revenue-redirections", get(get_revenue_redirections))
        .route("/poc/analysis-results", get(get_poc_analysis_results))
        .route("/poc/disputes", get(get_poc_disputes))
        .route("/poc/disputes/:id", get(get_poc_dispute_by_id))
        .route("/poc/disputes/:id/votes", get(get_dispute_votes))
        .route("/poc/analytics", get(get_poc_analytics))
        .route("/poc/configuration", get(get_poc_configuration))
        .route("/posts/:id/poc-badges", get(get_post_poc_badges))
        .route(
            "/posts/:id/revenue-redirections",
            get(get_post_revenue_redirections),
        )
        // Subscription endpoints (using TimescaleDB)
        .route("/subscriptions", get(get_subscriptions))
        .route("/subscription-services", get(get_subscription_services))
        .route("/subscription-revenue", get(get_subscription_revenue))
        .route("/subscriptions/:id/status", get(get_subscription_status))
        .route(
            "/subscription-access/:subscriber/:content_id",
            get(check_subscription_access),
        )
        .route("/subscription-analytics", get(get_subscription_analytics))
        .route("/service-performance", get(get_service_performance))
        .route("/subscribers/:address/summary", get(get_subscriber_summary))
        // Vesting endpoints (using TimescaleDB)
        .route("/vesting/wallets/active", get(get_active_vesting_wallets))
        .route("/vesting/wallets", get(get_vesting_wallets))
        .route("/vesting/wallets/:wallet_id", get(get_vesting_wallet_by_id))
        .route(
            "/vesting/wallets/:wallet_id/events",
            get(get_vesting_wallet_events),
        )
        .route(
            "/vesting/wallets/:wallet_id/claimable",
            get(get_vesting_wallet_claimable),
        )
        .route(
            "/vesting/users/:address/wallets",
            get(get_user_vesting_wallets),
        )
        .route("/vesting/events", get(get_vesting_events))
        .route("/vesting/analytics", get(get_vesting_analytics))
        .route("/vesting/leaderboard", get(get_vesting_leaderboard))
        // Unified Revenue Analytics endpoints (using TimescaleDB)
        .route("/revenue/dashboard", get(get_revenue_dashboard))
        .route("/revenue/leaderboard", get(get_revenue_leaderboard))
        .route("/revenue/chart-data", get(get_revenue_chart_data))
        .route("/revenue/unified", get(get_unified_revenue))
        .route(
            "/revenue/creators/:address/stats",
            get(get_creator_revenue_stats),
        )
        .route(
            "/revenue/platforms/:address/stats",
            get(get_platform_revenue_stats),
        )
        .route("/revenue/spt/pools/:pool_id", get(get_spt_pool_revenue))
        // SPoT (Social Proof of Truth) endpoints
        .route("/spot/configuration", get(get_spot_configuration))
        .route("/spot/:post_id/record", get(get_spot_record))
        .route("/spot/:post_id/bets", get(list_spot_bets))
        .route("/spot/:post_id/payouts", get(list_spot_payouts))
        .route("/spot/:post_id/refunds", get(list_spot_refunds))
        // Insurance endpoints
        .route("/insurance/config", get(get_insurance_configuration))
        .route("/insurance/vaults", get(list_vaults))
        .route("/insurance/vaults/:vault_id", get(get_vault))
        .route(
            "/insurance/vaults/:vault_id/transactions",
            get(list_vault_transactions),
        )
        .route("/insurance/vaults/:vault_id/exposures", get(get_vault_exposures))
        .route("/insurance/policies", get(list_policies))
        .route("/insurance/policies/:policy_id", get(get_policy))
        .route("/insurance/markets/:market_id/policies", get(list_market_policies))
        // MyData Marketplace endpoints (using TimescaleDB)
        .route("/mydata", get(list_mydata))
        .route("/mydata/configuration", get(get_mydata_configuration))
        .route("/mydata/popular", get(get_popular_mydata))
        .route("/mydata/:id", get(get_mydata_by_id))
        .route("/mydata/:id/purchases", get(get_mydata_purchases))
        .route("/mydata/:id/subscriptions", get(get_mydata_subscriptions))
        .route("/mydata/:id/revenue", get(get_mydata_revenue))
        .route("/mydata/:id/access-logs", get(get_mydata_access_logs))
        .route("/mydata/:id/stats", get(get_mydata_stats))
        .route(
            "/mydata/:id/revenue-timeline",
            get(get_mydata_revenue_timeline),
        )
        .route(
            "/mydata/:id/access-analytics",
            get(get_mydata_access_analytics),
        )
        .route("/creators/:id/mydata", get(get_creator_mydata))
        // Governance endpoints
        .route("/governance/proposals", get(list_proposals))
        .route("/governance/proposals/:id", get(get_proposal_by_id))
        .route(
            "/governance/proposals/:id/votes",
            get(get_proposal_community_votes),
        )
        .route("/governance/delegates", get(list_delegates))
        .route(
            "/governance/delegates/:address",
            get(get_delegate_by_address),
        )
        .route(
            "/governance/delegates/:address/proposals",
            get(get_delegate_proposals),
        )
        .route(
            "/governance/delegates/:address/ratings",
            get(get_delegate_ratings),
        )
        .route("/governance/nominees", get(list_nominees))
        .route("/governance/registries", get(list_registries))
        .route(
            "/governance/registries/:registry_type",
            get(get_registry_by_type),
        )
        .route("/governance/events", get(list_governance_events))
        // Anonymous voting endpoints
        .route(
            "/governance/proposals/:id/anonymous-stats",
            get(get_proposal_anonymous_stats),
        )
        .route(
            "/governance/proposals/:id/anonymous-votes",
            get(get_proposal_anonymous_votes),
        )
        .route(
            "/governance/proposals/:id/decryption-failures",
            get(get_proposal_decryption_failures),
        )
        .route(
            "/governance/anonymous-voting-trends",
            get(get_anonymous_voting_trends),
        )
        // Add shared state - using the pool directly for all standard endpoints
        .with_state(pool);

    // Create a separate router for social proof token endpoints with the correct state type
    let spt_router = Router::new()
        .route("/social-proof-token/pools", get(list_spt_pools))
        .route("/social-proof-token/pools/:id", get(get_spt_pool_by_id))
        .route(
            "/social-proof-token/pools/by-associated-id/:id",
            get(get_spt_pool_by_associated_id),
        )
        .route(
            "/social-proof-token/pools/:id/transactions",
            get(get_spt_transactions),
        )
        .route(
            "/social-proof-token/pools/:id/holdings",
            get(get_spt_holdings),
        )
        .route(
            "/social-proof-token/pools/:id/price-history",
            get(get_spt_price_history),
        )
        .route(
            "/social-proof-token/reservation-pools",
            get(get_spt_reservation_pools),
        )
        .route(
            "/social-proof-token/reservation-pools/:id",
            get(get_spt_reservation_pool_by_id),
        )
        .route(
            "/social-proof-token/reservation-pools/:id/reservations",
            get(get_spt_reservations_by_pool),
        )
        .route("/social-proof-token/popular", get(get_popular_tokens))
        .route(
            "/social-proof-token/users/:address/holdings",
            get(get_user_spt_holdings),
        )
        .route(
            "/social-proof-token/analytics/top-performers",
            get(get_top_performing_tokens),
        )
        .route(
            "/social-proof-token/portfolios/:address/performance",
            get(get_user_portfolio_performance),
        )
        .route(
            "/social-proof-token/creators/:address/revenue-streams",
            get(get_creator_revenue_streams),
        )
        .route(
            "/social-proof-token/market-sentiment",
            get(get_market_sentiment),
        )
        .route(
            "/social-proof-token/pools/:id/liquidity-profile",
            get(get_token_liquidity_profile),
        )
        .route(
            "/social-proof-token/configuration",
            get(get_spt_configuration),
        )
        // Treasury endpoints
        .route("/treasury/current", get(get_current_treasury))
        .route("/treasury/history", get(get_treasury_history_endpoint))
        .route("/search", get(global_search))
        .with_state(db);

    // Merge the routers
    main_router.merge(spt_router)
}
