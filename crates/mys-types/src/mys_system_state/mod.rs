// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::ObjectID;
use crate::committee::CommitteeWithNetworkMetadata;
use crate::dynamic_field::{
    get_dynamic_field_from_store, get_dynamic_field_object_from_store, Field,
};
use crate::error::MysError;
use crate::object::{MoveObject, Object};
use crate::storage::ObjectStore;
use crate::mys_system_state::epoch_start_mys_system_state::EpochStartSystemState;
use crate::mys_system_state::mys_system_state_inner_v2::MysSystemStateInnerV2;
use crate::versioned::Versioned;
use crate::{id::UID, MoveTypeTagTrait, MYS_SYSTEM_ADDRESS, MYS_SYSTEM_STATE_OBJECT_ID};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use mys_protocol_config::{ProtocolConfig, ProtocolVersion};

use self::mys_system_state_inner_v1::{MysSystemStateInnerV1, ValidatorV1};
use self::mys_system_state_summary::{MysSystemStateSummary, MysValidatorSummary};

pub mod epoch_start_mys_system_state;
pub mod mys_system_state_inner_v1;
pub mod mys_system_state_inner_v2;
pub mod mys_system_state_summary;

#[cfg(msim)]
mod simtest_mys_system_state_inner;
#[cfg(msim)]
use self::simtest_mys_system_state_inner::{
    SimTestMysSystemStateInnerDeepV2, SimTestMysSystemStateInnerShallowV2,
    SimTestMysSystemStateInnerV1, SimTestValidatorDeepV2, SimTestValidatorV1,
};

const MYS_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("MysSystemState");

pub const MYS_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("mys_system");
pub const ADVANCE_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch");
pub const ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch_safe_mode");

#[cfg(msim)]
pub const MYS_SYSTEM_STATE_SIM_TEST_V1: u64 = 18446744073709551605; // u64::MAX - 10
#[cfg(msim)]
pub const MYS_SYSTEM_STATE_SIM_TEST_SHALLOW_V2: u64 = 18446744073709551606; // u64::MAX - 9
#[cfg(msim)]
pub const MYS_SYSTEM_STATE_SIM_TEST_DEEP_V2: u64 = 18446744073709551607; // u64::MAX - 8

/// Rust version of the Move mys::mys_system::MysSystemState type
/// This repreents the object with 0x5 ID.
/// In Rust, this type should be rarely used since it's just a thin
/// wrapper used to access the inner object.
/// Within this module, we use it to determine the current version of the system state inner object type,
/// so that we could deserialize the inner object correctly.
/// Outside of this module, we only use it in genesis snapshot and testing.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MysSystemStateWrapper {
    pub id: UID,
    pub version: u64,
}

impl MysSystemStateWrapper {
    pub fn type_() -> StructTag {
        StructTag {
            address: MYS_SYSTEM_ADDRESS,
            name: MYS_SYSTEM_STATE_WRAPPER_STRUCT_NAME.to_owned(),
            module: MYS_SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Advances epoch in safe mode natively in Rust, without involking Move.
    /// This ensures that there cannot be any failure from Move and is guaranteed to succeed.
    /// Returns the old and new inner system state object.
    pub fn advance_epoch_safe_mode(
        &self,
        params: &AdvanceEpochParams,
        object_store: &dyn ObjectStore,
        protocol_config: &ProtocolConfig,
    ) -> (Object, Object) {
        let id = self.id.id.bytes;
        let old_field_object = get_dynamic_field_object_from_store(object_store, id, &self.version)
            .expect("Dynamic field object of wrapper should always be present in the object store");
        let mut new_field_object = old_field_object.clone();
        let move_object = new_field_object
            .data
            .try_as_move_mut()
            .expect("Dynamic field object must be a Move object");
        match self.version {
            1 => {
                Self::advance_epoch_safe_mode_impl::<MysSystemStateInnerV1>(
                    move_object,
                    params,
                    protocol_config,
                );
            }
            2 => {
                Self::advance_epoch_safe_mode_impl::<MysSystemStateInnerV2>(
                    move_object,
                    params,
                    protocol_config,
                );
            }
            #[cfg(msim)]
            MYS_SYSTEM_STATE_SIM_TEST_V1 => {
                Self::advance_epoch_safe_mode_impl::<SimTestMysSystemStateInnerV1>(
                    move_object,
                    params,
                    protocol_config,
                );
            }
            #[cfg(msim)]
            MYS_SYSTEM_STATE_SIM_TEST_SHALLOW_V2 => {
                Self::advance_epoch_safe_mode_impl::<SimTestMysSystemStateInnerShallowV2>(
                    move_object,
                    params,
                    protocol_config,
                );
            }
            #[cfg(msim)]
            MYS_SYSTEM_STATE_SIM_TEST_DEEP_V2 => {
                Self::advance_epoch_safe_mode_impl::<SimTestMysSystemStateInnerDeepV2>(
                    move_object,
                    params,
                    protocol_config,
                );
            }
            _ => unreachable!(),
        }
        (old_field_object, new_field_object)
    }

    fn advance_epoch_safe_mode_impl<T>(
        move_object: &mut MoveObject,
        params: &AdvanceEpochParams,
        protocol_config: &ProtocolConfig,
    ) where
        T: Serialize + DeserializeOwned + MysSystemStateTrait,
    {
        let mut field: Field<u64, T> =
            bcs::from_bytes(move_object.contents()).expect("bcs deserialization should never fail");
        tracing::info!(
            "Advance epoch safe mode: current epoch: {}, protocol_version: {}, system_state_version: {}",
            field.value.epoch(),
            field.value.protocol_version(),
            field.value.system_state_version()
        );
        field.value.advance_epoch_safe_mode(params);
        tracing::info!(
            "Safe mode activated. New epoch: {}, protocol_version: {}, system_state_version: {}",
            field.value.epoch(),
            field.value.protocol_version(),
            field.value.system_state_version()
        );
        let new_contents = bcs::to_bytes(&field).expect("bcs serialization should never fail");
        move_object
            .update_contents(new_contents, protocol_config)
            .expect("Update mys system object content cannot fail since it should be small");
    }
}

/// This is the standard API that all inner system state object type should implement.
#[enum_dispatch]
pub trait MysSystemStateTrait {
    fn epoch(&self) -> u64;
    fn reference_gas_price(&self) -> u64;
    fn protocol_version(&self) -> u64;
    fn system_state_version(&self) -> u64;
    fn epoch_start_timestamp_ms(&self) -> u64;
    fn epoch_duration_ms(&self) -> u64;
    fn safe_mode(&self) -> bool;
    fn advance_epoch_safe_mode(&mut self, params: &AdvanceEpochParams);
    fn get_current_epoch_committee(&self) -> CommitteeWithNetworkMetadata;
    fn get_pending_active_validators<S: ObjectStore + ?Sized>(
        &self,
        object_store: &S,
    ) -> Result<Vec<MysValidatorSummary>, MysError>;
    fn into_epoch_start_state(self) -> EpochStartSystemState;
    fn into_mys_system_state_summary(self) -> MysSystemStateSummary;
}

/// MysSystemState provides an abstraction over multiple versions of the inner MysSystemStateInner object.
/// This should be the primary interface to the system state object in Rust.
/// We use enum dispatch to dispatch all methods defined in MysSystemStateTrait to the actual
/// implementation in the inner types.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[enum_dispatch(MysSystemStateTrait)]
pub enum MysSystemState {
    V1(MysSystemStateInnerV1),
    V2(MysSystemStateInnerV2),
    #[cfg(msim)]
    SimTestV1(SimTestMysSystemStateInnerV1),
    #[cfg(msim)]
    SimTestShallowV2(SimTestMysSystemStateInnerShallowV2),
    #[cfg(msim)]
    SimTestDeepV2(SimTestMysSystemStateInnerDeepV2),
}

/// This is the fixed type used by genesis.
pub type MysSystemStateInnerGenesis = MysSystemStateInnerV1;
pub type MysValidatorGenesis = ValidatorV1;

impl MysSystemState {
    /// Always return the version that we will be using for genesis.
    /// Genesis always uses this version regardless of the current version.
    /// Note that since it's possible for the actual genesis of the network to diverge from the
    /// genesis of the latest Rust code, it's important that we only use this for tooling purposes.
    pub fn into_genesis_version_for_tooling(self) -> MysSystemStateInnerGenesis {
        match self {
            MysSystemState::V1(inner) => inner,
            _ => unreachable!(),
        }
    }

    pub fn version(&self) -> u64 {
        self.system_state_version()
    }
}

pub fn get_mys_system_state_wrapper(
    object_store: &dyn ObjectStore,
) -> Result<MysSystemStateWrapper, MysError> {
    let wrapper = object_store
        .get_object(&MYS_SYSTEM_STATE_OBJECT_ID)
        // Don't panic here on None because object_store is a generic store.
        .ok_or_else(|| {
            MysError::MysSystemStateReadError("MysSystemStateWrapper object not found".to_owned())
        })?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        MysError::MysSystemStateReadError(
            "MysSystemStateWrapper object must be a Move object".to_owned(),
        )
    })?;
    let result = bcs::from_bytes::<MysSystemStateWrapper>(move_object.contents())
        .map_err(|err| MysError::MysSystemStateReadError(err.to_string()))?;
    Ok(result)
}

pub fn get_mys_system_state(object_store: &dyn ObjectStore) -> Result<MysSystemState, MysError> {
    let wrapper = get_mys_system_state_wrapper(object_store)?;
    let id = wrapper.id.id.bytes;
    match wrapper.version {
        1 => {
            let result: MysSystemStateInnerV1 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        MysError::DynamicFieldReadError(format!(
                            "Failed to load mys system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(MysSystemState::V1(result))
        }
        2 => {
            let result: MysSystemStateInnerV2 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        MysError::DynamicFieldReadError(format!(
                            "Failed to load mys system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(MysSystemState::V2(result))
        }
        #[cfg(msim)]
        MYS_SYSTEM_STATE_SIM_TEST_V1 => {
            let result: SimTestMysSystemStateInnerV1 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        MysError::DynamicFieldReadError(format!(
                            "Failed to load mys system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(MysSystemState::SimTestV1(result))
        }
        #[cfg(msim)]
        MYS_SYSTEM_STATE_SIM_TEST_SHALLOW_V2 => {
            let result: SimTestMysSystemStateInnerShallowV2 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        MysError::DynamicFieldReadError(format!(
                            "Failed to load mys system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(MysSystemState::SimTestShallowV2(result))
        }
        #[cfg(msim)]
        MYS_SYSTEM_STATE_SIM_TEST_DEEP_V2 => {
            let result: SimTestMysSystemStateInnerDeepV2 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        MysError::DynamicFieldReadError(format!(
                            "Failed to load mys system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(MysSystemState::SimTestDeepV2(result))
        }
        _ => Err(MysError::MysSystemStateReadError(format!(
            "Unsupported MysSystemState version: {}",
            wrapper.version
        ))),
    }
}

/// Given a system state type version, and the ID of the table, along with a key, retrieve the
/// dynamic field as a Validator type. We need the version to determine which inner type to use for
/// the Validator type. This is assuming that the validator is stored in the table as
/// ValidatorWrapper type.
pub fn get_validator_from_table<K>(
    object_store: &dyn ObjectStore,
    table_id: ObjectID,
    key: &K,
) -> Result<MysValidatorSummary, MysError>
where
    K: MoveTypeTagTrait + Serialize + DeserializeOwned + fmt::Debug,
{
    let field: ValidatorWrapper = get_dynamic_field_from_store(object_store, table_id, key)
        .map_err(|err| {
            MysError::MysSystemStateReadError(format!(
                "Failed to load validator wrapper from table: {:?}",
                err
            ))
        })?;
    let versioned = field.inner;
    let version = versioned.version;
    match version {
        1 => {
            let validator: ValidatorV1 =
                get_dynamic_field_from_store(object_store, versioned.id.id.bytes, &version)
                    .map_err(|err| {
                        MysError::MysSystemStateReadError(format!(
                            "Failed to load inner validator from the wrapper: {:?}",
                            err
                        ))
                    })?;
            Ok(validator.into_mys_validator_summary())
        }
        #[cfg(msim)]
        MYS_SYSTEM_STATE_SIM_TEST_V1 => {
            let validator: SimTestValidatorV1 =
                get_dynamic_field_from_store(object_store, versioned.id.id.bytes, &version)
                    .map_err(|err| {
                        MysError::MysSystemStateReadError(format!(
                            "Failed to load inner validator from the wrapper: {:?}",
                            err
                        ))
                    })?;
            Ok(validator.into_mys_validator_summary())
        }
        #[cfg(msim)]
        MYS_SYSTEM_STATE_SIM_TEST_DEEP_V2 => {
            let validator: SimTestValidatorDeepV2 =
                get_dynamic_field_from_store(object_store, versioned.id.id.bytes, &version)
                    .map_err(|err| {
                        MysError::MysSystemStateReadError(format!(
                            "Failed to load inner validator from the wrapper: {:?}",
                            err
                        ))
                    })?;
            Ok(validator.into_mys_validator_summary())
        }
        _ => Err(MysError::MysSystemStateReadError(format!(
            "Unsupported Validator version: {}",
            version
        ))),
    }
}

pub fn get_validators_from_table_vec<S, ValidatorType>(
    object_store: &S,
    table_id: ObjectID,
    table_size: u64,
) -> Result<Vec<ValidatorType>, MysError>
where
    S: ObjectStore + ?Sized,
    ValidatorType: Serialize + DeserializeOwned,
{
    let mut validators = vec![];
    for i in 0..table_size {
        let validator: ValidatorType = get_dynamic_field_from_store(&object_store, table_id, &i)
            .map_err(|err| {
                MysError::MysSystemStateReadError(format!(
                    "Failed to load validator from table: {:?}",
                    err
                ))
            })?;
        validators.push(validator);
    }
    Ok(validators)
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct PoolTokenExchangeRate {
    mys_amount: u64,
    pool_token_amount: u64,
}

impl PoolTokenExchangeRate {
    /// Rate of the staking pool, pool token amount : Mys amount
    pub fn rate(&self) -> f64 {
        if self.mys_amount == 0 {
            1_f64
        } else {
            self.pool_token_amount as f64 / self.mys_amount as f64
        }
    }

    pub fn new(mys_amount: u64, pool_token_amount: u64) -> Self {
        Self {
            mys_amount,
            pool_token_amount,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorWrapper {
    pub inner: Versioned,
}

#[derive(Debug)]
pub struct AdvanceEpochParams {
    pub epoch: u64,
    pub next_protocol_version: ProtocolVersion,
    pub storage_charge: u64,
    pub computation_charge: u64,
    pub storage_rebate: u64,
    pub non_refundable_storage_fee: u64,
    pub storage_fund_reinvest_rate: u64,
    pub reward_slashing_rate: u64,
    pub epoch_start_timestamp_ms: u64,
}

#[cfg(msim)]
pub mod advance_epoch_result_injection {
    use crate::{
        committee::EpochId,
        error::{ExecutionError, ExecutionErrorKind},
        execution::ResultWithTimings,
    };
    use std::cell::RefCell;

    thread_local! {
        /// Override the result of advance_epoch in the range [start, end).
        static OVERRIDE: RefCell<Option<(EpochId, EpochId)>>  = RefCell::new(None);
    }

    /// Override the result of advance_epoch transaction if new epoch is in the provided range [start, end).
    pub fn set_override(value: Option<(EpochId, EpochId)>) {
        OVERRIDE.with(|o| *o.borrow_mut() = value);
    }

    /// This function is used to modify the result of advance_epoch transaction for testing.
    /// If the override is set, the result will be an execution error, otherwise the original result will be returned.
    pub fn maybe_modify_result(
        result: ResultWithTimings<(), ExecutionError>,
        current_epoch: EpochId,
    ) -> ResultWithTimings<(), ExecutionError> {
        if let Some((start, end)) = OVERRIDE.with(|o| *o.borrow()) {
            if current_epoch >= start && current_epoch < end {
                return Err((
                    ExecutionError::new(ExecutionErrorKind::FunctionNotFound, None),
                    vec![],
                ));
            }
        }
        result
    }

    // For old execution versions that don't report timings
    pub fn maybe_modify_result_legacy(
        result: Result<(), ExecutionError>,
        current_epoch: EpochId,
    ) -> Result<(), ExecutionError> {
        if let Some((start, end)) = OVERRIDE.with(|o| *o.borrow()) {
            if current_epoch >= start && current_epoch < end {
                return Err(ExecutionError::new(
                    ExecutionErrorKind::FunctionNotFound,
                    None,
                ));
            }
        }
        result
    }
}
