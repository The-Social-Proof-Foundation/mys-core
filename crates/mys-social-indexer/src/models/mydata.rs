// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Import tables from schema
use crate::schema::{
    mydata_access_logs, mydata_config, mydata_data, mydata_purchases, mydata_registry,
    mydata_revenue, mydata_subscriptions,
};

// ============================================================================
// MARKETPLACE DATA MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, PartialEq)]
#[diesel(table_name = mydata_data)]
#[diesel(primary_key(mydata_id))]
pub struct MyDataData {
    pub mydata_id: String,
    pub owner: String,
    pub media_type: String,
    pub tags: Value,
    pub platform_id: Option<String>,
    pub timestamp_start: i64,
    pub timestamp_end: Option<i64>,
    pub created_at: i64,
    pub last_updated: i64,
    pub one_time_price: Option<i64>,
    pub subscription_price: Option<i64>,
    pub subscription_duration_days: i64,
    pub geographic_region: Option<String>,
    pub data_quality: Option<String>,
    pub sample_size: Option<i64>,
    pub collection_method: Option<String>,
    pub is_updating: bool,
    pub update_frequency: Option<String>,
    pub version: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_data)]
pub struct NewMyDataData {
    pub mydata_id: String,
    pub owner: String,
    pub media_type: String,
    pub tags: Value,
    pub platform_id: Option<String>,
    pub timestamp_start: i64,
    pub timestamp_end: Option<i64>,
    pub created_at: i64,
    pub last_updated: i64,
    pub one_time_price: Option<i64>,
    pub subscription_price: Option<i64>,
    pub subscription_duration_days: i64,
    pub geographic_region: Option<String>,
    pub data_quality: Option<String>,
    pub sample_size: Option<i64>,
    pub collection_method: Option<String>,
    pub is_updating: bool,
    pub update_frequency: Option<String>,
    pub version: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = mydata_purchases)]
pub struct MyDataPurchase {
    pub id: i32,
    pub mydata_id: String,
    pub buyer: String,
    pub price: i64,
    pub purchase_type: String,
    pub purchase_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_purchases)]
pub struct NewMyDataPurchase {
    pub mydata_id: String,
    pub buyer: String,
    pub price: i64,
    pub purchase_type: String,
    pub purchase_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = mydata_subscriptions)]
pub struct MyDataSubscription {
    pub id: i32,
    pub mydata_id: String,
    pub subscriber: String,
    pub subscription_start: i64,
    pub subscription_end: i64,
    pub price: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_subscriptions)]
pub struct NewMyDataSubscription {
    pub mydata_id: String,
    pub subscriber: String,
    pub subscription_start: i64,
    pub subscription_end: i64,
    pub price: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = mydata_revenue)]
pub struct MyDataRevenue {
    pub id: i32,
    pub mydata_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_revenue)]
pub struct NewMyDataRevenue {
    pub mydata_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = mydata_access_logs)]
pub struct MyDataAccessLog {
    pub id: i32,
    pub mydata_id: String,
    pub user_address: String,
    pub access_type: String,
    pub access_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_access_logs)]
pub struct NewMyDataAccessLog {
    pub mydata_id: String,
    pub user_address: String,
    pub access_type: String,
    pub access_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = mydata_registry)]
#[diesel(primary_key(ip_id))]
pub struct MyDataRegistry {
    pub ip_id: String,
    pub owner: String,
    pub registered_at: i64,
    pub unregistered_at: Option<i64>,
    pub is_active: bool,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_registry)]
pub struct NewMyDataRegistry {
    pub ip_id: String,
    pub owner: String,
    pub registered_at: i64,
    pub unregistered_at: Option<i64>,
    pub is_active: bool,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, PartialEq)]
#[diesel(table_name = mydata_config)]
#[diesel(primary_key(id, time))]
pub struct MyDataConfig {
    pub id: i32,
    pub updated_by: String,
    pub enable_flag: bool,
    pub max_tags: i64,
    pub max_subscription_days: i64,
    pub max_free_access_grants: i64,
    pub timestamp_ms: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mydata_config)]
pub struct NewMyDataConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub max_tags: i64,
    pub max_subscription_days: i64,
    pub max_free_access_grants: i64,
    pub timestamp_ms: i64,
    pub transaction_id: String,
}

// ============================================================================
// MARKETPLACE CONSTANTS
// ============================================================================

// Purchase types
pub const PURCHASE_TYPE_ONE_TIME: &str = "one_time";
pub const PURCHASE_TYPE_SUBSCRIPTION: &str = "subscription";

// Revenue types
pub const REVENUE_TYPE_ONE_TIME: &str = "one_time";
pub const REVENUE_TYPE_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_TYPE_GRANT: &str = "grant";

// Access types
pub const ACCESS_TYPE_ONE_TIME: &str = "one_time";
pub const ACCESS_TYPE_SUBSCRIPTION: &str = "subscription";
pub const ACCESS_TYPE_GRANT: &str = "grant";
pub const ACCESS_TYPE_PREVIEW: &str = "preview";

// Data quality levels
pub const DATA_QUALITY_HIGH: &str = "high";
pub const DATA_QUALITY_MEDIUM: &str = "medium";
pub const DATA_QUALITY_LOW: &str = "low";

// Update frequencies
pub const UPDATE_FREQUENCY_HOURLY: &str = "hourly";
pub const UPDATE_FREQUENCY_DAILY: &str = "daily";
pub const UPDATE_FREQUENCY_WEEKLY: &str = "weekly";
pub const UPDATE_FREQUENCY_MONTHLY: &str = "monthly";
pub const UPDATE_FREQUENCY_YEARLY: &str = "yearly";

// ============================================================================
// MARKETPLACE BUSINESS LOGIC
// ============================================================================

impl MyDataData {
    // Check if data is currently valid/available
    pub fn is_current(&self, current_time: i64) -> bool {
        if let Some(end_time) = self.timestamp_end {
            current_time <= end_time
        } else {
            true
        }
    }

    // Get pricing model
    pub fn pricing_model(&self) -> String {
        match (
            self.one_time_price.is_some(),
            self.subscription_price.is_some(),
        ) {
            (true, true) => "both".to_string(),
            (true, false) => "one_time".to_string(),
            (false, true) => "subscription".to_string(),
            (false, false) => "free".to_string(),
        }
    }

    // Check if data has pricing
    pub fn is_free(&self) -> bool {
        self.one_time_price.is_none() && self.subscription_price.is_none()
    }

    // Check if user has access based on current time
    pub fn has_subscription_access(&self, subscription_end: i64, current_time: i64) -> bool {
        subscription_end >= current_time
    }
}

impl MyDataSubscription {
    // Check if subscription is currently active
    pub fn is_active(&self, current_time: i64) -> bool {
        current_time >= self.subscription_start && current_time <= self.subscription_end
    }

    // Get remaining subscription time in seconds
    pub fn remaining_time(&self, current_time: i64) -> i64 {
        if self.is_active(current_time) {
            self.subscription_end - current_time
        } else {
            0
        }
    }
}

// ============================================================================
// API RESPONSE TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyDataDataWithStats {
    #[serde(flatten)]
    pub data: MyDataData,
    pub total_purchasers: i64,
    pub total_subscribers: i64,
    pub total_revenue: i64,
    pub unique_accesses: i64,
    pub is_trending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    pub total_data_entries: i64,
    pub total_revenue: i64,
    pub total_purchases: i64,
    pub total_subscriptions: i64,
    pub unique_creators: i64,
    pub unique_buyers: i64,
    pub active_subscriptions: i64,
    pub top_categories: Vec<CategoryStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub media_type: String,
    pub count: i64,
    pub total_revenue: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorStats {
    pub creator: String,
    pub data_entries: i64,
    pub total_revenue: i64,
    pub unique_customers: i64,
    pub avg_price: Option<f64>,
    pub popular_categories: Vec<String>,
}

// ============================================================================
// QUERY RESULT TYPES FOR ANALYTICS
// ============================================================================

#[derive(QueryableByName, Debug, Clone, Serialize, Deserialize)]
pub struct DailyRevenueStats {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub creator: String,
    #[diesel(sql_type = Text)]
    pub revenue_type: String,
    #[diesel(sql_type = BigInt)]
    pub daily_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub transaction_count: i64,
}

#[derive(QueryableByName, Debug, Clone, Serialize, Deserialize)]
pub struct AccessAnalytics {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub access_type: String,
    #[diesel(sql_type = BigInt)]
    pub unique_users: i64,
    #[diesel(sql_type = BigInt)]
    pub total_accesses: i64,
}

#[derive(QueryableByName, Debug, Clone, Serialize, Deserialize)]
pub struct PopularDataStats {
    #[diesel(sql_type = Timestamp)]
    pub hour: chrono::NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = BigInt)]
    pub unique_purchasers: i64,
    #[diesel(sql_type = BigInt)]
    pub one_time_purchases: i64,
    #[diesel(sql_type = BigInt)]
    pub subscriptions: i64,
    #[diesel(sql_type = BigInt)]
    pub total_revenue: i64,
}

// ============================================================================
// HELPER FUNCTIONS FOR COMMON QUERIES
// ============================================================================

impl MyDataData {
    pub fn get_tags_array(&self) -> Vec<String> {
        if let Value::Array(tags) = &self.tags {
            tags.iter()
                .filter_map(|tag| tag.as_str())
                .map(|s| s.to_string())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        let mut tags = self.get_tags_array();
        if !tags.contains(&tag) {
            tags.push(tag);
            self.tags = Value::Array(tags.into_iter().map(|t| Value::String(t)).collect());
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        let tags: Vec<String> = self
            .get_tags_array()
            .into_iter()
            .filter(|t| t != tag)
            .collect();
        self.tags = Value::Array(tags.into_iter().map(|t| Value::String(t)).collect());
    }
}
