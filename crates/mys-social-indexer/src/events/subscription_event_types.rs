// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::Utc;
use crate::models::subscription::*;

/// Subscription event types from the Move contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionEventType {
    ProfileSubscriptionCreated,
    ProfileSubscriptionRenewed,
    ProfileSubscriptionCancelled,
    ProfileSubscriptionUpdated,
}

impl SubscriptionEventType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            s if s.contains("::ProfileSubscriptionCreatedEvent") => Some(Self::ProfileSubscriptionCreated),
            s if s.contains("::ProfileSubscriptionRenewedEvent") => Some(Self::ProfileSubscriptionRenewed),
            s if s.contains("::ProfileSubscriptionCancelledEvent") => Some(Self::ProfileSubscriptionCancelled),
            s if s.contains("::ProfileSubscriptionUpdatedEvent") => Some(Self::ProfileSubscriptionUpdated),
            _ => None,
        }
    }
    
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::ProfileSubscriptionCreated => "ProfileSubscriptionCreatedEvent",
            Self::ProfileSubscriptionRenewed => "ProfileSubscriptionRenewedEvent",
            Self::ProfileSubscriptionCancelled => "ProfileSubscriptionCancelledEvent",
            Self::ProfileSubscriptionUpdated => "ProfileSubscriptionUpdatedEvent",
        }
    }
}

impl From<SubscriptionEventType> for String {
    fn from(event_type: SubscriptionEventType) -> Self {
        event_type.to_str().to_string()
    }
}

/// Event emitted when a profile subscription is created
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSubscriptionCreatedEvent {
    pub service_id: String,
    pub subscriber: String,
    pub expires_at: u64,
    pub monthly_fee: u64,
    pub auto_renew: bool,
}

impl ProfileSubscriptionCreatedEvent {
    pub fn into_model(&self, timestamp: u64, tx_id: String) -> Result<NewProfileSubscription> {
        Ok(NewProfileSubscription {
            subscription_id: generate_subscription_id(),
            service_id: self.service_id.clone(),
            subscriber: self.subscriber.clone(),
            created_at: timestamp as i64,
            expires_at: self.expires_at as i64,
            auto_renew: self.auto_renew,
            renewal_balance: 0,
            renewal_count: 0,
            cancelled_at: None,
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }

    pub fn into_revenue_model(&self, tx_id: String, profile_owner: String) -> Result<NewSubscriptionRevenue> {
        Ok(NewSubscriptionRevenue {
            service_id: self.service_id.clone(),
            subscription_id: Some(generate_subscription_id()),
            from_address: self.subscriber.clone(),
            to_address: profile_owner,
            amount: self.monthly_fee as i64,
            revenue_type: "subscription".to_string(),
            payment_time: self.expires_at as i64 - (30 * 24 * 60 * 60 * 1000), // Calculate payment time
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }

    pub fn into_event_model(&self, tx_id: String) -> Result<NewSubscriptionEvent> {
        Ok(NewSubscriptionEvent {
            event_type: SubscriptionEventType::ProfileSubscriptionCreated.to_str().to_string(),
            subscription_id: Some(generate_subscription_id()),
            service_id: Some(self.service_id.clone()),
            subscriber: Some(self.subscriber.clone()),
            event_data: serde_json::to_value(self)?,
            event_time: self.expires_at as i64 - (30 * 24 * 60 * 60 * 1000),
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }
}

/// Event emitted when a subscription is renewed
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSubscriptionRenewedEvent {
    pub subscription_id: String,
    pub subscriber: String,
    pub new_expires_at: u64,
    pub renewal_count: u64,
    pub auto_renewed: bool,
}

impl ProfileSubscriptionRenewedEvent {
    pub fn into_event_model(&self, tx_id: String) -> Result<NewSubscriptionEvent> {
        Ok(NewSubscriptionEvent {
            event_type: SubscriptionEventType::ProfileSubscriptionRenewed.to_str().to_string(),
            subscription_id: Some(self.subscription_id.clone()),
            service_id: None, // Will be fetched from existing subscription
            subscriber: Some(self.subscriber.clone()),
            event_data: serde_json::to_value(self)?,
            event_time: self.new_expires_at as i64,
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }

    pub fn into_revenue_model(&self, tx_id: String, service_id: String, monthly_fee: u64, profile_owner: String) -> Result<NewSubscriptionRevenue> {
        Ok(NewSubscriptionRevenue {
            service_id,
            subscription_id: Some(self.subscription_id.clone()),
            from_address: self.subscriber.clone(),
            to_address: profile_owner,
            amount: monthly_fee as i64,
            revenue_type: if self.auto_renewed { "auto_renewal".to_string() } else { "renewal".to_string() },
            payment_time: self.new_expires_at as i64 - (30 * 24 * 60 * 60 * 1000),
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }
}

/// Event emitted when a subscription is cancelled
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSubscriptionCancelledEvent {
    pub subscription_id: String,
    pub subscriber: String,
    pub refunded_amount: u64,
}

impl ProfileSubscriptionCancelledEvent {
    pub fn into_event_model(&self, tx_id: String) -> Result<NewSubscriptionEvent> {
        Ok(NewSubscriptionEvent {
            event_type: SubscriptionEventType::ProfileSubscriptionCancelled.to_str().to_string(),
            subscription_id: Some(self.subscription_id.clone()),
            service_id: None, // Will be fetched from existing subscription
            subscriber: Some(self.subscriber.clone()),
            event_data: serde_json::to_value(self)?,
            event_time: Utc::now().timestamp(),
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }

    pub fn into_revenue_model(&self, tx_id: String, service_id: String, profile_owner: String) -> Result<Option<NewSubscriptionRevenue>> {
        if self.refunded_amount > 0 {
            Ok(Some(NewSubscriptionRevenue {
                service_id,
                subscription_id: Some(self.subscription_id.clone()),
                from_address: profile_owner,
                to_address: self.subscriber.clone(),
                amount: -(self.refunded_amount as i64), // Negative for refund
                revenue_type: "refund".to_string(),
                payment_time: Utc::now().timestamp(),
                time: Utc::now().naive_utc(),
                transaction_id: tx_id,
                processing_success: true,
                processing_error: None,
            }))
        } else {
            Ok(None)
        }
    }
}

/// Event emitted when subscription service fee is updated
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSubscriptionUpdatedEvent {
    pub service_id: String,
    pub old_fee: u64,
    pub new_fee: u64,
    pub updated_by: String,
}

impl ProfileSubscriptionUpdatedEvent {
    pub fn into_event_model(&self, tx_id: String) -> Result<NewSubscriptionEvent> {
        Ok(NewSubscriptionEvent {
            event_type: SubscriptionEventType::ProfileSubscriptionUpdated.to_str().to_string(),
            subscription_id: None,
            service_id: Some(self.service_id.clone()),
            subscriber: None,
            event_data: serde_json::to_value(self)?,
            event_time: Utc::now().timestamp(),
            time: Utc::now().naive_utc(),
            transaction_id: tx_id,
            processing_success: true,
            processing_error: None,
        })
    }
}

/// Helper function to generate subscription ID
/// This could be replaced with actual logic to extract from blockchain event
pub fn generate_subscription_id() -> String {
    format!("sub_{}", uuid::Uuid::new_v4().to_string().replace("-", ""))
}

/// Helper function to validate subscription event data
pub fn validate_subscription_event(event_data: &serde_json::Value) -> Result<()> {
    // Basic validation that required fields are present
    if event_data.get("service_id").is_none() {
        return Err(anyhow::anyhow!("Missing service_id in subscription event"));
    }
    
    if event_data.get("subscriber").is_none() {
        return Err(anyhow::anyhow!("Missing subscriber in subscription event"));
    }
    
    Ok(())
}

/// Extract service ID from subscription event data
pub fn extract_service_id(event_data: &serde_json::Value) -> Option<String> {
    event_data.get("service_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract subscriber from subscription event data
pub fn extract_subscriber(event_data: &serde_json::Value) -> Option<String> {
    event_data.get("subscriber")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract subscription ID from subscription event data
pub fn extract_subscription_id(event_data: &serde_json::Value) -> Option<String> {
    event_data.get("subscription_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_subscription_event_type_from_str() {
        assert_eq!(
            SubscriptionEventType::from_str("social_contracts::subscription::ProfileSubscriptionCreatedEvent"),
            Some(SubscriptionEventType::ProfileSubscriptionCreated)
        );
        
        assert_eq!(
            SubscriptionEventType::from_str("social_contracts::subscription::ProfileSubscriptionRenewedEvent"),
            Some(SubscriptionEventType::ProfileSubscriptionRenewed)
        );
        
        assert_eq!(
            SubscriptionEventType::from_str("invalid_event"),
            None
        );
    }
    
    #[test]
    fn test_profile_subscription_created_event_conversion() {
        let event = ProfileSubscriptionCreatedEvent {
            service_id: "service_123".to_string(),
            subscriber: "subscriber_456".to_string(),
            expires_at: 1640995200000, // Jan 1, 2022 in milliseconds
            monthly_fee: 1000,
            auto_renew: true,
        };
        
        let model = event.into_model(1640995200, "tx_123".to_string()).unwrap();
        assert_eq!(model.service_id, "service_123");
        assert_eq!(model.subscriber, "subscriber_456");
        assert_eq!(model.auto_renew, true);
    }
    
    #[test]
    fn test_validate_subscription_event() {
        let valid_event = serde_json::json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000i64
        });
        
        assert!(validate_subscription_event(&valid_event).is_ok());
        
        let invalid_event = serde_json::json!({
            "expires_at": 1640995200000i64
        });
        
        assert!(validate_subscription_event(&invalid_event).is_err());
    }
} 