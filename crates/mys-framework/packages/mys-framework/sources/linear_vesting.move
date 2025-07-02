// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// ===========================================================================================
/// Module: linear_vesting
/// Description:
/// This module defines a vesting strategy that allows users to claim coins linearly over time.
///
/// Functionality:
/// - Defines a linear vesting schedule for treasury tokens at genesis.
/// ===========================================================================================
module mys::linear_vesting;

use mys::balance::{Self, Balance};
use mys::clock::Clock;
use mys::coin::{Self, Coin};
use mys::object::{Self, UID};
use mys::transfer;
use mys::tx_context::TxContext;

// === Errors ===
#[error]
const EInvalidStartTime: vector<u8> = b"Start time must be in the future.";
#[error]
const EInsufficientVestedAmount: vector<u8> = b"Insufficient vested amount available.";
#[error]
const EUnauthorizedClaimer: vector<u8> = b"Unauthorized claimer.";

// === Structs ===

/// [Owned] Wallet contains coins that are available for claiming over time.
public struct VestingWallet<phantom T> has key, store {
    id: UID,
    // Amount of coins remaining in the wallet
    balance: Balance<T>,
    // Time when the vesting started
    start: u64,
    // Amount of coins that have been claimed
    claimed: u64,
    // Total duration of the vesting schedule
    duration: u64,
    // Address authorized to claim from this wallet
    beneficiary: address,
}

// === Public Functions ===

/// Create a new vesting wallet with the given coins and vesting duration.
/// Note that full amount of coins is stored in the wallet when it is created;
/// it is just that the coins need to be claimed over time.
///
/// @aborts with `EInvalidStartTime` if the start time is not in the future.
public fun new_vesting_wallet<T>(
    coins: Coin<T>,
    clock: &Clock,
    start: u64,
    duration: u64,
    beneficiary: address,
    ctx: &mut TxContext,
): VestingWallet<T> {
    assert!(start > clock.timestamp_ms(), EInvalidStartTime);
    VestingWallet {
        id: object::new(ctx),
        balance: coins.into_balance(),
        start,
        claimed: 0,
        duration,
        beneficiary,
    }
}

/// Create a new vesting wallet at genesis time (no clock validation needed).
/// This function should only be called during genesis creation.
public(package) fun new_genesis_vesting_wallet<T>(
    coins: Coin<T>,
    start: u64,
    duration: u64,
    beneficiary: address,
    ctx: &mut TxContext,
): VestingWallet<T> {
    VestingWallet {
        id: object::new(ctx),
        balance: coins.into_balance(),
        start,
        claimed: 0,
        duration,
        beneficiary,
    }
}

/// Claim the coins that are available for claiming at the current time.
public fun claim<T>(
    self: &mut VestingWallet<T>, 
    clock: &Clock, 
    ctx: &mut TxContext
): Coin<T> {
    assert!(ctx.sender() == self.beneficiary, EUnauthorizedClaimer);
    
    let claimable_amount = self.claimable(clock);
    assert!(claimable_amount > 0, EInsufficientVestedAmount);
    
    self.claimed = self.claimed + claimable_amount;
    coin::from_balance(self.balance.split(claimable_amount), ctx)
}

/// Calculate the amount of coins that can be claimed at the current time.
public fun claimable<T>(self: &VestingWallet<T>, clock: &Clock): u64 {
    let timestamp = clock.timestamp_ms();
    if (timestamp < self.start) return 0;
    if (timestamp >= self.start + self.duration) return self.balance.value();
    
    let elapsed = timestamp - self.start;
    // Convert the balance to u128 to account for overflow in the calculation
    // Note that the division by zero is not possible because when duration is zero, the balance is returned above
    let total_original_amount = (self.balance.value() as u128) + (self.claimed as u128);
    let claimable: u128 = total_original_amount * (elapsed as u128) / (self.duration as u128);
    
    // Adjust the claimable amount by subtracting the already claimed amount
    let claimable_amount = (claimable as u64);
    if (claimable_amount > self.claimed) {
        claimable_amount - self.claimed
    } else {
        0
    }
}

/// Transfer ownership of the vesting wallet to a new beneficiary.
public fun transfer_ownership<T>(
    self: &mut VestingWallet<T>,
    new_beneficiary: address,
    ctx: &mut TxContext,
) {
    assert!(ctx.sender() == self.beneficiary, EUnauthorizedClaimer);
    self.beneficiary = new_beneficiary;
}

/// Delete the wallet if it is empty.
public fun delete_wallet<T>(self: VestingWallet<T>) {
    let VestingWallet { 
        id, 
        start: _, 
        balance, 
        claimed: _, 
        duration: _, 
        beneficiary: _ 
    } = self;
    id.delete();
    balance.destroy_zero();
}

/// Transfer the vesting wallet to another address.
public fun transfer_wallet<T>(wallet: VestingWallet<T>, recipient: address) {
    transfer::transfer(wallet, recipient);
}

/// Make the vesting wallet a shared object for public access.
public fun share_wallet<T>(wallet: VestingWallet<T>) {
    transfer::share_object(wallet);
}

// === Accessors ===

/// Get the remaining balance of the wallet.
public fun balance<T>(self: &VestingWallet<T>): u64 {
    self.balance.value()
}

/// Get the start time of the vesting schedule.
public fun start<T>(self: &VestingWallet<T>): u64 {
    self.start
}

/// Get the duration of the vesting schedule.
public fun duration<T>(self: &VestingWallet<T>): u64 {
    self.duration
}

/// Get the beneficiary of the vesting wallet.
public fun beneficiary<T>(self: &VestingWallet<T>): address {
    self.beneficiary
}

/// Get the amount already claimed.
public fun claimed<T>(self: &VestingWallet<T>): u64 {
    self.claimed
}

/// Get the total original amount (claimed + remaining).
public fun total_amount<T>(self: &VestingWallet<T>): u64 {
    self.balance.value() + self.claimed
}