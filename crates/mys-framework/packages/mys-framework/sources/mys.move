// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<MYS> is the token used to pay for gas in Mys.
/// It has 9 decimals, and the smallest unit (10^-9) is called "mist".
module mys::mys {
    use std::option;
    use mys::balance::{Self, Balance};
    use mys::coin::{Self, Coin, TreasuryCap};
    use mys::object::{Self, UID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::url::{Self, Url};

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    #[allow(unused_const)]
    /// The amount of Mist per Mys token based on the fact that mist is
    /// 10^-9 of a Mys token
    const MIST_PER_MYS: u64 = 1_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Mys denominated in whole Mys tokens (10 Billion)
    const TOTAL_SUPPLY_MYS: u64 = 10_000_000_000;

    /// The total supply of Mys denominated in Mist (10 Billion * 10^9)
    const TOTAL_SUPPLY_MIST: u64 = 10_000_000_000_000_000_000;

    /// Name of the coin
    public struct MYS has drop {}

    #[allow(unused_function)]
    /// Register the `MYS` Coin to acquire its `Supply`.
    /// This should be called only once during genesis creation.
    fun new(witness: MYS, ctx: &mut TxContext): Balance<MYS> {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        assert!(tx_context::epoch(ctx) == 0, EAlreadyMinted);

        let (treasury, metadata) = coin::create_currency_internal(
            witness,
            9,
            b"MySo",
            b"MySocial",
            // TODO: add appropriate description and logo url
            b"",
            option::none(),
            ctx,
        );
        
        transfer::public_freeze_object(metadata);
        let mut supply = coin::treasury_into_supply(treasury);
        let total_mys = balance::increase_supply(&mut supply, TOTAL_SUPPLY_MIST);
        balance::destroy_supply(supply);
        total_mys
    }

    public entry fun transfer(c: Coin<MYS>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }
}