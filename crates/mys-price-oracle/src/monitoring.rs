use crate::metrics::OracleMetrics;
use crate::persistence::StateManager;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use prometheus::TextEncoder;
use serde_json::json;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

#[derive(Clone)]
pub struct MonitoringState {
    pub metrics: OracleMetrics,
    pub state_manager: Arc<StateManager>,
    pub startup_time: SystemTime,
}

pub struct MonitoringServer {
    state: MonitoringState,
    health_check_port: u16,
    metrics_port: u16,
}

impl MonitoringServer {
    pub fn new(
        metrics: OracleMetrics,
        state_manager: Arc<StateManager>,
        health_check_port: u16,
        metrics_port: u16,
    ) -> Self {
        let state = MonitoringState {
            metrics,
            state_manager,
            startup_time: SystemTime::now(),
        };

        Self {
            state,
            health_check_port,
            metrics_port,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start health check server
        let health_state = self.state.clone();
        let health_port = self.health_check_port;
        tokio::spawn(async move {
            if let Err(e) = Self::start_health_server(health_state, health_port).await {
                error!("Health check server error: {}", e);
            }
        });

        // Start metrics server
        let metrics_state = self.state.clone();
        let metrics_port = self.metrics_port;
        tokio::spawn(async move {
            if let Err(e) = Self::start_metrics_server(metrics_state, metrics_port).await {
                error!("Metrics server error: {}", e);
            }
        });

        info!(
            health_port = health_port,
            metrics_port = metrics_port,
            "Started monitoring servers"
        );

        Ok(())
    }

    async fn start_health_server(
        state: MonitoringState,
        port: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/ready", get(readiness_check))
            .route("/status", get(status_check))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
            .with_state(state);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("Health check server listening on port {}", port);
        axum::serve(listener, app).await?;
        Ok(())
    }

    async fn start_metrics_server(
        state: MonitoringState,
        port: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let app = Router::new()
            .route("/metrics", get(metrics_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
            .with_state(state);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("Metrics server listening on port {}", port);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// Health check endpoint - basic liveness check
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "timestamp": SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

// Readiness check - more comprehensive check
async fn readiness_check(State(state): State<MonitoringState>) -> impl IntoResponse {
    // Check if we can access the state database
    match state.state_manager.load_state() {
        Ok(oracle_state) => Json(json!({
            "status": "ready",
            "timestamp": SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "database_accessible": true,
            "current_nonce": oracle_state.nonce,
            "last_price": oracle_state.last_price
        }))
        .into_response(),
        Err(_) => {
            let response = Json(json!({
                "status": "not_ready",
                "timestamp": SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "database_accessible": false,
                "error": "Cannot access state database"
            }));
            (StatusCode::SERVICE_UNAVAILABLE, response).into_response()
        }
    }
}

// Status endpoint with detailed information
async fn status_check(State(state): State<MonitoringState>) -> impl IntoResponse {
    let current_time = SystemTime::now();
    let uptime = current_time
        .duration_since(state.startup_time)
        .unwrap()
        .as_secs();

    match state.state_manager.load_state() {
        Ok(oracle_state) => {
            let last_update_ago = oracle_state
                .last_updated
                .elapsed()
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let last_bridge_update_ago = oracle_state
                .last_successful_bridge_update
                .and_then(|t| t.elapsed().ok())
                .map(|d| d.as_secs());

            Json(json!({
                "status": "ok",
                "timestamp": current_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "uptime_seconds": uptime,
                "oracle_state": {
                    "nonce": oracle_state.nonce,
                    "last_price": oracle_state.last_price,
                    "last_update_ago_seconds": last_update_ago,
                    "last_bridge_update_ago_seconds": last_bridge_update_ago
                },
                "system_info": {
                    "rust_version": env!("CARGO_PKG_VERSION"),
                    "build_info": "production_build"
                }
            }))
            .into_response()
        }
        Err(e) => {
            let response = Json(json!({
                "status": "error",
                "timestamp": current_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "uptime_seconds": uptime,
                "error": format!("Database error: {}", e)
            }));
            (StatusCode::SERVICE_UNAVAILABLE, response).into_response()
        }
    }
}

// Prometheus metrics endpoint
async fn metrics_handler(State(state): State<MonitoringState>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();

    match encoder.encode_to_string(&metric_families) {
        Ok(output) => Response::builder()
            .header("content-type", "text/plain; version=0.0.4")
            .body(output)
            .unwrap(),
        Err(e) => {
            error!("Error encoding metrics: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Error encoding metrics: {}", e))
                .unwrap()
        }
    }
}
