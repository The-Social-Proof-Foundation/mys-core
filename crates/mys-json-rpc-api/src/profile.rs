// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use mys_json_rpc_types::ProfilePage;
use mys_json_rpc_types::ProfileData;
use mys_open_rpc_macros::open_rpc;
use mys_types::base_types::MysAddress;
use mys_types::base_types::ObjectID;

#[open_rpc(namespace = "mysx", tag = "Profile API")]
#[rpc(server, client, namespace = "mysx")]
pub trait ProfileReadApi {
    /// Get profile by owner address
    #[method(name = "getProfileByOwner")]
    async fn get_profile_by_owner(
        &self,
        /// The owner's Mys address
        owner: MysAddress,
    ) -> RpcResult<Option<ProfileData>>;

    /// Get profile by object ID
    #[method(name = "getProfileByID")]
    async fn get_profile_by_id(
        &self,
        /// The profile object ID
        profile_id: ObjectID,
    ) -> RpcResult<Option<ProfileData>>;

    /// Get profiles by username
    #[method(name = "getProfileByUsername")]
    async fn get_profile_by_username(
        &self,
        /// The username to search for
        username: String,
    ) -> RpcResult<Option<ProfileData>>;
    
    /// Return all profiles with optional pagination
    #[method(name = "getProfiles")]
    async fn get_profiles(
        &self,
        /// optional paging cursor
        cursor: Option<String>,
        /// maximum number of items per page
        limit: Option<usize>,
    ) -> RpcResult<ProfilePage>;
}