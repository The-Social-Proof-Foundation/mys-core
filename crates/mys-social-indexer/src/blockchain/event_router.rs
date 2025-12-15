// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::blockchain::listener::BlockchainEvent;

/// Event pattern matching for routing
#[derive(Debug, Clone, PartialEq)]
pub enum EventPattern {
    /// Exact match on event type
    Exact(String),
    /// Pattern match (e.g., "::profile::" matches any profile event)
    Contains(String),
    /// Prefix match (e.g., starts with package address)
    StartsWith(String),
    /// Suffix match (e.g., ends with "Event")
    EndsWith(String),
    /// Module match (e.g., matches specific module within package)
    Module { package: String, module: String },
}

impl EventPattern {
    /// Check if this pattern matches the given event type
    pub fn matches(&self, event_type: &str) -> bool {
        match self {
            EventPattern::Exact(pattern) => event_type == pattern,
            EventPattern::Contains(pattern) => event_type.contains(pattern),
            EventPattern::StartsWith(pattern) => event_type.starts_with(pattern),
            EventPattern::EndsWith(pattern) => event_type.ends_with(pattern),
            EventPattern::Module { package, module } => {
                // Handle both full and short package address formats
                let package_matches = if package.len() > 10 && package.starts_with("0x") {
                    // Full format: extract last 4 hex characters (e.g., "0x...50c1" -> "0x50c1")
                    // Package format is "0x" + 64 hex chars, so last 4 chars are at index (len - 4)
                    let short_package = format!("0x{}", &package[package.len() - 4..]);
                    // Also try matching with just the last 4 hex digits without 0x prefix
                    let short_package_no_prefix = &package[package.len() - 4..];
                    event_type.starts_with(&short_package) 
                        || event_type.starts_with(package)
                        || event_type.starts_with(short_package_no_prefix)
                } else {
                    // Short format or other format
                    event_type.starts_with(package)
                };

                package_matches && event_type.contains(&format!("::{module}::"))
            }
        }
    }
}

/// Event handler registration
#[derive(Debug)]
pub struct EventHandlerRegistration {
    pub handler_name: String,
    pub patterns: Vec<EventPattern>,
    pub sender: mpsc::Sender<BlockchainEvent>,
    pub buffer_size: usize,
}

/// Centralized event router that distributes blockchain events to appropriate handlers
pub struct EventRouter {
    handlers: HashMap<String, EventHandlerRegistration>,
    metrics: EventRouterMetrics,
}

#[derive(Debug, Default)]
pub struct EventRouterMetrics {
    pub total_events_received: u64,
    pub total_events_routed: u64,
    pub events_dropped: u64,
    pub routing_errors: u64,
    pub handler_stats: HashMap<String, HandlerStats>,
}

#[derive(Debug, Default)]
pub struct HandlerStats {
    pub events_sent: u64,
    pub send_failures: u64,
    pub queue_full_drops: u64,
}

impl EventRouter {
    /// Create a new event router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            metrics: EventRouterMetrics::default(),
        }
    }

    /// Register a new event handler
    pub fn register_handler(
        &mut self,
        handler_name: String,
        patterns: Vec<EventPattern>,
        buffer_size: usize,
    ) -> mpsc::Receiver<BlockchainEvent> {
        let (sender, receiver) = mpsc::channel(buffer_size);

        let registration = EventHandlerRegistration {
            handler_name: handler_name.clone(),
            patterns,
            sender,
            buffer_size,
        };

        // Initialize handler stats
        self.metrics
            .handler_stats
            .insert(handler_name.clone(), HandlerStats::default());

        self.handlers.insert(handler_name.clone(), registration);

        info!(
            "Registered event handler '{}' with {} patterns and buffer size {}",
            handler_name,
            self.handlers[&handler_name].patterns.len(),
            buffer_size
        );

        receiver
    }

    /// Route an event to all matching handlers
    pub async fn route_event(&mut self, event: BlockchainEvent) -> Result<()> {
        self.metrics.total_events_received += 1;

        debug!(
            "Routing event: type={}, tx_digest={}, event_id={}",
            event.event_type, event.tx_digest, event.event_id
        );

        let mut routed_count = 0;
        let mut routing_errors = 0;

        // Find all handlers that should receive this event
        for (handler_name, registration) in &self.handlers {
            // Check if any pattern matches
            let matches = registration.patterns.iter().any(|pattern| {
                let result = pattern.matches(&event.event_type);

                // Enhanced logging for ReservationPoolCreatedEvent specifically
                if event.event_type.contains("ReservationPoolCreatedEvent") {
                    info!(
                        "🔍 RESERVATION POOL EVENT PATTERN CHECK: event='{}', handler='{}', pattern={:?}, matches={}",
                        event.event_type, handler_name, pattern, result
                    );

                    if let EventPattern::Module { package, module } = pattern {
                        let short_package = if package.len() > 10 && package.starts_with("0x") {
                            format!("0x{}", &package[package.len() - 4..])
                        } else {
                            package.clone()
                        };
                        info!(
                            "🔍 MODULE PATTERN DETAILS: full_package='{}', short_package='{}', module='{}', event_starts_with_short={}, event_contains_module={}",
                            package,
                            short_package,
                            module,
                            event.event_type.starts_with(&short_package),
                            event.event_type.contains(&format!("::{module}::"))
                        );
                    }
                }

                // Enhanced logging for profile events
                if event.event_type.contains("::profile::") {
                    info!(
                        "🔍 PROFILE EVENT PATTERN CHECK: event='{}', pattern={:?}, matches={}",
                        event.event_type, pattern, result
                    );

                    if let EventPattern::Module { package, module } = pattern {
                        let short_package = if package.len() > 10 && package.starts_with("0x") {
                            format!("0x{}", &package[package.len() - 4..])
                        } else {
                            package.clone()
                        };
                        info!(
                            "🔍 MODULE PATTERN DETAILS: full_package='{}', short_package='{}', module='{}', event_starts_with_short={}, event_contains_module={}",
                            package,
                            short_package,
                            module,
                            event.event_type.starts_with(&short_package),
                            event.event_type.contains(&format!("::{module}::"))
                        );
                    }
                }

                if result {
                    debug!(
                        "Event {} matches pattern {:?} for handler {}",
                        event.event_type, pattern, handler_name
                    );
                }
                result
            });

            if matches {
                // Get handler stats
                let handler_stats = self.metrics.handler_stats.get_mut(handler_name).unwrap();

                // Enhanced logging for ReservationPoolCreatedEvent routing
                if event.event_type.contains("ReservationPoolCreatedEvent") {
                    info!(
                        "✅ ROUTING ReservationPoolCreatedEvent to handler '{}'",
                        handler_name
                    );
                }

                // Try to send the event
                match registration.sender.try_send(event.clone()) {
                    Ok(_) => {
                        handler_stats.events_sent += 1;
                        routed_count += 1;
                        if event.event_type.contains("ReservationPoolCreatedEvent") {
                            info!(
                                "✅ Successfully sent ReservationPoolCreatedEvent to handler '{}'",
                                handler_name
                            );
                        } else {
                            debug!("Successfully routed event to handler '{}'", handler_name);
                        }
                    }
                    Err(mpsc::error::TrySendError::Full(_)) => {
                        handler_stats.queue_full_drops += 1;
                        self.metrics.events_dropped += 1;
                        warn!(
                            "Handler '{}' queue is full, dropping event {}",
                            handler_name, event.event_id
                        );
                    }
                    Err(mpsc::error::TrySendError::Closed(_)) => {
                        handler_stats.send_failures += 1;
                        routing_errors += 1;
                        error!(
                            "Handler '{}' channel is closed, cannot route event {}",
                            handler_name, event.event_id
                        );
                    }
                }
            }
        }

        self.metrics.total_events_routed += routed_count;
        self.metrics.routing_errors += routing_errors;

        if routed_count == 0 {
            debug!(
                "No handlers found for event type: {} (event_id: {})",
                event.event_type, event.event_id
            );
        } else {
            debug!(
                "Routed event {} to {} handlers",
                event.event_id, routed_count
            );
        }

        Ok(())
    }

    /// Get current routing metrics
    pub fn get_metrics(&self) -> &EventRouterMetrics {
        &self.metrics
    }

    /// Log current metrics
    pub fn log_metrics(&self) {
        info!("Event Router Metrics:");
        info!(
            "  Total events received: {}",
            self.metrics.total_events_received
        );
        info!(
            "  Total events routed: {}",
            self.metrics.total_events_routed
        );
        info!("  Events dropped: {}", self.metrics.events_dropped);
        info!("  Routing errors: {}", self.metrics.routing_errors);

        for (handler_name, stats) in &self.metrics.handler_stats {
            info!(
                "  Handler '{}': sent={}, failures={}, drops={}",
                handler_name, stats.events_sent, stats.send_failures, stats.queue_full_drops
            );
        }
    }

    /// Get list of registered handlers
    pub fn get_registered_handlers(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// Check if a handler is registered
    pub fn is_handler_registered(&self, handler_name: &str) -> bool {
        self.handlers.contains_key(handler_name)
    }
}

/// Helper functions for creating common event patterns
impl EventPattern {
    /// Create pattern for profile events
    pub fn profile_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "profile".to_string(),
        }
    }

    /// Create pattern for social graph events
    pub fn social_graph_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "social_graph".to_string(),
        }
    }

    /// Create pattern for platform events
    pub fn platform_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "platform".to_string(),
        }
    }

    /// Create pattern for post events
    pub fn post_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "post".to_string(),
        }
    }

    /// Create pattern for governance events
    pub fn governance_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "governance".to_string(),
        }
    }

    /// Create pattern for social proof token events
    pub fn social_proof_token_events(package_address: &str) -> Vec<EventPattern> {
        vec![
            EventPattern::Module {
                package: package_address.to_string(),
                module: "social_proof_token".to_string(),
            },
            EventPattern::Module {
                package: package_address.to_string(),
                module: "social_proof_tokens".to_string(), // Current module name
            },
            EventPattern::Module {
                package: package_address.to_string(),
                module: "token_exchange".to_string(),
            },
        ]
    }

    /// Create pattern for block list events
    pub fn block_list_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "block_list".to_string(),
        }
    }

    /// Create pattern for MyData events
    pub fn mydata_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "mydata".to_string(),
        }
    }

    /// Create pattern for subscription events
    pub fn subscription_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "subscription".to_string(),
        }
    }

    /// Create pattern for Social Proof of Truth (SPoT) events
    pub fn social_proof_of_truth_events(package_address: &str) -> EventPattern {
        EventPattern::Module {
            package: package_address.to_string(),
            module: "social_proof_of_truth".to_string(),
        }
    }

    /// Create pattern for PoC events
    pub fn poc_events(_package_address: &str) -> EventPattern {
        EventPattern::Contains("::poc::".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_pattern_matching() {
        let package_addr = "0x123";

        // Test exact match
        let exact = EventPattern::Exact("exact::match".to_string());
        assert!(exact.matches("exact::match"));
        assert!(!exact.matches("exact::nomatch"));

        // Test contains match
        let contains = EventPattern::Contains("::profile::".to_string());
        assert!(contains.matches("0x123::profile::ProfileCreatedEvent"));
        assert!(!contains.matches("0x123::platform::PlatformCreatedEvent"));

        // Test module match
        let module = EventPattern::Module {
            package: package_addr.to_string(),
            module: "profile".to_string(),
        };
        assert!(module.matches("0x123::profile::ProfileCreatedEvent"));
        assert!(!module.matches("0x456::profile::ProfileCreatedEvent")); // Wrong package
        assert!(!module.matches("0x123::platform::PlatformCreatedEvent")); // Wrong module

        // Test MyData events
        let mydata = EventPattern::mydata_events(package_addr);
        assert!(mydata.matches("0x123::mydata::DataCreatedEvent"));
        assert!(!mydata.matches("0x123::my_ip::IPRegisteredEvent"));

        // Test SPoT events
        let spot = EventPattern::social_proof_of_truth_events(package_addr);
        assert!(spot.matches("0x123::social_proof_of_truth::SpotBetPlacedEvent"));

        // Test PoC events (module match via substring)
        let poc = EventPattern::poc_events(package_addr);
        assert!(poc.matches("0x123::poc::PocBadgeIssuedEvent"));
    }

    #[tokio::test]
    async fn test_event_routing() {
        let mut router = EventRouter::new();

        // Register handler for profile events
        let profile_patterns = vec![EventPattern::Contains("::profile::".to_string())];
        let mut profile_rx =
            router.register_handler("profile-handler".to_string(), profile_patterns, 10);

        // Create test event
        let event = BlockchainEvent {
            tx_digest: "test_tx".to_string(),
            event_id: "test_event".to_string(),
            event_type: "0x123::profile::ProfileCreatedEvent".to_string(),
            data: serde_json::json!({}),
            timestamp_ms: 1234567890,
        };

        // Route the event
        router.route_event(event.clone()).await.unwrap();

        // Check that handler received the event
        let received_event = profile_rx.try_recv().unwrap();
        assert_eq!(received_event.event_type, event.event_type);
    }
}
