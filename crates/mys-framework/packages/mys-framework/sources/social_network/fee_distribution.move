// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Universal fee distribution module for Mys Framework. 
/// This module provides a standardized way to handle fee collection, distribution, and management
/// across different features like social network, token trading, AI data monetization, etc.
module mys::fee_distribution {
    use std::string::{Self, String};
    use std::vector;
    use mys::object::{Self, UID, ID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::table::{Self, Table};
    use mys::balance::{Self, Balance};
    use mys::coin::{Self, Coin};
    use mys::mys::MYS;
    use mys::type_name;
    use mys::dynamic_object_field;

    // === Errors ===
    /// Operation can only be performed by the authorized entity
    const ENotAuthorized: u64 = 0;
    /// Invalid fee configuration
    const EInvalidFeeConfig: u64 = 1;
    /// Invalid fee split (e.g., shares don't add up to 100%)
    const EInvalidFeeSplit: u64 = 2;
    /// Fee model not found
    const EFeeModelNotFound: u64 = 3;
    /// Withdrawal exceeds available balance
    const EWithdrawalExceedsBalance: u64 = 4;
    /// Recipient not registered
    const ERecipientNotRegistered: u64 = 5;

    // === Fee Model Types ===
    /// Percentage-based fee model (uses basis points)
    const FEE_MODEL_PERCENTAGE: u8 = 0;
    /// Fixed fee model (exact amount)
    const FEE_MODEL_FIXED: u8 = 1;
    /// Tiered fee model (levels with different fees)
    const FEE_MODEL_TIERED: u8 = 2;

    // === Max Constants ===
    /// Maximum fee percentage in basis points (50%)
    const MAX_FEE_BPS: u64 = 5000;

    // === Structs ===

    /// Admin capability for the fee distribution system
    /// This capability is used for creating new fee models and
    /// managing the fee registry
    struct AdminCap has key {
        id: UID,
    }

    /// Registry for all fee configurations
    /// This is the central repository for all fee models and recipient
    /// information across the system
    struct FeeRegistry has key {
        id: UID,
        /// Map of fee model ID to fee model
        fee_models: Table<ID, FeeModel>,
        /// Map of context name to fee model ID (for lookup by name)
        context_models: Table<String, ID>,
        /// Map of recipient ID to treasury ID
        recipient_treasuries: Table<address, ID>,
    }

    /// Fee model configuration
    /// Defines how fees are calculated and distributed for a specific context
    struct FeeModel has key, store {
        id: UID,
        /// Name of the fee model (e.g., "SocialToken", "AIDataUsage")
        name: String,
        /// Description of what this fee applies to
        description: String,
        /// Type of fee model (percentage, fixed, tiered)
        model_type: u8,
        /// Fee amount (interpretation depends on model_type)
        /// For percentage: basis points (1/100 of a percent)
        /// For fixed: absolute amount in smallest units
        fee_amount: u64,
        /// Tiered fee amounts if using tiered model
        tier_amounts: vector<u64>,
        /// Tier thresholds if using tiered model
        tier_thresholds: vector<u64>,
        /// Fee split configuration
        splits: vector<FeeSplit>,
        /// Total split percentage (must sum to 10000 basis points)
        total_split_bps: u64,
        /// Owner who can modify this fee model
        owner: address,
    }

    /// Fee split configuration for a recipient
    /// Defines how fees are distributed among different recipients
    struct FeeSplit has store, copy, drop {
        /// Recipient identifier
        recipient: address,
        /// Recipient name for reference
        recipient_name: String,
        /// Share percentage in basis points (1/100 of a percent)
        share_bps: u64,
    }

    /// Treasury for a fee recipient
    /// Holds collected fees for a specific recipient
    struct Treasury<phantom T> has key {
        id: UID,
        /// Recipient address
        recipient: address,
        /// Name of the recipient
        name: String,
        /// Balance of collected fees
        balance: Balance<T>,
        /// Total fees collected over all time
        total_collected: u64,
    }

    // === Events ===

    /// Event emitted when a new fee model is created
    struct FeeModelCreatedEvent has copy, drop {
        fee_model_id: ID,
        name: String,
        model_type: u8,
        fee_amount: u64,
        total_split_bps: u64,
        owner: address,
    }

    /// Event emitted when fees are collected and distributed
    struct FeesDistributedEvent has copy, drop {
        fee_model_id: ID,
        model_name: String,
        transaction_amount: u64,
        total_fee_amount: u64,
        token_type: String,
        timestamp: u64,
    }

    /// Event emitted when a recipient withdraws fees
    struct FeeWithdrawalEvent has copy, drop {
        recipient: address,
        token_type: String,
        amount: u64,
        timestamp: u64,
    }

    /// Event emitted when a fee model is updated
    struct FeeModelUpdatedEvent has copy, drop {
        fee_model_id: ID,
        name: String,
        fee_amount: u64,
        total_split_bps: u64,
        timestamp: u64,
    }

    // === Initialization ===

    /// Initialize the fee distribution system
    fun init(ctx: &mut TxContext) {
        // Create and transfer admin capability
        transfer::transfer(
            AdminCap {
                id: object::new(ctx),
            },
            tx_context::sender(ctx)
        );
        
        // Create and share fee registry
        transfer::share_object(
            FeeRegistry {
                id: object::new(ctx),
                fee_models: table::new(ctx),
                context_models: table::new(ctx),
                recipient_treasuries: table::new(ctx),
            }
        );
    }

    // === Admin Functions ===

    /// Create a new percentage-based fee model
    public entry fun create_percentage_fee_model(
        _admin_cap: &AdminCap,
        registry: &mut FeeRegistry,
        name: String,
        description: String,
        fee_bps: u64,
        recipient_addresses: vector<address>,
        recipient_names: vector<String>,
        recipient_shares: vector<u64>,
        owner: address,
        ctx: &mut TxContext
    ) {
        // Validate fee percentage
        assert!(fee_bps <= MAX_FEE_BPS, EInvalidFeeConfig);
        
        // Create and register the fee model
        create_fee_model(
            registry,
            name,
            description,
            FEE_MODEL_PERCENTAGE,
            fee_bps,
            vector::empty<u64>(), // No tier amounts for percentage model
            vector::empty<u64>(), // No tier thresholds for percentage model
            recipient_addresses,
            recipient_names,
            recipient_shares,
            owner,
            ctx
        );
    }

    /// Create a new fixed fee model
    public entry fun create_fixed_fee_model(
        _admin_cap: &AdminCap,
        registry: &mut FeeRegistry,
        name: String,
        description: String,
        fixed_amount: u64,
        recipient_addresses: vector<address>,
        recipient_names: vector<String>,
        recipient_shares: vector<u64>,
        owner: address,
        ctx: &mut TxContext
    ) {
        // Create and register the fee model
        create_fee_model(
            registry,
            name,
            description,
            FEE_MODEL_FIXED,
            fixed_amount,
            vector::empty<u64>(), // No tier amounts for fixed model
            vector::empty<u64>(), // No tier thresholds for fixed model
            recipient_addresses,
            recipient_names,
            recipient_shares,
            owner,
            ctx
        );
    }

    /// Create a new tiered fee model
    public entry fun create_tiered_fee_model(
        _admin_cap: &AdminCap,
        registry: &mut FeeRegistry,
        name: String,
        description: String,
        tier_amounts: vector<u64>,
        tier_thresholds: vector<u64>,
        recipient_addresses: vector<address>,
        recipient_names: vector<String>,
        recipient_shares: vector<u64>,
        owner: address,
        ctx: &mut TxContext
    ) {
        // Validate tier configuration
        assert!(vector::length(&tier_amounts) > 0, EInvalidFeeConfig);
        assert!(vector::length(&tier_amounts) == vector::length(&tier_thresholds), EInvalidFeeConfig);
        
        // Create and register the fee model
        create_fee_model(
            registry,
            name,
            description,
            FEE_MODEL_TIERED,
            0, // Base fee amount not used for tiered model
            tier_amounts,
            tier_thresholds,
            recipient_addresses,
            recipient_names,
            recipient_shares,
            owner,
            ctx
        );
    }

    // === User Functions ===

    /// Update fee model parameters (owner only)
    public entry fun update_fee_model(
        registry: &mut FeeRegistry,
        fee_model_id: ID,
        fee_amount: u64,
        recipient_addresses: vector<address>,
        recipient_shares: vector<u64>,
        ctx: &mut TxContext
    ) {
        // Get fee model
        assert!(table::contains(&registry.fee_models, fee_model_id), EFeeModelNotFound);
        let fee_model = table::borrow_mut(&mut registry.fee_models, fee_model_id);
        
        // Verify caller is the fee model owner
        assert!(fee_model.owner == tx_context::sender(ctx), ENotAuthorized);
        
        // Validate fee amount
        if (fee_model.model_type == FEE_MODEL_PERCENTAGE) {
            assert!(fee_amount <= MAX_FEE_BPS, EInvalidFeeConfig);
        };
        
        // Update fee amount
        fee_model.fee_amount = fee_amount;
        
        // Update fee splits if provided
        if (vector::length(&recipient_addresses) > 0) {
            // Validate input vectors
            assert!(vector::length(&recipient_addresses) == vector::length(&recipient_shares), EInvalidFeeConfig);
            
            // Calculate total split
            let total_split = 0u64;
            let i = 0;
            let len = vector::length(&recipient_shares);
            while (i < len) {
                total_split = total_split + *vector::borrow(&recipient_shares, i);
                i = i + 1;
            };
            
            // Validate total split is 100%
            assert!(total_split == 10000, EInvalidFeeSplit);
            
            // Clear existing splits
            fee_model.splits = vector::empty();
            fee_model.total_split_bps = 0;
            
            // Add new splits
            i = 0;
            while (i < len) {
                let recipient = *vector::borrow(&recipient_addresses, i);
                let share = *vector::borrow(&recipient_shares, i);
                
                // Add recipient treasury if it doesn't exist
                ensure_recipient_registered<MYS>(registry, recipient, string::utf8(b"Recipient"), ctx);
                
                // Add split
                let split = FeeSplit {
                    recipient,
                    recipient_name: string::utf8(b"Recipient"), // Basic placeholder
                    share_bps: share,
                };
                vector::push_back(&mut fee_model.splits, split);
                
                fee_model.total_split_bps = fee_model.total_split_bps + share;
                i = i + 1;
            };
        };
        
        // Emit update event
        event::emit(FeeModelUpdatedEvent {
            fee_model_id,
            name: fee_model.name,
            fee_amount,
            total_split_bps: fee_model.total_split_bps,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }

    /// Collect and distribute fees for a transaction
    public fun collect_and_distribute_fees<T>(
        registry: &mut FeeRegistry,
        fee_model_id: ID,
        amount: u64,
        payment: &mut Coin<T>,
        ctx: &mut TxContext
    ): u64 {
        // Get fee model
        assert!(table::contains(&registry.fee_models, fee_model_id), EFeeModelNotFound);
        let fee_model = table::borrow(&registry.fee_models, fee_model_id);
        
        // Calculate fee amount
        let fee_amount = calculate_fee_amount(fee_model, amount);
        
        // Check if payment is sufficient
        assert!(coin::value(payment) >= fee_amount, EWithdrawalExceedsBalance);
        
        // If fee is zero, return early
        if (fee_amount == 0) return 0;
        
        // Extract fee from payment
        let fee_coin = coin::split(payment, fee_amount, ctx);
        let fee_balance = coin::into_balance(fee_coin);
        
        // Distribute fee to recipients
        distribute_fee_balance<T>(registry, fee_model, fee_balance, ctx);
        
        // Emit fee distribution event
        event::emit(FeesDistributedEvent {
            fee_model_id,
            model_name: fee_model.name,
            transaction_amount: amount,
            total_fee_amount: fee_amount,
            token_type: get_type_name<T>(),
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
        
        fee_amount
    }

    /// Withdraw fees for a recipient
    public entry fun withdraw_fees<T>(
        registry: &mut FeeRegistry,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Get caller
        let recipient = tx_context::sender(ctx);
        
        // Check if recipient has a treasury
        assert!(table::contains(&registry.recipient_treasuries, recipient), ERecipientNotRegistered);
        
        // Get treasury ID
        let treasury_id = *table::borrow(&registry.recipient_treasuries, recipient);
        
        // Get treasury object
        let treasury: &mut Treasury<T> = borrow_treasury<T>(treasury_id);
        
        // Verify sufficient balance
        let available = balance::value(&treasury.balance);
        assert!(available >= amount, EWithdrawalExceedsBalance);
        
        // Create withdrawal coin
        let withdrawal_balance = balance::split(&mut treasury.balance, amount);
        let withdrawal_coin = coin::from_balance(withdrawal_balance, ctx);
        
        // Transfer to recipient
        transfer::public_transfer(withdrawal_coin, recipient);
        
        // Emit withdrawal event
        event::emit(FeeWithdrawalEvent {
            recipient,
            token_type: get_type_name<T>(),
            amount,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }

    // === Helper Functions ===

    /// Create a new fee model
    fun create_fee_model(
        registry: &mut FeeRegistry,
        name: String,
        description: String,
        model_type: u8,
        fee_amount: u64,
        tier_amounts: vector<u64>,
        tier_thresholds: vector<u64>,
        recipient_addresses: vector<address>,
        recipient_names: vector<String>,
        recipient_shares: vector<u64>,
        owner: address,
        ctx: &mut TxContext
    ) {
        // Validate input vectors
        assert!(vector::length(&recipient_addresses) == vector::length(&recipient_shares), EInvalidFeeConfig);
        assert!(vector::length(&recipient_addresses) == vector::length(&recipient_names), EInvalidFeeConfig);
        
        // Calculate total split
        let total_split = 0u64;
        let i = 0;
        let len = vector::length(&recipient_shares);
        while (i < len) {
            total_split = total_split + *vector::borrow(&recipient_shares, i);
            i = i + 1;
        };
        
        // Validate total split is 100%
        assert!(total_split == 10000, EInvalidFeeSplit);
        
        // Create fee splits
        let splits = vector::empty<FeeSplit>();
        i = 0;
        while (i < len) {
            let recipient = *vector::borrow(&recipient_addresses, i);
            let recipient_name = *vector::borrow(&recipient_names, i);
            let share = *vector::borrow(&recipient_shares, i);
            
            // Register recipient treasury if needed
            ensure_recipient_registered<MYS>(registry, recipient, recipient_name, ctx);
            
            // Create split
            let split = FeeSplit {
                recipient,
                recipient_name,
                share_bps: share,
            };
            vector::push_back(&mut splits, split);
            
            i = i + 1;
        };
        
        // Create the fee model
        let fee_model = FeeModel {
            id: object::new(ctx),
            name,
            description,
            model_type,
            fee_amount,
            tier_amounts,
            tier_thresholds,
            splits,
            total_split_bps: total_split,
            owner,
        };
        
        let fee_model_id = object::id(&fee_model);
        
        // Add to registry
        table::add(&mut registry.fee_models, fee_model_id, fee_model);
        
        // Add context name mapping
        table::add(&mut registry.context_models, name, fee_model_id);
        
        // Emit creation event
        event::emit(FeeModelCreatedEvent {
            fee_model_id,
            name,
            model_type,
            fee_amount,
            total_split_bps: total_split,
            owner,
        });
    }

    /// Calculate fee amount based on fee model and transaction amount
    fun calculate_fee_amount(fee_model: &FeeModel, amount: u64): u64 {
        if (fee_model.model_type == FEE_MODEL_PERCENTAGE) {
            // Percentage fee (basis points)
            return (amount * fee_model.fee_amount) / 10000
        } else if (fee_model.model_type == FEE_MODEL_FIXED) {
            // Fixed fee amount
            return fee_model.fee_amount
        } else if (fee_model.model_type == FEE_MODEL_TIERED) {
            // Tiered fee based on amount
            let i = 0;
            let len = vector::length(&fee_model.tier_thresholds);
            
            while (i < len) {
                let threshold = *vector::borrow(&fee_model.tier_thresholds, i);
                if (amount <= threshold) {
                    return *vector::borrow(&fee_model.tier_amounts, i)
                };
                i = i + 1;
            };
            
            // If amount exceeds all thresholds, use the last tier
            return *vector::borrow(&fee_model.tier_amounts, len - 1)
        };
        
        0 // Default case
    }

    /// Distribute fee balance to recipients
    fun distribute_fee_balance<T>(
        registry: &mut FeeRegistry,
        fee_model: &FeeModel,
        mut fee_balance: Balance<T>,
        ctx: &mut TxContext
    ) {
        let total_fee = balance::value(&fee_balance);
        let i = 0;
        let len = vector::length(&fee_model.splits);
        
        while (i < len && total_fee > 0) {
            let split = vector::borrow(&fee_model.splits, i);
            
            // Calculate recipient's share
            let share_amount = (total_fee * split.share_bps) / fee_model.total_split_bps;
            
            if (share_amount > 0) {
                // Get recipient treasury ID
                let treasury_id = *table::borrow(&registry.recipient_treasuries, split.recipient);
                
                // Get treasury object
                let treasury: &mut Treasury<T> = borrow_treasury<T>(treasury_id);
                
                // Add fee share to recipient balance
                balance::join(&mut treasury.balance, balance::split(&mut fee_balance, share_amount));
                
                // Update total collected
                treasury.total_collected = treasury.total_collected + share_amount;
            };
            
            i = i + 1;
        };
        
        // Handle any remaining dust (due to division rounding)
        if (balance::value(&fee_balance) > 0) {
            // Add remaining to first recipient
            if (len > 0) {
                let first_split = vector::borrow(&fee_model.splits, 0);
                let treasury_id = *table::borrow(&registry.recipient_treasuries, first_split.recipient);
                let treasury: &mut Treasury<T> = borrow_treasury<T>(treasury_id);
                
                // Update total collected with the dust amount
                treasury.total_collected = treasury.total_collected + balance::value(&fee_balance);
                
                // Add dust to recipient balance
                balance::join(&mut treasury.balance, fee_balance);
            } else {
                // Should never happen if splits are properly configured
                balance::destroy_zero(fee_balance);
            };
        } else {
            balance::destroy_zero(fee_balance);
        };
    }

    /// Ensure a recipient has a treasury registered
    fun ensure_recipient_registered<T>(
        registry: &mut FeeRegistry,
        recipient: address, 
        name: String,
        ctx: &mut TxContext
    ) {
        if (!table::contains(&registry.recipient_treasuries, recipient)) {
            // Create recipient treasury
            let treasury = Treasury<T> {
                id: object::new(ctx),
                recipient,
                name,
                balance: balance::zero(),
                total_collected: 0,
            };
            
            let treasury_id = object::id(&treasury);
            
            // Add to registry
            table::add(&mut registry.recipient_treasuries, recipient, treasury_id);
            
            // Share treasury object
            transfer::share_object(treasury);
        };
    }

    /// Borrow a treasury object by ID
    fun borrow_treasury<T>(treasury_id: ID): &mut Treasury<T> {
        // In a real implementation, we would use dynamic object field to access treasury
        // Here's how it would be properly implemented
        let treasury_obj = object::borrow_mut(treasury_id);
        dynamic_object_field::borrow_mut<String, Treasury<T>>(
            treasury_obj,
            string::utf8(b"treasury")
        )
    }
    
    /// Get token type name as a string
    fun get_type_name<T>(): String {
        // Use Sui's type name capability to get a string representation
        // of the token type
        let type_name = type_name::get<T>();
        let (_, name) = type_name::get_address_and_module_name(type_name);
        name
    }

    // === Public Accessors ===

    /// Find fee model by context name
    public fun find_fee_model_by_name(
        registry: &FeeRegistry,
        name: String
    ): (bool, ID) {
        if (table::contains(&registry.context_models, name)) {
            (true, *table::borrow(&registry.context_models, name))
        } else {
            (false, object::id_from_address(@0x0))
        }
    }

    /// Get fee model information
    public fun get_fee_model_info(
        registry: &FeeRegistry,
        fee_model_id: ID
    ): (bool, String, u8, u64, u64) {
        if (!table::contains(&registry.fee_models, fee_model_id)) {
            return (false, string::utf8(b""), 0, 0, 0)
        };
        
        let fee_model = table::borrow(&registry.fee_models, fee_model_id);
        (
            true,
            fee_model.name,
            fee_model.model_type,
            fee_model.fee_amount,
            fee_model.total_split_bps
        )
    }

    /// Calculate expected fee for a given transaction
    public fun calculate_expected_fee(
        registry: &FeeRegistry,
        fee_model_id: ID,
        amount: u64
    ): u64 {
        if (!table::contains(&registry.fee_models, fee_model_id)) {
            return 0
        };
        
        let fee_model = table::borrow(&registry.fee_models, fee_model_id);
        calculate_fee_amount(fee_model, amount)
    }

    /// Get recipient balance
    public fun get_recipient_balance<T>(
        registry: &FeeRegistry,
        recipient: address
    ): (bool, u64, u64) {
        if (!table::contains(&registry.recipient_treasuries, recipient)) {
            return (false, 0, 0)
        };
        
        let treasury_id = *table::borrow(&registry.recipient_treasuries, recipient);
        let treasury: &Treasury<T> = borrow_treasury<T>(treasury_id);
        
        (
            true,
            balance::value(&treasury.balance),
            treasury.total_collected
        )
    }

    /// Get fee splits for a model
    public fun get_fee_splits(
        registry: &FeeRegistry,
        fee_model_id: ID
    ): vector<FeeSplit> {
        if (!table::contains(&registry.fee_models, fee_model_id)) {
            return vector::empty<FeeSplit>()
        };
        
        let fee_model = table::borrow(&registry.fee_models, fee_model_id);
        fee_model.splits
    }
    
    // === Integration Examples ===
    // This section demonstrates how other modules would integrate with this universal fee distribution module
    
    /*
    Example: Using this module in ai_data_monetization
    
    // Instead of maintaining separate fee logic:
    public entry fun pay_for_data_usage(
        registry: &mut fee_distribution::FeeRegistry,
        fee_model_id: ID,  // Retrieved from registry by name "AIDataUsage"
        agent_cap: &AgentCap,
        platform_id: ID,
        profile_id: ID,
        usage_type: u8,
        payment: &mut Coin<MYS>,
        duration_hours: u64,
        ctx: &mut TxContext
    ) {
        // Verify profile has opted in for monetization
        // ... existing verification logic ...
        
        // Use fee_distribution module to calculate and distribute fees
        let transaction_amount = determine_payment_amount_for_usage(profile_settings);
        
        // Collect and distribute fees (returns the fee amount that was collected)
        let fee_amount = fee_distribution::collect_and_distribute_fees<MYS>(
            registry,
            fee_model_id,
            transaction_amount,
            payment,
            ctx
        );
        
        // Create authorization token and handle remaining business logic
        // ... rest of the function ...
    }
    
    Example: Using this module in token_orderbook_integration
    
    // Process fees for a token transaction
    fun process_token_fees<T>(
        registry: &mut fee_distribution::FeeRegistry,
        coin: &mut Coin<T>,
        ctx: &mut TxContext
    ) {
        // Get token-specific fee model
        let token_type_str = type_name::get_address_and_module_name(type_name::get<T>()).1;
        let (exists, fee_model_id) = fee_distribution::find_fee_model_by_name(registry, token_type_str);
        
        if (exists) {
            let amount = coin::value(coin);
            fee_distribution::collect_and_distribute_fees<T>(
                registry,
                fee_model_id,
                amount,
                coin,
                ctx
            );
        };
    }
    */

    // === System Initialization ===
    
    /// Initialize all standard fee models for the ecosystem
    /// This function should be called during system initialization by an admin
    public entry fun initialize_all_fee_models(
        admin_cap: &AdminCap,
        registry: &mut FeeRegistry,
        ctx: &mut TxContext
    ) {
        // AI Data Usage Fee Models
        
        // Recipient addresses for AI data monetization fee models
        let ai_recipient_addresses = vector[
            // User representative address (placeholder - in real usage this will be dynamic)
            @0x0,
            // Platform representative address
            tx_context::sender(ctx),
            // MySocial treasury address
            tx_context::sender(ctx)
        ];
        
        // Recipient names for AI data monetization
        let ai_recipient_names = vector[
            string::utf8(b"User"),
            string::utf8(b"Platform"),
            string::utf8(b"MySocial Treasury")
        ];
        
        // Recipient shares for AI data monetization (in basis points)
        let ai_recipient_shares = vector[
            5000, // 50% to user
            3000, // 30% to platform
            2000  // 20% to MySocial treasury
        ];
        
        // Create fee model for basic AI data usage
        create_fixed_fee_model(
            admin_cap,
            registry,
            string::utf8(b"AI_Data_Basic"),
            string::utf8(b"Fee for basic AI data usage"),
            10 * 100000000, // 10 tokens fixed fee in smallest units
            ai_recipient_addresses,
            ai_recipient_names,
            ai_recipient_shares,
            tx_context::sender(ctx), // Owner (admin)
            ctx
        );
        
        // Create fee model for standard AI data usage
        create_fixed_fee_model(
            admin_cap,
            registry,
            string::utf8(b"AI_Data_Standard"),
            string::utf8(b"Fee for standard AI data usage"),
            50 * 100000000, // 50 tokens fixed fee in smallest units
            ai_recipient_addresses,
            ai_recipient_names,
            ai_recipient_shares,
            tx_context::sender(ctx), // Owner (admin)
            ctx
        );
        
        // Create fee model for premium AI data usage
        create_fixed_fee_model(
            admin_cap,
            registry,
            string::utf8(b"AI_Data_Premium"),
            string::utf8(b"Fee for premium AI data usage"),
            100 * 100000000, // 100 tokens fixed fee in smallest units
            ai_recipient_addresses,
            ai_recipient_names,
            ai_recipient_shares,
            tx_context::sender(ctx), // Owner (admin)
            ctx
        );
        
        // Token Trading Fee Model
        
        // Recipient addresses for token trading fee model
        let token_recipient_addresses = vector[
            // Creator representative (placeholder - in real usage this will be dynamic)
            @0x0,
            // Platform representative
            tx_context::sender(ctx)
        ];
        
        // Recipient names for token trading
        let token_recipient_names = vector[
            string::utf8(b"Token Creator"),
            string::utf8(b"Platform")
        ];
        
        // Recipient shares for token trading (in basis points)
        let token_recipient_shares = vector[
            8000, // 80% to creator
            2000  // 20% to platform
        ];
        
        // Create fee model for token trading
        create_percentage_fee_model(
            admin_cap,
            registry,
            string::utf8(b"Token_Trading_Fee"),
            string::utf8(b"Fee for token trading and swaps"),
            50, // 0.5% fee in basis points
            token_recipient_addresses,
            token_recipient_names,
            token_recipient_shares,
            tx_context::sender(ctx), // Owner (admin)
            ctx
        );
    }
}