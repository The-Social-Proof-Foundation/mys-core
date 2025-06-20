// Copyright (c) The Social Proof Foundation LLC
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
    /// An auction was created
    AuctionCreated,
    /// A contribution was made to an auction
    AuctionContribution,
    /// An auction was finalized
    AuctionFinalized,
    /// Exchange configuration was updated
    ConfigUpdated,
}

impl SocialProofTokenEventType {
    /// Convert event type to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TokenPoolCreated => "TokenPoolCreated",
            Self::TokenBought => "TokenBought",
            Self::TokenSold => "TokenSold",
            Self::TokensAdded => "TokensAdded",
            Self::AuctionCreated => "AuctionCreated",
            Self::AuctionContribution => "AuctionContribution",
            Self::AuctionFinalized => "AuctionFinalized",
            Self::ConfigUpdated => "ConfigUpdated",
        }
    }
    
    /// Try to parse a string into an event type
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "TokenPoolCreated" => Some(Self::TokenPoolCreated),
            "TokenBought" => Some(Self::TokenBought),
            "TokenSold" => Some(Self::TokenSold),
            "TokensAdded" => Some(Self::TokensAdded), 
            "AuctionCreated" => Some(Self::AuctionCreated),
            "AuctionContribution" => Some(Self::AuctionContribution),
            "AuctionFinalized" => Some(Self::AuctionFinalized),
            "ConfigUpdated" => Some(Self::ConfigUpdated),
            _ => None,
        }
    }
} 