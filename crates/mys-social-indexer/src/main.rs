// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use rustls;
use std::sync::Arc;
use tracing::{error, info, warn};

use mys_social_indexer::{
    api,
    blockchain::{
        handler_trait::spawn_handler_task, BlockListEventHandler, BlockchainEventListener,
        EventPattern, EventRouter, GovernanceEventHandler, InsuranceEventHandler,
        MyDataEventHandler, PlatformEventHandler, PocEventHandler, PostEventHandler,
        ProfileEventListener, SocialGraphEventHandler, SocialProofOfTruthEventHandler,
        SocialProofTokenHandler, SubscriptionEventHandler,
    },
    config::Config,
    db::{self, ConnectionManager},
    get_mysocial_package_address, set_mysocial_package_address,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Install default crypto provider for rustls
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    info!("🚀 Starting MySocial Social Indexer with unified architecture...");

    // Log deployment environment info
    if let Ok(railway_env) = std::env::var("RAILWAY_ENVIRONMENT") {
        info!("🚂 Running on Railway environment: {}", railway_env);
    }
    if let Ok(railway_service) = std::env::var("RAILWAY_SERVICE_NAME") {
        info!("🚂 Railway service: {}", railway_service);
    }

    // Load configuration
    let config = Config::from_env();
    info!("📋 Configuration loaded successfully");
    info!("🔧 Server: {}:{}", config.server.host, config.server.port);
    info!(
        "🔧 Database max connections: {}",
        config.database.max_connections
    );

    // Set MySocial package address
    let env_var_names = [
        "MYSOCIAL_PACKAGE_ADDRESS",
        "PROFILE_PACKAGE_ADDRESS",
        "PLATFORM_PACKAGE_ADDRESS",
    ];
    let mut address_set = false;

    for var_name in env_var_names {
        if let Ok(address) = std::env::var(var_name) {
            set_mysocial_package_address(address.clone());
            info!("📍 Package address set to {} (from {})", address, var_name);
            address_set = true;
            break;
        }
    }

    if !address_set {
        info!(
            "📍 Using default package address: {}",
            get_mysocial_package_address()
        );
    }

    // Run database migrations
    info!("🗄️  Running database migrations...");
    if let Err(e) = db::run_migrations(&config) {
        error!("❌ Database migrations failed: {}", e);
        return Err(e);
    }
    info!("✅ Database migrations completed");

    // Set up database connection pool
    info!("🔗 Setting up database connection pool...");
    let db_pool = match db::setup_connection_pool(&config).await {
        Ok(pool) => {
            info!("✅ Database connection pool established");
            pool
        }
        Err(e) => {
            error!("❌ Failed to setup database connection pool: {}", e);
            return Err(e);
        }
    };

    // Create connection manager for standardized database access
    let connection_manager = Arc::new(ConnectionManager::new(db_pool.clone()));

    // Test database connectivity
    info!("🔍 Testing database connectivity...");
    if let Err(e) = connection_manager.health_check().await {
        error!("❌ Database health check failed: {}", e);
        return Err(e);
    }
    info!("✅ Database connectivity verified");

    // Create the event router
    let mut event_router = EventRouter::new();
    let package_address = get_mysocial_package_address();

    // Register handlers with the event router
    info!("📡 Registering event handlers...");

    // Profile handler
    let profile_patterns = vec![EventPattern::profile_events(package_address)];
    let profile_rx = event_router.register_handler(
        "profile-handler".to_string(),
        profile_patterns,
        1000, // Buffer size
    );

    // Social graph handler
    let social_graph_patterns = vec![EventPattern::social_graph_events(package_address)];
    let social_graph_rx = event_router.register_handler(
        "social-graph-handler".to_string(),
        social_graph_patterns,
        1000,
    );

    // Platform handler
    let platform_patterns = vec![EventPattern::platform_events(package_address)];
    let platform_rx =
        event_router.register_handler("platform-handler".to_string(), platform_patterns, 1000);

    // Block list handler
    let block_list_patterns = vec![EventPattern::block_list_events(package_address)];
    let block_list_rx =
        event_router.register_handler("block-list-handler".to_string(), block_list_patterns, 1000);

    // Post handler
    let post_patterns = vec![EventPattern::post_events(package_address)];
    let post_rx = event_router.register_handler("post-handler".to_string(), post_patterns, 1000);

    // Governance handler
    let governance_patterns = vec![EventPattern::governance_events(package_address)];
    let governance_rx =
        event_router.register_handler("governance-handler".to_string(), governance_patterns, 1000);

    // MyData handler
    let mydata_patterns = vec![EventPattern::mydata_events(package_address)];
    let mydata_rx =
        event_router.register_handler("mydata-handler".to_string(), mydata_patterns, 1000);

    // Subscription handler
    let subscription_patterns = vec![EventPattern::subscription_events(package_address)];
    let subscription_rx = event_router.register_handler(
        "subscription-handler".to_string(),
        subscription_patterns,
        1000,
    );

    // Social Proof Token handler (new unified architecture)
    let spt_patterns = EventPattern::social_proof_token_events(package_address);
    let spt_rx =
        event_router.register_handler("social-proof-token-handler".to_string(), spt_patterns, 1000);

    // Social Proof of Truth (SPoT) handler
    let spot_patterns = vec![EventPattern::social_proof_of_truth_events(package_address)];
    let spot_rx = event_router.register_handler("spot-handler".to_string(), spot_patterns, 1000);

    // Proof of Creativity handler
    let poc_patterns = vec![EventPattern::poc_events(package_address)];
    let poc_rx = event_router.register_handler("poc-handler".to_string(), poc_patterns, 1000);

    // Insurance handler
    let insurance_patterns = vec![EventPattern::insurance_events(package_address)];
    let insurance_rx = event_router.register_handler("insurance-handler".to_string(), insurance_patterns, 1000);

    info!("✅ All event handlers registered successfully");

    // Create handler instances
    let profile_handler =
        ProfileEventListener::new(db_pool.clone(), profile_rx, "profile-worker".to_string());

    let social_graph_handler = SocialGraphEventHandler::new(
        db_pool.clone(),
        social_graph_rx,
        "social-graph-worker".to_string(),
    );

    let platform_handler =
        PlatformEventHandler::new(db_pool.clone(), platform_rx, "platform-worker".to_string());

    let block_list_handler = BlockListEventHandler::new(
        db_pool.clone(),
        block_list_rx,
        "block-list-worker".to_string(),
    );

    let post_handler = PostEventHandler::new(db_pool.clone(), post_rx, "post-worker".to_string());

    let governance_handler = GovernanceEventHandler::new(
        db_pool.clone(),
        governance_rx,
        "governance-worker".to_string(),
    );

    let mydata_handler =
        MyDataEventHandler::new(db_pool.clone(), mydata_rx, "mydata-worker".to_string());

    let subscription_handler = SubscriptionEventHandler::new(
        db_pool.clone(),
        subscription_rx,
        "subscription-worker".to_string(),
    );

    // Create new SPT handler with unified architecture
    let spt_handler = SocialProofTokenHandler::new(db_pool.clone());

    let spot_handler =
        SocialProofOfTruthEventHandler::new(db_pool.clone(), spot_rx, "spot-worker".to_string());

    let poc_handler = PocEventHandler::new(db_pool.clone(), poc_rx, "poc-worker".to_string());

    let insurance_handler = InsuranceEventHandler::new(
        db_pool.clone(),
        insurance_rx,
        "insurance-worker".to_string(),
    );

    // Spawn handler tasks
    info!("🔄 Starting event handler tasks...");

    let profile_task = tokio::spawn(async move {
        let mut handler = profile_handler;
        if let Err(e) = handler.start().await {
            error!("Profile handler error: {}", e);
        }
    });

    let social_graph_task = tokio::spawn(async move {
        let mut handler = social_graph_handler;
        if let Err(e) = handler.start().await {
            error!("Social graph handler error: {}", e);
        }
    });

    let platform_task = tokio::spawn(async move {
        let mut handler = platform_handler;
        if let Err(e) = handler.start().await {
            error!("Platform handler error: {}", e);
        }
    });

    let block_list_task = tokio::spawn(async move {
        let mut handler = block_list_handler;
        if let Err(e) = handler.start().await {
            error!("Block list handler error: {}", e);
        }
    });

    let post_task = tokio::spawn(async move {
        let mut handler = post_handler;
        if let Err(e) = handler.start().await {
            error!("Post handler error: {}", e);
        }
    });

    let governance_task = tokio::spawn(async move {
        let mut handler = governance_handler;
        if let Err(e) = handler.start().await {
            error!("Governance handler error: {}", e);
        }
    });

    let mydata_task = tokio::spawn(async move {
        let mut handler = mydata_handler;
        if let Err(e) = handler.start().await {
            error!("MyData handler error: {}", e);
        }
    });

    let subscription_task = tokio::spawn(async move {
        let mut handler = subscription_handler;
        if let Err(e) = handler.start().await {
            error!("Subscription handler error: {}", e);
        }
    });

    // Spawn SPT handler with new architecture
    let spt_task = spawn_handler_task(spt_handler, spt_rx);

    let spot_task = tokio::spawn(async move {
        let mut handler = spot_handler;
        if let Err(e) = handler.start().await {
            error!("SPoT handler error: {}", e);
        }
    });

    let poc_task = tokio::spawn(async move {
        let mut handler = poc_handler;
        if let Err(e) = handler.start().await {
            error!("PoC handler error: {}", e);
        }
    });

    let insurance_task = tokio::spawn(async move {
        let mut handler = insurance_handler;
        if let Err(e) = handler.start().await {
            error!("Insurance handler error: {}", e);
        }
    });

    info!("✅ All handler tasks started");

    // Create blockchain event listener
    let blockchain_listener = Arc::new(BlockchainEventListener::new(
        config.clone(),
        db_pool.clone(),
    ));

    // Test blockchain connectivity
    info!("🔗 Testing blockchain connectivity...");
    match blockchain_listener.test_connectivity().await {
        Ok(_) => info!("✅ Blockchain connectivity test passed"),
        Err(e) => {
            error!("❌ Blockchain connectivity test failed: {}", e);
            warn!("Blockchain events may not work, but API server will continue");
        }
    }

    // Start the event router with blockchain listener
    let event_router_arc = Arc::new(tokio::sync::Mutex::new(event_router));
    let blockchain_task = tokio::spawn({
        let listener = blockchain_listener.clone();
        let router = event_router_arc.clone();
        async move {
            // Register the router as an event handler for the blockchain listener
            let (router_tx, mut router_rx) = tokio::sync::mpsc::channel(10000);
            listener.register_event_handler(router_tx).await;

            // Start the blockchain listener
            let listener_task = tokio::spawn(async move {
                loop {
                    match listener.start().await {
                        Ok(_) => {
                            info!("Blockchain listener completed normally");
                            break;
                        }
                        Err(e) => {
                            error!("Blockchain listener error: {}", e);
                            warn!("Retrying blockchain connection in 30 seconds...");
                            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                        }
                    }
                }
            });

            // Event routing loop
            let routing_task = tokio::spawn(async move {
                while let Some(event) = router_rx.recv().await {
                    let mut router_guard = router.lock().await;
                    if let Err(e) = router_guard.route_event(event).await {
                        error!("Event routing error: {}", e);
                    }

                    // Log metrics every 1000 events
                    if router_guard.get_metrics().total_events_received % 1000 == 0
                        && router_guard.get_metrics().total_events_received > 0
                    {
                        router_guard.log_metrics();
                    }
                }
            });

            // Wait for either task to complete
            tokio::select! {
                _ = listener_task => {
                    warn!("Blockchain listener task completed");
                }
                _ = routing_task => {
                    warn!("Event routing task completed");
                }
            }
        }
    });

    // Start the API server
    info!("🌐 Starting API server...");
    let api_task = tokio::spawn(async move {
        if let Err(e) = api::start_api_server(db_pool, &config).await {
            error!("❌ API server failed: {}", e);
            std::process::exit(1);
        }
    });

    info!("🎉 MySocial Social Indexer started successfully!");
    info!("📊 Event routing metrics will be logged every 1000 events");

    // Wait for the API server (main service)
    match api_task.await {
        Ok(_) => info!("API server completed"),
        Err(e) => error!("Failed to join API server task: {}", e),
    }

    // Graceful shutdown of all tasks
    info!("🛑 Initiating graceful shutdown...");

    profile_task.abort();
    social_graph_task.abort();
    platform_task.abort();
    block_list_task.abort();
    post_task.abort();
    governance_task.abort();
    mydata_task.abort();
    subscription_task.abort();
    spt_task.abort();
    spot_task.abort();
    poc_task.abort();
    insurance_task.abort();
    blockchain_task.abort();

    // Log final metrics
    {
        let router_guard = event_router_arc.lock().await;
        info!("📊 Final event routing metrics:");
        router_guard.log_metrics();
    }

    info!("✅ MySocial Social Indexer shutdown complete");
    Ok(())
}
