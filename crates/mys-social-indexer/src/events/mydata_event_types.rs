// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::events::event_utils::{deserialize_u64_from_string, deserialize_optional_u64_from_string};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// MARKETPLACE EVENT TYPES
// ============================================================================

/// Event emitted when new MyData is created (from smart contract)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyDataCreatedEvent {
    pub ip_id: String,
    pub owner: String,
    pub media_type: String,
    pub platform_id: Option<String>,
    pub one_time_price: Option<u64>,
    pub subscription_price: Option<u64>,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub created_at: u64,
}

/// Event emitted when MyData is purchased (one-time or subscription)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseEvent {
    pub ip_id: String,
    pub buyer: String,
    pub price: u64,
    pub purchase_type: String, // "one_time" or "subscription"
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Event emitted when access is granted (pricing update, content update, or free access)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrantedEvent {
    pub ip_id: String,
    pub user: String,
    pub access_type: String, // "pricing_update", "content_update", "one_time", "subscription"
    pub granted_by: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Event emitted when new data is added to the marketplace (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCreatedEvent {
    pub mydata_id: String,
    pub owner: String,
    pub media_type: String,
    pub tags: Vec<String>,
    pub platform_id: Option<String>,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp_start: u64,
    #[serde(deserialize_with = "deserialize_optional_u64_from_string")]
    pub timestamp_end: Option<u64>,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub created_at: u64,
    pub one_time_price: Option<u64>,
    pub subscription_price: Option<u64>,
    pub subscription_duration_days: u64,
    pub geographic_region: Option<String>,
    pub data_quality: Option<String>,
    pub sample_size: Option<u64>,
    pub collection_method: Option<String>,
    pub is_updating: bool,
    pub update_frequency: Option<String>,
}

/// Event emitted when data metadata is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUpdatedEvent {
    pub mydata_id: String,
    pub updater: String,
    pub old_tags: Vec<String>,
    pub new_tags: Vec<String>,
    pub old_price_one_time: Option<u64>,
    pub new_price_one_time: Option<u64>,
    pub old_price_subscription: Option<u64>,
    pub new_price_subscription: Option<u64>,
    pub old_data_quality: Option<String>,
    pub new_data_quality: Option<String>,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub last_updated: u64,
}

/// Event emitted when data ownership is transferred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransferredEvent {
    pub mydata_id: String,
    pub from_owner: String,
    pub to_owner: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub transfer_time: u64,
    pub transfer_price: Option<u64>,
}

/// Event emitted when data is purchased (one-time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPurchasedEvent {
    pub mydata_id: String,
    pub buyer: String,
    pub price: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub purchase_time: u64,
    pub payment_token: Option<String>,
}

/// Event emitted when a subscription is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCreatedEvent {
    pub mydata_id: String,
    pub subscriber: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub subscription_start: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub subscription_end: u64,
    pub price: u64,
    pub payment_token: Option<String>,
}

/// Event emitted when a subscription is renewed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRenewedEvent {
    pub mydata_id: String,
    pub subscriber: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub old_subscription_end: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub new_subscription_end: u64,
    pub renewal_price: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub renewal_time: u64,
}

/// Event emitted when a subscription is cancelled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCancelledEvent {
    pub mydata_id: String,
    pub subscriber: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub cancellation_time: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub effective_end_time: u64, // When access actually ends
    pub refund_amount: Option<u64>,
}

/// Event emitted when data access is granted (free or special access)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessGrantedEvent {
    pub mydata_id: String,
    pub grantor: String,
    pub grantee: String,
    pub access_type: String, // "preview", "grant", etc.
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub grant_time: u64,
    #[serde(deserialize_with = "deserialize_optional_u64_from_string")]
    pub expiration_time: Option<u64>,
}

/// Event emitted when revenue is distributed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributedEvent {
    pub mydata_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
    pub revenue_type: String, // "one_time", "subscription", "grant"
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub distribution_time: u64,
    pub transaction_hash: Option<String>,
}

/// Event emitted when data is accessed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessedEvent {
    pub mydata_id: String,
    pub user_address: String,
    pub access_type: String, // "one_time", "subscription", "grant", "preview"
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub access_time: u64,
    pub session_id: Option<String>,
}

/// Event emitted when data pricing is changed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPricingChangedEvent {
    pub mydata_id: String,
    pub owner: String,
    pub old_one_time_price: Option<u64>,
    pub new_one_time_price: Option<u64>,
    pub old_subscription_price: Option<u64>,
    pub new_subscription_price: Option<u64>,
    pub old_subscription_duration: u64,
    pub new_subscription_duration: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub change_time: u64,
}

/// Event emitted when data is removed from marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRemovedEvent {
    pub mydata_id: String,
    pub owner: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub removal_time: u64,
    pub removal_reason: Option<String>,
}

// ============================================================================
// ANALYTICS EVENTS
// ============================================================================

/// Event emitted for analytics aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_type: String,
    pub mydata_id: String,
    pub user_address: Option<String>,
    pub metadata: Value,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Event emitted when data becomes trending
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTrendingEvent {
    pub mydata_id: String,
    pub media_type: String,
    pub trending_score: f64,
    pub unique_purchasers_24h: u64,
    pub revenue_24h: u64,
    pub access_count_24h: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

// ============================================================================
// ERROR AND SYSTEM EVENTS
// ============================================================================

/// Event emitted when an operation fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFailedEvent {
    pub operation_type: String,
    pub ip_id: Option<String>,
    pub user_address: Option<String>,
    pub error_code: String,
    pub error_message: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Event emitted for system maintenance operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMaintenanceEvent {
    pub maintenance_type: String,
    pub affected_data: Vec<String>, // List of ip_ids
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub start_time: u64,
    #[serde(deserialize_with = "deserialize_optional_u64_from_string")]
    pub end_time: Option<u64>,
    pub maintenance_reason: String,
}

// ============================================================================
// EVENT CONSTANTS
// ============================================================================

// Event type constants for easy reference
pub const EVENT_DATA_CREATED: &str = "data_created";
pub const EVENT_DATA_UPDATED: &str = "data_updated";
pub const EVENT_DATA_TRANSFERRED: &str = "data_transferred";
pub const EVENT_DATA_PURCHASED: &str = "data_purchased";
pub const EVENT_SUBSCRIPTION_CREATED: &str = "subscription_created";
pub const EVENT_SUBSCRIPTION_RENEWED: &str = "subscription_renewed";
pub const EVENT_SUBSCRIPTION_CANCELLED: &str = "subscription_cancelled";
pub const EVENT_DATA_ACCESS_GRANTED: &str = "data_access_granted";
pub const EVENT_REVENUE_DISTRIBUTED: &str = "revenue_distributed";
pub const EVENT_DATA_ACCESSED: &str = "data_accessed";
pub const EVENT_DATA_PRICING_CHANGED: &str = "data_pricing_changed";
pub const EVENT_DATA_REMOVED: &str = "data_removed";
pub const EVENT_DATA_TRENDING: &str = "data_trending";
pub const EVENT_OPERATION_FAILED: &str = "operation_failed";
pub const EVENT_SYSTEM_MAINTENANCE: &str = "system_maintenance";

// Media type constants
pub const MEDIA_TYPE_TEXT: &str = "text";
pub const MEDIA_TYPE_IMAGE: &str = "image";
pub const MEDIA_TYPE_VIDEO: &str = "video";
pub const MEDIA_TYPE_AUDIO: &str = "audio";
pub const MEDIA_TYPE_SOCIAL: &str = "social";
pub const MEDIA_TYPE_FINANCIAL: &str = "financial";
pub const MEDIA_TYPE_IOT: &str = "iot";
pub const MEDIA_TYPE_ANALYTICS: &str = "analytics";
pub const MEDIA_TYPE_MIXED: &str = "mixed";

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

impl DataCreatedEvent {
    pub fn to_analytics_event(&self) -> AnalyticsEvent {
        AnalyticsEvent {
            event_type: EVENT_DATA_CREATED.to_string(),
            mydata_id: self.mydata_id.clone(),
            user_address: Some(self.owner.clone()),
            metadata: serde_json::to_value(self).unwrap_or(Value::Null),
            timestamp: self.created_at,
        }
    }
}

impl DataPurchasedEvent {
    pub fn to_analytics_event(&self) -> AnalyticsEvent {
        AnalyticsEvent {
            event_type: EVENT_DATA_PURCHASED.to_string(),
            mydata_id: self.mydata_id.clone(),
            user_address: Some(self.buyer.clone()),
            metadata: serde_json::to_value(self).unwrap_or(Value::Null),
            timestamp: self.purchase_time,
        }
    }
}

impl SubscriptionCreatedEvent {
    pub fn to_analytics_event(&self) -> AnalyticsEvent {
        AnalyticsEvent {
            event_type: EVENT_SUBSCRIPTION_CREATED.to_string(),
            mydata_id: self.mydata_id.clone(),
            user_address: Some(self.subscriber.clone()),
            metadata: serde_json::to_value(self).unwrap_or(Value::Null),
            timestamp: self.subscription_start,
        }
    }
}
