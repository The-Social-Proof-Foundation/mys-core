// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
// Serde json utilities

use crate::db::{Database, DbConnection};
use crate::events::event_utils;
use crate::events::platform_events::PlatformEventType;
use crate::models::platform::*;
use crate::schema;

use super::listener::BlockchainEvent;

// Helper functions for extracting fields from blockchain events
fn extract_string_field(data: &serde_json::Value, field_name: &str) -> String {
    // Try direct access
    if let Some(value) = data.get(field_name) {
        if let Some(s) = value.as_str() {
            return s.to_string();
        }
    }

    // Try content.fields.field_name structure (common in blockchain events)
    if let Some(content) = data.get("content") {
        if let Some(fields) = content.get("fields") {
            if let Some(value) = fields.get(field_name) {
                if let Some(s) = value.as_str() {
                    return s.to_string();
                }
                // Try as number (for status)
                if let Some(n) = value.as_u64() {
                    return n.to_string();
                }
            }
        }
    }

    // Try fields.field_name structure
    if let Some(fields) = data.get("fields") {
        if let Some(value) = fields.get(field_name) {
            if let Some(s) = value.as_str() {
                return s.to_string();
            }
            // Try as number (for status)
            if let Some(n) = value.as_u64() {
                return n.to_string();
            }
        }
    }

    // Try nested fields
    if field_name.contains('.') {
        let parts: Vec<&str> = field_name.split('.').collect();
        let mut current = data;

        for part in parts {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return String::new();
            }
        }

        if let Some(s) = current.as_str() {
            return s.to_string();
        }

        // Try as number (for status)
        if let Some(n) = current.as_u64() {
            return n.to_string();
        }
    }

    // Try accessing any field that might match
    for (key, value) in data.as_object().unwrap_or(&serde_json::Map::new()) {
        if key.contains(field_name) || field_name.contains(key) {
            if let Some(s) = value.as_str() {
                return s.to_string();
            }
        }
    }

    String::new()
}

fn extract_string_array(data: &serde_json::Value, field_name: &str) -> Vec<String> {
    // Try direct access
    if let Some(value) = data.get(field_name) {
        if let Some(arr) = value.as_array() {
            return arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }
    }

    // Try content.fields.field_name structure
    if let Some(content) = data.get("content") {
        if let Some(fields) = content.get("fields") {
            if let Some(value) = fields.get(field_name) {
                if let Some(arr) = value.as_array() {
                    return arr
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                }
            }
        }
    }

    // Try fields.field_name structure
    if let Some(fields) = data.get("fields") {
        if let Some(value) = fields.get(field_name) {
            if let Some(arr) = value.as_array() {
                return arr
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
        }
    }

    // Try as a single string
    let single = extract_string_field(data, field_name);
    if !single.is_empty() {
        return vec![single];
    }

    Vec::new()
}

fn extract_number_field(data: &serde_json::Value, field_name: &str) -> Option<u8> {
    // Try direct access
    if let Some(value) = data.get(field_name) {
        if let Some(n) = value.as_u64() {
            return Some(n as u8);
        }
    }

    // Try content.fields.field_name structure
    if let Some(content) = data.get("content") {
        if let Some(fields) = content.get("fields") {
            if let Some(value) = fields.get(field_name) {
                if let Some(n) = value.as_u64() {
                    return Some(n as u8);
                }
            }
        }
    }

    // Try fields.field_name structure
    if let Some(fields) = data.get("fields") {
        if let Some(value) = fields.get(field_name) {
            if let Some(n) = value.as_u64() {
                return Some(n as u8);
            }
        }
    }

    // Try nested fields
    if field_name.contains('.') {
        let parts: Vec<&str> = field_name.split('.').collect();
        let mut current = data;

        for part in parts {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return None;
            }
        }

        if let Some(n) = current.as_u64() {
            return Some(n as u8);
        }
    }

    // Try as string
    let str_val = extract_string_field(data, field_name);
    if !str_val.is_empty() {
        if let Ok(n) = str_val.parse::<u8>() {
            return Some(n);
        }
    }

    None
}

fn extract_u64_optional_field(data: &serde_json::Value, field_name: &str) -> Option<u64> {
    // Try direct access
    if let Some(value) = data.get(field_name) {
        if let Some(n) = value.as_u64() {
            return Some(n);
        }
        if let Some(s) = value.as_str() {
            if let Ok(n) = s.parse::<u64>() {
                return Some(n);
            }
        }
    }

    // Try content.fields.field_name structure
    if let Some(content) = data.get("content") {
        if let Some(fields) = content.get("fields") {
            if let Some(value) = fields.get(field_name) {
                if let Some(n) = value.as_u64() {
                    return Some(n);
                }
                if let Some(s) = value.as_str() {
                    if let Ok(n) = s.parse::<u64>() {
                        return Some(n);
                    }
                }
            }
        }
    }

    // Try fields.field_name structure
    if let Some(fields) = data.get("fields") {
        if let Some(value) = fields.get(field_name) {
            if let Some(n) = value.as_u64() {
                return Some(n);
            }
            if let Some(s) = value.as_str() {
                if let Ok(n) = s.parse::<u64>() {
                    return Some(n);
                }
            }
        }
    }

    // Try nested fields
    if field_name.contains('.') {
        let parts: Vec<&str> = field_name.split('.').collect();
        let mut current = data;

        for part in parts {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return None;
            }
        }

        if let Some(n) = current.as_u64() {
            return Some(n);
        }
        if let Some(s) = current.as_str() {
            if let Ok(n) = s.parse::<u64>() {
                return Some(n);
            }
        }
    }

    None
}

/// Normalize date format from "MM/DD/YY" or "MM/DD/YYYY" to "YYYY-MM-DD"
/// If the date is already in "YYYY-MM-DD" format, returns it unchanged
/// If parsing fails, returns the original string
fn normalize_date_format(date_str: &str) -> String {
    if date_str.is_empty() {
        return date_str.to_string();
    }

    // If already in YYYY-MM-DD format, return as-is
    if date_str.matches('-').count() == 2 {
        // Check if it matches YYYY-MM-DD pattern
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() == 3 && parts[0].len() == 4 {
            return date_str.to_string();
        }
    }

    // Try to parse MM/DD/YY or MM/DD/YYYY format
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() == 3 {
        if let (Ok(month), Ok(day), year_str) = (
            parts[0].parse::<u32>(),
            parts[1].parse::<u32>(),
            parts[2],
        ) {
            // Parse year
            let year = if year_str.len() == 2 {
                // 2-digit year: assume 20YY for years < 50, 19YY for years >= 50
                if let Ok(yy) = year_str.parse::<u32>() {
                    if yy < 50 {
                        2000 + yy
                    } else {
                        1900 + yy
                    }
                } else {
                    return date_str.to_string();
                }
            } else if year_str.len() == 4 {
                // 4-digit year
                year_str.parse::<u32>().unwrap_or(0)
            } else {
                return date_str.to_string();
            };

            // Validate month and day
            if month >= 1 && month <= 12 && day >= 1 && day <= 31 && year > 0 {
                return format!("{:04}-{:02}-{:02}", year, month, day);
            }
        }
    }

    // If parsing failed, return original string
    date_str.to_string()
}

fn extract_bool_optional_field(data: &serde_json::Value, field_name: &str) -> Option<bool> {
    // Try direct access
    if let Some(value) = data.get(field_name) {
        if let Some(b) = value.as_bool() {
            return Some(b);
        }
        if let Some(s) = value.as_str() {
            if let Ok(b) = s.parse::<bool>() {
                return Some(b);
            }
        }
    }

    // Try content.fields.field_name structure
    if let Some(content) = data.get("content") {
        if let Some(fields) = content.get("fields") {
            if let Some(value) = fields.get(field_name) {
                if let Some(b) = value.as_bool() {
                    return Some(b);
                }
                if let Some(s) = value.as_str() {
                    if let Ok(b) = s.parse::<bool>() {
                        return Some(b);
                    }
                }
            }
        }
    }

    // Try fields.field_name structure
    if let Some(fields) = data.get("fields") {
        if let Some(value) = fields.get(field_name) {
            if let Some(b) = value.as_bool() {
                return Some(b);
            }
            if let Some(s) = value.as_str() {
                if let Ok(b) = s.parse::<bool>() {
                    return Some(b);
                }
            }
        }
    }

    // Try nested fields
    if field_name.contains('.') {
        let parts: Vec<&str> = field_name.split('.').collect();
        let mut current = data;

        for part in parts {
            if let Some(next) = current.get(part) {
                current = next;
            } else {
                return None;
            }
        }

        if let Some(b) = current.as_bool() {
            return Some(b);
        }
        if let Some(s) = current.as_str() {
            if let Ok(b) = s.parse::<bool>() {
                return Some(b);
            }
        }
    }

    None
}

/// Handler for platform-related blockchain events
pub struct PlatformEventHandler {
    /// Database connection
    db: Arc<Database>,
    /// Event receiver channel
    rx: mpsc::Receiver<BlockchainEvent>,
}

impl PlatformEventHandler {
    /// Create a new platform event handler
    pub fn new(db: Arc<Database>, rx: mpsc::Receiver<BlockchainEvent>, _worker_id: String) -> Self {
        Self { db, rx }
    }

    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    /// Process a platform created event
    async fn process_platform_created_event(
        &self,
        event: &PlatformCreatedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing platform created event");

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::PlatformCreated.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Check if platform already exists
                    let platform_exists = schema::platforms::table
                        .filter(schema::platforms::platform_id.eq(&event.platform_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if platform_exists {
                        debug!("Platform already exists: {}", event.platform_id);
                        // Update existing platform with new data if needed
                        let platform_update = UpdatePlatform {
                            name: Some(event.name.clone()),
                            tagline: Some(event.tagline.clone()),
                            description: event.description.clone(), // Use the description from the event
                            logo: event.logo.clone(),               // Use the logo from the event
                            terms_of_service: Some(event.terms_of_service.clone()),
                            privacy_policy: Some(event.privacy_policy.clone()),
                            platform_names: Some(
                                serde_json::to_value(&event.platforms).unwrap_or_default(),
                            ),
                            links: Some(serde_json::to_value(&event.links).unwrap_or_default()),
                            status: Some(event.status.status as i16),
                            release_date: Some(event.release_date.clone()),
                            shutdown_date: event.shutdown_date.clone(),
                            updated_at: Some(
                                chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                    .unwrap_or_else(|| chrono::Utc::now())
                                    .naive_utc(),
                            ),
                            is_approved: None, // Don't change approval status on update
                            approval_changed_at: None, // Don't change approval timestamp
                            approved_by: None, // Don't change approver
                            wants_dao_governance: event.wants_dao_governance,
                            governance_registry_id: event.governance_registry_id.clone(),
                            delegate_count: event.delegate_count.map(|v| v as i64),
                            delegate_term_epochs: event.delegate_term_epochs.map(|v| v as i64),
                            max_votes_per_user: event.max_votes_per_user.map(|v| v as i64),
                            min_on_chain_age_days: event.min_on_chain_age_days.map(|v| v as i64),
                            proposal_submission_cost: event.proposal_submission_cost.map(|v| v as i64),
                            quadratic_base_cost: event.quadratic_base_cost.map(|v| v as i64),
                            quorum_votes: event.quorum_votes.map(|v| v as i64),
                            voting_period_epochs: event.voting_period_epochs.map(|v| v as i64),
                            treasury: event.treasury.map(|v| v as i64),
                            version: event.version.map(|v| v as i64),
                        };

                        diesel::update(schema::platforms::table)
                            .filter(schema::platforms::platform_id.eq(&event.platform_id))
                            .set(&platform_update)
                            .execute(&mut conn)
                            .await?;

                        info!("Updated existing platform: {}", event.platform_id);
                    } else {
                        // Create new platform
                        let new_platform = NewPlatform {
                            platform_id: event.platform_id.clone(),
                            name: event.name.clone(),
                            tagline: event.tagline.clone(),
                            description: event.description.clone(), // Use the description from the event
                            logo: event.logo.clone(),               // Use the logo from the event
                            developer_address: event.developer.clone(),
                            terms_of_service: Some(event.terms_of_service.clone()),
                            privacy_policy: Some(event.privacy_policy.clone()),
                            platform_names: Some(
                                serde_json::to_value(&event.platforms).unwrap_or_default(),
                            ),
                            links: Some(serde_json::to_value(&event.links).unwrap_or_default()),
                            status: event.status.status as i16,
                            release_date: Some(event.release_date.clone()),
                            shutdown_date: event.shutdown_date.clone(),
                            created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                            updated_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                            is_approved: false, // New platforms are not approved by default
                            approval_changed_at: None, // No approval change yet
                            approved_by: None,  // No approver yet
                            wants_dao_governance: event.wants_dao_governance,
                            governance_registry_id: event.governance_registry_id.clone(),
                            delegate_count: event.delegate_count.map(|v| v as i64),
                            delegate_term_epochs: event.delegate_term_epochs.map(|v| v as i64),
                            max_votes_per_user: event.max_votes_per_user.map(|v| v as i64),
                            min_on_chain_age_days: event.min_on_chain_age_days.map(|v| v as i64),
                            proposal_submission_cost: event.proposal_submission_cost.map(|v| v as i64),
                            quadratic_base_cost: event.quadratic_base_cost.map(|v| v as i64),
                            quorum_votes: event.quorum_votes.map(|v| v as i64),
                            voting_period_epochs: event.voting_period_epochs.map(|v| v as i64),
                            treasury: event.treasury.map(|v| v as i64),
                            version: event.version.map(|v| v as i64),
                        };

                        // Insert platform
                        diesel::insert_into(schema::platforms::table)
                            .values(&new_platform)
                            .execute(&mut conn)
                            .await?;

                        // Add developer as a moderator
                        let new_moderator = NewPlatformModerator {
                            platform_id: event.platform_id.clone(),
                            moderator_address: event.developer.clone(),
                            added_by: event.developer.clone(), // Developer adds themselves
                            created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                        };

                        // Insert developer as moderator
                        diesel::insert_into(schema::platform_moderators::table)
                            .values(&new_moderator)
                            .on_conflict((
                                schema::platform_moderators::platform_id,
                                schema::platform_moderators::moderator_address,
                            ))
                            .do_nothing() // If already exists, do nothing
                            .execute(&mut conn)
                            .await?;

                        info!("Created new platform: {}", event.platform_id);
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed platform created event");

        Ok(())
    }

    /// Process a platform updated event
    async fn process_platform_updated_event(
        &self,
        event: &PlatformUpdatedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing platform updated event");

        let mut conn = self.get_connection().await?;

        // Extract timestamp from blockchain event before moving into closure
        let event_timestamp_ms = blockchain_event.map(|e| e.timestamp_ms);

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::PlatformUpdated.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Check if platform exists
                    let platform_exists = schema::platforms::table
                        .filter(schema::platforms::platform_id.eq(&event.platform_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if platform_exists {
                        // Use blockchain event timestamp if available, otherwise use current time
                        // The event.updated_at field is not a real timestamp but an epoch/sequence number
                        let updated_at = if let Some(timestamp_ms) = event_timestamp_ms {
                            // Convert milliseconds to seconds for from_timestamp
                            chrono::DateTime::from_timestamp(
                                (timestamp_ms / 1000) as i64,
                                0,
                            )
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc()
                        } else {
                            // Fallback to current time if no blockchain event timestamp
                            chrono::Utc::now().naive_utc()
                        };

                        // Update existing platform
                        let platform_update = UpdatePlatform {
                            name: Some(event.name.clone()),
                            tagline: Some(event.tagline.clone()),
                            description: Some(event.description.clone()),
                            logo: None, // Not in updated event
                            terms_of_service: Some(event.terms_of_service.clone()),
                            privacy_policy: Some(event.privacy_policy.clone()),
                            platform_names: Some(
                                serde_json::to_value(&event.platforms).unwrap_or_default(),
                            ),
                            links: Some(serde_json::to_value(&event.links).unwrap_or_default()),
                            status: Some(event.status.status as i16),
                            release_date: Some(normalize_date_format(&event.release_date)),
                            shutdown_date: event.shutdown_date.clone().map(|d| Some(normalize_date_format(&d))).unwrap_or(None),
                            updated_at: Some(updated_at),
                            is_approved: None, // Don't change approval status on regular update
                            approval_changed_at: None, // Don't change approval timestamp
                            approved_by: None, // Don't change approver
                            wants_dao_governance: None, // Not in update event
                            governance_registry_id: None, // Not in update event
                            delegate_count: None, // Not in update event
                            delegate_term_epochs: None, // Not in update event
                            max_votes_per_user: None, // Not in update event
                            min_on_chain_age_days: None, // Not in update event
                            proposal_submission_cost: None, // Not in update event
                            quadratic_base_cost: None, // Not in update event
                            quorum_votes: None, // Not in update event
                            voting_period_epochs: None, // Not in update event
                            treasury: None, // Not in update event
                            version: None, // Not in update event
                        };

                        diesel::update(schema::platforms::table)
                            .filter(schema::platforms::platform_id.eq(&event.platform_id))
                            .set(&platform_update)
                            .execute(&mut conn)
                            .await?;

                        info!("Updated platform: {}", event.platform_id);
                    } else {
                        // Platform doesn't exist, this is unusual but we'll create it
                        warn!(
                            "Platform update for non-existent platform: {}",
                            event.platform_id
                        );

                        // Create platform with limited information from update event
                        // (we don't have developer info in the update event)
                        let new_platform = NewPlatform {
                            platform_id: event.platform_id.clone(),
                            name: event.name.clone(),
                            tagline: event.tagline.clone(),
                            description: Some(event.description.clone()),
                            logo: None,
                            developer_address: "unknown".to_string(), // We don't have this info
                            terms_of_service: Some(event.terms_of_service.clone()),
                            privacy_policy: Some(event.privacy_policy.clone()),
                            platform_names: Some(
                                serde_json::to_value(&event.platforms).unwrap_or_default(),
                            ),
                            links: Some(serde_json::to_value(&event.links).unwrap_or_default()),
                            status: event.status.status as i16,
                            release_date: Some(normalize_date_format(&event.release_date)),
                            shutdown_date: event.shutdown_date.clone().map(|d| Some(normalize_date_format(&d))).unwrap_or(None),
                            created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                            updated_at: {
                                // Use blockchain event timestamp if available, otherwise use current time
                                if let Some(timestamp_ms) = event_timestamp_ms {
                                    chrono::DateTime::from_timestamp(
                                        (timestamp_ms / 1000) as i64,
                                        0,
                                    )
                                    .unwrap_or_else(|| chrono::Utc::now())
                                    .naive_utc()
                                } else {
                                    chrono::Utc::now().naive_utc()
                                }
                            },
                            is_approved: false, // New platforms are not approved by default
                            approval_changed_at: None, // No approval change yet
                            approved_by: None,  // No approver yet
                            wants_dao_governance: None, // Not in update event
                            governance_registry_id: None, // Not in update event
                            delegate_count: None, // Not in update event
                            delegate_term_epochs: None, // Not in update event
                            max_votes_per_user: None, // Not in update event
                            min_on_chain_age_days: None, // Not in update event
                            proposal_submission_cost: None, // Not in update event
                            quadratic_base_cost: None, // Not in update event
                            quorum_votes: None, // Not in update event
                            voting_period_epochs: None, // Not in update event
                            treasury: None, // Not in update event
                            version: None, // Not in update event
                        };

                        diesel::insert_into(schema::platforms::table)
                            .values(&new_platform)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Created missing platform from update event: {}",
                            event.platform_id
                        );
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed platform updated event");

        Ok(())
    }

    /// Process a moderator added event
    async fn process_moderator_added_event(
        &self,
        event: &ModeratorAddedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing moderator added event");

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::ModeratorAdded.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Check if platform exists
                    let platform_exists = schema::platforms::table
                        .filter(schema::platforms::platform_id.eq(&event.platform_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if !platform_exists {
                        // Create a placeholder platform if it doesn't exist
                        warn!(
                            "Moderator added for non-existent platform: {}",
                            event.platform_id
                        );

                        let new_platform = NewPlatform {
                            platform_id: event.platform_id.clone(),
                            name: format!("Unknown Platform ({})", event.platform_id),
                            tagline: "Platform metadata not available".to_string(),
                            description: None,
                            logo: None,
                            developer_address: event.added_by.clone(), // Assume the adder is the developer
                            terms_of_service: None,
                            privacy_policy: None,
                            platform_names: None,
                            links: None,
                            status: PLATFORM_STATUS_DEVELOPMENT, // Default to development status
                            release_date: None,
                            shutdown_date: None,
                            created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                            updated_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                            is_approved: false, // New platforms are not approved by default
                            approval_changed_at: None, // No approval change yet
                            approved_by: None,  // No approver yet
                            wants_dao_governance: None, // Not available for placeholder
                            governance_registry_id: None, // Not available for placeholder
                            delegate_count: None, // Not available for placeholder
                            delegate_term_epochs: None, // Not available for placeholder
                            max_votes_per_user: None, // Not available for placeholder
                            min_on_chain_age_days: None, // Not available for placeholder
                            proposal_submission_cost: None, // Not available for placeholder
                            quadratic_base_cost: None, // Not available for placeholder
                            quorum_votes: None, // Not available for placeholder
                            voting_period_epochs: None, // Not available for placeholder
                            treasury: None, // Not available for placeholder
                            version: None, // Not available for placeholder
                        };

                        diesel::insert_into(schema::platforms::table)
                            .values(&new_platform)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Created placeholder platform for moderator: {}",
                            event.platform_id
                        );
                    }

                    // Add moderator to platform
                    let new_moderator = NewPlatformModerator {
                        platform_id: event.platform_id.clone(),
                        moderator_address: event.moderator_address.clone(),
                        added_by: event.added_by.clone(),
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                    };

                    // Insert moderator with conflict handling
                    diesel::insert_into(schema::platform_moderators::table)
                        .values(&new_moderator)
                        .on_conflict((
                            schema::platform_moderators::platform_id,
                            schema::platform_moderators::moderator_address,
                        ))
                        .do_nothing() // If already exists, do nothing
                        .execute(&mut conn)
                        .await?;

                    info!(
                        "Added moderator {} to platform {}",
                        event.moderator_address, event.platform_id
                    );

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        // Write to relay outbox for notifications - notify the moderator (outside transaction)
        let mut outbox_conn = self.get_connection().await?;
        let event_data = serde_json::json!({
            "platform_id": event.platform_id,
            "moderator_address": event.moderator_address,
            "added_by": event.added_by,
        });
        if let Err(e) = crate::relay_outbox::write_notification_event(
            &mut outbox_conn,
            "platform.moderator_added",
            &event_data,
            blockchain_event.map(|e| e.event_id.as_str()),
            blockchain_event.map(|e| e.tx_digest.as_str()),
        )
        .await
        {
            warn!("Failed to write moderator added event to outbox: {}", e);
        }

        info!("Successfully processed moderator added event");

        Ok(())
    }

    /// Process a moderator removed event
    async fn process_moderator_removed_event(
        &self,
        event: &ModeratorRemovedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing moderator removed event");

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::ModeratorRemoved.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Remove moderator from platform
                    diesel::delete(
                        schema::platform_moderators::table
                            .filter(schema::platform_moderators::platform_id.eq(&event.platform_id))
                            .filter(
                                schema::platform_moderators::moderator_address
                                    .eq(&event.moderator_address),
                            ),
                    )
                    .execute(&mut conn)
                    .await?;

                    info!(
                        "Removed moderator {} from platform {}",
                        event.moderator_address, event.platform_id
                    );

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        // Write to relay outbox for notifications - notify the moderator (outside transaction)
        let mut outbox_conn = self.get_connection().await?;
        let event_data = serde_json::json!({
            "platform_id": event.platform_id,
            "moderator_address": event.moderator_address,
            "removed_by": event.removed_by,
        });
        if let Err(e) = crate::relay_outbox::write_notification_event(
            &mut outbox_conn,
            "platform.moderator_removed",
            &event_data,
            blockchain_event.map(|e| e.event_id.as_str()),
            blockchain_event.map(|e| e.tx_digest.as_str()),
        )
        .await
        {
            warn!("Failed to write moderator removed event to outbox: {}", e);
        }

        info!("Successfully processed moderator removed event");

        Ok(())
    }

    /// Process a profile blocked event
    async fn process_profile_blocked_event(
        &self,
        event: &PlatformBlockedProfileEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing profile blocked event");

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::ProfileBlocked.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Check if this platform-profile relationship already exists
                    let existing_relationship = schema::platform_blocked_profiles::table
                        .filter(
                            schema::platform_blocked_profiles::platform_id.eq(&event.platform_id),
                        )
                        .filter(schema::platform_blocked_profiles::profile_id.eq(&event.profile_id))
                        .first::<PlatformBlockedProfile>(&mut conn)
                        .await;

                    match existing_relationship {
                        Ok(_) => {
                            // Delete the existing record - we'll insert a new one to reset the timestamps
                            diesel::delete(schema::platform_blocked_profiles::table)
                                .filter(
                                    schema::platform_blocked_profiles::platform_id
                                        .eq(&event.platform_id),
                                )
                                .filter(
                                    schema::platform_blocked_profiles::profile_id
                                        .eq(&event.profile_id),
                                )
                                .execute(&mut conn)
                                .await?;

                            info!("Deleted existing block relationship to refresh timestamp");

                            // Create new blocked profile relationship below
                        }
                        Err(diesel::result::Error::NotFound) => {
                            // No existing relationship - we'll create a new one
                        }
                        Err(e) => {
                            error!("Error checking for existing block relationship: {}", e);
                            return Err(e);
                        }
                    }

                    // Create new blocked profile relationship
                    let new_blocked_profile = (
                        schema::platform_blocked_profiles::platform_id
                            .eq(event.platform_id.clone()),
                        schema::platform_blocked_profiles::profile_id.eq(event.profile_id.clone()),
                        schema::platform_blocked_profiles::blocked_by.eq(event.blocked_by.clone()),
                        schema::platform_blocked_profiles::created_at.eq(
                            chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                        ),
                    );

                    diesel::insert_into(schema::platform_blocked_profiles::table)
                        .values(new_blocked_profile)
                        .execute(&mut conn)
                        .await?;

                    info!(
                        "Created new blocked profile relationship: {} on platform {}",
                        event.profile_id, event.platform_id
                    );

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed profile blocked event");

        Ok(())
    }

    /// Process a profile unblocked event
    async fn process_profile_unblocked_event(
        &self,
        event: &PlatformUnblockedProfileEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing profile unblocked event");

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::ProfileUnblocked.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Delete the block relationship entirely instead of updating it
                    let deleted_count = diesel::delete(schema::platform_blocked_profiles::table)
                        .filter(
                            schema::platform_blocked_profiles::platform_id.eq(&event.platform_id),
                        )
                        .filter(schema::platform_blocked_profiles::profile_id.eq(&event.profile_id))
                        .execute(&mut conn)
                        .await?;

                    if deleted_count > 0 {
                        info!(
                            "Deleted block relationship: {} on platform {}",
                            event.profile_id, event.platform_id
                        );
                    } else {
                        warn!(
                            "No block relationship found to delete: {} on platform {}",
                            event.profile_id, event.platform_id
                        );
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed profile unblocked event");

        Ok(())
    }

    /// Process a platform approval changed event
    async fn process_platform_approval_changed_event(
        &self,
        event: &PlatformApprovalChangedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!(
            "Processing platform approval changed event for platform: {}",
            event.platform_id
        );

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::PlatformApprovalChanged
                            .to_str()
                            .to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: event.reasoning.clone(),
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Check if platform exists
                    let platform_exists = schema::platforms::table
                        .filter(schema::platforms::platform_id.eq(&event.platform_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if platform_exists {
                        // Get timestamp from event
                        let approval_changed_at =
                            chrono::DateTime::from_timestamp(event.changed_at as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc();

                        // Update platform approval status
                        let platform_update = UpdatePlatform {
                            name: None,
                            tagline: None,
                            description: None,
                            logo: None,
                            terms_of_service: None,
                            privacy_policy: None,
                            platform_names: None,
                            links: None,
                            status: None,
                            release_date: None,
                            shutdown_date: None,
                            updated_at: Some(approval_changed_at),
                            is_approved: Some(event.is_approved),
                            approval_changed_at: Some(approval_changed_at),
                            approved_by: Some(event.approved_by.clone()),
                            wants_dao_governance: None, // Don't change governance fields
                            governance_registry_id: None, // Don't change governance fields
                            delegate_count: None, // Don't change governance fields
                            delegate_term_epochs: None, // Don't change governance fields
                            max_votes_per_user: None, // Don't change governance fields
                            min_on_chain_age_days: None, // Don't change governance fields
                            proposal_submission_cost: None, // Don't change governance fields
                            quadratic_base_cost: None, // Don't change governance fields
                            quorum_votes: None, // Don't change governance fields
                            voting_period_epochs: None, // Don't change governance fields
                            treasury: None, // Don't change governance fields
                            version: None, // Don't change governance fields
                        };

                        diesel::update(schema::platforms::table)
                            .filter(schema::platforms::platform_id.eq(&event.platform_id))
                            .set(&platform_update)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Updated platform approval status: platform_id={}, is_approved={}",
                            event.platform_id, event.is_approved
                        );
                    } else {
                        warn!(
                            "Platform not found for approval change: {}",
                            event.platform_id
                        );
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed platform approval changed event");

        Ok(())
    }

    /// Process a user joined platform event
    async fn process_user_joined_platform_event(
        &self,
        event: &UserJoinedPlatformEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing user joined platform event");

        let mut conn = self.get_connection().await?;

        // Extract timestamp from blockchain event before moving into closure
        // The event.timestamp field is not a real timestamp but an epoch/sequence number
        let event_timestamp_ms = blockchain_event.map(|e| e.timestamp_ms);

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::UserJoinedPlatform.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Check if the platform is approved - only approved platforms can be joined
                    let platform_is_approved = schema::platforms::table
                        .filter(schema::platforms::platform_id.eq(&event.platform_id))
                        .select(schema::platforms::is_approved)
                        .first::<bool>(&mut conn)
                        .await
                        .unwrap_or(false);

                    if !platform_is_approved {
                        warn!(
                            "Ignoring join event for non-approved platform: {}",
                            event.platform_id
                        );
                        return Ok(());
                    }

                    // Check if the profile is blocked by the platform
                    let profile_is_blocked = schema::platform_blocked_profiles::table
                        .filter(
                            schema::platform_blocked_profiles::platform_id.eq(&event.platform_id),
                        )
                        .filter(schema::platform_blocked_profiles::profile_id.eq(&event.profile_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if profile_is_blocked {
                        warn!(
                            "Ignoring join event for blocked profile: {} in platform {}",
                            event.profile_id, event.platform_id
                        );
                        return Ok(());
                    }

                    // Check if membership already exists
                    let membership_exists = schema::platform_memberships::table
                        .filter(schema::platform_memberships::platform_id.eq(&event.platform_id))
                        .filter(schema::platform_memberships::profile_id.eq(&event.profile_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if !membership_exists {
                        // Use current time for joined_at, matching platform creation style
                        let joined_at = chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc();

                        // Create new membership
                        let new_membership = NewPlatformMembership {
                            platform_id: event.platform_id.clone(),
                            profile_id: event.profile_id.clone(),
                            joined_at,
                        };

                        // Insert membership
                        diesel::insert_into(schema::platform_memberships::table)
                            .values(new_membership)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Created new platform membership: {} -> {}",
                            event.profile_id, event.platform_id
                        );

                        // Also create a profile event for this action to track in profile history
                        // Use blockchain event timestamp in milliseconds, or current time as fallback
                        let profile_event_timestamp = if let Some(timestamp_ms) = event_timestamp_ms {
                            timestamp_ms
                        } else {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64
                        };

                        let platform_join_event =
                            crate::events::profile_event_types::PlatformJoinedEvent {
                                profile_id: event.profile_id.clone(),
                                platform_id: event.platform_id.clone(),
                                timestamp: profile_event_timestamp,
                            };

                        // We need to get the event ID again since it was moved in the platform_event
                        let event_id_for_profile = blockchain_event.map(|e| e.event_id.clone());

                        let profile_event =
                            crate::models::profile_events::NewProfileEvent::from_platform_joined(
                                &platform_join_event,
                                event_id_for_profile,
                            );

                        // Insert into profile events table
                        diesel::insert_into(schema::profile_events::table)
                            .values(&profile_event)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Created profile event for platform join: {} -> {}",
                            event.profile_id, event.platform_id
                        );
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        // Write to relay outbox for notifications - notify platform moderators/owners (outside transaction)
        // Note: We could also notify the user who joined, but typically platform events
        // are more relevant to platform admins
        let mut outbox_conn = self.get_connection().await?;
        let event_data = serde_json::json!({
            "platform_id": event.platform_id,
            "profile_id": event.profile_id,
        });
        if let Err(e) = crate::relay_outbox::write_notification_event(
            &mut outbox_conn,
            "platform.user_joined",
            &event_data,
            blockchain_event.map(|e| e.event_id.as_str()),
            blockchain_event.map(|e| e.tx_digest.as_str()),
        )
        .await
        {
            warn!("Failed to write user joined event to outbox: {}", e);
        }

        info!("Successfully processed user joined platform event");

        Ok(())
    }

    /// Process a user left platform event
    async fn process_user_left_platform_event(
        &self,
        event: &UserLeftPlatformEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing user left platform event");

        let mut conn = self.get_connection().await?;

        // Extract timestamp from blockchain event before moving into closure
        // The event.timestamp field is not a real timestamp but an epoch/sequence number
        let event_timestamp_ms = blockchain_event.map(|e| e.timestamp_ms);

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::UserLeftPlatform.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Update existing membership if it exists
                    let membership_exists = schema::platform_memberships::table
                        .filter(schema::platform_memberships::platform_id.eq(&event.platform_id))
                        .filter(schema::platform_memberships::profile_id.eq(&event.profile_id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if membership_exists {
                        // Delete the membership record
                        diesel::delete(schema::platform_memberships::table)
                            .filter(
                                schema::platform_memberships::platform_id.eq(&event.platform_id),
                            )
                            .filter(schema::platform_memberships::profile_id.eq(&event.profile_id))
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Deleted platform membership for user leaving: {} -> {}",
                            event.profile_id, event.platform_id
                        );

                        // Also create a profile event for this action to track in profile history
                        // Use blockchain event timestamp in milliseconds, or current time as fallback
                        let profile_event_timestamp = if let Some(timestamp_ms) = event_timestamp_ms {
                            timestamp_ms
                        } else {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64
                        };

                        let platform_left_event =
                            crate::events::profile_event_types::PlatformLeftEvent {
                                profile_id: event.profile_id.clone(),
                                platform_id: event.platform_id.clone(),
                                timestamp: profile_event_timestamp,
                            };

                        // We need to get the event ID again since it was moved in the platform_event
                        let event_id_for_profile = blockchain_event.map(|e| e.event_id.clone());

                        let profile_event =
                            crate::models::profile_events::NewProfileEvent::from_platform_left(
                                &platform_left_event,
                                event_id_for_profile,
                            );

                        // Insert into profile events table
                        diesel::insert_into(schema::profile_events::table)
                            .values(&profile_event)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Created profile event for platform leave: {} -> {}",
                            event.profile_id, event.platform_id
                        );
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        // Write to relay outbox for notifications - notify platform moderators/owners (outside transaction)
        let mut outbox_conn = self.get_connection().await?;
        let event_data = serde_json::json!({
            "platform_id": event.platform_id,
            "profile_id": event.profile_id,
        });
        if let Err(e) = crate::relay_outbox::write_notification_event(
            &mut outbox_conn,
            "platform.user_left",
            &event_data,
            blockchain_event.map(|e| e.event_id.as_str()),
            blockchain_event.map(|e| e.tx_digest.as_str()),
        )
        .await
        {
            warn!("Failed to write user left event to outbox: {}", e);
        }

        info!("Successfully processed user left platform event");

        Ok(())
    }

    /// Process a treasury funded event
    async fn process_treasury_funded_event(
        &self,
        event: &TreasuryFundedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing treasury funded event");

        let mut conn = self.get_connection().await?;

        // Start a transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Store event for historical record
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();

                    // Get event_id from blockchain_event if available
                    let event_id = blockchain_event.map(|e| e.event_id.clone());

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::TreasuryFunded.to_str().to_string(),
                        platform_id: event.platform_id.clone(),
                        event_data: serde_json::to_value(event).unwrap_or_default(),
                        event_id,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        reasoning: None,
                    };

                    // Insert platform event
                    diesel::insert_into(schema::platform_events::table)
                        .values(&platform_event)
                        .execute(&mut conn)
                        .await?;

                    // Update platform treasury balance
                    let platform_update = UpdatePlatform {
                        name: None,
                        tagline: None,
                        description: None,
                        logo: None,
                        terms_of_service: None,
                        privacy_policy: None,
                        platform_names: None,
                        links: None,
                        status: None,
                        release_date: None,
                        shutdown_date: None,
                        updated_at: Some(
                            chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                .unwrap_or_else(|| chrono::Utc::now())
                                .naive_utc(),
                        ),
                        is_approved: None,
                        approval_changed_at: None,
                        approved_by: None,
                        wants_dao_governance: None,
                        governance_registry_id: None,
                        delegate_count: None,
                        delegate_term_epochs: None,
                        max_votes_per_user: None,
                        min_on_chain_age_days: None,
                        proposal_submission_cost: None,
                        quadratic_base_cost: None,
                        quorum_votes: None,
                        voting_period_epochs: None,
                        treasury: Some(event.new_balance as i64),
                        version: None,
                    };

                    diesel::update(schema::platforms::table)
                        .filter(schema::platforms::platform_id.eq(&event.platform_id))
                        .set(&platform_update)
                        .execute(&mut conn)
                        .await?;

                    info!(
                        "Updated platform treasury: platform_id={}, new_balance={}",
                        event.platform_id, event.new_balance
                    );

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed treasury funded event");

        Ok(())
    }

    /// Process raw blockchain events
    async fn process_event(&self, event: BlockchainEvent) -> Result<()> {
        debug!("Platform handler examining event: {}", event.event_type);

        // Log the raw event data for debugging
        info!("Platform handler received event: {}", event.event_type);
        info!(
            "Event data: {}",
            serde_json::to_string_pretty(&event.data).unwrap_or_default()
        );

        // Use the PlatformEventType from_str method which handles package prefixes
        if let Some(event_type) =
            crate::events::platform_events::PlatformEventType::from_str(&event.event_type)
        {
            info!("Identified platform event type: {:?}", event_type);

            match event_type {
                PlatformEventType::PlatformCreated => {
                    info!("Processing PlatformCreated event");
                    // Log complete event data for debugging
                    info!(
                        "PlatformCreated event data: {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );

                    // First try to normalize the event data structure
                    let normalized_data = match event_utils::extract_event_fields(&event.data) {
                        Ok(fields) => {
                            debug!("Extracted fields from event data");
                            fields
                        }
                        Err(_) => {
                            debug!("Could not extract fields, using raw event data");
                            event.data.clone()
                        }
                    };

                    // Try deserialization with normalized data first
                    match serde_json::from_value::<PlatformCreatedEvent>(normalized_data.clone()) {
                        Ok(platform_event) => {
                            info!("Successfully deserialized PlatformCreatedEvent");
                            self.process_platform_created_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            warn!("Failed to deserialize PlatformCreatedEvent normally: {}", e);

                            // Try to extract fields manually if normal deserialization fails
                            // Use normalized_data if available, otherwise fall back to original
                            let data_to_extract = if normalized_data != event.data {
                                &normalized_data
                            } else {
                                &event.data
                            };
                            let mut platform_event = PlatformCreatedEvent {
                                platform_id: extract_string_field(data_to_extract, "platform_id"),
                                name: extract_string_field(data_to_extract, "name"),
                                tagline: extract_string_field(data_to_extract, "tagline"),
                                description: {
                                    // Simple description extraction
                                    let desc = extract_string_field(data_to_extract, "description");
                                    if !desc.is_empty() {
                                        Some(desc)
                                    } else {
                                        None
                                    }
                                },
                                developer: extract_string_field(data_to_extract, "developer"),
                                logo: {
                                    // Simple logo extraction
                                    let logo = extract_string_field(data_to_extract, "logo");
                                    if !logo.is_empty() {
                                        Some(logo)
                                    } else {
                                        None
                                    }
                                },
                                terms_of_service: extract_string_field(
                                    data_to_extract,
                                    "terms_of_service",
                                ),
                                privacy_policy: extract_string_field(data_to_extract, "privacy_policy"),
                                platforms: extract_string_array(data_to_extract, "platforms"),
                                links: extract_string_array(data_to_extract, "links"),
                                status: PlatformStatus {
                                    status: extract_number_field(data_to_extract, "status.status")
                                        .unwrap_or(0),
                                },
                                release_date: extract_string_field(data_to_extract, "release_date"),
                                shutdown_date: {
                                    let shutdown = extract_string_field(data_to_extract, "shutdown_date");
                                    if !shutdown.is_empty() {
                                        Some(shutdown)
                                    } else {
                                        None
                                    }
                                },
                                wants_dao_governance: extract_bool_optional_field(data_to_extract, "wants_dao_governance"),
                                governance_registry_id: {
                                    let reg_id = extract_string_field(data_to_extract, "governance_registry_id");
                                    if !reg_id.is_empty() {
                                        Some(reg_id)
                                    } else {
                                        None
                                    }
                                },
                                delegate_count: extract_u64_optional_field(data_to_extract, "delegate_count"),
                                delegate_term_epochs: extract_u64_optional_field(data_to_extract, "delegate_term_epochs"),
                                max_votes_per_user: extract_u64_optional_field(data_to_extract, "max_votes_per_user"),
                                min_on_chain_age_days: extract_u64_optional_field(data_to_extract, "min_on_chain_age_days"),
                                proposal_submission_cost: extract_u64_optional_field(data_to_extract, "proposal_submission_cost"),
                                quadratic_base_cost: extract_u64_optional_field(data_to_extract, "quadratic_base_cost"),
                                quorum_votes: extract_u64_optional_field(data_to_extract, "quorum_votes"),
                                voting_period_epochs: extract_u64_optional_field(data_to_extract, "voting_period_epochs"),
                                treasury: extract_u64_optional_field(data_to_extract, "treasury"),
                                version: extract_u64_optional_field(data_to_extract, "version"),
                            };

                            // If platform_id is empty, try other formats
                            if platform_event.platform_id.is_empty() {
                                platform_event.platform_id = data_to_extract
                                    .get("platform_id")
                                    .and_then(|v| v.as_str())
                                    .map(String::from)
                                    .unwrap_or_else(|| {
                                        // Try in original event.data as fallback
                                        event.data
                                            .get("platform_id")
                                            .and_then(|v| v.as_str())
                                            .map(String::from)
                                            .unwrap_or_default()
                                    });
                            }

                            info!("Manually extracted platform event: {:?}", platform_event);
                            self.process_platform_created_event(&platform_event, Some(&event))
                                .await?;
                        }
                    }
                }
                PlatformEventType::PlatformUpdated => {
                    info!("Processing PlatformUpdated event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<PlatformUpdatedEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize PlatformUpdatedEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_platform_updated_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformUpdatedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::ModeratorAdded => {
                    info!("Processing ModeratorAdded event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<ModeratorAddedEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize ModeratorAddedEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_moderator_added_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse ModeratorAddedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::ModeratorRemoved => {
                    info!("Processing ModeratorRemoved event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<ModeratorRemovedEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize ModeratorRemovedEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_moderator_removed_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse ModeratorRemovedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::ProfileBlocked => {
                    info!("Processing ProfileBlocked event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<PlatformBlockedProfileEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize PlatformBlockedProfileEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_profile_blocked_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformBlockedProfileEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::ProfileUnblocked => {
                    info!("Processing ProfileUnblocked event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<PlatformUnblockedProfileEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize PlatformUnblockedProfileEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_profile_unblocked_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformUnblockedProfileEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::PlatformApprovalChanged => {
                    info!("Processing PlatformApprovalChanged event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<PlatformApprovalChangedEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize PlatformApprovalChangedEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_platform_approval_changed_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformApprovalChangedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::UserJoinedPlatform => {
                    info!("Processing UserJoinedPlatform event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<UserJoinedPlatformEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize UserJoinedPlatformEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_user_joined_platform_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse UserJoinedPlatformEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::UserLeftPlatform => {
                    info!("Processing UserLeftPlatform event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<UserLeftPlatformEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize UserLeftPlatformEvent: {}", e))
                    }) {
                        Ok(platform_event) => {
                            self.process_user_left_platform_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse UserLeftPlatformEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
                PlatformEventType::TreasuryFunded => {
                    info!("Processing TreasuryFunded event");
                    match event_utils::extract_event_fields(&event.data).and_then(|fields| {
                        serde_json::from_value::<TreasuryFundedEvent>(fields)
                            .map_err(|e| anyhow!("Failed to deserialize TreasuryFundedEvent: {}", e))
                    }) {
                        Ok(treasury_event) => {
                            self.process_treasury_funded_event(&treasury_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse TreasuryFundedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(e);
                        }
                    }
                }
            }
        } else {
            // Check if it contains platform in the event name for debugging
            if event.event_type.to_lowercase().contains("platform") {
                info!(
                    "Found potential platform event but type not recognized: {}",
                    event.event_type
                );
                info!(
                    "Event data: {}",
                    serde_json::to_string_pretty(&event.data).unwrap_or_default()
                );
            }
            debug!("Not a recognized platform event: {}", event.event_type);
        }

        Ok(())
    }

    /// Start listening for platform events
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting platform event handler");

        while let Some(event) = self.rx.recv().await {
            debug!("Received event: {:?}", event.event_type);

            if let Err(e) = self.process_event(event).await {
                error!("Error processing event: {}", e);
            }
        }

        warn!("Platform event handler channel closed");
        Ok(())
    }
}
