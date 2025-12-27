// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::db::{Database, DbConnection};
use crate::events::{
    event_utils::extract_event_fields,
    mydata_event_types::{
        AccessGrantedEvent, DataAccessGrantedEvent, DataAccessedEvent, DataCreatedEvent,
        DataPricingChangedEvent, DataPurchasedEvent, DataRemovedEvent, DataTransferredEvent,
        DataTrendingEvent, DataUpdatedEvent, MyDataConfigUpdatedEvent, MyDataCreatedEvent,
        OperationFailedEvent, PurchaseEvent, RevenueDistributedEvent, SubscriptionCancelledEvent,
        SubscriptionCreatedEvent, SubscriptionRenewedEvent, SystemMaintenanceEvent,
    },
    mydata_events::{process_mydata_registered_event, process_mydata_unregistered_event, EventBatch},
    parse_event,
};

use crate::models::mydata::NewMyDataAccessLog;
use crate::schema::{
    mydata_access_logs, mydata_config, mydata_data, mydata_purchases, mydata_revenue,
    mydata_subscriptions,
};
use mys_types::event::Event as MysEvent;

use super::listener::BlockchainEvent;

/// Handler for MyData Marketplace events from the blockchain
pub struct MyDataEventHandler {
    /// Database connection
    db: Arc<Database>,
    /// Event receiver channel
    rx: mpsc::Receiver<BlockchainEvent>,
    /// Worker ID for tracking progress
    worker_id: String,
}

impl MyDataEventHandler {
    /// Create a new MyDataEventHandler with the given database connection
    pub fn new(db: Arc<Database>, rx: mpsc::Receiver<BlockchainEvent>, worker_id: String) -> Self {
        Self { db, rx, worker_id }
    }

    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    /// Start the event processing loop
    pub async fn start(&mut self) -> Result<()> {
        info!(
            "Starting MyData marketplace event handler: {}",
            self.worker_id
        );

        while let Some(blockchain_event) = self.rx.recv().await {
            // Filter for MyData marketplace events
            let event_type = &blockchain_event.event_type;

            if self.is_mydata_event(event_type) {
                info!("Processing MyData marketplace event: {}", event_type);

                if let Err(e) = self.handle_blockchain_event(&blockchain_event).await {
                    error!(
                        "Failed to process MyData marketplace event {}: {}",
                        blockchain_event.event_id, e
                    );
                }
            }
        }

        info!("MyData marketplace event handler stopped");
        Ok(())
    }

    /// Check if an event type is a MyData marketplace event
    fn is_mydata_event(&self, event_type: &str) -> bool {
        event_type.contains("::mydata::")
            || event_type.contains("::marketplace::")
            || event_type.ends_with("::DataCreatedEvent")
            || event_type.ends_with("::DataUpdatedEvent")
            || event_type.ends_with("::DataTransferredEvent")
            || event_type.ends_with("::DataPurchasedEvent")
            || event_type.ends_with("::SubscriptionCreatedEvent")
            || event_type.ends_with("::SubscriptionRenewedEvent")
            || event_type.ends_with("::SubscriptionCancelledEvent")
            || event_type.ends_with("::DataAccessGrantedEvent")
            || event_type.ends_with("::RevenueDistributedEvent")
            || event_type.ends_with("::DataAccessedEvent")
            || event_type.ends_with("::DataPricingChangedEvent")
            || event_type.ends_with("::DataRemovedEvent")
            || event_type.ends_with("::DataTrendingEvent")
            || event_type.ends_with("::OperationFailedEvent")
            || event_type.ends_with("::SystemMaintenanceEvent")
            || event_type.ends_with("::MyDataConfigUpdatedEvent")
            || event_type.ends_with("MyDataConfigUpdatedEvent")
    }

    /// Handle a blockchain event for MyData marketplace events
    async fn handle_blockchain_event(&self, blockchain_event: &BlockchainEvent) -> Result<()> {
        let event_type = &blockchain_event.event_type;

        info!(
            "Processing MyData marketplace blockchain event: {}",
            event_type
        );

        // Process each marketplace event type based on the parsed JSON data
        match () {
            // Actual contract events
            _ if event_type.ends_with("::MyDataCreatedEvent") || event_type.ends_with("MyDataCreatedEvent") => {
                self.handle_mydata_created_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::PurchaseEvent") || event_type.ends_with("PurchaseEvent") => {
                self.handle_purchase_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::AccessGrantedEvent") || event_type.ends_with("AccessGrantedEvent") => {
                self.handle_access_granted_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::MyDataRegisteredEvent") || event_type.ends_with("MyDataRegisteredEvent") => {
                self.handle_mydata_registered_from_json(
                    &blockchain_event.data,
                    &blockchain_event.event_id,
                )
                .await?;
            }
            _ if event_type.ends_with("::MyDataUnregisteredEvent") || event_type.ends_with("MyDataUnregisteredEvent") => {
                self.handle_mydata_unregistered_from_json(
                    &blockchain_event.data,
                    &blockchain_event.event_id,
                )
                .await?;
            }
            _ if event_type.ends_with("::MyDataConfigUpdatedEvent") || event_type.ends_with("MyDataConfigUpdatedEvent") => {
                self.handle_mydata_config_updated_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            // Legacy event names (for backward compatibility)
            _ if event_type.ends_with("::DataCreatedEvent") => {
                self.handle_data_created_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataPurchasedEvent") => {
                self.handle_data_purchased_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::SubscriptionCreatedEvent") => {
                self.handle_subscription_created_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataAccessGrantedEvent") => {
                self.handle_data_access_granted_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::RevenueDistributedEvent") => {
                self.handle_revenue_distributed_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataAccessedEvent") => {
                self.handle_data_accessed_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataUpdatedEvent") => {
                self.handle_data_updated_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataTransferredEvent") => {
                self.handle_data_transferred_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::SubscriptionRenewedEvent") => {
                self.handle_subscription_renewed_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::SubscriptionCancelledEvent") => {
                self.handle_subscription_cancelled_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataPricingChangedEvent") => {
                self.handle_data_pricing_changed_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataRemovedEvent") => {
                self.handle_data_removed_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::DataTrendingEvent") => {
                self.handle_data_trending_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::OperationFailedEvent") => {
                self.handle_operation_failed_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ if event_type.ends_with("::SystemMaintenanceEvent") => {
                self.handle_system_maintenance_from_json(
                    &blockchain_event.data,
                    &blockchain_event.tx_digest,
                )
                .await?;
            }
            _ => {
                warn!("Received unhandled MyData event: {} (event_id: {})", event_type, blockchain_event.event_id);
            }
        }

        Ok(())
    }

    /// Handle data created event from JSON
    async fn handle_data_created_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataCreatedEvent from JSON");

        // Extract fields from the JSON data
        let mydata_id = data
            .get("mydata_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing mydata_id field"))?;

        let owner = data
            .get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing owner field"))?;

        let media_type = data
            .get("media_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing media_type field"))?;

        info!(
            "Parsed DataCreatedEvent: mydata_id={}, owner={}, media_type={}",
            mydata_id, owner, media_type
        );

        // Get a database connection
        let mut conn = self.get_connection().await?;

        // Create a new data entry manually from the JSON data
        let new_data = crate::models::mydata::NewMyDataData {
            mydata_id: mydata_id.to_string(),
            owner: owner.to_string(),
            media_type: media_type.to_string(),
            tags: data.get("tags").cloned().unwrap_or(serde_json::json!([])),
            platform_id: data
                .get("platform_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            timestamp_start: data
                .get("timestamp_start")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            timestamp_end: data.get("timestamp_end").and_then(|v| v.as_i64()),
            created_at: Utc::now().timestamp(),
            last_updated: Utc::now().timestamp(),
            one_time_price: data.get("one_time_price").and_then(|v| v.as_i64()),
            subscription_price: data.get("subscription_price").and_then(|v| v.as_i64()),
            subscription_duration_days: data
                .get("subscription_duration_days")
                .and_then(|v| v.as_i64())
                .unwrap_or(30),
            geographic_region: data
                .get("geographic_region")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            data_quality: data
                .get("data_quality")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            sample_size: data.get("sample_size").and_then(|v| v.as_i64()),
            collection_method: data
                .get("collection_method")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            is_updating: data
                .get("is_updating")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            update_frequency: data
                .get("update_frequency")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            version: data.get("version").and_then(|v| v.as_i64()).unwrap_or(1),
            transaction_id: transaction_id.to_string(),
        };

        // Insert the new data entry into the database
        diesel::insert_into(mydata_data::table)
            .values(&new_data)
            .on_conflict(mydata_data::mydata_id)
            .do_update()
            .set((
                mydata_data::owner.eq(&new_data.owner),
                mydata_data::media_type.eq(&new_data.media_type),
                mydata_data::tags.eq(&new_data.tags),
                mydata_data::one_time_price.eq(&new_data.one_time_price),
                mydata_data::subscription_price.eq(&new_data.subscription_price),
                mydata_data::data_quality.eq(&new_data.data_quality),
                mydata_data::last_updated.eq(Utc::now().timestamp()),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataCreatedEvent successfully for mydata_id: {}",
            mydata_id
        );
        Ok(())
    }

    /// Handle other marketplace events from JSON (simplified implementations)
    async fn handle_data_purchased_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataPurchasedEvent from JSON");

        // Extract fields from JSON data according to event structure
        let mydata_id = data
            .get("mydata_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing mydata_id field"))?;

        let buyer = data
            .get("buyer")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing buyer field"))?;

        let price = data
            .get("price")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing price field"))?;

        let purchase_time = data
            .get("purchase_time")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing purchase_time field"))?;

        let mut conn = self.get_connection().await?;

        // Record the purchase
        let purchase = crate::models::NewMyDataPurchase {
            mydata_id: mydata_id.to_string(),
            buyer: buyer.to_string(),
            price: price as i64,
            purchase_type: crate::models::mydata::PURCHASE_TYPE_ONE_TIME.to_string(),
            purchase_time: purchase_time as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;

        // Record access log
        let access_log = crate::models::NewMyDataAccessLog {
            mydata_id: mydata_id.to_string(),
            user_address: buyer.to_string(),
            access_type: crate::models::mydata::ACCESS_TYPE_ONE_TIME.to_string(),
            access_time: purchase_time as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataPurchasedEvent from JSON for mydata_id: {}",
            mydata_id
        );
        Ok(())
    }

    async fn handle_subscription_created_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing SubscriptionCreatedEvent from JSON");

        // Extract fields from JSON data according to event structure
        let mydata_id = data
            .get("mydata_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing mydata_id field"))?;

        let subscriber = data
            .get("subscriber")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing subscriber field"))?;

        let subscription_start = data
            .get("subscription_start")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing subscription_start field"))?;

        let subscription_end = data
            .get("subscription_end")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing subscription_end field"))?;

        let price = data
            .get("price")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing price field"))?;

        let mut conn = self.get_connection().await?;

        // Record the subscription
        let subscription = crate::models::NewMyDataSubscription {
            mydata_id: mydata_id.to_string(),
            subscriber: subscriber.to_string(),
            subscription_start: subscription_start as i64,
            subscription_end: subscription_end as i64,
            price: price as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_subscriptions::table)
            .values(&subscription)
            .execute(&mut conn)
            .await?;

        // Record as a purchase too for analytics
        let purchase = crate::models::NewMyDataPurchase {
            mydata_id: mydata_id.to_string(),
            buyer: subscriber.to_string(),
            price: price as i64,
            purchase_type: crate::models::mydata::PURCHASE_TYPE_SUBSCRIPTION.to_string(),
            purchase_time: subscription_start as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;

        // Record access log
        let access_log = crate::models::NewMyDataAccessLog {
            mydata_id: mydata_id.to_string(),
            user_address: subscriber.to_string(),
            access_type: crate::models::mydata::ACCESS_TYPE_SUBSCRIPTION.to_string(),
            access_time: subscription_start as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed SubscriptionCreatedEvent from JSON for mydata_id: {}",
            mydata_id
        );
        Ok(())
    }

    async fn handle_data_access_granted_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataAccessGrantedEvent from JSON");

        // Extract fields from JSON data according to event structure
        let mydata_id = data
            .get("mydata_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing mydata_id field"))?;

        let grantee = data
            .get("grantee")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing grantee field"))?;

        let grant_time = data
            .get("grant_time")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing grant_time field"))?;

        let mut conn = self.get_connection().await?;

        // Record access log
        let access_log = crate::models::NewMyDataAccessLog {
            mydata_id: mydata_id.to_string(),
            user_address: grantee.to_string(),
            access_type: crate::models::mydata::ACCESS_TYPE_GRANT.to_string(),
            access_time: grant_time as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataAccessGrantedEvent from JSON for mydata_id: {}",
            mydata_id
        );
        Ok(())
    }

    async fn handle_revenue_distributed_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing RevenueDistributedEvent from JSON");

        // Extract fields from JSON data according to event structure
        let mydata_id = data
            .get("mydata_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing mydata_id field"))?;

        let from_address = data
            .get("from_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing from_address field"))?;

        let to_address = data
            .get("to_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing to_address field"))?;

        let amount = data
            .get("amount")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing amount field"))?;

        let revenue_type = data
            .get("revenue_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing revenue_type field"))?;

        let distribution_time = data
            .get("distribution_time")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing distribution_time field"))?;

        let mut conn = self.get_connection().await?;

        // Record revenue distribution
        let revenue = crate::models::NewMyDataRevenue {
            mydata_id: mydata_id.to_string(),
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
            amount: amount as i64,
            revenue_type: revenue_type.to_string(),
            revenue_time: distribution_time as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;

        // Also create unified revenue record
        let unified_revenue = crate::models::NewUnifiedRevenue::from_myip(
            revenue_type.to_string(),
            to_address.to_string(), // creator is recipient
            amount as i64,
            mydata_id.to_string(),
            from_address.to_string(), // payer
            to_address.to_string(),   // recipient
            distribution_time as i64,
            transaction_id.to_string(),
        );

        diesel::insert_into(crate::schema::unified_revenue::table)
            .values(&unified_revenue)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed RevenueDistributedEvent from JSON for mydata_id: {}",
            mydata_id
        );
        Ok(())
    }

    async fn handle_data_accessed_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataAccessedEvent from JSON");

        // Extract fields from JSON data according to event structure
        let mydata_id = data
            .get("mydata_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing mydata_id field"))?;

        let user_address = data
            .get("user_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing user_address field"))?;

        let access_type = data
            .get("access_type")
            .and_then(|v| v.as_str())
            .unwrap_or("preview");

        let access_time = data
            .get("access_time")
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);

        let mut conn = self.get_connection().await?;

        // Record access log
        let access_log = crate::models::NewMyDataAccessLog {
            mydata_id: mydata_id.to_string(),
            user_address: user_address.to_string(),
            access_type: access_type.to_string(),
            access_time: access_time as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataAccessedEvent from JSON for mydata_id: {}",
            mydata_id
        );
        Ok(())
    }

    /// Handle a MyData marketplace event from the blockchain
    pub async fn handle_event(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        let event_type = &event.type_.to_string(); // Convert StructTag to String

        info!("Processing MyData marketplace event: {}", event_type);

        // Process each marketplace event type
        match () {
            _ if event_type.ends_with("::DataCreatedEvent") => {
                self.handle_data_created(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::DataPurchasedEvent") => {
                self.handle_data_purchased(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::SubscriptionCreatedEvent") => {
                self.handle_subscription_created(event, transaction_id)
                    .await?;
            }
            _ if event_type.ends_with("::DataAccessGrantedEvent") => {
                self.handle_data_access_granted(event, transaction_id)
                    .await?;
            }
            _ if event_type.ends_with("::RevenueDistributedEvent") => {
                self.handle_revenue_distributed(event, transaction_id)
                    .await?;
            }
            _ if event_type.ends_with("::DataAccessedEvent") => {
                self.handle_data_accessed(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::DataUpdatedEvent") => {
                self.handle_data_updated(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::DataTransferredEvent") => {
                self.handle_data_transferred(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::SubscriptionRenewedEvent") => {
                self.handle_subscription_renewed(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::SubscriptionCancelledEvent") => {
                self.handle_subscription_cancelled(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::DataPricingChangedEvent") => {
                self.handle_data_pricing_changed(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::DataRemovedEvent") => {
                self.handle_data_removed(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::DataTrendingEvent") => {
                self.handle_data_trending(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::OperationFailedEvent") => {
                self.handle_operation_failed(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::SystemMaintenanceEvent") => {
                self.handle_system_maintenance(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::MyDataRegisteredEvent") => {
                self.handle_mydata_registered(event, transaction_id).await?;
            }
            _ if event_type.ends_with("::MyDataUnregisteredEvent") => {
                self.handle_mydata_unregistered(event, transaction_id).await?;
            }
            _ => {
                debug!("Unhandled MyData marketplace event type: {}", event_type);
            }
        }

        Ok(())
    }

    /// Handle data created event
    async fn handle_data_created(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataCreatedEvent");

        // Parse the event
        let parsed_event = parse_event::<DataCreatedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataCreatedEvent: {}", e))?;

        info!(
            "Parsed DataCreatedEvent: mydata_id={}, owner={}, media_type={}",
            parsed_event.mydata_id, parsed_event.owner, parsed_event.media_type
        );

        // Get a database connection
        let mut conn = self.db.get_connection().await?;

        // Convert event to model
        let new_data = parsed_event.into_model(transaction_id.to_string())?;

        // Insert the new data entry into the database
        diesel::insert_into(mydata_data::table)
            .values(&new_data)
            .on_conflict(mydata_data::mydata_id)
            .do_update()
            .set((
                mydata_data::owner.eq(&new_data.owner),
                mydata_data::media_type.eq(&new_data.media_type),
                mydata_data::tags.eq(&new_data.tags),
                mydata_data::one_time_price.eq(&new_data.one_time_price),
                mydata_data::subscription_price.eq(&new_data.subscription_price),
                mydata_data::data_quality.eq(&new_data.data_quality),
                mydata_data::last_updated.eq(Utc::now().timestamp()),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataCreatedEvent successfully for mydata_id: {}",
            parsed_event.mydata_id
        );
        Ok(())
    }

    /// Handle data purchased event
    async fn handle_data_purchased(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataPurchasedEvent");

        let parsed_event = parse_event::<DataPurchasedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataPurchasedEvent: {}", e))?;

        info!(
            "Parsed DataPurchasedEvent: mydata_id={}, buyer={}, price={}",
            parsed_event.mydata_id, parsed_event.buyer, parsed_event.price
        );

        let mut conn = self.db.get_connection().await?;

        // Record the purchase
        let purchase = parsed_event.into_purchase(transaction_id.to_string())?;
        diesel::insert_into(mydata_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;

        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataPurchasedEvent successfully for mydata_id: {}",
            parsed_event.mydata_id
        );
        Ok(())
    }

    /// Handle subscription created event
    async fn handle_subscription_created(
        &self,
        event: &MysEvent,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing SubscriptionCreatedEvent");

        let parsed_event = parse_event::<SubscriptionCreatedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse SubscriptionCreatedEvent: {}", e))?;

        info!(
            "Parsed SubscriptionCreatedEvent: mydata_id={}, subscriber={}, price={}",
            parsed_event.mydata_id, parsed_event.subscriber, parsed_event.price
        );

        let mut conn = self.db.get_connection().await?;

        // Record the subscription
        let subscription = parsed_event.into_subscription(transaction_id.to_string())?;
        diesel::insert_into(mydata_subscriptions::table)
            .values(&subscription)
            .execute(&mut conn)
            .await?;

        // Record as a purchase too for analytics
        let purchase = parsed_event.into_purchase(transaction_id.to_string())?;
        diesel::insert_into(mydata_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;

        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed SubscriptionCreatedEvent successfully for mydata_id: {}",
            parsed_event.mydata_id
        );
        Ok(())
    }

    /// Handle data access granted event
    async fn handle_data_access_granted(
        &self,
        event: &MysEvent,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataAccessGrantedEvent");

        let parsed_event = parse_event::<DataAccessGrantedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataAccessGrantedEvent: {}", e))?;

        info!(
            "Parsed DataAccessGrantedEvent: mydata_id={}, grantor={}, grantee={}",
            parsed_event.mydata_id, parsed_event.grantor, parsed_event.grantee
        );

        let mut conn = self.db.get_connection().await?;

        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed DataAccessGrantedEvent successfully for mydata_id: {}",
            parsed_event.mydata_id
        );
        Ok(())
    }

    /// Handle data accessed event
    async fn handle_data_accessed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataAccessedEvent");

        let parsed_event = parse_event::<DataAccessedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataAccessedEvent: {}", e))?;

        info!(
            "Parsed DataAccessedEvent: mydata_id={}, user={}",
            parsed_event.mydata_id, parsed_event.user_address
        );

        let mut conn = self.db.get_connection().await?;

        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Handle revenue distributed event (updated for marketplace)
    async fn handle_revenue_distributed(
        &self,
        event: &MysEvent,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing RevenueDistributedEvent");

        let parsed_event = parse_event::<RevenueDistributedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse RevenueDistributedEvent: {}", e))?;

        info!(
            "Parsed RevenueDistributedEvent: mydata_id={}, from={}, to={}, amount={}",
            parsed_event.mydata_id,
            parsed_event.from_address,
            parsed_event.to_address,
            parsed_event.amount
        );

        let mut conn = self.db.get_connection().await?;

        // Record revenue distribution
        let revenue = parsed_event.into_revenue(transaction_id.to_string())?;
        diesel::insert_into(mydata_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed RevenueDistributedEvent successfully for mydata_id: {}",
            parsed_event.mydata_id
        );
        Ok(())
    }

    /// Update marketplace data statistics
    pub async fn update_data_statistics(&self, mydata_id: &str) -> Result<()> {
        info!("Updating statistics for data: {}", mydata_id);

        let mut conn = self.db.get_connection().await?;

        // Get total revenue for the data
        let total_revenue: Option<BigDecimal> = mydata_revenue::table
            .filter(mydata_revenue::mydata_id.eq(mydata_id))
            .select(diesel::dsl::sum(mydata_revenue::amount))
            .first::<Option<BigDecimal>>(&mut conn)
            .await?;

        let total_revenue: i64 = total_revenue
            .map(|bd| bd.to_string().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);

        // Get purchase and subscription stats
        let purchase_count: i64 = mydata_purchases::table
            .filter(mydata_purchases::mydata_id.eq(mydata_id))
            .count()
            .get_result(&mut conn)
            .await?;

        let subscription_count: i64 = mydata_subscriptions::table
            .filter(mydata_subscriptions::mydata_id.eq(mydata_id))
            .count()
            .get_result(&mut conn)
            .await?;

        let access_count: i64 = mydata_access_logs::table
            .filter(mydata_access_logs::mydata_id.eq(mydata_id))
            .count()
            .get_result(&mut conn)
            .await?;

        info!(
            "Data {} stats: purchases={}, subscriptions={}, accesses={}, total_revenue={}",
            mydata_id, purchase_count, subscription_count, access_count, total_revenue
        );

        Ok(())
    }

    /// Process events in batch for better performance
    pub async fn handle_events_batch(&self, events: Vec<(&MysEvent, &str)>) -> Result<()> {
        let events_len = events.len();
        info!("Processing batch of {} events", events_len);

        let mut event_batch = EventBatch::new();

        for (event, transaction_id) in &events {
            let event_type = &event.type_.to_string();

            match () {
                _ if event_type.ends_with("::DataCreatedEvent") => {
                    if let Ok(parsed) = parse_event::<DataCreatedEvent>(event) {
                        event_batch.add_data_created(&parsed, transaction_id.to_string())?;
                    }
                }
                _ if event_type.ends_with("::DataPurchasedEvent") => {
                    if let Ok(parsed) = parse_event::<DataPurchasedEvent>(event) {
                        event_batch.add_data_purchased(&parsed, transaction_id.to_string())?;
                    }
                }
                _ if event_type.ends_with("::SubscriptionCreatedEvent") => {
                    if let Ok(parsed) = parse_event::<SubscriptionCreatedEvent>(event) {
                        event_batch
                            .add_subscription_created(&parsed, transaction_id.to_string())?;
                    }
                }
                _ if event_type.ends_with("::RevenueDistributedEvent") => {
                    if let Ok(parsed) = parse_event::<RevenueDistributedEvent>(event) {
                        event_batch.add_revenue_distributed(&parsed, transaction_id.to_string())?;
                    }
                }
                _ if event_type.ends_with("::DataAccessedEvent") => {
                    if let Ok(parsed) = parse_event::<DataAccessedEvent>(event) {
                        event_batch.add_data_accessed(&parsed, transaction_id.to_string())?;
                    }
                }
                _ => {
                    // Handle non-batchable events individually
                    self.handle_event(event, transaction_id).await?;
                }
            }
        }

        // Execute batch operations
        if !event_batch.is_empty() {
            let mut conn = self.db.get_connection().await?;

            if !event_batch.data_entries.is_empty() {
                diesel::insert_into(mydata_data::table)
                    .values(&event_batch.data_entries)
                    .on_conflict(mydata_data::mydata_id)
                    .do_nothing()
                    .execute(&mut conn)
                    .await?;
            }

            if !event_batch.purchases.is_empty() {
                diesel::insert_into(mydata_purchases::table)
                    .values(&event_batch.purchases)
                    .execute(&mut conn)
                    .await?;
            }

            if !event_batch.subscriptions.is_empty() {
                diesel::insert_into(mydata_subscriptions::table)
                    .values(&event_batch.subscriptions)
                    .execute(&mut conn)
                    .await?;
            }

            if !event_batch.revenue_records.is_empty() {
                diesel::insert_into(mydata_revenue::table)
                    .values(&event_batch.revenue_records)
                    .execute(&mut conn)
                    .await?;
            }

            if !event_batch.access_logs.is_empty() {
                diesel::insert_into(mydata_access_logs::table)
                    .values(&event_batch.access_logs)
                    .execute(&mut conn)
                    .await?;
            }
        }

        info!("Processed batch of {} events successfully", events_len);
        Ok(())
    }

    /// Handle data updated event from JSON
    async fn handle_data_updated_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataUpdatedEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: DataUpdatedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse DataUpdatedEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        // Update mydata_data table with new values
        let new_tags = serde_json::json!(parsed_event.new_tags);
        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::tags.eq(new_tags),
                mydata_data::one_time_price.eq(parsed_event.new_price_one_time.map(|p| p as i64)),
                mydata_data::subscription_price.eq(parsed_event.new_price_subscription.map(|p| p as i64)),
                mydata_data::data_quality.eq(parsed_event.new_data_quality.clone()),
                mydata_data::last_updated.eq(parsed_event.last_updated as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!("Processed DataUpdatedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data transferred event from JSON
    async fn handle_data_transferred_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataTransferredEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: DataTransferredEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse DataTransferredEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        // Update owner
        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::owner.eq(&parsed_event.to_owner),
                mydata_data::last_updated.eq(parsed_event.transfer_time as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        // Record transfer revenue if transfer_price exists
        if let Some(revenue) = parsed_event.into_revenue(transaction_id.to_string()) {
            diesel::insert_into(mydata_revenue::table)
                .values(&revenue)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed DataTransferredEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle subscription renewed event from JSON
    async fn handle_subscription_renewed_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing SubscriptionRenewedEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: SubscriptionRenewedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse SubscriptionRenewedEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        // Update existing subscription end time
        diesel::update(mydata_subscriptions::table)
            .filter(mydata_subscriptions::mydata_id.eq(&parsed_event.mydata_id))
            .filter(mydata_subscriptions::subscriber.eq(&parsed_event.subscriber))
            .filter(mydata_subscriptions::subscription_end.eq(parsed_event.old_subscription_end as i64))
            .set(mydata_subscriptions::subscription_end.eq(parsed_event.new_subscription_end as i64))
            .execute(&mut conn)
            .await?;

        // Create new subscription record for renewal period
        let renewal_subscription = parsed_event.into_subscription_update(transaction_id.to_string())?;
        diesel::insert_into(mydata_subscriptions::table)
            .values(&renewal_subscription)
            .execute(&mut conn)
            .await?;

        // Get owner for revenue record
        let owner: String = mydata_data::table
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .select(mydata_data::owner)
            .first(&mut conn)
            .await?;

        // Record renewal revenue
        let mut revenue = parsed_event.into_revenue(transaction_id.to_string())?;
        revenue.to_address = owner;
        diesel::insert_into(mydata_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;

        info!("Processed SubscriptionRenewedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle subscription cancelled event from JSON
    async fn handle_subscription_cancelled_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing SubscriptionCancelledEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: SubscriptionCancelledEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse SubscriptionCancelledEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        // Update subscription end time to effective_end_time
        diesel::update(mydata_subscriptions::table)
            .filter(mydata_subscriptions::mydata_id.eq(&parsed_event.mydata_id))
            .filter(mydata_subscriptions::subscriber.eq(&parsed_event.subscriber))
            .filter(mydata_subscriptions::subscription_end.gt(parsed_event.cancellation_time as i64))
            .set(mydata_subscriptions::subscription_end.eq(parsed_event.effective_end_time as i64))
            .execute(&mut conn)
            .await?;

        // Record refund revenue if applicable
        if parsed_event.refund_amount.is_some() && parsed_event.refund_amount.unwrap_or(0) > 0 {
            let owner: String = mydata_data::table
                .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
                .select(mydata_data::owner)
                .first(&mut conn)
                .await?;

            let refund_revenue = parsed_event.into_revenue(owner, transaction_id.to_string())?;
            diesel::insert_into(mydata_revenue::table)
                .values(&refund_revenue)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed SubscriptionCancelledEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data pricing changed event from JSON
    async fn handle_data_pricing_changed_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataPricingChangedEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: DataPricingChangedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse DataPricingChangedEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        // Update pricing fields
        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::one_time_price.eq(parsed_event.new_one_time_price.map(|p| p as i64)),
                mydata_data::subscription_price.eq(parsed_event.new_subscription_price.map(|p| p as i64)),
                mydata_data::subscription_duration_days.eq(parsed_event.new_subscription_duration as i64),
                mydata_data::last_updated.eq(parsed_event.change_time as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!("Processed DataPricingChangedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data removed event from JSON
    async fn handle_data_removed_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataRemovedEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: DataRemovedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse DataRemovedEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        // Mark data as removed by setting timestamp_end to removal_time
        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::timestamp_end.eq(Some(parsed_event.removal_time as i64)),
                mydata_data::last_updated.eq(parsed_event.removal_time as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!("Processed DataRemovedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data trending event from JSON
    async fn handle_data_trending_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing DataTrendingEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: DataTrendingEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse DataTrendingEvent: {}", e))?;

        // Log trending event to access_logs with special access_type for analytics
        let mut conn = self.get_connection().await?;
        let access_log = NewMyDataAccessLog {
            mydata_id: parsed_event.mydata_id.clone(),
            user_address: "system".to_string(),
            access_type: format!("trending_score_{}", parsed_event.trending_score),
            access_time: parsed_event.timestamp as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!("Processed DataTrendingEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle operation failed event from JSON
    async fn handle_operation_failed_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing OperationFailedEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: OperationFailedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse OperationFailedEvent: {}", e))?;

        // Log operation failure to access_logs with error details
        // Note: OperationFailedEvent uses ip_id field name, which maps to mydata_id
        if let Some(mydata_id) = &parsed_event.ip_id {
            let mut conn = self.get_connection().await?;
            let access_log = NewMyDataAccessLog {
                mydata_id: mydata_id.clone(),
                user_address: parsed_event.user_address.clone().unwrap_or_else(|| "unknown".to_string()),
                access_type: format!("operation_failed_{}_{}", parsed_event.operation_type, parsed_event.error_code),
                access_time: parsed_event.timestamp as i64,
                transaction_id: transaction_id.to_string(),
            };

            diesel::insert_into(mydata_access_logs::table)
                .values(&access_log)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed OperationFailedEvent successfully");
        Ok(())
    }

    /// Handle system maintenance event from JSON
    async fn handle_system_maintenance_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing SystemMaintenanceEvent from JSON");

        let fields = extract_event_fields(data)?;
        let parsed_event: SystemMaintenanceEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse SystemMaintenanceEvent: {}", e))?;

        // Log maintenance event for each affected data entry
        let mut conn = self.get_connection().await?;
        for mydata_id in &parsed_event.affected_data {
            let access_log = NewMyDataAccessLog {
                mydata_id: mydata_id.clone(),
                user_address: "system".to_string(),
                access_type: format!("maintenance_{}", parsed_event.maintenance_type),
                access_time: parsed_event.start_time as i64,
                transaction_id: transaction_id.to_string(),
            };

            diesel::insert_into(mydata_access_logs::table)
                .values(&access_log)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed SystemMaintenanceEvent successfully for {} affected entries", parsed_event.affected_data.len());
        Ok(())
    }

    /// Handle data updated event
    async fn handle_data_updated(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataUpdatedEvent");

        let parsed_event = parse_event::<DataUpdatedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataUpdatedEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;

        let new_tags = serde_json::json!(parsed_event.new_tags);
        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::tags.eq(new_tags),
                mydata_data::one_time_price.eq(parsed_event.new_price_one_time.map(|p| p as i64)),
                mydata_data::subscription_price.eq(parsed_event.new_price_subscription.map(|p| p as i64)),
                mydata_data::data_quality.eq(parsed_event.new_data_quality.clone()),
                mydata_data::last_updated.eq(parsed_event.last_updated as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!("Processed DataUpdatedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data transferred event
    async fn handle_data_transferred(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataTransferredEvent");

        let parsed_event = parse_event::<DataTransferredEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataTransferredEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;

        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::owner.eq(&parsed_event.to_owner),
                mydata_data::last_updated.eq(parsed_event.transfer_time as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        if let Some(revenue) = parsed_event.into_revenue(transaction_id.to_string()) {
            diesel::insert_into(mydata_revenue::table)
                .values(&revenue)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed DataTransferredEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle subscription renewed event
    async fn handle_subscription_renewed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing SubscriptionRenewedEvent");

        let parsed_event = parse_event::<SubscriptionRenewedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse SubscriptionRenewedEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;

        diesel::update(mydata_subscriptions::table)
            .filter(mydata_subscriptions::mydata_id.eq(&parsed_event.mydata_id))
            .filter(mydata_subscriptions::subscriber.eq(&parsed_event.subscriber))
            .filter(mydata_subscriptions::subscription_end.eq(parsed_event.old_subscription_end as i64))
            .set(mydata_subscriptions::subscription_end.eq(parsed_event.new_subscription_end as i64))
            .execute(&mut conn)
            .await?;

        let renewal_subscription = parsed_event.into_subscription_update(transaction_id.to_string())?;
        diesel::insert_into(mydata_subscriptions::table)
            .values(&renewal_subscription)
            .execute(&mut conn)
            .await?;

        let owner: String = mydata_data::table
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .select(mydata_data::owner)
            .first(&mut conn)
            .await?;

        let mut revenue = parsed_event.into_revenue(transaction_id.to_string())?;
        revenue.to_address = owner;
        diesel::insert_into(mydata_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;

        info!("Processed SubscriptionRenewedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle subscription cancelled event
    async fn handle_subscription_cancelled(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing SubscriptionCancelledEvent");

        let parsed_event = parse_event::<SubscriptionCancelledEvent>(event)
            .map_err(|e| anyhow!("Failed to parse SubscriptionCancelledEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;

        diesel::update(mydata_subscriptions::table)
            .filter(mydata_subscriptions::mydata_id.eq(&parsed_event.mydata_id))
            .filter(mydata_subscriptions::subscriber.eq(&parsed_event.subscriber))
            .filter(mydata_subscriptions::subscription_end.gt(parsed_event.cancellation_time as i64))
            .set(mydata_subscriptions::subscription_end.eq(parsed_event.effective_end_time as i64))
            .execute(&mut conn)
            .await?;

        if parsed_event.refund_amount.is_some() && parsed_event.refund_amount.unwrap_or(0) > 0 {
            let owner: String = mydata_data::table
                .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
                .select(mydata_data::owner)
                .first(&mut conn)
                .await?;

            let refund_revenue = parsed_event.into_revenue(owner, transaction_id.to_string())?;
            diesel::insert_into(mydata_revenue::table)
                .values(&refund_revenue)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed SubscriptionCancelledEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data pricing changed event
    async fn handle_data_pricing_changed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataPricingChangedEvent");

        let parsed_event = parse_event::<DataPricingChangedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataPricingChangedEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;

        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::one_time_price.eq(parsed_event.new_one_time_price.map(|p| p as i64)),
                mydata_data::subscription_price.eq(parsed_event.new_subscription_price.map(|p| p as i64)),
                mydata_data::subscription_duration_days.eq(parsed_event.new_subscription_duration as i64),
                mydata_data::last_updated.eq(parsed_event.change_time as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!("Processed DataPricingChangedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data removed event
    async fn handle_data_removed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataRemovedEvent");

        let parsed_event = parse_event::<DataRemovedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataRemovedEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;

        diesel::update(mydata_data::table)
            .filter(mydata_data::mydata_id.eq(&parsed_event.mydata_id))
            .set((
                mydata_data::timestamp_end.eq(Some(parsed_event.removal_time as i64)),
                mydata_data::last_updated.eq(parsed_event.removal_time as i64),
                mydata_data::transaction_id.eq(transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        info!("Processed DataRemovedEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle data trending event
    async fn handle_data_trending(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataTrendingEvent");

        let parsed_event = parse_event::<DataTrendingEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataTrendingEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;
        let access_log = NewMyDataAccessLog {
            mydata_id: parsed_event.mydata_id.clone(),
            user_address: "system".to_string(),
            access_type: format!("trending_score_{}", parsed_event.trending_score),
            access_time: parsed_event.timestamp as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        info!("Processed DataTrendingEvent successfully for mydata_id: {}", parsed_event.mydata_id);
        Ok(())
    }

    /// Handle operation failed event
    async fn handle_operation_failed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing OperationFailedEvent");

        let parsed_event = parse_event::<OperationFailedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse OperationFailedEvent: {}", e))?;

        if let Some(mydata_id) = &parsed_event.ip_id {
            let mut conn = self.db.get_connection().await?;
            let access_log = NewMyDataAccessLog {
                mydata_id: mydata_id.clone(),
                user_address: parsed_event.user_address.clone().unwrap_or_else(|| "unknown".to_string()),
                access_type: format!("operation_failed_{}_{}", parsed_event.operation_type, parsed_event.error_code),
                access_time: parsed_event.timestamp as i64,
                transaction_id: transaction_id.to_string(),
            };

            diesel::insert_into(mydata_access_logs::table)
                .values(&access_log)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed OperationFailedEvent successfully");
        Ok(())
    }

    /// Handle system maintenance event
    async fn handle_system_maintenance(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing SystemMaintenanceEvent");

        let parsed_event = parse_event::<SystemMaintenanceEvent>(event)
            .map_err(|e| anyhow!("Failed to parse SystemMaintenanceEvent: {}", e))?;

        let mut conn = self.db.get_connection().await?;
        for mydata_id in &parsed_event.affected_data {
            let access_log = NewMyDataAccessLog {
                mydata_id: mydata_id.clone(),
                user_address: "system".to_string(),
                access_type: format!("maintenance_{}", parsed_event.maintenance_type),
                access_time: parsed_event.start_time as i64,
                transaction_id: transaction_id.to_string(),
            };

            diesel::insert_into(mydata_access_logs::table)
                .values(&access_log)
                .execute(&mut conn)
                .await?;
        }

        info!("Processed SystemMaintenanceEvent successfully for {} affected entries", parsed_event.affected_data.len());
        Ok(())
    }

    /// Handle MyDataCreatedEvent from JSON (actual contract event)
    async fn handle_mydata_created_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing MyDataCreatedEvent from JSON");

        // Parse the event using serde
        let fields = extract_event_fields(data)?;
        let event: MyDataCreatedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse MyDataCreatedEvent: {}", e))?;

        info!(
            "Parsed MyDataCreatedEvent: ip_id={}, owner={}, media_type={}",
            event.ip_id, event.owner, event.media_type
        );

        let mut conn = self.get_connection().await?;

        // Create a new data entry
        let new_data = crate::models::mydata::NewMyDataData {
            mydata_id: event.ip_id.clone(),
            owner: event.owner.clone(),
            media_type: event.media_type.clone(),
            tags: serde_json::json!([]), // Tags not in MyDataCreatedEvent
            platform_id: event.platform_id.clone(),
            timestamp_start: 0, // Not in event
            timestamp_end: None, // Not in event
            created_at: event.created_at as i64,
            last_updated: event.created_at as i64,
            one_time_price: event.one_time_price.map(|p| p as i64),
            subscription_price: event.subscription_price.map(|p| p as i64),
            subscription_duration_days: 30, // Default, not in event
            geographic_region: None,
            data_quality: None,
            sample_size: None,
            collection_method: None,
            is_updating: false,
            update_frequency: None,
            version: 1, // Default version for new data
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_data::table)
            .values(&new_data)
            .on_conflict(mydata_data::mydata_id)
            .do_update()
            .set((
                mydata_data::owner.eq(&new_data.owner),
                mydata_data::media_type.eq(&new_data.media_type),
                mydata_data::one_time_price.eq(new_data.one_time_price),
                mydata_data::subscription_price.eq(new_data.subscription_price),
                mydata_data::last_updated.eq(new_data.last_updated),
                mydata_data::transaction_id.eq(transaction_id.to_string()),
            ))
            .execute(&mut conn)
            .await?;

        info!(
            "Processed MyDataCreatedEvent successfully for ip_id: {}",
            event.ip_id
        );
        Ok(())
    }

    /// Handle PurchaseEvent from JSON (actual contract event - handles both one-time and subscription)
    async fn handle_purchase_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing PurchaseEvent from JSON");

        // Parse the event using serde
        let fields = extract_event_fields(data)?;
        let event: PurchaseEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse PurchaseEvent: {}", e))?;

        info!(
            "Parsed PurchaseEvent: ip_id={}, buyer={}, price={}, purchase_type={}",
            event.ip_id, event.buyer, event.price, event.purchase_type
        );

        let mut conn = self.get_connection().await?;

        // Record the purchase
        let purchase = crate::models::mydata::NewMyDataPurchase {
            mydata_id: event.ip_id.clone(),
            buyer: event.buyer.clone(),
            price: event.price as i64,
            purchase_type: event.purchase_type.clone(),
            purchase_time: event.timestamp as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;

        // If it's a subscription purchase, also create a subscription record
        if event.purchase_type == "subscription" {
            // We need to calculate subscription end time - but it's not in the event
            // We'll need to get it from the MyData record or use a default
            // For now, we'll create a subscription record with the purchase time as start
            // The actual end time should be calculated based on subscription_duration_days
            let subscription = crate::models::mydata::NewMyDataSubscription {
                mydata_id: event.ip_id.clone(),
                subscriber: event.buyer.clone(),
                subscription_start: event.timestamp as i64,
                subscription_end: event.timestamp as i64 + (30 * 24 * 60 * 60), // Default 30 days, should be fetched from MyData
                price: event.price as i64,
                transaction_id: transaction_id.to_string(),
            };

            diesel::insert_into(mydata_subscriptions::table)
                .values(&subscription)
                .on_conflict((mydata_subscriptions::mydata_id, mydata_subscriptions::subscriber))
                .do_update()
                .set((
                    mydata_subscriptions::subscription_end.eq(subscription.subscription_end),
                    mydata_subscriptions::transaction_id.eq(transaction_id.to_string()),
                ))
                .execute(&mut conn)
                .await?;
        }

        // Record access log
        let access_log = crate::models::NewMyDataAccessLog {
            mydata_id: event.ip_id.clone(),
            user_address: event.buyer.clone(),
            access_type: event.purchase_type.clone(),
            access_time: event.timestamp as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;

        // Record revenue - we need to get the owner from MyData record
        // For now, we'll leave to_address empty and update it later if needed
        let revenue = crate::models::mydata::NewMyDataRevenue {
            mydata_id: event.ip_id.clone(),
            from_address: event.buyer.clone(),
            to_address: "".to_string(), // Owner should be fetched from MyData record
            amount: event.price as i64,
            revenue_type: event.purchase_type.clone(),
            revenue_time: event.timestamp as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed PurchaseEvent successfully for ip_id: {}, purchase_type: {}",
            event.ip_id, event.purchase_type
        );
        Ok(())
    }

    /// Handle AccessGrantedEvent from JSON (actual contract event - handles pricing_update, content_update, and free access)
    async fn handle_access_granted_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing AccessGrantedEvent from JSON");

        // Parse the event using serde
        let fields = extract_event_fields(data)?;
        let event: AccessGrantedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse AccessGrantedEvent: {}", e))?;

        info!(
            "Parsed AccessGrantedEvent: ip_id={}, user={}, access_type={}, granted_by={}",
            event.ip_id, event.user, event.access_type, event.granted_by
        );

        let mut conn = self.get_connection().await?;

        // Handle different access types
        match event.access_type.as_str() {
            "pricing_update" => {
                // This is a pricing update - we should update the MyData record
                // But the event doesn't contain the new prices, so we'll just log it
                info!("Pricing update for MyData: {}", event.ip_id);
                
                // Log as access event for tracking
                let access_log = crate::models::NewMyDataAccessLog {
                    mydata_id: event.ip_id.clone(),
                    user_address: event.user.clone(),
                    access_type: "pricing_update".to_string(),
                    access_time: event.timestamp as i64,
                    transaction_id: transaction_id.to_string(),
                };

                diesel::insert_into(mydata_access_logs::table)
                    .values(&access_log)
                    .execute(&mut conn)
                    .await?;
            }
            "content_update" => {
                // This is a content update - update last_updated timestamp
                diesel::update(mydata_data::table)
                    .filter(mydata_data::mydata_id.eq(&event.ip_id))
                    .set(mydata_data::last_updated.eq(event.timestamp as i64))
                    .execute(&mut conn)
                    .await?;

                // Log as access event for tracking
                let access_log = crate::models::NewMyDataAccessLog {
                    mydata_id: event.ip_id.clone(),
                    user_address: event.user.clone(),
                    access_type: "content_update".to_string(),
                    access_time: event.timestamp as i64,
                    transaction_id: transaction_id.to_string(),
                };

                diesel::insert_into(mydata_access_logs::table)
                    .values(&access_log)
                    .execute(&mut conn)
                    .await?;
            }
            "one_time" | "subscription" => {
                // Free access granted - create access log and potentially subscription record
                let access_log = crate::models::NewMyDataAccessLog {
                    mydata_id: event.ip_id.clone(),
                    user_address: event.user.clone(),
                    access_type: event.access_type.clone(),
                    access_time: event.timestamp as i64,
                    transaction_id: transaction_id.to_string(),
                };

                diesel::insert_into(mydata_access_logs::table)
                    .values(&access_log)
                    .execute(&mut conn)
                    .await?;

                // If it's a subscription grant, create subscription record
                if event.access_type == "subscription" {
                    // Default to 30 days subscription duration
                    let subscription = crate::models::mydata::NewMyDataSubscription {
                        mydata_id: event.ip_id.clone(),
                        subscriber: event.user.clone(),
                        subscription_start: event.timestamp as i64,
                        subscription_end: event.timestamp as i64 + (30 * 24 * 60 * 60),
                        price: 0, // Free access
                        transaction_id: transaction_id.to_string(),
                    };

                    diesel::insert_into(mydata_subscriptions::table)
                        .values(&subscription)
                        .on_conflict((mydata_subscriptions::mydata_id, mydata_subscriptions::subscriber))
                        .do_update()
                        .set((
                            mydata_subscriptions::subscription_end.eq(subscription.subscription_end),
                            mydata_subscriptions::transaction_id.eq(transaction_id.to_string()),
                        ))
                        .execute(&mut conn)
                        .await?;
                }
            }
            _ => {
                warn!("Unknown access_type in AccessGrantedEvent: {}", event.access_type);
                
                // Still log it as an access event
                let access_log = crate::models::NewMyDataAccessLog {
                    mydata_id: event.ip_id.clone(),
                    user_address: event.user.clone(),
                    access_type: event.access_type.clone(),
                    access_time: event.timestamp as i64,
                    transaction_id: transaction_id.to_string(),
                };

                diesel::insert_into(mydata_access_logs::table)
                    .values(&access_log)
                    .execute(&mut conn)
                    .await?;
            }
        }

        info!(
            "Processed AccessGrantedEvent successfully for ip_id: {}, access_type: {}",
            event.ip_id, event.access_type
        );
        Ok(())
    }

    /// Handle MyDataRegisteredEvent from JSON data
    async fn handle_mydata_registered_from_json(
        &self,
        event_data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        let mut conn = self.db.get_connection().await?;
        process_mydata_registered_event(&mut conn, event_data, event_id).await?;
        Ok(())
    }

    /// Handle MyDataUnregisteredEvent from JSON data
    async fn handle_mydata_unregistered_from_json(
        &self,
        event_data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        let mut conn = self.db.get_connection().await?;
        process_mydata_unregistered_event(&mut conn, event_data, event_id).await?;
        Ok(())
    }

    /// Handle MyDataRegisteredEvent from parsed event
    async fn handle_mydata_registered(
        &self,
        event: &MysEvent,
        transaction_id: &str,
    ) -> Result<()> {
        let mut conn = self.db.get_connection().await?;
        let event_data = extract_event_fields(&serde_json::to_value(event)?)?;
        process_mydata_registered_event(&mut conn, &event_data, transaction_id).await?;
        Ok(())
    }

    /// Handle MyDataUnregisteredEvent from parsed event
    async fn handle_mydata_unregistered(
        &self,
        event: &MysEvent,
        transaction_id: &str,
    ) -> Result<()> {
        let mut conn = self.db.get_connection().await?;
        let event_data = extract_event_fields(&serde_json::to_value(event)?)?;
        process_mydata_unregistered_event(&mut conn, &event_data, transaction_id).await?;
        Ok(())
    }

    /// Handle MyDataConfigUpdatedEvent from JSON data
    async fn handle_mydata_config_updated_from_json(
        &self,
        data: &serde_json::Value,
        transaction_id: &str,
    ) -> Result<()> {
        info!("Processing MyDataConfigUpdatedEvent from JSON");

        let fields = extract_event_fields(data)?;
        let config_event: MyDataConfigUpdatedEvent = serde_json::from_value(fields)
            .map_err(|e| anyhow!("Failed to parse MyDataConfigUpdatedEvent: {}", e))?;

        let mut conn = self.get_connection().await?;

        let new_config = crate::models::mydata::NewMyDataConfig {
            updated_by: config_event.updated_by.clone(),
            enable_flag: config_event.enable_flag,
            max_tags: config_event.max_tags as i64,
            max_subscription_days: config_event.max_subscription_days as i64,
            max_free_access_grants: config_event.max_free_access_grants as i64,
            timestamp_ms: config_event.timestamp as i64,
            transaction_id: transaction_id.to_string(),
        };

        diesel::insert_into(mydata_config::table)
            .values(&new_config)
            .execute(&mut conn)
            .await?;

        info!(
            "Processed MyDataConfigUpdatedEvent successfully: updated_by={}, enable_flag={}, max_tags={}, max_subscription_days={}, max_free_access_grants={}",
            config_event.updated_by,
            config_event.enable_flag,
            config_event.max_tags,
            config_event.max_subscription_days,
            config_event.max_free_access_grants
        );

        Ok(())
    }
}
