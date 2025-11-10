// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::db::{Database, DbConnection};
// Import event types specifically to avoid ambiguity
use crate::events::post_event_types::{
    CommentCreatedEvent, ContentUpdateEvent, DeletionEvent as PostDeletionEvent,
    ModerationEvent as PostModerationEvent, PostCreatedEvent, PromotedPostCreatedEvent,
    PromotedPostViewConfirmedEvent, PromotionFundsWithdrawnEvent, PromotionStatusToggledEvent,
    ReactionEvent, RemoveReactionEvent, ReportEvent, RepostEvent, TipEvent,
    OwnershipTransferEvent, PredictionCreatedEvent, PredictionBetPlacedEvent,
    PredictionResolvedEvent, PredictionPayoutEvent, PredictionBetWithdrawnEvent,
    PostParametersUpdatedEvent,
};
use crate::events::{event_utils::parse_json_event, parse_event};
use crate::models::indexer::NewIndexerProgress;
use crate::schema;
use mys_types::event::Event as MysEvent;

use crate::schema::{
    comments, posts, posts_deletion_events, posts_moderation_events, posts_reports, promoted_posts,
    promotion_budget_events, promotion_status_events, promotion_views, reaction_counts, reactions,
    reposts, tips,
};

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
        Self { db, rx, worker_id }
    }

    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
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
                schema::indexer_progress::last_checkpoint_processed
                    .eq(progress.last_checkpoint_processed),
                schema::indexer_progress::last_processed_at.eq(progress.last_processed_at),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Process a post created event
    async fn process_post_created(&self, event: &PostCreatedEvent, tx_id: &str, timestamp_ms: Option<u64>) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!("Processing post created: {}", event.post_id);

        // Convert event to database model
        let mut new_post = event.into_model()?;
        
        // If created_at is 0 (missing from event), use blockchain event timestamp
        if new_post.created_at == 0 {
            if let Some(ts_ms) = timestamp_ms {
                new_post.created_at = (ts_ms / 1000) as i64;
                // Update the ID to use the timestamp
                new_post.id = format!("{}:{}", event.post_id, new_post.created_at);
            }
        }
        
        new_post.transaction_id = tx_id.to_string();

        // Insert into the database
        diesel::insert_into(schema::posts::table)
            .values(new_post)
            .on_conflict(schema::posts::id)
            .do_update()
            .set(schema::posts::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;

        // Increment post_count for the profile (with graceful error handling)
        match diesel::update(crate::schema::profiles::table)
            .filter(crate::schema::profiles::owner_address.eq(&event.owner))
            .set(crate::schema::profiles::post_count.eq(crate::schema::profiles::post_count + 1))
            .execute(&mut conn)
            .await
        {
            Ok(updated_rows) => {
                if updated_rows > 0 {
                    info!(
                        "Successfully incremented post_count for profile: {}",
                        event.owner
                    );
                } else {
                    warn!(
                        "No profile found to increment post_count for owner: {}",
                        event.owner
                    );
                }
            }
            Err(e) => {
                // Log error but don't fail the transaction
                error!(
                    "Failed to increment post_count for profile {}: {}. Post creation succeeded.",
                    event.owner, e
                );
            }
        }

        info!("Successfully processed post created: {}", event.post_id);
        Ok(())
    }

    /// Process a comment created event
    async fn process_comment_created(
        &self,
        event: &CommentCreatedEvent,
        tx_id: &str,
    ) -> Result<()> {
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

        info!(
            "Successfully processed comment created: {}",
            event.comment_id
        );
        Ok(())
    }

    /// Process a reaction event
    async fn process_reaction(&self, event: &ReactionEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing reaction: {} {} {}",
            event.user_address, event.reaction_text, event.object_id
        );

        // Convert event to database model
        let mut new_reaction = event.into_model()?;
        new_reaction.transaction_id = tx_id.to_string();

        // Insert into reactions table (replacing existing if needed)
        diesel::insert_into(schema::reactions::table)
            .values(new_reaction)
            .on_conflict((
                schema::reactions::object_id,
                schema::reactions::user_address,
            ))
            .do_update()
            .set(schema::reactions::transaction_id.eq(tx_id))
            .execute(&mut conn)
            .await?;

        // Update or insert into reaction_counts
        let reaction_count = event.into_reaction_count()?;

        diesel::insert_into(schema::reaction_counts::table)
            .values(reaction_count)
            .on_conflict((
                schema::reaction_counts::object_id,
                schema::reaction_counts::reaction_text,
            ))
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
    async fn process_remove_reaction(
        &self,
        event: &RemoveReactionEvent,
        _tx_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing remove reaction: {} {} {}",
            event.user_address, event.reaction_text, event.object_id
        );

        // First get the reaction to be removed to know the reaction_text
        let reaction_row = diesel::sql_query(
            "SELECT reaction_text FROM reactions WHERE object_id = $1 AND user_address = $2",
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
            info!(
                "No reaction found to remove for user {} on object {}",
                event.user_address, event.object_id
            );
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

        info!(
            "Processing tip: {} to {} for {}",
            event.from, event.to, event.object_id
        );

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
                .set(
                    schema::posts::tips_received
                        .eq(schema::posts::tips_received + event.amount as i64),
                )
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.object_id))
                .set(
                    schema::comments::tips_received
                        .eq(schema::comments::tips_received + event.amount as i64),
                )
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

        info!(
            "Processing moderation: {} by {}",
            event.object_id, event.moderated_by
        );

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
        let media_urls_json = event
            .media_urls
            .as_ref()
            .map(|urls| serde_json::to_value(urls).unwrap_or(serde_json::json!(null)));

        let mentions_json = event
            .mentions
            .as_ref()
            .map(|mentions| serde_json::to_value(mentions).unwrap_or(serde_json::json!(null)));

        // Parse metadata JSON if present
        let metadata_json = event
            .metadata_json
            .as_ref()
            .map(|json_str| serde_json::from_str(json_str).unwrap_or(serde_json::json!(null)));

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

        info!(
            "Processing report: {} by {}",
            event.object_id, event.reporter
        );

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

            // Decrement post_count for the profile (with graceful error handling)
            match diesel::update(crate::schema::profiles::table)
                .filter(crate::schema::profiles::owner_address.eq(&event.owner))
                .set(
                    crate::schema::profiles::post_count.eq(crate::schema::profiles::post_count - 1),
                )
                .execute(&mut conn)
                .await
            {
                Ok(updated_rows) => {
                    if updated_rows > 0 {
                        info!(
                            "Successfully decremented post_count for profile: {}",
                            event.owner
                        );
                    } else {
                        warn!(
                            "No profile found to decrement post_count for owner: {}",
                            event.owner
                        );
                    }
                }
                Err(e) => {
                    // Log error but don't fail the transaction
                    error!("Failed to decrement post_count for profile {}: {}. Post deletion succeeded.", event.owner, e);
                }
            }
        } else {
            // Get post_id to update comment count
            let post_id_result = diesel::sql_query(
                "SELECT post_id FROM comments WHERE comment_id = $1 AND owner = $2",
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

    /// Process an ownership transfer event
    async fn process_ownership_transfer(&self, event: &OwnershipTransferEvent, tx_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing ownership transfer: {} from {} to {}",
            event.object_id, event.previous_owner, event.new_owner
        );

        if event.is_post {
            diesel::update(schema::posts::table)
                .filter(schema::posts::post_id.eq(&event.object_id))
                .set(schema::posts::owner.eq(&event.new_owner))
                .execute(&mut conn)
                .await?;
        } else {
            diesel::update(schema::comments::table)
                .filter(schema::comments::comment_id.eq(&event.object_id))
                .set(schema::comments::owner.eq(&event.new_owner))
                .execute(&mut conn)
                .await?;
        }

        info!("Successfully processed ownership transfer");
        Ok(())
    }

    /// Process a promoted post created event
    async fn process_promoted_post_created(
        &self,
        event: &PromotedPostCreatedEvent,
        tx_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!("Processing promoted post created: {}", event.post_id);

        // Generate unique promotion_id using timestamp and post_id
        let promotion_id = format!(
            "promo_{}_{}",
            event.created_at,
            event.post_id.replace("0x", "")
        );

        // Create promoted post record
        diesel::insert_into(promoted_posts::table)
            .values((
                promoted_posts::promotion_id.eq(&promotion_id),
                promoted_posts::post_id.eq(&event.post_id),
                promoted_posts::owner.eq(&event.owner),
                promoted_posts::profile_id.eq(&event.profile_id),
                promoted_posts::payment_per_view.eq(event.payment_per_view as i64),
                promoted_posts::total_budget.eq(event.total_budget as i64),
                promoted_posts::remaining_budget.eq(event.total_budget as i64),
                promoted_posts::active.eq(false), // Starts inactive until platform approves
                promoted_posts::created_at.eq(event.created_at as i64),
                promoted_posts::transaction_id.eq(tx_id),
            ))
            .execute(&mut conn)
            .await?;

        // Update post with promotion_id
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.post_id))
            .set(posts::promotion_id.eq(&promotion_id))
            .execute(&mut conn)
            .await?;

        // Create initial budget event
        diesel::insert_into(promotion_budget_events::table)
            .values((
                promotion_budget_events::promotion_id.eq(&promotion_id),
                promotion_budget_events::post_id.eq(&event.post_id),
                promotion_budget_events::event_type.eq("initial_deposit"),
                promotion_budget_events::amount.eq(event.total_budget as i64),
                promotion_budget_events::remaining_budget.eq(event.total_budget as i64),
                promotion_budget_events::timestamp.eq(event.created_at as i64),
                promotion_budget_events::transaction_id.eq(tx_id),
            ))
            .execute(&mut conn)
            .await?;

        info!(
            "Successfully processed promoted post created: {}",
            event.post_id
        );
        Ok(())
    }

    /// Process a promoted post view confirmed event
    async fn process_promoted_post_view_confirmed(
        &self,
        event: &PromotedPostViewConfirmedEvent,
        tx_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing promoted post view confirmed: {} by {}",
            event.post_id, event.viewer
        );

        // Get promotion_id from post
        let promotion_id_result: Option<String> = posts::table
            .filter(posts::post_id.eq(&event.post_id))
            .select(posts::promotion_id)
            .first(&mut conn)
            .await?;

        if let Some(promotion_id) = promotion_id_result {
            // Record the view
            diesel::insert_into(promotion_views::table)
                .values((
                    promotion_views::post_id.eq(&event.post_id),
                    promotion_views::promotion_id.eq(&promotion_id),
                    promotion_views::viewer.eq(&event.viewer),
                    promotion_views::payment_amount.eq(event.payment_amount as i64),
                    promotion_views::view_duration.eq(event.view_duration as i64),
                    promotion_views::platform_id.eq(&event.platform_id),
                    promotion_views::timestamp.eq(event.timestamp as i64),
                    promotion_views::transaction_id.eq(tx_id),
                ))
                .execute(&mut conn)
                .await?;

            // Update remaining budget
            diesel::update(promoted_posts::table)
                .filter(promoted_posts::promotion_id.eq(&promotion_id))
                .set(
                    promoted_posts::remaining_budget
                        .eq(promoted_posts::remaining_budget - event.payment_amount as i64),
                )
                .execute(&mut conn)
                .await?;

            // Create budget event for the payment
            diesel::insert_into(promotion_budget_events::table)
                .values((
                    promotion_budget_events::promotion_id.eq(&promotion_id),
                    promotion_budget_events::post_id.eq(&event.post_id),
                    promotion_budget_events::event_type.eq("view_payment"),
                    promotion_budget_events::amount.eq(event.payment_amount as i64),
                    promotion_budget_events::remaining_budget.eq(promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(&promotion_id))
                        .select(promoted_posts::remaining_budget)
                        .first::<i64>(&mut conn)
                        .await?),
                    promotion_budget_events::timestamp.eq(event.timestamp as i64),
                    promotion_budget_events::transaction_id.eq(tx_id),
                ))
                .execute(&mut conn)
                .await?;
        }

        info!(
            "Successfully processed promoted post view confirmed: {}",
            event.post_id
        );
        Ok(())
    }

    /// Process a promotion status toggled event
    async fn process_promotion_status_toggled(
        &self,
        event: &PromotionStatusToggledEvent,
        tx_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing promotion status toggled: {} to {}",
            event.post_id, event.new_status
        );

        // Get promotion_id from post
        let promotion_id_result: Option<String> = posts::table
            .filter(posts::post_id.eq(&event.post_id))
            .select(posts::promotion_id)
            .first(&mut conn)
            .await?;

        if let Some(promotion_id) = promotion_id_result {
            // Update promotion status
            diesel::update(promoted_posts::table)
                .filter(promoted_posts::promotion_id.eq(&promotion_id))
                .set(promoted_posts::active.eq(event.new_status))
                .execute(&mut conn)
                .await?;

            // Create status event
            diesel::insert_into(promotion_status_events::table)
                .values((
                    promotion_status_events::post_id.eq(&event.post_id),
                    promotion_status_events::promotion_id.eq(&promotion_id),
                    promotion_status_events::event_type.eq("status_toggled"),
                    promotion_status_events::triggered_by.eq(&event.toggled_by),
                    promotion_status_events::new_status.eq(Some(event.new_status)),
                    promotion_status_events::timestamp.eq(event.timestamp as i64),
                    promotion_status_events::transaction_id.eq(tx_id),
                ))
                .execute(&mut conn)
                .await?;
        }

        info!(
            "Successfully processed promotion status toggled: {}",
            event.post_id
        );
        Ok(())
    }

    /// Process a promotion funds withdrawn event
    async fn process_promotion_funds_withdrawn(
        &self,
        event: &PromotionFundsWithdrawnEvent,
        tx_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing promotion funds withdrawn: {} amount: {}",
            event.post_id, event.withdrawn_amount
        );

        // Get promotion_id from post
        let promotion_id_result: Option<String> = posts::table
            .filter(posts::post_id.eq(&event.post_id))
            .select(posts::promotion_id)
            .first(&mut conn)
            .await?;

        if let Some(promotion_id) = promotion_id_result {
            // Update promotion - set active to false and remaining budget to 0
            diesel::update(promoted_posts::table)
                .filter(promoted_posts::promotion_id.eq(&promotion_id))
                .set((
                    promoted_posts::active.eq(false),
                    promoted_posts::remaining_budget.eq(0),
                ))
                .execute(&mut conn)
                .await?;

            // Create status event
            diesel::insert_into(promotion_status_events::table)
                .values((
                    promotion_status_events::post_id.eq(&event.post_id),
                    promotion_status_events::promotion_id.eq(&promotion_id),
                    promotion_status_events::event_type.eq("funds_withdrawn"),
                    promotion_status_events::triggered_by.eq(&event.owner),
                    promotion_status_events::new_status.eq(Some(false)),
                    promotion_status_events::amount.eq(Some(event.withdrawn_amount as i64)),
                    promotion_status_events::timestamp.eq(event.timestamp as i64),
                    promotion_status_events::transaction_id.eq(tx_id),
                ))
                .execute(&mut conn)
                .await?;

            // Create budget event
            diesel::insert_into(promotion_budget_events::table)
                .values((
                    promotion_budget_events::promotion_id.eq(&promotion_id),
                    promotion_budget_events::post_id.eq(&event.post_id),
                    promotion_budget_events::event_type.eq("withdrawal"),
                    promotion_budget_events::amount.eq(event.withdrawn_amount as i64),
                    promotion_budget_events::remaining_budget.eq(0),
                    promotion_budget_events::timestamp.eq(event.timestamp as i64),
                    promotion_budget_events::transaction_id.eq(tx_id),
                ))
                .execute(&mut conn)
                .await?;
        }

        info!(
            "Successfully processed promotion funds withdrawn: {}",
            event.post_id
        );
        Ok(())
    }

    /// Start listening for post events
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting post event handler");

        while let Some(event) = self.rx.recv().await {
            debug!("Received blockchain event: {:?}", event);

            // Check if this is a post-related event
            if event.event_type.contains("::post::")
                || event.event_type.contains("::PostCreated")
                || event.event_type.contains("::Comment")
                || event.event_type.contains("::Reaction")
            {
                info!("Processing post event: {}", event.event_type);

                let tx_id = event.tx_digest.clone();

                // Handle post created event
                if event.event_type.ends_with("::PostCreatedEvent") {
                    match parse_json_event::<PostCreatedEvent>(&event.data) {
                        Ok(post_event) => {
                            if let Err(e) = self.process_post_created(&post_event, &tx_id, Some(event.timestamp_ms)).await {
                                error!("Failed to process post created event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize post created event: {}", e);
                        }
                    }
                }
                // Handle comment created event
                else if event.event_type.ends_with("::CommentCreatedEvent") {
                    match parse_json_event::<CommentCreatedEvent>(&event.data) {
                        Ok(comment_event) => {
                            if let Err(e) =
                                self.process_comment_created(&comment_event, &tx_id).await
                            {
                                error!("Failed to process comment created event: {}", e);
                            }
                        }
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
                        }
                        Err(e) => {
                            error!("Failed to deserialize reaction event: {}", e);
                        }
                    }
                }
                // Handle remove reaction event
                else if event.event_type.ends_with("::RemoveReactionEvent") {
                    match parse_json_event::<RemoveReactionEvent>(&event.data) {
                        Ok(remove_reaction_event) => {
                            if let Err(e) = self
                                .process_remove_reaction(&remove_reaction_event, &tx_id)
                                .await
                            {
                                error!("Failed to process remove reaction event: {}", e);
                            }
                        }
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
                        }
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
                        }
                        Err(e) => {
                            error!("Failed to deserialize tip event: {}", e);
                        }
                    }
                }
                // Handle moderation event
                else if event.event_type.ends_with("::ModerationEvent") {
                    match parse_json_event::<PostModerationEvent>(&event.data) {
                        Ok(moderation_event) => {
                            if let Err(e) = self.process_moderation(&moderation_event, &tx_id).await
                            {
                                error!("Failed to process moderation event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize moderation event: {}", e);
                        }
                    }
                }
                // Handle content update event
                else if event.event_type.ends_with("::ContentUpdateEvent")
                    || event.event_type.ends_with("::PostUpdatedEvent")
                    || event.event_type.ends_with("::CommentUpdatedEvent")
                {
                    match parse_json_event::<ContentUpdateEvent>(&event.data) {
                        Ok(update_event) => {
                            if let Err(e) = self.process_content_update(&update_event, &tx_id).await
                            {
                                error!("Failed to process content update event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize content update event: {}", e);
                        }
                    }
                }
                // Handle report event
                else if event.event_type.ends_with("::ReportEvent")
                    || event.event_type.ends_with("::PostReportedEvent")
                    || event.event_type.ends_with("::CommentReportedEvent")
                {
                    match parse_json_event::<ReportEvent>(&event.data) {
                        Ok(report_event) => {
                            if let Err(e) = self.process_report(&report_event, &tx_id).await {
                                error!("Failed to process report event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize report event: {}", e);
                        }
                    }
                }
                // Handle deletion event
                else if event.event_type.ends_with("::DeletionEvent")
                    || event.event_type.ends_with("::PostDeletedEvent")
                    || event.event_type.ends_with("::CommentDeletedEvent")
                {
                    match parse_json_event::<PostDeletionEvent>(&event.data) {
                        Ok(deletion_event) => {
                            if let Err(e) = self.process_deletion(&deletion_event, &tx_id).await {
                                error!("Failed to process deletion event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize deletion event: {}", e);
                        }
                    }
                }
                // Handle promoted post created event
                else if event.event_type.ends_with("::PromotedPostCreatedEvent") {
                    match parse_json_event::<PromotedPostCreatedEvent>(&event.data) {
                        Ok(promoted_post_event) => {
                            if let Err(e) = self
                                .process_promoted_post_created(&promoted_post_event, &tx_id)
                                .await
                            {
                                error!("Failed to process promoted post created event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize promoted post created event: {}", e);
                        }
                    }
                }
                // Handle post updated event
                else if event.event_type.ends_with("::PostUpdatedEvent") {
                    match parse_json_event::<ContentUpdateEvent>(&event.data) {
                        Ok(update_event) => {
                            if let Err(e) = self.process_content_update(&update_event, &tx_id).await {
                                error!("Failed to process post updated event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize post updated event: {}", e);
                        }
                    }
                }
                // Handle comment updated event
                else if event.event_type.ends_with("::CommentUpdatedEvent") {
                    match parse_json_event::<ContentUpdateEvent>(&event.data) {
                        Ok(update_event) => {
                            if let Err(e) = self.process_content_update(&update_event, &tx_id).await {
                                error!("Failed to process comment updated event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize comment updated event: {}", e);
                        }
                    }
                }
                // Handle ownership transfer event
                else if event.event_type.ends_with("::OwnershipTransferEvent") {
                    match parse_json_event::<OwnershipTransferEvent>(&event.data) {
                        Ok(transfer_event) => {
                            if let Err(e) = self.process_ownership_transfer(&transfer_event, &tx_id).await {
                                error!("Failed to process ownership transfer event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize ownership transfer event: {}", e);
                        }
                    }
                }
                // Handle prediction events (logged for now, can be extended later)
                else if event.event_type.ends_with("::PredictionCreatedEvent") {
                    match parse_json_event::<PredictionCreatedEvent>(&event.data) {
                        Ok(prediction_event) => {
                            info!("Prediction created: post_id={}, prediction_data_id={}", 
                                prediction_event.post_id, prediction_event.prediction_data_id);
                        }
                        Err(e) => {
                            error!("Failed to deserialize prediction created event: {}", e);
                        }
                    }
                }
                else if event.event_type.ends_with("::PredictionBetPlacedEvent") {
                    match parse_json_event::<PredictionBetPlacedEvent>(&event.data) {
                        Ok(bet_event) => {
                            info!("Prediction bet placed: post_id={}, user={}, amount={}", 
                                bet_event.post_id, bet_event.user, bet_event.amount);
                        }
                        Err(e) => {
                            error!("Failed to deserialize prediction bet placed event: {}", e);
                        }
                    }
                }
                else if event.event_type.ends_with("::PredictionResolvedEvent") {
                    match parse_json_event::<PredictionResolvedEvent>(&event.data) {
                        Ok(resolved_event) => {
                            info!("Prediction resolved: post_id={}, winning_option_id={}", 
                                resolved_event.post_id, resolved_event.winning_option_id);
                        }
                        Err(e) => {
                            error!("Failed to deserialize prediction resolved event: {}", e);
                        }
                    }
                }
                else if event.event_type.ends_with("::PredictionPayoutEvent") {
                    match parse_json_event::<PredictionPayoutEvent>(&event.data) {
                        Ok(payout_event) => {
                            info!("Prediction payout: post_id={}, user={}, amount={}", 
                                payout_event.post_id, payout_event.user, payout_event.amount);
                        }
                        Err(e) => {
                            error!("Failed to deserialize prediction payout event: {}", e);
                        }
                    }
                }
                else if event.event_type.ends_with("::PredictionBetWithdrawnEvent") {
                    match parse_json_event::<PredictionBetWithdrawnEvent>(&event.data) {
                        Ok(withdrawn_event) => {
                            info!("Prediction bet withdrawn: post_id={}, user={}, withdrawal_amount={}", 
                                withdrawn_event.post_id, withdrawn_event.user, withdrawn_event.withdrawal_amount);
                        }
                        Err(e) => {
                            error!("Failed to deserialize prediction bet withdrawn event: {}", e);
                        }
                    }
                }
                // Handle post parameters updated event
                else if event.event_type.ends_with("::PostParametersUpdatedEvent") {
                    match parse_json_event::<PostParametersUpdatedEvent>(&event.data) {
                        Ok(params_event) => {
                            info!("Post parameters updated by: {}", params_event.updated_by);
                        }
                        Err(e) => {
                            error!("Failed to deserialize post parameters updated event: {}", e);
                        }
                    }
                }
                // Handle promoted post view confirmed event
                else if event
                    .event_type
                    .ends_with("::PromotedPostViewConfirmedEvent")
                {
                    match parse_json_event::<PromotedPostViewConfirmedEvent>(&event.data) {
                        Ok(view_event) => {
                            if let Err(e) = self
                                .process_promoted_post_view_confirmed(&view_event, &tx_id)
                                .await
                            {
                                error!(
                                    "Failed to process promoted post view confirmed event: {}",
                                    e
                                );
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to deserialize promoted post view confirmed event: {}",
                                e
                            );
                        }
                    }
                }
                // Handle promotion status toggled event
                else if event.event_type.ends_with("::PromotionStatusToggledEvent") {
                    match parse_json_event::<PromotionStatusToggledEvent>(&event.data) {
                        Ok(status_event) => {
                            if let Err(e) = self
                                .process_promotion_status_toggled(&status_event, &tx_id)
                                .await
                            {
                                error!("Failed to process promotion status toggled event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to deserialize promotion status toggled event: {}",
                                e
                            );
                        }
                    }
                }
                // Handle promotion funds withdrawn event
                else if event.event_type.ends_with("::PromotionFundsWithdrawnEvent") {
                    match parse_json_event::<PromotionFundsWithdrawnEvent>(&event.data) {
                        Ok(withdrawn_event) => {
                            if let Err(e) = self
                                .process_promotion_funds_withdrawn(&withdrawn_event, &tx_id)
                                .await
                            {
                                error!("Failed to process promotion funds withdrawn event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to deserialize promotion funds withdrawn event: {}",
                                e
                            );
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
pub async fn handle_event(
    db: &Arc<Database>,
    event: &MysEvent,
    transaction_id: &str,
) -> Result<()> {
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
        handle_remove_reaction(db, event, transaction_id).await?;
    } else if event_type.ends_with("::RepostEvent") {
        handle_repost(db, event, transaction_id).await?;
    } else if event_type.ends_with("::TipEvent") {
        handle_tip(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostModerationEvent") {
        handle_moderation(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostReportedEvent")
        || event_type.ends_with("::CommentReportedEvent")
    {
        handle_report(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostDeletedEvent")
        || event_type.ends_with("::CommentDeletedEvent")
    {
        handle_deletion(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostUpdatedEvent") {
        handle_post_updated(db, event, transaction_id).await?;
    } else if event_type.ends_with("::CommentUpdatedEvent") {
        handle_comment_updated(db, event, transaction_id).await?;
    } else if event_type.ends_with("::OwnershipTransferEvent") {
        handle_ownership_transfer(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PredictionCreatedEvent") {
        handle_prediction_created(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PredictionBetPlacedEvent") {
        handle_prediction_bet_placed(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PredictionResolvedEvent") {
        handle_prediction_resolved(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PredictionPayoutEvent") {
        handle_prediction_payout(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PredictionBetWithdrawnEvent") {
        handle_prediction_bet_withdrawn(db, event, transaction_id).await?;
    } else if event_type.ends_with("::PostParametersUpdatedEvent") {
        handle_post_parameters_updated(db, event, transaction_id).await?;
    } else {
        debug!("Unhandled post event type: {}", event_type);
    }

    Ok(())
}

/// Handle post created event
async fn handle_post_created(
    db: &Arc<Database>,
    event: &MysEvent,
    transaction_id: &str,
) -> Result<()> {
    info!("Processing PostCreatedEvent");

    // Parse the event
    let parsed_event = parse_event::<PostCreatedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PostCreatedEvent: {}", e))?;

    info!("Parsed PostCreatedEvent: post_id={}", parsed_event.post_id);

    // Get a database connection
    let mut conn = db.get_connection().await?;

    // Convert event to model
    let mut new_post = parsed_event.into_model()?;
    
    // If created_at is 0 (missing from event), use current time
    if new_post.created_at == 0 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        new_post.created_at = now.as_secs() as i64;
        // Update the ID to use the timestamp
        new_post.id = format!("{}:{}", parsed_event.post_id, new_post.created_at);
    }

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
            posts::mydata_id.eq(&new_post.mydata_id),
            posts::created_at.eq(&new_post.created_at),
            posts::transaction_id.eq(transaction_id),
        ))
        .execute(&mut conn)
        .await?;

    // Increment post_count for the profile (with graceful error handling)
    match diesel::update(crate::schema::profiles::table)
        .filter(crate::schema::profiles::owner_address.eq(&parsed_event.owner))
        .set(crate::schema::profiles::post_count.eq(crate::schema::profiles::post_count + 1))
        .execute(&mut conn)
        .await
    {
        Ok(updated_rows) => {
            if updated_rows > 0 {
                info!(
                    "Successfully incremented post_count for profile: {}",
                    parsed_event.owner
                );
            } else {
                warn!(
                    "No profile found to increment post_count for owner: {}",
                    parsed_event.owner
                );
            }
        }
        Err(e) => {
            // Log error but don't fail the transaction
            error!(
                "Failed to increment post_count for profile {}: {}. Post creation succeeded.",
                parsed_event.owner, e
            );
        }
    }

    info!("Processed PostCreatedEvent successfully");
    Ok(())
}

/// Handle comment created event
async fn handle_comment_created(
    db: &Arc<Database>,
    event: &MysEvent,
    transaction_id: &str,
) -> Result<()> {
    info!("Processing CommentCreatedEvent");

    // Parse the event
    let parsed_event = parse_event::<CommentCreatedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse CommentCreatedEvent: {}", e))?;

    info!(
        "Parsed CommentCreatedEvent: comment_id={}, post_id={}",
        parsed_event.comment_id, parsed_event.post_id
    );

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
            comments::transaction_id.eq(transaction_id),
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

    info!(
        "Parsed ReactionEvent: object_id={}, reaction={}",
        parsed_event.object_id, parsed_event.reaction_text
    );

    // Get a database connection
    let mut conn = db.get_connection().await?;

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

    info!(
        "Parsed RepostEvent: repost_id={}, original_id={}",
        parsed_event.repost_id, parsed_event.original_id
    );

    // Get a database connection
    let mut conn = db.get_connection().await?;

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
            reposts::transaction_id.eq(transaction_id),
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
    let parsed_event =
        parse_event::<TipEvent>(event).map_err(|e| anyhow!("Failed to parse TipEvent: {}", e))?;

    info!(
        "Parsed TipEvent: from={}, to={}, amount={}",
        parsed_event.from, parsed_event.to, parsed_event.amount
    );

    // Get a database connection
    let mut conn = db.get_connection().await?;

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
async fn handle_moderation(
    db: &Arc<Database>,
    event: &MysEvent,
    transaction_id: &str,
) -> Result<()> {
    info!("Processing PostModerationEvent");

    // Parse the event
    let parsed_event = parse_event::<PostModerationEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PostModerationEvent: {}", e))?;

    info!(
        "Parsed PostModerationEvent: object_id={}, platform_id={}, removed={}",
        parsed_event.object_id, parsed_event.platform_id, parsed_event.removed
    );

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
    match true {
        // Change based on how to detect if this is post or comment
        true => {
            diesel::update(posts::table)
                .filter(posts::post_id.eq(&parsed_event.object_id))
                .set((
                    posts::removed_from_platform.eq(parsed_event.removed),
                    posts::removed_by.eq(Some(parsed_event.moderated_by.clone())),
                ))
                .execute(&mut conn)
                .await?;
        }
        false => {
            diesel::update(comments::table)
                .filter(comments::comment_id.eq(&parsed_event.object_id))
                .set((
                    comments::removed_from_platform.eq(parsed_event.removed),
                    comments::removed_by.eq(Some(parsed_event.moderated_by.clone())),
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

    info!(
        "Parsed ReportEvent: object_id={}, reporter={}, reason={}",
        parsed_event.object_id, parsed_event.reporter, parsed_event.reason_code
    );

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

    info!(
        "Parsed DeletionEvent: object_id={}, is_post={}",
        parsed_event.object_id, parsed_event.is_post
    );

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

/// Handle remove reaction event
async fn handle_remove_reaction(
    db: &Arc<Database>,
    event: &MysEvent,
    transaction_id: &str,
) -> Result<()> {
    info!("Processing RemoveReactionEvent");

    let parsed_event = parse_event::<RemoveReactionEvent>(event)
        .map_err(|e| anyhow!("Failed to parse RemoveReactionEvent: {}", e))?;

    let mut conn = db.get_connection().await?;

    // Update reaction count in the database
    if parsed_event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&parsed_event.object_id))
            .set(posts::reaction_count.eq(posts::reaction_count - 1))
            .execute(&mut conn)
            .await?;
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&parsed_event.object_id))
            .set(comments::reaction_count.eq(comments::reaction_count - 1))
            .execute(&mut conn)
            .await?;
    }

    info!("Processed RemoveReactionEvent successfully");
    Ok(())
}

/// Handle post updated event
async fn handle_post_updated(
    db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PostUpdatedEvent");

    let parsed_event = parse_event::<ContentUpdateEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PostUpdatedEvent: {}", e))?;

    let mut conn = db.get_connection().await?;

    // Convert media_urls and mentions to JSON if present
    let media_urls_json: Option<serde_json::Value> = parsed_event
        .media_urls
        .as_ref()
        .map(|urls| serde_json::to_value(urls).unwrap_or(serde_json::json!(null)));

    let mentions_json: Option<serde_json::Value> = parsed_event
        .mentions
        .as_ref()
        .map(|mentions| serde_json::to_value(mentions).unwrap_or(serde_json::json!(null)));

    // Parse metadata JSON if present
    let metadata_json: Option<serde_json::Value> = parsed_event
        .metadata_json
        .as_ref()
        .map(|json_str| serde_json::from_str(json_str).unwrap_or(serde_json::json!(null)));

    // Update post content
    diesel::update(posts::table)
        .filter(posts::post_id.eq(&parsed_event.object_id))
        .set((
            posts::content.eq(&parsed_event.content),
            posts::media_urls.eq(&media_urls_json),
            posts::mentions.eq(&mentions_json),
            posts::metadata_json.eq(&metadata_json),
            posts::updated_at.eq(Some(parsed_event.updated_at as i64)),
        ))
        .execute(&mut conn)
        .await?;

    info!("Processed PostUpdatedEvent successfully");
    Ok(())
}

/// Handle comment updated event
async fn handle_comment_updated(
    db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing CommentUpdatedEvent");

    let parsed_event = parse_event::<ContentUpdateEvent>(event)
        .map_err(|e| anyhow!("Failed to parse CommentUpdatedEvent: {}", e))?;

    let mut conn = db.get_connection().await?;

    // Convert media_urls and mentions to JSON if present
    let media_urls_json: Option<serde_json::Value> = parsed_event
        .media_urls
        .as_ref()
        .map(|urls| serde_json::to_value(urls).unwrap_or(serde_json::json!(null)));

    let mentions_json: Option<serde_json::Value> = parsed_event
        .mentions
        .as_ref()
        .map(|mentions| serde_json::to_value(mentions).unwrap_or(serde_json::json!(null)));

    // Parse metadata JSON if present
    let metadata_json: Option<serde_json::Value> = parsed_event
        .metadata_json
        .as_ref()
        .map(|json_str| serde_json::from_str(json_str).unwrap_or(serde_json::json!(null)));

    // Update comment content
    diesel::update(comments::table)
        .filter(comments::comment_id.eq(&parsed_event.object_id))
        .set((
            comments::content.eq(&parsed_event.content),
            comments::media_urls.eq(&media_urls_json),
            comments::mentions.eq(&mentions_json),
            comments::metadata_json.eq(&metadata_json),
            comments::updated_at.eq(Some(parsed_event.updated_at as i64)),
        ))
        .execute(&mut conn)
        .await?;

    info!("Processed CommentUpdatedEvent successfully");
    Ok(())
}

/// Handle ownership transfer event
async fn handle_ownership_transfer(
    db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing OwnershipTransferEvent");

    let parsed_event = parse_event::<OwnershipTransferEvent>(event)
        .map_err(|e| anyhow!("Failed to parse OwnershipTransferEvent: {}", e))?;

    let mut conn = db.get_connection().await?;

    if parsed_event.is_post {
        // Update post ownership
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&parsed_event.object_id))
            .set(posts::owner.eq(&parsed_event.new_owner))
            .execute(&mut conn)
            .await?;
    } else {
        // Update comment ownership
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&parsed_event.object_id))
            .set(comments::owner.eq(&parsed_event.new_owner))
            .execute(&mut conn)
            .await?;
    }

    info!("Processed OwnershipTransferEvent successfully");
    Ok(())
}

/// Handle prediction created event
async fn handle_prediction_created(
    _db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PredictionCreatedEvent");

    let parsed_event = parse_event::<PredictionCreatedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PredictionCreatedEvent: {}", e))?;

    // Prediction posts are handled as regular posts, but we log this for tracking
    info!(
        "Prediction post created: post_id={}, prediction_data_id={}, options={:?}",
        parsed_event.post_id, parsed_event.prediction_data_id, parsed_event.options
    );

    // The post itself is already handled by PostCreatedEvent
    // This handler is for tracking prediction-specific metadata if needed in the future
    Ok(())
}

/// Handle prediction bet placed event
async fn handle_prediction_bet_placed(
    _db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PredictionBetPlacedEvent");

    let parsed_event = parse_event::<PredictionBetPlacedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PredictionBetPlacedEvent: {}", e))?;

    info!(
        "Prediction bet placed: post_id={}, user={}, option_id={}, amount={}",
        parsed_event.post_id, parsed_event.user, parsed_event.option_id, parsed_event.amount
    );

    // Log prediction bets for analytics
    // Future: Could store in a predictions_bets table if needed
    Ok(())
}

/// Handle prediction resolved event
async fn handle_prediction_resolved(
    _db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PredictionResolvedEvent");

    let parsed_event = parse_event::<PredictionResolvedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PredictionResolvedEvent: {}", e))?;

    info!(
        "Prediction resolved: post_id={}, winning_option_id={}, total_bet_amount={}, winning_amount={}, resolved_by={}",
        parsed_event.post_id,
        parsed_event.winning_option_id,
        parsed_event.total_bet_amount,
        parsed_event.winning_amount,
        parsed_event.resolved_by
    );

    // Log prediction resolution for analytics
    // Future: Could update a predictions table with resolution status
    Ok(())
}

/// Handle prediction payout event
async fn handle_prediction_payout(
    _db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PredictionPayoutEvent");

    let parsed_event = parse_event::<PredictionPayoutEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PredictionPayoutEvent: {}", e))?;

    info!(
        "Prediction payout: post_id={}, user={}, amount={}",
        parsed_event.post_id, parsed_event.user, parsed_event.amount
    );

    // Log prediction payouts for analytics
    // Future: Could track payouts in a predictions_payouts table
    Ok(())
}

/// Handle prediction bet withdrawn event
async fn handle_prediction_bet_withdrawn(
    _db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PredictionBetWithdrawnEvent");

    let parsed_event = parse_event::<PredictionBetWithdrawnEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PredictionBetWithdrawnEvent: {}", e))?;

    info!(
        "Prediction bet withdrawn: post_id={}, user={}, option_id={}, original_amount={}, withdrawal_amount={}",
        parsed_event.post_id,
        parsed_event.user,
        parsed_event.option_id,
        parsed_event.original_amount,
        parsed_event.withdrawal_amount
    );

    // Log prediction bet withdrawals for analytics
    Ok(())
}

/// Handle post parameters updated event
async fn handle_post_parameters_updated(
    _db: &Arc<Database>,
    event: &MysEvent,
    _transaction_id: &str,
) -> Result<()> {
    info!("Processing PostParametersUpdatedEvent");

    let parsed_event = parse_event::<PostParametersUpdatedEvent>(event)
        .map_err(|e| anyhow!("Failed to parse PostParametersUpdatedEvent: {}", e))?;

    info!(
        "Post parameters updated by: {}, max_content_length={}, max_media_urls={}",
        parsed_event.updated_by, parsed_event.max_content_length, parsed_event.max_media_urls
    );

    // Log configuration changes for audit purposes
    // Future: Could store in a post_config_history table
    Ok(())
}
