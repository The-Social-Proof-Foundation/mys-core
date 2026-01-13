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
use crate::models::governance::{NewGovernanceRegistry, NewGovernanceEvent};
use crate::schema;
use chrono::Utc;

use super::listener::BlockchainEvent;

// Helper function for normalizing date formats (still used)

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

    /// Normalize DAO governance fields when wants_dao_governance is true
    /// Ensures that DAO fields are properly set (not null) when the platform wants DAO governance
    /// Also detects DAO platforms by checking for presence of DAO-related fields even if wants_dao_governance is None
    fn normalize_dao_fields(event: &PlatformCreatedEvent) -> (
        Option<bool>,
        Option<String>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
    ) {
        // Check if this is a DAO platform:
        // 1. Explicitly set wants_dao_governance to true
        // 2. Or has governance_registry_id set (indicates DAO setup)
        // 3. Or has any DAO governance numeric fields set (indicates DAO configuration)
        let explicit_dao = event.wants_dao_governance.unwrap_or(false);
        let has_governance_registry = event.governance_registry_id.is_some();
        let has_dao_fields = event.delegate_count.is_some()
            || event.delegate_term_epochs.is_some()
            || event.max_votes_per_user.is_some()
            || event.min_on_chain_age_days.is_some()
            || event.proposal_submission_cost.is_some()
            || event.quadratic_base_cost.is_some()
            || event.quorum_votes.is_some()
            || event.voting_period_epochs.is_some();
        
        let is_dao = explicit_dao || has_governance_registry || has_dao_fields;
        
        debug!(
            "DAO detection for platform {}: explicit_dao={}, has_governance_registry={}, has_dao_fields={}, is_dao={}, wants_dao_governance={:?}",
            event.platform_id, explicit_dao, has_governance_registry, has_dao_fields, is_dao, event.wants_dao_governance
        );
        
        if is_dao {
            // If wants_dao_governance was None but we detected it's a DAO, set it to true
            let wants_dao = if explicit_dao {
                Some(true)
            } else if event.wants_dao_governance.is_none() {
                info!("Platform {} detected as DAO (has governance fields) but wants_dao_governance was None - setting to true", event.platform_id);
                Some(true)
            } else {
                event.wants_dao_governance
            };
            
            info!("Platform {} is a DAO - normalizing DAO governance fields", event.platform_id);
            
            // For DAO platforms, ensure numeric fields are properly set
            // Use the values from the event if present, otherwise keep as None (don't force defaults)
            // This preserves the actual values from the blockchain while ensuring proper typing
            (
                wants_dao,
                event.governance_registry_id.clone(),
                event.delegate_count.map(|v| v as i64),
                event.delegate_term_epochs.map(|v| v as i64),
                event.max_votes_per_user.map(|v| v as i64),
                event.min_on_chain_age_days.map(|v| v as i64),
                event.proposal_submission_cost.map(|v| v as i64),
                event.quadratic_base_cost.map(|v| v as i64),
                event.quorum_votes.map(|v| v as i64),
                event.voting_period_epochs.map(|v| v as i64),
                event.treasury.map(|v| v as i64),
                event.version.map(|v| v as i64),
            )
        } else {
            // For non-DAO platforms, keep the original values (which may be None)
            (
                event.wants_dao_governance,
                event.governance_registry_id.clone(),
                event.delegate_count.map(|v| v as i64),
                event.delegate_term_epochs.map(|v| v as i64),
                event.max_votes_per_user.map(|v| v as i64),
                event.min_on_chain_age_days.map(|v| v as i64),
                event.proposal_submission_cost.map(|v| v as i64),
                event.quadratic_base_cost.map(|v| v as i64),
                event.quorum_votes.map(|v| v as i64),
                event.voting_period_epochs.map(|v| v as i64),
                event.treasury.map(|v| v as i64),
                event.version.map(|v| v as i64),
            )
        }
    }

    /// Validate platform categories
    /// Logs warnings for invalid categories but doesn't fail (blockchain validation handles errors)
    fn validate_platform_categories(primary: &str, secondary: Option<&str>) -> Result<(), anyhow::Error> {
        match crate::models::platform::validate_categories(primary, secondary) {
            Ok(()) => Ok(()),
            Err(e) => {
                warn!("Invalid platform categories - primary: '{}', secondary: {:?}, error: {}", primary, secondary, e);
                // Don't fail here - blockchain validation will reject invalid transactions
                // We just log for monitoring purposes
                Ok(())
            }
        }
    }

    /// Process a platform created event
    async fn process_platform_created_event(
        &self,
        event: &PlatformCreatedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing platform created event for platform_id: {}", event.platform_id);
        info!("Raw event values - wants_dao_governance: {:?}, governance_registry_id: {:?}, delegate_count: {:?}", 
            event.wants_dao_governance, event.governance_registry_id, event.delegate_count);

        let mut conn = self.get_connection().await?;

        // Normalize DAO fields if this is a DAO platform
        let (
            wants_dao_governance,
            governance_registry_id,
            delegate_count,
            delegate_term_epochs,
            max_votes_per_user,
            min_on_chain_age_days,
            proposal_submission_cost,
            quadratic_base_cost,
            quorum_votes,
            voting_period_epochs,
            treasury,
            version,
        ) = Self::normalize_dao_fields(event);
        
        info!("Normalized DAO fields - wants_dao_governance: {:?}, governance_registry_id: {:?}, delegate_count: {:?}", 
            wants_dao_governance, governance_registry_id, delegate_count);

        // Clone governance_registry_id for later use in governance creation
        // (it will be moved into UpdatePlatform/NewPlatform structs)
        let governance_registry_id_for_governance = governance_registry_id.clone();

        // Extract and validate categories
        let primary_category = event.primary_category.clone();
        let secondary_category = event.secondary_category.clone();
        
        // Validate categories (logs warnings but doesn't fail - blockchain validation handles errors)
        if let Err(e) = Self::validate_platform_categories(&primary_category, secondary_category.as_deref()) {
            warn!("Category validation error for platform {}: {}", event.platform_id, e);
        }

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

                    // Check if event with this event_id already exists
                    let should_insert = if let Some(ref eid) = platform_event.event_id {
                        let existing_count = schema::platform_events::table
                            .filter(schema::platform_events::event_id.eq(eid))
                            .count()
                            .get_result::<i64>(&mut conn)
                            .await
                            .unwrap_or(0);
                        
                        if existing_count > 0 {
                            info!("Platform event with event_id {} already exists, skipping duplicate insert", eid);
                            false
                        } else {
                            true
                        }
                    } else {
                        // If event_id is None, insert anyway (no way to deduplicate)
                        true
                    };

                    // Insert platform event only if it doesn't already exist
                    if should_insert {
                        info!("📝 Inserting into platform_events table for platform_id: {}", event.platform_id);
                        let platform_event_result = diesel::insert_into(schema::platform_events::table)
                            .values(&platform_event)
                            .execute(&mut conn)
                            .await;
                        match platform_event_result {
                            Ok(rows) => {
                                info!("✅ Successfully inserted into platform_events: {} rows affected", rows);
                            }
                            Err(e) => {
                                error!("❌ Failed to insert into platform_events: {}", e);
                                return Err(e);
                            }
                        }
                    }

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
                            wants_dao_governance,
                            governance_registry_id,
                            delegate_count,
                            delegate_term_epochs,
                            max_votes_per_user,
                            min_on_chain_age_days,
                            proposal_submission_cost,
                            quadratic_base_cost,
                            quorum_votes,
                            voting_period_epochs,
                            treasury,
                            version,
                            primary_category: Some(primary_category.clone()),
                            secondary_category: secondary_category.clone(),
                            deleted_at: None,
                        };

                        info!("📝 Updating existing platform in platforms table: {}", event.platform_id);
                        let update_result = diesel::update(schema::platforms::table)
                            .filter(schema::platforms::platform_id.eq(&event.platform_id))
                            .set(&platform_update)
                            .execute(&mut conn)
                            .await;
                        match update_result {
                            Ok(rows) => {
                                info!("✅ Successfully updated platform in platforms table: {} rows affected", rows);
                            }
                            Err(e) => {
                                error!("❌ Failed to update platform in platforms table: {}", e);
                                return Err(e);
                            }
                        }

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
                            wants_dao_governance,
                            governance_registry_id,
                            delegate_count,
                            delegate_term_epochs,
                            max_votes_per_user,
                            min_on_chain_age_days,
                            proposal_submission_cost,
                            quadratic_base_cost,
                            quorum_votes,
                            voting_period_epochs,
                            treasury,
                            version,
                            primary_category: primary_category.clone(),
                            secondary_category: secondary_category.clone(),
                            deleted_at: None,
                        };

                        // Insert platform
                        info!("📝 Inserting new platform into platforms table: {}", event.platform_id);
                        let platform_insert_result = diesel::insert_into(schema::platforms::table)
                            .values(&new_platform)
                            .execute(&mut conn)
                            .await;
                        match platform_insert_result {
                            Ok(rows) => {
                                info!("✅ Successfully inserted platform into platforms table: {} rows affected", rows);
                            }
                            Err(e) => {
                                error!("❌ Failed to insert platform into platforms table: {}", e);
                                return Err(e);
                            }
                        }

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
                        info!("📝 Inserting developer as moderator into platform_moderators table: platform_id={}, moderator={}", 
                            event.platform_id, event.developer);
                        let moderator_insert_result = diesel::insert_into(schema::platform_moderators::table)
                            .values(&new_moderator)
                            .on_conflict((
                                schema::platform_moderators::platform_id,
                                schema::platform_moderators::moderator_address,
                            ))
                            .do_nothing() // If already exists, do nothing
                            .execute(&mut conn)
                            .await;
                        match moderator_insert_result {
                            Ok(rows) => {
                                info!("✅ Successfully inserted moderator into platform_moderators: {} rows affected", rows);
                            }
                            Err(e) => {
                                error!("❌ Failed to insert moderator into platform_moderators: {}", e);
                                return Err(e);
                            }
                        }

                        info!("Created new platform: {}", event.platform_id);
                    }

                    // If this is a DAO platform, create governance registry and event entries
                    // This ensures PlatformCreatedEvent is the single source of truth for platform governance
                    let has_dao_governance = wants_dao_governance == Some(true) || governance_registry_id_for_governance.is_some() || delegate_count.is_some();
                    if has_dao_governance {
                        if let Some(registry_id) = governance_registry_id_for_governance.as_ref() {
                            info!("Creating governance registry entry for platform: {} with registry_id: {}", event.platform_id, registry_id);
                            
                            // Determine registry_type - for platform governance, we'll need to check what type is used
                            // Since PlatformCreatedEvent doesn't have registry_type, we'll need to determine it
                            // For now, we'll try to query if a GovernanceRegistryCreatedEvent exists with this registry_id
                            // to get the registry_type, otherwise use a default
                            
                            // Try to find existing registry to get registry_type
                            let existing_registry = schema::governance_registries::table
                                .filter(schema::governance_registries::registry_id.eq(registry_id))
                                .select(schema::governance_registries::registry_type)
                                .first::<i16>(&mut conn)
                                .await
                                .optional()?;

                            // Use existing registry_type if found, otherwise default to 3 (platform governance)
                            // Note: This assumes platform governance uses registry_type = 3
                            // If GovernanceRegistryCreatedEvent was already processed, we'll use its type
                            let registry_type = existing_registry.unwrap_or(3i16);

                            let updated_at_ms = now.as_millis() as i64;
                            let event_id_str = blockchain_event.as_ref()
                                .map(|e| e.event_id.clone())
                                .unwrap_or_else(|| format!("platform_created_{}", event.platform_id));

                            let new_registry = NewGovernanceRegistry {
                                registry_type,
                                registry_id: registry_id.clone(),
                                delegate_count: delegate_count.unwrap_or(0),
                                delegate_term_epochs: delegate_term_epochs.unwrap_or(0),
                                proposal_submission_cost: proposal_submission_cost.unwrap_or(0),
                                min_on_chain_age_days: min_on_chain_age_days.unwrap_or(0),
                                max_votes_per_user: max_votes_per_user.unwrap_or(0),
                                quadratic_base_cost: quadratic_base_cost.unwrap_or(0),
                                voting_period_epochs: voting_period_epochs.unwrap_or(0),
                                quorum_votes: quorum_votes.unwrap_or(0),
                                updated_at: updated_at_ms,
                                transaction_id: event_id_str.clone(),
                            };

                            // Check if registry with this registry_id already exists
                            let existing_registry_id = schema::governance_registries::table
                                .filter(schema::governance_registries::registry_id.eq(registry_id))
                                .select(schema::governance_registries::id)
                                .first::<i32>(&mut conn)
                                .await
                                .optional()?;

                            let registry_result = if existing_registry_id.is_some() {
                                // Update existing registry
                                diesel::update(schema::governance_registries::table)
                                    .filter(schema::governance_registries::registry_id.eq(registry_id))
                                    .set((
                                        schema::governance_registries::registry_type.eq(new_registry.registry_type),
                                        schema::governance_registries::delegate_count.eq(new_registry.delegate_count),
                                        schema::governance_registries::delegate_term_epochs.eq(new_registry.delegate_term_epochs),
                                        schema::governance_registries::proposal_submission_cost.eq(new_registry.proposal_submission_cost),
                                        schema::governance_registries::min_on_chain_age_days.eq(new_registry.min_on_chain_age_days),
                                        schema::governance_registries::max_votes_per_user.eq(new_registry.max_votes_per_user),
                                        schema::governance_registries::quadratic_base_cost.eq(new_registry.quadratic_base_cost),
                                        schema::governance_registries::voting_period_epochs.eq(new_registry.voting_period_epochs),
                                        schema::governance_registries::quorum_votes.eq(new_registry.quorum_votes),
                                        schema::governance_registries::updated_at.eq(new_registry.updated_at),
                                        schema::governance_registries::transaction_id.eq(new_registry.transaction_id.clone()),
                                    ))
                                    .execute(&mut conn)
                                    .await
                            } else {
                                // Insert new registry
                                // Note: If registry_type has a unique constraint and this type already exists,
                                // we'll need to handle that case. For now, try insert and handle error.
                                diesel::insert_into(schema::governance_registries::table)
                                    .values(&new_registry)
                                    .execute(&mut conn)
                                    .await
                            };

                            match registry_result {
                                Ok(rows) => {
                                    info!("✅ Successfully created/updated governance registry: {} rows affected", rows);
                                }
                                Err(e) => {
                                    error!("❌ Failed to create/update governance registry: {}", e);
                                    // If insert failed due to unique constraint on registry_type, try updating by registry_type
                                    // This handles the case where registry_type = 3 already exists for another platform
                                    if existing_registry_id.is_none() {
                                        warn!("Insert failed, trying update by registry_type");
                                        let transaction_id_clone = new_registry.transaction_id.clone();
                                        let registry_result_alt = diesel::update(schema::governance_registries::table)
                                            .filter(schema::governance_registries::registry_type.eq(new_registry.registry_type))
                                            .set((
                                                schema::governance_registries::registry_id.eq(new_registry.registry_id.clone()),
                                                schema::governance_registries::delegate_count.eq(new_registry.delegate_count),
                                                schema::governance_registries::delegate_term_epochs.eq(new_registry.delegate_term_epochs),
                                                schema::governance_registries::proposal_submission_cost.eq(new_registry.proposal_submission_cost),
                                                schema::governance_registries::min_on_chain_age_days.eq(new_registry.min_on_chain_age_days),
                                                schema::governance_registries::max_votes_per_user.eq(new_registry.max_votes_per_user),
                                                schema::governance_registries::quadratic_base_cost.eq(new_registry.quadratic_base_cost),
                                                schema::governance_registries::voting_period_epochs.eq(new_registry.voting_period_epochs),
                                                schema::governance_registries::quorum_votes.eq(new_registry.quorum_votes),
                                                schema::governance_registries::updated_at.eq(new_registry.updated_at),
                                                schema::governance_registries::transaction_id.eq(transaction_id_clone),
                                            ))
                                            .execute(&mut conn)
                                            .await;
                                        
                                        match registry_result_alt {
                                            Ok(rows) => {
                                                info!("✅ Successfully updated governance registry by registry_type: {} rows affected", rows);
                                            }
                                            Err(e2) => {
                                                error!("❌ Failed to update governance registry by registry_type: {}", e2);
                                                return Err(e2);
                                            }
                                        }
                                    } else {
                                        return Err(e);
                                    }
                                }
                            }

                            // Create governance event entry
                            let governance_event = NewGovernanceEvent {
                                event_type: "GovernanceRegistryCreatedEvent".to_string(),
                                registry_type,
                                event_data: serde_json::to_value(event).unwrap_or_default(),
                                event_id: event_id_str.clone(),
                                created_at: Utc::now(),
                                anonymous_voting_related: None,
                            };

                            let event_result = diesel::insert_into(schema::governance_events::table)
                                .values(&governance_event)
                                .execute(&mut conn)
                                .await;

                            match event_result {
                                Ok(rows) => {
                                    info!("✅ Successfully created governance event: {} rows affected", rows);
                                }
                                Err(e) => {
                                    error!("❌ Failed to create governance event: {}", e);
                                    // Don't fail the entire transaction - log and continue
                                    warn!("Continuing despite governance event creation failure");
                                }
                            }
                        } else {
                            warn!("Platform {} has DAO governance enabled but no governance_registry_id provided", event.platform_id);
                        }
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

        // Extract and validate categories
        let primary_category = event.primary_category.clone();
        let secondary_category = event.secondary_category.clone();
        
        // Validate categories (logs warnings but doesn't fail - blockchain validation handles errors)
        if let Err(e) = Self::validate_platform_categories(&primary_category, secondary_category.as_deref()) {
            warn!("Category validation error for platform {}: {}", event.platform_id, e);
        }

        // Extract timestamp from blockchain event before moving into closure
        // Filter out 0 timestamps (invalid/unset) - use None instead so we can fallback to current time
        let event_timestamp_ms = blockchain_event.and_then(|e| {
            if e.timestamp_ms > 0 {
                Some(e.timestamp_ms)
            } else {
                debug!("blockchain_event.timestamp_ms is 0, will use fallback");
                None
            }
        });

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
                        // Use blockchain event timestamp if available, otherwise try event.updated_at,
                        // otherwise use current time
                        // Note: event.updated_at may be an epoch/sequence number, not a timestamp,
                        // so we prefer blockchain_event.timestamp_ms when available
                        debug!("PlatformUpdated - event_timestamp_ms: {:?}, event.updated_at: {} (ms)", event_timestamp_ms, event.updated_at);
                        let updated_at = if let Some(timestamp_ms) = event_timestamp_ms {
                            // Use blockchain event timestamp (already in milliseconds)
                            // Helper function validates timestamp and handles 0/invalid values
                            crate::models::platform::milliseconds_to_naive_datetime(timestamp_ms)
                        } else {
                            // Fallback: try event.updated_at if it looks like a reasonable timestamp
                            // (check if it's within a reasonable range - between 2020 and 2100)
                            let min_timestamp_ms = 1577836800000u64; // 2020-01-01
                            let max_timestamp_ms = 4102444800000u64; // 2100-01-01
                            if event.updated_at >= min_timestamp_ms && event.updated_at <= max_timestamp_ms {
                                // Looks like a valid timestamp, use it
                                crate::models::platform::milliseconds_to_naive_datetime(event.updated_at)
                            } else {
                                // Doesn't look like a timestamp, use current time
                                debug!("event.updated_at {} is not in valid range, using current time", event.updated_at);
                                chrono::Utc::now().naive_utc()
                            }
                        };
                        debug!("Converted updated_at: {:?}", updated_at);

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
                            primary_category: Some(primary_category.clone()),
                            secondary_category: secondary_category.clone(),
                            deleted_at: None,
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
                                // Use blockchain event timestamp if available, otherwise try event.updated_at,
                                // otherwise use current time
                                if let Some(timestamp_ms) = event_timestamp_ms {
                                    crate::models::platform::milliseconds_to_naive_datetime(timestamp_ms)
                                } else {
                                    // Check if event.updated_at looks like a valid timestamp
                                    let min_timestamp_ms = 1577836800000u64; // 2020-01-01
                                    let max_timestamp_ms = 4102444800000u64; // 2100-01-01
                                    if event.updated_at >= min_timestamp_ms && event.updated_at <= max_timestamp_ms {
                                        crate::models::platform::milliseconds_to_naive_datetime(event.updated_at)
                                    } else {
                                        chrono::Utc::now().naive_utc()
                                    }
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
                            primary_category: primary_category.clone(),
                            secondary_category: secondary_category.clone(),
                            deleted_at: None,
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
                            primary_category: "".to_string(), // Placeholder - needs manual categorization
                            secondary_category: None, // Not available for placeholder
                            deleted_at: None,
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
                        // event.changed_at is in milliseconds (from deserialize_timestamp_optional)
                        // Use helper function for consistent conversion (validates and handles 0/invalid timestamps)
                        debug!("PlatformApprovalChanged event.changed_at: {} (ms)", event.changed_at);
                        let approval_changed_at = crate::models::platform::milliseconds_to_naive_datetime(event.changed_at);
                        debug!("Converted approval_changed_at: {:?}", approval_changed_at);

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
                            primary_category: None, // Don't change category
                            secondary_category: None, // Don't change category
                            deleted_at: None,
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

    /// Process a platform deleted event
    async fn process_platform_deleted_event(
        &self,
        event: &PlatformDeletedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!(
            "Processing platform deleted event for platform: {}",
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

                    // Convert timestamp from milliseconds to NaiveDateTime
                    let deleted_at = crate::models::platform::milliseconds_to_naive_datetime(event.timestamp);

                    // Create new platform event record
                    let platform_event = NewPlatformEvent {
                        event_type: PlatformEventType::PlatformDeleted
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
                        // Update platform to mark as deleted (soft delete)
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
                            updated_at: Some(deleted_at),
                            is_approved: None, // Don't change approval status
                            approval_changed_at: None,
                            approved_by: None,
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
                            primary_category: None, // Don't change category
                            secondary_category: None, // Don't change category
                            deleted_at: Some(deleted_at),
                        };

                        diesel::update(schema::platforms::table)
                            .filter(schema::platforms::platform_id.eq(&event.platform_id))
                            .set(&platform_update)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Marked platform as deleted: platform_id={}, name={}, deleted_by={}",
                            event.platform_id, event.name, event.deleted_by
                        );
                    } else {
                        warn!(
                            "Platform not found for deletion: {}",
                            event.platform_id
                        );
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed platform deleted event");

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
        // Filter out 0 timestamps (invalid/unset) - use None instead so we can fallback to current time
        let event_timestamp_ms = blockchain_event.and_then(|e| {
            if e.timestamp_ms > 0 {
                Some(e.timestamp_ms)
            } else {
                debug!("blockchain_event.timestamp_ms is 0, will use fallback");
                None
            }
        });

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

                    // Check if the wallet is blocked by the platform
                    // Note: platform_blocked_profiles uses profile_id which may be wallet address
                    let wallet_is_blocked = schema::platform_blocked_profiles::table
                        .filter(
                            schema::platform_blocked_profiles::platform_id.eq(&event.platform_id),
                        )
                        .filter(schema::platform_blocked_profiles::profile_id.eq(&event.wallet_address))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if wallet_is_blocked {
                        warn!(
                            "Ignoring join event for blocked wallet: {} in platform {}",
                            event.wallet_address, event.platform_id
                        );
                        return Ok(());
                    }

                    // Check if membership already exists
                    let membership_exists = schema::platform_memberships::table
                        .filter(schema::platform_memberships::platform_id.eq(&event.platform_id))
                        .filter(schema::platform_memberships::wallet_address.eq(&event.wallet_address))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    if !membership_exists {
                        // Use blockchain event timestamp if available, otherwise try event.timestamp,
                        // otherwise use current time
                        // Note: event.timestamp may be an epoch/sequence number, not a timestamp,
                        // so we prefer blockchain_event.timestamp_ms when available
                        let joined_at = if let Some(timestamp_ms) = event_timestamp_ms {
                            crate::models::platform::milliseconds_to_naive_datetime(timestamp_ms)
                        } else {
                            // Check if event.timestamp looks like a valid timestamp
                            let min_timestamp_ms = 1577836800000u64; // 2020-01-01
                            let max_timestamp_ms = 4102444800000u64; // 2100-01-01
                            if event.timestamp >= min_timestamp_ms && event.timestamp <= max_timestamp_ms {
                                crate::models::platform::milliseconds_to_naive_datetime(event.timestamp)
                            } else {
                                // Doesn't look like a timestamp, use current time
                                chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                                    .unwrap_or_else(|| chrono::Utc::now())
                                    .naive_utc()
                            }
                        };

                        // Create new membership
                        let new_membership = NewPlatformMembership {
                            platform_id: event.platform_id.clone(),
                            wallet_address: event.wallet_address.clone(),
                            joined_at,
                        };

                        // Insert membership
                        diesel::insert_into(schema::platform_memberships::table)
                            .values(new_membership)
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Created new platform membership: {} -> {}",
                            event.wallet_address, event.platform_id
                        );

                        // Create profile event if profile exists for this wallet
                        // Look up profile by owner_address (wallet address)
                        let profile_id_opt = schema::profiles::table
                            .filter(schema::profiles::owner_address.eq(&event.wallet_address))
                            .select(schema::profiles::profile_id.nullable())
                            .first::<Option<String>>(&mut conn)
                            .await
                            .ok()
                            .flatten();

                        if let Some(profile_id) = profile_id_opt {
                            // Create a profile event from UserJoinedPlatformEvent
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
                                    profile_id: profile_id.clone(),
                                    platform_id: event.platform_id.clone(),
                                    timestamp: profile_event_timestamp,
                                };

                            // Get event ID from blockchain_event
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
                                profile_id, event.platform_id
                            );
                        }
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
            "wallet_address": event.wallet_address,
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
        // Filter out 0 timestamps (invalid/unset) - use None instead so we can fallback to current time
        let event_timestamp_ms = blockchain_event.and_then(|e| {
            if e.timestamp_ms > 0 {
                Some(e.timestamp_ms)
            } else {
                debug!("blockchain_event.timestamp_ms is 0, will use fallback");
                None
            }
        });

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
                        .filter(schema::platform_memberships::wallet_address.eq(&event.wallet_address))
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
                            .filter(schema::platform_memberships::wallet_address.eq(&event.wallet_address))
                            .execute(&mut conn)
                            .await?;

                        info!(
                            "Deleted platform membership for user leaving: {} -> {}",
                            event.wallet_address, event.platform_id
                        );

                        // Create profile event if profile exists for this wallet
                        let profile_id_opt = schema::profiles::table
                            .filter(schema::profiles::owner_address.eq(&event.wallet_address))
                            .select(schema::profiles::profile_id.nullable())
                            .first::<Option<String>>(&mut conn)
                            .await
                            .ok()
                            .flatten();

                        if let Some(profile_id) = profile_id_opt {
                            // Create a profile event from UserLeftPlatformEvent
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
                                    profile_id: profile_id.clone(),
                                    platform_id: event.platform_id.clone(),
                                    timestamp: profile_event_timestamp,
                                };

                            // Get event ID from blockchain_event
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
                                profile_id, event.platform_id
                            );
                        }
                    }

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        // Write to relay outbox for notifications - notify platform moderators/owners (outside transaction)
        let mut outbox_conn = self.get_connection().await?;
        let event_data = serde_json::json!({
            "platform_id": event.platform_id,
            "wallet_address": event.wallet_address,
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

    /// Process a token airdrop event
    async fn process_token_airdrop_event(
        &self,
        event: &TokenAirdropEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        debug!("Processing token airdrop event");

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
                        event_type: PlatformEventType::TokenAirdrop.to_str().to_string(),
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

                    // Create detailed airdrop record
                    let airdrop_record = crate::models::platform::NewPlatformTokenAirdrop {
                        platform_id: event.platform_id.clone(),
                        recipient: event.recipient.clone(),
                        amount: event.amount as i64,
                        reason_code: event.reason_code as i16,
                        executed_by: event.executed_by.clone(),
                        timestamp: event.timestamp as i64,
                        created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc(),
                        event_id: blockchain_event.map(|e| e.event_id.clone()),
                    };

                    // Insert detailed airdrop record
                    diesel::insert_into(schema::platform_token_airdrops::table)
                        .values(&airdrop_record)
                        .execute(&mut conn)
                        .await?;

                    info!(
                        "Processed token airdrop: platform_id={}, recipient={}, amount={}, reason_code={}",
                        event.platform_id, event.recipient, event.amount, event.reason_code
                    );

                    Result::<_, diesel::result::Error>::Ok(())
                })
            })
            .await?;

        info!("Successfully processed token airdrop event");

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
                        primary_category: None, // Don't change category
                        secondary_category: None, // Don't change category
                        deleted_at: None,
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
        // CRITICAL: Log at the very start to confirm events reach the handler
        info!("🔵 PLATFORM HANDLER ENTRY: Received event type: {}", event.event_type);
        debug!("Platform handler examining event: {}", event.event_type);

        // Log the raw event data for debugging
        info!("Platform handler received event: {}", event.event_type);
        info!(
            "Event data: {}",
            serde_json::to_string_pretty(&event.data).unwrap_or_default()
        );

        // Use the PlatformEventType from_str method which handles package prefixes
        let event_type = match crate::events::platform_events::PlatformEventType::from_str(&event.event_type) {
            Some(et) => {
                info!("Identified platform event type: {:?}", et);
                et
            }
            None => {
                // Log when platform events match pattern but don't match PlatformEventType
                if event.event_type.contains("::platform::") || event.event_type.contains("Platform") {
                    warn!(
                        "⚠️ Platform event matched pattern but failed PlatformEventType matching: {}",
                        event.event_type
                    );
                    info!(
                        "Event data structure: {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );
                }
                return Ok(()); // Not a platform event we handle
            }
        };

            match event_type {
                PlatformEventType::PlatformCreated => {
                    info!("Processing PlatformCreated event");
                    // Log complete event data for debugging
                    info!(
                        "PlatformCreated event data (raw): {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );
                    
                    // Check if content.fields exists and log it
                    if let Some(content) = event.data.get("content") {
                        if let Some(fields) = content.get("fields") {
                            info!(
                                "PlatformCreated event content.fields: {}",
                                serde_json::to_string_pretty(fields).unwrap_or_default()
                            );
                        }
                    } else if let Some(fields) = event.data.get("fields") {
                        info!(
                            "PlatformCreated event fields (direct): {}",
                            serde_json::to_string_pretty(fields).unwrap_or_default()
                        );
                    }

                    // Use MoveObjectFields wrapper to handle nested content.fields structure
                    match serde_json::from_value::<event_utils::MoveObjectFields<PlatformCreatedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let mut platform_event = wrapper.into_inner();
                            
                            // If platform_id is empty, try to extract from id.id structure
                            if platform_event.platform_id.is_empty() {
                                if let Some(content) = event.data.get("content") {
                                    if let Some(fields) = content.get("fields") {
                                        if let Some(id_obj) = fields.get("id") {
                                            if let Some(id_str) = id_obj.get("id") {
                                                if let Some(s) = id_str.as_str() {
                                                    platform_event.platform_id = s.to_string();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            info!("Successfully deserialized PlatformCreatedEvent using MoveObjectFields");
                            info!("Deserialized event - wants_dao_governance: {:?}, governance_registry_id: {:?}, delegate_count: {:?}, delegate_term_epochs: {:?}, max_votes_per_user: {:?}, quorum_votes: {:?}", 
                                platform_event.wants_dao_governance, 
                                platform_event.governance_registry_id,
                                platform_event.delegate_count,
                                platform_event.delegate_term_epochs,
                                platform_event.max_votes_per_user,
                                platform_event.quorum_votes);
                            
                            // Log the full event data for debugging
                            debug!("Full deserialized PlatformCreatedEvent: {:?}", platform_event);
                            
                            self.process_platform_created_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to deserialize PlatformCreatedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse PlatformCreatedEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::PlatformUpdated => {
                    info!("Processing PlatformUpdated event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<PlatformUpdatedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_platform_updated_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformUpdatedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse PlatformUpdatedEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::ModeratorAdded => {
                    info!("Processing ModeratorAdded event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<ModeratorAddedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_moderator_added_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse ModeratorAddedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse ModeratorAddedEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::ModeratorRemoved => {
                    info!("Processing ModeratorRemoved event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<ModeratorRemovedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_moderator_removed_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse ModeratorRemovedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse ModeratorRemovedEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::ProfileBlocked => {
                    info!("Processing ProfileBlocked event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<PlatformBlockedProfileEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_profile_blocked_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformBlockedProfileEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse PlatformBlockedProfileEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::ProfileUnblocked => {
                    info!("Processing ProfileUnblocked event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<PlatformUnblockedProfileEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_profile_unblocked_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformUnblockedProfileEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse PlatformUnblockedProfileEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::PlatformApprovalChanged => {
                    info!("Processing PlatformApprovalChanged event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<PlatformApprovalChangedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_platform_approval_changed_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformApprovalChangedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse PlatformApprovalChangedEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::UserJoinedPlatform => {
                    info!("Processing UserJoinedPlatform event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<UserJoinedPlatformEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_user_joined_platform_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse UserJoinedPlatformEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse UserJoinedPlatformEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::UserLeftPlatform => {
                    info!("Processing UserLeftPlatform event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<UserLeftPlatformEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_user_left_platform_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse UserLeftPlatformEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse UserLeftPlatformEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::TokenAirdrop => {
                    info!("Processing TokenAirdrop event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<TokenAirdropEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let airdrop_event = wrapper.into_inner();
                            self.process_token_airdrop_event(&airdrop_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse TokenAirdropEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse TokenAirdropEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::TreasuryFunded => {
                    info!("Processing TreasuryFunded event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<TreasuryFundedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let treasury_event = wrapper.into_inner();
                            self.process_treasury_funded_event(&treasury_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse TreasuryFundedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse TreasuryFundedEvent: {}", e));
                        }
                    }
                }
                PlatformEventType::PlatformDeleted => {
                    info!("Processing PlatformDeleted event");
                    match serde_json::from_value::<event_utils::MoveObjectFields<PlatformDeletedEvent>>(event.data.clone()) {
                        Ok(wrapper) => {
                            let platform_event = wrapper.into_inner();
                            self.process_platform_deleted_event(&platform_event, Some(&event))
                                .await?;
                        }
                        Err(e) => {
                            error!("Failed to parse PlatformDeletedEvent: {}", e);
                            error!(
                                "Event data: {}",
                                serde_json::to_string_pretty(&event.data).unwrap_or_default()
                            );
                            return Err(anyhow!("Failed to parse PlatformDeletedEvent: {}", e));
                        }
                    }
                }
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
