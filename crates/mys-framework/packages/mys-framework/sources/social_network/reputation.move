// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Reputation system for MySocial platforms and users.
/// This module tracks reputation scores for platforms and users,
/// and provides mechanisms for reputation changes based on behavior.
module mys::reputation {
    use std::string::{Self, String};
    use std::vector;
    use mys::object::{Self, UID, ID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::table::{Self, Table};
    
    use mys::platform::{Self, Platform, PlatformStats};
    use mys::profile_platform::{Self, ProfilePlatformLink};
    
    // === Errors ===
    /// Unauthorized operation
    const EUnauthorized: u64 = 0;
    /// Entity not found
    const ENotFound: u64 = 1;
    /// Invalid reputation score
    const EInvalidScore: u64 = 2;
    /// Invalid operation
    const EInvalidOperation: u64 = 3;
    
    // === Constants ===
    // Minimum reputation score
    const MIN_REPUTATION_SCORE: u64 = 1;
    // Maximum reputation score
    const MAX_REPUTATION_SCORE: u64 = 1000;
    // Default starting reputation
    const DEFAULT_REPUTATION: u64 = 500;
    
    // Reputation change reasons
    const REASON_POST_ENGAGEMENT: u8 = 0;
    const REASON_REPORT: u8 = 1;
    const REASON_PLATFORM_GROWTH: u8 = 2;
    const REASON_ADVERTISING_COMPLIANCE: u8 = 3;
    const REASON_ADMIN_ADJUSTMENT: u8 = 4;
    
    // === Structs ===
    
    /// Admin capability for the reputation system
    struct AdminCap has key, store {
        id: UID,
    }
    
    /// Reputation system configuration
    struct ReputationConfig has key {
        id: UID,
        // Weights for different reputation factors (out of 100)
        engagement_weight: u64,      // Weight for user engagement
        report_weight: u64,          // Weight for user reports
        growth_weight: u64,          // Weight for platform growth
        compliance_weight: u64,      // Weight for advertising compliance
        // Thresholds
        min_reports_threshold: u64,  // Minimum reports to trigger automatic penalty
        max_daily_increase: u64,     // Maximum reputation increase per day
        cooldown_period_ms: u64,     // Cooldown between reputation changes (in milliseconds)
    }
    
    /// Registry for reputation tracking
    struct ReputationRegistry has key {
        id: UID,
        // Map from platform ID to last reputation update timestamp
        platform_updates: Table<ID, u64>,
        // Map from user profile ID to last reputation update timestamp
        user_updates: Table<ID, u64>,
        // Reports tracking
        reports: Table<ID, u64>,      // Entity ID -> number of reports
        reporters: Table<address, vector<ID>>, // Reporter -> entities reported
    }
    
    // === Events ===
    
    /// Event emitted when a platform's reputation changes
    struct PlatformReputationChangedEvent has copy, drop {
        platform_id: ID,
        old_score: u64,
        new_score: u64,
        reason: String,
        reason_code: u8,
        timestamp: u64,
    }
    
    /// Event emitted when a user's reputation changes on a platform
    struct UserReputationChangedEvent has copy, drop {
        user_profile_id: ID,
        platform_id: ID,
        old_score: u64,
        new_score: u64,
        reason: String,
        reason_code: u8,
        timestamp: u64,
    }
    
    /// Event emitted when an entity is reported
    struct EntityReportedEvent has copy, drop {
        entity_id: ID,
        reporter: address,
        reason: String,
        timestamp: u64,
    }
    
    // === Initialization ===
    
    /// Initialize the reputation system
    fun init(ctx: &mut TxContext) {
        // Create and transfer admin capability
        transfer::transfer(
            AdminCap {
                id: object::new(ctx)
            },
            tx_context::sender(ctx)
        );
        
        // Create reputation config with default values
        transfer::share_object(
            ReputationConfig {
                id: object::new(ctx),
                engagement_weight: 40,     // 40% weight for engagement
                report_weight: 20,         // 20% weight for reports
                growth_weight: 20,         // 20% weight for growth
                compliance_weight: 20,     // 20% weight for compliance
                min_reports_threshold: 5,  // 5 reports to trigger penalty
                max_daily_increase: 50,    // Max 50 points increase per day
                cooldown_period_ms: 86400000, // 24 hours cooldown
            }
        );
        
        // Create reputation registry
        transfer::share_object(
            ReputationRegistry {
                id: object::new(ctx),
                platform_updates: table::new(ctx),
                user_updates: table::new(ctx),
                reports: table::new(ctx),
                reporters: table::new(ctx),
            }
        );
    }
    
    // === Admin Functions ===
    
    /// Internal function to update platform reputation
    /// This is called by platform.move's update_reputation function
    public fun update_platform_reputation_internal(
        platform: &mut platform::Platform,
        stats: &mut platform::PlatformStats,
        new_score: u64,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify new score is within bounds
        assert!(new_score >= MIN_REPUTATION_SCORE && new_score <= MAX_REPUTATION_SCORE, EInvalidScore);
        
        // Verify stats belong to platform
        assert!(stats.platform_id == object::id(platform), ENotFound);
        
        // Store old score for event
        let old_score = platform::get_platform_reputation(platform);
        
        // Update platform reputation (directly accessing platform's internal field)
        platform.reputation_score = new_score;
        
        // Add to reputation history
        vector::push_back(&mut stats.reputation_history, new_score);
        if (vector::length(&stats.reputation_history) > 30) {
            // Keep only last 30 entries
            vector::remove(&mut stats.reputation_history, 0);
        };
        
        // Emit event
        event::emit(PlatformReputationChangedEvent {
            platform_id: object::id(platform),
            old_score,
            new_score,
            reason: string::utf8(reason),
            reason_code: REASON_ADMIN_ADJUSTMENT,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Update reputation system configuration (admin only)
    public entry fun update_config(
        _admin_cap: &AdminCap,
        config: &mut ReputationConfig,
        engagement_weight: u64,
        report_weight: u64,
        growth_weight: u64,
        compliance_weight: u64,
        min_reports_threshold: u64,
        max_daily_increase: u64,
        cooldown_period_ms: u64,
        _ctx: &mut TxContext
    ) {
        // Ensure weights sum to 100
        assert!(engagement_weight + report_weight + growth_weight + compliance_weight == 100, EInvalidOperation);
        
        // Update configuration
        config.engagement_weight = engagement_weight;
        config.report_weight = report_weight;
        config.growth_weight = growth_weight;
        config.compliance_weight = compliance_weight;
        config.min_reports_threshold = min_reports_threshold;
        config.max_daily_increase = max_daily_increase;
        config.cooldown_period_ms = cooldown_period_ms;
    }
    
    /// Manually adjust platform reputation (admin only)
    public entry fun adjust_platform_reputation(
        _admin_cap: &AdminCap,
        registry: &mut ReputationRegistry,
        platform: &mut Platform,
        stats: &mut PlatformStats,
        new_score: u64,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        let platform_id = object::id(platform);
        
        // Update platform reputation using our centralized function
        update_platform_reputation_internal(
            platform,
            stats,
            new_score,
            reason,
            ctx
        );
        
        // Update last update timestamp
        if (table::contains(&registry.platform_updates, platform_id)) {
            let timestamp = table::borrow_mut(&mut registry.platform_updates, platform_id);
            *timestamp = tx_context::epoch_timestamp_ms(ctx);
        } else {
            table::add(&mut registry.platform_updates, platform_id, tx_context::epoch_timestamp_ms(ctx));
        };
    }
    
    /// Manually adjust user reputation on a platform (admin only)
    /// Simplified version that only emits events for the indexer to track
    public entry fun adjust_user_reputation(
        _admin_cap: &AdminCap,
        registry: &mut ReputationRegistry,
        link: &ProfilePlatformLink,
        platform_id: ID,
        new_score: u64,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Ensure score is within valid range
        assert!(new_score >= MIN_REPUTATION_SCORE && new_score <= MAX_REPUTATION_SCORE, EInvalidScore);
        
        // Get profile ID
        let profile_id = link.profile_id;
        
        // Update last update timestamp
        if (table::contains(&registry.user_updates, profile_id)) {
            let timestamp = table::borrow_mut(&mut registry.user_updates, profile_id);
            *timestamp = tx_context::epoch_timestamp_ms(ctx);
        } else {
            table::add(&mut registry.user_updates, profile_id, tx_context::epoch_timestamp_ms(ctx));
        };
        
        // Emit user reputation changed event using profile_platform module
        profile_platform::emit_user_reputation_update(
            profile_id,
            platform_id,
            0, // We don't have the old score in this simplified version
            new_score,
            reason,
            ctx
        );
        
        // Also emit local event for compatibility
        event::emit(UserReputationChangedEvent {
            user_profile_id: profile_id,
            platform_id,
            old_score: 0, 
            new_score,
            reason: string::utf8(reason),
            reason_code: REASON_ADMIN_ADJUSTMENT,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Reset reports for an entity (admin only)
    public entry fun reset_reports(
        _admin_cap: &AdminCap,
        registry: &mut ReputationRegistry,
        entity_id: ID,
        _ctx: &mut TxContext
    ) {
        if (table::contains(&registry.reports, entity_id)) {
            let reports = table::borrow_mut(&mut registry.reports, entity_id);
            *reports = 0;
        };
    }
    
    // === User Functions ===
    
    /// Report an entity (platform, post, user, etc.) for reputation impact
    public entry fun report_entity(
        registry: &mut ReputationRegistry,
        entity_id: ID,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        let reporter = tx_context::sender(ctx);
        
        // Track reporter's reports
        if (!table::contains(&registry.reporters, reporter)) {
            table::add(&mut registry.reporters, reporter, vector::empty<ID>());
        };
        
        let reported_entities = table::borrow_mut(&mut registry.reporters, reporter);
        if (!vector::contains(reported_entities, &entity_id)) {
            vector::push_back(reported_entities, entity_id);
            
            // Increment report count for the entity
            if (!table::contains(&registry.reports, entity_id)) {
                table::add(&mut registry.reports, entity_id, 1);
            } else {
                let count = table::borrow_mut(&mut registry.reports, entity_id);
                *count = *count + 1;
            };
            
            // Emit event
            event::emit(EntityReportedEvent {
                entity_id,
                reporter,
                reason: string::utf8(reason),
                timestamp: tx_context::epoch_timestamp_ms(ctx),
            });
        };
    }
    
    /// Adjust reputation based on platform growth
    /// Simplified version that uses indexer data rather than on-chain analytics
    public entry fun adjust_for_growth(
        registry: &mut ReputationRegistry,
        config: &ReputationConfig,
        platform: &mut Platform,
        stats: &mut PlatformStats,
        growth_metric: u64, // This metric would come from indexer data
        ctx: &mut TxContext
    ) {
        let platform_id = object::id(platform);
        let current_time = tx_context::epoch_timestamp_ms(ctx);
        
        // Check cooldown period
        if (table::contains(&registry.platform_updates, platform_id)) {
            let last_update = *table::borrow(&registry.platform_updates, platform_id);
            if (current_time - last_update < config.cooldown_period_ms) {
                return // Still in cooldown
            };
        };
        
        // Get current reputation
        let old_score = platform::get_platform_reputation(platform);
        
        // Calculate adjustment based on the growth metric from indexer
        let adjustment = (config.growth_weight * growth_metric) / 100;
        
        // Cap adjustment
        let adjustment = if (adjustment > config.max_daily_increase) {
            config.max_daily_increase
        } else {
            adjustment
        };
        
        if (adjustment > 0) {
            // Calculate new score
            let new_score = if (old_score + adjustment > MAX_REPUTATION_SCORE) {
                MAX_REPUTATION_SCORE
            } else {
                old_score + adjustment
            };
            
            // Update platform reputation using our centralized function
            update_platform_reputation_internal(
                platform,
                stats,
                new_score,
                b"platform_growth",
                ctx
            );
            
            // Update last update timestamp
            if (table::contains(&registry.platform_updates, platform_id)) {
                let timestamp = table::borrow_mut(&mut registry.platform_updates, platform_id);
                *timestamp = current_time;
            } else {
                table::add(&mut registry.platform_updates, platform_id, current_time);
            };
            
            // Update stats last_updated field
            stats.last_updated = current_time;
        };
    }
    
    /// Adjust reputation based on reports
    public entry fun adjust_for_reports(
        registry: &mut ReputationRegistry,
        config: &ReputationConfig,
        platform: &mut Platform,
        stats: &mut PlatformStats,
        ctx: &mut TxContext
    ) {
        let platform_id = object::id(platform);
        
        // Check if platform has reports
        if (!table::contains(&registry.reports, platform_id)) {
            return // No reports
        };
        
        let reports_count = *table::borrow(&registry.reports, platform_id);
        if (reports_count < config.min_reports_threshold) {
            return // Not enough reports to trigger adjustment
        };
        
        // Get current reputation
        let old_score = platform::get_platform_reputation(platform);
        
        // Calculate penalty based on reports
        let penalty = (config.report_weight * reports_count) / 100;
        
        // Calculate new score
        let mut new_score = if (old_score > penalty) { old_score - penalty } else { MIN_REPUTATION_SCORE };
        if (new_score < MIN_REPUTATION_SCORE) {
            new_score = MIN_REPUTATION_SCORE;
        };
        
        // Update platform reputation using our centralized function
        update_platform_reputation_internal(
            platform,
            stats,
            new_score,
            b"user_reports",
            ctx
        );
        
        // Update last update timestamp
        if (table::contains(&registry.platform_updates, platform_id)) {
            let timestamp = table::borrow_mut(&mut registry.platform_updates, platform_id);
            *timestamp = tx_context::epoch_timestamp_ms(ctx);
        } else {
            table::add(&mut registry.platform_updates, platform_id, tx_context::epoch_timestamp_ms(ctx));
        };
        
        // Reset reports count
        let reports = table::borrow_mut(&mut registry.reports, platform_id);
        *reports = 0;
    }
    
    // === Public Accessor Functions ===
    
    /// Get report count for an entity
    public fun get_report_count(registry: &ReputationRegistry, entity_id: ID): u64 {
        if (table::contains(&registry.reports, entity_id)) {
            *table::borrow(&registry.reports, entity_id)
        } else {
            0
        }
    }
    
    /// Check if an entity has been reported by a specific address
    public fun is_reported_by(registry: &ReputationRegistry, entity_id: ID, reporter: address): bool {
        if (!table::contains(&registry.reporters, reporter)) {
            return false
        };
        
        let reported_entities = table::borrow(&registry.reporters, reporter);
        vector::contains(reported_entities, &entity_id)
    }
    
    /// Get time since last reputation update
    public fun time_since_last_update(registry: &ReputationRegistry, entity_id: ID, is_platform: bool, ctx: &TxContext): u64 {
        let current_time = tx_context::epoch_timestamp_ms(ctx);
        
        if (is_platform) {
            if (table::contains(&registry.platform_updates, entity_id)) {
                let last_update = *table::borrow(&registry.platform_updates, entity_id);
                return current_time - last_update
            };
        } else {
            if (table::contains(&registry.user_updates, entity_id)) {
                let last_update = *table::borrow(&registry.user_updates, entity_id);
                return current_time - last_update
            };
        };
        
        // Return max value if no update history
        18446744073709551615 // u64::MAX
    }
}