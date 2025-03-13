// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;

use crate::schema::profiles;
use crate::schema::profile_events;

#[derive(Queryable, Insertable, Debug, Identifiable, Clone, QueryableByName)]
#[diesel(table_name = profiles, primary_key(profile_id))]
pub struct StoredProfile {
    pub profile_id: Vec<u8>,
    pub owner: Vec<u8>,
    pub display_name: String,
    pub bio: String,
    pub profile_picture: Option<String>,
    pub created_at: i64,
    pub username_nft_id: Option<Vec<u8>>,
    pub tx_sequence_number: i64,
    pub checkpoint_sequence_number: i64,
    pub timestamp_ms: i64,
}

#[derive(Queryable, Insertable, Debug, Identifiable, Clone, QueryableByName)]
#[diesel(table_name = profile_events, primary_key(tx_sequence_number, event_sequence_number))]
pub struct StoredProfileEvent {
    pub tx_sequence_number: i64,
    pub event_sequence_number: i64,
    pub profile_id: Vec<u8>,
    pub event_type: String,
    pub owner: Vec<u8>,
    pub timestamp_ms: i64,
    pub data: serde_json::Value,
}