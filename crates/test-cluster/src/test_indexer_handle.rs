// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use std::path::PathBuf;
use std::time::Duration;
use mys_config::local_ip_utils::new_local_tcp_socket_for_testing_string;
use mys_indexer::test_utils::{
    start_indexer_jsonrpc_for_testing, start_indexer_writer_for_testing,
};
use mys_json_rpc_api::ReadApiClient;
use mys_pg_db::temp::TempDb;
use mys_sdk::{MysClient, MysClientBuilder};
use tempfile::TempDir;
use tokio::time::sleep;

pub(crate) struct IndexerHandle {
    pub(crate) rpc_client: HttpClient,
    pub(crate) mys_client: MysClient,
    pub(crate) rpc_url: String,
    #[allow(unused)]
    cancellation_tokens: Vec<tokio_util::sync::DropGuard>,
    #[allow(unused)]
    data_ingestion_dir: Option<TempDir>,
    #[allow(unused)]
    database: TempDb,
}

impl IndexerHandle {
    // TODO: this only starts indexer writer and reader (jsonrpc server) today.
    // Consider adding graphql server here as well.
    pub async fn new(
        fullnode_rpc_url: String,
        data_ingestion_dir: Option<TempDir>,
        data_ingestion_path: PathBuf,
    ) -> IndexerHandle {
        let mut cancellation_tokens = vec![];
        let database = TempDb::new().unwrap();
        let pg_address = database.database().url().as_str().to_owned();
        let indexer_jsonrpc_address = new_local_tcp_socket_for_testing_string();

        // Start indexer writer
        let (_, _, writer_token) = start_indexer_writer_for_testing(
            pg_address.clone(),
            None,
            None,
            Some(data_ingestion_path.clone()),
            None,
            None,
            None,
        )
        .await;
        cancellation_tokens.push(writer_token.drop_guard());

        // Start indexer jsonrpc service
        let (_, reader_token) = start_indexer_jsonrpc_for_testing(
            pg_address.clone(),
            fullnode_rpc_url,
            indexer_jsonrpc_address.clone(),
            None,
        )
        .await;
        cancellation_tokens.push(reader_token.drop_guard());

        let rpc_address = format!("http://{}", indexer_jsonrpc_address);

        let rpc_client = HttpClientBuilder::default().build(&rpc_address).unwrap();

        // Wait for the rpc client to be ready
        while rpc_client.get_chain_identifier().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }

        let mys_client = MysClientBuilder::default()
            .build(&rpc_address)
            .await
            .unwrap();

        IndexerHandle {
            rpc_client,
            mys_client,
            rpc_url: rpc_address.clone(),
            database,
            data_ingestion_dir,
            cancellation_tokens,
        }
    }
}
