// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module mys_system::mys_system {
    use std::vector;

    use mys::balance::Balance;
    use mys::object::UID;
    use mys::mys::MYS;
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::dynamic_field;

    use mys_system::validator::Validator;
    use mys_system::mys_system_state_inner::{Self, MysSystemStateInner, MysSystemStateInnerV2};

    public struct MysSystemState has key {
        id: UID,
        version: u64,
    }

    public(package) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<MYS>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = mys_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = mys_system_state_inner::genesis_system_state_version();
        let mut self = MysSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<MYS>,
        computation_reward: Balance<MYS>,
        wrapper: &mut MysSystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        _reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<MYS> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x0, 0);
        let storage_rebate = mys_system_state_inner::advance_epoch(
            self,
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            epoch_start_timestamp_ms,
        );

        storage_rebate
    }

    public fun active_validator_addresses(wrapper: &mut MysSystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut MysSystemState): &mut MysSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut MysSystemState): &mut MysSystemStateInnerV2 {
        let mut version = self.version;
        if (version == mys_system_state_inner::genesis_system_state_version()) {
            let inner: MysSystemStateInner = dynamic_field::remove(&mut self.id, version);
            let new_inner = mys_system_state_inner::v1_to_v2(inner);
            version = mys_system_state_inner::system_state_version(&new_inner);
            dynamic_field::add(&mut self.id, version, new_inner);
            self.version = version;
        };

        let inner: &mut MysSystemStateInnerV2 = dynamic_field::borrow_mut(&mut self.id, version);
        assert!(mys_system_state_inner::system_state_version(inner) == version, 0);
        inner
    }
}
