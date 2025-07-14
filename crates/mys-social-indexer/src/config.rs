// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub blockchain: BlockchainConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub poll_interval_ms: u64,
    pub batch_size: usize,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env file if present
        let _ = dotenv::dotenv();

        Config {
            database: DatabaseConfig {
                // Railway provides multiple ways to connect to PostgreSQL
                url: Self::get_database_url(),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            },
            server: ServerConfig {
                // Use 0.0.0.0 by default for containerized deployments (Railway, Docker, etc.)
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                // Railway provides PORT env var, fall back to SERVER_PORT, then default 8080
                port: env::var("PORT")
                    .or_else(|_| env::var("SERVER_PORT"))
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("PORT/SERVER_PORT must be a number"),
            },
            blockchain: BlockchainConfig {
                rpc_url: env::var("RPC_URL")
                    .unwrap_or_else(|_| "https://fullnode.testnet.mysocial.network:9000".to_string()),
                ws_url: env::var("WS_URL")
                    .unwrap_or_else(|_| "wss://fullnode.testnet.mysocial.network:9000".to_string()),
                poll_interval_ms: env::var("POLL_INTERVAL_MS")
                    .unwrap_or_else(|_| "5000".to_string()) // 5 seconds by default
                    .parse()
                    .expect("POLL_INTERVAL_MS must be a number"),
                batch_size: env::var("EVENT_BATCH_SIZE")
                    .unwrap_or_else(|_| "50".to_string()) // 50 events per batch by default
                    .parse()
                    .expect("EVENT_BATCH_SIZE must be a number"),
            },
        }
    }
    
    /// Get database URL with Railway PostgreSQL support and SSL configuration
    fn get_database_url() -> String {
        // Debug all environment variables first
        tracing::info!("🔍 Debugging database environment variables:");
        
        let db_env_vars = [
            "DATABASE_URL",
            "DATABASE_PRIVATE_URL", 
            "PGHOST",
            "PGPORT",
            "PGUSER",
            "PGPASSWORD",
            "PGDATABASE"
        ];
        
        for var_name in &db_env_vars {
            match env::var(var_name) {
                Ok(value) => {
                    let masked_value = if var_name.contains("PASSWORD") || var_name.contains("URL") {
                        if var_name.contains("URL") {
                            // Show more of the URL for debugging while masking password
                            if let Some(at_pos) = value.find('@') {
                                let (before_at, after_at) = value.split_at(at_pos);
                                if let Some(colon_pos) = before_at.rfind(':') {
                                    format!("{}:****@{}", &before_at[..colon_pos], after_at)
                                } else {
                                    format!("{}@{}", "postgres://user:****", after_at)
                                }
                            } else {
                                format!("{}...", &value[..20.min(value.len())])
                            }
                        } else {
                            "****".to_string()
                        }
                    } else {
                        value.clone()
                    };
                    tracing::info!("  {}: {}", var_name, masked_value);
                },
                Err(_) => {
                    tracing::info!("  {}: NOT_SET", var_name);
                }
            }
        }
        
        // PRIORITY 1: Try individual PostgreSQL environment variables first (most reliable)
        if let (Ok(host), Ok(user), Ok(password), Ok(database)) = (
            env::var("PGHOST"),
            env::var("PGUSER"), 
            env::var("PGPASSWORD"),
            env::var("PGDATABASE")
        ) {
            tracing::info!("✅ Using individual PostgreSQL environment variables (PGHOST, PGUSER, etc.)");
            let port = env::var("PGPORT").unwrap_or_else(|_| "5432".to_string());
            
            // Validate that we have all required components
            if password.is_empty() {
                tracing::error!("PGPASSWORD is empty!");
            } else {
                let constructed_url = format!(
                    "postgres://{}:{}@{}:{}/{}?sslmode=require",
                    user, password, host, port, database
                );
                
                tracing::info!("Constructed database URL from individual variables with SSL enabled");
                return constructed_url;
            }
        } else {
            tracing::info!("Individual PostgreSQL environment variables not complete:");
            tracing::info!("  PGHOST: {}", if env::var("PGHOST").is_ok() { "✅" } else { "❌" });
            tracing::info!("  PGUSER: {}", if env::var("PGUSER").is_ok() { "✅" } else { "❌" });
            tracing::info!("  PGPASSWORD: {}", if env::var("PGPASSWORD").is_ok() { "✅" } else { "❌" });
            tracing::info!("  PGDATABASE: {}", if env::var("PGDATABASE").is_ok() { "✅" } else { "❌" });
        }
        
        // PRIORITY 2: Try Railway's provided DATABASE_URL
        if let Ok(url) = env::var("DATABASE_URL") {
            tracing::info!("⬇️ Falling back to DATABASE_URL environment variable");
            
            // Validate that the URL contains authentication
            if !url.contains('@') {
                tracing::warn!("DATABASE_URL missing authentication credentials (no @ symbol)");
            } else if url.contains(":@") {
                tracing::warn!("DATABASE_URL appears to have empty password");
            } else {
                tracing::info!("DATABASE_URL appears to have authentication credentials");
                
                // Ensure SSL is enabled for Railway PostgreSQL
                if url.contains("railway.app") || url.contains("timescale") {
                    if !url.contains("sslmode") {
                        let ssl_url = format!("{}?sslmode=require", url);
                        tracing::info!("Added SSL requirement to Railway database URL");
                        return ssl_url;
                    }
                }
                return url;
            }
        }
        
        // PRIORITY 3: Try Railway's private database URL
        if let Ok(url) = env::var("DATABASE_PRIVATE_URL") {
            tracing::info!("⬇️ Falling back to DATABASE_PRIVATE_URL environment variable");
            
            if !url.contains("sslmode") {
                let ssl_url = format!("{}?sslmode=require", url);
                tracing::info!("Added SSL requirement to private database URL");
                return ssl_url;
            }
            return url;
        }
        
        // PRIORITY 4: Default fallback for local development
        tracing::warn!("⬇️ Using default local database URL as final fallback");
        "postgres://postgres:postgres@localhost:5432/mys_social_indexer".to_string()
    }
}