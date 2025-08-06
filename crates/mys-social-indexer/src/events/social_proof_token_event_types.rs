// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social Proof Token event types enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocialProofTokenEventType {
    /// A token pool was created
    TokenPoolCreated,
    /// Tokens were bought
    TokenBought,
    /// Tokens were sold
    TokenSold,
    /// Tokens were added to an existing holding
    TokensAdded,
    /// MySo was reserved towards a post/profile
    ReservationCreated,
    /// MySo reservation was withdrawn
    ReservationWithdrawn,
    /// Staking threshold was met for the first time
    ThresholdMet,
    /// Exchange configuration was updated
    ConfigUpdated,
    /// Emergency kill switch was toggled
    EmergencyKillSwitch,
}

impl SocialProofTokenEventType {
    /// Convert event type to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TokenPoolCreated => "TokenPoolCreated",
            Self::TokenBought => "TokenBought",
            Self::TokenSold => "TokenSold",
            Self::TokensAdded => "TokensAdded",
            Self::ReservationCreated => "ReservationCreated",
            Self::ReservationWithdrawn => "ReservationWithdrawn",
            Self::ThresholdMet => "ThresholdMet",
            Self::ConfigUpdated => "ConfigUpdated",
            Self::EmergencyKillSwitch => "EmergencyKillSwitch",
        }
    }
    
    /// Try to parse a string into an event type
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "TokenPoolCreated" => Some(Self::TokenPoolCreated),
            "TokenBought" => Some(Self::TokenBought),
            "TokenSold" => Some(Self::TokenSold),
            "TokensAdded" => Some(Self::TokensAdded), 
            "ReservationCreated" => Some(Self::ReservationCreated),
            "ReservationWithdrawn" => Some(Self::ReservationWithdrawn),
            "ThresholdMet" => Some(Self::ThresholdMet),
            "ConfigUpdated" => Some(Self::ConfigUpdated),
            "EmergencyKillSwitch" => Some(Self::EmergencyKillSwitch),
            _ => None,
        }
    }
} 