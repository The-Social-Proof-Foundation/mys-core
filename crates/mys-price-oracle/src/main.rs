use clap::Parser;
use mys_config::Config;
use rust_decimal::Decimal;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tokio::time::sleep;
use tracing::{error, info, warn};
use uuid::Uuid;

mod bridge_client;
mod config;
mod errors;
mod metrics;
mod monitoring;
mod persistence;
mod price_fetcher;

use bridge_client::BridgeClient;
use config::PriceOracleConfig;
use errors::OracleError;
use metrics::OracleMetrics;
use monitoring::MonitoringServer;
use persistence::StateManager;
use price_fetcher::{create_price_fetcher, PriceFetcher};

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
struct Args {
    #[clap(long, help = "Path to configuration file")]
    pub config_path: Option<PathBuf>,

    #[clap(
        long,
        help = "Use environment variables for configuration (Railway mode)"
    )]
    pub env: bool,

    #[clap(long, help = "Dry run mode - don't send updates to bridge")]
    pub dry_run: bool,

    #[clap(long, help = "Validate configuration and exit")]
    pub validate_config: bool,
}

struct PriceOracle {
    config: PriceOracleConfig,
    price_fetcher: Box<dyn PriceFetcher>,
    bridge_client: BridgeClient,
    state_manager: Arc<StateManager>,
    monitoring_server: MonitoringServer,
    dry_run: bool,
}

impl PriceOracle {
    pub async fn new(config: PriceOracleConfig, dry_run: bool) -> Result<Self, OracleError> {
        // Validate configuration
        config.validate()?;

        // Initialize metrics
        let metrics = OracleMetrics::new()
            .map_err(|e| OracleError::DataFormat(format!("Failed to initialize metrics: {}", e)))?;

        // Initialize state manager
        let state_manager = Arc::new(StateManager::new(&config.persistence.database_path)?);

        // Initialize price fetcher
        let price_fetcher =
            create_price_fetcher(&config.source, config.validation.clone(), metrics.clone());

        // Initialize bridge client
        let bridge_client = BridgeClient::new(
            config.server_url.clone(),
            config.auth.clone(),
            metrics.clone(),
        );

        // Initialize monitoring server
        let monitoring_server = MonitoringServer::new(
            metrics.clone(),
            state_manager.clone(),
            config.monitoring.health_check_port,
            config.monitoring.metrics_port,
        );

        Ok(Self {
            config,
            price_fetcher,
            bridge_client,
            state_manager,
            monitoring_server,
            dry_run,
        })
    }

    pub async fn run(&self) -> Result<(), OracleError> {
        info!("Starting price oracle");

        // Start monitoring servers
        self.monitoring_server.start().await.map_err(|e| {
            OracleError::DataFormat(format!("Failed to start monitoring server: {}", e))
        })?;

        // Load initial state
        let mut last_price: Option<Decimal> = self.state_manager.load_state()?.last_price;

        info!(
            chain_id = self.config.chain_id,
            token_id = self.config.token_id,
            update_interval = ?self.config.update_interval,
            price_threshold = %self.config.price_change_threshold,
            dry_run = self.dry_run,
            "Oracle configuration loaded"
        );

        // Main oracle loop
        loop {
            let iteration_id = Uuid::new_v4();

            match self
                .process_price_update(iteration_id, &mut last_price)
                .await
            {
                Ok(updated) => {
                    if updated {
                        info!(iteration_id = %iteration_id, "Price update completed successfully");
                    }
                }
                Err(e) => {
                    error!(
                        iteration_id = %iteration_id,
                        error = %e,
                        "Price update iteration failed"
                    );
                }
            }

            // Check for shutdown signal
            tokio::select! {
                _ = signal::ctrl_c() => {
                    info!("Received shutdown signal, stopping oracle");
                    break;
                }
                _ = sleep(self.config.update_interval) => {
                    // Continue to next iteration
                }
            }
        }

        info!("Price oracle stopped");
        Ok(())
    }

    async fn process_price_update(
        &self,
        iteration_id: Uuid,
        last_price: &mut Option<Decimal>,
    ) -> Result<bool, OracleError> {
        // Fetch current price
        let current_price = match self.price_fetcher.fetch_price().await {
            Ok(price) => {
                info!(
                    iteration_id = %iteration_id,
                    price = %price,
                    "Successfully fetched current price"
                );
                price
            }
            Err(e) => {
                warn!(
                    iteration_id = %iteration_id,
                    error = %e,
                    "Failed to fetch price, skipping update"
                );
                return Err(e);
            }
        };

        // Determine if we should send an update
        let should_update = match last_price {
            Some(prev_price) => {
                let price_change = self.calculate_price_change(*prev_price, current_price)?;
                let should_update = price_change > self.config.price_change_threshold;

                info!(
                    iteration_id = %iteration_id,
                    previous_price = %prev_price,
                    current_price = %current_price,
                    price_change_percent = %price_change,
                    threshold_percent = %self.config.price_change_threshold,
                    should_update = should_update,
                    "Price change analysis completed"
                );

                should_update
            }
            None => {
                info!(
                    iteration_id = %iteration_id,
                    current_price = %current_price,
                    "No previous price, sending initial update"
                );
                true
            }
        };

        if should_update {
            if self.dry_run {
                info!(
                    iteration_id = %iteration_id,
                    "DRY RUN: Would send price update to bridge"
                );
            } else {
                // Get next nonce and send update
                let nonce = self.state_manager.get_next_nonce()?;

                match self
                    .bridge_client
                    .update_price(
                        self.config.chain_id as u8,
                        nonce,
                        self.config.token_id,
                        current_price,
                    )
                    .await
                {
                    Ok(_) => {
                        // Update state after successful bridge update
                        self.state_manager.update_price(current_price)?;
                        self.state_manager.record_successful_bridge_update()?;
                        *last_price = Some(current_price);

                        info!(
                            iteration_id = %iteration_id,
                            nonce = nonce,
                            price = %current_price,
                            "Price update sent to bridge successfully"
                        );
                    }
                    Err(e) => {
                        error!(
                            iteration_id = %iteration_id,
                            nonce = nonce,
                            error = %e,
                            "Failed to send price update to bridge"
                        );
                        return Err(e);
                    }
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn calculate_price_change(
        &self,
        old_price: Decimal,
        new_price: Decimal,
    ) -> Result<Decimal, OracleError> {
        if old_price.is_zero() {
            return Ok(Decimal::ONE); // 100% change if old price was zero
        }

        let change = (new_price - old_price).abs() / old_price;

        // Validate the change isn't unreasonable
        if change > self.config.validation.max_price_deviation_percent / Decimal::from(100) {
            return Err(OracleError::PriceDeviationTooLarge {
                current: old_price,
                new: new_price,
                deviation: change * Decimal::from(100),
            });
        }

        Ok(change)
    }
}

fn load_configuration(args: &Args) -> Result<PriceOracleConfig, Box<dyn std::error::Error>> {
    if args.env {
        // Load from environment variables (Railway mode)
        info!("Loading configuration from environment variables");
        Ok(PriceOracleConfig::from_env()?)
    } else if let Some(config_path) = &args.config_path {
        // Load from config file
        info!("Loading configuration from file: {}", config_path.display());
        Ok(PriceOracleConfig::load(config_path)?)
    } else {
        Err("Either --config-path or --env must be specified".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize telemetry
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    // Load configuration based on mode
    let config = load_configuration(&args)?;

    // Validate configuration if requested
    if args.validate_config {
        match config.validate() {
            Ok(_) => {
                info!("Configuration validation passed");

                // Print some key configuration values for verification
                info!("Server URL: {}", config.server_url);
                info!("Chain ID: {}", config.chain_id);
                info!("Token ID: {}", config.token_id);
                info!("Update interval: {:?}", config.update_interval);
                info!("Price change threshold: {}", config.price_change_threshold);

                match &config.source {
                    config::DataSource::GraphQL(source) => {
                        info!("Using GraphQL source: {}", source.url);
                        info!("Token address: {}", source.token_address);
                    }
                    config::DataSource::RestApi(source) => {
                        info!("Using REST API source: {}", source.url);
                        info!("JSON path: {}", source.json_path);
                    }
                }

                return Ok(());
            }
            Err(e) => {
                error!("Configuration validation failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Create and run oracle
    match PriceOracle::new(config, args.dry_run).await {
        Ok(oracle) => {
            if let Err(e) = oracle.run().await {
                error!("Oracle execution failed: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to initialize oracle: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
