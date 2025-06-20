// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Type of post event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostEventType {
    PostCreated,
    CommentCreated,
    Repost,
    Reaction,
    RemoveReaction,
    Tip,
    OwnershipTransfer,
    PostModeration,
    PostUpdated,
    CommentUpdated,
    PostReported,
    CommentReported,
    PostDeleted,
    CommentDeleted,
}

/// Post created event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCreatedEvent {
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub mentions: Option<Vec<String>>,
    pub metadata_json: Option<String>,
    pub post_type: String,
    pub parent_post_id: Option<String>,
    pub created_at: u64,
    /// Reference to MyIP license if attached
    pub my_ip_id: Option<String>,
}

/// Comment created event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreatedEvent {
    pub comment_id: String,
    pub post_id: String,
    pub parent_comment_id: Option<String>,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub mentions: Option<Vec<String>>,
    pub metadata_json: Option<String>,
    pub created_at: u64,
}

/// Reaction event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionEvent {
    pub object_id: String,
    pub user_address: String,
    pub reaction_text: String,
    pub is_post: bool,
    pub created_at: u64,
}

/// Remove reaction event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveReactionEvent {
    pub object_id: String,
    pub user_address: String,
    pub reaction_text: String,
    pub is_post: bool,
}

/// Repost event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepostEvent {
    pub repost_id: String,
    pub original_id: String,
    pub original_post_id: String,
    pub is_original_post: bool,
    pub owner: String,
    pub profile_id: String,
    pub created_at: u64,
}

/// Tip event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipEvent {
    pub object_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub is_post: bool,
    pub tip_time: u64,
    /// Original intended recipient (before potential MyIP redirection)
    pub original_recipient: Option<String>,
    /// MyIP license responsible for redirection, if any
    pub license_id: Option<String>,
}

impl TipEvent {
    /// Create unified revenue record for tip
    pub fn create_unified_revenue_record(&self, transaction_id: String) -> anyhow::Result<crate::models::NewUnifiedRevenue> {
        let revenue_type = if self.is_post {
            crate::models::revenue::REVENUE_TYPE_TIPS_POST.to_string()
        } else {
            crate::models::revenue::REVENUE_TYPE_TIPS_COMMENT.to_string()
        };
        
        let content_type = if self.is_post {
            crate::models::revenue::CONTENT_TYPE_POST.to_string()
        } else {
            crate::models::revenue::CONTENT_TYPE_COMMENT.to_string()
        };
        
        Ok(crate::models::NewUnifiedRevenue::from_tip(
            revenue_type,
            self.to.clone(),
            self.amount as i64,
            self.object_id.clone(),
            content_type,
            self.from.clone(),
            self.tip_time as i64,
            transaction_id,
        ))
    }
}

/// Post/Comment moderation event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationEvent {
    pub object_id: String,
    pub platform_id: String,
    pub removed: bool,
    pub moderated_by: String,
    pub moderated_at: u64,
}

/// Post/Comment update event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentUpdateEvent {
    pub object_id: String,
    pub is_post: bool,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub mentions: Option<Vec<String>>,
    pub metadata_json: Option<String>,
    pub updated_at: u64,
}

/// Report event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportEvent {
    pub object_id: String,
    pub is_comment: bool,
    pub reporter: String,
    pub reason_code: u8,
    pub description: String,
    pub reported_at: u64,
}

/// Post/Comment deletion event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionEvent {
    pub object_id: String,
    pub owner: String,
    pub profile_id: String,
    pub is_post: bool,
    pub post_type: Option<String>,
    pub post_id: Option<String>,
    pub deleted_at: u64,
} 