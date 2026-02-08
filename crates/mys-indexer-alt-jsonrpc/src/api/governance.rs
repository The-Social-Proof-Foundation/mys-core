// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::proc_macros::rpc;
use mys_indexer_alt_schema::schema::kv_epoch_starts;
use mys_json_rpc_api::GovernanceReadApiClient;
use mys_json_rpc_types::DelegatedStake;
use mys_json_rpc_types::ValidatorApys;
use mys_open_rpc::Module;
use mys_open_rpc_macros::open_rpc;
use mys_types::MYS_SYSTEM_STATE_OBJECT_ID;
use mys_types::TypeTag;
use mys_types::base_types::ObjectID;
use mys_types::base_types::MysAddress;
use mys_types::dynamic_field::Field;
use mys_types::dynamic_field::derive_dynamic_field_id;
use mys_types::mys_serde::BigInt;
use mys_types::mys_system_state::MysSystemStateTrait;
use mys_types::mys_system_state::MysSystemStateWrapper;
use mys_types::mys_system_state::mys_system_state_inner_v1::MysSystemStateInnerV1;
use mys_types::mys_system_state::mys_system_state_inner_v2::MysSystemStateInnerV2;
use mys_types::mys_system_state::mys_system_state_summary::MysSystemStateSummary;

use crate::api::rpc_module::RpcModule;
use crate::context::Context;
use crate::data::load_live_deserialized;
use crate::error::RpcError;
use crate::error::client_error_to_error_object;
use crate::error::rpc_bail;

#[open_rpc(namespace = "mysx", tag = "Governance API")]
#[rpc(server, namespace = "mysx")]
trait GovernanceApi {
    /// Return the reference gas price for the network as of the latest epoch.
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return a summary of the latest version of the Mys System State object (0x5), on-chain.
    #[method(name = "getLatestMysSystemState")]
    async fn get_latest_mys_system_state(&self) -> RpcResult<MysSystemStateSummary>;
}

#[open_rpc(namespace = "mysx", tag = "Delegation Governance API")]
#[rpc(server, namespace = "mysx")]
trait DelegationGovernanceApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_mys_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: MysAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}

pub(crate) struct Governance(pub Context);
pub(crate) struct DelegationGovernance(HttpClient);

impl DelegationGovernance {
    pub(crate) fn new(client: HttpClient) -> Self {
        Self(client)
    }
}

#[async_trait::async_trait]
impl GovernanceApiServer for Governance {
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        Ok(rgp_response(&self.0).await?)
    }

    async fn get_latest_mys_system_state(&self) -> RpcResult<MysSystemStateSummary> {
        Ok(latest_mys_system_state_response(&self.0).await?)
    }
}

#[async_trait::async_trait]
impl DelegationGovernanceApiServer for DelegationGovernance {
    async fn get_stakes_by_ids(
        &self,
        staked_mys_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        let Self(client) = self;

        client
            .get_stakes_by_ids(staked_mys_ids)
            .await
            .map_err(client_error_to_error_object)
    }

    async fn get_stakes(&self, owner: MysAddress) -> RpcResult<Vec<DelegatedStake>> {
        let Self(client) = self;

        client
            .get_stakes(owner)
            .await
            .map_err(client_error_to_error_object)
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        let Self(client) = self;

        client
            .get_validators_apy()
            .await
            .map_err(client_error_to_error_object)
    }
}

impl RpcModule for Governance {
    fn schema(&self) -> Module {
        GovernanceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

impl RpcModule for DelegationGovernance {
    fn schema(&self) -> Module {
        DelegationGovernanceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

/// Load data and generate response for `getReferenceGasPrice`.
async fn rgp_response(ctx: &Context) -> Result<BigInt<u64>, RpcError> {
    use kv_epoch_starts::dsl as e;

    let mut conn = ctx
        .pg_reader()
        .connect()
        .await
        .context("Failed to connect to the database")?;

    let rgp: i64 = conn
        .first(
            e::kv_epoch_starts
                .select(e::reference_gas_price)
                .order(e::epoch.desc()),
        )
        .await
        .context("Failed to fetch the reference gas price")?;

    Ok((rgp as u64).into())
}

/// Load data and generate response for `getLatestMysSystemState`.
async fn latest_mys_system_state_response(
    ctx: &Context,
) -> Result<MysSystemStateSummary, RpcError> {
    let wrapper: MysSystemStateWrapper = load_live_deserialized(ctx, MYS_SYSTEM_STATE_OBJECT_ID)
        .await
        .context("Failed to fetch system state wrapper object")?;

    let inner_id = derive_dynamic_field_id(
        MYS_SYSTEM_STATE_OBJECT_ID,
        &TypeTag::U64,
        &bcs::to_bytes(&wrapper.version).context("Failed to serialize system state version")?,
    )
    .context("Failed to derive inner system state field ID")?;

    Ok(match wrapper.version {
        1 => load_live_deserialized::<Field<u64, MysSystemStateInnerV1>>(ctx, inner_id)
            .await
            .context("Failed to fetch inner system state object")?
            .value
            .into_mys_system_state_summary(),
        2 => load_live_deserialized::<Field<u64, MysSystemStateInnerV2>>(ctx, inner_id)
            .await
            .context("Failed to fetch inner system state object")?
            .value
            .into_mys_system_state_summary(),
        v => rpc_bail!("Unexpected inner system state version: {v}"),
    })
}
