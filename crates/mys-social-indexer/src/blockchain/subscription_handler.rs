// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use chrono::Utc;

use crate::db::{Database, DbConnection};
use crate::blockchain::listener::BlockchainEvent;
use crate::events::subscription_event_types::*;
use crate::events::subscription_events::*;
use crate::events::SubscriptionEventType;
use crate::models::subscription::*;
use crate::schema;
use crate::SUBSCRIPTION_MODULE_NAME;

/// Handler for subscription events from the blockchain
pub struct SubscriptionEventHandler {
    db: Arc<Database>,
    receiver: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl SubscriptionEventHandler {
    /// Create a new SubscriptionEventHandler instance
    pub fn new(
        db: Arc<Database>,
        receiver: mpsc::Receiver<BlockchainEvent>,
        worker_name: String,
    ) -> Self {
        Self {
            db,
            receiver,
            worker_name,
        }
    }

    /// Start the subscription event handler
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting subscription event handler: {}", self.worker_name);
        
        while let Some(event) = self.receiver.recv().await {
            // Extract the module name from the event type
            let parts: Vec<&str> = event.event_type.split("::").collect();
            if parts.len() < 2 {
                continue; // Skip malformed event types
            }
            
            let module_name = parts[1]; // Second part is the module name
            
            // Skip if this is not a subscription module event
            if module_name != SUBSCRIPTION_MODULE_NAME {
                continue;
            }
            
            let mut conn = self.db.get_connection().await?;
            if let Err(e) = self.process_event(&mut conn, &event).await {
                error!("Error processing subscription event {}: {}", event.event_id, e);
                
                // Log failed event for debugging
                if let Err(log_error) = self.log_failed_event(&mut conn, &event, &e.to_string()).await {
                    error!("Failed to log failed subscription event: {}", log_error);
                }
            }
        }
        
        Ok(())
    }

    /// Process a subscription event from the blockchain
    async fn process_event(&self, conn: &mut DbConnection, event: &BlockchainEvent) -> Result<()> {
        debug!("Processing subscription event: {}", event.event_type);

        // Validate and sanitize event data
        let mut event_data = event.data.clone();
        sanitize_event_data(&mut event_data)?;
        validate_subscription_event_detailed(&event_data, &event.event_type)?;
        validate_business_rules(&event_data, &event.event_type)?;

        // Determine event type and process accordingly
        if let Some(subscription_event_type) = SubscriptionEventType::from_str(&event.event_type) {
            match subscription_event_type {
                SubscriptionEventType::ProfileSubscriptionCreated => {
                    self.process_subscription_created(conn, &event_data, &event.tx_digest).await?;
                }
                SubscriptionEventType::ProfileSubscriptionRenewed => {
                    self.process_subscription_renewed(conn, &event_data, &event.tx_digest).await?;
                }
                SubscriptionEventType::ProfileSubscriptionCancelled => {
                    self.process_subscription_cancelled(conn, &event_data, &event.tx_digest).await?;
                }
                SubscriptionEventType::ProfileSubscriptionUpdated => {
                    self.process_subscription_updated(conn, &event_data, &event.tx_digest).await?;
                }
            }
        } else {
            warn!("Unknown subscription event type: {}", event.event_type);
        }

        info!("Successfully processed subscription event: {}", event.event_type);
        Ok(())
    }

    /// Process ProfileSubscriptionCreatedEvent
    async fn process_subscription_created(
        &self,
        conn: &mut DbConnection,
        event_data: &serde_json::Value,
        tx_id: &str,
    ) -> Result<()> {
        // Parse the event
        let event: ProfileSubscriptionCreatedEvent = parse_subscription_event(event_data)?;
        
        info!(
            "Processing subscription creation: service_id={}, subscriber={}",
            event.service_id, event.subscriber
        );

        // Start database transaction
        conn.transaction(|conn| {
            Box::pin(async move {
                // 1. Create subscription record
                let new_subscription = event.into_model(Utc::now().timestamp() as u64, tx_id.to_string())?;
                
                diesel::insert_into(schema::profile_subscriptions::table)
                    .values(&new_subscription)
                    .on_conflict(schema::profile_subscriptions::subscription_id)
                    .do_update()
                    .set((
                        schema::profile_subscriptions::expires_at.eq(&new_subscription.expires_at),
                        schema::profile_subscriptions::auto_renew.eq(&new_subscription.auto_renew),
                        schema::profile_subscriptions::processing_success.eq(true),
                        schema::profile_subscriptions::processing_error.eq::<Option<String>>(None),
                    ))
                    .execute(conn)
                    .await?;

                // 2. Update service subscriber count
                diesel::update(schema::profile_subscription_services::table)
                    .filter(schema::profile_subscription_services::service_id.eq(&event.service_id))
                    .set(schema::profile_subscription_services::subscriber_count.eq(
                        schema::profile_subscription_services::subscriber_count + 1
                    ))
                    .execute(conn)
                    .await?;

                // 3. Create revenue record
                let profile_owner = self.get_profile_owner_for_service(conn, &event.service_id).await?;
                let revenue = event.into_revenue_model(tx_id.to_string(), profile_owner)?;
                
                diesel::insert_into(schema::subscription_revenue::table)
                    .values(&revenue)
                    .execute(conn)
                    .await?;

                // 4. Create event log
                let event_log = event.into_event_model(tx_id.to_string())?;
                
                diesel::insert_into(schema::subscription_events::table)
                    .values(&event_log)
                    .execute(conn)
                    .await?;

                info!("Successfully created subscription: {}", new_subscription.subscription_id);
                Ok(())
            })
        }).await
    }

    /// Process ProfileSubscriptionRenewedEvent
    async fn process_subscription_renewed(
        &self,
        conn: &mut DbConnection,
        event_data: &serde_json::Value,
        tx_id: &str,
    ) -> Result<()> {
        // Parse the event
        let event: ProfileSubscriptionRenewedEvent = parse_subscription_event(event_data)?;
        
        info!(
            "Processing subscription renewal: subscription_id={}, new_expires_at={}",
            event.subscription_id, event.new_expires_at
        );

        // Start database transaction
        conn.transaction(|conn| {
            Box::pin(async move {
                // 1. Update subscription record
                diesel::update(schema::profile_subscriptions::table)
                    .filter(schema::profile_subscriptions::subscription_id.eq(&event.subscription_id))
                    .set((
                        schema::profile_subscriptions::expires_at.eq(event.new_expires_at as i64),
                        schema::profile_subscriptions::renewal_count.eq(event.renewal_count as i64),
                        schema::profile_subscriptions::processing_success.eq(true),
                        schema::profile_subscriptions::processing_error.eq::<Option<String>>(None),
                    ))
                    .execute(conn)
                    .await?;

                // 2. Get service details for revenue record
                let service_info: (String, i64, String) = schema::profile_subscriptions::table
                    .inner_join(schema::profile_subscription_services::table.on(
                        schema::profile_subscriptions::service_id.eq(schema::profile_subscription_services::service_id)
                    ))
                    .filter(schema::profile_subscriptions::subscription_id.eq(&event.subscription_id))
                    .select((
                        schema::profile_subscription_services::service_id,
                        schema::profile_subscription_services::monthly_fee,
                        schema::profile_subscription_services::profile_owner,
                    ))
                    .first(conn)
                    .await?;

                let (service_id, monthly_fee, profile_owner) = service_info;

                // 3. Create revenue record
                let revenue = event.into_revenue_model(tx_id.to_string(), service_id, monthly_fee as u64, profile_owner)?;
                
                diesel::insert_into(schema::subscription_revenue::table)
                    .values(&revenue)
                    .execute(conn)
                    .await?;

                // 4. Create event log
                let event_log = event.into_event_model(tx_id.to_string())?;
                
                diesel::insert_into(schema::subscription_events::table)
                    .values(&event_log)
                    .execute(conn)
                    .await?;

                info!("Successfully renewed subscription: {}", event.subscription_id);
                Ok(())
            })
        }).await
    }

    /// Process ProfileSubscriptionCancelledEvent
    async fn process_subscription_cancelled(
        &self,
        conn: &mut DbConnection,
        event_data: &serde_json::Value,
        tx_id: &str,
    ) -> Result<()> {
        // Parse the event
        let event: ProfileSubscriptionCancelledEvent = parse_subscription_event(event_data)?;
        
        info!(
            "Processing subscription cancellation: subscription_id={}, refunded_amount={}",
            event.subscription_id, event.refunded_amount
        );

        // Start database transaction
        conn.transaction(|conn| {
            Box::pin(async move {
                // 1. Update subscription record to mark as cancelled
                let now = Utc::now().timestamp();
                diesel::update(schema::profile_subscriptions::table)
                    .filter(schema::profile_subscriptions::subscription_id.eq(&event.subscription_id))
                    .set((
                        schema::profile_subscriptions::cancelled_at.eq(Some(now)),
                        schema::profile_subscriptions::processing_success.eq(true),
                        schema::profile_subscriptions::processing_error.eq::<Option<String>>(None),
                    ))
                    .execute(conn)
                    .await?;

                // 2. Get service details and update subscriber count
                let service_id: String = schema::profile_subscriptions::table
                    .filter(schema::profile_subscriptions::subscription_id.eq(&event.subscription_id))
                    .select(schema::profile_subscriptions::service_id)
                    .first(conn)
                    .await?;

                diesel::update(schema::profile_subscription_services::table)
                    .filter(schema::profile_subscription_services::service_id.eq(&service_id))
                    .set(schema::profile_subscription_services::subscriber_count.eq(
                        schema::profile_subscription_services::subscriber_count - 1
                    ))
                    .execute(conn)
                    .await?;

                // 3. Create refund revenue record if applicable
                if event.refunded_amount > 0 {
                    let profile_owner = self.get_profile_owner_for_service(conn, &service_id).await?;
                    if let Some(refund_revenue) = event.into_revenue_model(tx_id.to_string(), service_id.clone(), profile_owner)? {
                        diesel::insert_into(schema::subscription_revenue::table)
                            .values(&refund_revenue)
                            .execute(conn)
                            .await?;
                    }
                }

                // 4. Create event log
                let event_log = event.into_event_model(tx_id.to_string())?;
                
                diesel::insert_into(schema::subscription_events::table)
                    .values(&event_log)
                    .execute(conn)
                    .await?;

                info!("Successfully cancelled subscription: {}", event.subscription_id);
                Ok(())
            })
        }).await
    }

    /// Process ProfileSubscriptionUpdatedEvent
    async fn process_subscription_updated(
        &self,
        conn: &mut DbConnection,
        event_data: &serde_json::Value,
        tx_id: &str,
    ) -> Result<()> {
        // Parse the event
        let event: ProfileSubscriptionUpdatedEvent = parse_subscription_event(event_data)?;
        
        info!(
            "Processing subscription service update: service_id={}, old_fee={}, new_fee={}",
            event.service_id, event.old_fee, event.new_fee
        );

        // Start database transaction
        conn.transaction(|conn| {
            Box::pin(async move {
                // 1. Update service record
                let now = Utc::now().timestamp();
                diesel::update(schema::profile_subscription_services::table)
                    .filter(schema::profile_subscription_services::service_id.eq(&event.service_id))
                    .set((
                        schema::profile_subscription_services::monthly_fee.eq(event.new_fee as i64),
                        schema::profile_subscription_services::updated_at.eq(Some(now)),
                    ))
                    .execute(conn)
                    .await?;

                // 2. Create event log
                let event_log = event.into_event_model(tx_id.to_string())?;
                
                diesel::insert_into(schema::subscription_events::table)
                    .values(&event_log)
                    .execute(conn)
                    .await?;

                info!("Successfully updated subscription service: {}", event.service_id);
                Ok(())
            })
        }).await
    }

    /// Helper function to get profile owner for a service
    async fn get_profile_owner_for_service(&self, conn: &mut DbConnection, service_id: &str) -> Result<String> {
        let profile_owner: String = schema::profile_subscription_services::table
            .filter(schema::profile_subscription_services::service_id.eq(service_id))
            .select(schema::profile_subscription_services::profile_owner)
            .first(conn)
            .await
            .map_err(|_| anyhow!("Service not found: {}", service_id))?;
            
        Ok(profile_owner)
    }

    /// Log a failed event for debugging purposes
    async fn log_failed_event(
        &self,
        conn: &mut DbConnection,
        event: &BlockchainEvent,
        error_message: &str,
    ) -> Result<()> {
        let failed_event = NewSubscriptionEvent {
            event_type: event.event_type.clone(),
            subscription_id: extract_subscription_id(&event.data),
            service_id: extract_service_id(&event.data),
            subscriber: extract_subscriber(&event.data),
            event_data: event.data.clone(),
            event_time: Utc::now().timestamp(),
            time: Utc::now().naive_utc(),
            transaction_id: event.tx_digest.clone(),
            processing_success: false,
            processing_error: Some(error_message.to_string()),
        };

        diesel::insert_into(schema::subscription_events::table)
            .values(&failed_event)
            .execute(conn)
            .await?;

        Ok(())
    }
}

/// Direct event handler for integration with main blockchain event system
/// This function is called by the main event dispatcher
pub async fn handle_subscription_event(
    db: &Arc<Database>,
    event: &BlockchainEvent,
) -> Result<()> {
    let handler = SubscriptionEventHandler::new(
        db.clone(),
        mpsc::channel(1).1, // Dummy receiver, won't be used
        "direct-handler".to_string(),
    );

    let mut conn = db.get_connection().await?;
    handler.process_event(&mut conn, event).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_subscription_event_validation() {
        let valid_created_event = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 1000u64,
            "auto_renew": true
        });
        
        assert!(validate_subscription_event_detailed(
            &valid_created_event, 
            "ProfileSubscriptionCreatedEvent"
        ).is_ok());
        
        let invalid_event = json!({
            "service_id": "service_123"
            // Missing required fields
        });
        
        assert!(validate_subscription_event_detailed(
            &invalid_event, 
            "ProfileSubscriptionCreatedEvent"
        ).is_err());
    }
    
    #[test]
    fn test_parse_subscription_created_event() {
        let event_data = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 1000u64,
            "auto_renew": true
        });
        
        let parsed: Result<ProfileSubscriptionCreatedEvent> = parse_subscription_event(&event_data);
        assert!(parsed.is_ok());
        
        let event = parsed.unwrap();
        assert_eq!(event.service_id, "service_123");
        assert_eq!(event.subscriber, "subscriber_456");
        assert_eq!(event.monthly_fee, 1000);
        assert!(event.auto_renew);
    }
    
    #[test]
    fn test_subscription_business_rules_validation() {
        let valid_event = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 1000u64,
            "auto_renew": true
        });
        
        assert!(validate_business_rules(&valid_event, "ProfileSubscriptionCreatedEvent").is_ok());
        
        let invalid_fee_event = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 2_000_000_000u64, // Exceeds maximum
            "auto_renew": true
        });
        
        assert!(validate_business_rules(&invalid_fee_event, "ProfileSubscriptionCreatedEvent").is_err());
    }
} 