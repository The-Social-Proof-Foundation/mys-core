// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde_json::Value;
use tracing::{debug, error};

use crate::events::subscription_event_types::*;

/// Parse and validate subscription events from blockchain
pub fn parse_subscription_event<T>(event_data: &Value) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    // First validate the event has required fields
    validate_subscription_event(event_data)?;
    
    // Parse the specific event type
    serde_json::from_value(event_data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse subscription event: {}", e))
}

/// Comprehensive validation for subscription events
pub fn validate_subscription_event_detailed(event_data: &Value, event_type: &str) -> Result<()> {
    match event_type {
        t if t.contains("ProfileSubscriptionCreatedEvent") => {
            validate_created_event(event_data)
        }
        t if t.contains("ProfileSubscriptionRenewedEvent") => {
            validate_renewed_event(event_data)
        }
        t if t.contains("ProfileSubscriptionCancelledEvent") => {
            validate_cancelled_event(event_data)
        }
        t if t.contains("ProfileSubscriptionUpdatedEvent") => {
            validate_updated_event(event_data)
        }
        _ => {
            error!("Unknown subscription event type: {}", event_type);
            Err(anyhow::anyhow!("Unknown subscription event type: {}", event_type))
        }
    }
}

/// Validate ProfileSubscriptionCreatedEvent structure
fn validate_created_event(event_data: &Value) -> Result<()> {
    let required_fields = ["service_id", "subscriber", "expires_at", "monthly_fee"];
    
    for field in &required_fields {
        if event_data.get(field).is_none() {
            return Err(anyhow::anyhow!("Missing required field '{}' in ProfileSubscriptionCreatedEvent", field));
        }
    }
    
    // Validate data types
    if let Some(expires_at) = event_data.get("expires_at") {
        if !expires_at.is_number() {
            return Err(anyhow::anyhow!("expires_at must be a number"));
        }
    }
    
    if let Some(monthly_fee) = event_data.get("monthly_fee") {
        if !monthly_fee.is_number() {
            return Err(anyhow::anyhow!("monthly_fee must be a number"));
        }
    }
    
    if let Some(auto_renew) = event_data.get("auto_renew") {
        if !auto_renew.is_boolean() {
            return Err(anyhow::anyhow!("auto_renew must be a boolean"));
        }
    }
    
    debug!("ProfileSubscriptionCreatedEvent validation passed");
    Ok(())
}

/// Validate ProfileSubscriptionRenewedEvent structure
fn validate_renewed_event(event_data: &Value) -> Result<()> {
    let required_fields = ["subscription_id", "subscriber", "new_expires_at", "renewal_count"];
    
    for field in &required_fields {
        if event_data.get(field).is_none() {
            return Err(anyhow::anyhow!("Missing required field '{}' in ProfileSubscriptionRenewedEvent", field));
        }
    }
    
    // Validate data types
    if let Some(new_expires_at) = event_data.get("new_expires_at") {
        if !new_expires_at.is_number() {
            return Err(anyhow::anyhow!("new_expires_at must be a number"));
        }
    }
    
    if let Some(renewal_count) = event_data.get("renewal_count") {
        if !renewal_count.is_number() {
            return Err(anyhow::anyhow!("renewal_count must be a number"));
        }
    }
    
    if let Some(auto_renewed) = event_data.get("auto_renewed") {
        if !auto_renewed.is_boolean() {
            return Err(anyhow::anyhow!("auto_renewed must be a boolean"));
        }
    }
    
    debug!("ProfileSubscriptionRenewedEvent validation passed");
    Ok(())
}

/// Validate ProfileSubscriptionCancelledEvent structure
fn validate_cancelled_event(event_data: &Value) -> Result<()> {
    let required_fields = ["subscription_id", "subscriber", "refunded_amount"];
    
    for field in &required_fields {
        if event_data.get(field).is_none() {
            return Err(anyhow::anyhow!("Missing required field '{}' in ProfileSubscriptionCancelledEvent", field));
        }
    }
    
    // Validate data types
    if let Some(refunded_amount) = event_data.get("refunded_amount") {
        if !refunded_amount.is_number() {
            return Err(anyhow::anyhow!("refunded_amount must be a number"));
        }
    }
    
    debug!("ProfileSubscriptionCancelledEvent validation passed");
    Ok(())
}

/// Validate ProfileSubscriptionUpdatedEvent structure
fn validate_updated_event(event_data: &Value) -> Result<()> {
    let required_fields = ["service_id", "old_fee", "new_fee", "updated_by"];
    
    for field in &required_fields {
        if event_data.get(field).is_none() {
            return Err(anyhow::anyhow!("Missing required field '{}' in ProfileSubscriptionUpdatedEvent", field));
        }
    }
    
    // Validate data types
    if let Some(old_fee) = event_data.get("old_fee") {
        if !old_fee.is_number() {
            return Err(anyhow::anyhow!("old_fee must be a number"));
        }
    }
    
    if let Some(new_fee) = event_data.get("new_fee") {
        if !new_fee.is_number() {
            return Err(anyhow::anyhow!("new_fee must be a number"));
        }
    }
    
    debug!("ProfileSubscriptionUpdatedEvent validation passed");
    Ok(())
}

/// Extract profile owner from service ID
/// This would typically require a database lookup to get the profile owner from service ID
pub fn extract_profile_owner_from_service(service_id: &str) -> Result<String> {
    // This is a placeholder - in real implementation, this would:
    // 1. Query the profile_subscription_services table
    // 2. Get the profile_owner field for the given service_id
    // For now, we'll use a placeholder logic
    debug!("Extracting profile owner for service_id: {}", service_id);
    
    // Placeholder logic - in real implementation this would be a database query
    Ok(format!("profile_owner_for_{}", service_id))
}

/// Subscription event error types
#[derive(Debug, thiserror::Error)]
pub enum SubscriptionEventError {
    #[error("Invalid event structure: {0}")]
    InvalidStructure(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid data type for field {field}: expected {expected}, got {actual}")]
    InvalidDataType {
        field: String,
        expected: String,
        actual: String,
    },
    
    #[error("Database operation failed: {0}")]
    DatabaseError(String),
    
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(String),
}

/// Constants for subscription validation
pub const MAX_MONTHLY_FEE: u64 = 1_000_000_000; // 1 billion in smallest units
pub const MIN_MONTHLY_FEE: u64 = 1; // Minimum 1 unit
pub const MAX_REFUND_AMOUNT: u64 = 1_000_000_000;

/// Validate business rules for subscription events
pub fn validate_business_rules(event_data: &Value, event_type: &str) -> Result<()> {
    match event_type {
        t if t.contains("ProfileSubscriptionCreatedEvent") => {
            if let Some(monthly_fee) = event_data.get("monthly_fee").and_then(|v| v.as_u64()) {
                if monthly_fee < MIN_MONTHLY_FEE || monthly_fee > MAX_MONTHLY_FEE {
                    return Err(anyhow::anyhow!(
                        "Monthly fee {} is outside valid range [{}, {}]",
                        monthly_fee, MIN_MONTHLY_FEE, MAX_MONTHLY_FEE
                    ));
                }
            }
        }
        t if t.contains("ProfileSubscriptionCancelledEvent") => {
            if let Some(refunded_amount) = event_data.get("refunded_amount").and_then(|v| v.as_u64()) {
                if refunded_amount > MAX_REFUND_AMOUNT {
                    return Err(anyhow::anyhow!(
                        "Refund amount {} exceeds maximum allowed {}",
                        refunded_amount, MAX_REFUND_AMOUNT
                    ));
                }
            }
        }
        _ => {
            // No specific business rules for other event types
        }
    }
    
    Ok(())
}

/// Sanitize and normalize subscription event data
pub fn sanitize_event_data(event_data: &mut Value) -> Result<()> {
    // Ensure all string fields are properly trimmed
    if let Some(service_id) = event_data.get_mut("service_id") {
        if let Some(s) = service_id.as_str() {
            *service_id = Value::String(s.trim().to_string());
        }
    }
    
    if let Some(subscriber) = event_data.get_mut("subscriber") {
        if let Some(s) = subscriber.as_str() {
            *subscriber = Value::String(s.trim().to_string());
        }
    }
    
    if let Some(subscription_id) = event_data.get_mut("subscription_id") {
        if let Some(s) = subscription_id.as_str() {
            *subscription_id = Value::String(s.trim().to_string());
        }
    }
    
    // Ensure numeric fields are within valid ranges
    if let Some(monthly_fee) = event_data.get_mut("monthly_fee") {
        if let Some(fee) = monthly_fee.as_u64() {
            if fee > MAX_MONTHLY_FEE {
                return Err(anyhow::anyhow!("Monthly fee {} exceeds maximum {}", fee, MAX_MONTHLY_FEE));
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_validate_created_event_success() {
        let event_data = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 1000u64,
            "auto_renew": true
        });
        
        assert!(validate_created_event(&event_data).is_ok());
    }
    
    #[test]
    fn test_validate_created_event_missing_field() {
        let event_data = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456"
            // Missing expires_at and monthly_fee
        });
        
        assert!(validate_created_event(&event_data).is_err());
    }
    
    #[test]
    fn test_validate_business_rules() {
        let valid_event = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 1000u64,
            "auto_renew": true
        });
        
        assert!(validate_business_rules(&valid_event, "ProfileSubscriptionCreatedEvent").is_ok());
        
        let invalid_event = json!({
            "service_id": "service_123",
            "subscriber": "subscriber_456",
            "expires_at": 1640995200000u64,
            "monthly_fee": 2_000_000_000u64, // Exceeds max
            "auto_renew": true
        });
        
        assert!(validate_business_rules(&invalid_event, "ProfileSubscriptionCreatedEvent").is_err());
    }
    
    #[test]
    fn test_sanitize_event_data() {
        let mut event_data = json!({
            "service_id": "  service_123  ",
            "subscriber": "  subscriber_456  ",
            "monthly_fee": 1000u64
        });
        
        assert!(sanitize_event_data(&mut event_data).is_ok());
        assert_eq!(event_data["service_id"], "service_123");
        assert_eq!(event_data["subscriber"], "subscriber_456");
    }
} 