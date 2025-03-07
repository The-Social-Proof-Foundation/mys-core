// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Integration module that connects user tokens with the DeepBook order book system
module mys::token_orderbook_integration {
    use std::ascii;
    use std::string;
    use mys::object::{Self, UID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::coin::{Self, Coin, CoinMetadata};
    use mys::balance::{Self, Balance};
    use mys::clock::{Self, Clock};
    use mys::mys::MYS;

    use mys::user_token::{Self, UserTokenInfo, TokenRegistry, FeeCollector};
    
    use deepbook::custodian_v2::{Self as custodian, AccountCap};
    use deepbook::clob_v2::{Self as clob, Pool, PoolOwnerCap};
    
    // === Errors ===
    /// Operation can only be performed by the platform admin
    const ENotAuthorized: u64 = 0;
    /// Order book already exists for this pair
    const EOrderBookExists: u64 = 1;
    /// Order book not found
    const EOrderBookNotFound: u64 = 2;
    /// Invalid price
    const EInvalidPrice: u64 = 3;
    /// Invalid quantity
    const EInvalidQuantity: u64 = 4;
    /// Insufficient balance
    const EInsufficientBalance: u64 = 5;
    /// Token not registered
    const ETokenNotRegistered: u64 = 6;
    
    // === Constants ===
    // Default tick size
    const DEFAULT_TICK_SIZE: u64 = 1_000;
    // Default lot size
    const DEFAULT_LOT_SIZE: u64 = 100_000;
    
    // === Structs ===
    
    /// Registry that tracks all created order books for user tokens
    struct OrderBookRegistry has key {
        id: UID,
        // Maps token pair (base_token, quote_token) to Pool ID
        token_pools: vector<TokenPairInfo>,
    }
    
    /// Information about a token pair and its associated order book
    struct TokenPairInfo has store, drop, copy {
        base_token: address,
        quote_token: address,
        pool_id: address,
        base_token_creator: address,
        quote_token_creator: address,
    }
    
    // === Events ===
    
    /// Event emitted when a new order book is created for a token pair
    struct OrderBookCreatedEvent has copy, drop {
        pool_id: address,
        base_token: address,
        quote_token: address,
        base_symbol: ascii::String,
        quote_symbol: ascii::String,
        tick_size: u64,
        lot_size: u64,
    }
    
    /// Event emitted when fees are collected from a token trade
    struct TokenTradeFeesCollectedEvent has copy, drop {
        token_id: address,
        token_creator: address,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
    }
    
    // === Initialization ===
    
    /// Initialize the order book integration
    fun init(ctx: &mut TxContext) {
        // Create and share order book registry
        transfer::share_object(
            OrderBookRegistry {
                id: object::new(ctx),
                token_pools: vector::empty(),
            }
        );
    }
    
    // === Admin Functions ===
    
    /// Create a DeepBook pool for a user token pair
    public entry fun create_orderbook_for_token_pair<BaseAsset, QuoteAsset>(
        registry: &mut OrderBookRegistry,
        token_registry: &TokenRegistry,
        base_metadata: &CoinMetadata<BaseAsset>,
        quote_metadata: &CoinMetadata<QuoteAsset>,
        tick_size: u64,
        lot_size: u64,
        creation_fee: Coin<MYS>,
        ctx: &mut TxContext
    ) {
        // Get token addresses
        let base_token = tx_context::type_into_address<BaseAsset>();
        let quote_token = tx_context::type_into_address<QuoteAsset>();
        
        // Ensure order book doesn't already exist
        let i = 0;
        let len = vector::length(&registry.token_pools);
        while (i < len) {
            let pair_info = vector::borrow(&registry.token_pools, i);
            if (pair_info.base_token == base_token && pair_info.quote_token == quote_token) {
                assert!(false, EOrderBookExists);
            };
            i = i + 1;
        };
        
        // Create the order book in DeepBook
        let pool = clob::create_pool_with_return<BaseAsset, QuoteAsset>(
            tick_size,
            lot_size,
            creation_fee,
            ctx
        );
        
        // Extract token creators from token registry if they're user tokens
        let mut base_token_creator = @0x0;
        let mut quote_token_creator = @0x0;
        
        // Check if base token is a user token
        if (is_user_token(token_registry, base_token)) {
            let token_info = user_token::find_token_info(token_registry, base_token);
            base_token_creator = token_info.user;
        };
        
        // Check if quote token is a user token
        if (is_user_token(token_registry, quote_token)) {
            let token_info = user_token::find_token_info(token_registry, quote_token);
            quote_token_creator = token_info.user;
        };
        
        // Store pool information in registry
        let pool_id = object::uid_to_address(&pool.id);
        let pair_info = TokenPairInfo {
            base_token,
            quote_token,
            pool_id,
            base_token_creator,
            quote_token_creator,
        };
        vector::push_back(&mut registry.token_pools, pair_info);
        
        // Emit event
        event::emit(OrderBookCreatedEvent {
            pool_id,
            base_token,
            quote_token,
            base_symbol: coin::get_symbol(base_metadata),
            quote_symbol: coin::get_symbol(quote_metadata),
            tick_size,
            lot_size,
        });
        
        // Share the pool object
        transfer::share_object(pool);
    }
    
    // === User Functions ===
    
    /// Create a DeepBook account for trading
    public entry fun create_account(ctx: &mut TxContext): AccountCap {
        clob::create_account(ctx)
    }
    
    /// Deposit base asset into DeepBook
    public entry fun deposit_base<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        token_registry: &TokenRegistry,
        fee_collector: &mut FeeCollector<BaseAsset>,
        coin: Coin<BaseAsset>,
        account_cap: &AccountCap,
        ctx: &mut TxContext
    ) {
        // Process fees if base asset is a user token
        let token_type = tx_context::type_into_address<BaseAsset>();
        let mut processed_coin = coin;
        
        if (is_user_token(token_registry, token_type)) {
            process_token_fees<BaseAsset>(
                token_registry,
                fee_collector,
                &mut processed_coin,
                ctx
            );
        };
        
        // Deposit to DeepBook
        clob::deposit_base(pool, processed_coin, account_cap);
    }
    
    /// Deposit quote asset into DeepBook
    public entry fun deposit_quote<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        token_registry: &TokenRegistry,
        fee_collector: &mut FeeCollector<QuoteAsset>,
        coin: Coin<QuoteAsset>,
        account_cap: &AccountCap,
        ctx: &mut TxContext
    ) {
        // Process fees if quote asset is a user token
        let token_type = tx_context::type_into_address<QuoteAsset>();
        let mut processed_coin = coin;
        
        if (is_user_token(token_registry, token_type)) {
            process_token_fees<QuoteAsset>(
                token_registry,
                fee_collector,
                &mut processed_coin,
                ctx
            );
        };
        
        // Deposit to DeepBook
        clob::deposit_quote(pool, processed_coin, account_cap);
    }
    
    /// Withdraw base asset from DeepBook
    public entry fun withdraw_base<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        account_cap: &AccountCap,
        quantity: u64,
        ctx: &mut TxContext
    ) {
        clob::withdraw_base(pool, quantity, account_cap, ctx);
    }
    
    /// Withdraw quote asset from DeepBook
    public entry fun withdraw_quote<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        account_cap: &AccountCap,
        quantity: u64,
        ctx: &mut TxContext
    ) {
        clob::withdraw_quote(pool, quantity, account_cap, ctx);
    }
    
    /// Place a limit order to buy base asset with quote asset
    public entry fun place_limit_order<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        account_cap: &AccountCap,
        client_order_id: u64,
        price: u64,
        quantity: u64,
        is_bid: bool,
        clock: &Clock,
        expire_timestamp: u64,
        restriction: u8,
        ctx: &mut TxContext
    ) {
        clob::place_limit_order(
            pool,
            account_cap,
            client_order_id,
            price,
            quantity,
            is_bid,
            clock::timestamp_ms(clock),
            expire_timestamp,
            restriction,
            ctx
        );
    }
    
    /// Place a market order to buy base asset with quote asset
    public entry fun place_market_order<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        token_registry: &TokenRegistry,
        fee_collector_base: &mut FeeCollector<BaseAsset>,
        fee_collector_quote: &mut FeeCollector<QuoteAsset>,
        account_cap: &AccountCap,
        client_order_id: u64,
        quantity: u64,
        is_bid: bool,
        base_coin: Coin<BaseAsset>,
        quote_coin: Coin<QuoteAsset>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Process fees for input coins if needed
        let token_type_base = tx_context::type_into_address<BaseAsset>();
        let token_type_quote = tx_context::type_into_address<QuoteAsset>();
        
        let mut processed_base_coin = base_coin;
        let mut processed_quote_coin = quote_coin;
        
        // Apply fees to the appropriate asset based on order direction
        if (is_bid && is_user_token(token_registry, token_type_quote)) {
            process_token_fees<QuoteAsset>(
                token_registry,
                fee_collector_quote,
                &mut processed_quote_coin,
                ctx
            );
        } else if (!is_bid && is_user_token(token_registry, token_type_base)) {
            process_token_fees<BaseAsset>(
                token_registry,
                fee_collector_base,
                &mut processed_base_coin,
                ctx
            );
        };
        
        // Place order in DeepBook
        let (ret_base_coin, ret_quote_coin) = clob::place_market_order(
            pool,
            account_cap,
            client_order_id,
            quantity,
            is_bid,
            processed_base_coin,
            processed_quote_coin,
            clock,
            ctx
        );
        
        // Process fees on output coins if needed
        if (is_bid && is_user_token(token_registry, token_type_base) && coin::value(&ret_base_coin) > 0) {
            // Process fees on received base token
            process_token_fees_and_return<BaseAsset>(
                token_registry,
                fee_collector_base,
                ret_base_coin,
                ctx
            );
        } else if (!is_bid && is_user_token(token_registry, token_type_quote) && coin::value(&ret_quote_coin) > 0) {
            // Process fees on received quote token
            process_token_fees_and_return<QuoteAsset>(
                token_registry,
                fee_collector_quote,
                ret_quote_coin,
                ctx
            );
        } else {
            // Return coins directly if no fee processing needed
            if (coin::value(&ret_base_coin) > 0) {
                transfer::public_transfer(ret_base_coin, tx_context::sender(ctx));
            } else {
                coin::destroy_zero(ret_base_coin);
            };
            
            if (coin::value(&ret_quote_coin) > 0) {
                transfer::public_transfer(ret_quote_coin, tx_context::sender(ctx));
            } else {
                coin::destroy_zero(ret_quote_coin);
            };
        };
    }
    
    /// Swap exact base for quote using DeepBook
    public entry fun swap_exact_base_for_quote<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        token_registry: &TokenRegistry,
        fee_collector_base: &mut FeeCollector<BaseAsset>,
        fee_collector_quote: &mut FeeCollector<QuoteAsset>,
        account_cap: &AccountCap,
        client_order_id: u64,
        quantity: u64,
        base_coin: Coin<BaseAsset>,
        quote_coin: Coin<QuoteAsset>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Process fees for input base coin if needed
        let token_type_base = tx_context::type_into_address<BaseAsset>();
        let token_type_quote = tx_context::type_into_address<QuoteAsset>();
        
        let mut processed_base_coin = base_coin;
        let mut processed_quote_coin = quote_coin;
        
        // Process fees on input base asset if it's a user token
        if (is_user_token(token_registry, token_type_base)) {
            process_token_fees<BaseAsset>(
                token_registry,
                fee_collector_base,
                &mut processed_base_coin,
                ctx
            );
        };
        
        // Execute swap in DeepBook
        let (ret_base_coin, ret_quote_coin, _amount_out) = clob::swap_exact_base_for_quote(
            pool,
            client_order_id,
            account_cap,
            quantity,
            processed_base_coin,
            processed_quote_coin,
            clock,
            ctx
        );
        
        // Process fees on output quote coin if it's a user token
        if (is_user_token(token_registry, token_type_quote) && coin::value(&ret_quote_coin) > 0) {
            let processed_coin = process_token_fees_and_return<QuoteAsset>(
                token_registry,
                fee_collector_quote,
                ret_quote_coin,
                ctx
            );
            transfer::public_transfer(processed_coin, tx_context::sender(ctx));
        } else if (coin::value(&ret_quote_coin) > 0) {
            transfer::public_transfer(ret_quote_coin, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(ret_quote_coin);
        };
        
        // Return any remaining base coin
        if (coin::value(&ret_base_coin) > 0) {
            transfer::public_transfer(ret_base_coin, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(ret_base_coin);
        };
    }
    
    /// Swap exact quote for base using DeepBook
    public entry fun swap_exact_quote_for_base<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        token_registry: &TokenRegistry,
        fee_collector_base: &mut FeeCollector<BaseAsset>,
        fee_collector_quote: &mut FeeCollector<QuoteAsset>,
        account_cap: &AccountCap,
        client_order_id: u64,
        quantity: u64,
        quote_coin: Coin<QuoteAsset>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Process fees for input quote coin if needed
        let token_type_base = tx_context::type_into_address<BaseAsset>();
        let token_type_quote = tx_context::type_into_address<QuoteAsset>();
        
        let mut processed_quote_coin = quote_coin;
        
        // Process fees on input quote asset if it's a user token
        if (is_user_token(token_registry, token_type_quote)) {
            process_token_fees<QuoteAsset>(
                token_registry,
                fee_collector_quote,
                &mut processed_quote_coin,
                ctx
            );
        };
        
        // Execute swap in DeepBook
        let (ret_base_coin, ret_quote_coin, _amount_out) = clob::swap_exact_quote_for_base(
            pool,
            client_order_id,
            account_cap,
            quantity,
            clock,
            processed_quote_coin,
            ctx
        );
        
        // Process fees on output base coin if it's a user token
        if (is_user_token(token_registry, token_type_base) && coin::value(&ret_base_coin) > 0) {
            let processed_coin = process_token_fees_and_return<BaseAsset>(
                token_registry,
                fee_collector_base,
                ret_base_coin,
                ctx
            );
            transfer::public_transfer(processed_coin, tx_context::sender(ctx));
        } else if (coin::value(&ret_base_coin) > 0) {
            transfer::public_transfer(ret_base_coin, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(ret_base_coin);
        };
        
        // Return any remaining quote coin
        if (coin::value(&ret_quote_coin) > 0) {
            transfer::public_transfer(ret_quote_coin, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(ret_quote_coin);
        };
    }
    
    /// Cancel an order in DeepBook
    public entry fun cancel_order<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        account_cap: &AccountCap,
        order_id: u64,
        ctx: &mut TxContext
    ) {
        clob::cancel_order(pool, account_cap, order_id, ctx);
    }
    
    /// Cancel all orders for a user in DeepBook
    public entry fun cancel_all_orders<BaseAsset, QuoteAsset>(
        pool: &mut Pool<BaseAsset, QuoteAsset>,
        account_cap: &AccountCap,
        ctx: &mut TxContext
    ) {
        clob::cancel_all_orders(pool, account_cap, ctx);
    }
    
    // === Helper Functions ===
    
    /// Check if a token is a user token
    public fun is_user_token(registry: &TokenRegistry, token_type: address): bool {
        let keys = user_token::get_all_tokens(registry);
        let i = 0;
        let len = vector::length(&keys);
        
        while (i < len) {
            if (*vector::borrow(&keys, i) == token_type) {
                return true
            };
            i = i + 1;
        };
        
        false
    }
    
    /// Get token pair info from registry
    public fun get_token_pair_info(
        registry: &OrderBookRegistry,
        base_token: address,
        quote_token: address
    ): (bool, TokenPairInfo) {
        let i = 0;
        let len = vector::length(&registry.token_pools);
        
        while (i < len) {
            let pair_info = vector::borrow(&registry.token_pools, i);
            if (pair_info.base_token == base_token && pair_info.quote_token == quote_token) {
                return (true, *pair_info)
            };
            i = i + 1;
        };
        
        (false, TokenPairInfo {
            base_token: @0x0,
            quote_token: @0x0,
            pool_id: @0x0,
            base_token_creator: @0x0,
            quote_token_creator: @0x0,
        })
    }
    
    /// Get all order books for a specific token
    public fun get_all_pools_for_token(
        registry: &OrderBookRegistry,
        token: address
    ): vector<TokenPairInfo> {
        let result = vector::empty<TokenPairInfo>();
        let i = 0;
        let len = vector::length(&registry.token_pools);
        
        while (i < len) {
            let pair_info = vector::borrow(&registry.token_pools, i);
            if (pair_info.base_token == token || pair_info.quote_token == token) {
                vector::push_back(&mut result, *pair_info);
            };
            i = i + 1;
        };
        
        result
    }
    
    /// Process fees for a coin
    fun process_token_fees<T>(
        registry: &TokenRegistry,
        fee_collector: &mut FeeCollector<T>,
        coin: &mut Coin<T>,
        ctx: &mut TxContext
    ) {
        let token_type = tx_context::type_into_address<T>();
        
        // Get token info
        if (!is_user_token(registry, token_type)) {
            return
        };
        
        let token_info = user_token::find_token_info(registry, token_type);
        
        // Calculate fees
        let amount = coin::value(coin);
        let fee_amount = (amount * token_info.commission_bps) / 10000;
        
        if (fee_amount > 0) {
            let creator_fee = (fee_amount * token_info.creator_split_bps) / 10000;
            let platform_fee = fee_amount - creator_fee;
            
            // Take fee from coin
            let fee_coin = coin::split(coin, fee_amount, ctx);
            let fee_balance = coin::into_balance(fee_coin);
            
            // Split fee between creator and platform
            if (creator_fee > 0) {
                let creator_fee_balance = balance::split(&mut fee_balance, creator_fee);
                balance::join(&mut fee_collector.creator_balance, creator_fee_balance);
            };
            
            // Add remaining fee to platform
            balance::join(&mut fee_collector.platform_balance, fee_balance);
            
            // Emit event for fee collection
            event::emit(TokenTradeFeesCollectedEvent {
                token_id: token_type,
                token_creator: token_info.user,
                fee_amount,
                creator_fee,
                platform_fee,
            });
        };
    }
    
    /// Process fees for a coin and return the processed coin
    fun process_token_fees_and_return<T>(
        registry: &TokenRegistry,
        fee_collector: &mut FeeCollector<T>,
        coin: Coin<T>,
        ctx: &mut TxContext
    ): Coin<T> {
        let mut processed_coin = coin;
        process_token_fees(registry, fee_collector, &mut processed_coin, ctx);
        processed_coin
    }
}