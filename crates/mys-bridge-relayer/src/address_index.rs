// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

/// In-memory map from deposit EVM address -> Mys user address.
///
/// This is the core scaling property: matching is O(1) and does not grow RPC usage with users.
#[derive(Debug, Default, Clone)]
pub struct AddressIndex {
    pub chain_name: String,
    pub last_loaded_id: i64,
    pub by_evm: HashMap<[u8; 20], [u8; 32]>,
}

impl AddressIndex {
    pub fn new(chain_name: String) -> Self {
        Self {
            chain_name,
            last_loaded_id: 0,
            by_evm: HashMap::new(),
        }
    }

    pub fn apply_row(&mut self, id: i64, evm_address: [u8; 20], mys_address: [u8; 32]) {
        self.by_evm.insert(evm_address, mys_address);
        if id > self.last_loaded_id {
            self.last_loaded_id = id;
        }
    }

    pub fn lookup(&self, evm_address: &[u8; 20]) -> Option<&[u8; 32]> {
        self.by_evm.get(evm_address)
    }
}
