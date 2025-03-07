// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Block list module for the social network ecosystem.
/// This module allows platforms to block users and profiles to block other profiles.
module mys::block_list {
    use std::string::{Self, String};
    use std::vector;
    use mys::object::{Self, UID, ID};
    use mys::tx_context::{Self, TxContext};
    use mys::transfer;
    use mys::event;
    use mys::table::{Self, Table};
    
    // === Errors ===
    /// Caller is not authorized
    const ENotAuthorized: u64 = 0;
    /// Entity already blocked
    const EAlreadyBlocked: u64 = 1;
    /// Entity not blocked
    const ENotBlocked: u64 = 2;
    /// Invalid block type
    const EInvalidBlockType: u64 = 3;
    
    // === Block types ===
    /// Profile blocking another profile
    const BLOCK_TYPE_PROFILE: u8 = 0;
    /// Platform blocking a profile
    const BLOCK_TYPE_PLATFORM: u8 = 1;
    
    // === Structs ===
    
    /// Block list for a profile or platform
    /// This stores the list of entities that the owner has blocked
    struct BlockList has key {
        id: UID,
        /// ID of the entity that owns this block list (profile or platform)
        owner_id: ID,
        /// Address of the owner
        owner_address: address,
        /// Type of block list (profile or platform)
        block_type: u8,
        /// List of blocked entity IDs
        blocked_entities: vector<ID>,
        /// Map from entity ID to block timestamp and reason
        block_info: Table<ID, BlockInfo>,
    }
    
    /// Information about a block
    struct BlockInfo has store, copy, drop {
        /// When the block was created
        timestamp: u64,
        /// Optional reason for the block
        reason: String,
    }
    
    /// Registry of all block lists
    struct BlockListRegistry has key {
        id: UID,
        /// Map from entity ID to their block list ID
        block_lists: Table<ID, ID>,
    }
    
    // === Events ===
    
    /// Event emitted when an entity is blocked
    struct EntityBlockedEvent has copy, drop {
        /// ID of the entity that created the block
        blocker_id: ID,
        /// Type of the blocker (profile or platform)
        blocker_type: u8,
        /// ID of the blocked entity
        blocked_id: ID,
        /// Reason for the block
        reason: String,
        /// Timestamp when the block occurred
        timestamp: u64,
    }
    
    /// Event emitted when an entity is unblocked
    struct EntityUnblockedEvent has copy, drop {
        /// ID of the entity that removed the block
        blocker_id: ID,
        /// Type of the blocker (profile or platform)
        blocker_type: u8,
        /// ID of the unblocked entity
        unblocked_id: ID,
        /// Timestamp when the unblock occurred
        timestamp: u64,
    }
    
    // === Initialization ===
    
    /// Initialize the block list system
    fun init(ctx: &mut TxContext) {
        // Create and share the registry
        transfer::share_object(
            BlockListRegistry {
                id: object::new(ctx),
                block_lists: table::new(ctx),
            }
        );
    }
    
    // === Core Functions ===
    
    /// Create a new block list for an entity (profile or platform)
    public fun create_block_list(
        owner_id: ID,
        block_type: u8,
        ctx: &mut TxContext
    ): BlockList {
        // Verify block type is valid
        assert!(
            block_type == BLOCK_TYPE_PROFILE || block_type == BLOCK_TYPE_PLATFORM,
            EInvalidBlockType
        );
        
        BlockList {
            id: object::new(ctx),
            owner_id,
            owner_address: tx_context::sender(ctx),
            block_type,
            blocked_entities: vector::empty(),
            block_info: table::new(ctx),
        }
    }
    
    /// Create and register a block list for a profile
    public entry fun create_profile_block_list(
        registry: &mut BlockListRegistry,
        profile_id: ID,
        ctx: &mut TxContext
    ) {
        // Create the block list
        let block_list = create_block_list(
            profile_id,
            BLOCK_TYPE_PROFILE,
            ctx
        );
        
        // Register in the registry
        let block_list_id = object::id(&block_list);
        table::add(&mut registry.block_lists, profile_id, block_list_id);
        
        // Transfer to sender
        transfer::transfer(block_list, tx_context::sender(ctx));
    }
    
    /// Create and register a block list for a platform
    public entry fun create_platform_block_list(
        registry: &mut BlockListRegistry,
        platform_id: ID,
        ctx: &mut TxContext
    ) {
        // Create the block list
        let block_list = create_block_list(
            platform_id,
            BLOCK_TYPE_PLATFORM,
            ctx
        );
        
        // Register in the registry
        let block_list_id = object::id(&block_list);
        table::add(&mut registry.block_lists, platform_id, block_list_id);
        
        // Transfer to sender
        transfer::transfer(block_list, tx_context::sender(ctx));
    }
    
    /// Block an entity
    public entry fun block_entity(
        block_list: &mut BlockList,
        entity_id: ID,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify caller is the owner
        assert!(tx_context::sender(ctx) == block_list.owner_address, ENotAuthorized);
        
        // Check if already blocked
        assert!(!vector::contains(&block_list.blocked_entities, &entity_id), EAlreadyBlocked);
        
        // Get current timestamp
        let timestamp = tx_context::epoch_timestamp_ms(ctx);
        
        // Add to block list
        vector::push_back(&mut block_list.blocked_entities, entity_id);
        
        // Store block info
        table::add(
            &mut block_list.block_info,
            entity_id,
            BlockInfo {
                timestamp,
                reason: string::utf8(reason),
            }
        );
        
        // Emit event
        event::emit(EntityBlockedEvent {
            blocker_id: block_list.owner_id,
            blocker_type: block_list.block_type,
            blocked_id: entity_id,
            reason: string::utf8(reason),
            timestamp,
        });
    }
    
    /// Unblock an entity
    public entry fun unblock_entity(
        block_list: &mut BlockList,
        entity_id: ID,
        ctx: &mut TxContext
    ) {
        // Verify caller is the owner
        assert!(tx_context::sender(ctx) == block_list.owner_address, ENotAuthorized);
        
        // Check if blocked
        let (is_blocked, index) = vector::index_of(&block_list.blocked_entities, &entity_id);
        assert!(is_blocked, ENotBlocked);
        
        // Remove from blocked list
        vector::remove(&mut block_list.blocked_entities, index);
        
        // Remove block info
        table::remove(&mut block_list.block_info, entity_id);
        
        // Emit event
        event::emit(EntityUnblockedEvent {
            blocker_id: block_list.owner_id,
            blocker_type: block_list.block_type,
            unblocked_id: entity_id,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    // === Query Functions ===
    
    /// Check if an entity is blocked
    public fun is_blocked(block_list: &BlockList, entity_id: ID): bool {
        vector::contains(&block_list.blocked_entities, &entity_id)
    }
    
    /// Get block info for an entity
    public fun get_block_info(block_list: &BlockList, entity_id: ID): (bool, u64, String) {
        if (table::contains(&block_list.block_info, entity_id)) {
            let info = table::borrow(&block_list.block_info, entity_id);
            (true, info.timestamp, info.reason)
        } else {
            (false, 0, string::utf8(b""))
        }
    }
    
    /// Get all blocked entities
    public fun get_blocked_entities(block_list: &BlockList): vector<ID> {
        block_list.blocked_entities
    }
    
    /// Find block list ID for an entity
    public fun find_block_list(registry: &BlockListRegistry, entity_id: ID): (bool, ID) {
        if (table::contains(&registry.block_lists, entity_id)) {
            (true, *table::borrow(&registry.block_lists, entity_id))
        } else {
            (false, object::id_from_address(@0x0))
        }
    }
    
    // === Constants for public use ===
    
    /// Get the profile block type constant
    public fun profile_block_type(): u8 {
        BLOCK_TYPE_PROFILE
    }
    
    /// Get the platform block type constant
    public fun platform_block_type(): u8 {
        BLOCK_TYPE_PLATFORM
    }
}