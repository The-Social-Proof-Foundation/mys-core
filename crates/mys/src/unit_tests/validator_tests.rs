// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::validator_commands::{
    get_validator_summary, MysValidatorCommand, MysValidatorCommandResponse,
};
use anyhow::Ok;
use fastcrypto::encoding::{Base64, Encoding};
use shared_crypto::intent::{Intent, IntentMessage};
use mys_types::crypto::MysKeyPair;
use mys_types::transaction::TransactionData;
use mys_types::{base_types::MysAddress, crypto::Signature, transaction::Transaction};
use test_cluster::TestClusterBuilder;

#[tokio::test]
async fn test_print_raw_rgp_txn() -> Result<(), anyhow::Error> {
    let test_cluster = TestClusterBuilder::new().build().await;
    let keypair: &MysKeyPair = test_cluster
        .swarm
        .config()
        .validator_configs
        .first()
        .unwrap()
        .account_key_pair
        .keypair();
    let validator_address: MysAddress = MysAddress::from(&keypair.public());
    let mut context = test_cluster.wallet;
    let mys_client = context.get_client().await?;
    let (_, summary) = get_validator_summary(&mys_client, validator_address)
        .await?
        .unwrap();
    let operation_cap_id = summary.operation_cap_id;

    // Execute the command and get the serialized transaction data.
    let response = MysValidatorCommand::DisplayGasPriceUpdateRawTxn {
        sender_address: validator_address,
        new_gas_price: 42,
        operation_cap_id,
        gas_budget: None,
    }
    .execute(&mut context)
    .await?;
    let MysValidatorCommandResponse::DisplayGasPriceUpdateRawTxn {
        data,
        serialized_data,
    } = response
    else {
        panic!("Expected DisplayGasPriceUpdateRawTxn");
    };

    // Construct the signed transaction and execute it.
    let deserialized_data =
        bcs::from_bytes::<TransactionData>(&Base64::decode(&serialized_data).unwrap())?;
    let signature = Signature::new_secure(
        &IntentMessage::new(Intent::mys_transaction(), deserialized_data),
        keypair,
    );
    let txn = Transaction::from_data(data, vec![signature]);
    context.execute_transaction_must_succeed(txn).await;
    let (_, summary) = get_validator_summary(&mys_client, validator_address)
        .await?
        .unwrap();

    // Check that the gas price is updated correctly.
    assert_eq!(summary.next_epoch_gas_price, 42);
    Ok(())
}
