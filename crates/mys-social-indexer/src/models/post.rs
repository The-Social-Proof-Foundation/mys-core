// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::sql_types::*;
use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Post model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    #[diesel(sql_type = Varchar)]
    pub id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub owner: String,
    #[diesel(sql_type = Varchar)]
    pub profile_id: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub media_urls: Option<Value>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub mentions: Option<Value>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub metadata_json: Option<Value>,
    #[diesel(sql_type = Varchar)]
    pub post_type: String,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub parent_post_id: Option<String>,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<Int8>)]
    pub updated_at: Option<i64>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub deleted_at: Option<i64>,
    #[diesel(sql_type = Int8)]
    pub reaction_count: i64,
    #[diesel(sql_type = Int8)]
    pub comment_count: i64,
    #[diesel(sql_type = Int8)]
    pub repost_count: i64,
    #[diesel(sql_type = Int8)]
    pub tips_received: i64,
    #[diesel(sql_type = Bool)]
    pub removed_from_platform: bool,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub removed_by: Option<String>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    pub time: DateTime<Utc>,
    pub mydata_id: Option<String>,
    pub revenue_recipient: Option<String>,
    pub promotion_id: Option<String>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub poc_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_reasoning: Option<String>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub poc_evidence_urls: Option<Value>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub poc_similarity_score: Option<i64>,
    #[diesel(sql_type = Nullable<Int2>)]
    pub poc_media_type: Option<i16>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub poc_oracle_address: Option<String>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub poc_analyzed_at: Option<i64>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub revenue_redirect_to: Option<String>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub revenue_redirect_percentage: Option<i64>,
    #[diesel(sql_type = Bool)]
    pub enable_spt: bool,
    #[diesel(sql_type = Bool)]
    pub enable_poc: bool,
    #[diesel(sql_type = Bool)]
    pub enable_spot: bool,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub spot_id: Option<String>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub spt_id: Option<String>,
}

/// New post model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub media_urls: Option<Value>,
    pub mentions: Option<Value>,
    pub metadata_json: Option<Value>,
    pub post_type: String,
    pub parent_post_id: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub repost_count: i64,
    pub tips_received: i64,
    pub removed_from_platform: bool,
    pub removed_by: Option<String>,
    pub transaction_id: String,
    pub mydata_id: Option<String>,
    pub revenue_recipient: Option<String>,
    pub promotion_id: Option<String>,
    pub poc_id: Option<String>,
    pub poc_reasoning: Option<String>,
    pub poc_evidence_urls: Option<Value>,
    pub poc_similarity_score: Option<i64>,
    pub poc_media_type: Option<i16>,
    pub poc_oracle_address: Option<String>,
    pub poc_analyzed_at: Option<i64>,
    pub revenue_redirect_to: Option<String>,
    pub revenue_redirect_percentage: Option<i64>,
    pub enable_spt: bool,
    pub enable_poc: bool,
    pub enable_spot: bool,
    pub spot_id: Option<String>,
    pub spt_id: Option<String>,
}

/// Comment model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    #[diesel(sql_type = Varchar)]
    pub id: String,
    #[diesel(sql_type = Varchar)]
    pub comment_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub parent_comment_id: Option<String>,
    #[diesel(sql_type = Varchar)]
    pub owner: String,
    #[diesel(sql_type = Varchar)]
    pub profile_id: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub media_urls: Option<Value>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub mentions: Option<Value>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub metadata_json: Option<Value>,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<Int8>)]
    pub updated_at: Option<i64>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub deleted_at: Option<i64>,
    #[diesel(sql_type = Int8)]
    pub reaction_count: i64,
    #[diesel(sql_type = Int8)]
    pub comment_count: i64,
    #[diesel(sql_type = Int8)]
    pub repost_count: i64,
    #[diesel(sql_type = Int8)]
    pub tips_received: i64,
    #[diesel(sql_type = Bool)]
    pub removed_from_platform: bool,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub removed_by: Option<String>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    pub time: DateTime<Utc>,
}

/// New comment model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub id: String,
    pub comment_id: String,
    pub post_id: String,
    pub parent_comment_id: Option<String>,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub media_urls: Option<Value>,
    pub mentions: Option<Value>,
    pub metadata_json: Option<Value>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub repost_count: i64,
    pub tips_received: i64,
    pub removed_from_platform: bool,
    pub removed_by: Option<String>,
    pub transaction_id: String,
}

/// Reaction model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::reactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reaction {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub object_id: String,
    #[diesel(sql_type = Varchar)]
    pub user_address: String,
    #[diesel(sql_type = Varchar)]
    pub reaction_text: String,
    #[diesel(sql_type = Bool)]
    pub is_post: bool,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New reaction model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::reactions)]
pub struct NewReaction {
    pub object_id: String,
    pub user_address: String,
    pub reaction_text: String,
    pub is_post: bool,
    pub created_at: i64,
    pub transaction_id: String,
}

/// Reaction count model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::reaction_counts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReactionCount {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub object_id: String,
    #[diesel(sql_type = Varchar)]
    pub reaction_text: String,
    #[diesel(sql_type = Int8)]
    pub count: i64,
}

/// New reaction count model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::reaction_counts)]
pub struct NewReactionCount {
    pub object_id: String,
    pub reaction_text: String,
    pub count: i64,
}

/// Repost model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::reposts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Repost {
    #[diesel(sql_type = Varchar)]
    pub id: String,
    #[diesel(sql_type = Varchar)]
    pub repost_id: String,
    #[diesel(sql_type = Varchar)]
    pub original_id: String,
    #[diesel(sql_type = Varchar)]
    pub original_post_id: String,
    #[diesel(sql_type = Bool)]
    pub is_original_post: bool,
    #[diesel(sql_type = Varchar)]
    pub owner: String,
    #[diesel(sql_type = Varchar)]
    pub profile_id: String,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New repost model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::reposts)]
pub struct NewRepost {
    pub id: String,
    pub repost_id: String,
    pub original_id: String,
    pub original_post_id: String,
    pub is_original_post: bool,
    pub owner: String,
    pub profile_id: String,
    pub created_at: i64,
    pub transaction_id: String,
}

/// Tip model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::tips)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tip {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub tipper: String,
    #[diesel(sql_type = Varchar)]
    pub recipient: String,
    #[diesel(sql_type = Varchar)]
    pub object_id: String,
    #[diesel(sql_type = Int8)]
    pub amount: i64,
    #[diesel(sql_type = Bool)]
    pub is_post: bool,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New tip model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tips)]
pub struct NewTip {
    pub tipper: String,
    pub recipient: String,
    pub object_id: String,
    pub amount: i64,
    pub is_post: bool,
    pub created_at: i64,
    pub transaction_id: String,
}

/// Moderation event model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::posts_moderation_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ModerationEvent {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub object_id: String,
    #[diesel(sql_type = Varchar)]
    pub platform_id: String,
    #[diesel(sql_type = Bool)]
    pub removed: bool,
    #[diesel(sql_type = Varchar)]
    pub moderated_by: String,
    #[diesel(sql_type = Int8)]
    pub moderated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New moderation event model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts_moderation_events)]
pub struct NewModerationEvent {
    pub object_id: String,
    pub platform_id: String,
    pub removed: bool,
    pub moderated_by: String,
    pub moderated_at: i64,
    pub transaction_id: String,
}

/// Report model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::posts_reports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Report {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub object_id: String,
    #[diesel(sql_type = Bool)]
    pub is_comment: bool,
    #[diesel(sql_type = Varchar)]
    pub reporter: String,
    #[diesel(sql_type = Int2)]
    pub reason_code: i16,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = Int8)]
    pub reported_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New report model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts_reports)]
pub struct NewReport {
    pub object_id: String,
    pub is_comment: bool,
    pub reporter: String,
    pub reason_code: i16,
    pub description: String,
    pub reported_at: i64,
    pub transaction_id: String,
}

/// Deletion event model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::posts_deletion_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeletionEvent {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub object_id: String,
    #[diesel(sql_type = Varchar)]
    pub owner: String,
    #[diesel(sql_type = Varchar)]
    pub profile_id: String,
    #[diesel(sql_type = Bool)]
    pub is_post: bool,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub post_type: Option<String>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub post_id: Option<String>,
    #[diesel(sql_type = Int8)]
    pub deleted_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New deletion event model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts_deletion_events)]
pub struct NewDeletionEvent {
    pub object_id: String,
    pub owner: String,
    pub profile_id: String,
    pub is_post: bool,
    pub post_type: Option<String>,
    pub post_id: Option<String>,
    pub deleted_at: i64,
    pub transaction_id: String,
}

// Types for database results
#[derive(Debug, QueryableByName)]
pub struct PostWithEngagement {
    #[diesel(embed)]
    pub post: Post,
    #[diesel(sql_type = Int8)]
    pub engagement_score: i64,
    #[diesel(sql_type = Float8)]
    pub trending_score: f64,
}

/// Promoted post model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::promoted_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PromotedPost {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub promotion_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub owner: String,
    #[diesel(sql_type = Varchar)]
    pub profile_id: String,
    #[diesel(sql_type = Int8)]
    pub payment_per_view: i64,
    #[diesel(sql_type = Int8)]
    pub total_budget: i64,
    #[diesel(sql_type = Int8)]
    pub remaining_budget: i64,
    #[diesel(sql_type = Bool)]
    pub active: bool,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New promoted post model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::promoted_posts)]
pub struct NewPromotedPost {
    pub promotion_id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: i64,
    pub total_budget: i64,
    pub remaining_budget: i64,
    pub active: bool,
    pub created_at: i64,
    pub transaction_id: String,
}

/// Promotion view model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::promotion_views)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PromotionView {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub promotion_id: String,
    #[diesel(sql_type = Varchar)]
    pub viewer: String,
    #[diesel(sql_type = Int8)]
    pub payment_amount: i64,
    #[diesel(sql_type = Int8)]
    pub view_duration: i64,
    #[diesel(sql_type = Varchar)]
    pub platform_id: String,
    #[diesel(sql_type = Int8)]
    pub timestamp: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New promotion view model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::promotion_views)]
pub struct NewPromotionView {
    pub post_id: String,
    pub promotion_id: String,
    pub viewer: String,
    pub payment_amount: i64,
    pub view_duration: i64,
    pub platform_id: String,
    pub timestamp: i64,
    pub transaction_id: String,
}

/// Promotion status event model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::promotion_status_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PromotionStatusEvent {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub promotion_id: String,
    #[diesel(sql_type = Varchar)]
    pub event_type: String,
    #[diesel(sql_type = Varchar)]
    pub triggered_by: String,
    #[diesel(sql_type = Nullable<Bool>)]
    pub new_status: Option<bool>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub amount: Option<i64>,
    #[diesel(sql_type = Int8)]
    pub timestamp: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New promotion status event model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::promotion_status_events)]
pub struct NewPromotionStatusEvent {
    pub post_id: String,
    pub promotion_id: String,
    pub event_type: String,
    pub triggered_by: String,
    pub new_status: Option<bool>,
    pub amount: Option<i64>,
    pub timestamp: i64,
    pub transaction_id: String,
}

/// Promotion budget event model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::promotion_budget_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PromotionBudgetEvent {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub promotion_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub event_type: String,
    #[diesel(sql_type = Int8)]
    pub amount: i64,
    #[diesel(sql_type = Int8)]
    pub remaining_budget: i64,
    #[diesel(sql_type = Int8)]
    pub timestamp: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
}

/// New promotion budget event model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::promotion_budget_events)]
pub struct NewPromotionBudgetEvent {
    pub promotion_id: String,
    pub post_id: String,
    pub event_type: String,
    pub amount: i64,
    pub remaining_budget: i64,
    pub timestamp: i64,
    pub transaction_id: String,
}

/// Post config model for database (comprehensive PostConfig settings)
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::post_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostConfig {
    pub id: i32,
    pub updated_by: String,
    pub max_content_length: i64,
    pub max_media_urls: i64,
    pub max_mentions: i64,
    pub max_metadata_size: i64,
    pub max_description_length: i64,
    pub max_reaction_length: i64,
    pub commenter_tip_percentage: i64,
    pub repost_tip_percentage: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// New post config model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::post_config)]
pub struct NewPostConfig {
    pub updated_by: String,
    pub max_content_length: i64,
    pub max_media_urls: i64,
    pub max_mentions: i64,
    pub max_metadata_size: i64,
    pub max_description_length: i64,
    pub max_reaction_length: i64,
    pub commenter_tip_percentage: i64,
    pub repost_tip_percentage: i64,
    pub version: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}
