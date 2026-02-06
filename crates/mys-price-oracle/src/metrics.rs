use prometheus::{Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry};
use std::sync::Arc;

#[derive(Clone)]
pub struct OracleMetrics {
    pub registry: Arc<Registry>,
    pub price_fetches_total: IntCounter,
    pub price_fetch_errors_total: IntCounter,
    pub bridge_updates_total: IntCounter,
    pub bridge_update_errors_total: IntCounter,
    pub current_price_usd: Gauge,
    pub last_update_timestamp: IntGauge,
    pub price_fetch_duration: Histogram,
    pub bridge_update_duration: Histogram,
    pub validation_errors_total: IntCounter,
}

impl OracleMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Arc::new(Registry::new());

        let price_fetches_total = IntCounter::with_opts(Opts::new(
            "oracle_price_fetches_total",
            "Total number of price fetch attempts",
        ))?;

        let price_fetch_errors_total = IntCounter::with_opts(Opts::new(
            "oracle_price_fetch_errors_total",
            "Total number of price fetch errors",
        ))?;

        let bridge_updates_total = IntCounter::with_opts(Opts::new(
            "oracle_bridge_updates_total",
            "Total number of bridge update attempts",
        ))?;

        let bridge_update_errors_total = IntCounter::with_opts(Opts::new(
            "oracle_bridge_update_errors_total",
            "Total number of bridge update errors",
        ))?;

        let current_price_usd = Gauge::with_opts(Opts::new(
            "oracle_current_price_usd",
            "Current price in USD",
        ))?;

        let last_update_timestamp = IntGauge::with_opts(Opts::new(
            "oracle_last_update_timestamp",
            "Timestamp of last successful price update",
        ))?;

        let price_fetch_duration = Histogram::with_opts(HistogramOpts::new(
            "oracle_price_fetch_duration_seconds",
            "Duration of price fetch operations in seconds",
        ))?;

        let bridge_update_duration = Histogram::with_opts(HistogramOpts::new(
            "oracle_bridge_update_duration_seconds",
            "Duration of bridge update operations in seconds",
        ))?;

        let validation_errors_total = IntCounter::with_opts(Opts::new(
            "oracle_validation_errors_total",
            "Total number of price validation errors",
        ))?;

        // Register all metrics
        registry.register(Box::new(price_fetches_total.clone()))?;
        registry.register(Box::new(price_fetch_errors_total.clone()))?;
        registry.register(Box::new(bridge_updates_total.clone()))?;
        registry.register(Box::new(bridge_update_errors_total.clone()))?;
        registry.register(Box::new(current_price_usd.clone()))?;
        registry.register(Box::new(last_update_timestamp.clone()))?;
        registry.register(Box::new(price_fetch_duration.clone()))?;
        registry.register(Box::new(bridge_update_duration.clone()))?;
        registry.register(Box::new(validation_errors_total.clone()))?;

        Ok(Self {
            registry,
            price_fetches_total,
            price_fetch_errors_total,
            bridge_updates_total,
            bridge_update_errors_total,
            current_price_usd,
            last_update_timestamp,
            price_fetch_duration,
            bridge_update_duration,
            validation_errors_total,
        })
    }

    pub fn record_price_fetch(&self) {
        self.price_fetches_total.inc();
    }

    pub fn record_price_fetch_error(&self) {
        self.price_fetch_errors_total.inc();
    }

    pub fn record_bridge_update(&self) {
        self.bridge_updates_total.inc();
    }

    pub fn record_bridge_update_error(&self) {
        self.bridge_update_errors_total.inc();
    }

    pub fn update_current_price(&self, price: f64) {
        self.current_price_usd.set(price);
    }

    pub fn update_last_update_timestamp(&self, timestamp: i64) {
        self.last_update_timestamp.set(timestamp);
    }

    pub fn record_validation_error(&self) {
        self.validation_errors_total.inc();
    }
}
