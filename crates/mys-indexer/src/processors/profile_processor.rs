// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use diesel::prelude::*;
use tracing::error;

use mys_types::base_types::{MysAddress, ObjectID};
use mys_types::event::{Event, EventID};
use mys_types::object::Object;
use mys_types::object::{MoveObject, Owner};
use mys_types::transaction::TransactionData;
use mys_types::Identifier;

use crate::errors::IndexerError;
use crate::metrics::IndexerMetrics;
use crate::models::profile::{StoredProfile, StoredProfileEvent};
use crate::processors::processor::IndexingProcessor;

const PROFILE_MODULE: &str = "profile";
const PROFILE_STRUCT: &str = "Profile";
const PROFILE_CREATED_EVENT: &str = "ProfileCreatedEvent";
const PROFILE_UPDATED_EVENT: &str = "ProfileUpdatedEvent";
const USERNAME_UPDATED_EVENT: &str = "UsernameUpdatedEvent";
const USERNAME_NFT_ASSIGNED_EVENT: &str = "UsernameNFTAssignedEvent";
const USERNAME_NFT_REMOVED_EVENT: &str = "UsernameNFTRemovedEvent";

pub struct ProfileProcessor {
    metrics: Arc<IndexerMetrics>,
}

impl ProfileProcessor {
    pub fn new(metrics: Arc<IndexerMetrics>) -> Self {
        Self { metrics }
    }

    pub fn process_events(
        &self,
        conn: &mut AsyncPgConnection,
        events: &[Event],
        timestamp_ms: u64,
        tx_sequence_number: i64,
        checkpoint_sequence_number: i64,
    ) -> Result<(), IndexerError> {
        let mut profile_events = Vec::new();

        for (event_index, event) in events.iter().enumerate() {
            // Skip events not from the profile module
            if !Self::is_profile_event(event) {
                continue;
            }

            // Process different event types
            match Self::get_event_type(event) {
                Some(PROFILE_CREATED_EVENT) => {
                    if let Err(e) = self.handle_profile_created_event(
                        conn,
                        event,
                        event_index as i64,
                        timestamp_ms,
                        tx_sequence_number,
                        checkpoint_sequence_number,
                        &mut profile_events,
                    ) {
                        error!("Failed to handle profile created event: {}", e);
                    }
                }
                Some(PROFILE_UPDATED_EVENT) => {
                    if let Err(e) = self.handle_profile_updated_event(
                        conn,
                        event,
                        event_index as i64,
                        timestamp_ms,
                        &mut profile_events,
                    ) {
                        error!("Failed to handle profile updated event: {}", e);
                    }
                }
                Some(USERNAME_UPDATED_EVENT) | 
                Some(USERNAME_NFT_ASSIGNED_EVENT) | 
                Some(USERNAME_NFT_REMOVED_EVENT) => {
                    if let Err(e) = self.handle_username_event(
                        conn,
                        event,
                        event_index as i64,
                        timestamp_ms,
                        &mut profile_events,
                    ) {
                        error!("Failed to handle username event: {}", e);
                    }
                }
                _ => {}
            }
        }

        // Insert profile events in batch
        if !profile_events.is_empty() {
            diesel::insert_into(crate::schema::profile_events::table)
                .values(&profile_events)
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        Ok(())
    }

    fn is_profile_event(event: &Event) -> bool {
        if let Some(package) = event.package_id() {
            if let Some(module_name) = event.type_.get_module() {
                if module_name.as_str() == PROFILE_MODULE {
                    return true;
                }
            }
        }
        false
    }

    fn get_event_type(event: &Event) -> Option<&str> {
        event.type_.get_struct()?.as_str()
    }

    fn handle_profile_created_event(
        &self,
        conn: &mut AsyncPgConnection,
        event: &Event,
        event_sequence_number: i64,
        timestamp_ms: u64,
        tx_sequence_number: i64,
        checkpoint_sequence_number: i64,
        profile_events: &mut Vec<StoredProfileEvent>,
    ) -> Result<(), IndexerError> {
        let event_data = &event.contents;
        let parsed_json: serde_json::Value = serde_json::from_slice(event_data)
            .map_err(|e| IndexerError::GenericError(format!("Failed to parse event JSON: {}", e)))?;
            
        let profile_id = parsed_json["profile_id"].as_str().ok_or_else(|| {
            IndexerError::GenericError(format!("Missing profile_id in {}", PROFILE_CREATED_EVENT))
        })?;
        let display_name = parsed_json["display_name"].as_str().ok_or_else(|| {
            IndexerError::GenericError(format!("Missing display_name in {}", PROFILE_CREATED_EVENT))
        })?;
        let owner = parsed_json["owner"].as_str().ok_or_else(|| {
            IndexerError::GenericError(format!("Missing owner in {}", PROFILE_CREATED_EVENT))
        })?;

        // Get or query the profile object to get more data
        let profile_id_bytes = ObjectID::from_str(profile_id).map_err(|e| 
            IndexerError::GenericError(format!("Failed to parse ObjectID: {}", e))
        )?.to_vec();
        let owner_bytes = MysAddress::from_str(owner).map_err(|e| 
            IndexerError::GenericError(format!("Failed to parse MysAddress: {}", e))
        )?.to_vec();

        // For now, just add placeholder values for fields not in the event
        let stored_profile = StoredProfile {
            profile_id: profile_id_bytes.clone(),
            owner: owner_bytes.clone(),
            display_name: display_name.to_string(),
            bio: "".to_string(), // This would come from the actual object
            profile_picture: None,
            created_at: timestamp_ms as i64,
            username_nft_id: None,
            tx_sequence_number,
            checkpoint_sequence_number,
            timestamp_ms: timestamp_ms as i64,
        };

        // Insert the profile record
        diesel::insert_into(crate::schema::profiles::table)
            .values(&stored_profile)
            .on_conflict(crate::schema::profiles::profile_id)
            .do_update()
            .set(&stored_profile)
            .execute(conn)?;

        // Add event record
        let tx_seq = tx_sequence_number; // Capture in local to avoid confusion
        profile_events.push(StoredProfileEvent {
            tx_sequence_number: tx_seq,
            event_sequence_number,
            profile_id: profile_id_bytes,
            event_type: PROFILE_CREATED_EVENT.to_string(),
            owner: owner_bytes,
            timestamp_ms: timestamp_ms as i64,
            data: parsed_json,
        });

        Ok(())
    }

    fn handle_profile_updated_event(
        &self,
        conn: &mut AsyncPgConnection,
        event: &Event,
        event_sequence_number: i64,
        timestamp_ms: u64,
        profile_events: &mut Vec<StoredProfileEvent>,
    ) -> Result<(), IndexerError> {
        let event_data = &event.contents;
        let parsed_json: serde_json::Value = serde_json::from_slice(event_data)
            .map_err(|e| IndexerError::GenericError(format!("Failed to parse event JSON: {}", e)))?;
            
        let profile_id = parsed_json["profile_id"].as_str().ok_or_else(|| {
            IndexerError::GenericError(format!("Missing profile_id in {}", PROFILE_UPDATED_EVENT))
        })?;
        let owner = parsed_json["owner"].as_str().ok_or_else(|| {
            IndexerError::GenericError(format!("Missing owner in {}", PROFILE_UPDATED_EVENT))
        })?;

        // Convert IDs to bytes
        let profile_id_bytes = ObjectID::from_str(profile_id).map_err(|e| 
            IndexerError::GenericError(format!("Failed to parse ObjectID: {}", e))
        )?.to_vec();
        let owner_bytes = MysAddress::from_str(owner).map_err(|e| 
            IndexerError::GenericError(format!("Failed to parse MysAddress: {}", e))
        )?.to_vec();

        // Update profile record
        // In a full implementation, we would update all fields of the profile
        // but for this example, we just add the event
        let tx_seq = tx_sequence_number; // Capture in local to avoid confusion
        profile_events.push(StoredProfileEvent {
            tx_sequence_number: tx_seq,
            event_sequence_number,
            profile_id: profile_id_bytes,
            event_type: PROFILE_UPDATED_EVENT.to_string(),
            owner: owner_bytes,
            timestamp_ms: timestamp_ms as i64,
            data: parsed_json,
        });

        Ok(())
    }

    fn handle_username_event(
        &self,
        conn: &mut AsyncPgConnection,
        event: &Event,
        event_sequence_number: i64,
        timestamp_ms: u64,
        profile_events: &mut Vec<StoredProfileEvent>,
    ) -> Result<(), IndexerError> {
        let event_data = &event.contents;
        let parsed_json: serde_json::Value = serde_json::from_slice(event_data)
            .map_err(|e| IndexerError::GenericError(format!("Failed to parse event JSON: {}", e)))?;
            
        let profile_id = parsed_json["profile_id"].as_str().ok_or_else(|| {
            IndexerError::GenericError(format!("Missing profile_id in username event"))
        })?;

        let event_type = match Self::get_event_type(event) {
            Some(t) => t,
            None => return Err(IndexerError::GenericError("Unknown username event type".into())),
        };

        // Convert ID to bytes
        let profile_id_bytes = ObjectID::from_str(profile_id).map_err(|e| 
            IndexerError::GenericError(format!("Failed to parse ObjectID: {}", e))
        )?.to_vec();
        
        // Extract owner from event or use a placeholder
        let owner_bytes = if let Some(owner) = parsed_json["owner"].as_str() {
            MysAddress::from_str(owner).map_err(|e| 
                IndexerError::GenericError(format!("Failed to parse MysAddress: {}", e))
            )?.to_vec()
        } else {
            // In a real implementation, we would query the profile to get the owner
            vec![]
        };

        // Add event record
        let tx_seq = tx_sequence_number; // Capture in local to avoid confusion
        profile_events.push(StoredProfileEvent {
            tx_sequence_number: tx_seq,
            event_sequence_number,
            profile_id: profile_id_bytes.clone(),
            event_type: event_type.to_string(),
            owner: owner_bytes,
            timestamp_ms: timestamp_ms as i64,
            data: parsed_json,
        });

        // In a full implementation, we would also update the profile record
        // with the username information

        Ok(())
    }
}

impl IndexingProcessor for ProfileProcessor {
    fn name(&self) -> &'static str {
        "profile_processor"
    }

    fn index_tx(
        &self,
        conn: &mut AsyncPgConnection,
        _tx: &TransactionData,
        events: &[Event],
        timestamp_ms: u64,
        tx_sequence_number: i64,
        checkpoint_sequence_number: i64,
    ) -> Result<(), IndexerError> {
        self.process_events(
            conn,
            events,
            timestamp_ms,
            tx_sequence_number,
            checkpoint_sequence_number,
        )
    }
}