// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::db::Database;

// Import all handlers
use crate::api::handlers::health::health_check;
use crate::api::handlers::posts::{
    get_post_by_id,
    list_posts,
    get_post_comments,
    get_trending_posts,
    get_profile_posts,
    get_post_reactions,
    get_post_reposts,
    get_promoted_posts,
    get_post_promotion,
    get_promotion_views,
    get_promotion_stats,
    get_promotion_time_analytics,
    get_top_performing_promotions,
    get_promotion_hourly_stats,
    get_promotion_spending_trends,
};
use crate::api::handlers::profiles::{
    latest_profiles,
    get_profile_by_address,
    get_profile_by_username,
    check_username_availability,
};
use crate::api::handlers::profile_events::{
    get_profile_events,
    get_platform_memberships,
    get_blocking_history,
};
use crate::api::handlers::social_graph::{
    get_following,
    get_followers,
    check_following,
    get_follow_stats,
};
use crate::api::handlers::blocking::{
    check_profile_blocked,
    get_blocked_profiles,
    get_blocked_platforms,
    check_platform_blocked,
};
use crate::api::handlers::platforms::{
    get_platforms,
    get_platform_by_id,
    get_platform_moderators,
    get_approved_platforms,
    get_platform_approval_status,
    get_platform_blocked_profiles,
};
use crate::api::handlers::my_ip::{
    get_marketplace_data_by_id,
    list_marketplace_data,
    get_ip_purchases,
    get_ip_subscriptions,
    get_ip_revenue,
    get_ip_access_logs,
    get_creator_data,
    get_marketplace_stats,
    get_revenue_timeline,
    get_access_analytics,
    get_popular_marketplace_data,
};
use crate::api::handlers::governance::{
    list_proposals,
    get_proposal_by_id,
    get_proposal_community_votes,
    list_delegates,
    get_delegate_by_address,
    get_delegate_proposals,
    get_delegate_ratings,
    list_nominees,
    list_registries,
    get_registry_by_type,
    list_governance_events,
    get_proposal_anonymous_stats,
    get_proposal_anonymous_votes,
    get_proposal_decryption_failures,
    get_anonymous_voting_trends,
};
// Import social proof token handlers
use crate::api::handlers::social_proof_token::{
    get_spt_pool_by_id,
    list_spt_pools,
    get_spt_pool_by_associated_id,
    get_spt_transactions,
    get_spt_holdings,
    get_spt_price_history,
    get_spt_reservation_pools,
    get_spt_reservation_pool_by_id,
    get_spt_reservations_by_pool,
    get_user_spt_holdings,
    get_popular_tokens,
    get_top_performing_tokens,
    get_user_portfolio_performance,
    get_creator_revenue_streams,
    get_market_sentiment,
    get_token_liquidity_profile,
};
// Import search handler
use crate::api::handlers::search::global_search;
// Import PoC handlers
use crate::api::handlers::poc::{
    get_poc_badges,
    get_poc_badge_by_id,
    get_revenue_redirections,
    get_poc_analysis_results,
    get_poc_disputes,
    get_poc_dispute_by_id,
    get_dispute_votes,
    get_poc_analytics,
    get_poc_configuration,
    get_post_poc_badges,
    get_post_revenue_redirections,
};
// Import subscription handlers
use crate::api::handlers::subscriptions::{
    get_subscriptions,
    get_subscription_services,
    get_subscription_revenue,
    check_subscription_access,
    get_subscription_status,
    get_subscription_analytics,
    get_service_performance,
    get_subscriber_summary,
};
// Import vesting handlers  
use crate::api::handlers::vesting::{
    get_vesting_wallets,
    get_vesting_wallet_by_id,
    get_vesting_wallet_events,
    get_vesting_wallet_claimable,
    get_user_vesting_wallets,
    get_vesting_events,
    get_vesting_analytics,
    get_vesting_leaderboard,
};
// Import revenue handlers
use crate::api::handlers::revenue::{
    get_revenue_dashboard,
    get_revenue_leaderboard,
    get_revenue_chart_data,
    get_creator_revenue_stats,
    get_platform_revenue_stats,
    get_unified_revenue,
    get_spt_pool_revenue,
};
// Import stats handlers
use crate::api::handlers::stats::get_system_stats;
use crate::api::handlers::spot::{
    get_spot_record,
    list_spot_bets,
    list_spot_payouts,
    list_spot_refunds,
};

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
        .route("/profiles/username/:username/availability", get(check_username_availability))
        .route("/profiles/:id/posts", get(get_profile_posts))
        .route("/profiles/:id/events", get(get_profile_events))
        .route("/profiles/:id/platforms", get(get_platform_memberships))
        .route("/profiles/:id/blocking", get(get_blocking_history))
        
        // Social Graph endpoints
        .route("/profiles/:id/following", get(get_following))
        .route("/profiles/:id/followers", get(get_followers))
        .route("/profiles/:id/stats", get(get_follow_stats))
        .route("/social-graph/check/:follower/:following", get(check_following))
        
        // Blocking endpoints
        .route("/profiles/:id/blocked", get(get_blocked_profiles))
        .route("/profiles/:id/blocked-platforms", get(get_blocked_platforms))
        .route("/blocklist/check/profile/:blocker/:blocked", get(check_profile_blocked))
        .route("/blocklist/check/platform/:profile/:platform", get(check_platform_blocked))
        
        // Platform endpoints
        .route("/platforms", get(get_platforms))
        .route("/platforms/approved", get(get_approved_platforms))
        .route("/platforms/:id", get(get_platform_by_id))
        .route("/platforms/:id/moderators", get(get_platform_moderators))
        .route("/platforms/:id/approval", get(get_platform_approval_status))
        .route("/platforms/:id/blocked", get(get_platform_blocked_profiles))

        // Post endpoints (using TimescaleDB)
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post_by_id))
        .route("/posts/:id/comments", get(get_post_comments))
        .route("/posts/:id/reactions", get(get_post_reactions))
        .route("/posts/:id/reposts", get(get_post_reposts))
        .route("/posts/trending", get(get_trending_posts))
        
        // Promotion endpoints
        .route("/promotions", get(get_promoted_posts))
        .route("/posts/:id/promotion", get(get_post_promotion))
        .route("/promotions/:id/views", get(get_promotion_views))
        .route("/promotions/:id/stats", get(get_promotion_stats))
        
        // TimescaleDB-optimized promotion analytics endpoints
        .route("/promotions/:id/analytics/time-series", get(get_promotion_time_analytics))
        .route("/promotions/:id/analytics/hourly", get(get_promotion_hourly_stats))
        .route("/promotions/analytics/top-performing", get(get_top_performing_promotions))
        .route("/promotions/analytics/spending-trends", get(get_promotion_spending_trends))
        
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
        .route("/posts/:id/revenue-redirections", get(get_post_revenue_redirections))
        
        // Subscription endpoints (using TimescaleDB)
        .route("/subscriptions", get(get_subscriptions))
        .route("/subscription-services", get(get_subscription_services))
        .route("/subscription-revenue", get(get_subscription_revenue))
        .route("/subscriptions/:id/status", get(get_subscription_status))
        .route("/subscription-access/:subscriber/:content_id", get(check_subscription_access))
        .route("/subscription-analytics", get(get_subscription_analytics))
        .route("/service-performance", get(get_service_performance))
        .route("/subscribers/:address/summary", get(get_subscriber_summary))
        
        // Vesting endpoints (using TimescaleDB)
        .route("/vesting/wallets", get(get_vesting_wallets))
        .route("/vesting/wallets/:wallet_id", get(get_vesting_wallet_by_id))
        .route("/vesting/wallets/:wallet_id/events", get(get_vesting_wallet_events))
        .route("/vesting/wallets/:wallet_id/claimable", get(get_vesting_wallet_claimable))
        .route("/vesting/users/:address/wallets", get(get_user_vesting_wallets))
        .route("/vesting/events", get(get_vesting_events))
        .route("/vesting/analytics", get(get_vesting_analytics))
        .route("/vesting/leaderboard", get(get_vesting_leaderboard))
        
        // Unified Revenue Analytics endpoints (using TimescaleDB)
        .route("/revenue/dashboard", get(get_revenue_dashboard))
        .route("/revenue/leaderboard", get(get_revenue_leaderboard))
        .route("/revenue/chart-data", get(get_revenue_chart_data))
        .route("/revenue/unified", get(get_unified_revenue))
        .route("/revenue/creators/:address/stats", get(get_creator_revenue_stats))
        .route("/revenue/platforms/:address/stats", get(get_platform_revenue_stats))
        .route("/revenue/spt/pools/:pool_id", get(get_spt_pool_revenue))
        
        // SPoT (Social Proof of Truth) endpoints
        .route("/spot/:post_id/record", get(get_spot_record))
        .route("/spot/:post_id/bets", get(list_spot_bets))
        .route("/spot/:post_id/payouts", get(list_spot_payouts))
        .route("/spot/:post_id/refunds", get(list_spot_refunds))
        
        // MyIP Marketplace endpoints (using TimescaleDB)
        .route("/marketplace", get(list_marketplace_data))
        .route("/marketplace/popular", get(get_popular_marketplace_data))
        .route("/marketplace/:id", get(get_marketplace_data_by_id))
        .route("/marketplace/:id/purchases", get(get_ip_purchases))
        .route("/marketplace/:id/subscriptions", get(get_ip_subscriptions))
        .route("/marketplace/:id/revenue", get(get_ip_revenue))
        .route("/marketplace/:id/access-logs", get(get_ip_access_logs))
        .route("/marketplace/:id/stats", get(get_marketplace_stats))
        .route("/marketplace/:id/revenue-timeline", get(get_revenue_timeline))
        .route("/marketplace/:id/access-analytics", get(get_access_analytics))
        .route("/creators/:id/marketplace-data", get(get_creator_data))

        // Governance endpoints
        .route("/governance/proposals", get(list_proposals))
        .route("/governance/proposals/:id", get(get_proposal_by_id))
        .route("/governance/proposals/:id/votes", get(get_proposal_community_votes))
        .route("/governance/delegates", get(list_delegates))
        .route("/governance/delegates/:address", get(get_delegate_by_address))
        .route("/governance/delegates/:address/proposals", get(get_delegate_proposals))
        .route("/governance/delegates/:address/ratings", get(get_delegate_ratings))
        .route("/governance/nominees", get(list_nominees))
        .route("/governance/registries", get(list_registries))
        .route("/governance/registries/:registry_type", get(get_registry_by_type))
        .route("/governance/events", get(list_governance_events))
        // Anonymous voting endpoints
        .route("/governance/proposals/:id/anonymous-stats", get(get_proposal_anonymous_stats))
        .route("/governance/proposals/:id/anonymous-votes", get(get_proposal_anonymous_votes))
        .route("/governance/proposals/:id/decryption-failures", get(get_proposal_decryption_failures))
        .route("/governance/anonymous-voting-trends", get(get_anonymous_voting_trends))
        
        // Add shared state - using the pool directly for all standard endpoints
        .with_state(pool);
        
    // Create a separate router for social proof token endpoints with the correct state type
    let spt_router = Router::new()
        .route("/social-proof-token/pools", get(list_spt_pools))
        .route("/social-proof-token/pools/:id", get(get_spt_pool_by_id))
        .route("/social-proof-token/pools/by-associated-id/:id", get(get_spt_pool_by_associated_id))
        .route("/social-proof-token/pools/:id/transactions", get(get_spt_transactions))
        .route("/social-proof-token/pools/:id/holdings", get(get_spt_holdings))
        .route("/social-proof-token/pools/:id/price-history", get(get_spt_price_history))
        .route("/social-proof-token/reservation-pools", get(get_spt_reservation_pools))
        .route("/social-proof-token/reservation-pools/:id", get(get_spt_reservation_pool_by_id))
        .route("/social-proof-token/reservation-pools/:id/reservations", get(get_spt_reservations_by_pool))
        .route("/social-proof-token/popular", get(get_popular_tokens))
        .route("/social-proof-token/users/:address/holdings", get(get_user_spt_holdings))
        .route("/social-proof-token/analytics/top-performers", get(get_top_performing_tokens))
        .route("/social-proof-token/portfolios/:address/performance", get(get_user_portfolio_performance))
        .route("/social-proof-token/creators/:address/revenue-streams", get(get_creator_revenue_streams))
        .route("/social-proof-token/market-sentiment", get(get_market_sentiment))
        .route("/social-proof-token/pools/:id/liquidity-profile", get(get_token_liquidity_profile))
        .route("/search", get(global_search))
        .with_state(db);
    
    // Merge the routers
    main_router.merge(spt_router)
}
