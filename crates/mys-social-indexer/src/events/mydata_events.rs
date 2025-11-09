// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use serde_json::Value;

// Import marketplace event types
use crate::events::mydata_event_types::{
    AnalyticsEvent, DataAccessGrantedEvent, DataAccessedEvent, DataCreatedEvent,
    DataPricingChangedEvent, DataPurchasedEvent, DataRemovedEvent, DataTransferredEvent,
    DataTrendingEvent, DataUpdatedEvent, OperationFailedEvent, RevenueDistributedEvent,
    SubscriptionCancelledEvent, SubscriptionCreatedEvent, SubscriptionRenewedEvent,
    SystemMaintenanceEvent,
};

// Import marketplace model types
use crate::models::mydata::{
    NewMyDataAccessLog, NewMyDataData, NewMyDataPurchase, NewMyDataRevenue, NewMyDataSubscription,
};

// ============================================================================
// MARKETPLACE EVENT MODEL CONVERTERS
// ============================================================================

// Model conversion impl for DataCreatedEvent
impl DataCreatedEvent {
    pub fn into_model(&self, transaction_id: String) -> Result<NewMyDataData> {
        Ok(NewMyDataData {
            mydata_id: self.mydata_id.clone(),
            owner: self.owner.clone(),
            media_type: self.media_type.clone(),
            tags: Value::Array(self.tags.iter().map(|t| Value::String(t.clone())).collect()),
            platform_id: self.platform_id.clone(),
            timestamp_start: self.timestamp_start as i64,
            timestamp_end: self.timestamp_end.map(|t| t as i64),
            created_at: self.created_at as i64,
            last_updated: self.created_at as i64,
            one_time_price: self.one_time_price.map(|p| p as i64),
            subscription_price: self.subscription_price.map(|p| p as i64),
            subscription_duration_days: self.subscription_duration_days as i64,
            geographic_region: self.geographic_region.clone(),
            data_quality: self.data_quality.clone(),
            sample_size: self.sample_size.map(|s| s as i64),
            collection_method: self.collection_method.clone(),
            is_updating: self.is_updating,
            update_frequency: self.update_frequency.clone(),
            version: 1, // Initial version
            transaction_id,
        })
    }
}

// Model conversion impl for DataPurchasedEvent
impl DataPurchasedEvent {
    pub fn into_purchase(&self, transaction_id: String) -> Result<NewMyDataPurchase> {
        Ok(NewMyDataPurchase {
            mydata_id: self.mydata_id.clone(),
            buyer: self.buyer.clone(),
            price: self.price as i64,
            purchase_type: "one_time".to_string(),
            purchase_time: self.purchase_time as i64,
            transaction_id,
        })
    }

    pub fn into_access_log(&self, transaction_id: String) -> Result<NewMyDataAccessLog> {
        Ok(NewMyDataAccessLog {
            mydata_id: self.mydata_id.clone(),
            user_address: self.buyer.clone(),
            access_type: "one_time".to_string(),
            access_time: self.purchase_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for SubscriptionCreatedEvent
impl SubscriptionCreatedEvent {
    pub fn into_subscription(&self, transaction_id: String) -> Result<NewMyDataSubscription> {
        Ok(NewMyDataSubscription {
            mydata_id: self.mydata_id.clone(),
            subscriber: self.subscriber.clone(),
            subscription_start: self.subscription_start as i64,
            subscription_end: self.subscription_end as i64,
            price: self.price as i64,
            transaction_id,
        })
    }

    pub fn into_purchase(&self, transaction_id: String) -> Result<NewMyDataPurchase> {
        Ok(NewMyDataPurchase {
            mydata_id: self.mydata_id.clone(),
            buyer: self.subscriber.clone(),
            price: self.price as i64,
            purchase_type: "subscription".to_string(),
            purchase_time: self.subscription_start as i64,
            transaction_id,
        })
    }

    pub fn into_access_log(&self, transaction_id: String) -> Result<NewMyDataAccessLog> {
        Ok(NewMyDataAccessLog {
            mydata_id: self.mydata_id.clone(),
            user_address: self.subscriber.clone(),
            access_type: "subscription".to_string(),
            access_time: self.subscription_start as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for DataAccessGrantedEvent
impl DataAccessGrantedEvent {
    pub fn into_access_log(&self, transaction_id: String) -> Result<NewMyDataAccessLog> {
        Ok(NewMyDataAccessLog {
            mydata_id: self.mydata_id.clone(),
            user_address: self.grantee.clone(),
            access_type: self.access_type.clone(),
            access_time: self.grant_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for DataAccessedEvent
impl DataAccessedEvent {
    pub fn into_access_log(&self, transaction_id: String) -> Result<NewMyDataAccessLog> {
        Ok(NewMyDataAccessLog {
            mydata_id: self.mydata_id.clone(),
            user_address: self.user_address.clone(),
            access_type: self.access_type.clone(),
            access_time: self.access_time as i64,
            transaction_id,
        })
    }
}

// Model conversion impl for RevenueDistributedEvent (updated for marketplace)
impl RevenueDistributedEvent {
    pub fn into_revenue(&self, transaction_id: String) -> Result<NewMyDataRevenue> {
        Ok(NewMyDataRevenue {
            mydata_id: self.mydata_id.clone(),
            from_address: self.from_address.clone(),
            to_address: self.to_address.clone(),
            amount: self.amount as i64,
            revenue_type: self.revenue_type.clone(),
            revenue_time: self.distribution_time as i64,
            transaction_id,
        })
    }
}

// ============================================================================
// MARKETPLACE ANALYTICS HELPERS
// ============================================================================

pub fn create_analytics_event(
    event_type: &str,
    mydata_id: &str,
    user_address: Option<&str>,
    metadata: Value,
    timestamp: u64,
) -> AnalyticsEvent {
    AnalyticsEvent {
        event_type: event_type.to_string(),
        mydata_id: mydata_id.to_string(),
        user_address: user_address.map(|s| s.to_string()),
        metadata,
        timestamp,
    }
}

pub fn create_trending_event(
    mydata_id: &str,
    media_type: &str,
    trending_score: f64,
    unique_purchasers_24h: u64,
    revenue_24h: u64,
    access_count_24h: u64,
) -> DataTrendingEvent {
    DataTrendingEvent {
        mydata_id: mydata_id.to_string(),
        media_type: media_type.to_string(),
        trending_score,
        unique_purchasers_24h,
        revenue_24h,
        access_count_24h,
        timestamp: Utc::now().timestamp() as u64,
    }
}

pub fn create_operation_failed_event(
    operation_type: &str,
    ip_id: Option<&str>,
    user_address: Option<&str>,
    error_code: &str,
    error_message: &str,
) -> OperationFailedEvent {
    OperationFailedEvent {
        operation_type: operation_type.to_string(),
        ip_id: ip_id.map(|s| s.to_string()),
        user_address: user_address.map(|s| s.to_string()),
        error_code: error_code.to_string(),
        error_message: error_message.to_string(),
        timestamp: Utc::now().timestamp() as u64,
    }
}

// ============================================================================
// SUBSCRIPTION RENEWAL HANDLER
// ============================================================================

impl SubscriptionRenewedEvent {
    pub fn into_subscription_update(
        &self,
        transaction_id: String,
    ) -> Result<NewMyDataSubscription> {
        Ok(NewMyDataSubscription {
            mydata_id: self.mydata_id.clone(),
            subscriber: self.subscriber.clone(),
            subscription_start: self.renewal_time as i64,
            subscription_end: self.new_subscription_end as i64,
            price: self.renewal_price as i64,
            transaction_id,
        })
    }

    pub fn into_revenue(&self, transaction_id: String) -> Result<NewMyDataRevenue> {
        Ok(NewMyDataRevenue {
            mydata_id: self.mydata_id.clone(),
            from_address: self.subscriber.clone(),
            to_address: self.subscriber.clone(), // Will be updated with actual owner
            amount: self.renewal_price as i64,
            revenue_type: "subscription_renewal".to_string(),
            revenue_time: self.renewal_time as i64,
            transaction_id,
        })
    }
}

// ============================================================================
// SUBSCRIPTION CANCELLATION HANDLER
// ============================================================================

impl SubscriptionCancelledEvent {
    pub fn into_revenue(&self, owner_address: String, transaction_id: String) -> Result<NewMyDataRevenue> {
        Ok(NewMyDataRevenue {
            mydata_id: self.mydata_id.clone(),
            from_address: owner_address.clone(),
            to_address: self.subscriber.clone(),
            amount: self.refund_amount.unwrap_or(0) as i64,
            revenue_type: "subscription_refund".to_string(),
            revenue_time: self.cancellation_time as i64,
            transaction_id,
        })
    }
}

// ============================================================================
// DATA TRANSFER HANDLER
// ============================================================================

impl DataTransferredEvent {
    pub fn into_revenue(&self, transaction_id: String) -> Option<NewMyDataRevenue> {
        self.transfer_price.map(|price| NewMyDataRevenue {
            mydata_id: self.mydata_id.clone(),
            from_address: self.to_owner.clone(),
            to_address: self.from_owner.clone(),
            amount: price as i64,
            revenue_type: "transfer".to_string(),
            revenue_time: self.transfer_time as i64,
            transaction_id,
        })
    }
}

// ============================================================================
// BATCH EVENT PROCESSING HELPERS
// ============================================================================

pub struct EventBatch {
    pub data_entries: Vec<NewMyDataData>,
    pub purchases: Vec<NewMyDataPurchase>,
    pub subscriptions: Vec<NewMyDataSubscription>,
    pub revenue_records: Vec<NewMyDataRevenue>,
    pub access_logs: Vec<NewMyDataAccessLog>,
}

impl EventBatch {
    pub fn new() -> Self {
        Self {
            data_entries: vec![],
            purchases: vec![],
            subscriptions: vec![],
            revenue_records: vec![],
            access_logs: vec![],
        }
    }

    pub fn add_data_created(
        &mut self,
        event: &DataCreatedEvent,
        transaction_id: String,
    ) -> Result<()> {
        self.data_entries.push(event.into_model(transaction_id)?);
        Ok(())
    }

    pub fn add_data_purchased(
        &mut self,
        event: &DataPurchasedEvent,
        transaction_id: String,
    ) -> Result<()> {
        self.purchases
            .push(event.into_purchase(transaction_id.clone())?);
        self.access_logs
            .push(event.into_access_log(transaction_id)?);
        Ok(())
    }

    pub fn add_subscription_created(
        &mut self,
        event: &SubscriptionCreatedEvent,
        transaction_id: String,
    ) -> Result<()> {
        self.subscriptions
            .push(event.into_subscription(transaction_id.clone())?);
        self.purchases
            .push(event.into_purchase(transaction_id.clone())?);
        self.access_logs
            .push(event.into_access_log(transaction_id)?);
        Ok(())
    }

    pub fn add_revenue_distributed(
        &mut self,
        event: &RevenueDistributedEvent,
        transaction_id: String,
    ) -> Result<()> {
        self.revenue_records
            .push(event.into_revenue(transaction_id)?);
        Ok(())
    }

    pub fn add_data_accessed(
        &mut self,
        event: &DataAccessedEvent,
        transaction_id: String,
    ) -> Result<()> {
        self.access_logs
            .push(event.into_access_log(transaction_id)?);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.data_entries.is_empty()
            && self.purchases.is_empty()
            && self.subscriptions.is_empty()
            && self.revenue_records.is_empty()
            && self.access_logs.is_empty()
    }
}
