// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::{debug, info, error};
use bigdecimal::BigDecimal;
use chrono::Utc;
use tokio::sync::mpsc;

use crate::db::{Database, DbConnection};
use crate::events::{
    parse_event,
    my_ip_event_types::{
        DataCreatedEvent,
        DataPurchasedEvent,
        SubscriptionCreatedEvent,
        DataAccessGrantedEvent,
        RevenueDistributedEvent,
        DataAccessedEvent,
    },
    my_ip_events::{EventBatch}
};

use crate::schema::{my_ip_data, my_ip_purchases, my_ip_subscriptions, my_ip_revenue, my_ip_access_logs};
use mys_types::event::Event as MysEvent;

use super::listener::BlockchainEvent;

/// Handler for MyIP Data Marketplace events from the blockchain
pub struct MyIpEventHandler {
    /// Database connection
    db: Arc<Database>,
    /// Event receiver channel
    rx: mpsc::Receiver<BlockchainEvent>,
    /// Worker ID for tracking progress
    worker_id: String,
}

impl MyIpEventHandler {
    /// Create a new MyIpEventHandler with the given database connection
    pub fn new(db: Arc<Database>, rx: mpsc::Receiver<BlockchainEvent>, worker_id: String) -> Self {
        Self { db, rx, worker_id }
    }
    
    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db.get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }
    
    /// Start the event processing loop
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting MyIP marketplace event handler: {}", self.worker_id);
        
        while let Some(blockchain_event) = self.rx.recv().await {
            // Filter for MyIP marketplace events
            let event_type = &blockchain_event.event_type;
            
            if self.is_myip_event(event_type) {
                info!("Processing MyIP marketplace event: {}", event_type);
                
                if let Err(e) = self.handle_blockchain_event(&blockchain_event).await {
                    error!("Failed to process MyIP marketplace event {}: {}", blockchain_event.event_id, e);
                }
            }
        }
        
        info!("MyIP marketplace event handler stopped");
        Ok(())
    }
    
    /// Check if an event type is a MyIP marketplace event
    fn is_myip_event(&self, event_type: &str) -> bool {
        event_type.contains("::my_ip::") || 
        event_type.contains("::marketplace::") ||
        event_type.ends_with("::DataCreatedEvent") ||
        event_type.ends_with("::DataPurchasedEvent") ||
        event_type.ends_with("::SubscriptionCreatedEvent") ||
        event_type.ends_with("::DataAccessGrantedEvent") ||
        event_type.ends_with("::RevenueDistributedEvent") ||
        event_type.ends_with("::DataAccessedEvent")
    }
    
    /// Handle a blockchain event for MyIP marketplace events
    async fn handle_blockchain_event(&self, blockchain_event: &BlockchainEvent) -> Result<()> {
        let event_type = &blockchain_event.event_type;
        
        info!("Processing MyIP marketplace blockchain event: {}", event_type);
        
        // Process each marketplace event type based on the parsed JSON data
        match () {
            _ if event_type.ends_with("::DataCreatedEvent") => {
                self.handle_data_created_from_json(&blockchain_event.data, &blockchain_event.tx_digest).await?;
            },
            _ if event_type.ends_with("::DataPurchasedEvent") => {
                self.handle_data_purchased_from_json(&blockchain_event.data, &blockchain_event.tx_digest).await?;
            },
            _ if event_type.ends_with("::SubscriptionCreatedEvent") => {
                self.handle_subscription_created_from_json(&blockchain_event.data, &blockchain_event.tx_digest).await?;
            },
            _ if event_type.ends_with("::DataAccessGrantedEvent") => {
                self.handle_data_access_granted_from_json(&blockchain_event.data, &blockchain_event.tx_digest).await?;
            },
            _ if event_type.ends_with("::RevenueDistributedEvent") => {
                self.handle_revenue_distributed_from_json(&blockchain_event.data, &blockchain_event.tx_digest).await?;
            },
            _ if event_type.ends_with("::DataAccessedEvent") => {
                self.handle_data_accessed_from_json(&blockchain_event.data, &blockchain_event.tx_digest).await?;
            },
            _ => {
                debug!("Unhandled MyIP marketplace event type: {}", event_type);
            }
        }
        
        Ok(())
    }
    
    /// Handle data created event from JSON
    async fn handle_data_created_from_json(&self, data: &serde_json::Value, transaction_id: &str) -> Result<()> {
        info!("Processing DataCreatedEvent from JSON");
        
        // Extract fields from the JSON data
        let ip_id = data.get("ip_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ip_id field"))?;
        
        let owner = data.get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing owner field"))?;
        
        let media_type = data.get("media_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing media_type field"))?;
        
        info!("Parsed DataCreatedEvent: ip_id={}, owner={}, media_type={}", 
            ip_id, owner, media_type);
        
        // Get a database connection
        let mut conn = self.get_connection().await?;
        
        // Create a new data entry manually from the JSON data
        let new_data = crate::models::my_ip::NewMyIPData {
            ip_id: ip_id.to_string(),
            owner: owner.to_string(),
            media_type: media_type.to_string(),
            tags: data.get("tags").cloned().unwrap_or(serde_json::json!([])),
            platform_id: data.get("platform_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            timestamp_start: data.get("timestamp_start").and_then(|v| v.as_i64()).unwrap_or(0),
            timestamp_end: data.get("timestamp_end").and_then(|v| v.as_i64()),
            created_at: Utc::now().timestamp(),
            last_updated: Utc::now().timestamp(),
            one_time_price: data.get("one_time_price").and_then(|v| v.as_i64()),
            subscription_price: data.get("subscription_price").and_then(|v| v.as_i64()),
            subscription_duration_days: data.get("subscription_duration_days").and_then(|v| v.as_i64()).unwrap_or(30),
            geographic_region: data.get("geographic_region").and_then(|v| v.as_str()).map(|s| s.to_string()),
            data_quality: data.get("data_quality").and_then(|v| v.as_str()).map(|s| s.to_string()),
            sample_size: data.get("sample_size").and_then(|v| v.as_i64()),
            collection_method: data.get("collection_method").and_then(|v| v.as_str()).map(|s| s.to_string()),
            is_updating: data.get("is_updating").and_then(|v| v.as_bool()).unwrap_or(false),
            update_frequency: data.get("update_frequency").and_then(|v| v.as_str()).map(|s| s.to_string()),
            version: data.get("version").and_then(|v| v.as_i64()).unwrap_or(1),
            transaction_id: transaction_id.to_string(),
        };
        
        // Insert the new data entry into the database
        diesel::insert_into(my_ip_data::table)
            .values(&new_data)
            .on_conflict(my_ip_data::ip_id)
            .do_update()
            .set((
                my_ip_data::owner.eq(&new_data.owner),
                my_ip_data::media_type.eq(&new_data.media_type),
                my_ip_data::tags.eq(&new_data.tags),
                my_ip_data::one_time_price.eq(&new_data.one_time_price),
                my_ip_data::subscription_price.eq(&new_data.subscription_price),
                my_ip_data::data_quality.eq(&new_data.data_quality),
                my_ip_data::last_updated.eq(Utc::now().timestamp()),
                my_ip_data::transaction_id.eq(transaction_id)
            ))
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataCreatedEvent successfully for ip_id: {}", ip_id);
        Ok(())
    }
    
    /// Handle other marketplace events from JSON (simplified implementations)
    async fn handle_data_purchased_from_json(&self, data: &serde_json::Value, transaction_id: &str) -> Result<()> {
        info!("Processing DataPurchasedEvent from JSON");
        
        // Extract fields from JSON data according to event structure
        let ip_id = data.get("ip_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ip_id field"))?;
        
        let buyer = data.get("buyer")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing buyer field"))?;
        
        let price = data.get("price")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing price field"))?;
        
        let purchase_time = data.get("purchase_time")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing purchase_time field"))?;
        
        let mut conn = self.get_connection().await?;
        
        // Record the purchase
        let purchase = crate::models::NewMyIPPurchase {
            ip_id: ip_id.to_string(),
            buyer: buyer.to_string(),
            price: price as i64,
            purchase_type: crate::models::my_ip::PURCHASE_TYPE_ONE_TIME.to_string(),
            purchase_time: purchase_time as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;
        
        // Record access log
        let access_log = crate::models::NewMyIPAccessLog {
            ip_id: ip_id.to_string(),
            user_address: buyer.to_string(),
            access_type: crate::models::my_ip::ACCESS_TYPE_ONE_TIME.to_string(),
            access_time: purchase_time as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataPurchasedEvent from JSON for ip_id: {}", ip_id);
        Ok(())
    }
    
    async fn handle_subscription_created_from_json(&self, data: &serde_json::Value, transaction_id: &str) -> Result<()> {
        info!("Processing SubscriptionCreatedEvent from JSON");
        
        // Extract fields from JSON data according to event structure
        let ip_id = data.get("ip_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ip_id field"))?;
        
        let subscriber = data.get("subscriber")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing subscriber field"))?;
        
        let subscription_start = data.get("subscription_start")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing subscription_start field"))?;
        
        let subscription_end = data.get("subscription_end")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing subscription_end field"))?;
        
        let price = data.get("price")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing price field"))?;
        
        let mut conn = self.get_connection().await?;
        
        // Record the subscription
        let subscription = crate::models::NewMyIPSubscription {
            ip_id: ip_id.to_string(),
            subscriber: subscriber.to_string(),
            subscription_start: subscription_start as i64,
            subscription_end: subscription_end as i64,
            price: price as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_subscriptions::table)
            .values(&subscription)
            .execute(&mut conn)
            .await?;
        
        // Record as a purchase too for analytics
        let purchase = crate::models::NewMyIPPurchase {
            ip_id: ip_id.to_string(),
            buyer: subscriber.to_string(),
            price: price as i64,
            purchase_type: crate::models::my_ip::PURCHASE_TYPE_SUBSCRIPTION.to_string(),
            purchase_time: subscription_start as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;
        
        // Record access log
        let access_log = crate::models::NewMyIPAccessLog {
            ip_id: ip_id.to_string(),
            user_address: subscriber.to_string(),
            access_type: crate::models::my_ip::ACCESS_TYPE_SUBSCRIPTION.to_string(),
            access_time: subscription_start as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed SubscriptionCreatedEvent from JSON for ip_id: {}", ip_id);
        Ok(())
    }
    
    async fn handle_data_access_granted_from_json(&self, data: &serde_json::Value, transaction_id: &str) -> Result<()> {
        info!("Processing DataAccessGrantedEvent from JSON");
        
        // Extract fields from JSON data according to event structure
        let ip_id = data.get("ip_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ip_id field"))?;
        
        let grantee = data.get("grantee")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing grantee field"))?;
        
        let grant_time = data.get("grant_time")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing grant_time field"))?;
        
        let mut conn = self.get_connection().await?;
        
        // Record access log
        let access_log = crate::models::NewMyIPAccessLog {
            ip_id: ip_id.to_string(),
            user_address: grantee.to_string(),
            access_type: crate::models::my_ip::ACCESS_TYPE_GRANT.to_string(),
            access_time: grant_time as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataAccessGrantedEvent from JSON for ip_id: {}", ip_id);
        Ok(())
    }
    
    async fn handle_revenue_distributed_from_json(&self, data: &serde_json::Value, transaction_id: &str) -> Result<()> {
        info!("Processing RevenueDistributedEvent from JSON");
        
        // Extract fields from JSON data according to event structure
        let ip_id = data.get("ip_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ip_id field"))?;
        
        let from_address = data.get("from_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing from_address field"))?;
        
        let to_address = data.get("to_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing to_address field"))?;
        
        let amount = data.get("amount")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing amount field"))?;
        
        let revenue_type = data.get("revenue_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing revenue_type field"))?;
        
        let distribution_time = data.get("distribution_time")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing distribution_time field"))?;
        
        let mut conn = self.get_connection().await?;
        
        // Record revenue distribution
        let revenue = crate::models::NewMyIPRevenue {
            ip_id: ip_id.to_string(),
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
            amount: amount as i64,
            revenue_type: revenue_type.to_string(),
            revenue_time: distribution_time as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;
        
        // Also create unified revenue record
        let unified_revenue = crate::models::NewUnifiedRevenue::from_myip(
            revenue_type.to_string(),
            to_address.to_string(), // creator is recipient
            amount as i64,
            ip_id.to_string(),
            from_address.to_string(), // payer
            to_address.to_string(), // recipient  
            distribution_time as i64,
            transaction_id.to_string(),
        );
        
        diesel::insert_into(crate::schema::unified_revenue::table)
            .values(&unified_revenue)
            .execute(&mut conn)
            .await?;
        
        info!("Processed RevenueDistributedEvent from JSON for ip_id: {}", ip_id);
        Ok(())
    }
    
    async fn handle_data_accessed_from_json(&self, data: &serde_json::Value, transaction_id: &str) -> Result<()> {
        info!("Processing DataAccessedEvent from JSON");
        
        // Extract fields from JSON data according to event structure
        let ip_id = data.get("ip_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ip_id field"))?;
        
        let user_address = data.get("user_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing user_address field"))?;
        
        let access_type = data.get("access_type")
            .and_then(|v| v.as_str())
            .unwrap_or("preview");
        
        let access_time = data.get("access_time")
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
        
        let mut conn = self.get_connection().await?;
        
        // Record access log
        let access_log = crate::models::NewMyIPAccessLog {
            ip_id: ip_id.to_string(),
            user_address: user_address.to_string(),
            access_type: access_type.to_string(),
            access_time: access_time as i64,
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataAccessedEvent from JSON for ip_id: {}", ip_id);
        Ok(())
    }

    /// Handle a MyIP marketplace event from the blockchain
    pub async fn handle_event(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        let event_type = &event.type_.to_string(); // Convert StructTag to String
        
        info!("Processing MyIP marketplace event: {}", event_type);
        
        // Process each marketplace event type
        match () {
            _ if event_type.ends_with("::DataCreatedEvent") => {
                self.handle_data_created(event, transaction_id).await?;
            },
            _ if event_type.ends_with("::DataPurchasedEvent") => {
                self.handle_data_purchased(event, transaction_id).await?;
            },
            _ if event_type.ends_with("::SubscriptionCreatedEvent") => {
                self.handle_subscription_created(event, transaction_id).await?;
            },
            _ if event_type.ends_with("::DataAccessGrantedEvent") => {
                self.handle_data_access_granted(event, transaction_id).await?;
            },
            _ if event_type.ends_with("::RevenueDistributedEvent") => {
                self.handle_revenue_distributed(event, transaction_id).await?;
            },
            _ if event_type.ends_with("::DataAccessedEvent") => {
                self.handle_data_accessed(event, transaction_id).await?;
            },
            _ => {
                debug!("Unhandled MyIP marketplace event type: {}", event_type);
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
        
        info!("Parsed DataCreatedEvent: ip_id={}, owner={}, media_type={}", 
            parsed_event.ip_id, parsed_event.owner, parsed_event.media_type);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Convert event to model
        let new_data = parsed_event.into_model(transaction_id.to_string())?;
        
        // Insert the new data entry into the database
        diesel::insert_into(my_ip_data::table)
            .values(&new_data)
            .on_conflict(my_ip_data::ip_id)
            .do_update()
            .set((
                my_ip_data::owner.eq(&new_data.owner),
                my_ip_data::media_type.eq(&new_data.media_type),
                my_ip_data::tags.eq(&new_data.tags),
                my_ip_data::one_time_price.eq(&new_data.one_time_price),
                my_ip_data::subscription_price.eq(&new_data.subscription_price),
                my_ip_data::data_quality.eq(&new_data.data_quality),
                my_ip_data::last_updated.eq(Utc::now().timestamp()),
                my_ip_data::transaction_id.eq(transaction_id)
            ))
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataCreatedEvent successfully for ip_id: {}", parsed_event.ip_id);
        Ok(())
    }

    /// Handle data purchased event
    async fn handle_data_purchased(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataPurchasedEvent");
        
        let parsed_event = parse_event::<DataPurchasedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataPurchasedEvent: {}", e))?;
        
        info!("Parsed DataPurchasedEvent: ip_id={}, buyer={}, price={}", 
            parsed_event.ip_id, parsed_event.buyer, parsed_event.price);
        
        let mut conn = self.db.get_connection().await?;
        
        // Record the purchase
        let purchase = parsed_event.into_purchase(transaction_id.to_string())?;
        diesel::insert_into(my_ip_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;
        
        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataPurchasedEvent successfully for ip_id: {}", parsed_event.ip_id);
        Ok(())
    }

    /// Handle subscription created event
    async fn handle_subscription_created(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing SubscriptionCreatedEvent");
        
        let parsed_event = parse_event::<SubscriptionCreatedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse SubscriptionCreatedEvent: {}", e))?;
        
        info!("Parsed SubscriptionCreatedEvent: ip_id={}, subscriber={}, price={}", 
            parsed_event.ip_id, parsed_event.subscriber, parsed_event.price);
        
        let mut conn = self.db.get_connection().await?;
        
        // Record the subscription
        let subscription = parsed_event.into_subscription(transaction_id.to_string())?;
        diesel::insert_into(my_ip_subscriptions::table)
            .values(&subscription)
            .execute(&mut conn)
            .await?;
        
        // Record as a purchase too for analytics
        let purchase = parsed_event.into_purchase(transaction_id.to_string())?;
        diesel::insert_into(my_ip_purchases::table)
            .values(&purchase)
            .execute(&mut conn)
            .await?;
        
        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed SubscriptionCreatedEvent successfully for ip_id: {}", parsed_event.ip_id);
        Ok(())
    }

    /// Handle data access granted event
    async fn handle_data_access_granted(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataAccessGrantedEvent");
        
        let parsed_event = parse_event::<DataAccessGrantedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataAccessGrantedEvent: {}", e))?;
        
        info!("Parsed DataAccessGrantedEvent: ip_id={}, grantor={}, grantee={}", 
            parsed_event.ip_id, parsed_event.grantor, parsed_event.grantee);
        
        let mut conn = self.db.get_connection().await?;
        
        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        info!("Processed DataAccessGrantedEvent successfully for ip_id: {}", parsed_event.ip_id);
        Ok(())
    }

    /// Handle data accessed event
    async fn handle_data_accessed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing DataAccessedEvent");
        
        let parsed_event = parse_event::<DataAccessedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse DataAccessedEvent: {}", e))?;
        
        info!("Parsed DataAccessedEvent: ip_id={}, user={}", 
            parsed_event.ip_id, parsed_event.user_address);
        
        let mut conn = self.db.get_connection().await?;
        
        // Record access log
        let access_log = parsed_event.into_access_log(transaction_id.to_string())?;
        diesel::insert_into(my_ip_access_logs::table)
            .values(&access_log)
            .execute(&mut conn)
            .await?;
        
        Ok(())
    }

    /// Handle revenue distributed event (updated for marketplace)
    async fn handle_revenue_distributed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing RevenueDistributedEvent");
        
        let parsed_event = parse_event::<RevenueDistributedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse RevenueDistributedEvent: {}", e))?;
        
        info!("Parsed RevenueDistributedEvent: ip_id={}, from={}, to={}, amount={}", 
            parsed_event.ip_id, parsed_event.from_address, parsed_event.to_address, parsed_event.amount);
        
        let mut conn = self.db.get_connection().await?;
        
        // Record revenue distribution
        let revenue = parsed_event.into_revenue(transaction_id.to_string())?;
        diesel::insert_into(my_ip_revenue::table)
            .values(&revenue)
            .execute(&mut conn)
            .await?;
        
        info!("Processed RevenueDistributedEvent successfully for ip_id: {}", parsed_event.ip_id);
        Ok(())
    }

    /// Update marketplace data statistics
    pub async fn update_data_statistics(&self, ip_id: &str) -> Result<()> {
        info!("Updating statistics for data: {}", ip_id);
        
        let mut conn = self.db.get_connection().await?;
        
        // Get total revenue for the data
        let total_revenue: Option<BigDecimal> = my_ip_revenue::table
            .filter(my_ip_revenue::ip_id.eq(ip_id))
            .select(diesel::dsl::sum(my_ip_revenue::amount))
            .first::<Option<BigDecimal>>(&mut conn)
            .await?;
        
        let total_revenue: i64 = total_revenue
            .map(|bd| bd.to_string().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        
        // Get purchase and subscription stats
        let purchase_count: i64 = my_ip_purchases::table
            .filter(my_ip_purchases::ip_id.eq(ip_id))
            .count()
            .get_result(&mut conn)
            .await?;
        
        let subscription_count: i64 = my_ip_subscriptions::table
            .filter(my_ip_subscriptions::ip_id.eq(ip_id))
            .count()
            .get_result(&mut conn)
            .await?;
        
        let access_count: i64 = my_ip_access_logs::table
            .filter(my_ip_access_logs::ip_id.eq(ip_id))
            .count()
            .get_result(&mut conn)
            .await?;
        
        info!("Data {} stats: purchases={}, subscriptions={}, accesses={}, total_revenue={}",
            ip_id, purchase_count, subscription_count, access_count, total_revenue);
        
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
                },
                _ if event_type.ends_with("::DataPurchasedEvent") => {
                    if let Ok(parsed) = parse_event::<DataPurchasedEvent>(event) {
                        event_batch.add_data_purchased(&parsed, transaction_id.to_string())?;
                    }
                },
                _ if event_type.ends_with("::SubscriptionCreatedEvent") => {
                    if let Ok(parsed) = parse_event::<SubscriptionCreatedEvent>(event) {
                        event_batch.add_subscription_created(&parsed, transaction_id.to_string())?;
                    }
                },
                _ if event_type.ends_with("::RevenueDistributedEvent") => {
                    if let Ok(parsed) = parse_event::<RevenueDistributedEvent>(event) {
                        event_batch.add_revenue_distributed(&parsed, transaction_id.to_string())?;
                    }
                },
                _ if event_type.ends_with("::DataAccessedEvent") => {
                    if let Ok(parsed) = parse_event::<DataAccessedEvent>(event) {
                        event_batch.add_data_accessed(&parsed, transaction_id.to_string())?;
                    }
                },
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
                diesel::insert_into(my_ip_data::table)
                    .values(&event_batch.data_entries)
                    .on_conflict(my_ip_data::ip_id)
                    .do_nothing()
                    .execute(&mut conn)
                    .await?;
            }
            
            if !event_batch.purchases.is_empty() {
                diesel::insert_into(my_ip_purchases::table)
                    .values(&event_batch.purchases)
                    .execute(&mut conn)
                    .await?;
            }
            
            if !event_batch.subscriptions.is_empty() {
                diesel::insert_into(my_ip_subscriptions::table)
                    .values(&event_batch.subscriptions)
                    .execute(&mut conn)
                    .await?;
            }
            
            if !event_batch.revenue_records.is_empty() {
                diesel::insert_into(my_ip_revenue::table)
                    .values(&event_batch.revenue_records)
                    .execute(&mut conn)
                    .await?;
            }
            
            if !event_batch.access_logs.is_empty() {
                diesel::insert_into(my_ip_access_logs::table)
                    .values(&event_batch.access_logs)
                    .execute(&mut conn)
                    .await?;
            }
        }
        
        info!("Processed batch of {} events successfully", events_len);
        Ok(())
    }
} 