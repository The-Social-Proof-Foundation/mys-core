// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::events::event_utils::{deserialize_u64_from_string, deserialize_u64_from_string_optional, deserialize_optional_u64_from_string};
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
    PromotedPostCreated,
    PromotedPostViewConfirmed,
    PromotionStatusToggled,
    PromotionFundsWithdrawn,
}

/// Post created event from blockchain
/// NOTE: This matches the contract's PostCreatedEvent structure exactly:
/// - post_id, owner, profile_id, content, post_type, parent_post_id, mentions
/// - media_urls, metadata_json, mydata_id, promotion_id, poc_badge_id, revenue_redirect_to, revenue_redirect_percentage, disable_auto_pool
/// - created_at is NOT in the contract event but is needed for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCreatedEvent {
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub post_type: String,
    pub parent_post_id: Option<String>,
    pub mentions: Option<Vec<String>>,
    pub media_urls: Option<Vec<String>>,
    pub metadata_json: Option<String>,
    pub mydata_id: Option<String>,
    pub promotion_id: Option<String>,
    pub poc_badge_id: Option<String>,
    pub revenue_redirect_to: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_u64_from_string")]
    pub revenue_redirect_percentage: Option<u64>,
    pub disable_auto_pool: bool,
    // This field is NOT in the contract event but is needed for database storage
    #[serde(default, deserialize_with = "deserialize_u64_from_string_optional")]
    pub created_at: u64,
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub created_at: u64,
}

/// Reaction event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionEvent {
    pub object_id: String,
    pub user_address: String,
    pub reaction_text: String,
    pub is_post: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub tip_time: u64,
    /// Original intended recipient (before potential MyIP redirection)
    pub original_recipient: Option<String>,
    /// MyIP license responsible for redirection, if any
    pub license_id: Option<String>,
}

impl TipEvent {
    /// Create unified revenue record for tip
    pub fn create_unified_revenue_record(
        &self,
        transaction_id: String,
    ) -> anyhow::Result<crate::models::NewUnifiedRevenue> {
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
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
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub deleted_at: u64,
}

/// Promoted post created event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotedPostCreatedEvent {
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: u64,
    pub total_budget: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub created_at: u64,
}

/// Promoted post view confirmed event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotedPostViewConfirmedEvent {
    pub post_id: String,
    pub viewer: String,
    pub payment_amount: u64,
    pub view_duration: u64,
    pub platform_id: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Promotion status toggled event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionStatusToggledEvent {
    pub post_id: String,
    pub toggled_by: String,
    pub new_status: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Promotion funds withdrawn event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionFundsWithdrawnEvent {
    pub post_id: String,
    pub owner: String,
    pub withdrawn_amount: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

/// Ownership transfer event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipTransferEvent {
    pub object_id: String,
    pub previous_owner: String,
    pub new_owner: String,
    pub is_post: bool,
}

/// Post parameters updated event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostParametersUpdatedEvent {
    pub updated_by: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
    pub max_content_length: u64,
    pub max_media_urls: u64,
    pub max_mentions: u64,
    pub max_metadata_size: u64,
    pub max_description_length: u64,
    pub max_reaction_length: u64,
    pub commenter_tip_percentage: u64,
    pub repost_tip_percentage: u64,
}

/// Auto pool disabled updated event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoPoolDisabledUpdatedEvent {
    pub post_id: String,
    pub owner: String,
    pub disabled: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

