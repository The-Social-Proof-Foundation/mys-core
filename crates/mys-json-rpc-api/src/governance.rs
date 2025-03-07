// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use mys_json_rpc_types::{DelegatedStake, MysCommittee, ValidatorApys};
use mys_open_rpc_macros::open_rpc;
use mys_types::base_types::{ObjectID, MysAddress};
use mys_types::mys_serde::BigInt;
use mys_types::mys_system_state::mys_system_state_summary::MysSystemStateSummary;

#[open_rpc(namespace = "mysx", tag = "Governance Read API")]
#[rpc(server, client, namespace = "mysx")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_mys_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: MysAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<MysCommittee>;

    /// Return the latest MYS system state object on-chain.
    #[method(name = "getLatestMysSystemState")]
    async fn get_latest_mys_system_state(&self) -> RpcResult<MysSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
