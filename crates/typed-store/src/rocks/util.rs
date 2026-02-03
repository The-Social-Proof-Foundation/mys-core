// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use rocksdb::{compaction_filter::Decision, MergeOperands};

/// An empty compaction filter that always keeps all entries.
/// Used as a placeholder when no compaction filtering is needed.
pub fn empty_compaction_filter(_level: u32, _key: &[u8], _value: &[u8]) -> Decision {
    Decision::Keep
}

/// A merge operator for reference counting.
/// Merges operands by summing them as u64 values.
/// This is used for reference counting where values are u64 counts.
pub fn reference_count_merge_operator(
    _key: &[u8],
    existing_value: Option<&[u8]>,
    operands: &MergeOperands,
) -> Option<Vec<u8>> {
    let mut sum: u64 = 0;
    
    // Add existing value if present
    if let Some(existing) = existing_value {
        if existing.len() == 8 {
            sum += u64::from_be_bytes([
                existing[0], existing[1], existing[2], existing[3],
                existing[4], existing[5], existing[6], existing[7],
            ]);
        }
    }
    
    // Add all operands
    for operand in operands {
        if operand.len() == 8 {
            sum += u64::from_be_bytes([
                operand[0], operand[1], operand[2], operand[3],
                operand[4], operand[5], operand[6], operand[7],
            ]);
        }
    }
    
    Some(sum.to_be_bytes().to_vec())
}

/// Check if a value is a reference count value (8 bytes, can be interpreted as u64).
pub fn is_ref_count_value(value: &[u8]) -> bool {
    value.len() == 8
}
