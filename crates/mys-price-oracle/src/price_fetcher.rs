use crate::config::{DataSource, GraphQLSource, RestApiSource, ValidationConfig};
use crate::errors::OracleError;
use crate::metrics::OracleMetrics;
use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::time::Duration;
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::info;
use uuid::Uuid;

#[async_trait]
pub trait PriceFetcher: Send + Sync {
    async fn fetch_price(&self) -> Result<Decimal, OracleError>;
}

pub struct GraphQLPriceFetcher {
    client: Client,
    config: GraphQLSource,
    validation: ValidationConfig,
    metrics: OracleMetrics,
}

impl GraphQLPriceFetcher {
    pub fn new(
        config: GraphQLSource,
        validation: ValidationConfig,
        metrics: OracleMetrics,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            config,
            validation,
            metrics,
        }
    }

    async fn fetch_uniswap_v3_price(&self) -> Result<Decimal, OracleError> {
        let correlation_id = Uuid::new_v4();

        // For Base network, use the appropriate subgraph
        let query = json!({
            "query": r#"
                query GetTokenPrice($tokenAddress: String!) {
                    token(id: $tokenAddress) {
                        id
                        symbol
                        name
                        derivedETH
                    }
                    bundle(id: "1") {
                        ethPriceUSD
                    }
                }
            "#,
            "variables": {
                "tokenAddress": self.config.token_address.to_lowercase()
            }
        });

        info!(
            correlation_id = %correlation_id,
            token_address = %self.config.token_address,
            "Fetching price from Uniswap V3 subgraph"
        );

        let response = self
            .client
            .post(&self.config.url)
            .json(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OracleError::GraphQL(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let data: Value = response.json().await?;

        // Check for GraphQL errors
        if let Some(errors) = data.get("errors") {
            return Err(OracleError::GraphQL(format!("GraphQL errors: {}", errors)));
        }

        // Extract price data
        let token_data = data
            .get("data")
            .and_then(|d| d.get("token"))
            .ok_or_else(|| OracleError::NoPriceData)?;

        let bundle_data = data
            .get("data")
            .and_then(|d| d.get("bundle"))
            .ok_or_else(|| OracleError::NoPriceData)?;

        let derived_eth_str = token_data
            .get("derivedETH")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OracleError::DataFormat("derivedETH not found".to_string()))?;

        let eth_price_usd_str = bundle_data
            .get("ethPriceUSD")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OracleError::DataFormat("ethPriceUSD not found".to_string()))?;

        let derived_eth = Decimal::from_str_exact(derived_eth_str)
            .map_err(|e| OracleError::DataFormat(format!("Invalid derivedETH: {}", e)))?;

        let eth_price_usd = Decimal::from_str_exact(eth_price_usd_str)
            .map_err(|e| OracleError::DataFormat(format!("Invalid ethPriceUSD: {}", e)))?;

        let token_price_usd = derived_eth * eth_price_usd;

        info!(
            correlation_id = %correlation_id,
            token_price_usd = %token_price_usd,
            derived_eth = %derived_eth,
            eth_price_usd = %eth_price_usd,
            "Successfully fetched price from Uniswap V3"
        );

        Ok(token_price_usd)
    }
}

#[async_trait]
impl PriceFetcher for GraphQLPriceFetcher {
    async fn fetch_price(&self) -> Result<Decimal, OracleError> {
        let _timer = self.metrics.price_fetch_duration.start_timer();
        self.metrics.record_price_fetch();

        let retry_strategy = ExponentialBackoff::from_millis(100)
            .max_delay(Duration::from_secs(30))
            .take(3);

        let result = Retry::spawn(retry_strategy, || async {
            self.fetch_uniswap_v3_price().await
        })
        .await;

        match &result {
            Ok(price) => {
                // Validate price
                if *price < self.validation.min_price_usd || *price > self.validation.max_price_usd
                {
                    let error = OracleError::PriceOutOfBounds {
                        price: *price,
                        min: self.validation.min_price_usd,
                        max: self.validation.max_price_usd,
                    };
                    self.metrics.record_validation_error();
                    return Err(error);
                }

                self.metrics
                    .update_current_price(price.to_f64().unwrap_or(0.0));
                self.metrics.update_last_update_timestamp(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                );
            }
            Err(_) => {
                self.metrics.record_price_fetch_error();
            }
        }

        result
    }
}

pub struct RestApiPriceFetcher {
    client: Client,
    config: RestApiSource,
    validation: ValidationConfig,
    metrics: OracleMetrics,
}

impl RestApiPriceFetcher {
    pub fn new(
        config: RestApiSource,
        validation: ValidationConfig,
        metrics: OracleMetrics,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            config,
            validation,
            metrics,
        }
    }
}

#[async_trait]
impl PriceFetcher for RestApiPriceFetcher {
    async fn fetch_price(&self) -> Result<Decimal, OracleError> {
        let _timer = self.metrics.price_fetch_duration.start_timer();
        self.metrics.record_price_fetch();
        let correlation_id = Uuid::new_v4();

        info!(
            correlation_id = %correlation_id,
            url = %self.config.url,
            "Fetching price from REST API"
        );

        let retry_strategy = ExponentialBackoff::from_millis(100)
            .max_delay(Duration::from_secs(30))
            .take(3);

        let result = Retry::spawn(retry_strategy, || async {
            let response = self.client.get(&self.config.url).send().await?;

            if !response.status().is_success() {
                return Err(OracleError::Network(reqwest::Error::from(
                    response.error_for_status().unwrap_err(),
                )));
            }

            let json: Value = response.json().await?;
            let data = jsonpath_lib::select(&json, &self.config.json_path)
                .map_err(|e| OracleError::DataFormat(format!("JSONPath error: {}", e)))?;

            let first = data.get(0).ok_or_else(|| OracleError::NoPriceData)?;

            // Try to get price as number first, then as string
            let price = if let Some(num) = first.as_f64() {
                Decimal::from_f64(num)
                    .ok_or_else(|| OracleError::DataFormat("Invalid price number".to_string()))?
            } else if let Some(price_str) = first.as_str() {
                Decimal::from_str_exact(price_str)
                    .map_err(|e| OracleError::DataFormat(format!("Invalid price format: {}", e)))?
            } else {
                return Err(OracleError::DataFormat(
                    "Price is neither number nor string".to_string(),
                ));
            };

            info!(
                correlation_id = %correlation_id,
                price = %price,
                "Successfully fetched price from REST API"
            );

            Ok(price)
        })
        .await;

        match &result {
            Ok(price) => {
                // Validate price
                if *price < self.validation.min_price_usd || *price > self.validation.max_price_usd
                {
                    let error = OracleError::PriceOutOfBounds {
                        price: *price,
                        min: self.validation.min_price_usd,
                        max: self.validation.max_price_usd,
                    };
                    self.metrics.record_validation_error();
                    return Err(error);
                }

                self.metrics
                    .update_current_price(price.to_f64().unwrap_or(0.0));
                self.metrics.update_last_update_timestamp(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                );
            }
            Err(_) => {
                self.metrics.record_price_fetch_error();
            }
        }

        result
    }
}

pub fn create_price_fetcher(
    source: &DataSource,
    validation: ValidationConfig,
    metrics: OracleMetrics,
) -> Box<dyn PriceFetcher> {
    match source {
        DataSource::GraphQL(config) => Box::new(GraphQLPriceFetcher::new(
            config.clone(),
            validation,
            metrics,
        )),
        DataSource::RestApi(config) => Box::new(RestApiPriceFetcher::new(
            config.clone(),
            validation,
            metrics,
        )),
    }
}
