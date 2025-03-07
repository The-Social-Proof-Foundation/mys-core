// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! A tool to semi automate fire drills. It still requires some manual work today. For example,
//! 1. update iptables for new tpc/udp ports
//! 2. restart the node in a new epoch when config file will be reloaded and take effects
//!
//! Example usage:
//! mys fire-drill metadata-rotation \
//! --mys-node-config-path validator.yaml \
//! --account-key-path account.key \
//! --fullnode-rpc-url http://fullnode-my-local-net:9000

use anyhow::bail;
use clap::*;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::{KeyPair, ToFromBytes};
use move_core_types::ident_str;
use std::path::{Path, PathBuf};
use mys_config::node::{AuthorityKeyPairWithPath, KeyPairWithPath};
use mys_config::{local_ip_utils, Config, NodeConfig, PersistedConfig};
use mys_json_rpc_types::{MysExecutionStatus, MysTransactionBlockResponseOptions};
use mys_keys::keypair_file::read_keypair_from_file;
use mys_sdk::{rpc_types::MysTransactionBlockEffectsAPI, MysClient, MysClientBuilder};
use mys_types::base_types::{ObjectRef, MysAddress};
use mys_types::crypto::{generate_proof_of_possession, get_key_pair, MysKeyPair};
use mys_types::multiaddr::{Multiaddr, Protocol};
use mys_types::transaction::{
    CallArg, Transaction, TransactionData, TEST_ONLY_GAS_UNIT_FOR_GENERIC,
};
use mys_types::{committee::EpochId, crypto::get_authority_key_pair, MYS_SYSTEM_PACKAGE_ID};
use tracing::info;

#[derive(Parser)]
pub enum FireDrill {
    MetadataRotation(MetadataRotation),
}

#[derive(Parser)]
pub struct MetadataRotation {
    /// Path to mys node config.
    #[clap(long = "mys-node-config-path")]
    mys_node_config_path: PathBuf,
    /// Path to account key file.
    #[clap(long = "account-key-path")]
    account_key_path: PathBuf,
    /// Jsonrpc url for a reliable fullnode.
    #[clap(long = "fullnode-rpc-url")]
    fullnode_rpc_url: String,
}

pub async fn run_fire_drill(fire_drill: FireDrill) -> anyhow::Result<()> {
    match fire_drill {
        FireDrill::MetadataRotation(metadata_rotation) => {
            run_metadata_rotation(metadata_rotation).await?;
        }
    }
    Ok(())
}

async fn run_metadata_rotation(metadata_rotation: MetadataRotation) -> anyhow::Result<()> {
    let MetadataRotation {
        mys_node_config_path,
        account_key_path,
        fullnode_rpc_url,
    } = metadata_rotation;
    let account_key = read_keypair_from_file(&account_key_path)?;
    let config: NodeConfig = PersistedConfig::read(&mys_node_config_path).map_err(|err| {
        err.context(format!(
            "Cannot open Mys Node Config file at {:?}",
            mys_node_config_path
        ))
    })?;

    let mys_client = MysClientBuilder::default().build(fullnode_rpc_url).await?;
    let mys_address = MysAddress::from(&account_key.public());
    let starting_epoch = current_epoch(&mys_client).await?;
    info!("Running Metadata Rotation fire drill for validator address {mys_address} in epoch {starting_epoch}.");

    // Prepare new metadata for next epoch
    let new_config_path =
        update_next_epoch_metadata(&mys_node_config_path, &config, &mys_client, &account_key)
            .await?;

    let current_epoch = current_epoch(&mys_client).await?;
    if current_epoch > starting_epoch {
        bail!("Epoch already advanced to {current_epoch}");
    }
    let target_epoch = starting_epoch + 1;
    wait_for_next_epoch(&mys_client, target_epoch).await?;
    info!("Just advanced to epoch {target_epoch}");

    // Replace new config
    std::fs::rename(new_config_path, mys_node_config_path)?;
    info!("Updated Mys Node config.");

    Ok(())
}

// TODO move this to a shared lib
pub async fn get_gas_obj_ref(
    mys_address: MysAddress,
    mys_client: &MysClient,
    minimal_gas_balance: u64,
) -> anyhow::Result<ObjectRef> {
    let coins = mys_client
        .coin_read_api()
        .get_coins(mys_address, Some("0x2::mys::MYS".into()), None, None)
        .await?
        .data;
    let gas_obj = coins.iter().find(|c| c.balance >= minimal_gas_balance);
    if gas_obj.is_none() {
        bail!("Validator doesn't have enough Mys coins to cover transaction fees.");
    }
    Ok(gas_obj.unwrap().object_ref())
}

async fn update_next_epoch_metadata(
    mys_node_config_path: &Path,
    config: &NodeConfig,
    mys_client: &MysClient,
    account_key: &MysKeyPair,
) -> anyhow::Result<PathBuf> {
    // Save backup config just in case
    let mut backup_config_path = mys_node_config_path.to_path_buf();
    backup_config_path.pop();
    backup_config_path.push("node_config_backup.yaml");
    let backup_config = config.clone();
    backup_config.persisted(&backup_config_path).save()?;

    let mys_address = MysAddress::from(&account_key.public());

    let mut new_config = config.clone();

    // protocol key
    let new_protocol_key_pair = get_authority_key_pair().1;
    let new_protocol_key_pair_copy = new_protocol_key_pair.copy();
    let pop = generate_proof_of_possession(&new_protocol_key_pair, mys_address);
    new_config.protocol_key_pair = AuthorityKeyPairWithPath::new(new_protocol_key_pair);

    // network key
    let new_network_key_pair: Ed25519KeyPair = get_key_pair().1;
    let new_network_key_pair_copy = new_network_key_pair.copy();
    new_config.network_key_pair = KeyPairWithPath::new(MysKeyPair::Ed25519(new_network_key_pair));

    // worker key
    let new_worker_key_pair: Ed25519KeyPair = get_key_pair().1;
    let new_worker_key_pair_copy = new_worker_key_pair.copy();
    new_config.worker_key_pair = KeyPairWithPath::new(MysKeyPair::Ed25519(new_worker_key_pair));

    let validators = mys_client
        .governance_api()
        .get_latest_mys_system_state()
        .await?
        .active_validators;
    let self_validator = validators
        .iter()
        .find(|v| v.mys_address == mys_address)
        .unwrap();

    // Network address
    let mut new_network_address = Multiaddr::try_from(self_validator.net_address.clone()).unwrap();
    info!("Current network address: {:?}", new_network_address);
    let http = new_network_address.pop().unwrap();
    // pop out tcp
    new_network_address.pop().unwrap();
    let localhost = local_ip_utils::localhost_for_testing();
    let new_port = local_ip_utils::get_available_port(&localhost);
    new_network_address.push(Protocol::Tcp(new_port));
    new_network_address.push(http);
    info!("New network address: {:?}", new_network_address);
    new_config.network_address = new_network_address.clone();

    // p2p address
    let mut new_external_address = config.p2p_config.external_address.clone().unwrap();
    info!("Current P2P external address: {:?}", new_external_address);
    // pop out udp
    new_external_address.pop().unwrap();
    let new_port = local_ip_utils::get_available_port(&localhost);
    new_external_address.push(Protocol::Udp(new_port));
    info!("New P2P external address: {:?}", new_external_address);
    new_config.p2p_config.external_address = Some(new_external_address.clone());

    let mut new_listen_address = config.p2p_config.listen_address;
    info!("Current P2P local listen address: {:?}", new_listen_address);
    new_listen_address.set_port(new_port);
    info!("New P2P local listen address: {:?}", new_listen_address);
    new_config.p2p_config.listen_address = new_listen_address;

    // primary address
    let mut new_primary_addresses =
        Multiaddr::try_from(self_validator.primary_address.clone()).unwrap();
    info!("Current primary address: {:?}", new_primary_addresses);
    // pop out udp
    new_primary_addresses.pop().unwrap();
    let new_port = local_ip_utils::get_available_port(&localhost);
    new_primary_addresses.push(Protocol::Udp(new_port));
    info!("New primary address: {:?}", new_primary_addresses);

    // worker address
    let mut new_worker_addresses = Multiaddr::try_from(
        validators
            .iter()
            .find(|v| v.mys_address == mys_address)
            .unwrap()
            .worker_address
            .clone(),
    )
    .unwrap();
    info!("Current worker address: {:?}", new_worker_addresses);
    // pop out udp
    new_worker_addresses.pop().unwrap();
    let new_port = local_ip_utils::get_available_port(&localhost);
    new_worker_addresses.push(Protocol::Udp(new_port));
    info!("New worker address:: {:?}", new_worker_addresses);

    // Save new config
    let mut new_config_path = mys_node_config_path.to_path_buf();
    new_config_path.pop();
    new_config_path.push(
        String::from(mys_node_config_path.file_name().unwrap().to_str().unwrap()) + ".next_epoch",
    );
    new_config.persisted(&new_config_path).save()?;

    // update protocol pubkey on chain
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_protocol_pubkey",
        vec![
            CallArg::Pure(
                bcs::to_bytes(&new_protocol_key_pair_copy.public().as_bytes().to_vec()).unwrap(),
            ),
            CallArg::Pure(bcs::to_bytes(&pop.as_bytes().to_vec()).unwrap()),
        ],
        mys_client,
    )
    .await?;

    // update network pubkey on chain
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_network_pubkey",
        vec![CallArg::Pure(
            bcs::to_bytes(&new_network_key_pair_copy.public().as_bytes().to_vec()).unwrap(),
        )],
        mys_client,
    )
    .await?;

    // update worker pubkey on chain
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_worker_pubkey",
        vec![CallArg::Pure(
            bcs::to_bytes(&new_worker_key_pair_copy.public().as_bytes().to_vec()).unwrap(),
        )],
        mys_client,
    )
    .await?;

    // update network address
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_network_address",
        vec![CallArg::Pure(bcs::to_bytes(&new_network_address).unwrap())],
        mys_client,
    )
    .await?;

    // update p2p address
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_p2p_address",
        vec![CallArg::Pure(bcs::to_bytes(&new_external_address).unwrap())],
        mys_client,
    )
    .await?;

    // update primary address
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_primary_address",
        vec![CallArg::Pure(
            bcs::to_bytes(&new_primary_addresses).unwrap(),
        )],
        mys_client,
    )
    .await?;

    // update worker address
    update_metadata_on_chain(
        account_key,
        "update_validator_next_epoch_worker_address",
        vec![CallArg::Pure(bcs::to_bytes(&new_worker_addresses).unwrap())],
        mys_client,
    )
    .await?;

    Ok(new_config_path)
}

async fn update_metadata_on_chain(
    account_key: &MysKeyPair,
    function: &'static str,
    call_args: Vec<CallArg>,
    mys_client: &MysClient,
) -> anyhow::Result<()> {
    let mys_address = MysAddress::from(&account_key.public());
    let gas_obj_ref = get_gas_obj_ref(mys_address, mys_client, 10000 * 100).await?;
    let rgp = mys_client
        .governance_api()
        .get_reference_gas_price()
        .await?;
    let mut args = vec![CallArg::MYS_SYSTEM_MUT];
    args.extend(call_args);
    let tx_data = TransactionData::new_move_call(
        mys_address,
        MYS_SYSTEM_PACKAGE_ID,
        ident_str!("mys_system").to_owned(),
        ident_str!(function).to_owned(),
        vec![],
        gas_obj_ref,
        args,
        rgp * TEST_ONLY_GAS_UNIT_FOR_GENERIC,
        rgp,
    )
    .unwrap();
    execute_tx(account_key, mys_client, tx_data, function).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    Ok(())
}

async fn execute_tx(
    account_key: &MysKeyPair,
    mys_client: &MysClient,
    tx_data: TransactionData,
    action: &str,
) -> anyhow::Result<()> {
    let tx = Transaction::from_data_and_signer(tx_data, vec![account_key]);
    info!("Executing {:?}", tx.digest());
    let tx_digest = *tx.digest();
    let resp = mys_client
        .quorum_driver_api()
        .execute_transaction_block(
            tx,
            MysTransactionBlockResponseOptions::full_content(),
            Some(mys_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .unwrap();
    if *resp.effects.unwrap().status() != MysExecutionStatus::Success {
        anyhow::bail!("Tx to update metadata {:?} failed", tx_digest);
    }
    info!("{action} succeeded");
    Ok(())
}

async fn wait_for_next_epoch(mys_client: &MysClient, target_epoch: EpochId) -> anyhow::Result<()> {
    loop {
        let epoch_id = current_epoch(mys_client).await?;
        if epoch_id > target_epoch {
            bail!(
                "Current epoch ID {} is higher than target {}, likely something is off.",
                epoch_id,
                target_epoch
            );
        }
        if epoch_id == target_epoch {
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn current_epoch(mys_client: &MysClient) -> anyhow::Result<EpochId> {
    Ok(mys_client.read_api().get_committee_info(None).await?.epoch)
}
