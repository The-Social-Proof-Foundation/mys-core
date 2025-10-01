// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod blocked_events;
pub mod blocked_profiles;
pub mod platform_blocks;

pub use blocked_events::*;
pub use blocked_profiles::*;
pub use platform_blocks::*;

// Event types for compatibility
use serde::{Deserialize, Serialize};

/// Events from block_list.move - direct definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBlockEvent {
    pub blocker: String,
    pub blocked: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnblockEvent {
    pub blocker: String,
    pub unblocked: String,
}
