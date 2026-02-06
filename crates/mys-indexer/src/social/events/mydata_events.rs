// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use serde_json::Value;

// Import marketplace event types
use crate::social::events::mydata_event_types::{
    AnalyticsEvent, DataAccessGrantedEvent, DataAccessedEvent, DataCreatedEvent,
    DataPurchasedEvent, DataTransferredEvent, DataTrendingEvent, MyDataRegisteredEvent,
    MyDataUnregisteredEvent, OperationFailedEvent, RevenueDistributedEvent,
    SubscriptionCancelledEvent, SubscriptionCreatedEvent, SubscriptionRenewedEvent,
};

// Import marketplace model types
use crate::social::models::mydata::{
    NewMyDataAccessLog, NewMyDataData, NewMyDataPurchase, NewMyDataRegistry, NewMyDataRevenue,
    NewMyDataSubscription,
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

// ============================================================================
// REGISTRY EVENT PROCESSING
// ============================================================================

/// Process a MyData registered event
pub async fn process_mydata_registered_event(
    conn: &mut crate::social::db::DbConnection,
    event: &serde_json::Value,
    event_id: &str,
) -> anyhow::Result<()> {
    use crate::social::events::event_utils::parse_json_event;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use tracing::{debug, info};

    debug!("Processing MyData registered event");

    // Parse the event
    let registered_event = parse_json_event::<MyDataRegisteredEvent>(event)?;

    // Create new registry entry
    let new_registry = NewMyDataRegistry {
        ip_id: registered_event.ip_id.clone(),
        owner: registered_event.owner.clone(),
        registered_at: registered_event.registered_at as i64,
        unregistered_at: None,
        is_active: true,
        transaction_id: event_id.to_string(),
    };

    // Insert or update registry entry
    // If entry exists, update it to active status (re-registration)
    let result = diesel::insert_into(crate::social::schema::mydata_registry::table)
        .values(&new_registry)
        .on_conflict(crate::social::schema::mydata_registry::ip_id)
        .do_update()
        .set((
            crate::social::schema::mydata_registry::owner.eq(new_registry.owner.clone()),
            crate::social::schema::mydata_registry::registered_at.eq(new_registry.registered_at),
            crate::social::schema::mydata_registry::unregistered_at.eq(None::<i64>),
            crate::social::schema::mydata_registry::is_active.eq(true),
            crate::social::schema::mydata_registry::transaction_id.eq(event_id.to_string()),
        ))
        .execute(conn)
        .await?;

    info!(
        "Processed MyData registered event: ip_id={}, owner={}, rows_affected={}",
        registered_event.ip_id, registered_event.owner, result
    );

    Ok(())
}

/// Process a MyData unregistered event
pub async fn process_mydata_unregistered_event(
    conn: &mut crate::social::db::DbConnection,
    event: &serde_json::Value,
    event_id: &str,
) -> anyhow::Result<()> {
    use crate::social::events::event_utils::parse_json_event;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use tracing::{debug, info, warn};

    debug!("Processing MyData unregistered event");

    // Parse the event
    let unregistered_event = parse_json_event::<MyDataUnregisteredEvent>(event)?;

    // Update registry entry to mark as inactive
    let result = diesel::update(crate::social::schema::mydata_registry::table)
        .filter(
            crate::social::schema::mydata_registry::ip_id
                .eq(&unregistered_event.ip_id)
                .and(crate::social::schema::mydata_registry::owner.eq(&unregistered_event.owner))
                .and(crate::social::schema::mydata_registry::is_active.eq(true)),
        )
        .set((
            crate::social::schema::mydata_registry::unregistered_at
                .eq(unregistered_event.unregistered_at as i64),
            crate::social::schema::mydata_registry::is_active.eq(false),
            crate::social::schema::mydata_registry::transaction_id.eq(event_id.to_string()),
        ))
        .execute(conn)
        .await?;

    if result == 0 {
        warn!(
            "MyData unregistered event: No active registry entry found for ip_id={}, owner={}",
            unregistered_event.ip_id, unregistered_event.owner
        );
    } else {
        info!(
            "Processed MyData unregistered event: ip_id={}, owner={}, rows_affected={}",
            unregistered_event.ip_id, unregistered_event.owner, result
        );
    }

    Ok(())
}

/// Process a MyData created event (from smart contract)
pub async fn process_mydata_created_event(
    conn: &mut crate::social::db::DbConnection,
    event: &serde_json::Value,
    event_id: &str,
) -> anyhow::Result<()> {
    use crate::social::events::event_utils::parse_json_event;
    use crate::social::events::mydata_event_types::MyDataCreatedEvent;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use tracing::{debug, info};

    debug!("Processing MyData created event");

    // Parse the event
    let created_event = parse_json_event::<MyDataCreatedEvent>(event)?;

    // Create new data entry
    let new_data = NewMyDataData {
        mydata_id: created_event.ip_id.clone(),
        owner: created_event.owner.clone(),
        media_type: created_event.media_type.clone(),
        tags: serde_json::Value::Array(vec![]),
        platform_id: created_event.platform_id.clone(),
        timestamp_start: created_event.created_at as i64,
        timestamp_end: None,
        created_at: created_event.created_at as i64,
        last_updated: created_event.created_at as i64,
        one_time_price: created_event.one_time_price.map(|p| p as i64),
        subscription_price: created_event.subscription_price.map(|p| p as i64),
        subscription_duration_days: 30, // Default
        geographic_region: None,
        data_quality: None,
        sample_size: None,
        collection_method: None,
        is_updating: false,
        update_frequency: None,
        version: 1,
        transaction_id: event_id.to_string(),
    };

    // Insert or update data entry
    diesel::insert_into(crate::social::schema::mydata_data::table)
        .values(&new_data)
        .on_conflict(crate::social::schema::mydata_data::mydata_id)
        .do_update()
        .set((
            crate::social::schema::mydata_data::owner.eq(&new_data.owner),
            crate::social::schema::mydata_data::media_type.eq(&new_data.media_type),
            crate::social::schema::mydata_data::platform_id.eq(&new_data.platform_id),
            crate::social::schema::mydata_data::one_time_price.eq(&new_data.one_time_price),
            crate::social::schema::mydata_data::subscription_price.eq(&new_data.subscription_price),
            crate::social::schema::mydata_data::last_updated.eq(&new_data.last_updated),
            crate::social::schema::mydata_data::transaction_id.eq(event_id.to_string()),
        ))
        .execute(conn)
        .await?;

    info!(
        "Processed MyData created event: ip_id={}, owner={}",
        created_event.ip_id, created_event.owner
    );

    Ok(())
}

/// Process a MyData purchase event
pub async fn process_mydata_purchase_event(
    conn: &mut crate::social::db::DbConnection,
    event: &serde_json::Value,
    event_id: &str,
) -> anyhow::Result<()> {
    use crate::social::events::event_utils::parse_json_event;
    use crate::social::events::mydata_event_types::PurchaseEvent;
    use diesel_async::RunQueryDsl;
    use tracing::{debug, info};

    debug!("Processing MyData purchase event");

    // Parse the event
    let purchase_event = parse_json_event::<PurchaseEvent>(event)?;

    // Create purchase record
    let new_purchase = NewMyDataPurchase {
        mydata_id: purchase_event.ip_id.clone(),
        buyer: purchase_event.buyer.clone(),
        price: purchase_event.price as i64,
        purchase_type: purchase_event.purchase_type.clone(),
        purchase_time: purchase_event.timestamp as i64,
        transaction_id: event_id.to_string(),
    };

    diesel::insert_into(crate::social::schema::mydata_purchases::table)
        .values(&new_purchase)
        .execute(conn)
        .await?;

    // Create access log
    let access_log = NewMyDataAccessLog {
        mydata_id: purchase_event.ip_id.clone(),
        user_address: purchase_event.buyer.clone(),
        access_type: purchase_event.purchase_type.clone(),
        access_time: purchase_event.timestamp as i64,
        transaction_id: event_id.to_string(),
    };

    diesel::insert_into(crate::social::schema::mydata_access_logs::table)
        .values(&access_log)
        .execute(conn)
        .await?;

    info!(
        "Processed MyData purchase event: ip_id={}, buyer={}, type={}",
        purchase_event.ip_id, purchase_event.buyer, purchase_event.purchase_type
    );

    Ok(())
}

/// Process a MyData access granted event
pub async fn process_mydata_access_granted_event(
    conn: &mut crate::social::db::DbConnection,
    event: &serde_json::Value,
    event_id: &str,
) -> anyhow::Result<()> {
    use crate::social::events::event_utils::parse_json_event;
    use crate::social::events::mydata_event_types::AccessGrantedEvent;
    use diesel_async::RunQueryDsl;
    use tracing::{debug, info};

    debug!("Processing MyData access granted event");

    // Parse the event
    let access_event = parse_json_event::<AccessGrantedEvent>(event)?;

    // Create access log
    let access_log = NewMyDataAccessLog {
        mydata_id: access_event.ip_id.clone(),
        user_address: access_event.user.clone(),
        access_type: access_event.access_type.clone(),
        access_time: access_event.timestamp as i64,
        transaction_id: event_id.to_string(),
    };

    diesel::insert_into(crate::social::schema::mydata_access_logs::table)
        .values(&access_log)
        .execute(conn)
        .await?;

    info!(
        "Processed MyData access granted event: ip_id={}, user={}, type={}",
        access_event.ip_id, access_event.user, access_event.access_type
    );

    Ok(())
}

/// Process a MyData config updated event
pub async fn process_mydata_config_updated_event(
    conn: &mut crate::social::db::DbConnection,
    event: &serde_json::Value,
    event_id: &str,
    timestamp_ms: u64,
) -> anyhow::Result<()> {
    use crate::social::events::event_utils::parse_json_event;
    use crate::social::events::mydata_event_types::MyDataConfigUpdatedEvent;
    use crate::social::models::mydata::MyDataConfig;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use tracing::{debug, info};

    debug!("Processing MyData config updated event");

    // Parse the event
    let config_event = parse_json_event::<MyDataConfigUpdatedEvent>(event)?;

    // Get latest config for fallback values
    let latest_config: Option<MyDataConfig> = crate::social::schema::mydata_config::table
        .order(crate::social::schema::mydata_config::id.desc())
        .first(conn)
        .await
        .ok();

    let new_config = config_event.into_config_model(
        timestamp_ms,
        event_id.to_string(),
        latest_config.as_ref(),
    );

    diesel::insert_into(crate::social::schema::mydata_config::table)
        .values(&new_config)
        .execute(conn)
        .await?;

    info!(
        "Processed MyData config updated event: updated_by={}, enable_flag={}",
        config_event.updated_by, config_event.enable_flag
    );

    Ok(())
}
