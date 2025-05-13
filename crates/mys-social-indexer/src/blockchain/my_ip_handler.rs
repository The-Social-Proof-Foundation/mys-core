// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::{debug, info};
use bigdecimal::BigDecimal;

use crate::db::Database;
use crate::events::{
    parse_event,
    my_ip_event_types::{
        LicenseCreatedEvent,
        LicenseUpdatedEvent,
        LicenseTransferredEvent,
        LicenseStateChangedEvent,
        LicenseLinkedEvent,
        LicenseRegisteredEvent,
        LicenseGrantedEvent,
        RevenueDistributedEvent,
    }
};

use crate::models::my_ip::NewMyIPEvent;

use crate::schema::{my_ip, my_ip_events, my_ip_grants, my_ip_revenue, posts};
use mys_types::event::Event as MysEvent;

/// Handler for MyIP events from the blockchain
pub struct MyIpEventHandler {
    db: Arc<Database>,
}

impl MyIpEventHandler {
    /// Create a new MyIpEventHandler with the given database connection
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Handle a MyIP event from the blockchain
    pub async fn handle_event(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        let event_type = &event.type_.to_string(); // Convert StructTag to String
        
        info!("Processing MyIP event: {}", event_type);
        
        // Process each event type
        if event_type.ends_with("::LicenseCreatedEvent") {
            self.handle_license_created(event, transaction_id).await?;
        } else if event_type.ends_with("::LicenseUpdatedEvent") {
            self.handle_license_updated(event, transaction_id).await?;
        } else if event_type.ends_with("::LicenseTransferredEvent") {
            self.handle_license_transferred(event, transaction_id).await?;
        } else if event_type.ends_with("::LicenseStateChangedEvent") {
            self.handle_license_state_changed(event, transaction_id).await?;
        } else if event_type.ends_with("::LicenseLinkedEvent") {
            self.handle_license_linked(event, transaction_id).await?;
        } else if event_type.ends_with("::LicenseRegisteredEvent") {
            self.handle_license_registered(event, transaction_id).await?;
        } else if event_type.ends_with("::LicenseGrantedEvent") {
            self.handle_license_granted(event, transaction_id).await?;
        } else if event_type.ends_with("::RevenueDistributedEvent") {
            self.handle_revenue_distributed(event, transaction_id).await?;
        } else {
            debug!("Unhandled MyIP event type: {}", event_type);
        }
        
        Ok(())
    }

    /// Handle license created event
    async fn handle_license_created(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseCreatedEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseCreatedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseCreatedEvent: {}", e))?;
        
        info!("Parsed LicenseCreatedEvent: license_id={}, creator={}", 
            parsed_event.license_id, parsed_event.creator);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Convert event to model
        let new_license = parsed_event.into_model(transaction_id.to_string())?;
        
        // Insert the new license into the database - explicitly setting fields
        diesel::insert_into(my_ip::table)
            .values(&new_license)
            .on_conflict(my_ip::license_id)
            .do_update()
            .set((
                my_ip::creator.eq(&new_license.creator),
                my_ip::description.eq(&new_license.description),
                my_ip::permission_flags.eq(&new_license.permission_flags),
                my_ip::license_state.eq(&new_license.license_state),
                my_ip::creation_time.eq(&new_license.creation_time),
                my_ip::transaction_id.eq(transaction_id)
            ))
            .execute(&mut conn)
            .await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseCreatedEvent successfully");
        Ok(())
    }

    /// Handle license updated event
    async fn handle_license_updated(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseUpdatedEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseUpdatedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseUpdatedEvent: {}", e))?;
        
        info!("Parsed LicenseUpdatedEvent: license_id={}, updater={}", 
            parsed_event.license_id, parsed_event.updater);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Update the license in the database
        diesel::update(my_ip::table.filter(my_ip::license_id.eq(&parsed_event.license_id)))
            .set(my_ip::permission_flags.eq(parsed_event.new_permission_flags as i64))
            .execute(&mut conn)
            .await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseUpdatedEvent successfully");
        Ok(())
    }

    /// Handle license transferred event
    async fn handle_license_transferred(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseTransferredEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseTransferredEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseTransferredEvent: {}", e))?;
        
        info!("Parsed LicenseTransferredEvent: license_id={}, from={}, to={}", 
            parsed_event.license_id, parsed_event.from, parsed_event.to);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Update the license owner in the database
        diesel::update(my_ip::table.filter(my_ip::license_id.eq(&parsed_event.license_id)))
            .set(my_ip::creator.eq(&parsed_event.to)) // After transfer, new creator is the recipient
            .execute(&mut conn)
            .await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        // Add a grant record
        let new_grant = parsed_event.into_grant(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_grants::table)
            .values(&new_grant)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseTransferredEvent successfully");
        Ok(())
    }

    /// Handle license state changed event
    async fn handle_license_state_changed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseStateChangedEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseStateChangedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseStateChangedEvent: {}", e))?;
        
        info!("Parsed LicenseStateChangedEvent: license_id={}, old_state={}, new_state={}", 
            parsed_event.license_id, parsed_event.old_state, parsed_event.new_state);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Update the license state in the database
        diesel::update(my_ip::table.filter(my_ip::license_id.eq(&parsed_event.license_id)))
            .set(my_ip::license_state.eq(parsed_event.new_state as i16))
            .execute(&mut conn)
            .await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseStateChangedEvent successfully");
        Ok(())
    }

    /// Handle license linked event
    async fn handle_license_linked(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseLinkedEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseLinkedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseLinkedEvent: {}", e))?;
        
        info!("Parsed LicenseLinkedEvent: license_id={}, post_id={}, linker={}", 
            parsed_event.license_id, parsed_event.post_id, parsed_event.linker);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Update the post to link it to the license
        diesel::update(posts::table.filter(posts::post_id.eq(&parsed_event.post_id)))
            .set(posts::my_ip_id.eq(&parsed_event.license_id))
            .execute(&mut conn)
            .await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseLinkedEvent successfully");
        Ok(())
    }

    /// Handle license registered event
    async fn handle_license_registered(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseRegisteredEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseRegisteredEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseRegisteredEvent: {}", e))?;
        
        info!("Parsed LicenseRegisteredEvent: license_id={}, registry_id={}", 
            parsed_event.license_id, parsed_event.registry_id);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // We don't need to do much with this event other than record it
        // since we already handle license creation separately
        
        // Add an event record
        let event_data = serde_json::json!({
            "registry_id": parsed_event.registry_id,
            "creator": parsed_event.creator,
            "permission_flags": parsed_event.permission_flags,
        });
        
        let new_event = NewMyIPEvent {
            event_type: "LICENSE_REGISTERED".to_string(),
            license_id: parsed_event.license_id,
            event_data,
            created_by: parsed_event.creator,
            created_at: chrono::Utc::now().timestamp(), // Use current time as event doesn't provide timestamp
            transaction_id: transaction_id.to_string(),
        };
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseRegisteredEvent successfully");
        Ok(())
    }

    /// Handle license granted event
    async fn handle_license_granted(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing LicenseGrantedEvent");
        
        // Parse the event
        let parsed_event = parse_event::<LicenseGrantedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse LicenseGrantedEvent: {}", e))?;
        
        info!("Parsed LicenseGrantedEvent: license_id={}, ip_id={}, grantor={}, grantee={}", 
            parsed_event.license_id, parsed_event.ip_id, parsed_event.grantor, parsed_event.grantee);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        // Add a grant record
        let new_grant = parsed_event.into_grant(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_grants::table)
            .values(&new_grant)
            .execute(&mut conn)
            .await?;
        
        info!("Processed LicenseGrantedEvent successfully");
        Ok(())
    }

    /// Handle revenue distributed event
    async fn handle_revenue_distributed(&self, event: &MysEvent, transaction_id: &str) -> Result<()> {
        info!("Processing RevenueDistributedEvent");
        
        // Parse the event
        let parsed_event = parse_event::<RevenueDistributedEvent>(event)
            .map_err(|e| anyhow!("Failed to parse RevenueDistributedEvent: {}", e))?;
        
        info!("Parsed RevenueDistributedEvent: license_id={}, from={}, to={}, amount={}", 
            parsed_event.license_id, parsed_event.from_address, parsed_event.to_address, parsed_event.amount);
        
        // Get a database connection
        let mut conn = self.db.get_connection().await?;
        
        // Add an event record
        let new_event = parsed_event.into_event(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;
        
        // Add a revenue record
        let new_revenue = parsed_event.into_revenue(transaction_id.to_string())?;
        
        diesel::insert_into(my_ip_revenue::table)
            .values(&new_revenue)
            .execute(&mut conn)
            .await?;
        
        info!("Processed RevenueDistributedEvent successfully");
        Ok(())
    }

    /// Update license statistics based on interactions
    pub async fn update_license_statistics(&self, license_id: &str) -> Result<()> {
        info!("Updating statistics for license: {}", license_id);
        
        let mut conn = self.db.get_connection().await?;
        
        // Get total revenue for the license - fix type to match Numeric database type
        let total_revenue: Option<BigDecimal> = my_ip_revenue::table
            .filter(my_ip_revenue::license_id.eq(license_id))
            .select(diesel::dsl::sum(my_ip_revenue::amount))
            .first::<Option<BigDecimal>>(&mut conn)
            .await?;
        
        let total_revenue: i64 = total_revenue
            .map(|bd| bd.to_string().parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        
        // Get post count, total reactions, comments, reposts, etc.
        let post_stats = diesel::sql_query("
            SELECT 
                COUNT(*) as post_count,
                SUM(reaction_count) as total_reactions, 
                SUM(comment_count) as total_comments,
                SUM(repost_count) as total_reposts,
                SUM(tips_received) as total_tips
            FROM posts 
            WHERE my_ip_id = $1
              AND deleted_at IS NULL
              AND removed_from_platform = false
        ")
            .bind::<diesel::sql_types::Text, _>(license_id)
            .get_results::<PostStats>(&mut conn)
            .await?;
        
        if !post_stats.is_empty() {
            let stats = &post_stats[0]; // Use indexing instead of first()
            info!("License {} stats: posts={}, reactions={}, comments={}, reposts={}, tips={}, total_revenue={}",
                license_id, stats.post_count, stats.total_reactions.unwrap_or(0), stats.total_comments.unwrap_or(0), 
                stats.total_reposts.unwrap_or(0), stats.total_tips.unwrap_or(0), total_revenue);
        }
        
        Ok(())
    }
}

#[derive(QueryableByName)]
struct PostStats {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub post_count: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    pub total_reactions: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    pub total_comments: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    pub total_reposts: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    pub total_tips: Option<i64>,
} 