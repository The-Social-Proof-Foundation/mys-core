// Copyright (c) The Social Proof Foundation, LLC.
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
                    .unwrap_or_else(|_| "http://fullnode.testnet.mysocial.network:9000".to_string()),
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
        tracing::info!("🔍 Getting database URL...");
        
        // PRIORITY 1: Try Railway's provided DATABASE_URL first (now with password!)
        if let Ok(url) = env::var("DATABASE_URL") {
            tracing::info!("✅ Using Railway's DATABASE_URL");
            
            // Log masked URL for debugging
            let masked_url = if let Some(at_pos) = url.find('@') {
                let (before_at, after_at) = url.split_at(at_pos);
                if let Some(colon_pos) = before_at.rfind(':') {
                    format!("{}:****@{}", &before_at[..colon_pos], after_at)
                } else {
                    format!("postgres://user:****@{}", after_at)
                }
            } else {
                format!("{}...", &url[..20.min(url.len())])
            };
            tracing::info!("  DATABASE_URL (masked): {}", masked_url);
            
            // Validate that the URL contains authentication
            if !url.contains('@') {
                tracing::warn!("DATABASE_URL missing authentication credentials");
            } else if url.contains(":@") {
                tracing::warn!("DATABASE_URL appears to have empty password");
            } else {
                tracing::info!("DATABASE_URL appears to have authentication credentials ✅");
            }
            
            // Railway's DATABASE_URL should already include proper SSL configuration
            // Temporarily disable SSL to test basic connectivity
            if url.contains("?sslmode=require") {
                let no_ssl_url = url.replace("?sslmode=require", "?sslmode=disable");
                tracing::info!("Temporarily disabled SSL for testing basic connectivity");
                return no_ssl_url;
            } else if url.contains("sslmode=require") {
                let no_ssl_url = url.replace("sslmode=require", "sslmode=disable");
                tracing::info!("Temporarily disabled SSL for testing basic connectivity");
                return no_ssl_url;
            } else if !url.contains("sslmode") {
                let no_ssl_url = format!("{}?sslmode=disable", url);
                tracing::info!("Added SSL disabled for testing basic connectivity");
                return no_ssl_url;
            }
            
            return url;
        }
        
        // PRIORITY 2: Fallback to individual PostgreSQL environment variables
        if let (Ok(host), Ok(user), Ok(password), Ok(database)) = (
            env::var("PGHOST"),
            env::var("PGUSER"), 
            env::var("PGPASSWORD"),
            env::var("PGDATABASE")
        ) {
            tracing::info!("⬇️ Fallback: Using individual PostgreSQL environment variables");
            let port = env::var("PGPORT").unwrap_or_else(|_| "5432".to_string());
            
            if password.is_empty() {
                tracing::error!("PGPASSWORD is empty!");
            } else {
                let constructed_url = format!(
                    "postgres://{}:{}@{}:{}/{}?sslmode=disable",
                    user, password, host, port, database
                );
                tracing::info!("Constructed database URL with SSL disabled for testing");
                return constructed_url;
            }
        } else {
            tracing::info!("Individual PostgreSQL environment variables not complete");
        }
        
        // PRIORITY 3: Local development fallback
        tracing::warn!("⬇️ Using local development database URL");
        "postgres://postgres:postgres@localhost:5432/mys_social_indexer".to_string()
    }
}