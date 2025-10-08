// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Import tables from schema
use crate::schema::{spt_revenue, unified_revenue};

// ============================================================================
// CONSTANTS
// ============================================================================

// SPT Revenue Transaction Types
pub const SPT_TRANSACTION_TYPE_BUY: &str = "buy";
pub const SPT_TRANSACTION_TYPE_SELL: &str = "sell";

// Revenue Sources
pub const REVENUE_SOURCE_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_SOURCE_MY_IP: &str = "my_ip";
pub const REVENUE_SOURCE_SPT: &str = "spt";
pub const REVENUE_SOURCE_TIPS: &str = "tips";
pub const REVENUE_SOURCE_POSTS: &str = "posts";

// Revenue Types by Source
pub const REVENUE_TYPE_SUBSCRIPTION_MONTHLY: &str = "monthly";
pub const REVENUE_TYPE_SUBSCRIPTION_RENEWAL: &str = "renewal";
pub const REVENUE_TYPE_SUBSCRIPTION_AUTO_RENEWAL: &str = "auto_renewal";
pub const REVENUE_TYPE_SUBSCRIPTION_REFUND: &str = "refund";

pub const REVENUE_TYPE_MYIP_ONE_TIME: &str = "one_time";
pub const REVENUE_TYPE_MYIP_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_TYPE_MYIP_GRANT: &str = "grant";

pub const REVENUE_TYPE_SPT_CREATOR_FEE: &str = "creator_fee";
pub const REVENUE_TYPE_SPT_PLATFORM_FEE: &str = "platform_fee";
pub const REVENUE_TYPE_SPT_TREASURY_FEE: &str = "treasury_fee";

pub const REVENUE_TYPE_TIPS_POST: &str = "post_tip";
pub const REVENUE_TYPE_TIPS_PROFILE: &str = "profile_tip";
pub const REVENUE_TYPE_TIPS_COMMENT: &str = "comment_tip";

pub const REVENUE_TYPE_POSTS_MONETIZATION: &str = "post_monetization";
pub const REVENUE_TYPE_POSTS_PREMIUM: &str = "premium_content";

// Content Types
pub const CONTENT_TYPE_POST: &str = "post";
pub const CONTENT_TYPE_PROFILE: &str = "profile";
pub const CONTENT_TYPE_SERVICE: &str = "service";
pub const CONTENT_TYPE_DATA: &str = "data";
pub const CONTENT_TYPE_TOKEN: &str = "token";
pub const CONTENT_TYPE_COMMENT: &str = "comment";

// Currency
pub const CURRENCY_MYSO: &str = "MYSO";

// MySo token has 9 decimal places
pub const MYSO_DECIMAL_PLACES: u32 = 9;
pub const MYSO_DECIMAL_FACTOR: i64 = 1_000_000_000; // 10^9

// ============================================================================
// SPT REVENUE MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = spt_revenue)]
#[diesel(primary_key(pool_id, time))]
pub struct SptRevenue {
    pub pool_id: String,
    pub transaction_type: String,
    pub trader: String,
    pub creator_address: String,
    pub platform_address: String,
    pub treasury_address: String,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub total_fee: i64,
    pub token_amount: i64,
    pub mys_amount: i64,
    pub token_price: i64,
    pub revenue_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = spt_revenue)]
pub struct NewSptRevenue {
    pub pool_id: String,
    pub transaction_type: String,
    pub trader: String,
    pub creator_address: String,
    pub platform_address: String,
    pub treasury_address: String,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub total_fee: i64,
    pub token_amount: i64,
    pub mys_amount: i64,
    pub token_price: i64,
    pub revenue_time: i64,
    pub transaction_id: String,
}

// ============================================================================
// UNIFIED REVENUE MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = unified_revenue)]
#[diesel(primary_key(revenue_source, time))]
pub struct UnifiedRevenue {
    pub revenue_source: String,
    pub revenue_type: String,
    pub creator_address: String,
    pub platform_address: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub payer_address: String,
    pub recipient_address: String,
    pub revenue_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = unified_revenue)]
pub struct NewUnifiedRevenue {
    pub revenue_source: String,
    pub revenue_type: String,
    pub creator_address: String,
    pub platform_address: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub payer_address: String,
    pub recipient_address: String,
    pub revenue_time: i64,
    pub transaction_id: String,
}

// ============================================================================
// AGGREGATED REVENUE MODELS (for API responses)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorRevenueStats {
    pub creator_address: String,
    pub total_revenue: i64,
    pub subscription_revenue: i64,
    pub myip_revenue: i64,
    pub spt_revenue: i64,
    pub tips_revenue: i64,
    pub posts_revenue: i64,
    pub total_transactions: i64,
    pub unique_payers: i64,
    pub largest_transaction: i64,
    pub active_days: i64,
    pub last_revenue_date: Option<DateTime<Utc>>,
    pub revenue_rank: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRevenueStats {
    pub platform_address: String,
    pub total_revenue: i64,
    pub subscription_revenue: i64,
    pub myip_revenue: i64,
    pub spt_revenue: i64,
    pub total_transactions: i64,
    pub unique_creators: i64,
    pub unique_payers: i64,
    pub avg_transaction_amount: f64,
    pub active_months: i64,
    pub last_active_month: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueTimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub revenue_source: String,
    pub total_revenue: i64,
    pub transaction_count: i64,
    pub unique_creators: i64,
    pub unique_payers: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueLeaderboardEntry {
    pub rank: i64,
    pub creator_address: String,
    pub total_revenue: i64,
    pub revenue_breakdown: RevenueBreakdown,
    pub growth_rate: Option<f64>, // 30-day growth rate
    pub transaction_count: i64,
    pub unique_payers: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueBreakdown {
    pub subscription_revenue: i64,
    pub myip_revenue: i64,
    pub spt_revenue: i64,
    pub tips_revenue: i64,
    pub posts_revenue: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDashboard {
    pub total_revenue_24h: i64,
    pub total_transactions_24h: i64,
    pub unique_creators_24h: i64,
    pub unique_payers_24h: i64,
    pub largest_transaction_24h: i64,
    pub revenue_by_source: Vec<RevenueSourceStats>,
    pub top_creators: Vec<RevenueLeaderboardEntry>,
    pub recent_trends: Vec<RevenueTimeSeriesPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSourceStats {
    pub revenue_source: String,
    pub total_revenue: i64,
    pub transaction_count: i64,
    pub percentage_of_total: f64,
    pub growth_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SptRevenueStats {
    pub pool_id: String,
    pub creator_address: String,
    pub total_fees: i64,
    pub creator_fees: i64,
    pub platform_fees: i64,
    pub treasury_fees: i64,
    pub total_volume: i64,
    pub total_tokens: i64,
    pub transaction_count: i64,
    pub unique_traders: i64,
    pub avg_price: f64,
    pub max_price: i64,
    pub min_price: i64,
    pub buy_volume: i64,
    pub sell_volume: i64,
    pub net_flow: i64, // buy_volume - sell_volume
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

impl NewSptRevenue {
    /// Create SPT revenue from buy event
    pub fn from_buy_event(
        pool_id: String,
        trader: String,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        token_amount: i64,
        mys_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            pool_id,
            transaction_type: SPT_TRANSACTION_TYPE_BUY.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee + platform_fee + treasury_fee,
            token_amount,
            mys_amount,
            token_price,
            revenue_time,
            transaction_id,
        }
    }

    /// Create SPT revenue from sell event
    pub fn from_sell_event(
        pool_id: String,
        trader: String,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        token_amount: i64,
        mys_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            pool_id,
            transaction_type: SPT_TRANSACTION_TYPE_SELL.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee + platform_fee + treasury_fee,
            token_amount,
            mys_amount,
            token_price,
            revenue_time,
            transaction_id,
        }
    }
}

impl NewUnifiedRevenue {
    /// Create unified revenue from subscription
    pub fn from_subscription(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        service_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_SUBSCRIPTION.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(service_id),
            content_type: Some(CONTENT_TYPE_SERVICE.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            transaction_id,
        }
    }

    /// Create unified revenue from MyIP
    pub fn from_myip(
        revenue_type: String,
        creator_address: String,
        amount: i64,
        mydata_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_MY_IP.to_string(),
            revenue_type,
            creator_address,
            platform_address: None,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(mydata_id),
            content_type: Some(CONTENT_TYPE_DATA.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            transaction_id,
        }
    }

    /// Create unified revenue from SPT fees
    pub fn from_spt(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        pool_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_SPT.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(pool_id),
            content_type: Some(CONTENT_TYPE_TOKEN.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            transaction_id,
        }
    }

    /// Create unified revenue from tips
    pub fn from_tip(
        revenue_type: String,
        creator_address: String,
        amount: i64,
        content_id: String,
        content_type: String,
        payer_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_TIPS.to_string(),
            revenue_type,
            creator_address: creator_address.clone(),
            platform_address: None,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(content_id),
            content_type: Some(content_type),
            payer_address,
            recipient_address: creator_address,
            revenue_time,
            transaction_id,
        }
    }

    /// Create unified revenue from posts
    pub fn from_post(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        post_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_POSTS.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(post_id),
            content_type: Some(CONTENT_TYPE_POST.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            transaction_id,
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Convert MySo amount from blockchain units to human-readable decimal
pub fn myso_from_blockchain_units(amount: i64) -> f64 {
    amount as f64 / MYSO_DECIMAL_FACTOR as f64
}

/// Convert MySo amount from human-readable decimal to blockchain units
pub fn myso_to_blockchain_units(amount: f64) -> i64 {
    (amount * MYSO_DECIMAL_FACTOR as f64) as i64
}

/// Format MySo amount for display (with proper decimal places)
pub fn format_myso_amount(amount: i64) -> String {
    let decimal_amount = myso_from_blockchain_units(amount);
    format!("{:.4} MYSO", decimal_amount)
}

/// Calculate percentage of total
pub fn calculate_percentage(part: i64, total: i64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// Calculate growth rate between two periods
pub fn calculate_growth_rate(current: i64, previous: i64) -> Option<f64> {
    if previous == 0 {
        None
    } else {
        Some(((current - previous) as f64 / previous as f64) * 100.0)
    }
}
