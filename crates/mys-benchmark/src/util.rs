// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::workloads::Gas;
use crate::ValidatorProxy;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use mys_keys::keystore::{AccountKeystore, FileBasedKeystore};
use mys_test_transaction_builder::TestTransactionBuilder;
use mys_types::base_types::ObjectRef;
use mys_types::crypto::{AccountKeyPair, KeypairTraits};
use mys_types::object::Owner;
use mys_types::transaction::{Transaction, TransactionData, TEST_ONLY_GAS_UNIT_FOR_TRANSFER};
use mys_types::utils::to_sender_signed_transaction;
use mys_types::{base_types::MysAddress, crypto::MysKeyPair};

// This is the maximum gas we will transfer from primary coin into any gas coin
// for running the benchmark

pub type UpdatedAndNewlyMintedGasCoins = Vec<Gas>;

pub fn get_ed25519_keypair_from_keystore(
    keystore_path: PathBuf,
    requested_address: &MysAddress,
) -> Result<AccountKeyPair> {
    let keystore = FileBasedKeystore::new(&keystore_path)?;
    match keystore.get_key(requested_address) {
        Ok(MysKeyPair::Ed25519(kp)) => Ok(kp.copy()),
        other => Err(anyhow::anyhow!("Invalid key type: {:?}", other)),
    }
}

pub fn make_pay_tx(
    input_coins: Vec<ObjectRef>,
    sender: MysAddress,
    addresses: Vec<MysAddress>,
    split_amounts: Vec<u64>,
    gas: ObjectRef,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> Result<Transaction> {
    let pay = TransactionData::new_pay(
        sender,
        input_coins,
        addresses,
        split_amounts,
        gas,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER * gas_price,
        gas_price,
    )?;
    Ok(to_sender_signed_transaction(pay, keypair))
}

pub async fn publish_basics_package(
    gas: ObjectRef,
    proxy: Arc<dyn ValidatorProxy + Sync + Send>,
    sender: MysAddress,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> ObjectRef {
    let transaction = TestTransactionBuilder::new(sender, gas, gas_price)
        .publish_examples("basics")
        .build_and_sign(keypair);
    let effects = proxy.execute_transaction_block(transaction).await.unwrap();
    effects
        .created()
        .iter()
        .find(|(_, owner)| matches!(owner, Owner::Immutable))
        .map(|(reference, _)| *reference)
        .unwrap()
}
