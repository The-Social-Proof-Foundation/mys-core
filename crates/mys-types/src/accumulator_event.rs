// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::accumulator_root::AccumulatorObjId;
use crate::balance::Balance;
use crate::base_types::MysAddress;
use crate::effects::{
    AccumulatorAddress, AccumulatorOperation, AccumulatorValue, AccumulatorWriteV1,
};
use crate::error::MysError;
use crate::gas_coin::GAS;
use crate::TypeTag;

#[derive(Debug, Clone)]
pub struct AccumulatorEvent {
    pub accumulator_obj: AccumulatorObjId,
    pub write: AccumulatorWriteV1,
}

impl AccumulatorEvent {
    pub fn new(accumulator_obj: AccumulatorObjId, write: AccumulatorWriteV1) -> Self {
        Self {
            accumulator_obj,
            write,
        }
    }

    pub fn from_balance_change(
        address: MysAddress,
        balance_type: TypeTag,
        net_change: i64,
    ) -> Result<Self, MysError> {
        if !Balance::is_balance_type(&balance_type) {
            return Err(MysError::TypeError {
                error: "only Balance<T> is supported".to_string(),
            });
        }
        // Note: In a full implementation, we would compute the accumulator_obj here
        // For now, we'll use a placeholder. This may need to be implemented based on
        // how accumulator objects are derived in the MYS system.
        let accumulator_obj = AccumulatorObjId::new_unchecked(crate::base_types::ObjectID::random());

        let accumulator_address = AccumulatorAddress::new(address, balance_type);

        let (operation, amount) = if net_change > 0 {
            (AccumulatorOperation::Split, net_change as u64)
        } else {
            (AccumulatorOperation::Merge, (-net_change) as u64)
        };

        let accumulator_write = AccumulatorWriteV1 {
            address: accumulator_address,
            operation,
            value: AccumulatorValue::Integer(amount),
        };

        Ok(Self::new(accumulator_obj, accumulator_write))
    }

    pub fn total_mys_in_event(&self) -> (u64 /* input */, u64 /* output */) {
        let Self {
            write:
                AccumulatorWriteV1 {
                    address: AccumulatorAddress { ty, .. },
                    operation,
                    value,
                },
            ..
        } = self;

        let mys = if GAS::is_gas_type(ty) {
            match value {
                AccumulatorValue::Integer(v) => *v,
                _ => 0,
            }
        } else {
            0
        };

        match operation {
            AccumulatorOperation::Split => (mys, 0),
            AccumulatorOperation::Merge => (0, mys),
        }
    }
}
