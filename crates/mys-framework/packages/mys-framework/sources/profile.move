// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

/// Profile module for the MySocial network
/// Handles user identity, profile creation and management
#[allow(unused_const, duplicate_alias)]
module mys::profile {
    use std::string::String;
    
    use mys::object;
    use mys::tx_context;
    use mys::event;
    use mys::transfer;
    use mys::url::{Self, Url};
    use mys::dynamic_field;

    /// Error codes
    const EProfileAlreadyExists: u64 = 0;
    const EUnauthorized: u64 = 1;
    const EUsernameAlreadySet: u64 = 2;
    const EUsernameNotRegistered: u64 = 3;
    const EInvalidUsername: u64 = 4;
    const ENameRegistryMismatch: u64 = 5;

    /// Field names for dynamic fields
    const USERNAME_NFT_FIELD: vector<u8> = b"username_nft";

    /// Profile object that contains user information
    /// Note: Profile is deliberately not transferable (no 'store' ability)
    #[allow(unused_field)]
    public struct Profile has key {
        id: object::UID,
        /// Display name of the profile
        display_name: String,
        /// Bio of the profile
        bio: String,
        /// Profile picture URL
        profile_picture: std::option::Option<Url>,
        /// Profile creation timestamp
        created_at: u64,
        /// Profile owner address
        owner: address,
    }

    /// Profile created event
    #[allow(unused_field)]
    public struct ProfileCreatedEvent has copy, drop {
        profile_id: address,
        display_name: String,
        owner: address,
    }

    /// Profile updated event
    #[allow(unused_field)]
    public struct ProfileUpdatedEvent has copy, drop {
        profile_id: address,
        display_name: String,
        owner: address,
    }

    /// Username updated event
    #[allow(unused_field)]
    public struct UsernameUpdatedEvent has copy, drop {
        profile_id: address,
        old_username: String,
        new_username: String,
        owner: address,
    }
    
    /// Username NFT assigned event
    #[allow(unused_field)]
    public struct UsernameNFTAssignedEvent has copy, drop {
        profile_id: address,
        username_id: address,
        username: String,
        assigned_at: u64,
    }
    
    /// Username NFT removed event
    #[allow(unused_field)]
    public struct UsernameNFTRemovedEvent has copy, drop {
        profile_id: address,
        username_id: address,
        removed_at: u64,
    }

    /// Create a new profile
    public fun create_profile(
        display_name: String,
        bio: String,
        profile_picture: std::option::Option<Url>,
        ctx: &mut tx_context::TxContext
    ): Profile {
        let owner = tx_context::sender(ctx);
        let now = tx_context::epoch(ctx);
        
        let profile = Profile {
            id: object::new(ctx),
            display_name,
            bio,
            profile_picture,
            created_at: now,
            owner,
        };

        event::emit(ProfileCreatedEvent {
            profile_id: object::uid_to_address(&profile.id),
            display_name: profile.display_name,
            owner,
        });

        profile
    }

    /// Create a new profile and transfer to sender
    public entry fun create_and_register_profile(
        display_name: String,
        bio: String,
        profile_picture_url: vector<u8>,
        ctx: &mut tx_context::TxContext
    ) {
        let profile_picture = if (std::vector::length(&profile_picture_url) > 0) {
            std::option::some(url::new_unsafe_from_bytes(profile_picture_url))
        } else {
            std::option::none()
        };

        let profile = create_profile(
            display_name,
            bio,
            profile_picture,
            ctx
        );

        transfer::transfer(profile, tx_context::sender(ctx));
    }

    /// Update profile information
    public entry fun update_profile(
        profile: &mut Profile,
        new_display_name: String,
        new_bio: String,
        new_profile_picture_url: vector<u8>,
        ctx: &mut tx_context::TxContext
    ) {
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);

        profile.display_name = new_display_name;
        profile.bio = new_bio;
        
        if (std::vector::length(&new_profile_picture_url) > 0) {
            profile.profile_picture = std::option::some(url::new_unsafe_from_bytes(new_profile_picture_url));
        };

        event::emit(ProfileUpdatedEvent {
            profile_id: object::uid_to_address(&profile.id),
            display_name: profile.display_name,
            owner: profile.owner,
        });
    }

    // === Getters ===

    /// Get the display name of a profile
    public fun display_name(profile: &Profile): String {
        profile.display_name
    }

    /// Get the bio of a profile
    public fun bio(profile: &Profile): String {
        profile.bio
    }

    /// Get the profile picture URL of a profile
    public fun profile_picture(profile: &Profile): &std::option::Option<Url> {
        &profile.profile_picture
    }

    /// Get the creation timestamp of a profile
    public fun created_at(profile: &Profile): u64 {
        profile.created_at
    }

    /// Get the owner of a profile
    public fun owner(profile: &Profile): address {
        profile.owner
    }

    /// Get the ID of a profile
    public fun id(profile: &Profile): &object::UID {
        &profile.id
    }
    
    /// Check if a profile has a username NFT reference
    public fun has_username_nft(profile: &Profile): bool {
        dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)
    }
    
    /// Get the username NFT ID associated with this profile
    public fun username_nft_id(profile: &Profile): std::option::Option<address> {
        if (dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)) {
            std::option::some(*dynamic_field::borrow<vector<u8>, address>(&profile.id, USERNAME_NFT_FIELD))
        } else {
            std::option::none()
        }
    }
}