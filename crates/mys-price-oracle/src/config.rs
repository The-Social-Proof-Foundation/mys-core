use serde::{Deserialize, Serialize};
use std::time::Duration;
use mys_config::Config;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataSourceConfig {
    pub url: String,
    pub json_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PriceOracleConfig {
    pub server_url: String,
    pub chain_id: u8,
    pub token_id: u8,
    pub update_interval: Duration,
    pub price_change_threshold: f64,
    pub source: DataSourceConfig,
}

impl Config for PriceOracleConfig {}
