// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The MysBridgeStatus observable monitors whether the Mys Bridge is paused.

use crate::mys_bridge_watchdog::Observable;
use crate::mys_client::MysBridgeClient;
use async_trait::async_trait;
use prometheus::IntGauge;
use std::sync::Arc;

use tokio::time::Duration;
use tracing::{error, info};

pub struct MysBridgeStatus {
    mys_client: Arc<MysBridgeClient>,
    metric: IntGauge,
}

impl MysBridgeStatus {
    pub fn new(mys_client: Arc<MysBridgeClient>, metric: IntGauge) -> Self {
        Self { mys_client, metric }
    }
}

#[async_trait]
impl Observable for MysBridgeStatus {
    fn name(&self) -> &str {
        "MysBridgeStatus"
    }

    async fn observe_and_report(&self) {
        let status = self.mys_client.is_bridge_paused().await;
        match status {
            Ok(status) => {
                self.metric.set(status as i64);
                info!("Mys Bridge Status: {:?}", status);
            }
            Err(e) => {
                error!("Error getting mys bridge status: {:?}", e);
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
}
