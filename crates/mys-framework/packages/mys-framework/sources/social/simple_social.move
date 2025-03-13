// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// A simple social module for testing and demonstration
module mys::simple_social {
    use std::string::String;
    use mys::object::{Self, UID};
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::transfer;

    // Error codes
    const EInvalidValue: u64 = 0;
    const EInvalidUpdate: u64 = 1;
    
    // A simple value storage struct
    struct SocialValue has key, store {
        id: UID,
        value: u64,
        owner: address,
    }
    
    // Event definitions
    struct ValueCreatedEvent has copy, drop {
        value_id: address,
        initial_value: u64,
        owner: address,
    }
    
    struct ValueUpdatedEvent has copy, drop {
        value_id: address,
        old_value: u64,
        new_value: u64,
        owner: address,
    }
    
    // Create a new social value
    public fun create_value(
        initial_value: u64,
        ctx: &mut TxContext
    ) {
        assert!(initial_value > 0, EInvalidValue);
        
        let owner = tx_context::sender(ctx);
        let id = object::new(ctx);
        let value_id = object::uid_to_address(&id);
        
        let social_value = SocialValue {
            id,
            value: initial_value,
            owner,
        };
        
        // Emit creation event
        event::emit(ValueCreatedEvent {
            value_id,
            initial_value,
            owner,
        });
        
        // Transfer to the sender
        transfer::transfer(social_value, owner);
    }
    
    // Update an existing social value
    public fun update_value(
        social_value: &mut SocialValue,
        new_value: u64,
        ctx: &mut TxContext
    ) {
        // Only owner can update
        assert!(social_value.owner == tx_context::sender(ctx), EInvalidUpdate);
        
        let old_value = social_value.value;
        
        // Update the value
        social_value.value = new_value;
        
        // Emit update event
        event::emit(ValueUpdatedEvent {
            value_id: object::uid_to_address(&social_value.id),
            old_value,
            new_value,
            owner: social_value.owner,
        });
    }
    
    // Get the value
    public fun get_value(social_value: &SocialValue): u64 {
        social_value.value
    }
    
    // Get the owner
    public fun get_owner(social_value: &SocialValue): address {
        social_value.owner
    }
}