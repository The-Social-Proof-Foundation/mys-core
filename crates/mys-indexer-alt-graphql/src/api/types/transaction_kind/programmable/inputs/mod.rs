// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Union;

use move_core_types::annotated_value::MoveTypeLayout;

use crate::api::scalars::base64::Base64;
use crate::api::types::move_type::MoveType;
use crate::api::types::move_value::MoveValue;
use crate::scope::Scope;

// balance_withdraw module disabled - FundsWithdrawalArg not yet available in mys-types
// pub mod balance_withdraw;
pub mod object;
pub mod pure;

// pub use balance_withdraw::BalanceWithdraw;
pub use object::OwnedOrImmutable;
pub use object::Receiving;
pub use object::SharedInput;
pub use pure::Pure;

/// Input argument to a Programmable Transaction Block (PTB) command.
#[derive(Union)]
pub enum TransactionInput {
    Pure(Pure),
    Value(MoveValue),
    OwnedOrImmutable(OwnedOrImmutable),
    SharedInput(SharedInput),
    Receiving(Receiving),
    // BalanceWithdraw(BalanceWithdraw), // Disabled - FundsWithdrawalArg not yet available
}

impl TransactionInput {
    pub fn from(
        input: mys_types::transaction::CallArg,
        layout: Option<MoveTypeLayout>,
        scope: Scope,
    ) -> Self {
        use mys_types::transaction::CallArg as CA;
        use mys_types::transaction::ObjectArg as OA;

        match (input, layout) {
            // If the layout for the pure arg can be inferred, then represent it as a MoveValue.
            (CA::Pure(native), Some(layout)) => Self::Value(MoveValue {
                type_: MoveType::from_layout(layout, scope),
                native,
            }),

            (CA::Pure(bytes), None) => Self::Pure(Pure {
                bytes: Some(Base64::from(bytes)),
            }),

            (CA::Object(OA::ImmOrOwnedObject((id, version, digest))), _) => Self::OwnedOrImmutable(
                OwnedOrImmutable::from_object_ref(id, version, digest, scope),
            ),

            (
                CA::Object(OA::SharedObject {
                    id,
                    initial_shared_version,
                    mutability,
                }),
                _,
            ) => Self::SharedInput(SharedInput::from_shared_object(
                id,
                initial_shared_version,
                mutability.is_exclusive(),
            )),

            (CA::Object(OA::Receiving((id, version, digest))), _) => {
                Self::Receiving(Receiving::from_object_ref(id, version, digest, scope))
            }

            // FundsWithdrawal not yet available in mys-types CallArg
            // (CA::FundsWithdrawal(withdrawal), _) => {
            //     Self::BalanceWithdraw(BalanceWithdraw::from_native(withdrawal, scope))
            // }
        }
    }
}
