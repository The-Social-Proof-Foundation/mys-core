// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Represents the types of platform events in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformEventType {
    PlatformCreated,
    PlatformUpdated,
    PlatformStatusChanged,
    PlatformApprovalChanged,
    ModeratorAdded,
    ModeratorRemoved,
    UserJoined,
    UserLeft,
    UserBlocked,
    UserUnblocked,
}

/// Event emitted when platform status is changed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStatusChangedEvent {
    pub platform_id: String,
    pub new_status: u8,
    pub changed_by: String,
}