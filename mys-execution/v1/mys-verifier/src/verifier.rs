// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the public APIs supported by the bytecode verifier.

use move_binary_format::file_format::CompiledModule;
use mys_types::{error::ExecutionError, move_package::FnInfoMap};

use crate::{
    entry_points_verifier, global_storage_access_verifier, id_leak_verifier,
    one_time_witness_verifier, private_generics, struct_with_key_verifier,
};
use move_bytecode_verifier_meter::{dummy::DummyMeter, Meter};

/// Helper for a "canonical" verification of a module.
pub fn mys_verify_module_metered(
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut (impl Meter + ?Sized),
) -> Result<(), ExecutionError> {
    struct_with_key_verifier::verify_module(module)?;
    global_storage_access_verifier::verify_module(module)?;
    id_leak_verifier::verify_module(module, meter)?;
    private_generics::verify_module(module)?;
    entry_points_verifier::verify_module(module, fn_info_map)?;
    one_time_witness_verifier::verify_module(module, fn_info_map)
}

/// Runs the Mys verifier and checks if the error counts as a Mys verifier timeout
/// NOTE: this function only check if the verifier error is a timeout
/// All other errors are ignored
pub fn mys_verify_module_metered_check_timeout_only(
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut (impl Meter + ?Sized),
) -> Result<(), ExecutionError> {
    // Checks if the error counts as a Mys verifier timeout
    if let Err(error) = mys_verify_module_metered(module, fn_info_map, meter) {
        if matches!(
            error.kind(),
            mys_types::execution_status::ExecutionFailureStatus::MysMoveVerificationTimedout
        ) {
            return Err(error);
        }
    }
    // Any other scenario, including a non-timeout error counts as Ok
    Ok(())
}

pub fn mys_verify_module_unmetered(
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
) -> Result<(), ExecutionError> {
    mys_verify_module_metered(module, fn_info_map, &mut DummyMeter).inspect_err(|err| {
        // We must never see timeout error in execution
        debug_assert!(
            !matches!(
                err.kind(),
                mys_types::execution_status::ExecutionFailureStatus::MysMoveVerificationTimedout
            ),
            "Unexpected timeout error in execution"
        );
    })
}
