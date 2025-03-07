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
    use mys_system::mys_system_state_inner::MysSystemStateInner;
    use mys_system::mys_system_state_inner;

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
        _new_epoch: u64,
        _next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64,
        _reward_slashing_rate: u64,
        _epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) : Balance<MYS> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x1, 0); // aborts here
        mys_system_state_inner::advance_epoch(
            self,
            storage_reward,
            computation_reward,
            storage_rebate,
        )
    }

    public fun active_validator_addresses(wrapper: &mut MysSystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut MysSystemState): &mut MysSystemStateInner {
        let version = self.version;
        dynamic_field::borrow_mut(&mut self.id, version)
    }
}
