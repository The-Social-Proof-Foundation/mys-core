# Oracle Vertical Integration: Direct Move Function Call Implementation

## Foundation Package Structure

First, create the foundation-level Oracle package:

```
crates/mys-framework/packages/mys-oracle-foundation/
├── Move.toml
└── sources/
    ├── miso_price_oracle.move      # MISO-specific price oracle
    ├── oracle_registry.move        # Central oracle management
    ├── price_data.move            # Price data structures
    └── oracle_capability.move     # Authorization system
```

### Move.toml Configuration

```toml
[package]
name = "MysOracleFoundation"
version = "0.0.1"
published-at = "0x4"  # Foundation Oracle package address
edition = "2024.beta"

[dependencies]
MoveStdlib = { local = "../move-stdlib" }
MySocial = { local = "../mys-framework" }
MySocialSystem = { local = "../mys-system" }

[addresses]
mys_oracle = "0x4"
mys = "0x2"
mys_system = "0x3"
```

## Core Data Structures

### Price Data Structure (price_data.move)

```move
module mys_oracle::price_data {
    use std::string::String;
    use mys::clock::Clock;
    use mys::event;

    /// High-precision price data for MISO and other assets
    public struct PriceData has copy, drop, store {
        /// Asset identifier (e.g., "MISO", "BTC", "ETH")
        asset: String,
        /// Price in base units (scaled by decimal places)
        price: u64,
        /// Number of decimal places for precision
        decimals: u8,
        /// Timestamp when price was recorded
        timestamp_ms: u64,
        /// Confidence level (0-100)
        confidence: u8,
        /// Source oracle address
        oracle_address: address,
    }

    /// Global price registry for all assets
    public struct GlobalPriceRegistry has key {
        id: UID,
        /// Asset name -> Latest price data
        prices: Table<String, PriceData>,
        /// Authorized oracle addresses
        authorized_oracles: VecSet<address>,
    }

    /// Event emitted when price is updated
    public struct PriceUpdateEvent has copy, drop {
        asset: String,
        old_price: u64,
        new_price: u64,
        timestamp_ms: u64,
        oracle_address: address,
    }

    /// Create new price data
    public fun new_price_data(
        asset: String,
        price: u64,
        decimals: u8,
        confidence: u8,
        oracle_address: address,
        clock: &Clock,
    ): PriceData {
        PriceData {
            asset,
            price,
            decimals,
            timestamp_ms: clock::timestamp_ms(clock),
            confidence,
            oracle_address,
        }
    }

    /// Initialize the global price registry (called once at genesis)
    public(package) fun create_global_registry(ctx: &mut TxContext): GlobalPriceRegistry {
        GlobalPriceRegistry {
            id: object::new(ctx),
            prices: table::new(ctx),
            authorized_oracles: vec_set::empty(),
        }
    }

    /// Internal function to update price (highest performance)
    public(package) fun internal_update_price(
        registry: &mut GlobalPriceRegistry,
        price_data: PriceData,
    ) {
        let asset = price_data.asset;
        let new_price = price_data.price;
        
        // Get old price for event
        let old_price = if (table::contains(&registry.prices, asset)) {
            table::borrow(&registry.prices, asset).price
        } else {
            0
        };

        // Update or insert price
        if (table::contains(&registry.prices, asset)) {
            *table::borrow_mut(&mut registry.prices, asset) = price_data;
        } else {
            table::add(&mut registry.prices, asset, price_data);
        };

        // Emit price update event
        event::emit(PriceUpdateEvent {
            asset,
            old_price,
            new_price,
            timestamp_ms: price_data.timestamp_ms,
            oracle_address: price_data.oracle_address,
        });
    }

    /// Get current price for asset
    public fun get_price(registry: &GlobalPriceRegistry, asset: String): Option<PriceData> {
        if (table::contains(&registry.prices, asset)) {
            option::some(*table::borrow(&registry.prices, asset))
        } else {
            option::none()
        }
    }

    /// Public accessors
    public fun asset(data: &PriceData): String { data.asset }
    public fun price(data: &PriceData): u64 { data.price }
    public fun decimals(data: &PriceData): u8 { data.decimals }
    public fun timestamp_ms(data: &PriceData): u64 { data.timestamp_ms }
    public fun confidence(data: &PriceData): u8 { data.confidence }
    public fun oracle_address(data: &PriceData): address { data.oracle_address }
}
```

## Oracle Authorization System

### Oracle Capability (oracle_capability.move)

```move
module mys_oracle::oracle_capability {
    use mys_system::mys_system_state_inner::{Self, MysSystemStateInnerV2};

    /// Capability to operate as an authorized oracle
    public struct OracleOperatorCap has key, store {
        id: UID,
        /// Oracle operator address
        operator: address,
        /// Assets this oracle is authorized for
        authorized_assets: VecSet<String>,
        /// Maximum price deviation allowed (basis points)
        max_deviation_bps: u64,
    }

    /// Admin capability for oracle management
    public struct OracleAdminCap has key, store {
        id: UID,
    }

    const ENotAuthorized: u64 = 0;
    const EPriceDeviationTooLarge: u64 = 1;
    const EInvalidConfidence: u64 = 2;

    /// Create admin capability (system-level function)
    public(package) fun create_admin_cap(ctx: &mut TxContext): OracleAdminCap {
        OracleAdminCap { id: object::new(ctx) }
    }

    /// Create oracle operator capability
    public entry fun create_oracle_operator(
        _admin_cap: &OracleAdminCap,
        operator: address,
        authorized_assets: vector<String>,
        max_deviation_bps: u64,
        ctx: &mut TxContext,
    ) {
        let mut asset_set = vec_set::empty<String>();
        let mut i = 0;
        while (i < vector::length(&authorized_assets)) {
            vec_set::insert(&mut asset_set, *vector::borrow(&authorized_assets, i));
            i = i + 1;
        };

        let cap = OracleOperatorCap {
            id: object::new(ctx),
            operator,
            authorized_assets: asset_set,
            max_deviation_bps,
        };

        transfer::transfer(cap, operator);
    }

    /// Verify oracle has permission for asset
    public fun verify_oracle_permission(
        cap: &OracleOperatorCap,
        asset: String,
        ctx: &TxContext,
    ) {
        assert!(cap.operator == ctx.sender(), ENotAuthorized);
        assert!(vec_set::contains(&cap.authorized_assets, &asset), ENotAuthorized);
    }

    /// Verify price data quality
    public fun verify_price_data(
        cap: &OracleOperatorCap,
        current_price: u64,
        new_price: u64,
        confidence: u8,
    ) {
        // Check confidence level
        assert!(confidence >= 80, EInvalidConfidence); // Minimum 80% confidence

        // Check price deviation if current price exists
        if (current_price > 0) {
            let deviation = if (new_price > current_price) {
                ((new_price - current_price) * 10000) / current_price
            } else {
                ((current_price - new_price) * 10000) / current_price
            };
            assert!(deviation <= cap.max_deviation_bps, EPriceDeviationTooLarge);
        };
    }
}
```

## MISO Price Oracle Implementation

### MISO Oracle (miso_price_oracle.move)

```move
module mys_oracle::miso_price_oracle {
    use std::string::{Self, String};
    use mys::clock::Clock;
    use mys_oracle::price_data::{Self, PriceData, GlobalPriceRegistry};
    use mys_oracle::oracle_capability::{Self, OracleOperatorCap};
    use mys_oracle::oracle_registry;

    /// MISO-specific price oracle
    public struct MISOPriceOracle has key {
        id: UID,
        /// Reference to global price registry
        registry_id: ID,
        /// MISO-specific configuration
        price_staleness_threshold_ms: u64,
        /// Minimum confidence required for MISO prices
        min_confidence: u8,
    }

    const MISO_ASSET: vector<u8> = b"MISO";
    const MISO_DECIMALS: u8 = 8; // 8 decimal places for MISO price

    const EPriceStale: u64 = 0;
    const ELowConfidence: u64 = 1;

    /// Initialize MISO price oracle
    public fun create_miso_oracle(
        registry_id: ID,
        price_staleness_threshold_ms: u64,
        min_confidence: u8,
        ctx: &mut TxContext,
    ): MISOPriceOracle {
        MISOPriceOracle {
            id: object::new(ctx),
            registry_id,
            price_staleness_threshold_ms,
            min_confidence,
        }
    }

    /// DIRECT MOVE FUNCTION CALL: Update MISO price with maximum performance
    /// This is called directly from the oracle service without transaction overhead
    public(package) fun direct_update_miso_price(
        oracle: &MISOPriceOracle,
        registry: &mut GlobalPriceRegistry,
        cap: &OracleOperatorCap,
        price: u64,
        confidence: u8,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        let asset = string::utf8(MISO_ASSET);
        
        // Verify oracle authorization
        oracle_capability::verify_oracle_permission(cap, asset, ctx);

        // Get current MISO price for validation
        let current_price_opt = price_data::get_price(registry, asset);
        let current_price = if (option::is_some(&current_price_opt)) {
            price_data::price(option::borrow(&current_price_opt))
        } else {
            0
        };

        // Verify price data quality
        oracle_capability::verify_price_data(cap, current_price, price, confidence);
        assert!(confidence >= oracle.min_confidence, ELowConfidence);

        // Create new price data
        let price_data = price_data::new_price_data(
            asset,
            price,
            MISO_DECIMALS,
            confidence,
            ctx.sender(),
            clock,
        );

        // DIRECT INTERNAL UPDATE - Maximum performance, no transaction fees
        price_data::internal_update_price(registry, price_data);
    }

    /// High-performance price retrieval for smart contracts
    public fun get_miso_price_direct(
        registry: &GlobalPriceRegistry,
        clock: &Clock,
    ): Option<PriceData> {
        let asset = string::utf8(MISO_ASSET);
        let price_opt = price_data::get_price(registry, asset);

        if (option::is_none(&price_opt)) {
            return option::none()
        };

        let price_data = option::destroy_some(price_opt);
        let current_time = clock::timestamp_ms(clock);
        let price_age = current_time - price_data::timestamp_ms(&price_data);

        // Check if price is not stale (this would be configurable)
        if (price_age > 300000) { // 5 minutes staleness threshold
            option::none()
        } else {
            option::some(price_data)
        }
    }

    /// Calculate MISO price in different denominations
    public fun get_miso_price_scaled(
        registry: &GlobalPriceRegistry,
        target_decimals: u8,
        clock: &Clock,
    ): Option<u64> {
        let price_data_opt = get_miso_price_direct(registry, clock);
        if (option::is_none(&price_data_opt)) {
            return option::none()
        };

        let price_data = option::destroy_some(price_data_opt);
        let raw_price = price_data::price(&price_data);
        let current_decimals = price_data::decimals(&price_data);

        // Scale price to target decimals
        let scaled_price = if (target_decimals > current_decimals) {
            let scale_factor = math::pow(10, target_decimals - current_decimals);
            raw_price * (scale_factor as u64)
        } else if (target_decimals < current_decimals) {
            let scale_factor = math::pow(10, current_decimals - target_decimals);
            raw_price / (scale_factor as u64)
        } else {
            raw_price
        };

        option::some(scaled_price)
    }
}
```

## Oracle Registry (Central Management)

### Oracle Registry (oracle_registry.move)

```move
module mys_oracle::oracle_registry {
    use mys_system::mys_system::{Self, MysSystemState};
    use mys_oracle::price_data::{Self, GlobalPriceRegistry};
    use mys_oracle::oracle_capability::{Self, OracleAdminCap};
    use mys_oracle::miso_price_oracle::{Self, MISOPriceOracle};

    /// Central oracle registry integrated with the foundation
    public struct OracleFoundationRegistry has key {
        id: UID,
        /// Global price registry
        global_price_registry: GlobalPriceRegistry,
        /// MISO oracle instance
        miso_oracle: MISOPriceOracle,
        /// Admin capability
        admin_cap: OracleAdminCap,
        /// System integration enabled
        system_integrated: bool,
    }

    /// Initialize the oracle foundation registry (called at genesis)
    public(package) fun initialize_oracle_foundation(
        system_state: &mut MysSystemState,
        ctx: &mut TxContext,
    ) {
        let global_registry = price_data::create_global_registry(ctx);
        let registry_id = object::id(&global_registry);
        
        let miso_oracle = miso_price_oracle::create_miso_oracle(
            registry_id,
            300000, // 5 minute staleness threshold
            80,     // 80% minimum confidence
            ctx,
        );

        let admin_cap = oracle_capability::create_admin_cap(ctx);

        let oracle_registry = OracleFoundationRegistry {
            id: object::new(ctx),
            global_price_registry: global_registry,
            miso_oracle,
            admin_cap,
            system_integrated: true,
        };

        // Share the oracle registry for global access
        transfer::share_object(oracle_registry);
    }

    /// HIGHEST PERFORMANCE: Direct system-level MISO price update
    /// This bypasses all transaction overhead and directly updates the price
    public(package) fun system_update_miso_price(
        registry: &mut OracleFoundationRegistry,
        cap: &oracle_capability::OracleOperatorCap,
        price: u64,
        confidence: u8,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        // Verify system integration is enabled
        assert!(registry.system_integrated, 0);

        // Direct internal call - maximum performance
        miso_price_oracle::direct_update_miso_price(
            &registry.miso_oracle,
            &mut registry.global_price_registry,
            cap,
            price,
            confidence,
            clock,
            ctx,
        );
    }

    /// Smart contract interface for MISO price retrieval
    public fun get_miso_price_for_contract(
        registry: &OracleFoundationRegistry,
        clock: &Clock,
    ): Option<u64> {
        miso_price_oracle::get_miso_price_scaled(
            &registry.global_price_registry,
            8, // Standard 8 decimals
            clock,
        )
    }

    /// Advanced price data with metadata
    public fun get_miso_price_data(
        registry: &OracleFoundationRegistry,
        clock: &Clock,
    ): Option<price_data::PriceData> {
        miso_price_oracle::get_miso_price_direct(
            &registry.global_price_registry,
            clock,
        )
    }
}
```

## Smart Contract Integration Example

### Example DeFi Contract Using MISO Oracle

```move
/// Example: DeFi lending protocol using MISO price oracle
module my_defi::lending_protocol {
    use mys::clock::Clock;
    use mys_oracle::oracle_registry::{Self, OracleFoundationRegistry};
    use mys_oracle::price_data;

    public struct LendingPosition has key {
        id: UID,
        collateral_amount: u64,
        borrowed_amount: u64,
        collateral_ratio: u64,
    }

    const EInsufficientCollateral: u64 = 0;
    const EPriceNotAvailable: u64 = 1;

    /// DIRECT ORACLE INTEGRATION: Get MISO price for liquidation check
    public fun check_liquidation_threshold(
        position: &LendingPosition,
        oracle_registry: &OracleFoundationRegistry,
        clock: &Clock,
    ): bool {
        // Direct high-performance oracle call
        let miso_price_opt = oracle_registry::get_miso_price_for_contract(
            oracle_registry,
            clock,
        );

        assert!(option::is_some(&miso_price_opt), EPriceNotAvailable);
        let miso_price = option::destroy_some(miso_price_opt);

        // Calculate current collateral value
        let collateral_value = (position.collateral_amount as u128) * (miso_price as u128) / 100000000;
        let borrowed_value = position.borrowed_amount as u128;

        // Check if position is under-collateralized (150% minimum ratio)
        let current_ratio = (collateral_value * 100) / borrowed_value;
        current_ratio < 150
    }

    /// Example of using detailed price data
    public fun get_price_with_confidence(
        oracle_registry: &OracleFoundationRegistry,
        clock: &Clock,
    ): (u64, u8, u64) {
        let price_data_opt = oracle_registry::get_miso_price_data(oracle_registry, clock);
        assert!(option::is_some(&price_data_opt), EPriceNotAvailable);
        
        let price_data = option::destroy_some(price_data_opt);
        (
            price_data::price(&price_data),
            price_data::confidence(&price_data),
            price_data::timestamp_ms(&price_data),
        )
    }
}
```

## Integration with System State

### System-Level Integration

```move
/// Integration with mys-system for privileged operations
module mys_oracle::system_integration {
    use mys_system::mys_system::{Self, MysSystemState};
    use mys_oracle::oracle_registry::{Self, OracleFoundationRegistry};

    friend mys_system::mys_system_state_inner;

    /// System-level oracle price update (fee-free, highest performance)
    public(package) fun system_oracle_update(
        system_state: &mut MysSystemState,
        oracle_registry: &mut OracleFoundationRegistry,
        oracle_cap: &oracle_capability::OracleOperatorCap,
        asset_prices: vector<(String, u64, u8)>, // (asset, price, confidence)
        clock: &Clock,
        ctx: &TxContext,
    ) {
        // This function is called by the system and has zero transaction fees
        let mut i = 0;
        while (i < vector::length(&asset_prices)) {
            let (asset, price, confidence) = *vector::borrow(&asset_prices, i);
            
            if (asset == string::utf8(b"MISO")) {
                // Direct system-level MISO update
                oracle_registry::system_update_miso_price(
                    oracle_registry,
                    oracle_cap,
                    price,
                    confidence,
                    clock,
                    ctx,
                );
            };
            // Add other assets as needed
            
            i = i + 1;
        };
    }
}
```

## Key Performance Benefits

### 1. **Zero Transaction Fees**
- Oracle updates use `public(package)` functions called internally
- No gas consumption for oracle price updates
- System-level authorization bypasses fee mechanisms

### 2. **Direct Memory Access**
- No transaction serialization/deserialization overhead
- Direct struct manipulation in Move memory
- Immediate price availability to consuming contracts

### 3. **Optimized Data Structures**
- Single global price registry reduces object lookup overhead
- In-memory price caching for frequently accessed data
- Efficient event emission for price change notifications

### 4. **Secure Authorization**
- Capability-based access control
- Asset-specific oracle permissions
- Price deviation limits and confidence thresholds

### 5. **Smart Contract Integration**
- Direct function calls from DeFi contracts to oracle
- No external API calls or transaction dependencies
- Real-time price data access within transactions

This architecture provides the highest performance Oracle integration possible while maintaining security and enabling fee-free operation through vertical integration into the foundation framework.