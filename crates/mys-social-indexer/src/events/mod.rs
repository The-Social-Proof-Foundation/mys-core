// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod blocking_events;
pub mod event_utils;
pub mod governance_event_types;
pub mod governance_events;
pub mod insurance_event_types;
pub mod insurance_events;
pub mod mydata_event_types;
pub mod mydata_events;
pub mod platform_event_types;
pub mod platform_events;
pub mod poc_event_types;
pub mod poc_events;
pub mod post_event_types;
pub mod post_events;
pub mod profile_event_types;
pub mod profile_events;
pub mod social_graph_event_types;
pub mod social_graph_events;
pub mod social_proof_of_truth_events;
pub mod social_proof_token_event_types;
pub mod social_proof_token_events;
pub mod subscription_event_types;
pub mod subscription_events;

// Re-export all profile events
pub use profile_events::{
    ProfileCreatedEvent, ProfileUpdatedEvent,
    UsernameRegisteredEvent, UsernameUpdatedEvent, TokensVestedEvent, TokensClaimedEvent,
    ProfileOfferCreatedEvent, ProfileOfferAcceptedEvent, ProfileOfferRejectedEvent,
    ProfileSaleFeeEvent, BadgeAssignedEvent, BadgeRevokedEvent,
};

// Re-export profile event types
pub use profile_event_types::{
    BlockAddedEvent, BlockRemovedEvent, PlatformJoinedEvent, PlatformLeftEvent, ProfileEventType,
};

// Re-export platform events
pub use crate::models::platform::{
    ModeratorAddedEvent,
    ModeratorRemovedEvent,
    // These are also defined in blocking models, so use those instead
    // PlatformBlockedProfileEvent,
    // PlatformUnblockedProfileEvent,
    PlatformApprovalChangedEvent,
    PlatformCreatedEvent,
    PlatformUpdatedEvent,
};

// Re-export social graph events
pub use social_graph_events::{FollowEvent, UnfollowEvent};

// Re-export blocking events
pub use crate::models::blocking::{
    // Block events
    UserBlockEvent,
    UserUnblockEvent,
};

// Re-export platform events (from models::platform)
pub use crate::models::platform::{PlatformBlockedProfileEvent, PlatformUnblockedProfileEvent};

// Re-export post events
pub use post_event_types::{
    CommentCreatedEvent, ContentUpdateEvent, DeletionEvent, ModerationEvent, PostCreatedEvent,
    PostEventType, ReactionEvent, RemoveReactionEvent, ReportEvent, RepostEvent, TipEvent,
    OwnershipTransferEvent, PostParametersUpdatedEvent,
};

// Re-export social proof token events
pub use social_proof_token_events::{
    ConfigUpdatedEvent, PocRedirectionUpdatedEvent, ReservationCreatedEvent,
    ReservationPoolCreatedEvent, ReservationWithdrawnEvent, ThresholdMetEvent,
    TokenBoughtEvent, TokenPoolCreatedEvent, TokenSoldEvent, TokensAddedEvent,
    PostPoolAutoInitializedEvent, EmergencyKillSwitchEvent,
};

// Re-export SPoT events
pub use social_proof_of_truth_events::{
    SpotBetPlacedEvent, SpotBetWithdrawnEvent, SpotConfigUpdatedEvent, SpotDaoRequiredEvent,
    SpotPayoutEvent, SpotRecordCreatedEvent, SpotRefundEvent, SpotResolvedEvent,
};

// Re-export Insurance events
pub use insurance_events::{
    ConfigInitializedEvent, CoverageCancelledEvent, CoverageClaimedEvent,
    CoveragePurchasedEvent, new_insurance_event_log, PolicyExpiredEvent,
    UnderwriterVaultCreatedEvent, UnderwriterVaultDepositedEvent,
    UnderwriterVaultWithdrawnEvent,
};
// Export insurance ConfigUpdatedEvent with a qualified name to avoid conflict with social_proof_token_events
pub use insurance_events::ConfigUpdatedEvent as InsuranceConfigUpdatedEvent;

// Re-export Insurance event types
pub use insurance_event_types::{
    EVENT_COVERAGE_CANCELLED, EVENT_COVERAGE_CLAIMED, EVENT_COVERAGE_PURCHASED,
    EVENT_CONFIG_INITIALIZED, EVENT_CONFIG_UPDATED, EVENT_POLICY_EXPIRED,
    EVENT_VAULT_CREATED, EVENT_VAULT_DEPOSITED, EVENT_VAULT_WITHDRAWN,
    InsuranceEventType, POLICY_EVENT_CANCELLED, POLICY_EVENT_CLAIMED, POLICY_EVENT_EXPIRED,
    POLICY_EVENT_PURCHASED, STATUS_ACTIVE, STATUS_CANCELLED, STATUS_CLAIMED, STATUS_EXPIRED,
    TRANSACTION_TYPE_DEPOSIT, TRANSACTION_TYPE_WITHDRAWAL,
};

// Re-export social proof token event types
pub use social_proof_token_event_types::SocialProofTokenEventType;

// Re-export PoC events
pub use poc_events::{
    validate_analysis_submitted_event, validate_badge_issued_event, validate_config_updated_event,
    validate_dispute_submitted_event, validate_redirection_activated_event,
    validate_vote_cast_event, validation,
};

// Re-export PoC event types
pub use poc_event_types::{
    AnalysisSubmittedEvent, DisputeVoteCastEvent, PocBadgeIssuedEvent, PocConfigUpdatedEvent,
    PocDisputeResolvedEvent, PocDisputeSubmittedEvent, PocEventType,
    RevenueRedirectionActivatedEvent, TokenPoolSyncNeededEvent, VotingRewardClaimedEvent,
    DISPUTE_STATUS_RESOLVED_OVERTURNED, DISPUTE_STATUS_RESOLVED_UPHELD, DISPUTE_STATUS_VOTING,
    MEDIA_TYPE_AUDIO, MEDIA_TYPE_IMAGE, MEDIA_TYPE_VIDEO, VOTE_OVERTURN, VOTE_UPHOLD,
};

// Re-export subscription events
pub use subscription_events::{
    extract_profile_owner_from_service, parse_subscription_event, sanitize_event_data,
    validate_business_rules, validate_subscription_event_detailed, SubscriptionEventError,
    MAX_MONTHLY_FEE, MAX_REFUND_AMOUNT, MIN_MONTHLY_FEE,
};

// Re-export subscription event types
pub use subscription_event_types::{
    extract_service_id, extract_subscriber, extract_subscription_id, generate_subscription_id,
    validate_subscription_event, ProfileSubscriptionCancelledEvent,
    ProfileSubscriptionCreatedEvent, ProfileSubscriptionRenewedEvent,
    ProfileSubscriptionServiceCreatedEvent, ProfileSubscriptionServiceDeactivatedEvent,
    ProfileSubscriptionUpdatedEvent, RenewalBalanceFundedEvent, SubscriptionEventType,
};

// Define placeholder event types for other modules
// These should be moved to their own module files when implemented
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestPlatformCreatedEvent {
    pub platform_id: String,
    pub name: String,
    pub description: String,
    pub creator_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentCreatedEvent {
    pub content_id: String,
    pub creator_id: String,
    pub platform_id: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentInteractionEvent {
    pub content_id: String,
    pub profile_id: String,
    pub interaction_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityBlockedEvent {
    pub blocker_id: String,
    pub blocked_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IPRegisteredEvent {
    pub mydata_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseGrantedEvent {
    pub license_id: String,
    pub mydata_id: String,
    pub payment_amount: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCreatedEvent {
    pub proof_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeeModelCreatedEvent {
    pub fee_model_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeesDistributedEvent {
    pub fee_model_id: String,
    pub total_fee_amount: u64,
}

// Implementation traits will be properly implemented when needed
// Currently stubbed out to avoid compilation errors

/*
// For PlatformCreatedEvent
impl PlatformCreatedEvent {
    pub fn into_model(&self) -> Result<()> {
        // Placeholder implementation
        unimplemented!("PlatformCreatedEvent::into_model() not implemented yet")
    }
}

// For ContentCreatedEvent
impl ContentCreatedEvent {
    pub fn into_model(&self) -> Result<()> {
        // Placeholder implementation
        unimplemented!("ContentCreatedEvent::into_model() not implemented yet")
    }
}

// For ContentInteractionEvent
impl ContentInteractionEvent {
    pub fn into_model(&self) -> Result<()> {
        // Placeholder implementation
        unimplemented!("ContentInteractionEvent::into_model() not implemented yet")
    }
}

// For EntityBlockedEvent
impl EntityBlockedEvent {
    pub fn into_model(&self) -> Result<()> {
        // Placeholder implementation
        unimplemented!("EntityBlockedEvent::into_model() not implemented yet")
    }
}

// For IPRegisteredEvent
impl IPRegisteredEvent {
    pub fn into_model(&self, _content_id: Option<String>, _creator_id: Option<String>) -> Result<()> {
        // Placeholder implementation
        unimplemented!("IPRegisteredEvent::into_model() not implemented yet")
    }
}

// For LicenseGrantedEvent
impl LicenseGrantedEvent {
    pub fn into_model(&self, _licensee_id: Option<String>) -> Result<()> {
        // Placeholder implementation
        unimplemented!("LicenseGrantedEvent::into_model() not implemented yet")
    }
}

// For FeesDistributedEvent
impl FeesDistributedEvent {
    pub fn into_model(&self) -> Result<()> {
        // Placeholder implementation
        unimplemented!("FeesDistributedEvent::into_model() not implemented yet")
    }
}
*/

use anyhow::{anyhow, Result};
use mys_json_rpc_types::MysEvent;
use mys_types::event::Event;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

// Event type prefixes for each module - all use the same MySocial package address
use crate::DEFAULT_MYSOCIAL_PACKAGE_ADDRESS;

// Helper macro to create module prefixes using the main package address
macro_rules! module_prefix {
    () => {
        DEFAULT_MYSOCIAL_PACKAGE_ADDRESS
    };
}

// All modules are in the same package - using a macro means we only need to update one place
pub const MODULE_PREFIX_PROFILE: &str = module_prefix!();
pub const MODULE_PREFIX_PLATFORM: &str = module_prefix!();
pub const MODULE_PREFIX_CONTENT: &str = module_prefix!();
pub const MODULE_PREFIX_BLOCK_LIST: &str = module_prefix!();
pub const MODULE_PREFIX_MYDATA: &str = module_prefix!();
pub const MODULE_PREFIX_FEE_DISTRIBUTION: &str = module_prefix!();
pub const MODULE_PREFIX_SOCIAL_GRAPH: &str = module_prefix!();
pub const MODULE_PREFIX_POST: &str = module_prefix!();
pub const MODULE_PREFIX_GOVERNANCE: &str = module_prefix!();
pub const MODULE_PREFIX_SOCIAL_PROOF_TOKEN: &str = module_prefix!();
pub const MODULE_PREFIX_SOCIAL_PROOF_OF_TRUTH: &str = module_prefix!();
pub const MODULE_PREFIX_POC: &str = module_prefix!();
pub const MODULE_PREFIX_SUBSCRIPTION: &str = module_prefix!();
pub const MODULE_PREFIX_INSURANCE: &str = module_prefix!();

pub use event_utils::*;

/// Parse a blockchain event into a specific event type
pub fn parse_event<T: DeserializeOwned>(event: &Event) -> Result<T> {
    // First parse the event into a JSON value
    let json_value = serde_json::to_value(event)?;

    // Then extract the fields using the event_utils method
    let fields = event_utils::extract_event_fields(&json_value)?;

    // Then deserialize them into the event type
    match serde_json::from_value::<T>(fields) {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!("Failed to parse event fields to event type: {}", e)),
    }
}

/// General event parsing to JSON value
pub fn parse_json_fields(event: &Event) -> Result<JsonValue> {
    // First parse the event into a JSON value
    let json_value = serde_json::to_value(event)?;

    // Then extract the fields using the event_utils method
    event_utils::extract_event_fields(&json_value)
}

pub use blocking_events::*;
pub use governance_event_types::*;
pub use governance_events::*;
pub use mydata_event_types::*;
pub use platform_event_types::*;
pub use profile_event_types::*;
pub use social_graph_event_types::*;

/// Parse an event that is already in JSON format
pub fn parse_json_event<T: DeserializeOwned>(event: &MysEvent) -> Result<T> {
    // First convert MysEvent to a JSON value
    let json_event = serde_json::to_value(event)?;

    // Then use the event_utils extract_event_fields on the JSON value
    let fields = event_utils::extract_event_fields(&json_event)?;

    // Then deserialize them into the event type
    match serde_json::from_value::<T>(fields) {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!("Failed to parse JSON event to event type: {}", e)),
    }
}
