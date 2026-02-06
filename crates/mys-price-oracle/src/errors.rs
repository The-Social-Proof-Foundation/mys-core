use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("GraphQL error: {0}")]
    GraphQL(String),

    #[error("State persistence error: {0}")]
    Persistence(#[from] sled::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError),

    #[error("Bridge communication error: {status_code}, {message}")]
    BridgeCommunication { status_code: u16, message: String },

    #[error("Price out of bounds: {price} (min: {min}, max: {max})")]
    PriceOutOfBounds {
        price: Decimal,
        min: Decimal,
        max: Decimal,
    },

    #[error("Price deviation too large: {current} -> {new} ({deviation}% change)")]
    PriceDeviationTooLarge {
        current: Decimal,
        new: Decimal,
        deviation: Decimal,
    },

    #[error("No price data available")]
    NoPriceData,

    #[error("Data format error: {0}")]
    DataFormat(String),
}
