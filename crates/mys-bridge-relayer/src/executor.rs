// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::AsyncConnection;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::RunQueryDsl as AsyncRunQueryDsl;
use mys_sdk::MysClientBuilder;
use move_core_types::ident_str;
use mys_json_rpc_types::{DevInspectResults, MysObjectDataOptions};
use mys_json_rpc_types::MysTransactionBlockResponseOptions;
use mys_types::base_types::{MysAddress, ObjectID, SequenceNumber};
use mys_types::bridge::BRIDGE_MODULE_NAME;
use mys_types::crypto::{MysKeyPair, Signature};
use mys_types::Identifier;
use mys_types::object::Owner as MysOwner;
use mys_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mys_types::quorum_driver_types::ExecuteTransactionRequestType;
use mys_types::transaction::{ObjectArg, Transaction, TransactionData};
use mys_types::{BRIDGE_PACKAGE_ID, TypeTag};
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::config::RelayerConfig;
use crate::models::EvmDeposit;
use crate::postgres_manager::PgPool;
use crate::schema::evm_deposits;
use mys_keys::keypair_file::read_keypair_from_file;

/// Executor that calls relayer_mint_and_transfer for finalized deposits.
pub struct RelayerExecutor {
    config: RelayerConfig,
    pool: PgPool,
    mys_client: mys_sdk::MysClient,
    relayer_key: MysKeyPair,
    relayer_address: MysAddress,
    bridge_object_id: ObjectID,
}

impl RelayerExecutor {
    pub async fn new(config: RelayerConfig, pool: PgPool) -> Result<Self> {
        let mys_client = MysClientBuilder::default()
            .build(&config.mys_rpc_url)
            .await
            .map_err(|e| anyhow!("Failed to create MySo client: {e}"))?;

        // Load relayer key
        let relayer_key_path = config
            .relayer_key_path
            .as_ref()
            .ok_or_else(|| anyhow!("relayer_key_path must be set"))?;
        let relayer_key = read_keypair_from_file(relayer_key_path)
            .map_err(|e| anyhow!("Failed to load relayer key: {e}"))?;

        let relayer_address = MysAddress::from(&relayer_key.public());

        info!(
            relayer_address = %relayer_address,
            bridge_object_id = %config.bridge_object_id,
            "Relayer executor initialized"
        );

        Ok(Self {
            config,
            pool,
            mys_client,
            relayer_key,
            relayer_address,
            bridge_object_id: config.bridge_object_id.clone(),
        })
    }

    /// Main executor loop: process finalized deposits and mint on MySo.
    pub async fn run(&self) -> Result<()> {
        if self.config.observe_only {
            info!("Executor disabled (observe_only=true)");
            return Ok(());
        }

        info!("Starting relayer executor");

        loop {
            match self.executor_cycle().await {
                Ok(_) => {}
                Err(e) => {
                    error!(error = %e, "Executor cycle failed");
                    sleep(Duration::from_secs(10)).await;
                }
            }
            sleep(Duration::from_secs(5)).await; // Check every 5 seconds
        }
    }

    async fn executor_cycle(&self) -> Result<()> {
        // 1. Query for finalized deposits that haven't been credited yet
        let mut conn = self.pool.get().await?;
        let pending: Vec<EvmDeposit> = evm_deposits::table
            .filter(evm_deposits::status.eq("finalized"))
            .filter(evm_deposits::credited_at.is_null())
            .order_by(evm_deposits::block_number.asc())
            .limit(10) // Process in batches
            .load(&mut conn)
            .await?;

        if pending.is_empty() {
            return Ok(());
        }

        info!(count = pending.len(), "Processing {} finalized deposits");

        for deposit in pending {
            match self.process_deposit(&deposit).await {
                Ok(tx_digest) => {
                    // Update deposit status
                    diesel_async::RunQueryDsl::execute(
                        diesel::update(
                            evm_deposits::table
                                .filter(evm_deposits::id.eq(deposit.id)),
                        )
                        .set((
                            evm_deposits::status.eq("credited"),
                            evm_deposits::credited_at.eq(diesel::dsl::now),
                            evm_deposits::myso_tx_digest.eq(Some(tx_digest.inner().to_vec())),
                        )),
                        &mut conn,
                    )
                    .await?;

                    info!(
                        deposit_id = deposit.id,
                        tx_digest = %tx_digest,
                        "Successfully credited deposit"
                    );
                }
                Err(e) => {
                    error!(
                        deposit_id = deposit.id,
                        error = %e,
                        "Failed to process deposit"
                    );
                    // Mark as failed (could retry later)
                    diesel::update(
                        evm_deposits::table
                            .filter(evm_deposits::id.eq(deposit.id)),
                    )
                    .set(evm_deposits::status.eq("failed"))
                    .execute(&mut conn)
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn process_deposit(&self, deposit: &EvmDeposit) -> Result<mys_types::digests::TransactionDigest> {
        // 1. Get gas object
        let gas_coins = self
            .mys_client
            .coin_read_api()
            .get_coins(self.relayer_address, None, None, None)
            .await?;
        let gas = gas_coins
            .data
            .into_iter()
            .find(|coin| coin.balance >= 5_000_000_000) // Need at least 5 MYS
            .ok_or_else(|| anyhow!("No gas object with sufficient balance"))?;
        let gas_obj_ref = gas.object_ref();

        // 2. Get reference gas price
        let rgp = self
            .mys_client
            .governance_api()
            .get_reference_gas_price()
            .await?;

        // 3. Get bridge object arg
        let bridge_arg = self.get_bridge_object_arg().await?;

        // 4. Parse deposit data
        let asset_id: [u8; 32] = deposit
            .asset_id
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("Invalid asset_id length"))?;
        let deposit_hash: [u8; 32] = deposit
            .deposit_hash
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("Invalid deposit_hash length"))?;
        let mys_address_bytes: [u8; 32] = deposit
            .mys_address
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("Invalid mys_address length"))?;
        let mys_address = MysAddress::from_bytes(&mys_address_bytes)
            .map_err(|e| anyhow!("Invalid mys_address: {e}"))?;

        // Convert amount from wei (BigDecimal) to u64
        // Note: This assumes the amount fits in u64. For very large amounts, we'd need u128/u256 handling.
        let amount = deposit
            .amount_wei
            .to_u64()
            .ok_or_else(|| anyhow!("Amount too large for u64"))?;

        // Determine source chain ID from chain_name
        // Map chain_name to MySo bridge chain_id using config
        let source_chain = self
            .config
            .evm_chains
            .iter()
            .find(|c| c.chain_name == deposit.chain_name)
            .and_then(|c| c.mys_chain_id.or_else(|| {
                // Fallback: try to cast chain_id to u8 (will fail if > 255)
                if c.chain_id <= 255 {
                    Some(c.chain_id as u8)
                } else {
                    None
                }
            }))
            .ok_or_else(|| {
                anyhow!(
                    "Chain '{}' not found in config.evm_chains or chain_id > 255. \
                     Set mys_chain_id explicitly in config.",
                    deposit.chain_name
                )
            })?;

        // 5. Determine coin type from asset_id mapping
        // For now, we'll need to query the bridge to get the token_id, then construct TypeTag
        // This is a simplification - in production, you'd cache asset_id -> token_id -> TypeTag mappings
        let coin_type = self.get_coin_type_for_asset(&asset_id).await?;

        // 6. Build transaction
        let mut builder = ProgrammableTransactionBuilder::new();
        let arg_bridge = builder.obj(bridge_arg)?;
        let arg_asset_id = builder.pure(asset_id.to_vec())?;
        let arg_deposit_hash = builder.pure(deposit_hash.to_vec())?;
        let arg_amount = builder.pure(amount)?;
        let arg_recipient = builder.pure(mys_address)?;
        let arg_source_chain = builder.pure(source_chain)?;

        // Get Clock object (constant: 0x6)
        let clock_id = ObjectID::from_hex_literal("0x0000000000000000000000000000000000000000000000000000000000000006")
            .expect("Invalid clock object ID");
        let clock_arg = self.get_shared_object_arg(clock_id, false).await?;
        let arg_clock = builder.obj(clock_arg)?;

        builder.programmable_move_call(
            BRIDGE_PACKAGE_ID,
            BRIDGE_MODULE_NAME.to_owned(),
            move_core_types::ident_str!("relayer_mint_and_transfer").to_owned(),
            vec![coin_type],
            vec![
                arg_bridge,
                arg_asset_id,
                arg_deposit_hash,
                arg_amount,
                arg_recipient,
                arg_source_chain,
                arg_clock,
            ],
        );

        let pt = builder.finish();
        let tx_data = TransactionData::new_programmable(
            self.relayer_address,
            vec![gas_obj_ref],
            pt,
            500_000_000,
            rgp,
        );

        // 7. Sign and submit
        let sig = Signature::new_secure(
            &IntentMessage::new(Intent::mys_transaction(), tx_data.clone()),
            &self.relayer_key,
        );
        let signed_tx = Transaction::from_data(tx_data, vec![sig]);
        let resp = self
            .mys_client
            .quorum_driver_api()
            .execute_transaction_block(
                signed_tx,
                MysTransactionBlockResponseOptions::new()
                    .with_effects()
                    .with_events(),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await?;

        if !resp.status_ok().unwrap_or(false) {
            return Err(anyhow!("Transaction failed: {:?}", resp));
        }

        Ok(resp.digest)
    }

    async fn get_bridge_object_arg(&self) -> Result<ObjectArg> {
        self.get_shared_object_arg(self.bridge_object_id, true).await
    }

    async fn get_shared_object_arg(&self, id: ObjectID, mutable: bool) -> Result<ObjectArg> {
        let resp = self
            .mys_client
            .read_api()
            .get_object_with_options(id, MysObjectDataOptions::new().with_owner())
            .await?;
        let data = resp
            .data
            .ok_or_else(|| anyhow!("object {id} not found"))?;
        let owner = data
            .owner
            .ok_or_else(|| anyhow!("object {id} missing owner field"))?;
        match owner {
            MysOwner::Shared {
                initial_shared_version,
            } => Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version: SequenceNumber::from_u64(initial_shared_version),
                mutable,
            }),
            other => Err(anyhow!(
                "object {id} is not shared (owner={other:?})"
            )),
        }
    }

    /// Get coin TypeTag for an asset_id.
    /// 
    /// This queries the bridge's asset_id_to_token_id mapping via dev_inspect,
    /// then looks up the TypeTag from the treasury's id_token_type_map.
    /// 
    /// TODO: Cache this mapping to avoid repeated queries.
    async fn get_coin_type_for_asset(&self, asset_id: &[u8; 32]) -> Result<TypeTag> {
        // 1. Get bridge object arg
        let bridge_arg = self.get_bridge_object_arg().await?;
        
        // 2. Call get_token_id_for_asset_id via dev_inspect
        let token_id_opt = self
            .get_token_id_for_asset_id_via_dev_inspect(bridge_arg, asset_id.to_vec())
            .await?;
        
        let token_id = token_id_opt.ok_or_else(|| {
            anyhow!(
                "Asset ID {} not mapped to any token_id. Use 'myso-bridge relayer-admin set-asset-mapping' to register it.",
                hex::encode(asset_id)
            )
        })?;
        
        // 3. Get bridge summary to access treasury's id_token_type_map
        // Use mys-bridge client wrapper
        use mys_bridge::mys_client::MysClient;
        use mys_bridge::metrics::BridgeMetrics;
        use std::sync::Arc;
        
        let mys_rpc_url = self.config.mys_rpc_url.clone();
        let bridge_metrics = Arc::new(BridgeMetrics::new_for_testing());
        let bridge_client = MysClient::new(&mys_rpc_url, bridge_metrics)
            .await
            .map_err(|e| anyhow!("Failed to create bridge client: {e}"))?;
        
        let bridge_summary = bridge_client
            .get_bridge_summary()
            .await
            .map_err(|e| anyhow!("Failed to get bridge summary: {:?}", e))?;
        
        // 4. Look up TypeName for this token_id
        let type_name = bridge_summary
            .treasury
            .id_token_type_map
            .iter()
            .find(|(id, _)| *id == token_id)
            .map(|(_, name)| name.clone())
            .ok_or_else(|| {
                anyhow!(
                    "Token ID {} not found in treasury's id_token_type_map",
                    token_id
                )
            })?;
        
        // 5. Parse TypeName string to TypeTag
        // TypeName format: "0x<PACKAGE_ID>::<MODULE>::<TYPE>"
        mys_types::parse_mys_type_tag(&type_name)
            .map_err(|e| anyhow!("Failed to parse type name '{}' to TypeTag: {e}", type_name))
    }
    
    /// Call bridge::get_token_id_for_asset_id via dev_inspect.
    async fn get_token_id_for_asset_id_via_dev_inspect(
        &self,
        bridge_arg: ObjectArg,
        asset_id: Vec<u8>,
    ) -> Result<Option<u8>> {
        use mys_types::transaction::{Argument, CallArg, Command, ProgrammableTransaction, TransactionKind};
        use mys_types::base_types::MysAddress;
        use mys_types::Identifier;
        use mys_types::BRIDGE_PACKAGE_ID;
        use mys_types::bridge::BRIDGE_MODULE_NAME;
        
        let pt = ProgrammableTransaction {
            inputs: vec![
                CallArg::Object(bridge_arg),
                CallArg::Pure(bcs::to_bytes(&asset_id)?),
            ],
            commands: vec![Command::move_call(
                BRIDGE_PACKAGE_ID,
                Identifier::new(BRIDGE_MODULE_NAME.as_str())?,
                Identifier::new("get_token_id_for_asset_id")?,
                vec![],
                vec![Argument::Input(0), Argument::Input(1)],
            )],
        };
        
        let kind = TransactionKind::programmable(pt);
        let resp = self
            .mys_client
            .read_api()
            .dev_inspect_transaction_block(MysAddress::ZERO, kind, None, None, None)
            .await?;
        
        let DevInspectResults { results, .. } = resp;
        let Some(results) = results else {
            return Err(anyhow!("No results returned from dev_inspect"));
        };
        
        let return_values = &results
            .first()
            .ok_or_else(|| anyhow!("No execution results"))?
            .return_values;
        
        let (value_bytes, _type_tag) = return_values
            .first()
            .ok_or_else(|| anyhow!("No return values"))?;
        
        // Deserialize Option<u8>
        // Move's Option<u8> is encoded as: 0x00 for None, 0x01 || u8 for Some(u8)
        if value_bytes.is_empty() {
            return Ok(None);
        }
        if value_bytes[0] == 0 {
            Ok(None)
        } else if value_bytes[0] == 1 && value_bytes.len() == 2 {
            Ok(Some(value_bytes[1]))
        } else {
            Err(anyhow!("Invalid Option<u8> encoding: {:?}", value_bytes))
        }
    }
}
