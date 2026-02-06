// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::{
    profile_subscription_services, profile_subscriptions, subscription_access_logs,
    subscription_events, subscription_revenue,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ==============================================================================
// PROFILE SUBSCRIPTION SERVICES
// ==============================================================================

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = profile_subscription_services)]
pub struct ProfileSubscriptionService {
    pub service_id: String,
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub active: bool,
    pub subscriber_count: i64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = profile_subscription_services)]
pub struct NewProfileSubscriptionService {
    pub service_id: String,
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub active: bool,
    pub subscriber_count: i64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = profile_subscription_services)]
pub struct UpdateProfileSubscriptionService {
    pub monthly_fee: Option<i64>,
    pub active: Option<bool>,
    pub subscriber_count: Option<i64>,
    pub updated_at: Option<i64>,
}

// ==============================================================================
// PROFILE SUBSCRIPTIONS
// ==============================================================================

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = profile_subscriptions)]
pub struct ProfileSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub cancelled_at: Option<i64>,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = profile_subscriptions)]
pub struct NewProfileSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub cancelled_at: Option<i64>,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = profile_subscriptions)]
pub struct UpdateProfileSubscription {
    pub expires_at: Option<i64>,
    pub auto_renew: Option<bool>,
    pub renewal_balance: Option<i64>,
    pub renewal_count: Option<i64>,
    pub cancelled_at: Option<i64>,
    pub processing_success: Option<bool>,
    pub processing_error: Option<String>,
}

// ==============================================================================
// SUBSCRIPTION EVENTS
// ==============================================================================

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = subscription_events)]
pub struct SubscriptionEvent {
    pub event_type: String,
    pub subscription_id: Option<String>,
    pub service_id: Option<String>,
    pub subscriber: Option<String>,
    pub event_data: serde_json::Value,
    pub event_time: i64,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = subscription_events)]
pub struct NewSubscriptionEvent {
    pub event_type: String,
    pub subscription_id: Option<String>,
    pub service_id: Option<String>,
    pub subscriber: Option<String>,
    pub event_data: serde_json::Value,
    pub event_time: i64,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

// ==============================================================================
// SUBSCRIPTION REVENUE
// ==============================================================================

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = subscription_revenue)]
pub struct SubscriptionRevenue {
    pub service_id: String,
    pub subscription_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub payment_time: i64,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = subscription_revenue)]
pub struct NewSubscriptionRevenue {
    pub service_id: String,
    pub subscription_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub payment_time: i64,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

// ==============================================================================
// SUBSCRIPTION ACCESS LOGS
// ==============================================================================

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = subscription_access_logs)]
pub struct SubscriptionAccessLog {
    pub subscription_id: String,
    pub subscriber: String,
    pub content_type: String,
    pub content_id: String,
    pub access_time: i64,
    pub seal_id: Option<String>,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = subscription_access_logs)]
pub struct NewSubscriptionAccessLog {
    pub subscription_id: String,
    pub subscriber: String,
    pub content_type: String,
    pub content_id: String,
    pub access_time: i64,
    pub seal_id: Option<String>,
    pub time: NaiveDateTime,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

// ==============================================================================
// ANALYTICS STRUCTURES
// ==============================================================================

/// Comprehensive subscription analytics structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionAnalytics {
    pub service_id: String,
    pub total_revenue: i64,
    pub active_subscriptions: i64,
    pub cancelled_subscriptions: i64,
    pub monthly_recurring_revenue: i64,
    pub churn_rate: f64,
    pub average_subscription_duration: f64,
    pub total_renewals: i64,
    pub auto_renewal_rate: f64,
    pub refund_rate: f64,
    pub growth_metrics: Vec<SubscriptionGrowthMetric>,
}

/// Subscription growth metrics over time
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionGrowthMetric {
    pub period: String, // "2024-01-01" format
    pub new_subscriptions: i64,
    pub cancelled_subscriptions: i64,
    pub net_growth: i64,
    pub revenue: i64,
}

/// Revenue breakdown by type
#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueBreakdown {
    pub service_id: String,
    pub subscription_revenue: i64,
    pub renewal_revenue: i64,
    pub auto_renewal_revenue: i64,
    pub refunds: i64,
    pub net_revenue: i64,
    pub period_start: NaiveDateTime,
    pub period_end: NaiveDateTime,
}

/// Subscription with associated service information
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionWithService {
    pub subscription_id: String,
    pub service_id: String,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub cancelled_at: Option<i64>,
    pub transaction_id: String,
    // Service information
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub service_active: bool,
}

/// Subscriber summary with access information
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberSummary {
    pub subscriber: String,
    pub active_subscriptions: Vec<ActiveSubscription>,
    pub total_spent: i64,
    pub total_refunds: i64,
    pub subscription_count: i64,
    pub average_duration: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub profile_owner: String,
    pub monthly_fee: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_count: i64,
}

/// Service performance metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct ServicePerformance {
    pub service_id: String,
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub total_subscribers: i64,
    pub active_subscribers: i64,
    pub total_revenue: i64,
    pub monthly_recurring_revenue: i64,
    pub churn_rate: f64,
    pub average_lifetime_value: f64,
    pub conversion_rate: f64,
}

// ==============================================================================
// HELPER FUNCTIONS
// ==============================================================================

impl ProfileSubscription {
    /// Check if subscription is currently active
    pub fn is_active(&self, current_time: i64) -> bool {
        self.cancelled_at.is_none() && self.expires_at > current_time
    }

    /// Check if subscription is expired
    pub fn is_expired(&self, current_time: i64) -> bool {
        self.expires_at <= current_time
    }

    /// Check if subscription is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled_at.is_some()
    }

    /// Get subscription status as string
    pub fn status(&self, current_time: i64) -> String {
        if self.is_cancelled() {
            "cancelled".to_string()
        } else if self.is_expired(current_time) {
            "expired".to_string()
        } else {
            "active".to_string()
        }
    }

    /// Calculate days until expiration
    pub fn days_until_expiration(&self, current_time: i64) -> Option<i64> {
        if self.is_cancelled() || self.is_expired(current_time) {
            None
        } else {
            let seconds_remaining = (self.expires_at - current_time) / 1000;
            Some(seconds_remaining / (24 * 60 * 60))
        }
    }

    /// Check if subscription is eligible for auto-renewal
    pub fn can_auto_renew(&self, service_fee: i64) -> bool {
        self.auto_renew && self.renewal_balance >= service_fee && self.cancelled_at.is_none()
    }
}

impl ProfileSubscriptionService {
    /// Calculate expected monthly revenue
    pub fn expected_monthly_revenue(&self) -> i64 {
        self.monthly_fee * self.subscriber_count
    }

    /// Check if service accepts new subscriptions
    pub fn accepts_subscriptions(&self) -> bool {
        self.active
    }
}

impl SubscriptionRevenue {
    /// Check if this is a positive revenue transaction
    pub fn is_revenue(&self) -> bool {
        self.amount > 0 && !self.is_refund()
    }

    /// Check if this is a refund transaction
    pub fn is_refund(&self) -> bool {
        self.revenue_type == "refund" || self.amount < 0
    }

    /// Get absolute amount (useful for refunds which are negative)
    pub fn absolute_amount(&self) -> i64 {
        self.amount.abs()
    }
}

/// Constants for subscription validation and business logic
pub const MIN_SUBSCRIPTION_DURATION_DAYS: i64 = 1;
pub const MAX_SUBSCRIPTION_DURATION_DAYS: i64 = 365;
pub const MILLISECONDS_PER_DAY: i64 = 24 * 60 * 60 * 1000;

/// Revenue type constants
pub const REVENUE_TYPE_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_TYPE_RENEWAL: &str = "renewal";
pub const REVENUE_TYPE_AUTO_RENEWAL: &str = "auto_renewal";
pub const REVENUE_TYPE_REFUND: &str = "refund";

/// Content type constants for access logs
pub const CONTENT_TYPE_PROFILE: &str = "profile";
pub const CONTENT_TYPE_POST: &str = "post";

/// Business logic validation functions
pub fn validate_monthly_fee(fee: i64) -> Result<(), String> {
    if fee <= 0 {
        Err("Monthly fee must be positive".to_string())
    } else if fee > 1_000_000_000 {
        Err("Monthly fee exceeds maximum allowed".to_string())
    } else {
        Ok(())
    }
}

pub fn validate_subscription_duration(duration_ms: i64) -> Result<(), String> {
    let duration_days = duration_ms / MILLISECONDS_PER_DAY;
    if duration_days < MIN_SUBSCRIPTION_DURATION_DAYS {
        Err("Subscription duration too short".to_string())
    } else if duration_days > MAX_SUBSCRIPTION_DURATION_DAYS {
        Err("Subscription duration too long".to_string())
    } else {
        Ok(())
    }
}

pub fn calculate_subscription_end_time(start_time: i64, duration_days: i64) -> i64 {
    start_time + (duration_days * MILLISECONDS_PER_DAY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_subscription_is_active() {
        let current_time = Utc::now().timestamp_millis();
        let future_time = current_time + 100000;

        let subscription = ProfileSubscription {
            subscription_id: "test".to_string(),
            service_id: "service".to_string(),
            subscriber: "user".to_string(),
            created_at: current_time,
            expires_at: future_time,
            auto_renew: false,
            renewal_balance: 0,
            renewal_count: 0,
            cancelled_at: None,
            time: Utc::now().naive_utc(),
            transaction_id: "tx".to_string(),
            processing_success: true,
            processing_error: None,
        };

        assert!(subscription.is_active(current_time));
        assert!(!subscription.is_expired(current_time));
        assert!(!subscription.is_cancelled());
    }

    #[test]
    fn test_subscription_is_expired() {
        let current_time = Utc::now().timestamp_millis();
        let past_time = current_time - 100000;

        let subscription = ProfileSubscription {
            subscription_id: "test".to_string(),
            service_id: "service".to_string(),
            subscriber: "user".to_string(),
            created_at: past_time - 100000,
            expires_at: past_time,
            auto_renew: false,
            renewal_balance: 0,
            renewal_count: 0,
            cancelled_at: None,
            time: Utc::now().naive_utc(),
            transaction_id: "tx".to_string(),
            processing_success: true,
            processing_error: None,
        };

        assert!(!subscription.is_active(current_time));
        assert!(subscription.is_expired(current_time));
        assert_eq!(subscription.status(current_time), "expired");
    }

    #[test]
    fn test_subscription_is_cancelled() {
        let current_time = Utc::now().timestamp_millis();
        let future_time = current_time + 100000;

        let subscription = ProfileSubscription {
            subscription_id: "test".to_string(),
            service_id: "service".to_string(),
            subscriber: "user".to_string(),
            created_at: current_time,
            expires_at: future_time,
            auto_renew: false,
            renewal_balance: 0,
            renewal_count: 0,
            cancelled_at: Some(current_time),
            time: Utc::now().naive_utc(),
            transaction_id: "tx".to_string(),
            processing_success: true,
            processing_error: None,
        };

        assert!(!subscription.is_active(current_time));
        assert!(subscription.is_cancelled());
        assert_eq!(subscription.status(current_time), "cancelled");
    }

    #[test]
    fn test_validate_monthly_fee() {
        assert!(validate_monthly_fee(1000).is_ok());
        assert!(validate_monthly_fee(-100).is_err());
        assert!(validate_monthly_fee(0).is_err());
        assert!(validate_monthly_fee(2_000_000_000).is_err());
    }

    #[test]
    fn test_revenue_type_detection() {
        let revenue = SubscriptionRevenue {
            service_id: "service".to_string(),
            subscription_id: Some("sub".to_string()),
            from_address: "from".to_string(),
            to_address: "to".to_string(),
            amount: 1000,
            revenue_type: "subscription".to_string(),
            payment_time: 0,
            time: Utc::now().naive_utc(),
            transaction_id: "tx".to_string(),
            processing_success: true,
            processing_error: None,
        };

        assert!(revenue.is_revenue());
        assert!(!revenue.is_refund());

        let refund = SubscriptionRevenue {
            service_id: "service".to_string(),
            subscription_id: Some("sub".to_string()),
            from_address: "from".to_string(),
            to_address: "to".to_string(),
            amount: -500,
            revenue_type: "refund".to_string(),
            payment_time: 0,
            time: Utc::now().naive_utc(),
            transaction_id: "tx".to_string(),
            processing_success: true,
            processing_error: None,
        };

        assert!(!refund.is_revenue());
        assert!(refund.is_refund());
        assert_eq!(refund.absolute_amount(), 500);
    }
}
