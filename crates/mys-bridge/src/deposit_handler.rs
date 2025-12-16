// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit event handler - processes detected deposits and triggers bridging

use crate::deposit_bridge::DepositBridgeHandler;
use crate::deposit_monitor::EvmDepositEvent;
use crate::mys_client::MysClientInner;
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

