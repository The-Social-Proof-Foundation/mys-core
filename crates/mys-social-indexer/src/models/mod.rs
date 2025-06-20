// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

pub mod profile;
pub mod indexer;
pub mod social_graph;
pub mod platform;
pub mod blocking;
pub mod profile_events;
pub mod post;
pub mod governance;
pub mod my_ip;
pub mod social_proof_token;
pub mod poc;
pub mod subscription;
pub mod revenue;
pub mod token_exchange;

pub use profile::*;
pub use indexer::*;
pub use social_graph::*;
// Explicitly import what we need from platform and avoid ambiguous re-exports
pub use platform::{
    Platform, NewPlatform, UpdatePlatform, 
    PlatformModerator, NewPlatformModerator,
    PlatformWithDetails, PlatformCreatedEvent, PlatformApprovalChangedEvent,
    PlatformUpdatedEvent, PlatformStatus, ModeratorAddedEvent, ModeratorRemovedEvent,
    UserJoinedPlatformEvent, UserLeftPlatformEvent,
    NewPlatformMembership,
    PLATFORM_STATUS_DEVELOPMENT, PLATFORM_STATUS_ALPHA, PLATFORM_STATUS_BETA,
    PLATFORM_STATUS_LIVE, PLATFORM_STATUS_MAINTENANCE, PLATFORM_STATUS_SUNSET, PLATFORM_STATUS_SHUTDOWN
};
pub use blocking::*;
pub use profile_events::*;
pub use post::*;
pub use my_ip::*;
pub use social_proof_token::*;
pub use poc::*;
// Explicitly import subscription types to avoid conflict with my_ip::REVENUE_TYPE_SUBSCRIPTION
pub use subscription::{
    ProfileSubscription, NewProfileSubscription, UpdateProfileSubscription,
    ProfileSubscriptionService, NewProfileSubscriptionService, UpdateProfileSubscriptionService,
    SubscriptionEvent, NewSubscriptionEvent,
    SubscriptionRevenue, NewSubscriptionRevenue,
    SubscriptionAccessLog, NewSubscriptionAccessLog,
    SubscriptionAnalytics, SubscriptionGrowthMetric, RevenueBreakdown,
    SubscriptionWithService, SubscriberSummary, ActiveSubscription, ServicePerformance,
    MIN_SUBSCRIPTION_DURATION_DAYS, MAX_SUBSCRIPTION_DURATION_DAYS, MILLISECONDS_PER_DAY,
    REVENUE_TYPE_RENEWAL, REVENUE_TYPE_AUTO_RENEWAL, REVENUE_TYPE_REFUND,
    CONTENT_TYPE_PROFILE, CONTENT_TYPE_POST,
    validate_monthly_fee, validate_subscription_duration, calculate_subscription_end_time
};
// Import revenue types with specific naming to avoid conflicts
pub use revenue::{
    SptRevenue, NewSptRevenue, UnifiedRevenue, NewUnifiedRevenue,
    CreatorRevenueStats, PlatformRevenueStats, RevenueTimeSeriesPoint,
    RevenueLeaderboardEntry, RevenueBreakdown as RevenueBreakdownUnified, 
    RevenueDashboard, RevenueSourceStats, SptRevenueStats,
    // Constants with specific prefixes to avoid conflicts
    SPT_TRANSACTION_TYPE_BUY, SPT_TRANSACTION_TYPE_SELL,
    REVENUE_SOURCE_SUBSCRIPTION, REVENUE_SOURCE_MY_IP, REVENUE_SOURCE_SPT,
    REVENUE_SOURCE_TIPS, REVENUE_SOURCE_POSTS,
    REVENUE_TYPE_SPT_CREATOR_FEE, REVENUE_TYPE_SPT_PLATFORM_FEE, REVENUE_TYPE_SPT_TREASURY_FEE,
    REVENUE_TYPE_TIPS_POST, REVENUE_TYPE_TIPS_PROFILE, REVENUE_TYPE_TIPS_COMMENT,
    CURRENCY_MYSO, MYSO_DECIMAL_PLACES, MYSO_DECIMAL_FACTOR,
    myso_from_blockchain_units, myso_to_blockchain_units, format_myso_amount,
    calculate_percentage, calculate_growth_rate
};
pub use token_exchange::*;
