// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod blocking;
pub mod governance;
pub mod indexer;
pub mod my_ip;
pub mod platform;
pub mod poc;
pub mod post;
pub mod profile;
pub mod profile_events;
pub mod revenue;
pub mod social_graph;
pub mod social_proof_of_truth;
pub mod social_proof_token;
pub mod social_proof_tokens_config;
pub mod subscription;
pub mod vesting;

pub use indexer::*;
pub use profile::*;
pub use social_graph::*;
// Explicitly import what we need from platform and avoid ambiguous re-exports
pub use blocking::*;
pub use my_ip::*;
pub use platform::{
    ModeratorAddedEvent, ModeratorRemovedEvent, NewPlatform, NewPlatformMembership,
    NewPlatformModerator, Platform, PlatformApprovalChangedEvent, PlatformCreatedEvent,
    PlatformModerator, PlatformStatus, PlatformUpdatedEvent, PlatformWithDetails, UpdatePlatform,
    UserJoinedPlatformEvent, UserLeftPlatformEvent, PLATFORM_STATUS_ALPHA, PLATFORM_STATUS_BETA,
    PLATFORM_STATUS_DEVELOPMENT, PLATFORM_STATUS_LIVE, PLATFORM_STATUS_MAINTENANCE,
    PLATFORM_STATUS_SHUTDOWN, PLATFORM_STATUS_SUNSET,
};
pub use poc::*;
pub use post::*;
pub use profile_events::*;
pub use social_proof_token::*;
// Explicitly import subscription types to avoid conflict with my_ip::REVENUE_TYPE_SUBSCRIPTION
pub use subscription::{
    calculate_subscription_end_time, validate_monthly_fee, validate_subscription_duration,
    ActiveSubscription, NewProfileSubscription, NewProfileSubscriptionService,
    NewSubscriptionAccessLog, NewSubscriptionEvent, NewSubscriptionRevenue, ProfileSubscription,
    ProfileSubscriptionService, RevenueBreakdown, ServicePerformance, SubscriberSummary,
    SubscriptionAccessLog, SubscriptionAnalytics, SubscriptionEvent, SubscriptionGrowthMetric,
    SubscriptionRevenue, SubscriptionWithService, UpdateProfileSubscription,
    UpdateProfileSubscriptionService, CONTENT_TYPE_POST, CONTENT_TYPE_PROFILE,
    MAX_SUBSCRIPTION_DURATION_DAYS, MILLISECONDS_PER_DAY, MIN_SUBSCRIPTION_DURATION_DAYS,
    REVENUE_TYPE_AUTO_RENEWAL, REVENUE_TYPE_REFUND, REVENUE_TYPE_RENEWAL,
};
// Import revenue types with specific naming to avoid conflicts
pub use revenue::{
    calculate_growth_rate,
    calculate_percentage,
    format_myso_amount,
    myso_from_blockchain_units,
    myso_to_blockchain_units,
    CreatorRevenueStats,
    NewSptRevenue,
    NewUnifiedRevenue,
    PlatformRevenueStats,
    RevenueBreakdown as RevenueBreakdownUnified,
    RevenueDashboard,
    RevenueLeaderboardEntry,
    RevenueSourceStats,
    RevenueTimeSeriesPoint,
    SptRevenue,
    SptRevenueStats,
    UnifiedRevenue,
    CURRENCY_MYSO,
    MYSO_DECIMAL_FACTOR,
    MYSO_DECIMAL_PLACES,
    REVENUE_SOURCE_MY_IP,
    REVENUE_SOURCE_POSTS,
    REVENUE_SOURCE_SPT,
    REVENUE_SOURCE_SUBSCRIPTION,
    REVENUE_SOURCE_TIPS,
    REVENUE_TYPE_SPT_CREATOR_FEE,
    REVENUE_TYPE_SPT_PLATFORM_FEE,
    REVENUE_TYPE_SPT_TREASURY_FEE,
    REVENUE_TYPE_TIPS_COMMENT,
    REVENUE_TYPE_TIPS_POST,
    REVENUE_TYPE_TIPS_PROFILE,
    // Constants with specific prefixes to avoid conflicts
    SPT_TRANSACTION_TYPE_BUY,
    SPT_TRANSACTION_TYPE_SELL,
};
pub use social_proof_tokens_config::*;
// Import vesting types
pub use vesting::{
    NewVestingEvent, NewVestingWallet, UpdateVestingWallet, VestingEvent, VestingWallet,
    VestingWalletWithStatus, CURVE_FACTOR_LINEAR, CURVE_FACTOR_MAX, CURVE_FACTOR_MIN,
    VESTING_EVENT_TYPE_CLAIMED, VESTING_EVENT_TYPE_VESTED,
};
// SPoT types
pub use social_proof_of_truth::NewSocialProofOfTruthEvent;
pub use social_proof_of_truth::*;
