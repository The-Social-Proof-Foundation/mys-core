// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module mys_system::mys_system_state_inner {
    use mys::balance::{Self, Balance};
    use mys::mys::MYS;
    use mys::tx_context::TxContext;
    use mys::bag::{Self, Bag};
    use mys::table::{Self, Table};
    use mys::object::ID;

    use mys_system::validator::Validator;
    use mys_system::validator_wrapper::ValidatorWrapper;

    const SYSTEM_STATE_VERSION_V1: u64 = 18446744073709551605;  // u64::MAX - 10
    const SYSTEM_STATE_VERSION_V2: u64 = 18446744073709551606;  // u64::MAX - 9

    public struct SystemParameters has store {
        epoch_duration_ms: u64,
        extra_fields: Bag,
    }

    public struct ValidatorSet has store {
        active_validators: vector<Validator>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    public struct MysSystemStateInner has store {
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<MYS>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public struct MysSystemStateInnerV2 has store {
        new_dummy_field: u64,
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<MYS>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public(package) fun create(
        validators: vector<Validator>,
        storage_fund: Balance<MYS>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ): MysSystemStateInner {
        let validators = new_validator_set(validators, ctx);
        let system_state = MysSystemStateInner {
            epoch: 0,
            protocol_version,
            system_state_version: genesis_system_state_version(),
            validators,
            storage_fund,
            parameters: SystemParameters {
                epoch_duration_ms,
                extra_fields: bag::new(ctx),
            },
            reference_gas_price: 1,
            safe_mode: false,
            epoch_start_timestamp_ms,
            extra_fields: bag::new(ctx),
        };
        system_state
    }

    public(package) fun advance_epoch(
        self: &mut MysSystemStateInnerV2,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<MYS>,
        computation_reward: Balance<MYS>,
        storage_rebate_amount: u64,
        epoch_start_timestamp_ms: u64,
    ) : Balance<MYS> {
        self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;
        self.epoch = self.epoch + 1;
        assert!(new_epoch == self.epoch, 0);
        self.safe_mode = false;
        self.protocol_version = next_protocol_version;

        balance::join(&mut self.storage_fund, computation_reward);
        balance::join(&mut self.storage_fund, storage_reward);
        let storage_rebate = balance::split(&mut self.storage_fund, storage_rebate_amount);
        storage_rebate
    }

    public(package) fun protocol_version(self: &MysSystemStateInnerV2): u64 { self.protocol_version }
    public(package) fun system_state_version(self: &MysSystemStateInnerV2): u64 { self.system_state_version }
    public(package) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    fun new_validator_set(init_active_validators: vector<Validator>, ctx: &mut TxContext): ValidatorSet {
        ValidatorSet {
            active_validators: init_active_validators,
            inactive_validators: table::new(ctx),
            extra_fields: bag::new(ctx),
        }
    }

    public(package) fun v1_to_v2(v1: MysSystemStateInner): MysSystemStateInnerV2 {
        let MysSystemStateInner {
            epoch,
            protocol_version,
            system_state_version: old_system_state_version,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        } = v1;
        assert!(old_system_state_version == SYSTEM_STATE_VERSION_V1, 0);
        MysSystemStateInnerV2 {
            new_dummy_field: 100,
            epoch,
            protocol_version,
            system_state_version: SYSTEM_STATE_VERSION_V2,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        }
    }
}
