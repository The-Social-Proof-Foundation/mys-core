// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use anyhow::bail;
use async_trait::async_trait;
use embedded_reconfig_observer::EmbeddedReconfigObserver;
use fullnode_reconfig_observer::FullNodeReconfigObserver;
use prometheus::Registry;
use rand::Rng;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use mys_config::genesis::Genesis;
use mys_core::{
    authority_aggregator::{AuthorityAggregator, AuthorityAggregatorBuilder},
    authority_client::NetworkAuthorityClient,
    quorum_driver::{
        reconfig_observer::ReconfigObserver, QuorumDriver, QuorumDriverHandler,
        QuorumDriverHandlerBuilder, QuorumDriverMetrics,
    },
};
use mys_json_rpc_types::{
    MysObjectDataOptions, MysObjectResponse, MysObjectResponseQuery, MysTransactionBlockEffects,
    MysTransactionBlockEffectsAPI, MysTransactionBlockResponseOptions,
};
use mys_sdk::{MysClient, MysClientBuilder};
use mys_types::effects::{TransactionEffectsAPI, TransactionEvents};
use mys_types::gas::GasCostSummary;
use mys_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mys_types::quorum_driver_types::EffectsFinalityInfo;
use mys_types::quorum_driver_types::FinalizedEffects;
use mys_types::mys_system_state::mys_system_state_summary::MysSystemStateSummary;
use mys_types::transaction::Argument;
use mys_types::transaction::CallArg;
use mys_types::transaction::ObjectArg;
use mys_types::{
    base_types::ObjectID,
    committee::{Committee, EpochId},
    object::Object,
    transaction::Transaction,
};
use mys_types::{base_types::ObjectRef, crypto::AuthorityStrongQuorumSignInfo, object::Owner};
use mys_types::{base_types::SequenceNumber, gas_coin::GasCoin};
use mys_types::{
    base_types::{AuthorityName, MysAddress},
    mys_system_state::MysSystemStateTrait,
};
use tokio::time::sleep;
use tracing::{error, info, warn};

pub mod bank;
pub mod benchmark_setup;
pub mod drivers;
pub mod embedded_reconfig_observer;
pub mod fullnode_reconfig_observer;
pub mod in_memory_wallet;
pub mod options;
pub mod system_state_observer;
pub mod util;
pub mod workloads;
use mys_types::quorum_driver_types::{QuorumDriverError, QuorumDriverResponse};

#[derive(Debug)]
/// A wrapper on execution results to accommodate different types of
/// responses from LocalValidatorAggregatorProxy and FullNodeProxy
#[allow(clippy::large_enum_variant)]
pub enum ExecutionEffects {
    FinalizedTransactionEffects(FinalizedEffects, TransactionEvents),
    MysTransactionBlockEffects(MysTransactionBlockEffects),
}

impl ExecutionEffects {
    pub fn mutated(&self) -> Vec<(ObjectRef, Owner)> {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => {
                effects.data().mutated().to_vec()
            }
            ExecutionEffects::MysTransactionBlockEffects(mys_tx_effects) => mys_tx_effects
                .mutated()
                .iter()
                .map(|refe| (refe.reference.to_object_ref(), refe.owner.clone()))
                .collect(),
        }
    }

    pub fn created(&self) -> Vec<(ObjectRef, Owner)> {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => effects.data().created(),
            ExecutionEffects::MysTransactionBlockEffects(mys_tx_effects) => mys_tx_effects
                .created()
                .iter()
                .map(|refe| (refe.reference.to_object_ref(), refe.owner.clone()))
                .collect(),
        }
    }

    pub fn deleted(&self) -> Vec<ObjectRef> {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => {
                effects.data().deleted().to_vec()
            }
            ExecutionEffects::MysTransactionBlockEffects(mys_tx_effects) => mys_tx_effects
                .deleted()
                .iter()
                .map(|refe| refe.to_object_ref())
                .collect(),
        }
    }

    pub fn quorum_sig(&self) -> Option<&AuthorityStrongQuorumSignInfo> {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => {
                match &effects.finality_info {
                    EffectsFinalityInfo::Certified(sig) => Some(sig),
                    _ => None,
                }
            }
            ExecutionEffects::MysTransactionBlockEffects(_) => None,
        }
    }

    pub fn gas_object(&self) -> (ObjectRef, Owner) {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => {
                effects.data().gas_object()
            }
            ExecutionEffects::MysTransactionBlockEffects(mys_tx_effects) => {
                let refe = &mys_tx_effects.gas_object();
                (refe.reference.to_object_ref(), refe.owner.clone())
            }
        }
    }

    pub fn sender(&self) -> MysAddress {
        match self.gas_object().1 {
            Owner::AddressOwner(a) => a,
            Owner::ObjectOwner(_)
            | Owner::Shared { .. }
            | Owner::Immutable
            | Owner::ConsensusV2 { .. } => unreachable!(), // owner of gas object is always an address
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => {
                effects.data().status().is_ok()
            }
            ExecutionEffects::MysTransactionBlockEffects(mys_tx_effects) => {
                mys_tx_effects.status().is_ok()
            }
        }
    }

    pub fn status(&self) -> String {
        match self {
            ExecutionEffects::FinalizedTransactionEffects(effects, ..) => {
                format!("{:#?}", effects.data().status())
            }
            ExecutionEffects::MysTransactionBlockEffects(mys_tx_effects) => {
                format!("{:#?}", mys_tx_effects.status())
            }
        }
    }

    pub fn gas_cost_summary(&self) -> GasCostSummary {
        match self {
            crate::ExecutionEffects::FinalizedTransactionEffects(a, _) => {
                a.data().gas_cost_summary().clone()
            }
            crate::ExecutionEffects::MysTransactionBlockEffects(b) => {
                std::convert::Into::<GasCostSummary>::into(b.gas_cost_summary().clone())
            }
        }
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_cost_summary().gas_used()
    }

    pub fn net_gas_used(&self) -> i64 {
        self.gas_cost_summary().net_gas_usage()
    }

    pub fn print_gas_summary(&self) {
        let gas_object = self.gas_object();
        let sender = self.sender();
        let status = self.status();
        let gas_cost_summary = self.gas_cost_summary();
        let gas_used = self.gas_used();
        let net_gas_used = self.net_gas_used();

        info!(
            "Summary:\n\
             Gas Object: {gas_object:?}\n\
             Sender: {sender:?}\n\
             status: {status}\n\
             Gas Cost Summary: {gas_cost_summary:#?}\n\
             Gas Used: {gas_used}\n\
             Net Gas Used: {net_gas_used}"
        );
    }
}

#[async_trait]
pub trait ValidatorProxy {
    async fn get_object(&self, object_id: ObjectID) -> Result<Object, anyhow::Error>;

    async fn get_owned_objects(
        &self,
        account_address: MysAddress,
    ) -> Result<Vec<(u64, Object)>, anyhow::Error>;

    async fn get_latest_system_state_object(&self) -> Result<MysSystemStateSummary, anyhow::Error>;

    async fn execute_transaction_block(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects>;

    fn clone_committee(&self) -> Arc<Committee>;

    fn get_current_epoch(&self) -> EpochId;

    fn clone_new(&self) -> Box<dyn ValidatorProxy + Send + Sync>;

    async fn get_validators(&self) -> Result<Vec<MysAddress>, anyhow::Error>;
}

// TODO: Eventually remove this proxy because we shouldn't rely on validators to read objects.
pub struct LocalValidatorAggregatorProxy {
    _qd_handler: QuorumDriverHandler<NetworkAuthorityClient>,
    // Stress client does not verify individual validator signatures since this is very expensive
    qd: Arc<QuorumDriver<NetworkAuthorityClient>>,
    committee: Committee,
    clients: BTreeMap<AuthorityName, NetworkAuthorityClient>,
}

impl LocalValidatorAggregatorProxy {
    pub async fn from_genesis(
        genesis: &Genesis,
        registry: &Registry,
        reconfig_fullnode_rpc_url: Option<&str>,
    ) -> Self {
        let (aggregator, clients) = AuthorityAggregatorBuilder::from_genesis(genesis)
            .with_registry(registry)
            .build_network_clients();
        let committee = genesis.committee().unwrap();

        Self::new_impl(
            aggregator,
            registry,
            reconfig_fullnode_rpc_url,
            clients,
            committee,
        )
        .await
    }

    async fn new_impl(
        aggregator: AuthorityAggregator<NetworkAuthorityClient>,
        registry: &Registry,
        reconfig_fullnode_rpc_url: Option<&str>,
        clients: BTreeMap<AuthorityName, NetworkAuthorityClient>,
        committee: Committee,
    ) -> Self {
        let quorum_driver_metrics = Arc::new(QuorumDriverMetrics::new(registry));
        let (aggregator, reconfig_observer): (
            Arc<_>,
            Arc<dyn ReconfigObserver<NetworkAuthorityClient> + Sync + Send>,
        ) = if let Some(reconfig_fullnode_rpc_url) = reconfig_fullnode_rpc_url {
            info!(
                "Using FullNodeReconfigObserver: {:?}",
                reconfig_fullnode_rpc_url
            );
            let committee_store = aggregator.clone_committee_store();
            let reconfig_observer = Arc::new(
                FullNodeReconfigObserver::new(
                    reconfig_fullnode_rpc_url,
                    committee_store,
                    aggregator.safe_client_metrics_base.clone(),
                    aggregator.metrics.clone(),
                )
                .await,
            );
            (Arc::new(aggregator), reconfig_observer)
        } else {
            info!("Using EmbeddedReconfigObserver");
            let reconfig_observer = Arc::new(EmbeddedReconfigObserver::new());
            // Get the latest committee from config observer
            let aggregator = reconfig_observer
                .get_committee(Arc::new(aggregator))
                .await
                .expect("Failed to get latest committee");
            (aggregator, reconfig_observer)
        };
        let qd_handler_builder =
            QuorumDriverHandlerBuilder::new(aggregator.clone(), quorum_driver_metrics.clone())
                .with_reconfig_observer(reconfig_observer.clone());
        let qd_handler = qd_handler_builder.start();
        let qd = qd_handler.clone_quorum_driver();
        Self {
            _qd_handler: qd_handler,
            qd,
            clients,
            committee,
        }
    }
}

#[async_trait]
impl ValidatorProxy for LocalValidatorAggregatorProxy {
    async fn get_object(&self, object_id: ObjectID) -> Result<Object, anyhow::Error> {
        let auth_agg = self.qd.authority_aggregator().load();
        Ok(auth_agg
            .get_latest_object_version_for_testing(object_id)
            .await?)
    }

    async fn get_owned_objects(
        &self,
        _account_address: MysAddress,
    ) -> Result<Vec<(u64, Object)>, anyhow::Error> {
        unimplemented!("Not available for local proxy");
    }

    async fn get_latest_system_state_object(&self) -> Result<MysSystemStateSummary, anyhow::Error> {
        let auth_agg = self.qd.authority_aggregator().load();
        Ok(auth_agg
            .get_latest_system_state_object_for_testing()
            .await?
            .into_mys_system_state_summary())
    }

    async fn execute_transaction_block(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects> {
        let tx_digest = *tx.digest();
        let mut retry_cnt = 0;
        while retry_cnt < 3 {
            let ticket = self
                .qd
                .submit_transaction(
                    mys_types::quorum_driver_types::ExecuteTransactionRequestV3 {
                        transaction: tx.clone(),
                        include_events: true,
                        include_input_objects: false,
                        include_output_objects: false,
                        include_auxiliary_data: false,
                    },
                )
                .await?;
            // The ticket only times out when QuorumDriver exceeds the retry times
            match ticket.await {
                Ok(resp) => {
                    let QuorumDriverResponse {
                        effects_cert,
                        events,
                        ..
                    } = resp;
                    return Ok(ExecutionEffects::FinalizedTransactionEffects(
                        FinalizedEffects::new_from_effects_cert(effects_cert.into()),
                        events.unwrap_or_default(),
                    ));
                }
                Err(QuorumDriverError::NonRecoverableTransactionError { errors }) => {
                    bail!(QuorumDriverError::NonRecoverableTransactionError { errors });
                }
                Err(err) => {
                    let delay = Duration::from_millis(rand::thread_rng().gen_range(100..1000));
                    warn!(
                        ?tx_digest,
                        retry_cnt,
                        "Transaction failed with err: {:?}. Sleeping for {:?} ...",
                        err,
                        delay,
                    );
                    retry_cnt += 1;
                    sleep(delay).await;
                }
            }
        }
        bail!("Transaction {:?} failed for {retry_cnt} times", tx_digest);
    }

    fn clone_committee(&self) -> Arc<Committee> {
        self.qd.clone_committee()
    }

    fn get_current_epoch(&self) -> EpochId {
        self.qd.current_epoch()
    }

    fn clone_new(&self) -> Box<dyn ValidatorProxy + Send + Sync> {
        let qdh = self._qd_handler.clone_new();
        let qd = qdh.clone_quorum_driver();
        Box::new(Self {
            _qd_handler: qdh,
            qd,
            clients: self.clients.clone(),
            committee: self.committee.clone(),
        })
    }

    async fn get_validators(&self) -> Result<Vec<MysAddress>, anyhow::Error> {
        let system_state = self.get_latest_system_state_object().await?;
        Ok(system_state
            .active_validators
            .iter()
            .map(|v| v.mys_address)
            .collect())
    }
}

pub struct FullNodeProxy {
    mys_client: MysClient,
    committee: Arc<Committee>,
}

impl FullNodeProxy {
    pub async fn from_url(http_url: &str) -> Result<Self, anyhow::Error> {
        // Each request times out after 60s (default value)
        let mys_client = MysClientBuilder::default()
            .max_concurrent_requests(500_000)
            .build(http_url)
            .await?;

        let resp = mys_client.read_api().get_committee_info(None).await?;
        let epoch = resp.epoch;
        let committee_vec = resp.validators;
        let committee_map = BTreeMap::from_iter(committee_vec.into_iter());
        let committee =
            Committee::new_for_testing_with_normalized_voting_power(epoch, committee_map);

        Ok(Self {
            mys_client,
            committee: Arc::new(committee),
        })
    }
}

#[async_trait]
impl ValidatorProxy for FullNodeProxy {
    async fn get_object(&self, object_id: ObjectID) -> Result<Object, anyhow::Error> {
        let response = self
            .mys_client
            .read_api()
            .get_object_with_options(object_id, MysObjectDataOptions::bcs_lossless())
            .await?;

        if let Some(mys_object) = response.data {
            mys_object.try_into()
        } else if let Some(error) = response.error {
            bail!("Error getting object {:?}: {}", object_id, error)
        } else {
            bail!("Object {:?} not found and no error provided", object_id)
        }
    }

    async fn get_owned_objects(
        &self,
        account_address: MysAddress,
    ) -> Result<Vec<(u64, Object)>, anyhow::Error> {
        let mut objects: Vec<MysObjectResponse> = Vec::new();
        let mut cursor = None;
        loop {
            let response = self
                .mys_client
                .read_api()
                .get_owned_objects(
                    account_address,
                    Some(MysObjectResponseQuery::new_with_options(
                        MysObjectDataOptions::bcs_lossless(),
                    )),
                    cursor,
                    None,
                )
                .await?;

            objects.extend(response.data);

            if response.has_next_page {
                cursor = response.next_cursor;
            } else {
                break;
            }
        }

        let mut values_objects = Vec::new();

        for object in objects {
            let o = object.data;
            if let Some(o) = o {
                let temp: Object = o.clone().try_into()?;
                let gas_coin = GasCoin::try_from(&temp)?;
                values_objects.push((gas_coin.value(), o.clone().try_into()?));
            }
        }

        Ok(values_objects)
    }

    async fn get_latest_system_state_object(&self) -> Result<MysSystemStateSummary, anyhow::Error> {
        Ok(self
            .mys_client
            .governance_api()
            .get_latest_mys_system_state()
            .await?)
    }

    async fn execute_transaction_block(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects> {
        let tx_digest = *tx.digest();
        let mut retry_cnt = 0;
        while retry_cnt < 10 {
            // Fullnode could time out after WAIT_FOR_FINALITY_TIMEOUT (30s) in TransactionOrchestrator
            // MysClient times out after 60s
            match self
                .mys_client
                .quorum_driver_api()
                .execute_transaction_block(
                    tx.clone(),
                    MysTransactionBlockResponseOptions::new().with_effects(),
                    None,
                )
                .await
            {
                Ok(resp) => {
                    return Ok(ExecutionEffects::MysTransactionBlockEffects(
                        resp.effects.expect("effects field should not be None"),
                    ));
                }
                Err(err) => {
                    error!(
                        ?tx_digest,
                        retry_cnt, "Transaction failed with err: {:?}", err
                    );
                    retry_cnt += 1;
                }
            }
        }
        bail!("Transaction {:?} failed for {retry_cnt} times", tx_digest);
    }

    fn clone_committee(&self) -> Arc<Committee> {
        self.committee.clone()
    }

    fn get_current_epoch(&self) -> EpochId {
        self.committee.epoch
    }

    fn clone_new(&self) -> Box<dyn ValidatorProxy + Send + Sync> {
        Box::new(Self {
            mys_client: self.mys_client.clone(),
            committee: self.clone_committee(),
        })
    }

    async fn get_validators(&self) -> Result<Vec<MysAddress>, anyhow::Error> {
        let validators = self
            .mys_client
            .governance_api()
            .get_latest_mys_system_state()
            .await?
            .active_validators;
        Ok(validators.into_iter().map(|v| v.mys_address).collect())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BenchMoveCallArg {
    Pure(Vec<u8>),
    Shared((ObjectID, SequenceNumber, bool)),
    ImmOrOwnedObject(ObjectRef),
    ImmOrOwnedObjectVec(Vec<ObjectRef>),
    SharedObjectVec(Vec<(ObjectID, SequenceNumber, bool)>),
}

impl From<bool> for BenchMoveCallArg {
    fn from(b: bool) -> Self {
        // unwrap safe because every u8 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&b).unwrap())
    }
}

impl From<u8> for BenchMoveCallArg {
    fn from(n: u8) -> Self {
        // unwrap safe because every u8 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u16> for BenchMoveCallArg {
    fn from(n: u16) -> Self {
        // unwrap safe because every u16 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u32> for BenchMoveCallArg {
    fn from(n: u32) -> Self {
        // unwrap safe because every u32 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u64> for BenchMoveCallArg {
    fn from(n: u64) -> Self {
        // unwrap safe because every u64 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u128> for BenchMoveCallArg {
    fn from(n: u128) -> Self {
        // unwrap safe because every u128 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<&Vec<u8>> for BenchMoveCallArg {
    fn from(v: &Vec<u8>) -> Self {
        // unwrap safe because every vec<u8> value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(v).unwrap())
    }
}

impl From<ObjectRef> for BenchMoveCallArg {
    fn from(obj: ObjectRef) -> Self {
        BenchMoveCallArg::ImmOrOwnedObject(obj)
    }
}

impl From<CallArg> for BenchMoveCallArg {
    fn from(ca: CallArg) -> Self {
        match ca {
            CallArg::Pure(p) => BenchMoveCallArg::Pure(p),
            CallArg::Object(obj) => match obj {
                ObjectArg::ImmOrOwnedObject(imo) => BenchMoveCallArg::ImmOrOwnedObject(imo),
                ObjectArg::SharedObject {
                    id,
                    initial_shared_version,
                    mutable,
                } => BenchMoveCallArg::Shared((id, initial_shared_version, mutable)),
                ObjectArg::Receiving(_) => {
                    unimplemented!("Receiving is not supported for benchmarks")
                }
            },
        }
    }
}

/// Convert MoveCallArg to Vector of Argument for PT
pub fn convert_move_call_args(
    args: &[BenchMoveCallArg],
    pt_builder: &mut ProgrammableTransactionBuilder,
) -> Vec<Argument> {
    args.iter()
        .map(|arg| match arg {
            BenchMoveCallArg::Pure(bytes) => {
                pt_builder.input(CallArg::Pure(bytes.clone())).unwrap()
            }
            BenchMoveCallArg::Shared((id, initial_shared_version, mutable)) => pt_builder
                .input(CallArg::Object(ObjectArg::SharedObject {
                    id: *id,
                    initial_shared_version: *initial_shared_version,
                    mutable: *mutable,
                }))
                .unwrap(),
            BenchMoveCallArg::ImmOrOwnedObject(obj_ref) => {
                pt_builder.input((*obj_ref).into()).unwrap()
            }
            BenchMoveCallArg::ImmOrOwnedObjectVec(obj_refs) => pt_builder
                .make_obj_vec(obj_refs.iter().map(|q| ObjectArg::ImmOrOwnedObject(*q)))
                .unwrap(),
            BenchMoveCallArg::SharedObjectVec(obj_refs) => pt_builder
                .make_obj_vec(
                    obj_refs
                        .iter()
                        .map(
                            |(id, initial_shared_version, mutable)| ObjectArg::SharedObject {
                                id: *id,
                                initial_shared_version: *initial_shared_version,
                                mutable: *mutable,
                            },
                        ),
                )
                .unwrap(),
        })
        .collect()
}
