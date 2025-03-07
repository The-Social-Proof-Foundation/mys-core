// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Integration module that connects user profiles with their tokens
module mys::profile_token {
    use std::string;
    use mys::object::{Self, UID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::user_token::{Self, AdminCap, TokenRegistry};
    use mys::coin::{Self, TreasuryCap, CoinMetadata};
    use mys::url::{Self, Url};
    
    // === Errors ===
    /// Operation can only be performed by the platform admin
    const ENotAuthorized: u64 = 0;
    /// Profile does not exist
    const EProfileNotFound: u64 = 1;
    /// Token already exists for this profile
    const ETokenAlreadyExists: u64 = 2;
    /// Token does not exist for this profile
    const ETokenNotExists: u64 = 3;
    
    // === Structs ===
    
    /// Profile Token Manager - connects profile IDs to tokens
    /// Shared object that stores the mapping
    struct ProfileTokenManager has key {
        id: UID,
        // We use the profile's creator address as the key
        // This assumes that profile creation already enforces uniqueness
    }
    
    /// Request to create a token for a profile
    /// Created by a user and approved by admin
    struct TokenCreationRequest has key, store {
        id: UID,
        creator: address,
        profile_id: address, // The profile identifier
        symbol: string::String,
        name: string::String,
        description: string::String,
        icon_url: Option<Url>,
        commission_bps: u64,
        creator_split_bps: u64,
    }
    
    // === Events ===
    
    /// Event emitted when a token creation request is submitted
    struct TokenRequestCreatedEvent has copy, drop {
        request_id: address,
        creator: address,
        profile_id: address,
        symbol: string::String,
        name: string::String,
    }
    
    /// Event emitted when a profile token is created
    struct ProfileTokenCreatedEvent has copy, drop {
        profile_id: address,
        creator: address,
        token_id: address,
        symbol: string::String,
        name: string::String,
    }
    
    // === Initialization ===
    
    /// Initialize the profile token integration
    fun init(ctx: &mut TxContext) {
        // Create and share profile token manager
        transfer::share_object(
            ProfileTokenManager {
                id: object::new(ctx),
            }
        );
    }
    
    // === User Functions ===
    
    /// Create a request for a profile token
    /// This request must be approved by the admin
    public entry fun create_token_request(
        profile_id: address,
        symbol: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        icon_url: Option<Url>,
        commission_bps: u64,
        creator_split_bps: u64,
        ctx: &mut TxContext
    ) {
        // Convert inputs to the right format
        let symbol_str = string::utf8(symbol);
        let name_str = string::utf8(name);
        let description_str = string::utf8(description);
        
        // Create the request object
        let request = TokenCreationRequest {
            id: object::new(ctx),
            creator: tx_context::sender(ctx),
            profile_id,
            symbol: symbol_str,
            name: name_str,
            description: description_str,
            icon_url,
            commission_bps,
            creator_split_bps,
        };
        
        // Emit event
        event::emit(TokenRequestCreatedEvent {
            request_id: object::uid_to_address(&request.id),
            creator: request.creator,
            profile_id: request.profile_id,
            symbol: request.symbol,
            name: request.name,
        });
        
        // Transfer the request to admin
        // In a real implementation, you would transfer to a known admin address
        // or use a shared object queue for requests
        transfer::transfer(request, tx_context::sender(ctx));
    }
    
    // === Admin Functions ===
    
    /// Approve a token request and create the token
    /// This can only be called by the admin
    public entry fun approve_token_request<TOKEN: drop>(
        admin_cap: &AdminCap,
        registry: &mut TokenRegistry,
        request: TokenCreationRequest,
        decimals: u8,
        ctx: &mut TxContext
    ) {
        // Extract request information
        let TokenCreationRequest {
            id,
            creator,
            profile_id,
            symbol,
            name,
            description,
            icon_url,
            commission_bps,
            creator_split_bps
        } = request;
        
        // Clean up request ID
        object::delete(id);
        
        // Check if user already has a token
        assert!(!user_token::has_token(registry, creator), ETokenAlreadyExists);
        
        // Create the user token
        user_token::create_user_token<TOKEN>(
            admin_cap,
            registry,
            creator,
            TOKEN {}, // One-time witness
            decimals,
            string::bytes(&symbol), // Symbol
            string::bytes(&name), // Name
            string::bytes(&description), // Description
            icon_url,
            option::some(commission_bps), 
            option::some(creator_split_bps),
            ctx
        );
        
        // Emit event for profile token creation
        event::emit(ProfileTokenCreatedEvent {
            profile_id,
            creator,
            token_id: tx_context::type_into_address<TOKEN>(),
            symbol,
            name,
        });
    }
    
    /// Create tokens for a profile directly without request
    /// Admin-only function for expedited token creation
    public entry fun create_profile_token<TOKEN: drop>(
        admin_cap: &AdminCap,
        registry: &mut TokenRegistry,
        profile_id: address,
        creator: address,
        decimals: u8,
        symbol: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        icon_url: Option<Url>,
        commission_bps: u64,
        creator_split_bps: u64,
        ctx: &mut TxContext
    ) {
        // Check if user already has a token
        assert!(!user_token::has_token(registry, creator), ETokenAlreadyExists);
        
        // Create the user token
        user_token::create_user_token<TOKEN>(
            admin_cap,
            registry,
            creator,
            TOKEN {}, // One-time witness
            decimals,
            symbol,
            name,
            description,
            icon_url,
            option::some(commission_bps),
            option::some(creator_split_bps),
            ctx
        );
        
        // Convert inputs for the event
        let symbol_str = string::utf8(symbol);
        let name_str = string::utf8(name);
        
        // Emit event for profile token creation
        event::emit(ProfileTokenCreatedEvent {
            profile_id,
            creator,
            token_id: tx_context::type_into_address<TOKEN>(),
            symbol: symbol_str,
            name: name_str,
        });
    }
    
    /// Mint tokens for initial distribution (admin only)
    public entry fun mint_profile_tokens<T>(
        admin_cap: &AdminCap,
        treasury_cap: &mut TreasuryCap<T>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {
        // Mint tokens for initial distribution or liquidity
        user_token::mint_tokens(
            admin_cap,
            treasury_cap,
            amount,
            recipient,
            ctx
        );
    }
    
    // === Helper Functions ===
    
    /// Check if a profile has a token
    public fun has_profile_token(
        registry: &TokenRegistry,
        creator: address
    ): bool {
        user_token::has_token(registry, creator)
    }
    
    /// Get profile token info
    public fun get_profile_token_info(
        registry: &TokenRegistry,
        creator: address
    ): (bool, address, u64, u64, u64) {
        let (has_token, token_info) = user_token::get_user_token_info(registry, creator);
        if (has_token) {
            (
                true,
                token_info.token_id,
                token_info.commission_bps,
                token_info.creator_split_bps,
                token_info.platform_split_bps
            )
        } else {
            (false, @0x0, 0, 0, 0)
        }
    }
}