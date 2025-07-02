# Linear Vesting Integration Guide

This guide explains how to implement and use the linear vesting strategy for treasury tokens during Genesis initialization in the Mys blockchain.

## Overview

The linear vesting strategy allows tokens to be gradually released over time, ensuring long-term commitment and preventing market dumps. This implementation is based on the Sui documentation for vesting strategies but adapted for the Mys framework.

## Components

### 1. Move Module: `linear_vesting.move`

Located at: `crates/mys-framework/packages/mys-framework/sources/linear_vesting.move`

This module provides:
- **VestingWallet<T>**: A wallet that holds tokens and releases them linearly over time
- **Functions**:
  - `new_vesting_wallet()`: Create a new vesting wallet (with clock validation)
  - `new_genesis_vesting_wallet()`: Create a vesting wallet at genesis (no clock validation)
  - `claim()`: Claim available vested tokens
  - `claimable()`: Calculate how many tokens can be claimed at current time
  - Various accessor and management functions

**Key Features**:
- Linear vesting over a specified duration
- Beneficiary-based access control
- Overflow protection in calculations
- Support for ownership transfer

### 2. Genesis Configuration: `genesis.rs`

Enhanced structures to support vesting:

```rust
pub struct TokenAllocation {
    pub recipient_address: MysAddress,
    pub amount_mist: u64,
    pub staked_with_validator: Option<MysAddress>,
    pub vesting_schedule: Option<VestingSchedule>, // NEW
}

pub struct VestingSchedule {
    pub start_timestamp_ms: u64,
    pub duration_ms: u64,
    pub vesting_type: VestingType,
}

pub enum VestingType {
    Linear,
    // Future: Cliff, Graded, etc.
}
```

### 3. Genesis Creation: `genesis.move`

Updated to handle vesting allocations during genesis creation:
- Imports the `linear_vesting` module
- Creates vesting wallets for allocations with vesting schedules
- Transfers vesting wallets to beneficiaries

### 4. Builder Helper Methods

The `TokenDistributionScheduleBuilder` and genesis `Builder` include helper methods:

```rust
// Add single vesting allocation
builder.add_treasury_vesting_allocation(
    recipient_address,
    amount_mist,
    start_timestamp_ms,
    duration_ms,
);

// Add multiple vesting allocations
builder.add_treasury_vesting_allocations(
    recipients, // Vec<(MysAddress, u64)>
    start_timestamp_ms,
    duration_ms,
);
```

## Usage Examples

### Example 1: Basic Treasury Vesting

```rust
use mys_genesis_builder::Builder;
use mys_types::base_types::MysAddress;

let mut builder = Builder::new();

// Set up basic parameters
builder = builder.with_token_parameters(
    "MySo".to_string(),
    "MySocial".to_string(),
    "The native token of the MySocial blockchain.".to_string()
);

// Calculate timing
let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64;

let one_year_ms = 365 * 24 * 60 * 60 * 1000;
let four_years_ms = 4 * one_year_ms;

// Treasury vesting: Start in 1 year, vest over 4 years
let treasury_start = now + one_year_ms;
let treasury_duration = four_years_ms;

// Allocate 10% of total supply to treasury
let total_supply_mist = 1_000_000_000_000_000_000u64; // 1B tokens
let treasury_allocation = total_supply_mist / 10; // 10%

// Create multiple treasury vesting allocations
let treasury_recipients = vec![
    (core_team_address, treasury_allocation / 3),    // Core team
    (foundation_address, treasury_allocation / 3),   // Foundation
    (advisors_address, treasury_allocation / 3),     // Advisors
];

builder = builder.add_treasury_vesting_batch(
    treasury_recipients,
    treasury_start,
    treasury_duration,
);
```

### Example 2: Multiple Vesting Schedules

```rust
// Team allocation: 2-year vesting starting immediately
let team_start = now;
let team_duration = 2 * one_year_ms;
let team_allocation = total_supply_mist / 20; // 5%

builder = builder.add_treasury_vesting(
    team_address,
    team_allocation,
    team_start,
    team_duration,
);

// Investor allocation: 3-year vesting with 1-year cliff
let investor_start = now + one_year_ms; // 1-year cliff
let investor_duration = 3 * one_year_ms;
let investor_allocation = total_supply_mist / 10; // 10%

builder = builder.add_treasury_vesting(
    investor_address,
    investor_allocation,
    investor_start,
    investor_duration,
);
```

### Example 3: Using in Genesis Ceremony

```rust
// In genesis ceremony setup
let command = Ceremony {
    path: Some(genesis_dir.into()),
    protocol_version: None,
    token_symbol: Some("MySo".to_string()),
    token_name: Some("MySocial".to_string()),
    token_description: Some("Native token with treasury vesting".to_string()),
    token_supply: None,
    command: CeremonyCommand::Init,
};

// After initialization, you can modify the builder to add vesting
let mut builder = Builder::load(&genesis_dir)?;

// Add treasury vesting allocations
builder = builder.add_treasury_vesting_batch(
    treasury_allocations,
    vesting_start_time,
    vesting_duration,
);

builder.save(genesis_dir)?;
```

## Token Claiming Process

Once the blockchain is live, beneficiaries can claim their vested tokens:

### Move Code Example

```move
// In a transaction, beneficiaries can claim vested tokens
public entry fun claim_vested_tokens<T>(
    wallet: &mut VestingWallet<T>,
    clock: &Clock,
    ctx: &mut TxContext
) {
    // This will transfer claimable tokens to the sender
    let claimed_coins = linear_vesting::claim(wallet, clock, ctx);
    
    // Sender receives the coins automatically
    // Coins can be used immediately or stored
}

// Check how much can be claimed
public fun check_claimable<T>(
    wallet: &VestingWallet<T>,
    clock: &Clock
): u64 {
    linear_vesting::claimable(wallet, clock)
}
```

## Benefits of This Implementation

1. **Long-term Commitment**: Ensures team and treasury tokens are released gradually
2. **Market Stability**: Prevents large token dumps that could harm token value
3. **Transparency**: All vesting schedules are set at genesis and visible on-chain
4. **Flexibility**: Can support different vesting schedules for different stakeholders
5. **Security**: Beneficiary-based access control prevents unauthorized claiming

## Technical Details

### Vesting Calculation

The linear vesting calculation uses the formula:
```
claimable_amount = (total_amount × elapsed_time) / total_duration - already_claimed
```

Key features:
- Uses 128-bit arithmetic to prevent overflow
- Handles edge cases (before start, after end)
- Accounts for previously claimed amounts

### Gas Efficiency

- Minimal storage overhead
- Efficient calculation algorithms  
- Batch operations supported

### Security Considerations

- Beneficiary-only access control
- Overflow protection in calculations
- Proper error handling for edge cases
- Immutable vesting parameters once created

## Future Extensions

The current implementation focuses on linear vesting but can be extended to support:

1. **Cliff Vesting**: All tokens released after a specific date
2. **Graded Vesting**: Tokens released in tranches
3. **Milestone-based Vesting**: Tokens released based on achievements
4. **Hybrid Vesting**: Combinations of different strategies

## Testing

The implementation includes comprehensive testing for:
- Vesting calculation accuracy
- Edge case handling
- Access control
- Genesis integration

## Conclusion

This linear vesting implementation provides a robust foundation for treasury token management at genesis, ensuring long-term project sustainability while maintaining transparency and security. The modular design allows for future extensions and customizations based on project needs.