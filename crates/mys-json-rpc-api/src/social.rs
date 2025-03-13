// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use mys_json_rpc_types::SocialValuePage;
use mys_json_rpc_types::SocialValueData;
use mys_open_rpc_macros::open_rpc;
use mys_types::base_types::MysAddress;
use mys_types::base_types::ObjectID;

#[open_rpc(namespace = "mys", tag = "Social API")]
#[rpc(server, client, namespace = "mys")]
pub trait SocialReadApi {
    /// Get social value by ID
    #[method(name = "social_getValueById")]
    async fn get_value_by_id(
        &self,
        /// The social value object ID
        value_id: ObjectID,
    ) -> RpcResult<Option<SocialValueData>>;

    /// Get social values by owner
    #[method(name = "social_getValuesByOwner")]
    async fn get_values_by_owner(
        &self,
        /// The owner's Mys address
        owner: MysAddress,
        /// Optional paging cursor
        cursor: Option<String>,
        /// Maximum number of items per page
        limit: Option<usize>,
    ) -> RpcResult<SocialValuePage>;
    
    /// Set a social value for a user
    #[method(name = "social_setValue")]
    async fn set_value(
        &self,
        /// The owner's Mys address
        owner: MysAddress,
        /// The value to set
        value: u64,
    ) -> RpcResult<bool>;
}