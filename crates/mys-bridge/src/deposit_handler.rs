// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit event handler - processes detected deposits and triggers bridging

use crate::deposit_bridge::{DepositBridgeHandler, handle_mys_deposit};
use crate::deposit_addresses::DepositAddressManager;
use crate::deposit_gas_manager::DepositGasManager;
use crate::deposit_monitor::{EvmDepositEvent, MysDepositEvent};
use crate::mys_client::{MysBridgeClient, MysClientInner};
use crate::storage::BridgeOrchestratorTables;
use arc_swap::ArcSwap;
use mys_types::{TypeTag, transaction::ObjectArg};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

/// Runs the deposit processing loop
/// Receives deposit events from monitors and triggers bridge execution
pub async fn run_deposit_processor<C>(
    mut evm_deposit_rx: tokio::sync::mpsc::UnboundedReceiver<EvmDepositEvent>,
    bridge_handler: Arc<DepositBridgeHandler<C>>,
) where
    C: MysClientInner + 'static,
{
    info!("Starting deposit processor");

    while let Some(deposit_event) = evm_deposit_rx.recv().await {
        info!(
            tx_hash = ?deposit_event.tx_hash,
            to_address = ?deposit_event.to_address,
            amount = ?deposit_event.amount,
            "Processing EVM deposit"
        );

        match bridge_handler.handle_evm_deposit(deposit_event.clone()).await {
            Ok(bridge_tx_hash) => {
                info!(
                    deposit_tx = ?deposit_event.tx_hash,
                    bridge_tx = ?bridge_tx_hash,
                    "Successfully bridged EVM deposit"
                );
            }
            Err(e) => {
                error!(
                    deposit_tx = ?deposit_event.tx_hash,
                    error = ?e,
                    "Failed to bridge EVM deposit"
                );
                // Don't panic - log and continue
                // User can manually intervene if needed
            }
        }
    }

    info!("Deposit processor shutting down");
}

/// Runs the MySocial deposit processing loop
/// Receives deposit events from MySocial monitor and triggers bridge execution
pub async fn run_mys_deposit_processor(
    mut mys_deposit_rx: tokio::sync::mpsc::UnboundedReceiver<MysDepositEvent>,
    storage: Arc<BridgeOrchestratorTables>,
    address_manager: Arc<DepositAddressManager>,
    gas_manager: Arc<DepositGasManager<mys_sdk::MysClient>>,
    mys_client: Arc<MysBridgeClient>,
    bridge_object: ObjectArg,
    token_type_tags: Arc<ArcSwap<Arc<HashMap<u8, TypeTag>>>>,
) {
    info!("Starting MySocial deposit processor");

    while let Some(deposit_event) = mys_deposit_rx.recv().await {
        info!(
            tx_digest = ?deposit_event.tx_digest,
            recipient = ?deposit_event.recipient,
            amount = deposit_event.amount,
            coin_type = deposit_event.coin_type,
            "Processing MySocial deposit"
        );

        // Get current token type tags from ArcSwap
        let token_type_tags_map = token_type_tags.load();

        match handle_mys_deposit(
            deposit_event.clone(),
            &storage,
            &address_manager,
            &gas_manager,
            &mys_client,
            bridge_object,
            &token_type_tags_map,
        )
        .await
        {
            Ok(bridge_tx_digest) => {
                info!(
                    deposit_tx = ?deposit_event.tx_digest,
                    bridge_tx = ?bridge_tx_digest,
                    "Successfully bridged MySocial deposit"
                );
            }
            Err(e) => {
                error!(
                    deposit_tx = ?deposit_event.tx_digest,
                    error = ?e,
                    "Failed to bridge MySocial deposit"
                );
                // Don't panic - log and continue
                // User can manually intervene if needed
            }
        }
    }

    info!("MySocial deposit processor shutting down");
}

