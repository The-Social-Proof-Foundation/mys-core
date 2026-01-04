// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Platform module for the MySocial network
/// Manages social media platforms and their timelines

#[allow(duplicate_alias, unused_use)]
module social_contracts::platform {
    use std::{string::{Self, String}, option, vector};
    
    use mys::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        dynamic_field,
        vec_set::{Self, VecSet},
        table::{Self, Table},
        coin::{Self, Coin},
        balance::{Self, Balance},
        mys::MYS,
        event
    };
    
    use social_contracts::profile;
    use social_contracts::governance;
    use social_contracts::upgrade;
    use social_contracts::block_list;
    use social_contracts::social_graph;

    /// Error codes
    const EUnauthorized: u64 = 0;
    const EPlatformAlreadyExists: u64 = 1;
    const EInvalidTokenAmount: u64 = 2;
    const EAlreadyJoined: u64 = 3;
    const ENotJoined: u64 = 4;
    const EWrongVersion: u64 = 5;
    const EInsufficientTreasuryFunds: u64 = 6;
    const EEmptyRecipientsList: u64 = 7;
    const EInvalidBadgeType: u64 = 8;
    const EBadgeNameTooLong: u64 = 9;
    const EBadgeDescriptionTooLong: u64 = 10;
    const EBadgeMediaUrlTooLong: u64 = 11;
    const EInvalidReasoning: u64 = 12;
    const EBadgeIconUrlTooLong: u64 = 13;
    const EInvalidCategory: u64 = 14;
    const ECategoriesSame: u64 = 15;
    const EPlatformApproved: u64 = 16;

    /// Maximum lengths for badge fields
    const MAX_BADGE_NAME_LENGTH: u64 = 100;
    const MAX_BADGE_DESCRIPTION_LENGTH: u64 = 500;
    const MAX_BADGE_MEDIA_URL_LENGTH: u64 = 2048;
    const MAX_BADGE_ICON_URL_LENGTH: u64 = 2048;
    
    /// Maximum length for approval reasoning
    const MAX_REASONING_LENGTH: u64 = 2000; // Max characters for approval reasoning

    /// Platform category constants
    const CATEGORY_SOCIAL_NETWORK: vector<u8> = b"Social Network";
    const CATEGORY_MESSAGING: vector<u8> = b"Messaging";
    const CATEGORY_LONG_FORM_PUBLISHING: vector<u8> = b"Long Form Publishing";
    const CATEGORY_COMMUNITY_FORUM: vector<u8> = b"Community Forum";
    const CATEGORY_VIDEO_STREAMING: vector<u8> = b"Video Streaming";
    const CATEGORY_LIVE_STREAMING: vector<u8> = b"Live Streaming";
    const CATEGORY_AUDIO_STREAMING: vector<u8> = b"Audio Streaming";
    const CATEGORY_DECENTRALIZED_EXCHANGE: vector<u8> = b"Decentralized Exchange";
    const CATEGORY_PREDICTION_MARKET: vector<u8> = b"Prediction Market";
    const CATEGORY_INSURANCE_MARKET: vector<u8> = b"Insurance Market";
    const CATEGORY_AGENTIC_MARKET: vector<u8> = b"Agentic Market";
    const CATEGORY_YIELD_AND_STAKING: vector<u8> = b"Yield and Staking";
    const CATEGORY_REAL_WORLD_ASSET: vector<u8> = b"Real World Asset";
    const CATEGORY_TICKETING_AND_EVENTS: vector<u8> = b"Ticketing and Events";
    const CATEGORY_IP_LICENSING_AND_ROYALTIES: vector<u8> = b"IP Licensing and Royalties";
    const CATEGORY_DIGITAL_ASSET_VAULT: vector<u8> = b"Digital Asset Vault";
    const CATEGORY_REPUTATION: vector<u8> = b"Reputation";
    const CATEGORY_ADVERTISING: vector<u8> = b"Advertising";
    const CATEGORY_DATA_MARKETPLACE: vector<u8> = b"Data Marketplace";
    const CATEGORY_ORACLE_AND_DATA_FEEDS: vector<u8> = b"Oracle and Data Feeds";
    const CATEGORY_ANALYTICS: vector<u8> = b"Analytics";
    const CATEGORY_FILE_STORAGE: vector<u8> = b"File Storage";
    const CATEGORY_PRIVACY: vector<u8> = b"Privacy";
    const CATEGORY_GAMING: vector<u8> = b"Gaming";
    const CATEGORY_DEVELOPER_TOOLS: vector<u8> = b"Developer Tools";
    const CATEGORY_HARDWARE: vector<u8> = b"Hardware";
    const CATEGORY_RESEARCH: vector<u8> = b"Research";

    /// Field names for dynamic fields
    const MODERATORS_FIELD: vector<u8> = b"moderators";
    const JOINED_WALLETS_FIELD: vector<u8> = b"joined_wallets";

    /// Platform status constants
    const STATUS_DEVELOPMENT: u8 = 0;
    const STATUS_ALPHA: u8 = 1;
    const STATUS_BETA: u8 = 2;
    const STATUS_LIVE: u8 = 3;
    const STATUS_MAINTENANCE: u8 = 4;
    const STATUS_SUNSET: u8 = 5;
    const STATUS_SHUTDOWN: u8 = 6;

    /// Platform status enum
    public struct PlatformStatus has copy, drop, store {
        status: u8,
    }

    /// Admin capability for Platform system management
    public struct PlatformAdminCap has key, store {
        id: UID,
    }

    /// Platform object that contains information about a social media platform
    public struct Platform has key {
        id: UID,
        /// Platform name
        name: String,
        /// Platform tagline
        tagline: String,
        /// Platform description
        description: String,
        /// Platform logo URL
        logo: String,
        /// Platform developer address
        developer: address,
        /// Platform terms of service URL
        terms_of_service: String,
        /// Platform privacy policy URL
        privacy_policy: String,
        /// Platform names
        platforms: vector<String>,
        /// Platform URLs
        links: vector<String>,
        /// Primary platform category 
        primary_category: String,
        /// Secondary platform category (optional)
        secondary_category: Option<String>,
        /// Platform status
        status: PlatformStatus,
        /// Platform release date
        release_date: String,
        /// Platform shutdown date (optional)
        shutdown_date: Option<String>,
        /// Creation timestamp
        created_at: u64,
        /// Platform-specific MYS tokens treasury
        treasury: Balance<MYS>,
        /// Whether the platform wants DAO governance
        wants_dao_governance: bool,
        /// DAO governance configuration parameters (all optional)
        delegate_count: Option<u64>,
        delegate_term_epochs: Option<u64>,
        proposal_submission_cost: Option<u64>,
        min_on_chain_age_days: Option<u64>,
        max_votes_per_user: Option<u64>,
        quadratic_base_cost: Option<u64>,
        voting_period_epochs: Option<u64>,
        quorum_votes: Option<u64>,
        /// ID of governance registry if created
        governance_registry_id: Option<ID>,
        /// Version for upgrades
        version: u64,
    }

    /// Platform registry that keeps track of all platforms
    public struct PlatformRegistry has key {
        id: UID,
        /// Table mapping platform names to platform IDs
        platforms_by_name: Table<String, address>,
        /// Table mapping developer addresses to their platforms
        platforms_by_developer: Table<address, vector<address>>,
        /// Table mapping platform IDs to their approval status (admin-controlled)
        platform_approvals: Table<address, bool>,
        /// Version for upgrades
        version: u64,
    }

    /// Platform created event
    public struct PlatformCreatedEvent has copy, drop {
        platform_id: address,
        name: String,
        tagline: String,
        description: String,
        developer: address,
        logo: String,
        terms_of_service: String,
        privacy_policy: String,
        platforms: vector<String>,
        links: vector<String>,
        primary_category: String,
        secondary_category: Option<String>,
        status: PlatformStatus,
        release_date: String,
        wants_dao_governance: bool,
        governance_registry_id: Option<ID>,
        delegate_count: Option<u64>,
        delegate_term_epochs: Option<u64>,
        proposal_submission_cost: Option<u64>,
        min_on_chain_age_days: Option<u64>,
        max_votes_per_user: Option<u64>,
        quadratic_base_cost: Option<u64>,
        voting_period_epochs: Option<u64>,
        quorum_votes: Option<u64>,
    }

    /// Platform updated event
    public struct PlatformUpdatedEvent has copy, drop {
        platform_id: address,
        name: String,
        tagline: String,
        description: String,
        terms_of_service: String,
        privacy_policy: String,
        platforms: vector<String>,
        links: vector<String>,
        primary_category: String,
        secondary_category: Option<String>,
        status: PlatformStatus,
        release_date: String,
        shutdown_date: Option<String>,
        updated_at: u64,
    }


    /// Moderator added event
    public struct ModeratorAddedEvent has copy, drop {
        platform_id: address,
        moderator_address: address,
        added_by: address,
    }

    /// Moderator removed event
    public struct ModeratorRemovedEvent has copy, drop {
        platform_id: address,
        moderator_address: address,
        removed_by: address,
    }

    /// Platform approval status changed event
    public struct PlatformApprovalChangedEvent has copy, drop {
        platform_id: address,
        approved: bool,
        changed_by: address,
        reasoning: Option<String>, // Optional reasoning for approval/disapproval
    }

    /// Platform deleted event
    public struct PlatformDeletedEvent has copy, drop {
        platform_id: address,
        name: String,
        developer: address,
        deleted_by: address,
        timestamp: u64,
        reasoning: Option<String>, // Optional reasoning for deletion
    }

    /// Event emitted when a user joins a platform
    public struct UserJoinedPlatformEvent has copy, drop {
        wallet_address: address,
        platform_id: ID,
        timestamp: u64,
    }

    /// Event emitted when a user leaves a platform
    public struct UserLeftPlatformEvent has copy, drop {
        wallet_address: address,
        platform_id: ID,
        timestamp: u64,
    }

    /// Event emitted when tokens are airdropped from the platform treasury
    public struct TokenAirdropEvent has copy, drop {
        platform_id: address,
        recipient: address,
        amount: u64,
        reason_code: u8,
        executed_by: address,
        timestamp: u64,
    }

    /// Event emitted when tokens are added to platform treasury
    public struct TreasuryFundedEvent has copy, drop {
        platform_id: address,
        amount: u64,
        funded_by: address,
        new_balance: u64,
        timestamp: u64,
    }

    /// Bootstrap initialization function - creates the platform registry
    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        let registry = PlatformRegistry {
            id: object::new(ctx),
            platforms_by_name: table::new(ctx),
            platforms_by_developer: table::new(ctx),
            platform_approvals: table::new(ctx),
            version: upgrade::current_version(),
        };

        transfer::share_object(registry);
    }

    /// Create a new platform and transfer to developer
    public fun create_platform(
        registry: &mut PlatformRegistry,
        name: String,
        tagline: String,
        description: String,
        logo_url: String,
        terms_of_service: String,
        privacy_policy: String,
        platforms: vector<String>,
        links: vector<String>,
        primary_category: String,
        secondary_category: Option<String>,
        status: u8,
        release_date: String,
        wants_dao_governance: bool,
        delegate_count: Option<u64>,
        delegate_term_epochs: Option<u64>,
        proposal_submission_cost: Option<u64>,
        min_on_chain_age_days: Option<u64>,
        max_votes_per_user: Option<u64>,
        quadratic_base_cost: Option<u64>,
        voting_period_epochs: Option<u64>,
        quorum_votes: Option<u64>,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let platform_id = object::new(ctx);
        let developer = tx_context::sender(ctx);
        let now = tx_context::epoch(ctx);

        // Check if platform name is already taken
        assert!(!table::contains(&registry.platforms_by_name, name), EPlatformAlreadyExists);

        // Validate primary category
        assert!(is_valid_category(&primary_category), EInvalidCategory);
        
        // Validate secondary category if provided
        if (option::is_some(&secondary_category)) {
            let secondary = option::borrow(&secondary_category);
            assert!(is_valid_category(secondary), EInvalidCategory);
            // Ensure primary and secondary categories are different
            assert!(primary_category != *secondary, ECategoriesSame);
        };
        
        // Validate status code is one of the defined constants
        assert!(
            status == STATUS_DEVELOPMENT || 
            status == STATUS_ALPHA || 
            status == STATUS_BETA || 
            status == STATUS_LIVE || 
            status == STATUS_MAINTENANCE || 
            status == STATUS_SUNSET || 
            status == STATUS_SHUTDOWN,
            EUnauthorized
        );

        // If DAO governance is not wanted, set all governance parameters to None
        let actual_delegate_count = if (wants_dao_governance) delegate_count else option::none();
        let actual_delegate_term_epochs = if (wants_dao_governance) delegate_term_epochs else option::none();
        let actual_proposal_submission_cost = if (wants_dao_governance) proposal_submission_cost else option::none();
        let actual_min_on_chain_age_days = if (wants_dao_governance) min_on_chain_age_days else option::none();
        let actual_max_votes_per_user = if (wants_dao_governance) max_votes_per_user else option::none();
        let actual_quadratic_base_cost = if (wants_dao_governance) quadratic_base_cost else option::none();
        let actual_voting_period_epochs = if (wants_dao_governance) voting_period_epochs else option::none();
        let actual_quorum_votes = if (wants_dao_governance) quorum_votes else option::none();

        let mut platform = Platform {
            id: platform_id,
            name,
            tagline,
            description,
            logo: logo_url,
            developer,
            terms_of_service,
            privacy_policy,
            platforms,
            links,
            primary_category,
            secondary_category,
            status: new_status(status),
            release_date,
            shutdown_date: option::none(),
            created_at: now,
            treasury: balance::zero(),
            wants_dao_governance,
            delegate_count: actual_delegate_count,
            delegate_term_epochs: actual_delegate_term_epochs,
            proposal_submission_cost: actual_proposal_submission_cost,
            min_on_chain_age_days: actual_min_on_chain_age_days,
            max_votes_per_user: actual_max_votes_per_user,
            quadratic_base_cost: actual_quadratic_base_cost,
            voting_period_epochs: actual_voting_period_epochs,
            quorum_votes: actual_quorum_votes,
            governance_registry_id: option::none(),
            version: upgrade::current_version(),
        };
        
        // Create empty moderators set
        let mut moderators = vec_set::empty<address>();
        
        // Add developer as a moderator
        vec_set::insert(&mut moderators, developer);
        
        // Add moderators as a dynamic field
        dynamic_field::add(&mut platform.id, MODERATORS_FIELD, moderators);
        
        // Register platform in registry
        let platform_id = object::uid_to_address(&platform.id);
        
        // Add to platforms by name
        table::add(&mut registry.platforms_by_name, *&platform.name, platform_id);
        
        // Add to platforms by developer
        if (!table::contains(&registry.platforms_by_developer, developer)) {
            table::add(&mut registry.platforms_by_developer, developer, vector::empty<address>());
        };
        let developer_platforms = table::borrow_mut(&mut registry.platforms_by_developer, developer);
        vector::push_back(developer_platforms, platform_id);
        
        // Add to platform approvals (starts as not approved)
        table::add(&mut registry.platform_approvals, platform_id, false);
        
        // If platform wants DAO governance, create governance registry immediately so it can
        // be reflected in the creation event payload.
        if (platform.wants_dao_governance) {
            // Use default values if options are None
            let delegate_count = if (option::is_some(&platform.delegate_count)) {
                *option::borrow(&platform.delegate_count)
            } else {
                7 // Default value
            };
            
            let delegate_term_epochs = if (option::is_some(&platform.delegate_term_epochs)) {
                *option::borrow(&platform.delegate_term_epochs)
            } else {
                30 // Default value
            };
            
            let proposal_submission_cost = if (option::is_some(&platform.proposal_submission_cost)) {
                *option::borrow(&platform.proposal_submission_cost)
            } else {
                50_000_000 // Default value
            };
            
            let min_on_chain_age_days = if (option::is_some(&platform.min_on_chain_age_days)) {
                *option::borrow(&platform.min_on_chain_age_days)
            } else {
                7 // Default value
            };
            
            let max_votes_per_user = if (option::is_some(&platform.max_votes_per_user)) {
                *option::borrow(&platform.max_votes_per_user)
            } else {
                5 // Default value
            };
            
            let quadratic_base_cost = if (option::is_some(&platform.quadratic_base_cost)) {
                *option::borrow(&platform.quadratic_base_cost)
            } else {
                5_000_000 // Default value
            };
            
            let voting_period_epochs = if (option::is_some(&platform.voting_period_epochs)) {
                *option::borrow(&platform.voting_period_epochs)
            } else {
                3 // Default value
            };
            
            let quorum_votes = if (option::is_some(&platform.quorum_votes)) {
                *option::borrow(&platform.quorum_votes)
            } else {
                15 // Default value
            };
            
            // Create governance registry for this platform
            let registry_id = governance::create_platform_governance(
                delegate_count,
                delegate_term_epochs,
                proposal_submission_cost,
                min_on_chain_age_days,
                max_votes_per_user,
                quadratic_base_cost,
                voting_period_epochs,
                quorum_votes,
                ctx
            );
            
            // Store registry ID in the platform
            platform.governance_registry_id = option::some(registry_id);
        };
        
        // Emit platform created event (after governance registry creation so DAO fields are populated)
        event::emit(PlatformCreatedEvent {
            platform_id,
            name: platform.name,
            tagline: platform.tagline,
            description: platform.description,
            developer,
            logo: platform.logo,
            terms_of_service: platform.terms_of_service,
            privacy_policy: platform.privacy_policy,
            platforms: platform.platforms,
            links: platform.links,
            primary_category: platform.primary_category,
            secondary_category: platform.secondary_category,
            status: platform.status,
            release_date: platform.release_date,
            wants_dao_governance: platform.wants_dao_governance,
            governance_registry_id: platform.governance_registry_id,
            delegate_count: platform.delegate_count,
            delegate_term_epochs: platform.delegate_term_epochs,
            proposal_submission_cost: platform.proposal_submission_cost,
            min_on_chain_age_days: platform.min_on_chain_age_days,
            max_votes_per_user: platform.max_votes_per_user,
            quadratic_base_cost: platform.quadratic_base_cost,
            voting_period_epochs: platform.voting_period_epochs,
            quorum_votes: platform.quorum_votes,
        });
        
        // Share platform as a shared object (publicly accessible)
        transfer::share_object(platform);
    }

    /// Update platform information
    public fun update_platform(
        platform: &mut Platform,
        new_name: String,
        new_tagline: String,
        new_description: String,
        new_logo_url: String,
        new_terms_of_service: String,
        new_privacy_policy: String,
        new_platforms: vector<String>,
        new_links: vector<String>,
        new_primary_category: String,
        new_secondary_category: Option<String>,
        new_status: u8,
        new_release_date: String,
        new_shutdown_date: Option<String>,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        let now = tx_context::epoch(ctx);

        // Verify caller is platform developer
        assert!(platform.developer == tx_context::sender(ctx), EUnauthorized);
        
        // Validate primary category
        assert!(is_valid_category(&new_primary_category), EInvalidCategory);
        
        // Validate secondary category if provided
        if (option::is_some(&new_secondary_category)) {
            let secondary = option::borrow(&new_secondary_category);
            assert!(is_valid_category(secondary), EInvalidCategory);
            // Ensure primary and secondary categories are different
            assert!(new_primary_category != *secondary, ECategoriesSame);
        };
        
        // Update platform information
        platform.name = new_name;
        platform.tagline = new_tagline;
        platform.description = new_description;
        platform.logo = new_logo_url;
        platform.terms_of_service = new_terms_of_service;
        platform.privacy_policy = new_privacy_policy;
        platform.platforms = new_platforms;
        platform.links = new_links;
        platform.primary_category = new_primary_category;
        platform.secondary_category = new_secondary_category;
        platform.status = new_status(new_status);
        platform.release_date = new_release_date;
        platform.shutdown_date = new_shutdown_date;

        // Emit platform updated event
        event::emit(PlatformUpdatedEvent {
            platform_id: object::uid_to_address(&platform.id),
            name: platform.name,
            tagline: platform.tagline,
            description: platform.description,
            terms_of_service: platform.terms_of_service,
            privacy_policy: platform.privacy_policy,
            platforms: platform.platforms,
            links: platform.links,
            primary_category: platform.primary_category,
            secondary_category: platform.secondary_category,
            status: platform.status,
            release_date: platform.release_date,
            shutdown_date: platform.shutdown_date,
            updated_at: now,
        });
    }

    /// Get the version of a platform
    public fun platform_version(platform: &Platform): u64 {
        platform.version
    }

    /// Get a mutable reference to the platform version (only for upgrade module)
    public(package) fun borrow_platform_version_mut(platform: &mut Platform): &mut u64 {
        &mut platform.version
    }

    /// Get the version of the platform registry
    public fun registry_version(registry: &PlatformRegistry): u64 {
        registry.version
    }

    /// Get a mutable reference to the registry version (only for upgrade module)
    public(package) fun borrow_registry_version_mut(registry: &mut PlatformRegistry): &mut u64 {
        &mut registry.version
    }

    /// Add MYS tokens to platform treasury
    public(package) fun add_to_treasury(
        platform: &mut Platform,
        coin: &mut Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Check amount validity
        assert!(amount > 0 && coin::value(coin) >= amount, EInvalidTokenAmount);
        
        // Split coin and add to treasury
        let treasury_coin = coin::split(coin, amount, ctx);
        balance::join(&mut platform.treasury, coin::into_balance(treasury_coin));
        
        // Emit treasury funded event
        let platform_id = object::uid_to_address(&platform.id);
        let new_balance = balance::value(&platform.treasury);
        event::emit(TreasuryFundedEvent {
            platform_id,
            amount,
            funded_by: tx_context::sender(ctx),
            new_balance,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }

    /// Add a moderator to a platform
    public fun add_moderator(
        platform: &mut Platform,
        moderator_address: address,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Verify caller is platform developer
        let caller = tx_context::sender(ctx);
        assert!(platform.developer == caller, EUnauthorized);
        
        // Get moderators set
        let moderators = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, MODERATORS_FIELD);
        
        // Add moderator if not already a moderator
        if (!vec_set::contains(moderators, &moderator_address)) {
            vec_set::insert(moderators, moderator_address);
            
            // Emit moderator added event
            event::emit(ModeratorAddedEvent {
                platform_id: object::uid_to_address(&platform.id),
                moderator_address,
                added_by: caller,
            });
        };
    }

    /// Remove a moderator from a platform
    public fun remove_moderator(
        platform: &mut Platform,
        moderator_address: address,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Verify caller is platform developer
        let caller = tx_context::sender(ctx);
        assert!(platform.developer == caller, EUnauthorized);
        
        // Cannot remove developer as moderator
        assert!(moderator_address != platform.developer, EUnauthorized);
        
        // Get moderators set
        let moderators = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, MODERATORS_FIELD);
        
        // Remove moderator if they are a moderator
        if (vec_set::contains(moderators, &moderator_address)) {
            vec_set::remove(moderators, &moderator_address);
            
            // Emit moderator removed event
            event::emit(ModeratorRemovedEvent {
                platform_id: object::uid_to_address(&platform.id),
                moderator_address,
                removed_by: caller,
            });
        };
    }


    /// Block a wallet address from the platform
    /// Allows platform developers/moderators to block wallets using the platform address as the blocker
    /// This enables platforms (shared objects) to block user wallets
    public fun block_wallet(
        block_list_registry: &mut block_list::BlockListRegistry,
        social_graph: &mut social_graph::SocialGraph,
        username_registry: &profile::UsernameRegistry,
        platform: &mut Platform,
        blocked_wallet_address: address,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Verify caller is platform developer or moderator
        let caller = tx_context::sender(ctx);
        assert!(is_developer_or_moderator(platform, caller), EUnauthorized);
        
        // Get the platform address (this will be the blocker address)
        let platform_address = object::uid_to_address(&platform.id);
        
        // Call block_list's internal helper function with platform address as blocker
        block_list::block_wallet_internal(
            block_list_registry,
            social_graph,
            username_registry,
            platform_address,
            blocked_wallet_address,
            ctx
        );
    }

    /// Unblock a wallet address from the platform
    /// Allows platform developers/moderators to unblock wallets using the platform address as the blocker
    public fun unblock_wallet(
        block_list_registry: &mut block_list::BlockListRegistry,
        platform: &mut Platform,
        blocked_wallet_address: address,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Verify caller is platform developer or moderator
        let caller = tx_context::sender(ctx);
        assert!(is_developer_or_moderator(platform, caller), EUnauthorized);
        
        // Get the platform address (this is the blocker address)
        let platform_address = object::uid_to_address(&platform.id);
        
        // Call block_list's internal helper function with platform address as blocker
        block_list::unblock_wallet_internal(block_list_registry, platform_address, blocked_wallet_address);
    }

    /// Toggle platform approval status (requires PlatformAdminCap only)
    /// Optional reasoning can be provided to explain the decision
    public fun toggle_platform_approval(
        registry: &mut PlatformRegistry,
        platform_id: address,
        _: &PlatformAdminCap,
        reasoning: Option<String>,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        // Admin capability verification is handled by type system
        // Verify the platform exists in the registry
        assert!(table::contains(&registry.platform_approvals, platform_id), EUnauthorized);
        
        // Validate reasoning length if provided
        if (option::is_some(&reasoning)) {
            let reasoning_val = option::borrow(&reasoning);
            assert!(string::length(reasoning_val) <= MAX_REASONING_LENGTH, EInvalidReasoning);
        };
        
        // Get current approval status and toggle it
        let current_approval = *table::borrow(&registry.platform_approvals, platform_id);
        let new_approval = !current_approval;
        
        // Update the approval status in the registry
        *table::borrow_mut(&mut registry.platform_approvals, platform_id) = new_approval;
        
        // Emit approval status changed event with reasoning
        event::emit(PlatformApprovalChangedEvent {
            platform_id,
            approved: new_approval,
            changed_by: tx_context::sender(ctx),
            reasoning,
        });
    }

    /// Delete a platform (requires PlatformAdminCap only)
    /// Can only delete platforms that are NOT approved
    /// Optional reasoning can be provided to explain the deletion
    public fun delete_platform(
        registry: &mut PlatformRegistry,
        platform: &Platform,
        _: &PlatformAdminCap,
        reasoning: Option<String>,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        // Admin capability verification is handled by type system
        let platform_id = object::uid_to_address(&platform.id);
        let platform_name = name(platform);
        let developer = developer(platform);
        
        // Verify the platform exists in the registry
        assert!(table::contains(&registry.platform_approvals, platform_id), EUnauthorized);
        
        // Verify platform is NOT approved (can only delete unapproved platforms)
        let is_approved = *table::borrow(&registry.platform_approvals, platform_id);
        assert!(!is_approved, EPlatformApproved);
        
        // Validate reasoning length if provided
        if (option::is_some(&reasoning)) {
            let reasoning_val = option::borrow(&reasoning);
            assert!(string::length(reasoning_val) <= MAX_REASONING_LENGTH, EInvalidReasoning);
        };
        
        // Remove from platforms_by_name table
        if (table::contains(&registry.platforms_by_name, platform_name)) {
            table::remove(&mut registry.platforms_by_name, platform_name);
        };
        
        // Remove from platforms_by_developer table
        if (table::contains(&registry.platforms_by_developer, developer)) {
            let developer_platforms = table::borrow_mut(&mut registry.platforms_by_developer, developer);
            let mut i = 0;
            let len = vector::length(developer_platforms);
            while (i < len) {
                if (*vector::borrow(developer_platforms, i) == platform_id) {
                    vector::remove(developer_platforms, i);
                    break
                };
                i = i + 1;
            };
            // If developer has no more platforms, remove the entry
            if (vector::length(developer_platforms) == 0) {
                table::remove(&mut registry.platforms_by_developer, developer);
            };
        };
        
        // Remove from platform_approvals table
        table::remove(&mut registry.platform_approvals, platform_id);
        
        // Emit platform deleted event
        event::emit(PlatformDeletedEvent {
            platform_id,
            name: platform_name,
            developer,
            deleted_by: tx_context::sender(ctx),
            timestamp: tx_context::epoch_timestamp_ms(ctx),
            reasoning,
        });
    }

    /// Create a new platform status
    public fun new_status(status: u8): PlatformStatus {
        // Validate the status code is one of the defined constants
        assert!(
            status == STATUS_DEVELOPMENT || 
            status == STATUS_ALPHA || 
            status == STATUS_BETA || 
            status == STATUS_LIVE || 
            status == STATUS_MAINTENANCE || 
            status == STATUS_SUNSET || 
            status == STATUS_SHUTDOWN,
            EUnauthorized
        );
        
        PlatformStatus { status }
    }
    
    /// Get the status value
    public fun status_value(status: &PlatformStatus): u8 {
        status.status
    }

    /// Validate that a category string matches one of the allowed categories
    #[allow(implicit_const_copy)]
    fun is_valid_category(category: &String): bool {
        let category_bytes = string::as_bytes(category);
        category_bytes == CATEGORY_SOCIAL_NETWORK ||
        category_bytes == CATEGORY_MESSAGING ||
        category_bytes == CATEGORY_LONG_FORM_PUBLISHING ||
        category_bytes == CATEGORY_COMMUNITY_FORUM ||
        category_bytes == CATEGORY_VIDEO_STREAMING ||
        category_bytes == CATEGORY_LIVE_STREAMING ||
        category_bytes == CATEGORY_AUDIO_STREAMING ||
        category_bytes == CATEGORY_DECENTRALIZED_EXCHANGE ||
        category_bytes == CATEGORY_PREDICTION_MARKET ||
        category_bytes == CATEGORY_INSURANCE_MARKET ||
        category_bytes == CATEGORY_AGENTIC_MARKET ||
        category_bytes == CATEGORY_YIELD_AND_STAKING ||
        category_bytes == CATEGORY_REAL_WORLD_ASSET ||
        category_bytes == CATEGORY_TICKETING_AND_EVENTS ||
        category_bytes == CATEGORY_IP_LICENSING_AND_ROYALTIES ||
        category_bytes == CATEGORY_DIGITAL_ASSET_VAULT ||
        category_bytes == CATEGORY_REPUTATION ||
        category_bytes == CATEGORY_ADVERTISING ||
        category_bytes == CATEGORY_DATA_MARKETPLACE ||
        category_bytes == CATEGORY_ORACLE_AND_DATA_FEEDS ||
        category_bytes == CATEGORY_ANALYTICS ||
        category_bytes == CATEGORY_FILE_STORAGE ||
        category_bytes == CATEGORY_PRIVACY ||
        category_bytes == CATEGORY_GAMING ||
        category_bytes == CATEGORY_DEVELOPER_TOOLS ||
        category_bytes == CATEGORY_HARDWARE ||
        category_bytes == CATEGORY_RESEARCH
    }

    /// Join a platform - establishes initial connection between wallet and platform
    /// Checks for blocks before allowing the join and verifies platform is approved
    /// Works with wallet addresses only, no profile required
    public fun join_platform(
        platform_registry: &PlatformRegistry,
        block_list_registry: &block_list::BlockListRegistry,
        platform: &mut Platform,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        let platform_id = object::id(platform);
        let current_time = tx_context::epoch_timestamp_ms(ctx);
        
        // Check if the platform has blocked this wallet address
        let platform_address = object::uid_to_address(&platform.id);
        assert!(!block_list::is_blocked(block_list_registry, platform_address, caller), EUnauthorized);
        
        // Check if the platform is approved by the contract owner (use registry)
        let platform_id_addr = object::uid_to_address(&platform.id);
        assert!(is_approved(platform_registry, platform_id_addr), EUnauthorized);
        
        // Create joined wallets set if it doesn't exist
        if (!dynamic_field::exists_(&platform.id, JOINED_WALLETS_FIELD)) {
            let joined_wallets = vec_set::empty<address>();
            dynamic_field::add(&mut platform.id, JOINED_WALLETS_FIELD, joined_wallets);
        };
        
        // Get joined wallets set
        let joined_wallets = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, JOINED_WALLETS_FIELD);
        
        // Check if wallet is already joined to the platform
        assert!(!vec_set::contains(joined_wallets, &caller), EAlreadyJoined);
        
        // Add wallet to joined wallets
        vec_set::insert(joined_wallets, caller);
        
        // Emit event
        event::emit(UserJoinedPlatformEvent {
            wallet_address: caller,
            platform_id,
            timestamp: current_time,
        });
    }

    /// Leave a platform - removes the connection between wallet and platform
    public fun leave_platform(
        platform: &mut Platform,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        let platform_id = object::id(platform);
        let current_time = tx_context::epoch_timestamp_ms(ctx);
        
        // Check if joined wallets set exists
        assert!(dynamic_field::exists_(&platform.id, JOINED_WALLETS_FIELD), ENotJoined);
        
        // Get joined wallets set
        let joined_wallets = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, JOINED_WALLETS_FIELD);
        
        // Check if wallet is a member of the platform
        assert!(vec_set::contains(joined_wallets, &caller), ENotJoined);
        
        // Remove wallet from joined wallets
        vec_set::remove(joined_wallets, &caller);
        
        // Emit event
        event::emit(UserLeftPlatformEvent {
            wallet_address: caller,
            platform_id,
            timestamp: current_time,
        });
    }

    /// Get platform approval status from registry
    public fun is_approved(registry: &PlatformRegistry, platform_id: address): bool {
        if (!table::contains(&registry.platform_approvals, platform_id)) {
            return false
        };
        *table::borrow(&registry.platform_approvals, platform_id)
    }

    /// Check if a wallet address has joined a platform
    public fun has_joined_platform(platform: &Platform, wallet_address: address): bool {
        if (!dynamic_field::exists_(&platform.id, JOINED_WALLETS_FIELD)) {
            return false
        };
        
        let joined_wallets = dynamic_field::borrow<vector<u8>, VecSet<address>>(&platform.id, JOINED_WALLETS_FIELD);
        vec_set::contains(joined_wallets, &wallet_address)
    }

    // === Helper functions ===

    /// Check if an address is the platform developer or a moderator
    public fun is_developer_or_moderator(platform: &Platform, addr: address): bool {
        if (platform.developer == addr) {
            return true
        };
        
        let moderators = dynamic_field::borrow<vector<u8>, VecSet<address>>(&platform.id, MODERATORS_FIELD);
        vec_set::contains(moderators, &addr)
    }

    // === Getters ===

    /// Get platform name
    public fun name(platform: &Platform): String {
        platform.name
    }

    /// Get platform tagline
    public fun tagline(platform: &Platform): String {
        platform.tagline
    }

    /// Get platform description
    public fun description(platform: &Platform): String {
        platform.description
    }

    /// Get platform logo URL
    public fun logo(platform: &Platform): &String {
        &platform.logo
    }

    /// Get platform developer
    public fun developer(platform: &Platform): address {
        platform.developer
    }

    /// Get platform terms of service
    public fun terms_of_service(platform: &Platform): String {
        platform.terms_of_service
    }

    /// Get platform privacy policy
    public fun privacy_policy(platform: &Platform): String {
        platform.privacy_policy
    }

    /// Get platform platforms
    public fun get_platforms(platform: &Platform): &vector<String> {
        &platform.platforms
    }

    /// Get platform links
    public fun get_links(platform: &Platform): &vector<String> {
        &platform.links
    }

    /// Get platform primary category
    public fun primary_category(platform: &Platform): String {
        platform.primary_category
    }

    /// Get platform secondary category
    public fun secondary_category(platform: &Platform): &Option<String> {
        &platform.secondary_category
    }

    /// Get platform status
    public fun status(platform: &Platform): u8 {
        status_value(&platform.status)
    }

    /// Get platform release date
    public fun release_date(platform: &Platform): String {
        platform.release_date
    }

    /// Get platform shutdown date
    public fun shutdown_date(platform: &Platform): &Option<String> {
        &platform.shutdown_date
    }

    /// Get platform creation timestamp
    public fun created_at(platform: &Platform): u64 {
        platform.created_at
    }

    /// Get platform treasury balance
    public fun treasury_balance(platform: &Platform): u64 {
        balance::value(&platform.treasury)
    }

    /// Get platform ID
    public fun id(platform: &Platform): &UID {
        &platform.id
    }

    /// Check if an address is a moderator
    public fun is_moderator(platform: &Platform, addr: address): bool {
        let moderators = dynamic_field::borrow<vector<u8>, VecSet<address>>(&platform.id, MODERATORS_FIELD);
        vec_set::contains(moderators, &addr)
    }

    /// Get the list of moderators for a platform
    public fun get_moderators(platform: &Platform): vector<address> {
        let moderators = dynamic_field::borrow<vector<u8>, VecSet<address>>(&platform.id, MODERATORS_FIELD);
        vec_set::into_keys(*moderators)
    }

    /// Get platform by name from registry
    public fun get_platform_by_name(registry: &PlatformRegistry, name: String): Option<address> {
        if (!table::contains(&registry.platforms_by_name, name)) {
            return option::none()
        };
        
        option::some(*table::borrow(&registry.platforms_by_name, name))
    }

    /// Get platforms owned by a developer
    public fun get_platforms_by_developer(registry: &PlatformRegistry, developer: address): vector<address> {
        if (!table::contains(&registry.platforms_by_developer, developer)) {
            return vector::empty()
        };
        
        *table::borrow(&registry.platforms_by_developer, developer)
    }


    /// Check if platform wants DAO governance
    public fun wants_dao_governance(platform: &Platform): bool {
        platform.wants_dao_governance
    }

    /// Get platform's governance registry ID if available
    public fun governance_registry_id(platform: &Platform): &Option<ID> {
        &platform.governance_registry_id
    }

    /// Get platform's governance parameters
    public fun governance_parameters(platform: &Platform): (Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>) {
        (
            platform.delegate_count,
            platform.delegate_term_epochs,
            platform.proposal_submission_cost,
            platform.min_on_chain_age_days,
            platform.max_votes_per_user,
            platform.quadratic_base_cost,
            platform.voting_period_epochs,
            platform.quorum_votes
        )
    }

    /// Update governance parameters for this platform's governance registry
    /// Can only be called by the platform developer
    public fun update_platform_governance(
        platform: &Platform,
        registry: &mut governance::GovernanceDAO,
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        min_on_chain_age_days: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_epochs: u64,
        quorum_votes: u64,
        ctx: &mut TxContext
    ) {
        // Verify caller is platform developer
        let caller = tx_context::sender(ctx);
        assert!(developer(platform) == caller, EUnauthorized);
        
        // Verify that the platform's governance_registry_id matches this registry
        // This ensures the registry actually belongs to this platform
        let platform_registry_id_opt = governance_registry_id(platform);
        assert!(option::is_some(platform_registry_id_opt), EUnauthorized);
        let platform_registry_id = *option::borrow(platform_registry_id_opt);
        let registry_id = object::id(registry);
        assert!(platform_registry_id == registry_id, EUnauthorized);
        
        // Call governance function with verified platform developer address
        governance::update_platform_governance_parameters(
            registry,
            developer(platform),
            delegate_count,
            delegate_term_epochs,
            proposal_submission_cost,
            min_on_chain_age_days,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_epochs,
            quorum_votes,
            ctx
        );
    }

    /// Airdrop tokens to multiple recipients from the platform treasury
    /// Can only be called by platform developer or moderator
    public fun airdrop_from_treasury(
        platform: &mut Platform,
        recipients: vector<address>,
        amount_per_recipient: u64,
        reason_code: u8,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        
        // Verify caller is platform developer or moderator
        assert!(is_developer_or_moderator(platform, caller), EUnauthorized);
        
        // Check that recipients list is not empty
        let recipients_count = vector::length(&recipients);
        assert!(recipients_count > 0, EEmptyRecipientsList);
        
        // Calculate total amount needed
        let total_amount = amount_per_recipient * recipients_count;
        
        // Verify platform treasury has enough funds
        assert!(balance::value(&platform.treasury) >= total_amount, EInsufficientTreasuryFunds);
        
        // Get current timestamp for events
        let current_time = tx_context::epoch_timestamp_ms(ctx);
        let platform_id = object::uid_to_address(&platform.id);
        
        // Send tokens to each recipient
        let mut i = 0;
        while (i < recipients_count) {
            let recipient = *vector::borrow(&recipients, i);
            
            // Create coin from platform treasury balance
            let airdrop_coin = coin::from_balance(
                balance::split(&mut platform.treasury, amount_per_recipient), 
                ctx
            );
            
            // Transfer to recipient
            transfer::public_transfer(airdrop_coin, recipient);
            
            // Emit airdrop event for tracking
            event::emit(TokenAirdropEvent {
                platform_id,
                recipient,
                amount: amount_per_recipient,
                reason_code,
                executed_by: caller,
                timestamp: current_time,
            });
            
            i = i + 1;
        };
    }

    /// Assign a badge to a profile - can only be called by platform admin/moderator
    /// This is the primary entry point for badge assignment
    public fun assign_badge(
        platform_registry: &PlatformRegistry,
        platform: &Platform,
        profile: &mut profile::Profile,
        badge_name: String,
        badge_description: String,
        badge_media_url: String,
        badge_icon_url: String,
        badge_type: u8,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Verify caller is platform admin or moderator
        let caller = tx_context::sender(ctx);
        assert!(is_developer_or_moderator(platform, caller), EUnauthorized);
        
        // Verify platform is approved
        let platform_id = object::uid_to_address(&platform.id);
        assert!(is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Validate badge type (1-100 as documented)
        assert!(badge_type >= 1 && badge_type <= 100, EInvalidBadgeType);
        
        // Validate badge field lengths
        assert!(string::length(&badge_name) > 0 && string::length(&badge_name) <= MAX_BADGE_NAME_LENGTH, EBadgeNameTooLong);
        assert!(string::length(&badge_description) <= MAX_BADGE_DESCRIPTION_LENGTH, EBadgeDescriptionTooLong);
        assert!(string::length(&badge_media_url) > 0 && string::length(&badge_media_url) <= MAX_BADGE_MEDIA_URL_LENGTH, EBadgeMediaUrlTooLong);
        assert!(string::length(&badge_icon_url) > 0 && string::length(&badge_icon_url) <= MAX_BADGE_ICON_URL_LENGTH, EBadgeIconUrlTooLong);
        
        // Get current time
        let now = tx_context::epoch(ctx);
        
        // Create a unique badge ID by including platform ID to prevent collisions
        let mut badge_id = string::utf8(b"badge_");
        // Convert platform ID to hex string and append to ensure uniqueness
        let platform_id_str = mys::address::to_string(platform_id);
        string::append(&mut badge_id, platform_id_str);
        string::append(&mut badge_id, string::utf8(b"_"));
        string::append(&mut badge_id, badge_name);
        
        // Add the badge directly to the profile
        profile::add_badge_to_profile(
            profile,
            badge_id,
            badge_name,
            badge_description,
            badge_media_url,
            badge_icon_url,
            platform_id,
            now,
            caller,
            badge_type
        );
    }

    /// Revoke a badge from a profile - can only be called by platform admin/moderator
    /// This is the primary entry point for badge revocation
    public fun revoke_badge(
        platform_registry: &PlatformRegistry,
        platform: &Platform,
        profile: &mut profile::Profile,
        badge_id: String,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(platform.version == upgrade::current_version(), EWrongVersion);
        
        // Verify caller is platform admin or moderator
        let caller = tx_context::sender(ctx);
        assert!(is_developer_or_moderator(platform, caller), EUnauthorized);
        
        // Verify platform is approved
        let platform_id = object::uid_to_address(&platform.id);
        assert!(is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Get current time
        let now = tx_context::epoch(ctx);
        
        // Remove the badge directly from the profile
        profile::remove_badge_from_profile(
            profile,
            &badge_id,
            platform_id,
            caller,
            now
        );
    }

    /// When adding a moderator to a platform, register them with the profile module
    public(package) fun add_moderator_register(
        platform: &mut Platform,
        moderator_address: address,
        ctx: &TxContext
    ) {
        // Verify caller is platform developer
        let caller = tx_context::sender(ctx);
        assert!(platform.developer == caller, EUnauthorized);
        
        // Get moderators set
        let moderators = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, MODERATORS_FIELD);
        
        // Add moderator if not already a moderator
        if (!vec_set::contains(moderators, &moderator_address)) {
            vec_set::insert(moderators, moderator_address);
            
            // Emit moderator added event
            let platform_id = object::uid_to_address(&platform.id);
            event::emit(ModeratorAddedEvent {
                platform_id,
                moderator_address,
                added_by: caller,
            });
        };
    }
    
    /// When removing a moderator from a platform
    public(package) fun remove_moderator_unregister(
        platform: &mut Platform,
        moderator_address: address,
        ctx: &TxContext
    ) {
        // Verify caller is platform developer
        let caller = tx_context::sender(ctx);
        assert!(platform.developer == caller, EUnauthorized);
        
        // Cannot remove developer as moderator
        assert!(moderator_address != platform.developer, EUnauthorized);
        
        // Get moderators set
        let moderators = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, MODERATORS_FIELD);
        
        // Remove moderator if they are a moderator
        if (vec_set::contains(moderators, &moderator_address)) {
            vec_set::remove(moderators, &moderator_address);
            
            // Emit moderator removed event
            let platform_id = object::uid_to_address(&platform.id);
            event::emit(ModeratorRemovedEvent {
                platform_id,
                moderator_address,
                removed_by: caller,
            });
        };
    }

    #[test_only]
    /// Initialize test environment for platform module
    public fun test_init(ctx: &mut TxContext) {
        let registry = PlatformRegistry {
            id: object::new(ctx),
            platforms_by_name: table::new(ctx),
            platforms_by_developer: table::new(ctx),
            platform_approvals: table::new(ctx),
            version: 1, // Set to version 1 for testing
        };

        transfer::share_object(registry);
    }
    
    #[test_only]
    /// Test helper to directly set a wallet as joined to a platform
    /// Simplifies testing by bypassing the normal join flow
    public fun test_join_platform(platform: &mut Platform, wallet_address: address) {
        // Create joined wallets set if it doesn't exist
        if (!dynamic_field::exists_(&platform.id, JOINED_WALLETS_FIELD)) {
            let joined_wallets = vec_set::empty<address>();
            dynamic_field::add(&mut platform.id, JOINED_WALLETS_FIELD, joined_wallets);
        };
        
        // Get joined wallets set
        let joined_wallets = dynamic_field::borrow_mut<vector<u8>, VecSet<address>>(&mut platform.id, JOINED_WALLETS_FIELD);
        
        // Add wallet to joined wallets
        if (!vec_set::contains(joined_wallets, &wallet_address)) {
            vec_set::insert(joined_wallets, wallet_address);
        };
    }

    /// Create a PlatformAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_platform_admin_cap(ctx: &mut TxContext): PlatformAdminCap {
        PlatformAdminCap {
            id: object::new(ctx)
        }
    }
    
    #[test_only]
    /// Test helper to set the approval status of a platform in the registry
    public fun test_set_approval(registry: &mut PlatformRegistry, platform_id: address, approved: bool) {
        if (!table::contains(&registry.platform_approvals, platform_id)) {
            table::add(&mut registry.platform_approvals, platform_id, approved);
        } else {
            *table::borrow_mut(&mut registry.platform_approvals, platform_id) = approved;
        };
    }
}