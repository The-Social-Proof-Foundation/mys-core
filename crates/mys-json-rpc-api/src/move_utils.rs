// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use mys_json_rpc_types::{
    MoveFunctionArgType, MysMoveNormalizedFunction, MysMoveNormalizedModule,
    MysMoveNormalizedStruct,
};
use mys_open_rpc_macros::open_rpc;
use mys_types::base_types::ObjectID;

#[open_rpc(namespace = "mys", tag = "Move Utils")]
#[rpc(server, client, namespace = "mys")]
pub trait MoveUtils {
    /// Return the argument types of a Move function,
    /// based on normalized Type.
    #[method(name = "getMoveFunctionArgTypes")]
    async fn get_move_function_arg_types(
        &self,
        package: ObjectID,
        module: String,
        function: String,
    ) -> RpcResult<Vec<MoveFunctionArgType>>;

    /// Return structured representations of all modules in the given package
    #[method(name = "getNormalizedMoveModulesByPackage")]
    async fn get_normalized_move_modules_by_package(
        &self,
        package: ObjectID,
    ) -> RpcResult<BTreeMap<String, MysMoveNormalizedModule>>;

    /// Return a structured representation of Move module
    #[method(name = "getNormalizedMoveModule")]
    async fn get_normalized_move_module(
        &self,
        package: ObjectID,
        module_name: String,
    ) -> RpcResult<MysMoveNormalizedModule>;

    /// Return a structured representation of Move struct
    #[method(name = "getNormalizedMoveStruct")]
    async fn get_normalized_move_struct(
        &self,
        package: ObjectID,
        module_name: String,
        struct_name: String,
    ) -> RpcResult<MysMoveNormalizedStruct>;

    /// Return a structured representation of Move function
    #[method(name = "getNormalizedMoveFunction")]
    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<MysMoveNormalizedFunction>;
}
