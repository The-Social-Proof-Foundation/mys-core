// Copyright (c) MySocial Team
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
    get_license_by_id,
    list_licenses,
    get_license_events,
    get_license_grants,
    get_license_revenue,
    get_creator_licenses,
    get_license_posts,
    get_license_stats,
    get_revenue_timeline,
    get_popular_licenses,
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
};
// Import social proof token handlers
use crate::api::handlers::social_proof_token::{
    get_spt_pool_by_id,
    list_spt_pools,
    get_spt_pool_by_associated_id,
    get_spt_transactions,
    get_spt_holdings,
    get_spt_price_history,
    get_spt_auctions,
    get_spt_auction_by_id,
    get_spt_auction_contributions,
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

/// Build the application router with all API routes
pub fn build_router(db: Arc<Database>) -> Router {
    // Extract the pool from the Database wrapper
    let pool = (*db.pool).clone();
    
    // Create router with standard endpoints
    let main_router = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        
        // Profile endpoints
        .route("/profiles", get(latest_profiles))
        .route("/profiles/address/:address", get(get_profile_by_address))
        .route("/profiles/username/:username", get(get_profile_by_username))
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
        
        // License (IP) endpoints (using TimescaleDB)
        .route("/licenses", get(list_licenses))
        .route("/licenses/popular", get(get_popular_licenses))
        .route("/licenses/:id", get(get_license_by_id))
        .route("/licenses/:id/events", get(get_license_events))
        .route("/licenses/:id/grants", get(get_license_grants))
        .route("/licenses/:id/revenue", get(get_license_revenue))
        .route("/licenses/:id/posts", get(get_license_posts))
        .route("/licenses/:id/stats", get(get_license_stats))
        .route("/licenses/:id/revenue-timeline", get(get_revenue_timeline))
        .route("/creators/:id/licenses", get(get_creator_licenses))
        
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
        .route("/social-proof-token/auctions", get(get_spt_auctions))
        .route("/social-proof-token/auctions/:id", get(get_spt_auction_by_id))
        .route("/social-proof-token/auctions/:id/contributions", get(get_spt_auction_contributions))
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