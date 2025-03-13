// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use mys_json_rpc_api::SocialReadApiServer;
use mys_json_rpc_types::{SocialValueData, SocialValuePage};
use mys_sdk::MysClient;
use mys_types::base_types::{ObjectID, MysAddress};
use tracing::{debug, info};

use crate::metrics::SocialApiMetrics;

/// Implementation of the Social API.
pub struct SocialApi {
    client: Arc<MysClient>,
    metrics: SocialApiMetrics,
}

impl SocialApi {
    pub fn new(client: Arc<MysClient>, metrics: SocialApiMetrics) -> Self {
        Self { client, metrics }
    }

    pub fn new_for_testing(client: Arc<MysClient>) -> Self {
        Self {
            client,
            metrics: SocialApiMetrics::new_for_tests(),
        }
    }

    /// Create a JSON-RPC module with all the RPC handlers installed
    pub fn start_service(self) -> RpcModule<Self> {
        info!("Starting social RPC service");
        let mut module = RpcModule::new(self);
        
        // Register the module with the server
        module
    }
}

#[async_trait]
impl SocialReadApiServer for SocialApi {
    async fn get_value_by_id(&self, value_id: ObjectID) -> RpcResult<Option<SocialValueData>> {
        debug!("get_value_by_id called with ID: {}", value_id);
        
        let timer = self.metrics.get_value_by_id_latency.start_timer();
        self.metrics.get_value_by_id_calls.inc();
        
        // In a real implementation, we would fetch the object and convert it
        // For now, just return a mock response
        let result = Ok(Some(SocialValueData {
            value_id,
            version: mys_types::base_types::SequenceNumber::new(),
            digest: mys_types::base_types::ObjectDigest::random(),
            value: 42,
            owner: MysAddress::random_for_testing_only(),
            previous_transaction: mys_types::base_types::TransactionDigest::random(),
        }));
        
        timer.stop_and_record();
        result
    }

    async fn get_values_by_owner(
        &self, 
        owner: MysAddress,
        _cursor: Option<String>,
        limit: Option<usize>
    ) -> RpcResult<SocialValuePage> {
        debug!("get_values_by_owner called with owner: {}", owner);
        
        let timer = self.metrics.get_values_by_owner_latency.start_timer();
        self.metrics.get_values_by_owner_calls.inc();
        
        // In a real implementation, we would query for owned objects
        // For now, return mock data
        let _limit = limit.unwrap_or(10).min(50);
        let data = vec![
            SocialValueData {
                value_id: ObjectID::random(),
                version: mys_types::base_types::SequenceNumber::new(),
                digest: mys_types::base_types::ObjectDigest::random(),
                value: 42,
                owner,
                previous_transaction: mys_types::base_types::TransactionDigest::random(),
            }
        ];
        
        let result = Ok(SocialValuePage {
            data,
            next_cursor: None,
            has_next_page: false,
        });
        
        timer.stop_and_record();
        result
    }

    async fn set_value(&self, owner: MysAddress, value: u64) -> RpcResult<bool> {
        debug!("set_value called with owner: {} and value: {}", owner, value);
        
        let timer = self.metrics.set_value_latency.start_timer();
        self.metrics.set_value_calls.inc();
        
        // In a real implementation, we would:
        // 1. Build a transaction to call the create_value or update_value Move function
        // 2. Execute the transaction
        // 3. Return success/failure
        
        // For now, we just return success
        let result = Ok(true);
        
        timer.stop_and_record();
        result
    }
}