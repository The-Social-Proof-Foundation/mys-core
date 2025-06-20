// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::db::{Database, DbConnection};
// Import event types specifically to avoid ambiguity
use crate::events::post_event_types::{
    PostCreatedEvent,
    CommentCreatedEvent,
    ReactionEvent,
    RemoveReactionEvent,
    RepostEvent,
    TipEvent,
    ModerationEvent as PostModerationEvent,
    ContentUpdateEvent,
    ReportEvent,
    DeletionEvent as PostDeletionEvent,
};
use crate::events::{parse_event, event_utils::parse_json_event};
use crate::models::indexer::NewIndexerProgress;
use crate::schema;
use mys_types::event::Event as MysEvent;

use crate::schema::{posts, comments, reactions, reaction_counts, reposts, tips, 
                   posts_reports, posts_moderation_events, posts_deletion_events};

use super::listener::BlockchainEvent;

/// Handler for post events
pub struct PostEventHandler {
    /// Database connection
    db: Arc<Database>,
    /// Event receiver channel
    rx: mpsc::Receiver<BlockchainEvent>,
    /// Worker ID for tracking progress
    worker_id: String,
}

impl PostEventHandler {
    /// Create a new post event handler
    pub fn new(db: Arc<Database>, rx: mpsc::Receiver<BlockchainEvent>, worker_id: String) -> Self {
        Self {
            db,
            rx,
            worker_id,
        }
    }
    
    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db.get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }
    
    /// Update worker progress with timestamp
    async fn update_progress(&self, timestamp: u64) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();
        
        let progress = NewIndexerProgress {
            id: self.worker_id.clone(),
            last_checkpoint_processed: timestamp as i64,
            last_processed_at: now,
        };
        
        diesel::insert_into(schema::indexer_progress::table)
            .values(&progress)
            .on_conflict(schema::indexer_progress::id)
            .do_update()
            .set((
                schema::indexer_progress::last_checkpoint_processed.eq(progress.last_checkpoint_processed),
                schema::indexer_progress::last_processed_at.eq(progress.last_processed_at),
            ))
            .execute(&mut conn)
            .await?;
            
        Ok(())
    }
    
    /// Process a post created event
    async fn process_post_created(&self, event: &PostCreatedEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing post created: {}", event.post_id);
        
        // Convert event to database model
        let mut new_post = event.into_model()?;
        new_post.transaction_id = tx_id.to_string();
        
        // Insert into the database
        diesel::insert_into(schema::posts::table)
            .values(new_post)
            .on_conflict(schema::posts::id)
            .do_update()
            .set(schema::posts::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed post created: {}", event.post_id);
        Ok(())
    }
    
    /// Process a comment created event
    async fn process_comment_created(&self, event: &CommentCreatedEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing comment created: {}", event.comment_id);
        
        // Convert event to database model
        let mut new_comment = event.into_model()?;
        new_comment.transaction_id = tx_id.to_string();
        
        // Insert the comment
        diesel::insert_into(schema::comments::table)
            .values(new_comment)
            .on_conflict(schema::comments::id)
            .do_update()
            .set(schema::comments::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        // Update post comment count
        diesel::update(schema::posts::table)
            .filter(schema::posts::post_id.eq(&event.post_id))
            .set(schema::posts::comment_count.eq(schema::posts::comment_count + 1))
            .execute(&mut conn)
            .await?;
            
        // If this is a comment on another comment, update parent comment count
        if let Some(parent_comment_id) = &event.parent_comment_id {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(parent_comment_id))
                .set(schema::comments::comment_count.eq(schema::comments::comment_count + 1))
                .execute(&mut conn)
                .await?;
        }
            
        info!("Successfully processed comment created: {}", event.comment_id);
        Ok(())
    }
    
    /// Process a reaction event
    async fn process_reaction(&self, event: &ReactionEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing reaction: {} {} {}", event.user_address, event.reaction_text, event.object_id);
        
        // Convert event to database model
        let mut new_reaction = event.into_model()?;
        new_reaction.transaction_id = tx_id.to_string();
        
        // Insert into reactions table (replacing existing if needed)
        diesel::insert_into(schema::reactions::table)
            .values(new_reaction)
            .on_conflict((schema::reactions::object_id, schema::reactions::user_address))
            .do_update()
            .set(schema::reactions::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        // Update or insert into reaction_counts
        let reaction_count = event.into_reaction_count()?;
        
        diesel::insert_into(schema::reaction_counts::table)
            .values(reaction_count)
            .on_conflict((schema::reaction_counts::object_id, schema::reaction_counts::reaction_text))
            .do_update()
            .set(schema::reaction_counts::count.eq(schema::reaction_counts::count + 1))
            .execute(&mut conn)
            .await?;
            
        // Update the post or comment reaction count
        if event.is_post {
            diesel::update(schema::posts::table)
                .filter(schema::posts::post_id.eq(&event.object_id))
                .set(schema::posts::reaction_count.eq(schema::posts::reaction_count + 1))
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.object_id))
                .set(schema::comments::reaction_count.eq(schema::comments::reaction_count + 1))
                .execute(&mut conn)
                .await?;
        }
            
        info!("Successfully processed reaction");
        Ok(())
    }
    
    /// Process a remove reaction event
    async fn process_remove_reaction(&self, event: &RemoveReactionEvent, _tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing remove reaction: {} {} {}", event.user_address, event.reaction_text, event.object_id);
        
        // First get the reaction to be removed to know the reaction_text
        let reaction_row = diesel::sql_query(
            "SELECT reaction_text FROM reactions WHERE object_id = $1 AND user_address = $2"
        )
        .bind::<diesel::sql_types::Text, _>(&event.object_id)
        .bind::<diesel::sql_types::Text, _>(&event.user_address)
        .get_result::<ReactionTextResult>(&mut conn)
        .await;
        
        if let Ok(reaction) = reaction_row {
            let reaction_text = reaction.reaction_text;
            
            // Delete the reaction
            diesel::delete(schema::reactions::table)
                .filter(schema::reactions::object_id.eq(&event.object_id))
                .filter(schema::reactions::user_address.eq(&event.user_address))
                .execute(&mut conn)
                .await?;
                
            // Update reaction_counts
            diesel::update(schema::reaction_counts::table)
                .filter(schema::reaction_counts::object_id.eq(&event.object_id))
                .filter(schema::reaction_counts::reaction_text.eq(&reaction_text))
                .set(schema::reaction_counts::count.eq(schema::reaction_counts::count - 1))
                .execute(&mut conn)
                .await?;
                
            // Clean up zero counts
            diesel::delete(schema::reaction_counts::table)
                .filter(schema::reaction_counts::object_id.eq(&event.object_id))
                .filter(schema::reaction_counts::reaction_text.eq(&reaction_text))
                .filter(schema::reaction_counts::count.le(0))
                .execute(&mut conn)
                .await?;
                
            // Update the post or comment reaction count
            if event.is_post {
                diesel::update(schema::posts::table)
                    .filter(schema::posts::post_id.eq(&event.object_id))
                    .set(schema::posts::reaction_count.eq(schema::posts::reaction_count - 1))
                    .execute(&mut conn)
                    .await?;
            } else {
                diesel::update(schema::comments::table)
                    .filter(schema::comments::comment_id.eq(&event.object_id))
                    .set(schema::comments::reaction_count.eq(schema::comments::reaction_count - 1))
                    .execute(&mut conn)
                    .await?;
            }
        } else {
            info!("No reaction found to remove for user {} on object {}", event.user_address, event.object_id);
        }
            
        info!("Successfully processed remove reaction");
        Ok(())
    }
    
    /// Process a repost event
    async fn process_repost(&self, event: &RepostEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing repost: {}", event.repost_id);
        
        // Convert event to database model
        let mut new_repost = event.into_model()?;
        new_repost.transaction_id = tx_id.to_string();
        
        // Insert the repost
        diesel::insert_into(schema::reposts::table)
            .values(new_repost)
            .on_conflict(schema::reposts::id)
            .do_update()
            .set(schema::reposts::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        // Update original content repost count
        if event.is_original_post {
            diesel::update(schema::posts::table)
                .filter(schema::posts::post_id.eq(&event.original_post_id))
                .set(schema::posts::repost_count.eq(schema::posts::repost_count + 1))
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.original_id))
                .set(schema::comments::repost_count.eq(schema::comments::repost_count + 1))
                .execute(&mut conn)
                .await?;
        }
            
        info!("Successfully processed repost: {}", event.repost_id);
        Ok(())
    }
    
    /// Process a tip event
    async fn process_tip(&self, event: &TipEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing tip: {} to {} for {}", event.from, event.to, event.object_id);
        
        // Convert event to database model
        let mut new_tip = event.into_model()?;
        new_tip.transaction_id = tx_id.to_string();
        
        // Insert the tip
        diesel::insert_into(schema::tips::table)
            .values(&new_tip)
            .execute(&mut conn)
            .await?;
            
        // Update the tips received amount on the post or comment
        if event.is_post {
            diesel::update(schema::posts::table)
                .filter(schema::posts::post_id.eq(&event.object_id))
                .set(schema::posts::tips_received.eq(schema::posts::tips_received + event.amount as i64))
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.object_id))
                .set(schema::comments::tips_received.eq(schema::comments::tips_received + event.amount as i64))
                .execute(&mut conn)
                .await?;
        }
        
        // Create unified revenue record for the tip
        let unified_revenue = event.create_unified_revenue_record(tx_id.to_string())?;
        
        diesel::insert_into(crate::schema::unified_revenue::table)
            .values(&unified_revenue)
            .execute(&mut conn)
            .await?;
        
        info!("Processed TipEvent with revenue tracking successfully");
        Ok(())
    }
    
    /// Process a moderation event
    async fn process_moderation(&self, event: &PostModerationEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing moderation: {} by {}", event.object_id, event.moderated_by);
        
        // Convert event to database model
        let mut new_moderation = event.into_model()?;
        new_moderation.transaction_id = tx_id.to_string();
        
        // Insert the moderation event
        diesel::insert_into(schema::posts_moderation_events::table)
            .values(new_moderation)
            .execute(&mut conn)
            .await?;
            
        // Try to update post moderation status
        let post_updated = diesel::update(schema::posts::table)
            .filter(schema::posts::post_id.eq(&event.object_id))
            .set((
                schema::posts::removed_from_platform.eq(event.removed),
                schema::posts::removed_by.eq(event.moderated_by.clone()),
            ))
            .execute(&mut conn)
            .await?;
            
        if post_updated == 0 {
            // If no post was updated, try comment
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.object_id))
                .set((
                    schema::comments::removed_from_platform.eq(event.removed),
                    schema::comments::removed_by.eq(event.moderated_by.clone()),
                ))
                .execute(&mut conn)
                .await?;
        }
            
        info!("Successfully processed moderation");
        Ok(())
    }
    
    /// Process a content update event
    async fn process_content_update(&self, event: &ContentUpdateEvent, _tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing content update: {}", event.object_id);
        
        // Convert media_urls and mentions to JSON if present
        let media_urls_json = event.media_urls.as_ref().map(|urls| {
            serde_json::to_value(urls).unwrap_or(serde_json::json!(null))
        });
        
        let mentions_json = event.mentions.as_ref().map(|mentions| {
            serde_json::to_value(mentions).unwrap_or(serde_json::json!(null))
        });
        
        // Parse metadata JSON if present
        let metadata_json = event.metadata_json.as_ref().map(|json_str| {
            serde_json::from_str(json_str).unwrap_or(serde_json::json!(null))
        });
        
        // Update the content
        if event.is_post {
            diesel::update(schema::posts::table)
                .filter(schema::posts::post_id.eq(&event.object_id))
                .set((
                    schema::posts::content.eq(&event.content),
                    schema::posts::media_urls.eq(&media_urls_json),
                    schema::posts::mentions.eq(&mentions_json),
                    schema::posts::metadata_json.eq(&metadata_json),
                    schema::posts::updated_at.eq(event.updated_at as i64),
                ))
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.object_id))
                .set((
                    schema::comments::content.eq(&event.content),
                    schema::comments::media_urls.eq(&media_urls_json),
                    schema::comments::mentions.eq(&mentions_json),
                    schema::comments::metadata_json.eq(&metadata_json),
                    schema::comments::updated_at.eq(event.updated_at as i64),
                ))
                .execute(&mut conn)
                .await?;
        }
            
        info!("Successfully processed content update: {}", event.object_id);
        Ok(())
    }
    
    /// Process a report event
    async fn process_report(&self, event: &ReportEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing report: {} by {}", event.object_id, event.reporter);
        
        // Convert event to database model
        let mut new_report = event.into_model()?;
        new_report.transaction_id = tx_id.to_string();
        
        // Insert the report
        diesel::insert_into(schema::posts_reports::table)
            .values(new_report)
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed report");
        Ok(())
    }
    
    /// Process a deletion event
    async fn process_deletion(&self, event: &PostDeletionEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing deletion: {}", event.object_id);
        
        // Convert event to database model
        let mut new_deletion = event.into_model()?;
        new_deletion.transaction_id = tx_id.to_string();
        
        // Insert the deletion event
        diesel::insert_into(schema::posts_deletion_events::table)
            .values(new_deletion)
            .execute(&mut conn)
            .await?;
            
        // Mark content as deleted
        if event.is_post {
            diesel::update(schema::posts::table)
                .filter(schema::posts::post_id.eq(&event.object_id))
                .filter(schema::posts::owner.eq(&event.owner))
                .set(schema::posts::deleted_at.eq(event.deleted_at as i64))
                .execute(&mut conn)
                .await?;
        } else {
            // Get post_id to update comment count
            let post_id_result = diesel::sql_query(
                "SELECT post_id FROM comments WHERE comment_id = $1 AND owner = $2"
            )
            .bind::<diesel::sql_types::Text, _>(&event.object_id)
            .bind::<diesel::sql_types::Text, _>(&event.owner)
            .get_result::<PostIdResult>(&mut conn)
            .await;
            
            if let Ok(post_id_row) = post_id_result {
                let post_id = post_id_row.post_id;
                
                // Mark comment as deleted
                diesel::update(schema::comments::table)
                    .filter(schema::comments::comment_id.eq(&event.object_id))
                    .filter(schema::comments::owner.eq(&event.owner))
                    .set(schema::comments::deleted_at.eq(event.deleted_at as i64))
                    .execute(&mut conn)
                    .await?;
                    
                // Decrement post comment count
                diesel::update(schema::posts::table)
                    .filter(schema::posts::post_id.eq(&post_id))
                    .set(schema::posts::comment_count.eq(schema::posts::comment_count - 1))
                    .execute(&mut conn)
                    .await?;
            }
        }
            
        info!("Successfully processed deletion: {}", event.object_id);
        Ok(())
    }
    
    // ============================================================================
    // POC EVENT PROCESSING METHODS
    // ============================================================================
    
    /// Process a PoC analysis submitted event
    async fn process_poc_analysis_submitted(&self, event: &crate::events::AnalysisSubmittedEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing PoC analysis submitted: {} by {}", event.post_id, event.oracle_address);
        
        // Validate the event
        crate::events::validate_analysis_submitted_event(event)?;
        
        // Convert event to database model
        let mut new_analysis = event.into_model()?;
        new_analysis.transaction_id = tx_id.to_string();
        
        // Insert into the database
        diesel::insert_into(crate::schema::poc_analysis_results::table)
            .values(new_analysis)
            .on_conflict((crate::schema::poc_analysis_results::post_id, crate::schema::poc_analysis_results::time))
            .do_update()
            .set(crate::schema::poc_analysis_results::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed PoC analysis submitted for post: {}", event.post_id);
        Ok(())
    }
    
    /// Process a PoC badge issued event
    async fn process_poc_badge_issued(&self, event: &crate::events::PocBadgeIssuedEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing PoC badge issued: {} for post {}", event.badge_id, event.post_id);
        
        // Validate the event
        crate::events::validate_badge_issued_event(event)?;
        
        // Convert event to database model
        let mut new_badge = event.into_model()?;
        new_badge.transaction_id = tx_id.to_string();
        
        // Insert the badge
        diesel::insert_into(crate::schema::poc_badges::table)
            .values(&new_badge)
            .on_conflict((crate::schema::poc_badges::badge_id, crate::schema::poc_badges::time))
            .do_update()
            .set(crate::schema::poc_badges::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        // Update the post with the badge ID
        diesel::update(crate::schema::posts::table)
            .filter(crate::schema::posts::post_id.eq(&event.post_id))
            .set(crate::schema::posts::poc_badge_id.eq(&event.badge_id))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed PoC badge issued: {}", event.badge_id);
        Ok(())
    }
    
    /// Process a revenue redirection activated event
    async fn process_poc_revenue_redirection_activated(&self, event: &crate::events::RevenueRedirectionActivatedEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing revenue redirection: {} for accused post {}", event.redirection_id, event.accused_post_id);
        
        // Validate the event
        crate::events::validate_redirection_activated_event(event)?;
        
        // Convert event to database model
        let mut new_redirection = event.into_model()?;
        new_redirection.transaction_id = tx_id.to_string();
        
        // Insert the redirection
        diesel::insert_into(crate::schema::poc_revenue_redirections::table)
            .values(&new_redirection)
            .on_conflict((crate::schema::poc_revenue_redirections::redirection_id, crate::schema::poc_revenue_redirections::time))
            .do_update()
            .set(crate::schema::poc_revenue_redirections::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        // Update the accused post with revenue redirection info
        diesel::update(crate::schema::posts::table)
            .filter(crate::schema::posts::post_id.eq(&event.accused_post_id))
            .set((
                crate::schema::posts::revenue_redirect_to.eq(&event.original_post_id),
                crate::schema::posts::revenue_redirect_percentage.eq(event.redirect_percentage as i64),
            ))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed revenue redirection: {}", event.redirection_id);
        Ok(())
    }
    
    /// Process a PoC dispute submitted event
    async fn process_poc_dispute_submitted(&self, event: &crate::events::PocDisputeSubmittedEvent, tx_id: &str, evidence: String) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing PoC dispute submitted: {} by {}", event.dispute_id, event.disputer);
        
        // Validate the event
        crate::events::validate_dispute_submitted_event(event)?;
        
        // Convert event to database model
        let mut new_dispute = event.into_model(evidence)?;
        new_dispute.transaction_id = tx_id.to_string();
        
        // Insert the dispute
        diesel::insert_into(crate::schema::poc_disputes::table)
            .values(new_dispute)
            .on_conflict((crate::schema::poc_disputes::dispute_id, crate::schema::poc_disputes::time))
            .do_update()
            .set(crate::schema::poc_disputes::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed PoC dispute submitted: {}", event.dispute_id);
        Ok(())
    }
    
    /// Process a dispute vote cast event
    async fn process_poc_dispute_vote_cast(&self, event: &crate::events::DisputeVoteCastEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing dispute vote cast: {} by {} for dispute {}", event.vote_choice, event.voter, event.dispute_id);
        
        // Validate the event
        crate::events::validate_vote_cast_event(event)?;
        
        // Convert event to database model
        let mut new_vote = event.into_model()?;
        new_vote.transaction_id = tx_id.to_string();
        
        // Insert the vote
        diesel::insert_into(crate::schema::poc_dispute_votes::table)
            .values(new_vote)
            .on_conflict((crate::schema::poc_dispute_votes::dispute_id, crate::schema::poc_dispute_votes::voter, crate::schema::poc_dispute_votes::time))
            .do_update()
            .set(crate::schema::poc_dispute_votes::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed dispute vote cast for dispute: {}", event.dispute_id);
        Ok(())
    }
    
    /// Process a PoC dispute resolved event
    async fn process_poc_dispute_resolved(&self, event: &crate::events::PocDisputeResolvedEvent, _tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing PoC dispute resolved: {} for post {}", event.dispute_id, event.post_id);
        
        // Get dispute update fields
        let (resolution, winning_side, total_winning_stake, total_losing_stake, resolved_at) = 
            event.get_dispute_update_fields();
        
        // Update the dispute with resolution
        diesel::update(crate::schema::poc_disputes::table)
            .filter(crate::schema::poc_disputes::dispute_id.eq(&event.dispute_id))
            .set((
                crate::schema::poc_disputes::status.eq(resolution),
                crate::schema::poc_disputes::resolution.eq(resolution),
                crate::schema::poc_disputes::winning_side.eq(winning_side),
                crate::schema::poc_disputes::total_winning_stake.eq(total_winning_stake),
                crate::schema::poc_disputes::total_losing_stake.eq(total_losing_stake),
                crate::schema::poc_disputes::resolved_at.eq(resolved_at),
            ))
            .execute(&mut conn)
            .await?;
            
        // If badge should be revoked, update the badge
        if event.should_revoke_badge() {
            diesel::update(crate::schema::poc_badges::table)
                .filter(crate::schema::poc_badges::post_id.eq(&event.post_id))
                .set((
                    crate::schema::poc_badges::revoked.eq(true),
                    crate::schema::poc_badges::revoked_at.eq(resolved_at),
                ))
                .execute(&mut conn)
                .await?;
                
            // Remove badge from post
            diesel::update(crate::schema::posts::table)
                .filter(crate::schema::posts::post_id.eq(&event.post_id))
                .set(crate::schema::posts::poc_badge_id.eq::<Option<String>>(None))
                .execute(&mut conn)
                .await?;
        }
        
        // If redirection should be removed, update redirections
        if event.should_remove_redirection() {
            diesel::update(crate::schema::poc_revenue_redirections::table)
                .filter(crate::schema::poc_revenue_redirections::accused_post_id.eq(&event.post_id))
                .set((
                    crate::schema::poc_revenue_redirections::removed.eq(true),
                    crate::schema::poc_revenue_redirections::removed_at.eq(resolved_at),
                ))
                .execute(&mut conn)
                .await?;
                
            // Remove redirection from post
            diesel::update(crate::schema::posts::table)
                .filter(crate::schema::posts::post_id.eq(&event.post_id))
                .set((
                    crate::schema::posts::revenue_redirect_to.eq::<Option<String>>(None),
                    crate::schema::posts::revenue_redirect_percentage.eq::<Option<i64>>(None),
                ))
                .execute(&mut conn)
                .await?;
        }
            
        info!("Successfully processed PoC dispute resolved: {}", event.dispute_id);
        Ok(())
    }
    
    /// Process a voting reward claimed event
    async fn process_poc_voting_reward_claimed(&self, event: &crate::events::VotingRewardClaimedEvent, _tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing voting reward claimed: {} claimed {} for dispute {}", 
               event.voter, event.reward_amount, event.dispute_id);
        
        // Get reward update fields
        let (reward_claimed, reward_amount) = event.get_reward_update_fields();
        
        // Update the vote with reward information
        diesel::update(crate::schema::poc_dispute_votes::table)
            .filter(crate::schema::poc_dispute_votes::dispute_id.eq(&event.dispute_id))
            .filter(crate::schema::poc_dispute_votes::voter.eq(&event.voter))
            .set((
                crate::schema::poc_dispute_votes::reward_claimed.eq(reward_claimed),
                crate::schema::poc_dispute_votes::reward_amount.eq(reward_amount),
            ))
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed voting reward claimed for dispute: {}", event.dispute_id);
        Ok(())
    }
    
    /// Process a PoC configuration updated event
    async fn process_poc_config_updated(&self, event: &crate::events::PocConfigUpdatedEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing PoC config updated by: {}", event.updated_by);
        
        // Validate the event
        crate::events::validate_config_updated_event(event)?;
        
        // Convert event to database model
        let mut new_config = event.into_model()?;
        new_config.transaction_id = tx_id.to_string();
        
        // Insert the new configuration
        diesel::insert_into(crate::schema::poc_configuration::table)
            .values(new_config)
            .execute(&mut conn)
            .await?;
            
        info!("Successfully processed PoC config updated");
        Ok(())
    }
    
    /// Process a token pool sync needed event
    async fn process_poc_token_pool_sync_needed(&self, event: &crate::events::TokenPoolSyncNeededEvent, _tx_id: &str) -> Result<()> {
        info!("Processing token pool sync needed for post: {}", event.get_post_id());
        
        // This event signals that a post's token pool needs to be synchronized
        // For now, we just log it - actual sync logic would be handled by the social proof token handler
        info!("Token pool sync needed for post: {} at timestamp: {}", 
               event.get_post_id(), event.get_timestamp());
        
        // Future implementation could trigger an update to the social proof token tables
        // or send a message to another handler responsible for token pool management
        
        info!("Successfully processed token pool sync needed");
        Ok(())
    }
    
    /// Start listening for post events
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting post event handler");
        
        while let Some(event) = self.rx.recv().await {
            debug!("Received blockchain event: {:?}", event);
            
            // Check if this is a post-related event
            if event.event_type.contains("::post::") || event.event_type.contains("::PostCreated") || 
               event.event_type.contains("::Comment") || event.event_type.contains("::Reaction") {
                info!("Processing post event: {}", event.event_type);
                
                let tx_id = event.tx_digest.clone();
                
                // Handle post created event
                if event.event_type.ends_with("::PostCreatedEvent") {
                    match parse_json_event::<PostCreatedEvent>(&event.data) {
                        Ok(post_event) => {
                            if let Err(e) = self.process_post_created(&post_event, &tx_id).await {
                                error!("Failed to process post created event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize post created event: {}", e);
                        }
                    }
                }
                // Handle comment created event
                else if event.event_type.ends_with("::CommentCreatedEvent") {
                    match parse_json_event::<CommentCreatedEvent>(&event.data) {
                        Ok(comment_event) => {
                            if let Err(e) = self.process_comment_created(&comment_event, &tx_id).await {
                                error!("Failed to process comment created event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize comment created event: {}", e);
                        }
                    }
                }
                // Handle reaction event
                else if event.event_type.ends_with("::ReactionEvent") {
                    match parse_json_event::<ReactionEvent>(&event.data) {
                        Ok(reaction_event) => {
                            if let Err(e) = self.process_reaction(&reaction_event, &tx_id).await {
                                error!("Failed to process reaction event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize reaction event: {}", e);
                        }
                    }
                }
                // Handle remove reaction event
                else if event.event_type.ends_with("::RemoveReactionEvent") {
                    match parse_json_event::<RemoveReactionEvent>(&event.data) {
                        Ok(remove_reaction_event) => {
                            if let Err(e) = self.process_remove_reaction(&remove_reaction_event, &tx_id).await {
                                error!("Failed to process remove reaction event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize remove reaction event: {}", e);
                        }
                    }
                }
                // Handle repost event
                else if event.event_type.ends_with("::RepostEvent") {
                    match parse_json_event::<RepostEvent>(&event.data) {
                        Ok(repost_event) => {
                            if let Err(e) = self.process_repost(&repost_event, &tx_id).await {
                                error!("Failed to process repost event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize repost event: {}", e);
                        }
                    }
                }
                // Handle tip event
                else if event.event_type.ends_with("::TipEvent") {
                    match parse_json_event::<TipEvent>(&event.data) {
                        Ok(tip_event) => {
                            if let Err(e) = self.process_tip(&tip_event, &tx_id).await {
                                error!("Failed to process tip event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize tip event: {}", e);
                        }
                    }
                }
                // Handle moderation event
                else if event.event_type.ends_with("::ModerationEvent") {
                    match parse_json_event::<PostModerationEvent>(&event.data) {
                        Ok(moderation_event) => {
                            if let Err(e) = self.process_moderation(&moderation_event, &tx_id).await {
                                error!("Failed to process moderation event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize moderation event: {}", e);
                        }
                    }
                }
                // Handle content update event
                else if event.event_type.ends_with("::ContentUpdateEvent") || 
                         event.event_type.ends_with("::PostUpdatedEvent") || 
                         event.event_type.ends_with("::CommentUpdatedEvent") {
                    match parse_json_event::<ContentUpdateEvent>(&event.data) {
                        Ok(update_event) => {
                            if let Err(e) = self.process_content_update(&update_event, &tx_id).await {
                                error!("Failed to process content update event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize content update event: {}", e);
                        }
                    }
                }
                // Handle report event
                else if event.event_type.ends_with("::ReportEvent") || 
                         event.event_type.ends_with("::PostReportedEvent") || 
                         event.event_type.ends_with("::CommentReportedEvent") {
                    match parse_json_event::<ReportEvent>(&event.data) {
                        Ok(report_event) => {
                            if let Err(e) = self.process_report(&report_event, &tx_id).await {
                                error!("Failed to process report event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize report event: {}", e);
                        }
                    }
                }
                // Handle deletion event
                else if event.event_type.ends_with("::DeletionEvent") || 
                         event.event_type.ends_with("::PostDeletedEvent") || 
                         event.event_type.ends_with("::CommentDeletedEvent") {
                    match parse_json_event::<PostDeletionEvent>(&event.data) {
                        Ok(deletion_event) => {
                            if let Err(e) = self.process_deletion(&deletion_event, &tx_id).await {
                                error!("Failed to process deletion event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize deletion event: {}", e);
                        }
                    }
                }
                // Handle PoC analysis submitted event
                else if event.event_type.ends_with("::AnalysisSubmittedEvent") {
                    match parse_json_event::<crate::events::AnalysisSubmittedEvent>(&event.data) {
                        Ok(analysis_event) => {
                            if let Err(e) = self.process_poc_analysis_submitted(&analysis_event, &tx_id).await {
                                error!("Failed to process PoC analysis submitted event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize PoC analysis submitted event: {}", e);
                        }
                    }
                }
                // Handle PoC badge issued event
                else if event.event_type.ends_with("::PocBadgeIssuedEvent") {
                    match parse_json_event::<crate::events::PocBadgeIssuedEvent>(&event.data) {
                        Ok(badge_event) => {
                            if let Err(e) = self.process_poc_badge_issued(&badge_event, &tx_id).await {
                                error!("Failed to process PoC badge issued event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize PoC badge issued event: {}", e);
                        }
                    }
                }
                // Handle revenue redirection activated event
                else if event.event_type.ends_with("::RevenueRedirectionActivatedEvent") {
                    match parse_json_event::<crate::events::RevenueRedirectionActivatedEvent>(&event.data) {
                        Ok(redirection_event) => {
                            if let Err(e) = self.process_poc_revenue_redirection_activated(&redirection_event, &tx_id).await {
                                error!("Failed to process revenue redirection activated event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize revenue redirection activated event: {}", e);
                        }
                    }
                }
                // Handle PoC dispute submitted event
                else if event.event_type.ends_with("::PocDisputeSubmittedEvent") {
                    match parse_json_event::<crate::events::PocDisputeSubmittedEvent>(&event.data) {
                        Ok(dispute_event) => {
                            // For now, use empty evidence string - in real implementation this would come from the event data
                            let evidence = "Dispute evidence data".to_string();
                            if let Err(e) = self.process_poc_dispute_submitted(&dispute_event, &tx_id, evidence).await {
                                error!("Failed to process PoC dispute submitted event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize PoC dispute submitted event: {}", e);
                        }
                    }
                }
                // Handle dispute vote cast event
                else if event.event_type.ends_with("::DisputeVoteCastEvent") {
                    match parse_json_event::<crate::events::DisputeVoteCastEvent>(&event.data) {
                        Ok(vote_event) => {
                            if let Err(e) = self.process_poc_dispute_vote_cast(&vote_event, &tx_id).await {
                                error!("Failed to process dispute vote cast event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize dispute vote cast event: {}", e);
                        }
                    }
                }
                // Handle PoC dispute resolved event
                else if event.event_type.ends_with("::PocDisputeResolvedEvent") {
                    match parse_json_event::<crate::events::PocDisputeResolvedEvent>(&event.data) {
                        Ok(resolved_event) => {
                            if let Err(e) = self.process_poc_dispute_resolved(&resolved_event, &tx_id).await {
                                error!("Failed to process PoC dispute resolved event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize PoC dispute resolved event: {}", e);
                        }
                    }
                }
                // Handle voting reward claimed event
                else if event.event_type.ends_with("::VotingRewardClaimedEvent") {
                    match parse_json_event::<crate::events::VotingRewardClaimedEvent>(&event.data) {
                        Ok(reward_event) => {
                            if let Err(e) = self.process_poc_voting_reward_claimed(&reward_event, &tx_id).await {
                                error!("Failed to process voting reward claimed event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize voting reward claimed event: {}", e);
                        }
                    }
                }
                // Handle PoC configuration updated event
                else if event.event_type.ends_with("::PocConfigUpdatedEvent") {
                    match parse_json_event::<crate::events::PocConfigUpdatedEvent>(&event.data) {
                        Ok(config_event) => {
                            if let Err(e) = self.process_poc_config_updated(&config_event, &tx_id).await {
                                error!("Failed to process PoC config updated event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize PoC config updated event: {}", e);
                        }
                    }
                }
                // Handle token pool sync needed event
                else if event.event_type.ends_with("::TokenPoolSyncNeededEvent") {
                    match parse_json_event::<crate::events::TokenPoolSyncNeededEvent>(&event.data) {
                        Ok(sync_event) => {
                            if let Err(e) = self.process_poc_token_pool_sync_needed(&sync_event, &tx_id).await {
                                error!("Failed to process token pool sync needed event: {}", e);
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize token pool sync needed event: {}", e);
                        }
                    }
                }
                
                // Update progress after processing the event
                if let Err(e) = self.update_progress(event.timestamp_ms).await {
                    error!("Failed to update progress: {}", e);
                }
            }
        }
        
        info!("Post event handler terminated");
        Ok(())
    }
}

// Helper struct for sql queries
#[derive(Debug, QueryableByName)]
struct ReactionTextResult {
    #[diesel(sql_type = diesel::sql_types::Text)]
    reaction_text: String,
}

// Helper struct for sql queries
#[derive(Debug, QueryableByName)]
struct PostIdResult {
    #[diesel(sql_type = diesel::sql_types::Text)]
    post_id: String,
}

/// Handle post-related events from the blockchain
pub async fn handle_event(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    let event_type = &event.type_.to_string(); // Convert StructTag to String
    
    info!("Processing post event: {}", event_type);
    
    // Process each event type
    if event_type.ends_with("::PostCreatedEvent") {
        handle_post_created(db, event, transaction_id).await?;
    } else if event_type.ends_with("::CommentCreatedEvent") {
        handle_comment_created(db, event, transaction_id).await?;
    } else if event_type.ends_with("::ReactionEvent") {
        handle_reaction(db, event, transaction_id).await?;
    } else if event_type.ends_with("::RemoveReactionEvent") {
        // Handle remove reaction event if needed
    } else if event_type.ends_with("::RepostEvent") {
        handle_repost(db, event, transaction_id).await?;
    } else if event_type.ends_with("::TipEvent") {
        handle_tip(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostModerationEvent") {
        handle_moderation(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostReportedEvent") || event_type.ends_with("::CommentReportedEvent") {
        handle_report(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostDeletedEvent") || event_type.ends_with("::CommentDeletedEvent") {
        handle_deletion(db, event, transaction_id).await?;
    } else {
        debug!("Unhandled post event type: {}", event_type);
    }
    
    Ok(())
}

/// Handle post created event
async fn handle_post_created(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing PostCreatedEvent");
    
    // Parse the event
    let parsed_event = parse_event::<PostCreatedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PostCreatedEvent: {}", e))?;
    
    info!("Parsed PostCreatedEvent: post_id={}", parsed_event.post_id);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // Convert event to model
    let mut new_post = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_post.transaction_id = transaction_id.to_string();
    
    // Insert the new post into the database with explicit field updates
    diesel::insert_into(posts::table)
        .values(&new_post)
        .on_conflict(posts::id) // Assuming id is unique
        .do_update()
        .set((
            posts::post_id.eq(&new_post.post_id),
            posts::content.eq(&new_post.content),
            posts::owner.eq(&new_post.owner),
            posts::media_urls.eq(&new_post.media_urls),
            posts::mentions.eq(&new_post.mentions),
            posts::metadata_json.eq(&new_post.metadata_json),
            posts::my_ip_id.eq(&new_post.my_ip_id),
            posts::created_at.eq(&new_post.created_at),
            posts::transaction_id.eq(transaction_id)
        ))
        .execute(&mut conn)
        .await?;
    
    info!("Processed PostCreatedEvent successfully");
    Ok(())
}

/// Handle comment created event
async fn handle_comment_created(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing CommentCreatedEvent");
    
    // Parse the event
    let parsed_event = parse_event::<CommentCreatedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse CommentCreatedEvent: {}", e))?;
    
    info!("Parsed CommentCreatedEvent: comment_id={}, post_id={}", 
          parsed_event.comment_id, parsed_event.post_id);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    
    // Convert event to model
    let mut new_comment = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_comment.transaction_id = transaction_id.to_string();
    
    // Insert the new comment into the database with explicit field updates
    diesel::insert_into(comments::table)
        .values(&new_comment)
        .on_conflict(comments::id) // Assuming id is unique
        .do_update()
        .set((
            comments::comment_id.eq(&new_comment.comment_id),
            comments::post_id.eq(&new_comment.post_id),
            comments::parent_comment_id.eq(&new_comment.parent_comment_id),
            comments::content.eq(&new_comment.content),
            comments::owner.eq(&new_comment.owner),
            comments::media_urls.eq(&new_comment.media_urls),
            comments::mentions.eq(&new_comment.mentions),
            comments::metadata_json.eq(&new_comment.metadata_json),
            comments::created_at.eq(&new_comment.created_at),
            comments::transaction_id.eq(transaction_id)
        ))
        .execute(&mut conn)
        .await?;
    
    info!("Processed CommentCreatedEvent successfully");
    Ok(())
}

/// Handle reaction event
async fn handle_reaction(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing ReactionEvent");
    
    // Parse the event
    let parsed_event = parse_event::<ReactionEvent>(event)
        .map_err(|e| anyhow!("Failed to parse ReactionEvent: {}", e))?;
    
    info!("Parsed ReactionEvent: object_id={}, reaction={}", 
          parsed_event.object_id, parsed_event.reaction_text);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // TODO: MyIP marketplace permissions will be handled by marketplace events
    // For now, allow all reactions - marketplace restrictions will be applied separately
    
    // Convert event to model
    let mut new_reaction = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_reaction.transaction_id = transaction_id.to_string();
    
    // Insert the reaction
    diesel::insert_into(reactions::table)
        .values(&new_reaction)
        .execute(&mut conn)
        .await?;
    
    // Update or insert the reaction count
    let reaction_count = parsed_event.into_reaction_count()?;
    
    diesel::insert_into(reaction_counts::table)
        .values(&reaction_count)
        .on_conflict((reaction_counts::object_id, reaction_counts::reaction_text))
        .do_update()
        .set(reaction_counts::count.eq(reaction_counts::count + 1))
        .execute(&mut conn)
        .await?;
    
    // Update the reaction count on the post or comment
    if parsed_event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&parsed_event.object_id))
            .set(posts::reaction_count.eq(posts::reaction_count + 1))
            .execute(&mut conn)
            .await?;
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&parsed_event.object_id))
            .set(comments::reaction_count.eq(comments::reaction_count + 1))
            .execute(&mut conn)
            .await?;
    }
    
    info!("Processed ReactionEvent successfully");
    Ok(())
}

/// Handle repost event
async fn handle_repost(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing RepostEvent");
    
    // Parse the event
    let parsed_event = parse_event::<RepostEvent>(event)
        .map_err(|e| anyhow!("Failed to parse RepostEvent: {}", e))?;
    
    info!("Parsed RepostEvent: repost_id={}, original_id={}", 
          parsed_event.repost_id, parsed_event.original_id);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // TODO: MyIP marketplace permissions will be handled by marketplace events
    // For now, allow all reposts - marketplace restrictions will be applied separately
    
    // Convert event to model
    let mut new_repost = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_repost.transaction_id = transaction_id.to_string();
    
    // Insert the repost with explicit field updates
    diesel::insert_into(reposts::table)
        .values(&new_repost)
        .on_conflict(reposts::id)
        .do_update()
        .set((
            reposts::repost_id.eq(&new_repost.repost_id),
            reposts::original_id.eq(&new_repost.original_id),
            reposts::original_post_id.eq(&new_repost.original_post_id),
            reposts::is_original_post.eq(&new_repost.is_original_post),
            reposts::owner.eq(&new_repost.owner),
            reposts::created_at.eq(&new_repost.created_at),
            reposts::transaction_id.eq(transaction_id)
        ))
        .execute(&mut conn)
        .await?;
    
    // Update the repost count on the original post
    if parsed_event.is_original_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&parsed_event.original_id))
            .set(posts::repost_count.eq(posts::repost_count + 1))
            .execute(&mut conn)
            .await?;
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&parsed_event.original_id))
            .set(comments::repost_count.eq(comments::repost_count + 1))
            .execute(&mut conn)
            .await?;
    }
    
    info!("Processed RepostEvent successfully");
    Ok(())
}

/// Handle tip event
async fn handle_tip(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing TipEvent");
    
    // Parse the event
    let parsed_event = parse_event::<TipEvent>(event)
        .map_err(|e| anyhow!("Failed to parse TipEvent: {}", e))?;
    
    info!("Parsed TipEvent: from={}, to={}, amount={}", 
          parsed_event.from, parsed_event.to, parsed_event.amount);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // TODO: MyIP marketplace permissions and revenue redirection will be handled by marketplace events
    // For now, allow all tips - marketplace restrictions will be applied separately
    
    // Convert event to model
    let mut new_tip = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_tip.transaction_id = transaction_id.to_string();
    
    // Insert the tip
    diesel::insert_into(tips::table)
        .values(&new_tip)
        .execute(&mut conn)
        .await?;
    
    // Update the tips received amount on the post or comment
    if parsed_event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&parsed_event.object_id))
            .set(posts::tips_received.eq(posts::tips_received + parsed_event.amount as i64))
            .execute(&mut conn)
            .await?;
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&parsed_event.object_id))
            .set(comments::tips_received.eq(comments::tips_received + parsed_event.amount as i64))
            .execute(&mut conn)
            .await?;
    }
    
    // Create unified revenue record for the tip
    let unified_revenue = parsed_event.create_unified_revenue_record(transaction_id.to_string())?;
    
    diesel::insert_into(crate::schema::unified_revenue::table)
        .values(&unified_revenue)
        .execute(&mut conn)
        .await?;
    
    info!("Processed TipEvent with revenue tracking successfully");
    Ok(())
}

/// Handle moderation event
async fn handle_moderation(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing PostModerationEvent");
    
    // Parse the event
    let parsed_event = parse_event::<PostModerationEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PostModerationEvent: {}", e))?;
    
    info!("Parsed PostModerationEvent: object_id={}, platform_id={}, removed={}", 
          parsed_event.object_id, parsed_event.platform_id, parsed_event.removed);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // Convert event to model
    let mut new_moderation = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_moderation.transaction_id = transaction_id.to_string();
    
    // Insert the moderation event
    diesel::insert_into(posts_moderation_events::table)
        .values(&new_moderation)
        .execute(&mut conn)
        .await?;
    
    // Update the post or comment removed status
    // (Assumes we have a way to tell if this is for a post or comment)
    match true { // Change based on how to detect if this is post or comment
        true => {
            diesel::update(posts::table)
                .filter(posts::post_id.eq(&parsed_event.object_id))
                .set((
                    posts::removed_from_platform.eq(parsed_event.removed),
                    posts::removed_by.eq(Some(parsed_event.moderated_by.clone()))
                ))
                .execute(&mut conn)
                .await?;
        },
        false => {
            diesel::update(comments::table)
                .filter(comments::comment_id.eq(&parsed_event.object_id))
                .set((
                    comments::removed_from_platform.eq(parsed_event.removed),
                    comments::removed_by.eq(Some(parsed_event.moderated_by.clone()))
                ))
                .execute(&mut conn)
                .await?;
        }
    }
    
    info!("Processed PostModerationEvent successfully");
    Ok(())
}

/// Handle report event
async fn handle_report(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing ReportEvent");
    
    // Parse the event
    let parsed_event = parse_event::<ReportEvent>(event)
        .map_err(|e| anyhow!("Failed to parse ReportEvent: {}", e))?;
    
    info!("Parsed ReportEvent: object_id={}, reporter={}, reason={}", 
          parsed_event.object_id, parsed_event.reporter, parsed_event.reason_code);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // Convert event to model
    let mut new_report = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_report.transaction_id = transaction_id.to_string();
    
    // Insert the report
    diesel::insert_into(posts_reports::table)
        .values(&new_report)
        .execute(&mut conn)
        .await?;
    
    info!("Processed ReportEvent successfully");
    Ok(())
}

/// Handle deletion event
async fn handle_deletion(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    info!("Processing DeletionEvent");
    
    // Parse the event
    let parsed_event = parse_event::<PostDeletionEvent>(event)
        .map_err(|e| anyhow!("Failed to parse DeletionEvent: {}", e))?;
    
    info!("Parsed DeletionEvent: object_id={}, is_post={}", 
          parsed_event.object_id, parsed_event.is_post);
    
    // Get a database connection
    let mut conn = db.get_connection().await?;
    
    // Convert event to model
    let mut new_deletion = parsed_event.into_model()?;
    
    // Set the transaction ID
    new_deletion.transaction_id = transaction_id.to_string();
    
    // Insert the deletion event
    diesel::insert_into(posts_deletion_events::table)
        .values(&new_deletion)
        .execute(&mut conn)
        .await?;
    
    // Update the post or comment deleted_at field
    if parsed_event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&parsed_event.object_id))
            .set(posts::deleted_at.eq(Some(parsed_event.deleted_at as i64)))
            .execute(&mut conn)
            .await?;
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&parsed_event.object_id))
            .set(comments::deleted_at.eq(Some(parsed_event.deleted_at as i64)))
            .execute(&mut conn)
            .await?;
    }
    
    info!("Processed DeletionEvent successfully");
    Ok(())
} 