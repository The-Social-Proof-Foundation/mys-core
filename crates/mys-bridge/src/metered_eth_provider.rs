// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::metrics::BridgeMetrics;
use ethers::providers::{Http, HttpClientError, JsonRpcClient, Provider};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use url::{ParseError, Url};

// Rate limit: Allow at most 10 concurrent requests and enforce minimum delay between requests
// Alchemy free tier allows ~330 compute units per second, so we limit to ~10 requests/second
// with some headroom. Each eth_getLogs call can consume 10-100+ compute units depending on range.
const MAX_CONCURRENT_REQUESTS: usize = 10;
const MIN_DELAY_BETWEEN_REQUESTS_MS: u64 = 100; // 100ms = ~10 requests/second max

#[derive(Debug, Clone)]
pub struct MeteredEthHttpProvier {
    inner: Http,
    metrics: Arc<BridgeMetrics>,
    // Semaphore to limit concurrent requests
    semaphore: Arc<Semaphore>,
    // Last request time for rate limiting
    last_request_time: Arc<tokio::sync::Mutex<std::time::Instant>>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl JsonRpcClient for MeteredEthHttpProvier {
    type Error = HttpClientError;

    async fn request<T: Serialize + Send + Sync + Debug, R: DeserializeOwned + Send>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, HttpClientError> {
        // Acquire semaphore permit to limit concurrent requests
        let _permit = self
            .semaphore
            .acquire()
            .await
            .expect("Semaphore should not be closed");
        
        // Rate limiting: ensure minimum delay between requests
        let mut last_request = self.last_request_time.lock().await;
        let elapsed = last_request.elapsed();
        if elapsed < Duration::from_millis(MIN_DELAY_BETWEEN_REQUESTS_MS) {
            let delay = Duration::from_millis(MIN_DELAY_BETWEEN_REQUESTS_MS) - elapsed;
            tokio::time::sleep(delay).await;
        }
        *last_request = std::time::Instant::now();
        drop(last_request);
        
        self.metrics
            .eth_rpc_queries
            .with_label_values(&[method])
            .inc();
        let _guard = self
            .metrics
            .eth_rpc_queries_latency
            .with_label_values(&[method])
            .start_timer();
        self.inner.request(method, params).await
    }
}

impl MeteredEthHttpProvier {
    pub fn new(url: impl Into<Url>, metrics: Arc<BridgeMetrics>) -> Self {
        let inner = Http::new(url);
        Self {
            inner,
            metrics,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS)),
            last_request_time: Arc::new(tokio::sync::Mutex::new(std::time::Instant::now())),
        }
    }
}

pub fn new_metered_eth_provider(
    url: &str,
    metrics: Arc<BridgeMetrics>,
) -> Result<Provider<MeteredEthHttpProvier>, ParseError> {
    let http_provider = MeteredEthHttpProvier::new(Url::parse(url)?, metrics);
    Ok(Provider::new(http_provider))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::Middleware;
    use prometheus::Registry;

    #[tokio::test]
    async fn test_metered_eth_provider() {
        let metrics = Arc::new(BridgeMetrics::new(&Registry::new()));
        let provider = new_metered_eth_provider("http://localhost:9876", metrics.clone()).unwrap();

        assert_eq!(
            metrics
                .eth_rpc_queries
                .get_metric_with_label_values(&["eth_blockNumber"])
                .unwrap()
                .get(),
            0
        );
        assert_eq!(
            metrics
                .eth_rpc_queries_latency
                .get_metric_with_label_values(&["eth_blockNumber"])
                .unwrap()
                .get_sample_count(),
            0
        );

        provider.get_block_number().await.unwrap_err(); // the rpc cal will fail but we don't care

        assert_eq!(
            metrics
                .eth_rpc_queries
                .get_metric_with_label_values(&["eth_blockNumber"])
                .unwrap()
                .get(),
            1
        );
        assert_eq!(
            metrics
                .eth_rpc_queries_latency
                .get_metric_with_label_values(&["eth_blockNumber"])
                .unwrap()
                .get_sample_count(),
            1
        );
    }
}
