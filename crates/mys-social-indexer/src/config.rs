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
        // Try Railway's provided DATABASE_URL first
        if let Ok(url) = env::var("DATABASE_URL") {
            // Ensure SSL is enabled for Railway PostgreSQL
            if url.contains("railway.app") && !url.contains("sslmode") {
                return format!("{}?sslmode=require", url);
            }
            return url;
        }
        
        // Try Railway's private database URL
        if let Ok(url) = env::var("DATABASE_PRIVATE_URL") {
            if !url.contains("sslmode") {
                return format!("{}?sslmode=require", url);
            }
            return url;
        }
        
        // Construct from individual PostgreSQL environment variables (Railway fallback)
        if let (Ok(host), Ok(user), Ok(password), Ok(database)) = (
            env::var("PGHOST"),
            env::var("PGUSER"), 
            env::var("PGPASSWORD"),
            env::var("PGDATABASE")
        ) {
            let port = env::var("PGPORT").unwrap_or_else(|_| "5432".to_string());
            return format!(
                "postgres://{}:{}@{}:{}/{}?sslmode=require",
                user, password, host, port, database
            );
        }
        
        // Default fallback for local development
        "postgres://postgres:postgres@localhost:5432/mys_social_indexer".to_string()
    }
}