// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mys_macros::sim_test;
use mys_rpc_api::proto::node::v2alpha::node_service_client::NodeServiceClient;
use mys_rpc_api::proto::node::v2alpha::GetCoinInfoRequest;
use mys_rpc_api::proto::node::v2alpha::GetCoinInfoResponse;
use mys_sdk_types::TypeTag;
use test_cluster::TestClusterBuilder;

#[sim_test]
async fn get_coin_info() {
    let test_cluster = TestClusterBuilder::new().build().await;

    let mut grpc_client = NodeServiceClient::connect(test_cluster.rpc_url().to_owned())
        .await
        .unwrap();

    let coin_type_sdk: TypeTag = "0x2::mys::MYS".parse().unwrap();
    let request = GetCoinInfoRequest {
        coin_type: Some(coin_type_sdk.clone().into()),
    };

    let GetCoinInfoResponse {
        coin_type,
        metadata,
        treasury,
    } = grpc_client
        .get_coin_info(request)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(coin_type, Some(coin_type_sdk.into()));
    assert_eq!(metadata.unwrap().symbol, Some("MYS".to_owned()));
    assert_eq!(
        treasury.unwrap().total_supply,
        Some(mys_types::gas_coin::TOTAL_SUPPLY_MIST)
    );
}
