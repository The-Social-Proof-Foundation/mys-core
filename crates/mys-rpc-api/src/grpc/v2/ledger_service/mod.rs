// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::RpcService;
use mys_rpc::proto::mys::rpc::v2::BatchGetObjectsRequest;
use mys_rpc::proto::mys::rpc::v2::BatchGetObjectsResponse;
use mys_rpc::proto::mys::rpc::v2::BatchGetTransactionsRequest;
use mys_rpc::proto::mys::rpc::v2::BatchGetTransactionsResponse;
use mys_rpc::proto::mys::rpc::v2::GetCheckpointRequest;
use mys_rpc::proto::mys::rpc::v2::GetCheckpointResponse;
use mys_rpc::proto::mys::rpc::v2::GetEpochRequest;
use mys_rpc::proto::mys::rpc::v2::GetEpochResponse;
use mys_rpc::proto::mys::rpc::v2::GetObjectRequest;
use mys_rpc::proto::mys::rpc::v2::GetObjectResponse;
use mys_rpc::proto::mys::rpc::v2::GetServiceInfoRequest;
use mys_rpc::proto::mys::rpc::v2::GetServiceInfoResponse;
use mys_rpc::proto::mys::rpc::v2::GetTransactionRequest;
use mys_rpc::proto::mys::rpc::v2::GetTransactionResponse;
use mys_rpc::proto::mys::rpc::v2::ledger_service_server::LedgerService;

pub(crate) mod get_checkpoint;
mod get_epoch;
mod get_object;
mod get_service_info;
mod get_transaction;
pub use get_epoch::protocol_config_to_proto;
pub use get_object::validate_get_object_requests;

#[tonic::async_trait]
impl LedgerService for RpcService {
    async fn get_service_info(
        &self,
        _request: tonic::Request<GetServiceInfoRequest>,
    ) -> Result<tonic::Response<GetServiceInfoResponse>, tonic::Status> {
        get_service_info::get_service_info(self)
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_object(
        &self,
        request: tonic::Request<GetObjectRequest>,
    ) -> Result<tonic::Response<GetObjectResponse>, tonic::Status> {
        get_object::get_object(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn batch_get_objects(
        &self,
        request: tonic::Request<BatchGetObjectsRequest>,
    ) -> Result<tonic::Response<BatchGetObjectsResponse>, tonic::Status> {
        get_object::batch_get_objects(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_transaction(
        &self,
        request: tonic::Request<GetTransactionRequest>,
    ) -> Result<tonic::Response<GetTransactionResponse>, tonic::Status> {
        get_transaction::get_transaction(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn batch_get_transactions(
        &self,
        request: tonic::Request<BatchGetTransactionsRequest>,
    ) -> Result<tonic::Response<BatchGetTransactionsResponse>, tonic::Status> {
        get_transaction::batch_get_transactions(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_checkpoint(
        &self,
        request: tonic::Request<GetCheckpointRequest>,
    ) -> Result<tonic::Response<GetCheckpointResponse>, tonic::Status> {
        get_checkpoint::get_checkpoint(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_epoch(
        &self,
        request: tonic::Request<GetEpochRequest>,
    ) -> Result<tonic::Response<GetEpochResponse>, tonic::Status> {
        get_epoch::get_epoch(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
}
