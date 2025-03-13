// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::Page;
use mys_types::base_types::{ObjectDigest, ObjectID, SequenceNumber, TransactionDigest, MysAddress};
// Remove unused imports
use mys_types::mys_serde::SequenceNumber as AsSequenceNumber;

/// Type for paginated profile results
pub type ProfilePage = Page<ProfileData, String>;

/// Profile data structure for RPC responses
#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileData {
    /// The profile object ID
    pub profile_id: ObjectID,
    /// The sequence number of the profile object
    #[schemars(with = "AsSequenceNumber")]
    #[serde_as(as = "AsSequenceNumber")]
    pub version: SequenceNumber,
    /// Object digest
    pub digest: ObjectDigest,
    /// Display name of the profile
    pub display_name: String,
    /// Bio of the profile
    pub bio: String,
    /// Profile picture URL
    pub profile_picture: Option<String>,
    /// Profile creation timestamp
    pub created_at: u64,
    /// Profile owner address
    pub owner: MysAddress,
    /// Username associated with the profile, if any
    pub username: Option<String>,
    /// Transaction digest that created or last updated the profile
    pub previous_transaction: TransactionDigest,
}

impl ProfileData {
    /// Returns the object reference (ID, sequence, digest) of the profile
    pub fn object_ref(&self) -> (ObjectID, SequenceNumber, ObjectDigest) {
        (self.profile_id, self.version, self.digest)
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileEvent {
    /// Type of the profile event
    pub event_type: ProfileEventType,
    /// The profile object ID
    pub profile_id: ObjectID,
    /// Profile display name
    pub display_name: Option<String>,
    /// Profile owner address
    pub owner: MysAddress,
    /// Optional old username value in case of username update
    pub old_username: Option<String>,
    /// Optional new username value in case of username update or assignment
    pub new_username: Option<String>,
    /// Optional username object ID in case of username NFT assignment or removal
    pub username_id: Option<ObjectID>,
    /// Timestamp when the event occurred
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProfileEventType {
    /// Profile created event
    Created,
    /// Profile updated event
    Updated,
    /// Username updated event
    UsernameUpdated,
    /// Username NFT assigned event
    UsernameNftAssigned,
    /// Username NFT removed event
    UsernameNftRemoved,
}

impl fmt::Display for ProfileEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "Created"),
            Self::Updated => write!(f, "Updated"),
            Self::UsernameUpdated => write!(f, "UsernameUpdated"),
            Self::UsernameNftAssigned => write!(f, "UsernameNftAssigned"),
            Self::UsernameNftRemoved => write!(f, "UsernameNftRemoved"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProfileCursor {
    pub profile_id: ObjectID,
    pub created_at: u64,
}

impl ProfileCursor {
    pub fn new(profile_id: ObjectID, created_at: u64) -> Self {
        Self {
            profile_id,
            created_at,
        }
    }

    pub fn encode(&self) -> String {
        use base64::prelude::*;
        let json = serde_json::to_string(self).unwrap();
        BASE64_STANDARD.encode(json)
    }

    pub fn decode(cursor: &str) -> Option<Self> {
        use base64::prelude::*;
        let bytes = BASE64_STANDARD.decode(cursor).ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}