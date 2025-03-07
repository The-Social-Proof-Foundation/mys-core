module mys_framework::name_service {
    use std::string::{Self, String};
    use std::vector;
    use std::option::{Self, Option};
    
    use mys_framework::object::{Self, UID};
    use mys_framework::tx_context::{Self, TxContext};
    use mys_framework::event;
    use mys_framework::transfer;
    use mys_framework::table::{Self, Table};
    use mys_framework::coin::{Self, Coin};
    use mys_framework::balance::{Self, Balance};
    use mys_framework::mys::MYS;
    use mys_framework::clock::{Self, Clock};
    use mys_framework::profile::{Self, Profile};
    use mys_framework::display;
    
    /// Error codes
    const EUnauthorized: u64 = 0;
    const ENameRegistered: u64 = 1;
    const ENameNotRegistered: u64 = 2;
    const EInvalidName: u64 = 3;
    const EReservedName: u64 = 4;
    const EProfileHasName: u64 = 5;
    const EInsufficientPayment: u64 = 6;
    const ENameExpired: u64 = 7;
    const ENameNotExpired: u64 = 8;
    const ENameNotTransferable: u64 = 9;
    const EPriceNotSet: u64 = 10;
    const EInvalidDuration: u64 = 11;
    
    /// Name length categories for pricing
    const NAME_LENGTH_ULTRA_SHORT: u8 = 0;    // 2-4 characters
    const NAME_LENGTH_SHORT: u8 = 1;          // 5-7 characters
    const NAME_LENGTH_MEDIUM: u8 = 2;         // 8-12 characters
    const NAME_LENGTH_LONG: u8 = 3;           // 13+ characters
    
    /// Duration in seconds
    const SECONDS_PER_DAY: u64 = 86400;
    const SECONDS_PER_YEAR: u64 = 31536000;   // 365 days
    
    /// Name registration status
    const NAME_STATUS_ACTIVE: u8 = 0;
    const NAME_STATUS_EXPIRED: u8 = 1;
    const NAME_STATUS_RESERVED: u8 = 2;
    
    /// Username NFT representing a registered name
    struct Username has key, store {
        id: UID,
        /// The actual username string
        name: String,
        /// The profile this username is assigned to (if any)
        profile_id: Option<address>,
        /// Original registration timestamp
        registered_at: u64,
        /// Expiration timestamp
        expires_at: u64,
        /// Last renewal timestamp
        last_renewal: u64,
        /// Status of the name
        status: u8,
        /// Flag if name is transferable
        transferable: bool,
        /// ID of the owner's profile
        owner: address,
        /// Price if listed for sale (0 if not for sale)
        sale_price: u64,
    }
    
    /// Registry for the name service
    struct NameRegistry has key {
        id: UID,
        /// Table mapping name strings to their registration IDs
        names: Table<String, address>,
        /// Table mapping profile IDs to their username IDs
        profile_names: Table<address, address>,
        /// Table of reserved names that can't be registered
        reserved_names: Table<String, bool>,
        /// Table of premium names and their prices
        premium_names: Table<String, u64>,
        /// Price per year by name length category (in MYS tokens)
        price_by_length: Table<u8, u64>,
        /// Admin address (revenue recipient)
        admin: address,
        /// Total names registered
        total_names: u64,
        /// Treasury balance
        treasury: Balance<MYS>,
    }
    
    /// Cap for administrative control
    struct NameServiceCap has key, store {
        id: UID,
    }
    
    /// Events
    
    /// Event emitted when a name is registered
    struct NameRegisteredEvent has copy, drop {
        name: String,
        username_id: address,
        owner: address,
        profile_id: Option<address>,
        registered_at: u64,
        expires_at: u64,
    }
    
    /// Event emitted when a name is renewed
    struct NameRenewedEvent has copy, drop {
        name: String,
        username_id: address,
        owner: address,
        renewed_at: u64,
        expires_at: u64,
    }
    
    /// Event emitted when a name is transferred
    struct NameTransferredEvent has copy, drop {
        name: String,
        username_id: address,
        from: address,
        to: address,
        transferred_at: u64,
    }
    
    /// Event emitted when a name is assigned to a profile
    struct NameAssignedEvent has copy, drop {
        name: String,
        username_id: address,
        profile_id: address,
        assigned_at: u64,
    }
    
    /// Event emitted when a name is listed for sale
    struct NameListedEvent has copy, drop {
        name: String,
        username_id: address,
        owner: address,
        price: u64,
        listed_at: u64,
    }
    
    /// Event emitted when a name is purchased
    struct NamePurchasedEvent has copy, drop {
        name: String,
        username_id: address,
        previous_owner: address,
        new_owner: address,
        price: u64,
        purchased_at: u64,
    }
    
    /// Initialize the name service
    fun init_module(ctx: &mut TxContext) {
        let registry = NameRegistry {
            id: object::new(ctx),
            names: table::new(ctx),
            profile_names: table::new(ctx),
            reserved_names: table::new(ctx),
            premium_names: table::new(ctx),
            price_by_length: table::new(ctx),
            admin: tx_context::sender(ctx),
            total_names: 0,
            treasury: balance::zero(),
        };
        
        // Set default prices by name length
        table::add(&mut registry.price_by_length, NAME_LENGTH_ULTRA_SHORT, 10000000000); // 10,000 MYS
        table::add(&mut registry.price_by_length, NAME_LENGTH_SHORT, 1000000000);       // 1,000 MYS
        table::add(&mut registry.price_by_length, NAME_LENGTH_MEDIUM, 100000000);       // 100 MYS
        table::add(&mut registry.price_by_length, NAME_LENGTH_LONG, 0);                 // Free
        
        // Create admin capability
        let cap = NameServiceCap {
            id: object::new(ctx),
        };
        
        // Share registry object
        transfer::share_object(registry);
        
        // Transfer admin cap to sender
        transfer::transfer(cap, tx_context::sender(ctx));
        
        // Setup display for Username NFTs
        let publisher = tx_context::sender(ctx);
        let keys = vector[
            string::utf8(b"name"),
            string::utf8(b"image_url"),
            string::utf8(b"description"),
            string::utf8(b"project_url"),
            string::utf8(b"creator")
        ];
        let values = vector[
            string::utf8(b"{name}"),
            string::utf8(b"https://mysocial.network/username/{name}.png"),
            string::utf8(b"MySocial verified username: @{name}"),
            string::utf8(b"https://mysocial.network"),
            string::utf8(b"MySocial Network")
        ];
        let display = display::new_with_fields<Username>(
            &publisher, keys, values, ctx
        );
        display::update_version(&mut display);
    }
    
    /// Register a new username
    public fun register_username(
        registry: &mut NameRegistry,
        name: String,
        payment: &mut Coin<MYS>,
        duration_years: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): Username {
        // Validate the name
        validate_name(&name);
        
        // Check if name is already registered
        assert!(!table::contains(&registry.names, name), ENameRegistered);
        
        // Check if name is reserved
        assert!(!table::contains(&registry.reserved_names, name), EReservedName);
        
        // Calculate price based on name length
        let price = get_name_price(registry, &name, duration_years);
        
        // Verify payment
        assert!(coin::value(payment) >= price, EInsufficientPayment);
        
        // Get current time
        let now = clock::timestamp_ms(clock) / 1000; // Convert ms to seconds
        
        // Calculate expiration timestamp
        let expires_at = now + (duration_years * SECONDS_PER_YEAR);
        
        // Create username NFT
        let username = Username {
            id: object::new(ctx),
            name: name,
            profile_id: option::none(),
            registered_at: now,
            expires_at: expires_at,
            last_renewal: now,
            status: NAME_STATUS_ACTIVE,
            transferable: true,
            owner: tx_context::sender(ctx),
            sale_price: 0, // not for sale by default
        };
        
        let username_id = object::uid_to_address(&username.id);
        
        // Add name to registry
        table::add(&mut registry.names, username.name, username_id);
        
        // Increment total names counter
        registry.total_names = registry.total_names + 1;
        
        // Process payment
        if (price > 0) {
            let payment_coin = coin::split(payment, price, ctx);
            let payment_balance = coin::into_balance(payment_coin);
            balance::join(&mut registry.treasury, payment_balance);
        };
        
        // Emit registration event
        event::emit(NameRegisteredEvent {
            name: username.name,
            username_id,
            owner: username.owner,
            profile_id: username.profile_id,
            registered_at: username.registered_at,
            expires_at: username.expires_at,
        });
        
        username
    }
    
    /// Register a username and assign it to sender's profile
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
        assert!(profile::owner(profile) == tx_context::sender(ctx), EUnauthorized);
        
        // Make sure profile doesn't already have a name
        let profile_id = object::uid_to_address(profile::id(profile));
        assert!(!table::contains(&registry.profile_names, profile_id), EProfileHasName);
        
        // Register the username
        let username = register_username(
            registry,
            name,
            payment,
            duration_years,
            clock,
            ctx
        );
        
        // Assign to profile
        assign_to_profile_internal(registry, &mut username, profile);
        
        // Transfer username to sender
        transfer::transfer(username, tx_context::sender(ctx));
    }
    
    /// Renew an existing username
    public entry fun renew_username(
        registry: &mut NameRegistry,
        username: &mut Username,
        payment: &mut Coin<MYS>,
        duration_years: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the username
        assert!(username.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Calculate renewal price based on name length
        let price = get_name_price(registry, &username.name, duration_years);
        
        // Verify payment
        assert!(coin::value(payment) >= price, EInsufficientPayment);
        
        // Get current time
        let now = clock::timestamp_ms(clock) / 1000; // Convert ms to seconds
        
        // Calculate new expiration timestamp
        // If expired, start from now, otherwise extend current expiration
        let new_expires_at = if (username.expires_at < now) {
            now + (duration_years * SECONDS_PER_YEAR)
        } else {
            username.expires_at + (duration_years * SECONDS_PER_YEAR)
        };
        
        // Update username
        username.expires_at = new_expires_at;
        username.last_renewal = now;
        username.status = NAME_STATUS_ACTIVE;
        
        // Process payment
        if (price > 0) {
            let payment_coin = coin::split(payment, price, ctx);
            let payment_balance = coin::into_balance(payment_coin);
            balance::join(&mut registry.treasury, payment_balance);
        };
        
        // Emit renewal event
        event::emit(NameRenewedEvent {
            name: username.name,
            username_id: object::uid_to_address(&username.id),
            owner: username.owner,
            renewed_at: now,
            expires_at: new_expires_at,
        });
    }
    
    /// Assign a username to a profile
    public entry fun assign_to_profile(
        registry: &mut NameRegistry,
        username: &mut Username,
        profile: &Profile,
        ctx: &mut TxContext
    ) {
        // Verify caller owns both the username and profile
        assert!(username.owner == tx_context::sender(ctx), EUnauthorized);
        assert!(profile::owner(profile) == tx_context::sender(ctx), EUnauthorized);
        
        // Assign username to profile
        assign_to_profile_internal(registry, username, profile);
    }
    
    /// Unassign username from profile
    public entry fun unassign_from_profile(
        registry: &mut NameRegistry,
        username: &mut Username,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the username
        assert!(username.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Only proceed if username is assigned to a profile
        if (option::is_some(&username.profile_id)) {
            let profile_id = option::extract(&mut username.profile_id);
            
            // Only remove from table if it matches (defensive)
            if (table::contains(&registry.profile_names, profile_id) &&
                *table::borrow(&registry.profile_names, profile_id) == object::uid_to_address(&username.id)) {
                table::remove(&mut registry.profile_names, profile_id);
            };
        };
    }
    
    /// Internal function to assign a username to a profile
    fun assign_to_profile_internal(
        registry: &mut NameRegistry,
        username: &mut Username,
        profile: &Profile
    ) {
        let profile_id = object::uid_to_address(profile::id(profile));
        
        // Remove existing profile-name mapping if this profile already has a name
        if (table::contains(&registry.profile_names, profile_id)) {
            let previous_username_id = *table::borrow(&registry.profile_names, profile_id);
            // Note: In a real implementation, we would need to update the previous username's profile_id field,
            // but that would require having a reference to it, which we don't in this context
        };
        
        // Remove existing username-profile mapping if this username is already assigned
        if (option::is_some(&username.profile_id)) {
            let previous_profile_id = option::extract(&mut username.profile_id);
            // Only remove from table if it matches (defensive)
            if (table::contains(&registry.profile_names, previous_profile_id) && 
                *table::borrow(&registry.profile_names, previous_profile_id) == object::uid_to_address(&username.id)) {
                table::remove(&mut registry.profile_names, previous_profile_id);
            };
        };
        
        // Assign the username to the profile
        username.profile_id = option::some(profile_id);
        
        // Update registry
        table::upsert(&mut registry.profile_names, profile_id, object::uid_to_address(&username.id));
        
        // Emit assignment event
        event::emit(NameAssignedEvent {
            name: username.name,
            username_id: object::uid_to_address(&username.id),
            profile_id,
            assigned_at: tx_context::epoch_timestamp_ms() / 1000,
        });
    }
    
    /// List a username for sale
    public entry fun list_for_sale(
        username: &mut Username,
        price: u64,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the username
        assert!(username.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Verify username is transferable
        assert!(username.transferable, ENameNotTransferable);
        
        // Set sale price
        username.sale_price = price;
        
        // Emit listing event
        event::emit(NameListedEvent {
            name: username.name,
            username_id: object::uid_to_address(&username.id),
            owner: username.owner,
            price,
            listed_at: tx_context::epoch_timestamp_ms() / 1000,
        });
    }
    
    /// Cancel a listing
    public entry fun cancel_listing(
        username: &mut Username,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the username
        assert!(username.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Remove listing
        username.sale_price = 0;
    }
    
    /// Purchase a username that's listed for sale
    public entry fun purchase_username(
        registry: &mut NameRegistry,
        username: &mut Username,
        payment: &mut Coin<MYS>,
        ctx: &mut TxContext
    ) {
        // Verify username is for sale
        assert!(username.sale_price > 0, EPriceNotSet);
        
        // Verify payment
        assert!(coin::value(payment) >= username.sale_price, EInsufficientPayment);
        
        // Process payment - split to admin fee and owner payment
        let price = username.sale_price;
        let payment_coin = coin::split(payment, price, ctx);
        
        // Calculate platform fee (10%)
        let platform_fee = price / 10;
        let seller_payment = price - platform_fee;
        
        // Split payment for platform fee
        let platform_coin = coin::split(&mut payment_coin, platform_fee, ctx);
        let platform_balance = coin::into_balance(platform_coin);
        
        // Add platform fee to treasury
        balance::join(&mut registry.treasury, platform_balance);
        
        // Transfer remaining payment to current owner
        transfer::public_transfer(payment_coin, username.owner);
        
        let previous_owner = username.owner;
        
        // Update username ownership
        username.owner = tx_context::sender(ctx);
        username.sale_price = 0; // No longer for sale
        
        // If username was assigned to a profile, unassign it
        if (option::is_some(&username.profile_id)) {
            let profile_id = option::extract(&mut username.profile_id);
            
            // Only remove from table if it matches (defensive)
            if (table::contains(&registry.profile_names, profile_id) && 
                *table::borrow(&registry.profile_names, profile_id) == object::uid_to_address(&username.id)) {
                table::remove(&mut registry.profile_names, profile_id);
            };
        };
        
        // Emit purchase event
        event::emit(NamePurchasedEvent {
            name: username.name,
            username_id: object::uid_to_address(&username.id),
            previous_owner,
            new_owner: username.owner,
            price,
            purchased_at: tx_context::epoch_timestamp_ms() / 1000,
        });
    }
    
    // === Admin Functions ===
    
    /// Set price for a name length category (admin only)
    public entry fun set_price_by_length(
        registry: &mut NameRegistry,
        cap: &NameServiceCap,
        length_category: u8,
        price: u64,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(object::owner(cap) == tx_context::sender(ctx), EUnauthorized);
        
        // Update price in registry
        *table::borrow_mut(&mut registry.price_by_length, length_category) = price;
    }
    
    /// Add a premium name and set its price (admin only)
    public entry fun add_premium_name(
        registry: &mut NameRegistry,
        cap: &NameServiceCap,
        name: String,
        price: u64,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(object::owner(cap) == tx_context::sender(ctx), EUnauthorized);
        
        // Add name to premium list
        table::upsert(&mut registry.premium_names, name, price);
    }
    
    /// Remove a premium name (admin only)
    public entry fun remove_premium_name(
        registry: &mut NameRegistry,
        cap: &NameServiceCap,
        name: String,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(object::owner(cap) == tx_context::sender(ctx), EUnauthorized);
        
        // Remove name from premium list if it exists
        if (table::contains(&registry.premium_names, name)) {
            table::remove(&mut registry.premium_names, name);
        };
    }
    
    /// Reserve a name (admin only)
    public entry fun reserve_name(
        registry: &mut NameRegistry,
        cap: &NameServiceCap,
        name: String,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(object::owner(cap) == tx_context::sender(ctx), EUnauthorized);
        
        // Validate the name
        validate_name(&name);
        
        // Add name to reserved list
        table::upsert(&mut registry.reserved_names, name, true);
    }
    
    /// Unreserve a name (admin only)
    public entry fun unreserve_name(
        registry: &mut NameRegistry,
        cap: &NameServiceCap,
        name: String,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(object::owner(cap) == tx_context::sender(ctx), EUnauthorized);
        
        // Remove name from reserved list if it exists
        if (table::contains(&registry.reserved_names, name)) {
            table::remove(&mut registry.reserved_names, name);
        };
    }
    
    /// Withdraw treasury funds (admin only)
    public entry fun withdraw_treasury(
        registry: &mut NameRegistry,
        cap: &NameServiceCap,
        ctx: &mut TxContext
    ) {
        // Verify admin
        assert!(object::owner(cap) == tx_context::sender(ctx), EUnauthorized);
        
        // Get treasury balance
        let amount = balance::value(&registry.treasury);
        
        // Only withdraw if there's a balance
        if (amount > 0) {
            let coin = coin::from_balance(balance::split(&mut registry.treasury, amount), ctx);
            transfer::public_transfer(coin, registry.admin);
        };
    }
    
    // === Helper Functions ===
    
    /// Get the price for registering a name
    fun get_name_price(registry: &NameRegistry, name: &String, duration_years: u64): u64 {
        // Verify duration is valid
        assert!(duration_years > 0, EInvalidDuration);
        
        // Check if name is in premium list
        if (table::contains(&registry.premium_names, *name)) {
            return *table::borrow(&registry.premium_names, *name) * duration_years
        };
        
        // Determine length category and get base price
        let length = string::length(name);
        let category = if (length >= 2 && length <= 4) {
            NAME_LENGTH_ULTRA_SHORT
        } else if (length >= 5 && length <= 7) {
            NAME_LENGTH_SHORT
        } else if (length >= 8 && length <= 12) {
            NAME_LENGTH_MEDIUM
        } else {
            NAME_LENGTH_LONG
        };
        
        // Calculate price based on length category and duration
        *table::borrow(&registry.price_by_length, category) * duration_years
    }
    
    /// Validate a name
    fun validate_name(name: &String) {
        let length = string::length(name);
        
        // Check length (2-32 characters)
        assert!(length >= 2 && length <= 32, EInvalidName);
        
        // Validate characters (in a real implementation, would check for valid characters)
        // For simplicity here, we'll just check that length is valid
        assert!(length >= 2, EInvalidName);
    }
    
    // === Getters ===
    
    /// Check if a name is registered
    public fun is_registered(registry: &NameRegistry, name: &String): bool {
        table::contains(&registry.names, *name)
    }
    
    /// Check if a name is reserved
    public fun is_reserved(registry: &NameRegistry, name: &String): bool {
        table::contains(&registry.reserved_names, *name)
    }
    
    /// Check if a name is premium
    public fun is_premium(registry: &NameRegistry, name: &String): bool {
        table::contains(&registry.premium_names, *name)
    }
    
    /// Get the price for a name
    public fun get_price(registry: &NameRegistry, name: &String, duration_years: u64): u64 {
        get_name_price(registry, name, duration_years)
    }
    
    /// Get the ID of a username
    public fun id(username: &Username): &UID {
        &username.id
    }
    
    /// Get the profile ID for a username
    public fun get_profile_id(username: &Username): &Option<address> {
        &username.profile_id
    }
    
    /// Get the username for a profile
    public fun get_username_for_profile(registry: &NameRegistry, profile_id: address): Option<address> {
        if (table::contains(&registry.profile_names, profile_id)) {
            option::some(*table::borrow(&registry.profile_names, profile_id))
        } else {
            option::none()
        }
    }
    
    /// Get the owner of a username
    public fun owner(username: &Username): address {
        username.owner
    }
    
    /// Get the name string
    public fun name(username: &Username): String {
        username.name
    }
    
    /// Check if a username is expired
    public fun is_expired(username: &Username, clock: &Clock): bool {
        let now = clock::timestamp_ms(clock) / 1000; // Convert ms to seconds
        username.expires_at < now
    }
    
    /// Get the expiration time
    public fun expires_at(username: &Username): u64 {
        username.expires_at
    }
    
    /// Check if username is for sale
    public fun is_for_sale(username: &Username): bool {
        username.sale_price > 0
    }
    
    /// Get sale price
    public fun sale_price(username: &Username): u64 {
        username.sale_price
    }
}