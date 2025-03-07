module mys_framework::profile {
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use std::vector;
    use mys_framework::object::{Self, UID};
    use mys_framework::tx_context::{Self, TxContext};
    use mys_framework::event;
    use mys_framework::transfer;
    use mys_framework::url::{Self, Url};
    use mys_framework::display;
    use mys_framework::coin::{Self, Coin};
    use mys_framework::mys::MYS;
    use mys_framework::clock::{Self, Clock};
    use mys_framework::name_service::{Self, Username, NameRegistry};
    use mys_framework::dynamic_field;

    /// Error codes
    const EProfileAlreadyExists: u64 = 0;
    const EUnauthorized: u64 = 1;
    const EUsernameAlreadySet: u64 = 2;
    const EUsernameNotRegistered: u64 = 3;
    const EInvalidUsername: u64 = 4;
    const ENameRegistryMismatch: u64 = 5;
    const EInsufficientPayment: u64 = 6;

    /// Field names for dynamic fields
    const USERNAME_NFT_FIELD: vector<u8> = b"username_nft";

    /// Profile object that contains user information
    /// Note: Profile is deliberately not transferable (no 'store' ability)
    struct Profile has key {
        id: UID,
        /// Display name of the profile
        display_name: String,
        /// Bio of the profile
        bio: String,
        /// Profile picture URL
        profile_picture: Option<Url>,
        /// Profile creation timestamp
        created_at: u64,
        /// Profile owner address
        owner: address,
    }

    /// Profile created event
    struct ProfileCreatedEvent has copy, drop {
        profile_id: address,
        display_name: String,
        owner: address,
    }

    /// Profile updated event
    struct ProfileUpdatedEvent has copy, drop {
        profile_id: address,
        display_name: String,
        owner: address,
    }

    /// Username updated event
    struct UsernameUpdatedEvent has copy, drop {
        profile_id: address,
        old_username: String,
        new_username: String,
        owner: address,
    }
    
    /// Username NFT assigned event
    struct UsernameNFTAssignedEvent has copy, drop {
        profile_id: address,
        username_id: address,
        username: String,
        assigned_at: u64,
    }
    
    /// Username NFT removed event
    struct UsernameNFTRemovedEvent has copy, drop {
        profile_id: address,
        username_id: address,
        removed_at: u64,
    }

    /// Create a new profile
    public fun create_profile(
        display_name: String,
        bio: String,
        profile_picture: Option<Url>,
        ctx: &mut TxContext
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
        ctx: &mut TxContext
    ) {
        let profile_picture = if (vector::length(&profile_picture_url) > 0) {
            option::some(url::new_unsafe_from_bytes(profile_picture_url))
        } else {
            option::none()
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
        ctx: &mut TxContext
    ) {
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);

        profile.display_name = new_display_name;
        profile.bio = new_bio;
        
        if (vector::length(&new_profile_picture_url) > 0) {
            profile.profile_picture = option::some(url::new_unsafe_from_bytes(new_profile_picture_url));
        };

        event::emit(ProfileUpdatedEvent {
            profile_id: object::uid_to_address(&profile.id),
            display_name: profile.display_name,
            owner: profile.owner,
        });
    }
    
    /// Register a username and assign it to a profile 
    /// This is a wrapper around name_service::register_and_assign_username
    public entry fun register_and_assign_username(
        registry: &mut NameRegistry,
        profile: &mut Profile,
        name: String,
        payment: &mut Coin<MYS>,
        duration_years: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the profile
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Get the profile ID
        let profile_id = object::uid_to_address(&profile.id);
        
        // Register and assign the username through the name service
        name_service::register_and_assign_username(
            registry,
            profile,
            name,
            payment,
            duration_years,
            clock,
            ctx
        );
        
        // Get the username ID from the name registry
        let username_id_opt = name_service::get_username_for_profile(registry, profile_id);
        assert!(option::is_some(&username_id_opt), ENameRegistryMismatch);
        let username_id = option::extract(&mut username_id_opt);
        
        // Store the username NFT reference in the profile
        if (dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)) {
            dynamic_field::remove<vector<u8>, address>(&mut profile.id, USERNAME_NFT_FIELD);
        };
        dynamic_field::add(&mut profile.id, USERNAME_NFT_FIELD, username_id);
        
        // Emit username NFT assigned event
        event::emit(UsernameNFTAssignedEvent {
            profile_id,
            username_id,
            username: name,
            assigned_at: tx_context::epoch_timestamp_ms() / 1000,
        });
    }
    
    /// Assign an existing username to a profile
    /// This is a wrapper around name_service::assign_to_profile
    public entry fun assign_username_to_profile(
        registry: &mut NameRegistry,
        username: &mut Username,
        profile: &mut Profile,
        ctx: &mut TxContext
    ) {
        // Verify caller owns both the username and profile
        assert!(name_service::owner(username) == tx_context::sender(ctx), EUnauthorized);
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Verify username isn't already assigned to a profile
        let profile_id_opt = name_service::get_profile_id(username);
        assert!(option::is_none(profile_id_opt), EUsernameAlreadySet);
        
        // Assign the username to the profile through the name service
        name_service::assign_to_profile(registry, username, profile, ctx);
        
        // Get the username ID
        let username_id = object::uid_to_address(name_service::id(username));
        
        // Store the username NFT reference in the profile
        if (dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)) {
            dynamic_field::remove<vector<u8>, address>(&mut profile.id, USERNAME_NFT_FIELD);
        };
        dynamic_field::add(&mut profile.id, USERNAME_NFT_FIELD, username_id);
        
        // Emit username NFT assigned event
        event::emit(UsernameNFTAssignedEvent {
            profile_id: object::uid_to_address(&profile.id),
            username_id,
            username: name_service::name(username),
            assigned_at: tx_context::epoch_timestamp_ms() / 1000,
        });
    }
    
    /// Remove a username from a profile
    /// This is a wrapper around name_service::unassign_from_profile
    public entry fun remove_username_from_profile(
        registry: &mut NameRegistry,
        username: &mut Username,
        profile: &mut Profile,
        ctx: &mut TxContext
    ) {
        // Verify caller owns both the username and profile
        assert!(name_service::owner(username) == tx_context::sender(ctx), EUnauthorized);
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Get the profile ID
        let profile_id = object::uid_to_address(&profile.id);
        
        // Get the current profile assignment from the username
        let current_profile_id_opt = name_service::get_profile_id(username);
        
        // Verify the username is assigned to this profile
        assert!(option::is_some(&current_profile_id_opt), ENameRegistryMismatch);
        let current_profile_id = option::extract(&mut current_profile_id_opt);
        assert!(current_profile_id == profile_id, ENameRegistryMismatch);
        
        // Unassign the username from the profile in the name service
        name_service::unassign_from_profile(registry, username, ctx);
        
        // Remove the username NFT reference from the profile
        if (dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)) {
            let username_id = dynamic_field::remove<vector<u8>, address>(&mut profile.id, USERNAME_NFT_FIELD);
            
            // Emit username NFT removed event
            event::emit(UsernameNFTRemovedEvent {
                profile_id,
                username_id,
                removed_at: tx_context::epoch_timestamp_ms() / 1000,
            });
        };
    }
    
    // Removed legacy set_username function as it's no longer needed
    
    /// Sync a profile's username reference with its assigned name service username
    /// This function ensures that the profile's dynamic field reference matches what's
    /// in the name service registry
    public entry fun sync_username_from_nameservice(
        registry: &NameRegistry,
        profile: &mut Profile,
        username: &Username,
        ctx: &mut TxContext
    ) {
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Get the profile ID
        let profile_id = object::uid_to_address(&profile.id);
        
        // Get the username's assigned profile
        let assigned_profile_id_opt = name_service::get_profile_id(username);
        
        // Verify this username is assigned to this profile
        assert!(option::is_some(&assigned_profile_id_opt), EUsernameNotRegistered);
        let assigned_profile_id = option::extract(&mut assigned_profile_id_opt);
        assert!(assigned_profile_id == profile_id, EUnauthorized);
        
        // Get the username ID
        let username_id = object::uid_to_address(name_service::id(username));
        
        // Update the username NFT reference in the profile
        if (dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)) {
            dynamic_field::remove<vector<u8>, address>(&mut profile.id, USERNAME_NFT_FIELD);
        };
        dynamic_field::add(&mut profile.id, USERNAME_NFT_FIELD, username_id);
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
    public fun profile_picture(profile: &Profile): &Option<Url> {
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
    public fun id(profile: &Profile): &UID {
        &profile.id
    }
    
    /// Check if a profile has a username NFT reference
    public fun has_username_nft(profile: &Profile): bool {
        dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)
    }
    
    /// Get the username NFT ID associated with this profile
    public fun username_nft_id(profile: &Profile): Option<address> {
        if (dynamic_field::exists_(&profile.id, USERNAME_NFT_FIELD)) {
            option::some(*dynamic_field::borrow<vector<u8>, address>(&profile.id, USERNAME_NFT_FIELD))
        } else {
            option::none()
        }
    }
    
    /// Get the username information for a profile from the name service
    /// This is the recommended way to check if a profile has a username
    public fun get_username_info(
        registry: &NameRegistry, 
        profile: &Profile
    ): (bool, Option<address>) {
        let profile_id = object::uid_to_address(&profile.id);
        let username_id_opt = name_service::get_username_for_profile(registry, profile_id);
        
        if (option::is_none(&username_id_opt)) {
            return (false, option::none())
        };
        
        // We have a username assigned in the registry
        let username_id = option::extract(&mut option::some(*option::borrow(&username_id_opt)));
        
        // Return (has_username, username_id)
        (true, option::some(username_id))
    }
    
    // Migration function removed as it's no longer needed with the username field removal
}