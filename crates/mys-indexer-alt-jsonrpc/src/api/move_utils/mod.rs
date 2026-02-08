// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use mys_json_rpc_types::MysMoveNormalizedFunction;
use mys_open_rpc::Module;
use mys_open_rpc_macros::open_rpc;
use mys_types::base_types::ObjectID;

use crate::api::rpc_module::RpcModule;
use crate::context::Context;

mod error;
mod response;

#[open_rpc(namespace = "mys", tag = "Move APIs")]
#[rpc(server, namespace = "mys")]
trait MoveApi {
    #[method(name = "getNormalizedMoveFunction")]
    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<MysMoveNormalizedFunction>;
}

pub(crate) struct MoveUtils(pub Context);

#[async_trait::async_trait]
impl MoveApiServer for MoveUtils {
    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<MysMoveNormalizedFunction> {
        let Self(ctx) = self;
        Ok(response::function(ctx, package, &module_name, &function_name).await?)
    }
}

impl RpcModule for MoveUtils {
    fn schema(&self) -> Module {
        MoveApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}
