// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde_json::json;

// Import specific event types to avoid ambiguity
use crate::social::events::post_event_types::{
    CommentCreatedEvent, ContentUpdateEvent, DeletionEvent as PostDeletionEvent,
    ModerationEvent as PostModerationEvent, PostCreatedEvent, ReactionEvent, RemoveReactionEvent,
    ReportEvent, RepostEvent, TipEvent,
};

// Import model types
use crate::social::models::post::{
    NewComment, NewDeletionEvent, NewModerationEvent, NewPost, NewReaction, NewReactionCount,
    NewReport, NewRepost, NewTip,
};

// Model conversion impl for PostCreatedEvent
impl PostCreatedEvent {
    pub fn into_model(&self) -> Result<NewPost> {
        // Create a unique ID for the post
        let id = format!("{}:{}", self.post_id, self.created_at);

        // Convert media_urls and mentions to JSON if present
        let media_urls_json = self
            .media_urls
            .as_ref()
            .map(|urls| serde_json::to_value(urls).unwrap_or(json!(null)));

        let mentions_json = self
            .mentions
            .as_ref()
            .map(|mentions| serde_json::to_value(mentions).unwrap_or(json!(null)));

        // Parse metadata JSON if present
        let metadata_json = self
            .metadata_json
            .as_ref()
            .map(|json_str| serde_json::from_str(json_str).unwrap_or(json!(null)));

        // Create the model
        Ok(NewPost {
            id,
            post_id: self.post_id.clone(),
            owner: self.owner.clone(),
            profile_id: self.profile_id.clone(),
            content: self.content.clone(),
            media_urls: media_urls_json,
            mentions: mentions_json,
            metadata_json,
            post_type: self.post_type.clone(),
            parent_post_id: self.parent_post_id.clone(),
            created_at: self.created_at as i64,
            updated_at: None,
            deleted_at: None,
            reaction_count: 0,
            comment_count: 0,
            repost_count: 0,
            tips_received: 0,
            removed_from_platform: false,
            removed_by: None,
            transaction_id: "".to_string(), // Will be set by handler
            mydata_id: self.mydata_id.clone(),
            revenue_recipient: None, // Revenue tracking handled via unified revenue system
            promotion_id: self.promotion_id.clone(),
            poc_id: self.poc_id.clone(),
            poc_reasoning: None,
            poc_evidence_urls: None,
            poc_similarity_score: None,
            poc_media_type: None,
            poc_oracle_address: None,
            poc_analyzed_at: None,
            revenue_redirect_to: self.revenue_redirect_to.clone(),
            revenue_redirect_percentage: self.revenue_redirect_percentage.map(|p| p as i64),
            enable_spt: self.enable_spt,
            enable_poc: self.enable_poc,
            enable_spot: self.enable_spot,
            spot_id: self.spot_id.clone(),
            spt_id: self.spt_id.clone(),
        })
    }
}

// Model conversion impl for CommentCreatedEvent
impl CommentCreatedEvent {
    pub fn into_model(&self) -> Result<NewComment> {
        // Create a unique ID for the comment
        let id = format!("{}:{}", self.comment_id, self.created_at);

        // Convert media_urls and mentions to JSON if present
        let media_urls_json = self
            .media_urls
            .as_ref()
            .map(|urls| serde_json::to_value(urls).unwrap_or(json!(null)));

        let mentions_json = self
            .mentions
            .as_ref()
            .map(|mentions| serde_json::to_value(mentions).unwrap_or(json!(null)));

        // Parse metadata JSON if present
        let metadata_json = self
            .metadata_json
            .as_ref()
            .map(|json_str| serde_json::from_str(json_str).unwrap_or(json!(null)));

        // Create the model
        Ok(NewComment {
            id,
            comment_id: self.comment_id.clone(),
            post_id: self.post_id.clone(),
            parent_comment_id: self.parent_comment_id.clone(),
            owner: self.owner.clone(),
            profile_id: self.profile_id.clone(),
            content: self.content.clone(),
            media_urls: media_urls_json,
            mentions: mentions_json,
            metadata_json,
            created_at: self.created_at as i64,
            updated_at: None,
            deleted_at: None,
            reaction_count: 0,
            comment_count: 0,
            repost_count: 0,
            tips_received: 0,
            removed_from_platform: false,
            removed_by: None,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for ReactionEvent
impl ReactionEvent {
    pub fn into_model(&self) -> Result<NewReaction> {
        Ok(NewReaction {
            object_id: self.object_id.clone(),
            user_address: self.user_address.clone(),
            reaction_text: self.reaction_text.clone(),
            is_post: self.is_post,
            created_at: self.created_at as i64,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }

    pub fn into_reaction_count(&self) -> Result<NewReactionCount> {
        Ok(NewReactionCount {
            object_id: self.object_id.clone(),
            reaction_text: self.reaction_text.clone(),
            count: 1, // Will be handled properly by SQL upsert
        })
    }
}

// Model conversion impl for RepostEvent
impl RepostEvent {
    pub fn into_model(&self) -> Result<NewRepost> {
        // Create a unique ID for the repost
        let id = format!("{}:{}", self.repost_id, self.created_at);

        Ok(NewRepost {
            id,
            repost_id: self.repost_id.clone(),
            original_id: self.original_id.clone(),
            original_post_id: self.original_post_id.clone(),
            is_original_post: self.is_original_post,
            owner: self.owner.clone(),
            profile_id: self.profile_id.clone(),
            created_at: self.created_at as i64,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for TipEvent
impl TipEvent {
    pub fn into_model(&self) -> Result<NewTip> {
        Ok(NewTip {
            tipper: self.from.clone(),
            recipient: self.to.clone(),
            object_id: self.object_id.clone(),
            amount: self.amount as i64,
            is_post: self.is_post,
            created_at: self.tip_time as i64,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for ModerationEvent
impl PostModerationEvent {
    pub fn into_model(&self) -> Result<NewModerationEvent> {
        Ok(NewModerationEvent {
            object_id: self.object_id.clone(),
            platform_id: self.platform_id.clone(),
            removed: self.removed,
            moderated_by: self.moderated_by.clone(),
            moderated_at: self.moderated_at as i64,
            transaction_id: String::new(), // Will be filled in by the handler
        })
    }
}

// Model conversion impl for ReportEvent
impl ReportEvent {
    pub fn into_model(&self) -> Result<NewReport> {
        Ok(NewReport {
            object_id: self.object_id.clone(),
            is_comment: self.is_comment,
            reporter: self.reporter.clone(),
            reason_code: self.reason_code as i16,
            description: self.description.clone(),
            reported_at: self.reported_at as i64,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for DeletionEvent
impl PostDeletionEvent {
    pub fn into_model(&self) -> Result<NewDeletionEvent> {
        Ok(NewDeletionEvent {
            object_id: self.object_id.clone(),
            owner: self.owner.clone(),
            profile_id: self.profile_id.clone(),
            is_post: self.is_post,
            post_type: self.post_type.clone(),
            post_id: self.post_id.clone(),
            deleted_at: self.deleted_at as i64,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use anyhow::anyhow;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::{posts, comments, reactions, reaction_counts, reposts, tips,
    posts_moderation_events, posts_reports, posts_deletion_events};

/// Process a PostCreatedEvent and insert into the database
pub async fn process_post_created_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: PostCreatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PostCreatedEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    diesel::insert_into(posts::table)
        .values(&model)
        .on_conflict(posts::id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert post: {}", e))?;

    tracing::info!("Processed PostCreatedEvent for post_id: {}", event.post_id);
    Ok(())
}

/// Process a CommentCreatedEvent and insert into the database
pub async fn process_comment_created_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: CommentCreatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse CommentCreatedEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    diesel::insert_into(comments::table)
        .values(&model)
        .on_conflict(comments::id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert comment: {}", e))?;

    // Update the post's comment count
    diesel::update(posts::table)
        .filter(posts::post_id.eq(&event.post_id))
        .set(posts::comment_count.eq(posts::comment_count + 1))
        .execute(conn)
        .await
        .ok(); // Ignore errors on count update

    tracing::info!("Processed CommentCreatedEvent for comment_id: {} on post: {}",
        event.comment_id, event.post_id);
    Ok(())
}

/// Process a ReactionEvent and insert into the database
pub async fn process_reaction_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: ReactionEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ReactionEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    // Insert the reaction
    diesel::insert_into(reactions::table)
        .values(&model)
        .on_conflict_do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert reaction: {}", e))?;

    // Update reaction count
    let count_model = event.into_reaction_count()?;
    diesel::insert_into(reaction_counts::table)
        .values(&count_model)
        .on_conflict((reaction_counts::object_id, reaction_counts::reaction_text))
        .do_update()
        .set(reaction_counts::count.eq(reaction_counts::count + 1))
        .execute(conn)
        .await
        .ok(); // Ignore errors on count update

    // Update the post/comment's reaction count
    if event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.object_id))
            .set(posts::reaction_count.eq(posts::reaction_count + 1))
            .execute(conn)
            .await
            .ok();
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&event.object_id))
            .set(comments::reaction_count.eq(comments::reaction_count + 1))
            .execute(conn)
            .await
            .ok();
    }

    tracing::info!("Processed ReactionEvent: {} on {}",
        event.reaction_text, event.object_id);
    Ok(())
}

/// Process a RemoveReactionEvent and update the database
pub async fn process_remove_reaction_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: RemoveReactionEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse RemoveReactionEvent: {}", e))?;

    // Delete the reaction
    diesel::delete(reactions::table)
        .filter(reactions::object_id.eq(&event.object_id))
        .filter(reactions::user_address.eq(&event.user_address))
        .filter(reactions::reaction_text.eq(&event.reaction_text))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to delete reaction: {}", e))?;

    // Update reaction count (decrement)
    diesel::update(reaction_counts::table)
        .filter(reaction_counts::object_id.eq(&event.object_id))
        .filter(reaction_counts::reaction_text.eq(&event.reaction_text))
        .set(reaction_counts::count.eq(reaction_counts::count - 1))
        .execute(conn)
        .await
        .ok();

    // Update the post/comment's reaction count
    if event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.object_id))
            .set(posts::reaction_count.eq(posts::reaction_count - 1))
            .execute(conn)
            .await
            .ok();
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&event.object_id))
            .set(comments::reaction_count.eq(comments::reaction_count - 1))
            .execute(conn)
            .await
            .ok();
    }

    tracing::info!("Processed RemoveReactionEvent: {} from {}",
        event.reaction_text, event.object_id);
    Ok(())
}

/// Process a RepostEvent and insert into the database
pub async fn process_repost_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: RepostEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse RepostEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    diesel::insert_into(reposts::table)
        .values(&model)
        .on_conflict(reposts::id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert repost: {}", e))?;

    // Update the original post/comment's repost count
    if event.is_original_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.original_post_id))
            .set(posts::repost_count.eq(posts::repost_count + 1))
            .execute(conn)
            .await
            .ok();
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&event.original_id))
            .set(comments::repost_count.eq(comments::repost_count + 1))
            .execute(conn)
            .await
            .ok();
    }

    tracing::info!("Processed RepostEvent for repost_id: {}", event.repost_id);
    Ok(())
}

/// Process a TipEvent and insert into the database
pub async fn process_tip_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: TipEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse TipEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    diesel::insert_into(tips::table)
        .values(&model)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert tip: {}", e))?;

    // Update the post/comment's tips_received
    if event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.object_id))
            .set(posts::tips_received.eq(posts::tips_received + event.amount as i64))
            .execute(conn)
            .await
            .ok();
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&event.object_id))
            .set(comments::tips_received.eq(comments::tips_received + event.amount as i64))
            .execute(conn)
            .await
            .ok();
    }

    // Create unified revenue record
    if let Ok(revenue_record) = event.create_unified_revenue_record(event_id.to_string()) {
        use crate::social::schema::unified_revenue;
        diesel::insert_into(unified_revenue::table)
            .values(&revenue_record)
            .execute(conn)
            .await
            .ok();
    }

    tracing::info!("Processed TipEvent: {} MYS from {} to {}",
        event.amount, event.from, event.to);
    Ok(())
}

/// Process a ModerationEvent and update the database
pub async fn process_moderation_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: PostModerationEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ModerationEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    // Insert the moderation event
    diesel::insert_into(posts_moderation_events::table)
        .values(&model)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert moderation event: {}", e))?;

    // Update the post's removed_from_platform flag
    diesel::update(posts::table)
        .filter(posts::post_id.eq(&event.object_id))
        .set((
            posts::removed_from_platform.eq(event.removed),
            posts::removed_by.eq(Some(&event.moderated_by)),
        ))
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed ModerationEvent for object: {} removed: {}",
        event.object_id, event.removed);
    Ok(())
}

/// Process a ReportEvent and insert into the database
pub async fn process_report_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: ReportEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ReportEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    diesel::insert_into(posts_reports::table)
        .values(&model)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert report: {}", e))?;

    tracing::info!("Processed ReportEvent for object: {} by {}",
        event.object_id, event.reporter);
    Ok(())
}

/// Process a DeletionEvent and update the database
pub async fn process_deletion_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: PostDeletionEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse DeletionEvent: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = event_id.to_string();

    // Insert the deletion event for audit trail
    diesel::insert_into(posts_deletion_events::table)
        .values(&model)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert deletion event: {}", e))?;

    // Mark the post/comment as deleted
    if event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.object_id))
            .set(posts::deleted_at.eq(Some(event.deleted_at as i64)))
            .execute(conn)
            .await
            .ok();
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&event.object_id))
            .set(comments::deleted_at.eq(Some(event.deleted_at as i64)))
            .execute(conn)
            .await
            .ok();
    }

    tracing::info!("Processed DeletionEvent for object: {} is_post: {}",
        event.object_id, event.is_post);
    Ok(())
}

/// Process a ContentUpdateEvent and update the database
pub async fn process_content_update_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: ContentUpdateEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ContentUpdateEvent: {}", e))?;

    let media_urls_json = event.media_urls
        .as_ref()
        .map(|urls| serde_json::to_value(urls).unwrap_or(json!(null)));

    let mentions_json = event.mentions
        .as_ref()
        .map(|mentions| serde_json::to_value(mentions).unwrap_or(json!(null)));

    let metadata_json = event.metadata_json
        .as_ref()
        .map(|json_str| serde_json::from_str(json_str).unwrap_or(json!(null)));

    if event.is_post {
        diesel::update(posts::table)
            .filter(posts::post_id.eq(&event.object_id))
            .set((
                posts::content.eq(&event.content),
                posts::media_urls.eq(&media_urls_json),
                posts::mentions.eq(&mentions_json),
                posts::metadata_json.eq(&metadata_json),
                posts::updated_at.eq(Some(event.updated_at as i64)),
            ))
            .execute(conn)
            .await
            .map_err(|e| anyhow!("Failed to update post: {}", e))?;
    } else {
        diesel::update(comments::table)
            .filter(comments::comment_id.eq(&event.object_id))
            .set((
                comments::content.eq(&event.content),
                comments::media_urls.eq(&media_urls_json),
                comments::mentions.eq(&mentions_json),
                comments::metadata_json.eq(&metadata_json),
                comments::updated_at.eq(Some(event.updated_at as i64)),
            ))
            .execute(conn)
            .await
            .map_err(|e| anyhow!("Failed to update comment: {}", e))?;
    }

    tracing::info!("Processed ContentUpdateEvent for object: {} is_post: {}",
        event.object_id, event.is_post);
    Ok(())
}
