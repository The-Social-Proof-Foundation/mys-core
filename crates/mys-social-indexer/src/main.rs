// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

use mys_social_indexer::{
    api,
    blockchain::{
        // Import all handlers from the blockchain module
        BlockchainEventListener,
        ProfileEventListener,
        SocialGraphEventHandler,
        PlatformEventHandler,
        BlockListEventHandler,
        PostEventHandler,
        GovernanceEventHandler,
        MyIpEventHandler,
        SocialProofTokenHandler,
        SubscriptionEventHandler,
    },
    config::Config,
    db,
    set_mysocial_package_address,
    get_mysocial_package_address,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    info!("Starting MySocial indexer...");
    
    // Load config from environment
    let config = Config::from_env();
    
    // Set MySocial package address from environment variable if provided
    let env_var_names = ["MYSOCIAL_PACKAGE_ADDRESS", "PROFILE_PACKAGE_ADDRESS", "PLATFORM_PACKAGE_ADDRESS"];
    
    let mut address_set = false;
    for var_name in env_var_names {
        if let Ok(address) = std::env::var(var_name) {
            set_mysocial_package_address(address.clone());
            info!("Set MySocial package address to {} (from {})", address, var_name);
            address_set = true;
            break;
        }
    }
    
    if !address_set {
        info!("Using default MySocial package address: {}", get_mysocial_package_address());
    }
    
    // Run database migrations
    info!("Running database migrations...");
    if let Err(e) = db::run_migrations(&config) {
        error!("Failed to run migrations: {}", e);
        return Err(e);
    }
    
    // Set up database connection pool
    info!("Setting up database connection pool...");
    let db_pool = db::setup_connection_pool(&config).await?;
    
    // Create event channels
    let (profile_tx, profile_rx) = mpsc::channel(100);
    let (social_graph_tx, social_graph_rx) = mpsc::channel(100);
    let (platform_tx, platform_rx) = mpsc::channel(100);
    let (block_list_tx, block_list_rx) = mpsc::channel(100);
    let (post_tx, post_rx) = mpsc::channel(100);
    let (governance_tx, governance_rx) = mpsc::channel(100);
    let (my_ip_tx, my_ip_rx) = mpsc::channel(100);
    let (subscription_tx, subscription_rx) = mpsc::channel(100);
    
    // Create the blockchain event listener
    let blockchain_listener = Arc::new(BlockchainEventListener::new(config.clone(), db_pool.clone()));
    
    // Register event handlers
    blockchain_listener.register_event_handler(profile_tx).await;
    blockchain_listener.register_event_handler(social_graph_tx).await;
    blockchain_listener.register_event_handler(platform_tx).await;
    blockchain_listener.register_event_handler(block_list_tx).await;
    blockchain_listener.register_event_handler(post_tx).await;
    blockchain_listener.register_event_handler(governance_tx).await;
    blockchain_listener.register_event_handler(my_ip_tx).await;
    blockchain_listener.register_event_handler(subscription_tx).await;
    
    // Create and start profile event listener
    let mut profile_listener = ProfileEventListener::new(
        db_pool.clone(),
        profile_rx,
        "profile-worker".to_string(),
    );
    
    // Create and start social graph event handler
    let mut social_graph_handler = SocialGraphEventHandler::new(
        db_pool.clone(),
        social_graph_rx,
        "social-graph-worker".to_string(),
    );
    
    // Create and start platform event handler
    let mut platform_handler = PlatformEventHandler::new(
        db_pool.clone(),
        platform_rx,
        "platform-worker".to_string(),
    );
    
    // Create and start block list event handler
    let mut block_list_handler = BlockListEventHandler::new(
        db_pool.clone(),
        block_list_rx,
        "block-list-worker".to_string(),
    );
    
    // Create and start post event handler
    let mut post_handler = PostEventHandler::new(
        db_pool.clone(),
        post_rx,
        "post-worker".to_string(),
    );
    
    // Create and start governance event handler
    let mut governance_handler = GovernanceEventHandler::new(
        db_pool.clone(),
        governance_rx,
        "governance-worker".to_string(),
    );
    
    // Create and start MyIP event handler
    let mut my_ip_handler = MyIpEventHandler::new(
        db_pool.clone(),
        my_ip_rx,
        "my-ip-worker".to_string(),
    );
    
    // Create and start subscription event handler
    let mut subscription_handler = SubscriptionEventHandler::new(
        db_pool.clone(),
        subscription_rx,
        "subscription-worker".to_string(),
    );
    
    // Initialize the social proof token handler
    // Note: This handler has a different API pattern - it just needs a database connection
    let _social_proof_token_handler = SocialProofTokenHandler::new(db_pool.clone());
    
    let profile_handle = tokio::spawn(async move {
        if let Err(e) = profile_listener.start().await {
            error!("Profile event listener error: {}", e);
        }
    });
    
    let social_graph_handle = tokio::spawn(async move {
        if let Err(e) = social_graph_handler.start().await {
            error!("Social graph handler error: {}", e);
        }
    });
    
    let platform_handle = tokio::spawn(async move {
        if let Err(e) = platform_handler.start().await {
            error!("Platform handler error: {}", e);
        }
    });
    
    let block_list_handle = tokio::spawn(async move {
        if let Err(e) = block_list_handler.start().await {
            error!("Block list handler error: {}", e);
        }
    });
    
    let post_handle = tokio::spawn(async move {
        if let Err(e) = post_handler.start().await {
            error!("Post handler error: {}", e);
        }
    });
    
    let governance_handle = tokio::spawn(async move {
        if let Err(e) = governance_handler.start().await {
            error!("Governance handler error: {}", e);
        }
    });
    
    let my_ip_handle = tokio::spawn(async move {
        if let Err(e) = my_ip_handler.start().await {
            error!("MyIP marketplace handler error: {}", e);
        }
    });
    
    let subscription_handle = tokio::spawn(async move {
        if let Err(e) = subscription_handler.start().await {
            error!("Subscription handler error: {}", e);
        }
    });
    
    // Start the blockchain event listener
    let blockchain_handle = tokio::spawn({
        let listener = blockchain_listener.clone();
        async move {
            if let Err(e) = listener.start().await {
                error!("Blockchain event listener error: {}", e);
            }
        }
    });
    
    // Start the API server
    let api_handle = tokio::spawn(async move {
        if let Err(e) = api::start_api_server(db_pool, &config).await {
            error!("Failed to start API server: {}", e);
            std::process::exit(1);
        }
    });
    
    // Wait for all tasks to complete (they should run indefinitely)
    tokio::select! {
        _ = profile_handle => {
            error!("Profile event listener terminated unexpectedly");
        }
        _ = social_graph_handle => {
            error!("Social graph handler terminated unexpectedly");
        }
        _ = platform_handle => {
            error!("Platform handler terminated unexpectedly");
        }
        _ = block_list_handle => {
            error!("Block list handler terminated unexpectedly");
        }
        _ = post_handle => {
            error!("Post handler terminated unexpectedly");
        }
        _ = governance_handle => {
            error!("Governance handler terminated unexpectedly");
        }
        _ = my_ip_handle => {
            error!("MyIP marketplace handler terminated unexpectedly");
        }
        _ = subscription_handle => {
            error!("Subscription handler terminated unexpectedly");
        }
        _ = blockchain_handle => {
            error!("Blockchain event listener terminated unexpectedly");
        }
        _ = api_handle => {
            error!("API server terminated unexpectedly");
        }
    }
    
    info!("Indexer terminated");
    
    Ok(())
}