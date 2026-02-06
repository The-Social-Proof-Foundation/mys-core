// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Error;
use tracing::{info, warn};

use mys_bridge::events::{
    EmergencyOpEvent, MoveBlocklistValidatorEvent, MoveNewTokenEvent, MoveTokenDepositedEvent,
    MoveTokenRegistrationEvent, MoveTokenTransferApproved, MoveTokenTransferClaimed,
    UpdateRouteLimitEvent, UpdateTokenPriceEvent,
};
use mys_indexer_builder::indexer_builder::DataMapper;
use mys_indexer_builder::mys_datasource::CheckpointTxnData;
use mys_types::effects::TransactionEffectsAPI;
use mys_types::event::Event;
use mys_types::execution_status::ExecutionStatus;
use mys_types::full_checkpoint_content::CheckpointTransaction;
use mys_types::{BRIDGE_ADDRESS, MYS_BRIDGE_OBJECT_ID};

use crate::metrics::BridgeIndexerMetrics;
use crate::{
    BridgeDataSource, GovernanceAction, GovernanceActionType, MysTxnError, ProcessedTxnData,
    TokenTransfer, TokenTransferData, TokenTransferStatus,
};

/// Data mapper impl
#[derive(Clone)]
pub struct MysBridgeDataMapper {
    pub metrics: BridgeIndexerMetrics,
}

impl DataMapper<CheckpointTxnData, ProcessedTxnData> for MysBridgeDataMapper {
    fn map(
        &self,
        (data, checkpoint_num, timestamp_ms): CheckpointTxnData,
    ) -> Result<Vec<ProcessedTxnData>, Error> {
        self.metrics.total_mys_bridge_transactions.inc();
        if !data
            .input_objects
            .iter()
            .any(|obj| obj.id() == MYS_BRIDGE_OBJECT_ID)
        {
            return Ok(vec![]);
        }

        match &data.events {
            Some(events) => {
                let mut all_data = vec![];

                // Process main bridge events
                for ev in &events.data {
                    if let Some(processed) =
                        process_mys_event(ev, &data, checkpoint_num, timestamp_ms)?
                    {
                        // Check if this is a native MYS transfer event (token_id 0)
                        // and create corresponding treasury events
                        if let ProcessedTxnData::TokenTransfer(ref transfer) = processed {
                            if let Some(ref transfer_data) = transfer.data {
                                if transfer_data.token_id == 0 {
                                    // This is native MYS
                                    match transfer.status {
                                        crate::TokenTransferStatus::Deposited => {
                                            // Lock event: MYS being sent from MySo -> Eth
                                            all_data.push(ProcessedTxnData::TreasuryEvent(crate::TreasuryEvent {
                                                token_type: "0x0000000000000000000000000000000000000000000000000000000000000002::mys::MYS".to_string(),
                                                token_id: 0,
                                                event_type: crate::TreasuryEventType::Lock,
                                                amount: transfer_data.amount,
                                                tx_digest: transfer.txn_hash.clone(),
                                                block_height: transfer.block_height,
                                                timestamp_ms: transfer.timestamp_ms,
                                                sender: transfer.txn_sender.clone(),
                                            }));
                                        }
                                        crate::TokenTransferStatus::Claimed => {
                                            // Unlock event: MYS being claimed from Eth -> MySo
                                            // Only if source chain is not MySo (coming from Ethereum)
                                            if transfer.chain_id != 2 {
                                                all_data.push(ProcessedTxnData::TreasuryEvent(crate::TreasuryEvent {
                                                    token_type: "0x0000000000000000000000000000000000000000000000000000000000000002::mys::MYS".to_string(),
                                                    token_id: 0,
                                                    event_type: crate::TreasuryEventType::Unlock,
                                                    amount: transfer_data.amount,
                                                    tx_digest: transfer.txn_hash.clone(),
                                                    block_height: transfer.block_height,
                                                    timestamp_ms: transfer.timestamp_ms,
                                                    sender: transfer.txn_sender.clone(),
                                                }));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        all_data.push(processed);
                    }
                }

                if !all_data.is_empty() {
                    info!(
                        "MYS: Extracted {} bridge data entries for tx {}.",
                        all_data.len(),
                        data.transaction.digest()
                    );
                }
                Ok(all_data)
            }
            None => {
                if let ExecutionStatus::Failure { error, command } = data.effects.status() {
                    Ok(vec![ProcessedTxnData::Error(MysTxnError {
                        tx_digest: *data.transaction.digest(),
                        sender: data.transaction.sender_address(),
                        timestamp_ms,
                        failure_status: error.to_string(),
                        cmd_idx: command.map(|idx| idx as u64),
                    })])
                } else {
                    Ok(vec![])
                }
            }
        }
    }
}

fn process_mys_event(
    ev: &Event,
    tx: &CheckpointTransaction,
    checkpoint: u64,
    timestamp_ms: u64,
) -> Result<Option<ProcessedTxnData>, anyhow::Error> {
    Ok(if ev.type_.address == BRIDGE_ADDRESS {
        match ev.type_.name.as_str() {
            "TokenDepositedEvent" => {
                info!("Observed Mys Deposit {:?}", ev);
                // todo: metrics.total_mys_token_deposited.inc();
                let move_event: MoveTokenDepositedEvent = bcs::from_bytes(&ev.contents)?;
                Some(ProcessedTxnData::TokenTransfer(TokenTransfer {
                    chain_id: move_event.source_chain,
                    nonce: move_event.seq_num,
                    block_height: checkpoint,
                    timestamp_ms,
                    txn_hash: tx.transaction.digest().inner().to_vec(),
                    txn_sender: ev.sender.to_vec(),
                    status: TokenTransferStatus::Deposited,
                    gas_usage: tx.effects.gas_cost_summary().net_gas_usage(),
                    data_source: BridgeDataSource::Mys,
                    is_finalized: true,
                    data: Some(TokenTransferData {
                        destination_chain: move_event.target_chain,
                        sender_address: move_event.sender_address.clone(),
                        recipient_address: move_event.target_address.clone(),
                        token_id: move_event.token_type,
                        amount: move_event.amount_mys_adjusted,
                        is_finalized: true,
                    }),
                }))
            }
            "TokenTransferApproved" => {
                info!("Observed Mys Approval {:?}", ev);
                // todo: metrics.total_mys_token_transfer_approved.inc();
                let event: MoveTokenTransferApproved = bcs::from_bytes(&ev.contents)?;
                Some(ProcessedTxnData::TokenTransfer(TokenTransfer {
                    chain_id: event.message_key.source_chain,
                    nonce: event.message_key.bridge_seq_num,
                    block_height: checkpoint,
                    timestamp_ms,
                    txn_hash: tx.transaction.digest().inner().to_vec(),
                    txn_sender: ev.sender.to_vec(),
                    status: TokenTransferStatus::Approved,
                    gas_usage: tx.effects.gas_cost_summary().net_gas_usage(),
                    data_source: BridgeDataSource::Mys,
                    data: None,
                    is_finalized: true,
                }))
            }
            "TokenTransferClaimed" => {
                info!("Observed Mys Claim {:?}", ev);
                // todo: metrics.total_mys_token_transfer_claimed.inc();
                let event: MoveTokenTransferClaimed = bcs::from_bytes(&ev.contents)?;
                Some(ProcessedTxnData::TokenTransfer(TokenTransfer {
                    chain_id: event.message_key.source_chain,
                    nonce: event.message_key.bridge_seq_num,
                    block_height: checkpoint,
                    timestamp_ms,
                    txn_hash: tx.transaction.digest().inner().to_vec(),
                    txn_sender: ev.sender.to_vec(),
                    status: TokenTransferStatus::Claimed,
                    gas_usage: tx.effects.gas_cost_summary().net_gas_usage(),
                    data_source: BridgeDataSource::Mys,
                    data: None,
                    is_finalized: true,
                }))
            }
            "UpdateRouteLimitEvent" => {
                info!("Observed Mys Route Limit Update {:?}", ev);
                let event: UpdateRouteLimitEvent = bcs::from_bytes(&ev.contents)?;

                Some(ProcessedTxnData::GovernanceAction(GovernanceAction {
                    nonce: None,
                    data_source: BridgeDataSource::Mys,
                    tx_digest: tx.transaction.digest().inner().to_vec(),
                    sender: ev.sender.to_vec(),
                    timestamp_ms,
                    action: GovernanceActionType::UpdateBridgeLimit,
                    data: serde_json::to_value(event)?,
                }))
            }
            "EmergencyOpEvent" => {
                info!("Observed Mys Emergency Op {:?}", ev);
                let event: EmergencyOpEvent = bcs::from_bytes(&ev.contents)?;

                Some(ProcessedTxnData::GovernanceAction(GovernanceAction {
                    nonce: None,
                    data_source: BridgeDataSource::Mys,
                    tx_digest: tx.transaction.digest().inner().to_vec(),
                    sender: ev.sender.to_vec(),
                    timestamp_ms,
                    action: GovernanceActionType::EmergencyOperation,
                    data: serde_json::to_value(event)?,
                }))
            }
            "BlocklistValidatorEvent" => {
                info!("Observed Mys Blocklist Validator {:?}", ev);
                let event: MoveBlocklistValidatorEvent = bcs::from_bytes(&ev.contents)?;

                Some(ProcessedTxnData::GovernanceAction(GovernanceAction {
                    nonce: None,
                    data_source: BridgeDataSource::Mys,
                    tx_digest: tx.transaction.digest().inner().to_vec(),
                    sender: ev.sender.to_vec(),
                    timestamp_ms,
                    action: GovernanceActionType::UpdateCommitteeBlocklist,
                    data: serde_json::to_value(event)?,
                }))
            }
            "TokenRegistrationEvent" => {
                info!("Observed Mys Token Registration {:?}", ev);
                let event: MoveTokenRegistrationEvent = bcs::from_bytes(&ev.contents)?;

                Some(ProcessedTxnData::GovernanceAction(GovernanceAction {
                    nonce: None,
                    data_source: BridgeDataSource::Mys,
                    tx_digest: tx.transaction.digest().inner().to_vec(),
                    sender: ev.sender.to_vec(),
                    timestamp_ms,
                    action: GovernanceActionType::AddMysTokens,
                    data: serde_json::to_value(event)?,
                }))
            }
            "UpdateTokenPriceEvent" => {
                info!("Observed Mys Token Price Update {:?}", ev);
                let event: UpdateTokenPriceEvent = bcs::from_bytes(&ev.contents)?;

                Some(ProcessedTxnData::GovernanceAction(GovernanceAction {
                    nonce: None,
                    data_source: BridgeDataSource::Mys,
                    tx_digest: tx.transaction.digest().inner().to_vec(),
                    sender: ev.sender.to_vec(),
                    timestamp_ms,
                    action: GovernanceActionType::UpdateTokenPrices,
                    data: serde_json::to_value(event)?,
                }))
            }
            "NewTokenEvent" => {
                info!("Observed Mys New token event {:?}", ev);
                let event: MoveNewTokenEvent = bcs::from_bytes(&ev.contents)?;

                Some(ProcessedTxnData::GovernanceAction(GovernanceAction {
                    nonce: None,
                    data_source: BridgeDataSource::Mys,
                    tx_digest: tx.transaction.digest().inner().to_vec(),
                    sender: ev.sender.to_vec(),
                    timestamp_ms,
                    action: GovernanceActionType::AddMysTokens,
                    data: serde_json::to_value(event)?,
                }))
            }
            _ => {
                // todo: metrics.total_mys_bridge_txn_other.inc();
                warn!("Unexpected event {ev:?}.");
                None
            }
        }
    } else {
        None
    })
}
