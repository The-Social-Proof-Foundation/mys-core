// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::debug;

/// Extracts a field from an event object
pub fn extract_field<T>(event: &Value, field_name: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    debug!("Extracting field {} from event", field_name);
    
    if let Some(field) = event.get(field_name) {
        serde_json::from_value(field.clone())
            .map_err(|e| anyhow!("Failed to deserialize field {}: {}", field_name, e))
    } else {
        Err(anyhow!("Field {} not found in event", field_name))
    }
}

/// Extracts a field from event, handling null values by returning None
pub fn extract_optional_field<T>(event: &Value, field_name: &str) -> Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    debug!("Extracting optional field {} from event", field_name);
    
    if let Some(field) = event.get(field_name) {
        if field.is_null() {
            return Ok(None);
        }
        
        serde_json::from_value(field.clone())
            .map_err(|e| anyhow!("Failed to deserialize field {}: {}", field_name, e))
            .map(Some)
    } else {
        Ok(None)
    }
}

/// Attempts to find a field in different locations within an event
pub fn find_field<T>(event: &Value, field_names: &[&str]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    for field_name in field_names {
        if let Some(field) = event.get(field_name) {
            if let Ok(value) = serde_json::from_value(field.clone()) {
                return Ok(value);
            }
        }
    }
    
    Err(anyhow!("Fields {:?} not found in event", field_names))
}

/// Extracts a nested field from an event
pub fn extract_nested_field<T>(event: &Value, field_path: &[&str]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut current = event;
    
    for (i, &field) in field_path.iter().enumerate() {
        if let Some(next) = current.get(field) {
            current = next;
        } else {
            return Err(anyhow!("Nested field {} not found at path {:?}", field, &field_path[0..=i]));
        }
    }
    
    serde_json::from_value(current.clone())
        .map_err(|e| anyhow!("Failed to deserialize nested field at path {:?}: {}", field_path, e))
}

/// Parse a JSON value into the specified event type
/// This function is useful when dealing with already-extracted JSON data
pub fn parse_json_event<T>(value: &Value) -> Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_value::<T>(value.clone())
        .map_err(|e| anyhow!("Failed to parse JSON event: {}", e))
}

/// Extract fields from a JSON value in standard format
pub fn extract_event_fields(data: &Value) -> Result<Value> {
    // Try to get the fields directly
    if let Some(fields) = data.get("fields") {
        return Ok(fields.clone());
    }
    
    // If fields are not found, try to get the data directly
    if let Some(data_fields) = data.get("data") {
        // If data has fields, return those
        if let Some(inner_fields) = data_fields.get("fields") {
            return Ok(inner_fields.clone());
        }
        
        // Otherwise, return the data itself
        return Ok(data_fields.clone());
    }
    
    // If neither approach works, return the entire value as a fallback
    Ok(data.clone())
}

/// Parse a JSON value with automatic field extraction
pub fn parse_json_event_with_fields<T>(value: &Value) -> Result<T>
where
    T: DeserializeOwned,
{
    let fields = extract_event_fields(value)?;
    parse_json_event::<T>(&fields)
} 