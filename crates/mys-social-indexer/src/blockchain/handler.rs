// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::Result;
use tracing::info;

use mys_types::event::Event as MysEvent;

use crate::db::Database;
use crate::events::MODULE_PREFIX_PROFILE;
use crate::events::MODULE_PREFIX_PLATFORM;
use crate::events::MODULE_PREFIX_SOCIAL_GRAPH;
use crate::events::MODULE_PREFIX_GOVERNANCE;
use crate::events::MODULE_PREFIX_BLOCK_LIST;
use crate::events::MODULE_PREFIX_MYDATA;
use crate::events::MODULE_PREFIX_CONTENT;

use super::profile_handler;
use super::platform_handler;
use super::social_graph_handler;
use super::governance_handler;
use super::post_handler;
use super::mydata_handler;

/// Handle a blockchain event
pub async fn handle_event(db: &Arc<Database>, event: &MysEvent, tx_digest: &str) -> Result<()> {
    let event_type = &event.type_;
    info!("Blockchain event: {}", event_type);
    
    // Check for post events first (more specific than content)
    if event_type.contains("::post::") {
        post_handler::handle_event(db, event, tx_digest).await?;
    } else if event_type.contains("::profile::") {
        profile_handler::handle_event(db, event, tx_digest).await?;
    } else if event_type.contains("::platform::") {
        platform_handler::handle_event(db, event, tx_digest).await?;
    } else if event_type.contains("::social_graph::") {
        social_graph_handler::handle_event(db, event, tx_digest).await?;
    } else if event_type.contains("::governance::") {
        governance_handler::handle_event(db, event, tx_digest).await?;
    } else if event_type.contains("::block_list::") {
        // Block list events are handled by blockchain/events.rs
    } else if event_type.contains("::mydata::") {
        mydata_handler::handle_event(db, event, tx_digest).await?;
    } else if event_type.contains(MODULE_PREFIX_CONTENT) && !event_type.contains("::post::") {
        // Legacy content events (not post events)
        post_handler::handle_event(db, event, tx_digest).await?;
    }
    
    Ok(())
} 
