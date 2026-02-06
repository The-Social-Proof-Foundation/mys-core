use crate::config::AuthConfig;
use crate::errors::OracleError;
use crate::metrics::OracleMetrics;
use reqwest::Client;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use sha2::Digest;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct BridgeClient {
    client: Client,
    server_url: String,
    auth: AuthConfig,
    metrics: OracleMetrics,
}

impl BridgeClient {
    pub fn new(server_url: String, auth: AuthConfig, metrics: OracleMetrics) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        Self {
            client,
            server_url,
            auth,
            metrics,
        }
    }

    pub async fn update_price(
        &self,
        chain_id: u8,
        nonce: u64,
        token_id: u8,
        price: Decimal,
    ) -> Result<(), OracleError> {
        let _timer = self.metrics.bridge_update_duration.start_timer();
        self.metrics.record_bridge_update();
        let correlation_id = Uuid::new_v4();

        // Convert price to appropriate format for bridge (maintaining precision)
        let price_scaled = self.scale_price_for_bridge(price)?;

        info!(
            correlation_id = %correlation_id,
            chain_id = chain_id,
            nonce = nonce,
            token_id = token_id,
            price = %price,
            price_scaled = price_scaled,
            "Sending price update to bridge"
        );

        let retry_strategy = ExponentialBackoff::from_millis(200)
            .max_delay(Duration::from_secs(60))
            .take(3);

        let result = Retry::spawn(retry_strategy, || async {
            self.send_price_update(correlation_id, chain_id, nonce, token_id, price_scaled)
                .await
        })
        .await;

        match &result {
            Ok(_) => {
                info!(
                    correlation_id = %correlation_id,
                    "Successfully sent price update to bridge"
                );
            }
            Err(e) => {
                error!(
                    correlation_id = %correlation_id,
                    error = %e,
                    "Failed to send price update to bridge"
                );
                self.metrics.record_bridge_update_error();
            }
        }

        result
    }

    async fn send_price_update(
        &self,
        correlation_id: Uuid,
        chain_id: u8,
        nonce: u64,
        token_id: u8,
        price_scaled: u64,
    ) -> Result<(), OracleError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let request_builder = if let Some(hmac_secret) = &self.auth.hmac_secret {
            // Use HMAC authentication
            let message = format!(
                "{}/{}/{}/{}/{}",
                chain_id, nonce, token_id, price_scaled, timestamp
            );
            let signature = self.compute_hmac_signature(&message, hmac_secret);

            let url = format!(
                "{}/sign/update_asset_price/{}/{}/{}/{}",
                self.server_url, chain_id, nonce, token_id, price_scaled
            );

            self.client
                .get(&url)
                .header("X-Timestamp", timestamp.to_string())
                .header("X-Signature", signature)
                .header("X-Correlation-ID", correlation_id.to_string())
        } else if let Some(api_key) = &self.auth.api_key {
            // Use API key authentication
            let url = format!(
                "{}/sign/update_asset_price/{}/{}/{}/{}",
                self.server_url, chain_id, nonce, token_id, price_scaled
            );

            self.client
                .get(&url)
                .header("X-API-Key", api_key)
                .header("X-Correlation-ID", correlation_id.to_string())
        } else {
            warn!(
                correlation_id = %correlation_id,
                "No authentication configured - this is insecure!"
            );

            // Fallback to original format (INSECURE - should not be used in production)
            let url = format!(
                "{}/sign/update_asset_price/{}/{}/{}/{}",
                self.server_url, chain_id, nonce, token_id, price_scaled
            );

            self.client
                .get(&url)
                .header("X-Correlation-ID", correlation_id.to_string())
        };

        let response = request_builder.send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            Err(OracleError::BridgeCommunication {
                status_code,
                message: body,
            })
        }
    }

    fn compute_hmac_signature(&self, message: &str, secret: &str) -> String {
        use sha2::Sha256;
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", message, secret));
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn scale_price_for_bridge(&self, price: Decimal) -> Result<u64, OracleError> {
        // Scale price to appropriate units for bridge (e.g., with 8 decimal places)
        // This maintains precision while converting to integer format
        let scaling_factor = Decimal::from(100_000_000u64); // 8 decimal places
        let scaled = price * scaling_factor;

        // Ensure the scaled price fits in u64
        if scaled > Decimal::from(u64::MAX) {
            return Err(OracleError::DataFormat(format!(
                "Price too large to represent: {}",
                price
            )));
        }

        // Ensure we don't lose precision due to negative values
        if scaled < Decimal::ZERO {
            return Err(OracleError::DataFormat(format!(
                "Price cannot be negative: {}",
                price
            )));
        }

        Ok(scaled.to_u64().unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::OracleMetrics;

    #[test]
    fn test_price_scaling() {
        let metrics = OracleMetrics::new().unwrap();
        let auth = AuthConfig {
            api_key: None,
            hmac_secret: None,
        };
        let client = BridgeClient::new("http://test".to_string(), auth, metrics);

        // Test normal price
        let price = Decimal::from_str_exact("1.23456789").unwrap();
        let scaled = client.scale_price_for_bridge(price).unwrap();
        assert_eq!(scaled, 123456789);

        // Test small price
        let price = Decimal::from_str_exact("0.00000001").unwrap();
        let scaled = client.scale_price_for_bridge(price).unwrap();
        assert_eq!(scaled, 1);

        // Test zero price
        let price = Decimal::ZERO;
        let scaled = client.scale_price_for_bridge(price).unwrap();
        assert_eq!(scaled, 0);
    }

    #[test]
    fn test_hmac_signature() {
        let metrics = OracleMetrics::new().unwrap();
        let auth = AuthConfig {
            api_key: None,
            hmac_secret: Some("test_secret".to_string()),
        };
        let client = BridgeClient::new("http://test".to_string(), auth, metrics);

        let signature = client.compute_hmac_signature("test_message", "test_secret");
        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64); // SHA256 produces 32 bytes = 64 hex chars
    }
}
