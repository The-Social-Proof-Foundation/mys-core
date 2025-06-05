use serde::{Deserialize, Serialize};
use std::time::Duration;
use mys_config::Config;
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Invalid token address: {0}")]
    InvalidTokenAddress(String),
    #[error("Invalid price threshold: {0}")]
    InvalidPriceThreshold(String),
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    #[error("Environment variable error: {0}")]
    Environment(#[from] envy::Error),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GraphQLSource {
    pub url: String,
    pub token_address: String,
    pub pool_fee_tier: Option<u32>, // e.g., 3000 for 0.3%
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RestApiSource {
    pub url: String,
    pub json_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataSource {
    GraphQL(GraphQLSource),
    RestApi(RestApiSource),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthConfig {
    pub api_key: Option<String>,
    pub hmac_secret: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ValidationConfig {
    pub min_price_usd: Decimal,
    pub max_price_usd: Decimal,
    pub max_price_deviation_percent: Decimal, // e.g., 20.0 for 20%
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_price_usd: Decimal::from_parts(1, 0, 0, false, 6), // $0.000001
            max_price_usd: Decimal::from(1_000_000), // $1M
            max_price_deviation_percent: Decimal::from(50), // 50%
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MonitoringConfig {
    pub metrics_port: u16,
    pub health_check_port: u16,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_port: 9090,
            health_check_port: 8080,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PersistenceConfig {
    pub database_path: String,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            database_path: "./oracle_state.db".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PriceOracleConfig {
    pub server_url: String,
    pub chain_id: u64,
    pub token_id: u8,
    pub update_interval: Duration,
    pub price_change_threshold: Decimal,
    pub source: DataSource,
    pub auth: AuthConfig,
    pub retry: RetryConfig,
    pub validation: ValidationConfig,
    pub monitoring: MonitoringConfig,
    pub persistence: PersistenceConfig,
}

// Environment variable structure for Railway deployment
#[derive(Debug, Deserialize)]
struct OracleEnvVars {
    // Core configuration
    mys_oracle_server_url: String,
    mys_oracle_chain_id: u64,
    mys_oracle_token_id: u8,
    #[serde(deserialize_with = "deserialize_duration")]
    mys_oracle_update_interval: Duration,
    mys_oracle_price_change_threshold: Decimal,

    // Source configuration
    mys_oracle_source_type: String, // "graphql" or "rest_api"
    mys_oracle_source_url: String,
    mys_oracle_source_token_address: Option<String>,
    mys_oracle_source_pool_fee_tier: Option<u32>,
    mys_oracle_source_json_path: Option<String>,

    // Auth configuration
    mys_oracle_auth_api_key: Option<String>,
    mys_oracle_auth_hmac_secret: Option<String>,

    // Retry configuration
    mys_oracle_retry_max_attempts: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_duration", default)]
    mys_oracle_retry_initial_delay: Option<Duration>,
    #[serde(deserialize_with = "deserialize_optional_duration", default)]
    mys_oracle_retry_max_delay: Option<Duration>,
    mys_oracle_retry_multiplier: Option<f64>,

    // Validation configuration
    mys_oracle_validation_min_price_usd: Option<Decimal>,
    mys_oracle_validation_max_price_usd: Option<Decimal>,
    mys_oracle_validation_max_price_deviation_percent: Option<Decimal>,

    // Monitoring configuration
    mys_oracle_monitoring_metrics_port: Option<u16>,
    mys_oracle_monitoring_health_check_port: Option<u16>,

    // Persistence configuration
    mys_oracle_persistence_database_path: Option<String>,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    parse_duration(&s).map_err(serde::de::Error::custom)
}

fn deserialize_optional_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(s) => parse_duration(&s).map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    if s.ends_with("ms") {
        let num: u64 = s[..s.len()-2].parse().map_err(|_| format!("Invalid duration: {}", s))?;
        Ok(Duration::from_millis(num))
    } else if s.ends_with('s') {
        let num: u64 = s[..s.len()-1].parse().map_err(|_| format!("Invalid duration: {}", s))?;
        Ok(Duration::from_secs(num))
    } else if s.ends_with('m') {
        let num: u64 = s[..s.len()-1].parse().map_err(|_| format!("Invalid duration: {}", s))?;
        Ok(Duration::from_secs(num * 60))
    } else if s.ends_with('h') {
        let num: u64 = s[..s.len()-1].parse().map_err(|_| format!("Invalid duration: {}", s))?;
        Ok(Duration::from_secs(num * 3600))
    } else {
        Err(format!("Invalid duration format: {}", s))
    }
}

impl Config for PriceOracleConfig {}

impl PriceOracleConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let env_vars: OracleEnvVars = envy::from_env()?;

        // Build data source based on type
        let source = match env_vars.mys_oracle_source_type.as_str() {
            "graphql" => {
                DataSource::GraphQL(GraphQLSource {
                    url: env_vars.mys_oracle_source_url,
                    token_address: env_vars.mys_oracle_source_token_address
                        .ok_or_else(|| ConfigError::InvalidTokenAddress("GraphQL source requires token_address".to_string()))?,
                    pool_fee_tier: env_vars.mys_oracle_source_pool_fee_tier,
                })
            }
            "rest_api" => {
                DataSource::RestApi(RestApiSource {
                    url: env_vars.mys_oracle_source_url,
                    json_path: env_vars.mys_oracle_source_json_path
                        .ok_or_else(|| ConfigError::InvalidUrl("REST API source requires json_path".to_string()))?,
                })
            }
            _ => return Err(ConfigError::InvalidUrl(format!("Invalid source type: {}", env_vars.mys_oracle_source_type))),
        };

        let config = Self {
            server_url: env_vars.mys_oracle_server_url,
            chain_id: env_vars.mys_oracle_chain_id,
            token_id: env_vars.mys_oracle_token_id,
            update_interval: env_vars.mys_oracle_update_interval,
            price_change_threshold: env_vars.mys_oracle_price_change_threshold,
            source,
            auth: AuthConfig {
                api_key: env_vars.mys_oracle_auth_api_key,
                hmac_secret: env_vars.mys_oracle_auth_hmac_secret,
            },
            retry: RetryConfig {
                max_attempts: env_vars.mys_oracle_retry_max_attempts.unwrap_or(3),
                initial_delay: env_vars.mys_oracle_retry_initial_delay.unwrap_or(Duration::from_millis(100)),
                max_delay: env_vars.mys_oracle_retry_max_delay.unwrap_or(Duration::from_secs(30)),
                multiplier: env_vars.mys_oracle_retry_multiplier.unwrap_or(2.0),
            },
            validation: ValidationConfig {
                min_price_usd: env_vars.mys_oracle_validation_min_price_usd
                    .unwrap_or_else(|| Decimal::from_parts(1, 0, 0, false, 6)),
                max_price_usd: env_vars.mys_oracle_validation_max_price_usd
                    .unwrap_or_else(|| Decimal::from(1_000_000)),
                max_price_deviation_percent: env_vars.mys_oracle_validation_max_price_deviation_percent
                    .unwrap_or_else(|| Decimal::from(50)),
            },
            monitoring: MonitoringConfig {
                metrics_port: env_vars.mys_oracle_monitoring_metrics_port.unwrap_or(9090),
                health_check_port: env_vars.mys_oracle_monitoring_health_check_port
                    .or_else(|| std::env::var("PORT").ok().and_then(|p| p.parse().ok()))
                    .unwrap_or(8080),
            },
            persistence: PersistenceConfig {
                database_path: env_vars.mys_oracle_persistence_database_path
                    .unwrap_or_else(|| "./oracle_state.db".to_string()),
            },
        };

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate server URL
        if !self.server_url.starts_with("http://") && !self.server_url.starts_with("https://") {
            return Err(ConfigError::InvalidUrl(format!("Invalid server URL: {}", self.server_url)));
        }

        // Validate source URL
        match &self.source {
            DataSource::GraphQL(source) => {
                if !source.url.starts_with("http://") && !source.url.starts_with("https://") {
                    return Err(ConfigError::InvalidUrl(format!("Invalid GraphQL URL: {}", source.url)));
                }
                // Validate token address format (basic Ethereum address validation)
                if !source.token_address.starts_with("0x") || source.token_address.len() != 42 {
                    return Err(ConfigError::InvalidTokenAddress(source.token_address.clone()));
                }
            }
            DataSource::RestApi(source) => {
                if !source.url.starts_with("http://") && !source.url.starts_with("https://") {
                    return Err(ConfigError::InvalidUrl(format!("Invalid REST API URL: {}", source.url)));
                }
            }
        }

        // Validate price threshold
        if self.price_change_threshold <= Decimal::ZERO || self.price_change_threshold > Decimal::ONE {
            return Err(ConfigError::InvalidPriceThreshold(
                format!("Price threshold must be between 0 and 1, got: {}", self.price_change_threshold)
            ));
        }

        // Validate update interval
        if self.update_interval < Duration::from_secs(1) {
            return Err(ConfigError::InvalidDuration(
                "Update interval must be at least 1 second".to_string()
            ));
        }

        // Validate validation config
        if self.validation.min_price_usd >= self.validation.max_price_usd {
            return Err(ConfigError::InvalidPriceThreshold(
                "min_price_usd must be less than max_price_usd".to_string()
            ));
        }

        Ok(())
    }
}