// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::{profile_offers, profile_sale_fees, profile_badges};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ===========================================================================
// PROFILE OFFERS MODELS
// ===========================================================================

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_offers)]
pub struct ProfileOffer {
    pub id: i32,
    pub profile_id: String,
    pub offeror_address: String,
    pub amount: i64,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_offers)]
pub struct NewProfileOffer {
    pub profile_id: String,
    pub offeror_address: String,
    pub amount: i64,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
    pub time: NaiveDateTime,
}

// ===========================================================================
// PROFILE SALE FEES MODELS
// ===========================================================================

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_sale_fees)]
pub struct ProfileSaleFee {
    pub id: i32,
    pub profile_id: String,
    pub offeror_address: String,
    pub previous_owner_address: String,
    pub sale_amount: i64,
    pub fee_amount: i64,
    pub fee_recipient_address: String,
    pub timestamp: i64,
    pub transaction_id: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_sale_fees)]
pub struct NewProfileSaleFee {
    pub profile_id: String,
    pub offeror_address: String,
    pub previous_owner_address: String,
    pub sale_amount: i64,
    pub fee_amount: i64,
    pub fee_recipient_address: String,
    pub timestamp: i64,
    pub transaction_id: String,
    pub time: NaiveDateTime,
}

// ===========================================================================
// PROFILE BADGES MODELS
// ===========================================================================

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_badges)]
pub struct ProfileBadge {
    pub id: i32,
    pub profile_id: String,
    pub badge_id: String,
    pub badge_name: String,
    pub badge_description: Option<String>,
    pub badge_media_url: Option<String>,
    pub badge_icon_url: Option<String>,
    pub platform_id: String,
    pub assigned_by: String,
    pub assigned_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
    pub badge_type: i16,
    pub transaction_id: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_badges)]
pub struct NewProfileBadge {
    pub profile_id: String,
    pub badge_id: String,
    pub badge_name: String,
    pub badge_description: Option<String>,
    pub badge_media_url: Option<String>,
    pub badge_icon_url: Option<String>,
    pub platform_id: String,
    pub assigned_by: String,
    pub assigned_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
    pub badge_type: i16,
    pub transaction_id: String,
    pub time: NaiveDateTime,
}

