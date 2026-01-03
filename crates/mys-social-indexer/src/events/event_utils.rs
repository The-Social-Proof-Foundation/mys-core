// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer};
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
            return Err(anyhow!(
                "Nested field {} not found at path {:?}",
                field,
                &field_path[0..=i]
            ));
        }
    }

    serde_json::from_value(current.clone()).map_err(|e| {
        anyhow!(
            "Failed to deserialize nested field at path {:?}: {}",
            field_path,
            e
        )
    })
}

/// Parse a JSON value into the specified event type
/// This function automatically handles nested structures (content.fields, fields, etc.)
/// and attempts normalization before parsing
pub fn parse_json_event<T>(value: &Value) -> Result<T>
where
    T: DeserializeOwned,
{
    // First try to normalize the event data structure
    let normalized = extract_event_fields(value)?;
    
    // Try deserializing with normalized data first
    match serde_json::from_value::<T>(normalized.clone()) {
        Ok(result) => Ok(result),
        Err(_) => {
            // If normalized data fails, try original value as fallback
            // This maintains backward compatibility for events that don't need normalization
            serde_json::from_value::<T>(value.clone())
                .map_err(|e| anyhow!("Failed to parse JSON event: {}", e))
        }
    }
}

/// Extract fields from a JSON value in standard format
pub fn extract_event_fields(data: &Value) -> Result<Value> {
    // Try to get the fields directly
    if let Some(fields) = data.get("fields") {
        return Ok(fields.clone());
    }

    // Try content.fields structure (common in blockchain Move object events)
    if let Some(content) = data.get("content") {
        if let Some(fields) = content.get("fields") {
            return Ok(fields.clone());
        }
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

/// Helper function to deserialize strings as numbers (u64)
/// Handles both string and numeric inputs from blockchain events
/// This is useful when blockchain sends numeric values as strings
pub fn deserialize_u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(u64),
    }

    match StringOrNumber::deserialize(deserializer) {
        Ok(StringOrNumber::String(s)) => {
            s.parse::<u64>().map_err(serde::de::Error::custom)
        }
        Ok(StringOrNumber::Number(n)) => Ok(n),
        Err(e) => Err(e),
    }
}

/// Helper function to deserialize strings as numbers (u8)
/// Handles both string and numeric inputs from blockchain events
/// This is useful when blockchain sends numeric values as strings
pub fn deserialize_u8_from_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(u64),
    }

    match StringOrNumber::deserialize(deserializer) {
        Ok(StringOrNumber::String(s)) => {
            s.parse::<u8>().map_err(serde::de::Error::custom)
        }
        Ok(StringOrNumber::Number(n)) => Ok(n as u8),
        Err(e) => Err(e),
    }
}

/// Helper function to deserialize optional strings as optional numbers (Option<u64>)
/// Handles both string and numeric inputs from blockchain events, including None values
pub fn deserialize_optional_u64_from_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumberOrNone {
        String(String),
        Number(u64),
        None,
    }

    match StringOrNumberOrNone::deserialize(deserializer) {
        Ok(StringOrNumberOrNone::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<u64>()
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
        }
        Ok(StringOrNumberOrNone::Number(n)) => Ok(Some(n)),
        Ok(StringOrNumberOrNone::None) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Helper function to deserialize u64 from string/number with default to 0 if missing
/// Used for fields that may not be present in older event formats
pub fn deserialize_u64_from_string_optional<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(u64),
    }

    match StringOrNumber::deserialize(deserializer) {
        Ok(StringOrNumber::String(s)) => {
            if s.is_empty() {
                Ok(0)
            } else {
                s.parse::<u64>().map_err(serde::de::Error::custom)
            }
        }
        Ok(StringOrNumber::Number(n)) => Ok(n),
        Err(_) => Ok(0),
    }
}

/// Wrapper struct for deserializing Move object events that have nested structure
/// Handles the common pattern: { "content": { "fields": { ... } } }
/// Also supports fallback to { "fields": { ... } } or direct access
#[derive(Debug)]
pub struct MoveObjectFields<T> {
    pub inner: T,
}

impl<T> MoveObjectFields<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<'de, T> Deserialize<'de> for MoveObjectFields<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        
        // First deserialize as a generic Value to inspect structure
        let value = Value::deserialize(deserializer)?;
        
        // Try to extract fields from content.fields structure
        let fields_value = if let Some(content) = value.get("content") {
            if let Some(fields) = content.get("fields") {
                Some(fields.clone())
            } else {
                None
            }
        } else if let Some(fields) = value.get("fields") {
            // Fallback to direct fields access
            Some(fields.clone())
        } else {
            // Fallback to the entire value
            Some(value.clone())
        };
        
        // Deserialize the inner type from the extracted fields
        let inner = T::deserialize(fields_value.ok_or_else(|| {
            Error::custom("Could not find fields in content.fields or fields")
        })?)
        .map_err(|e| Error::custom(format!("Failed to deserialize Move object fields: {}", e)))?;
        
        Ok(MoveObjectFields { inner })
    }
}

/// Custom deserializer for nested Move object fields (e.g., status.fields.status)
/// Handles structures like: { "fields": { "status": 0 } }
pub fn deserialize_move_object_field<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    use serde::de::Error;
    
    let value = Value::deserialize(deserializer)?;
    
    // Try to extract from fields.field_name structure
    if let Some(fields) = value.get("fields") {
        // If fields is an object, try to deserialize directly
        if fields.is_object() {
            return T::deserialize(fields.clone())
                .map_err(|e| Error::custom(format!("Failed to deserialize from fields: {}", e)));
        }
    }
    
    // Fallback to direct deserialization
    T::deserialize(value)
        .map_err(|e| Error::custom(format!("Failed to deserialize Move object field: {}", e)))
}

/// Custom deserializer for nested ID fields (e.g., id.id)
/// Handles structures like: { "id": "0x..." } or { "id": { "id": "0x..." } }
/// Also handles platform_id field directly
pub fn deserialize_nested_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    let value = Value::deserialize(deserializer)?;
    
    // Try platform_id first (for events)
    if let Some(platform_id) = value.get("platform_id") {
        if let Some(s) = platform_id.as_str() {
            return Ok(s.to_string());
        }
    }
    
    // Try nested id.id structure
    if let Some(id_obj) = value.get("id") {
        if let Some(id_str) = id_obj.get("id") {
            if let Some(s) = id_str.as_str() {
                return Ok(s.to_string());
            }
        }
        // Fallback to direct id string
        if let Some(s) = id_obj.as_str() {
            return Ok(s.to_string());
        }
    }
    
    // Try direct string access
    if let Some(s) = value.as_str() {
        return Ok(s.to_string());
    }
    
    Err(Error::custom("Could not extract ID from nested or direct structure"))
}

/// Custom deserializer for platform_id field
/// Handles both direct platform_id field and nested id.id structure
/// When deserializing from MoveObjectFields, this receives the fields object,
/// so we need to check for platform_id directly or extract from id.id
pub fn deserialize_platform_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    let value = Value::deserialize(deserializer)?;
    
    // Try platform_id first (direct field)
    if let Some(platform_id) = value.get("platform_id") {
        if let Some(s) = platform_id.as_str() {
            return Ok(s.to_string());
        }
    }
    
    // Try nested id.id structure (common in Move objects)
    if let Some(id_obj) = value.get("id") {
        if let Some(id_str) = id_obj.get("id") {
            if let Some(s) = id_str.as_str() {
                return Ok(s.to_string());
            }
        }
        // Fallback to direct id string
        if let Some(s) = id_obj.as_str() {
            return Ok(s.to_string());
        }
    }
    
    // Try direct string access
    if let Some(s) = value.as_str() {
        return Ok(s.to_string());
    }
    
    Err(Error::custom("Could not extract platform_id from nested or direct structure"))
}

/// Custom deserializer for status field in nested Move objects
/// Handles structures like: { "fields": { "status": 0 } } or direct { "status": 0 }
pub fn deserialize_status_field<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    let value = Value::deserialize(deserializer)?;
    
    // Try nested fields.status structure first
    if let Some(fields) = value.get("fields") {
        if let Some(status_val) = fields.get("status") {
            if let Some(n) = status_val.as_u64() {
                return Ok(n as u8);
            }
            if let Some(s) = status_val.as_str() {
                return s.parse::<u8>()
                    .map_err(|e| Error::custom(format!("Failed to parse status string: {}", e)));
            }
        }
    }
    
    // Try direct status field
    if let Some(status_val) = value.get("status") {
        if let Some(n) = status_val.as_u64() {
            return Ok(n as u8);
        }
        if let Some(s) = status_val.as_str() {
            return s.parse::<u8>()
                .map_err(|e| Error::custom(format!("Failed to parse status string: {}", e)));
        }
    }
    
    // Try direct number
    if let Some(n) = value.as_u64() {
        return Ok(n as u8);
    }
    
    Err(Error::custom("Could not extract status from nested or direct structure"))
}
