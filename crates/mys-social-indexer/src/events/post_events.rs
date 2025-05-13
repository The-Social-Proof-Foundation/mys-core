// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde_json::json;

// Import specific event types to avoid ambiguity
use crate::events::post_event_types::{
    PostCreatedEvent,
    CommentCreatedEvent,
    ReactionEvent,
    RepostEvent,
    TipEvent,
    ModerationEvent as PostModerationEvent,
    ReportEvent,
    DeletionEvent as PostDeletionEvent,
};

// Import model types
use crate::models::post::{
    NewPost,
    NewComment,
    NewReaction,
    NewReactionCount,
    NewRepost,
    NewTip,
    NewModerationEvent,
    NewReport,
    NewDeletionEvent,
};

// Import MyIP model for revenue tracking
use crate::models::my_ip::NewMyIPRevenue;

// Model conversion impl for PostCreatedEvent
impl PostCreatedEvent {
    pub fn into_model(&self) -> Result<NewPost> {
        // Create a unique ID for the post
        let id = format!("{}:{}", self.post_id, self.created_at);
        
        // Convert media_urls and mentions to JSON if present
        let media_urls_json = self.media_urls.as_ref().map(|urls| {
            serde_json::to_value(urls).unwrap_or(json!(null))
        });
        
        let mentions_json = self.mentions.as_ref().map(|mentions| {
            serde_json::to_value(mentions).unwrap_or(json!(null))
        });
        
        // Parse metadata JSON if present
        let metadata_json = self.metadata_json.as_ref().map(|json_str| {
            serde_json::from_str(json_str).unwrap_or(json!(null))
        });
        
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
            my_ip_id: self.my_ip_id.clone(),
            revenue_recipient: None, // Will be set if needed based on MyIP
        })
    }
}

// Model conversion impl for CommentCreatedEvent
impl CommentCreatedEvent {
    pub fn into_model(&self) -> Result<NewComment> {
        // Create a unique ID for the comment
        let id = format!("{}:{}", self.comment_id, self.created_at);
        
        // Convert media_urls and mentions to JSON if present
        let media_urls_json = self.media_urls.as_ref().map(|urls| {
            serde_json::to_value(urls).unwrap_or(json!(null))
        });
        
        let mentions_json = self.mentions.as_ref().map(|mentions| {
            serde_json::to_value(mentions).unwrap_or(json!(null))
        });
        
        // Parse metadata JSON if present
        let metadata_json = self.metadata_json.as_ref().map(|json_str| {
            serde_json::from_str(json_str).unwrap_or(json!(null))
        });
        
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
    
    // New method for creating MyIP revenue records when tips involve revenue redirection
    pub fn into_my_ip_revenue(&self, transaction_id: String) -> Result<Option<NewMyIPRevenue>> {
        if let Some(license_id) = &self.license_id {
            Ok(Some(NewMyIPRevenue {
                license_id: license_id.clone(),
                post_id: Some(self.object_id.clone()),
                from_address: self.from.clone(),
                // Use the actual recipient (which may be different from original due to redirection)
                to_address: self.to.clone(),
                amount: self.amount as i64,
                revenue_type: "TIP".to_string(),
                revenue_time: self.tip_time as i64,
                transaction_id,
            }))
        } else {
            // No license involved, so no MyIP revenue record needed
            Ok(None)
        }
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