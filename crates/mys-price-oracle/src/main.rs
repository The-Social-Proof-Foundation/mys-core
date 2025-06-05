use clap::Parser;
use mys_config::Config;
use serde_json::Value;
use std::path::PathBuf;
use tokio::time::sleep;
use tracing::{error, info};

use reqwest::Client;

mod config;
use config::{DataSourceConfig, PriceOracleConfig};

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
struct Args {
    #[clap(long)]
    pub config_path: PathBuf,
}

async fn fetch_price(cfg: &DataSourceConfig) -> anyhow::Result<f64> {
    let resp = Client::new().get(&cfg.url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("request failed: {:?}", resp.status());
    }
    let json: Value = resp.json().await?;
    let data = jsonpath_lib::select(&json, &cfg.json_path)?;
    let first = data
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("no data for json path"))?;
    let as_str = first.as_str().ok_or_else(|| anyhow::anyhow!("invalid value"))?;
    let price = as_str.parse::<f64>()?;
    Ok(price)
}

async fn update_price(cfg: &PriceOracleConfig, nonce: u64, price: u64) -> anyhow::Result<()> {
    let url = format!(
        "{}/sign/update_asset_price/{}/{}/{}/{}",
        cfg.server_url, cfg.chain_id, nonce, cfg.token_id, price
    );
    let resp = Client::new().get(&url).send().await?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("server returned: {:?}", resp.status()))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cfg = PriceOracleConfig::load(&args.config_path)?;
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let mut nonce: u64 = 0;
    let mut last_price: Option<f64> = None;
    loop {
        match fetch_price(&cfg.source).await {
            Ok(price) => {
                let update = match last_price {
                    Some(prev) => (price - prev).abs() / prev > cfg.price_change_threshold,
                    None => true,
                };
                if update {
                    let price_int = price.round() as u64;
                    info!(price = price_int, "sending price update");
                    if let Err(e) = update_price(&cfg, nonce, price_int).await {
                        error!("failed to update price: {e}");
                    } else {
                        last_price = Some(price);
                        nonce += 1;
                    }
                }
            }
            Err(e) => error!("failed to fetch price: {e}"),
        }
        sleep(cfg.update_interval).await;
    }
}
