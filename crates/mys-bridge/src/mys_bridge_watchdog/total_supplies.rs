// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The MysBridgeStatus observable monitors whether the Mys Bridge is paused.

use crate::mys_bridge_watchdog::Observable;
use async_trait::async_trait;
use prometheus::IntGaugeVec;
use std::{collections::BTreeMap, sync::Arc};
use mys_sdk::MysClient;

use tokio::time::Duration;
use tracing::{error, info};

pub struct TotalSupplies {
    mys_client: Arc<MysClient>,
    coins: BTreeMap<String, String>,
    metric: IntGaugeVec,
}

impl TotalSupplies {
    pub fn new(
        mys_client: Arc<MysClient>,
        coins: BTreeMap<String, String>,
        metric: IntGaugeVec,
    ) -> Self {
        Self {
            mys_client,
            coins,
            metric,
        }
    }
}

#[async_trait]
impl Observable for TotalSupplies {
    fn name(&self) -> &str {
        "TotalSupplies"
    }

    async fn observe_and_report(&self) {
        for (coin_name, coin_type) in &self.coins {
            let resp = self
                .mys_client
                .coin_read_api()
                .get_total_supply(coin_type.clone())
                .await;
            match resp {
                Ok(supply) => {
                    self.metric
                        .with_label_values(&[coin_name])
                        .set(supply.value as i64);
                    info!("Total supply for {coin_type}: {}", supply.value);
                }
                Err(e) => {
                    error!("Error getting total supply for coin {coin_type}: {:?}", e);
                }
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
}
