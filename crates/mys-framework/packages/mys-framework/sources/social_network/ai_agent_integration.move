// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// AI Agent Integration module for MySocial network.
/// This module connects AI agents with social network features,
/// enabling them to perform tasks like content recommendation, moderation,
/// and trend analysis while respecting user privacy through the MPC layer.
module mys::ai_agent_integration {
    use std::vector;
    use std::string::{Self, String};
    use mys::object::{Self, UID, ID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::table::{Self, Table};
    use mys::coin::{Self, Coin};
    use mys::mys::MYS;
    
    use mys::ai_agent_mpc::{Self, AgentRegistry, SecureEnclave, ComputationResult, AgentCap};
    use mys::platform::{Self, Platform};
    use mys::profile::{Self, Profile};
    use mys::social_graph::{Self, SocialGraph};
    use mys::ai_data_monetization::{Self, DataUsageAuthorization, FeeConfig, DataMonetizationManager};
    
    // === Errors ===
    /// Unauthorized operation
    const EUnauthorized: u64 = 0;
    /// Agent not certified
    const EAgentNotCertified: u64 = 1;
    /// Agent capability doesn't match agent type
    const EAgentTypeMismatch: u64 = 2;
    /// Invalid recommendation parameters
    const EInvalidParameters: u64 = 3;
    /// Operation not authorized by user
    const EUserNotOptedIn: u64 = 4;
    /// Data usage not authorized
    const EDataUsageNotAuthorized: u64 = 5;
    /// Data usage authorization expired
    const EAuthorizationExpired: u64 = 6;
    
    // === Agent Action Types ===
    /// Content recommendation
    const ACTION_RECOMMENDATION: u8 = 0;
    /// Content moderation
    const ACTION_MODERATION: u8 = 1;
    /// Trend analysis
    const ACTION_TREND_ANALYSIS: u8 = 2;
    /// Profile suggestion
    const ACTION_PROFILE_SUGGESTION: u8 = 3;
    
    // === Agent Privacy Levels ===
    /// Public data only
    const PRIVACY_PUBLIC: u8 = 0;
    /// Aggregate data (anonymized)
    const PRIVACY_AGGREGATE: u8 = 1;
    /// Private data (requires MPC)
    const PRIVACY_PRIVATE: u8 = 2;
    
    // === Structs ===
    
    /// AI Agent Integration Manager
    struct AgentIntegrationManager has key {
        id: UID,
        /// Map from platform ID to allowed agent IDs
        platform_agents: Table<ID, vector<ID>>,
        /// Map from agent ID to platform IDs it can operate on
        agent_platforms: Table<ID, vector<ID>>,
        /// Map from profile ID to privacy preferences
        user_privacy_settings: Table<ID, UserPrivacySettings>,
        /// Map from agent ID to action counter
        agent_action_counts: Table<ID, AgentActionCount>,
    }
    
    /// User privacy settings for AI agents
    struct UserPrivacySettings has store, drop {
        /// Profile ID
        profile_id: ID,
        /// Overall privacy level preference
        privacy_level: u8,
        /// Specific opt-ins for agent types
        agent_type_opt_ins: vector<u8>,
        /// Specific opt-outs for agent types
        agent_type_opt_outs: vector<u8>,
        /// Last updated timestamp
        last_updated: u64,
    }
    
    /// Tracks actions performed by an agent
    struct AgentActionCount has store, drop {
        /// Count of recommendations
        recommendations: u64,
        /// Count of moderations
        moderations: u64,
        /// Count of trend analyses
        trend_analyses: u64,
        /// Count of profile suggestions
        profile_suggestions: u64,
        /// Last action timestamp
        last_action: u64,
    }
    
    /// Recommendation result from an AI agent
    struct RecommendationResult has key, store {
        id: UID,
        /// Agent that produced the recommendation
        agent_id: ID,
        /// Platform for which the recommendation was made
        platform_id: ID,
        /// Profile for which the recommendation was made
        profile_id: ID,
        /// Recommended content IDs
        content_ids: vector<ID>,
        /// Recommendation scores (0-100)
        scores: vector<u64>,
        /// Recommendation timestamp
        timestamp: u64,
    }
    
    /// Moderation result from an AI agent
    struct ModerationResult has key, store {
        id: UID,
        /// Agent that performed the moderation
        agent_id: ID,
        /// Content that was moderated
        content_id: ID,
        /// Moderation decision (0=approved, 1=flagged, 2=removed)
        decision: u8,
        /// Confidence score (0-100)
        confidence: u64,
        /// Categories of violations found
        violation_categories: vector<u8>,
        /// Moderation timestamp
        timestamp: u64,
    }
    
    /// Trend analysis result from an AI agent
    struct TrendAnalysisResult has key, store {
        id: UID,
        /// Agent that performed the analysis
        agent_id: ID,
        /// Platform for which the analysis was done
        platform_id: ID,
        /// Trending topic strings
        topics: vector<String>,
        /// Trend scores (0-100)
        scores: vector<u64>,
        /// Associated content IDs
        content_ids: vector<ID>,
        /// Analysis timestamp
        timestamp: u64,
    }
    
    // === Events ===
    
    /// Event emitted when an agent is authorized for a platform
    struct AgentAuthorizedEvent has copy, drop {
        agent_id: ID,
        platform_id: ID,
        authorizer: address,
        timestamp: u64,
    }
    
    /// Event emitted when a recommendation is made
    struct RecommendationEvent has copy, drop {
        agent_id: ID,
        platform_id: ID,
        profile_id: ID,
        recommendation_count: u64,
        privacy_level: u8,
        timestamp: u64,
    }
    
    /// Event emitted when content is moderated
    struct ModerationEvent has copy, drop {
        agent_id: ID,
        content_id: ID,
        decision: u8,
        timestamp: u64,
    }
    
    // === Initialization ===
    
    /// Initialize the AI Agent Integration system
    fun init(ctx: &mut TxContext) {
        transfer::share_object(
            AgentIntegrationManager {
                id: object::new(ctx),
                platform_agents: table::new(ctx),
                agent_platforms: table::new(ctx),
                user_privacy_settings: table::new(ctx),
                agent_action_counts: table::new(ctx),
            }
        );
    }
    
    // === Platform-Agent Integration ===
    
    /// Authorize an AI agent to operate on a platform
    public entry fun authorize_agent_for_platform(
        manager: &mut AgentIntegrationManager,
        platform: &Platform,
        agent_id: ID,
        ctx: &mut TxContext
    ) {
        // Verify caller is platform owner
        assert!(platform::get_owner(platform) == tx_context::sender(ctx), EUnauthorized);
        
        let platform_id = object::id(platform);
        
        // Initialize platform agents list if it doesn't exist
        if (!table::contains(&manager.platform_agents, platform_id)) {
            table::add(&mut manager.platform_agents, platform_id, vector::empty<ID>());
        };
        
        // Initialize agent platforms list if it doesn't exist
        if (!table::contains(&manager.agent_platforms, agent_id)) {
            table::add(&mut manager.agent_platforms, agent_id, vector::empty<ID>());
        };
        
        // Add agent to platform's allowed agents
        let platform_agents = table::borrow_mut(&mut manager.platform_agents, platform_id);
        if (!vector::contains(platform_agents, &agent_id)) {
            vector::push_back(platform_agents, agent_id);
        };
        
        // Add platform to agent's allowed platforms
        let agent_platforms = table::borrow_mut(&mut manager.agent_platforms, agent_id);
        if (!vector::contains(agent_platforms, &platform_id)) {
            vector::push_back(agent_platforms, platform_id);
        };
        
        // Initialize action counter if it doesn't exist
        if (!table::contains(&manager.agent_action_counts, agent_id)) {
            table::add(
                &mut manager.agent_action_counts,
                agent_id,
                AgentActionCount {
                    recommendations: 0,
                    moderations: 0,
                    trend_analyses: 0,
                    profile_suggestions: 0,
                    last_action: tx_context::epoch_timestamp_ms(ctx),
                }
            );
        };
        
        // Emit event
        event::emit(AgentAuthorizedEvent {
            agent_id,
            platform_id,
            authorizer: tx_context::sender(ctx),
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    // === User Privacy Settings ===
    
    /// Set user privacy preferences for AI agents
    public entry fun set_privacy_preferences(
        manager: &mut AgentIntegrationManager,
        profile: &Profile,
        privacy_level: u8,
        agent_type_opt_ins: vector<u8>,
        agent_type_opt_outs: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the profile
        assert!(profile::owner(profile) == tx_context::sender(ctx), EUnauthorized);
        
        let profile_id = object::id(profile);
        
        // Create privacy settings
        let settings = UserPrivacySettings {
            profile_id,
            privacy_level,
            agent_type_opt_ins,
            agent_type_opt_outs,
            last_updated: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Add or update settings
        if (table::contains(&manager.user_privacy_settings, profile_id)) {
            let existing_settings = table::borrow_mut(&mut manager.user_privacy_settings, profile_id);
            *existing_settings = settings;
        } else {
            table::add(&mut manager.user_privacy_settings, profile_id, settings);
        };
    }
    
    /// Check if a user has opted in to a specific agent type
    fun is_user_opted_in(
        manager: &AgentIntegrationManager,
        profile_id: ID,
        agent_type: u8
    ): bool {
        if (!table::contains(&manager.user_privacy_settings, profile_id)) {
            // Default to public data only if no settings exist
            return agent_type == ai_agent_mpc::recommendation_agent_type();
        };
        
        let settings = table::borrow(&manager.user_privacy_settings, profile_id);
        
        // Check if explicitly opted out
        if (vector::contains(&settings.agent_type_opt_outs, &agent_type)) {
            return false;
        };
        
        // Check if explicitly opted in
        if (vector::contains(&settings.agent_type_opt_ins, &agent_type)) {
            return true;
        };
        
        // Otherwise, base on privacy level
        // Allow recommendation agents by default at public privacy level
        settings.privacy_level == PRIVACY_PUBLIC && 
            agent_type == ai_agent_mpc::recommendation_agent_type()
    }
    
    // === AI Agent Actions ===
    
    /// Request content recommendations using MPC with data monetization
    public entry fun request_recommendations(
        manager: &mut AgentIntegrationManager,
        agent_registry: &AgentRegistry,
        enclave: &mut SecureEnclave,
        data_monetization_manager: &DataMonetizationManager,
        agent_cap: &AgentCap,
        authorization: &DataUsageAuthorization,
        platform_id: ID,
        profile_id: ID,
        parameters: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent is of recommendation type
        assert!(agent_cap.agent_type == ai_agent_mpc::recommendation_agent_type(), EAgentTypeMismatch);
        
        // Verify agent is authorized for platform
        assert!(
            table::contains(&manager.agent_platforms, agent_cap.agent_id) &&
            vector::contains(table::borrow(&manager.agent_platforms, agent_cap.agent_id), &platform_id),
            EUnauthorized
        );
        
        // Verify data usage authorization is valid
        assert!(authorization.agent_id == agent_cap.agent_id, EDataUsageNotAuthorized);
        assert!(authorization.platform_id == platform_id, EDataUsageNotAuthorized);
        assert!(authorization.profile_id == profile_id, EDataUsageNotAuthorized);
        assert!(authorization.expiration_timestamp > tx_context::epoch_timestamp_ms(ctx), EAuthorizationExpired);
        
        // Verify user has opted in to recommendations (through data monetization)
        assert!(
            ai_data_monetization::is_monetization_enabled(data_monetization_manager, profile_id),
            EUserNotOptedIn
        );
        
        // Get user privacy level from monetization level
        let privacy_level = authorization.monetization_level;
        
        // Determine computation type based on privacy level/monetization level
        let computation_type = if (privacy_level == ai_data_monetization::premium_monetization_level()) {
            ai_agent_mpc::mpc_computation_type()
        } else if (privacy_level == ai_data_monetization::standard_monetization_level()) {
            ai_agent_mpc::federated_computation_type()
        } else {
            ai_agent_mpc::private_computation_type()
        };
        
        // Generate input hash
        let input_data = vector::empty<u8>();
        vector::append(&mut input_data, *&parameters);
        vector::append(&mut input_data, object::id_to_bytes(&platform_id));
        vector::append(&mut input_data, object::id_to_bytes(&profile_id));
        let input_hash = std::hash::sha3_256(input_data);
        
        // Request computation via MPC
        ai_agent_mpc::request_computation(
            agent_registry,
            enclave,
            agent_cap,
            computation_type,
            parameters,
            input_hash,
            ctx
        );
        
        // Update agent action count
        let action_count = table::borrow_mut(&mut manager.agent_action_counts, agent_cap.agent_id);
        action_count.recommendations = action_count.recommendations + 1;
        action_count.last_action = tx_context::epoch_timestamp_ms(ctx);
        
        // Convert monetization level to privacy level for compatibility
        let privacy_level_compat = if (privacy_level == ai_data_monetization::premium_monetization_level()) {
            PRIVACY_PRIVATE
        } else if (privacy_level == ai_data_monetization::standard_monetization_level()) {
            PRIVACY_AGGREGATE
        } else {
            PRIVACY_PUBLIC
        };
        
        // Emit event
        event::emit(RecommendationEvent {
            agent_id: agent_cap.agent_id,
            platform_id,
            profile_id,
            recommendation_count: action_count.recommendations,
            privacy_level: privacy_level_compat,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Legacy request content recommendations (without monetization)
    public entry fun request_recommendations_legacy(
        manager: &mut AgentIntegrationManager,
        agent_registry: &AgentRegistry,
        enclave: &mut SecureEnclave,
        agent_cap: &AgentCap,
        platform_id: ID,
        profile_id: ID,
        parameters: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent is of recommendation type
        assert!(agent_cap.agent_type == ai_agent_mpc::recommendation_agent_type(), EAgentTypeMismatch);
        
        // Verify agent is authorized for platform
        assert!(
            table::contains(&manager.agent_platforms, agent_cap.agent_id) &&
            vector::contains(table::borrow(&manager.agent_platforms, agent_cap.agent_id), &platform_id),
            EUnauthorized
        );
        
        // Verify user has opted in to recommendations
        assert!(
            is_user_opted_in(manager, profile_id, ai_agent_mpc::recommendation_agent_type()),
            EUserNotOptedIn
        );
        
        // Get user privacy level
        let privacy_level = if (table::contains(&manager.user_privacy_settings, profile_id)) {
            table::borrow(&manager.user_privacy_settings, profile_id).privacy_level
        } else {
            PRIVACY_PUBLIC
        };
        
        // Determine computation type based on privacy level
        let computation_type = if (privacy_level == PRIVACY_PRIVATE) {
            ai_agent_mpc::mpc_computation_type()
        } else if (privacy_level == PRIVACY_AGGREGATE) {
            ai_agent_mpc::federated_computation_type()
        } else {
            ai_agent_mpc::private_computation_type()
        };
        
        // Generate input hash
        let input_data = vector::empty<u8>();
        vector::append(&mut input_data, *&parameters);
        vector::append(&mut input_data, object::id_to_bytes(&platform_id));
        vector::append(&mut input_data, object::id_to_bytes(&profile_id));
        let input_hash = std::hash::sha3_256(input_data);
        
        // Request computation via MPC
        ai_agent_mpc::request_computation(
            agent_registry,
            enclave,
            agent_cap,
            computation_type,
            parameters,
            input_hash,
            ctx
        );
        
        // Update agent action count
        let action_count = table::borrow_mut(&mut manager.agent_action_counts, agent_cap.agent_id);
        action_count.recommendations = action_count.recommendations + 1;
        action_count.last_action = tx_context::epoch_timestamp_ms(ctx);
        
        // Emit event
        event::emit(RecommendationEvent {
            agent_id: agent_cap.agent_id,
            platform_id,
            profile_id,
            recommendation_count: action_count.recommendations,
            privacy_level,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Process recommendation results and create a recommendation object
    /// Supports both monetized and non-monetized computation results
    public entry fun process_recommendation_results(
        manager: &AgentIntegrationManager,
        computation_result: &ComputationResult,
        authorization: &DataUsageAuthorization,
        platform_id: ID,
        profile_id: ID,
        content_ids: vector<ID>,
        scores: vector<u64>,
        ctx: &mut TxContext
    ) {
        // In a real implementation, we would verify the computation result
        // and decode the encrypted results
        
        // Verify authorization is valid
        assert!(authorization.agent_id == computation_result.computation_id, EDataUsageNotAuthorized);
        assert!(authorization.platform_id == platform_id, EDataUsageNotAuthorized);
        assert!(authorization.profile_id == profile_id, EDataUsageNotAuthorized);
        assert!(authorization.expiration_timestamp > tx_context::epoch_timestamp_ms(ctx), EAuthorizationExpired);
        
        // Create and transfer the recommendation result
        let recommendation = RecommendationResult {
            id: object::new(ctx),
            agent_id: object::id_from_bytes(computation_result.result),
            platform_id,
            profile_id,
            content_ids,
            scores,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        };
        
        transfer::transfer(recommendation, tx_context::sender(ctx));
    }
    
    /// Process recommendation results without monetization (legacy mode)
    public entry fun process_recommendation_results_legacy(
        manager: &AgentIntegrationManager,
        computation_result: &ComputationResult,
        platform_id: ID,
        profile_id: ID,
        content_ids: vector<ID>,
        scores: vector<u64>,
        ctx: &mut TxContext
    ) {
        // In a real implementation, we would verify the computation result
        // and decode the encrypted results
        
        // Create and transfer the recommendation result
        let recommendation = RecommendationResult {
            id: object::new(ctx),
            agent_id: object::id_from_bytes(computation_result.result),
            platform_id,
            profile_id,
            content_ids,
            scores,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        };
        
        transfer::transfer(recommendation, tx_context::sender(ctx));
    }
    
    /// Request content moderation using MPC with data monetization
    public entry fun request_moderation(
        manager: &mut AgentIntegrationManager,
        agent_registry: &AgentRegistry,
        enclave: &mut SecureEnclave,
        data_monetization_manager: &DataMonetizationManager,
        agent_cap: &AgentCap,
        authorization: &DataUsageAuthorization,
        content_id: ID,
        platform_id: ID,
        profile_id: ID,
        parameters: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent is of moderation type
        assert!(agent_cap.agent_type == ai_agent_mpc::moderation_agent_type(), EAgentTypeMismatch);
        
        // Verify data usage authorization is valid
        assert!(authorization.agent_id == agent_cap.agent_id, EDataUsageNotAuthorized);
        assert!(authorization.platform_id == platform_id, EDataUsageNotAuthorized);
        assert!(authorization.profile_id == profile_id, EDataUsageNotAuthorized);
        assert!(authorization.expiration_timestamp > tx_context::epoch_timestamp_ms(ctx), EAuthorizationExpired);
        
        // Verify usage type includes content
        assert!(vector::contains(&authorization.allowed_usage_types, &ai_data_monetization::content_usage_type()), EDataUsageNotAuthorized);
        
        // Generate input hash
        let input_data = vector::empty<u8>();
        vector::append(&mut input_data, *&parameters);
        vector::append(&mut input_data, object::id_to_bytes(&content_id));
        let input_hash = std::hash::sha3_256(input_data);
        
        // Determine computation type based on monetization level
        let computation_type = if (authorization.monetization_level == ai_data_monetization::premium_monetization_level()) {
            ai_agent_mpc::mpc_computation_type()
        } else if (authorization.monetization_level == ai_data_monetization::standard_monetization_level()) {
            ai_agent_mpc::federated_computation_type()
        } else {
            ai_agent_mpc::private_computation_type()
        };
        
        // Request computation via MPC
        ai_agent_mpc::request_computation(
            agent_registry,
            enclave,
            agent_cap,
            computation_type,
            parameters,
            input_hash,
            ctx
        );
        
        // Update agent action count
        let action_count = table::borrow_mut(&mut manager.agent_action_counts, agent_cap.agent_id);
        action_count.moderations = action_count.moderations + 1;
        action_count.last_action = tx_context::epoch_timestamp_ms(ctx);
    }
    
    /// Request content moderation using MPC (legacy version without monetization)
    public entry fun request_moderation_legacy(
        manager: &mut AgentIntegrationManager,
        agent_registry: &AgentRegistry,
        enclave: &mut SecureEnclave,
        agent_cap: &AgentCap,
        content_id: ID,
        parameters: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent is of moderation type
        assert!(agent_cap.agent_type == ai_agent_mpc::moderation_agent_type(), EAgentTypeMismatch);
        
        // Generate input hash
        let input_data = vector::empty<u8>();
        vector::append(&mut input_data, *&parameters);
        vector::append(&mut input_data, object::id_to_bytes(&content_id));
        let input_hash = std::hash::sha3_256(input_data);
        
        // Request computation via MPC
        ai_agent_mpc::request_computation(
            agent_registry,
            enclave,
            agent_cap,
            ai_agent_mpc::private_computation_type(),
            parameters,
            input_hash,
            ctx
        );
        
        // Update agent action count
        let action_count = table::borrow_mut(&mut manager.agent_action_counts, agent_cap.agent_id);
        action_count.moderations = action_count.moderations + 1;
        action_count.last_action = tx_context::epoch_timestamp_ms(ctx);
    }
    
    /// Process moderation results
    public entry fun process_moderation_results(
        computation_result: &ComputationResult,
        content_id: ID,
        decision: u8,
        confidence: u64,
        violation_categories: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Create and transfer the moderation result
        let moderation_result = ModerationResult {
            id: object::new(ctx),
            agent_id: object::id_from_bytes(computation_result.result),
            content_id,
            decision,
            confidence,
            violation_categories,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Emit event
        event::emit(ModerationEvent {
            agent_id: moderation_result.agent_id,
            content_id,
            decision,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
        
        transfer::transfer(moderation_result, tx_context::sender(ctx));
    }
    
    /// Request trend analysis using MPC
    public entry fun request_trend_analysis(
        manager: &mut AgentIntegrationManager,
        agent_registry: &AgentRegistry,
        enclave: &mut SecureEnclave,
        agent_cap: &AgentCap,
        platform_id: ID,
        parameters: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent is of trend analysis type
        assert!(agent_cap.agent_type == ai_agent_mpc::trend_analysis_agent_type(), EAgentTypeMismatch);
        
        // Verify agent is authorized for platform
        assert!(
            table::contains(&manager.agent_platforms, agent_cap.agent_id) &&
            vector::contains(table::borrow(&manager.agent_platforms, agent_cap.agent_id), &platform_id),
            EUnauthorized
        );
        
        // Generate input hash
        let input_data = vector::empty<u8>();
        vector::append(&mut input_data, *&parameters);
        vector::append(&mut input_data, object::id_to_bytes(&platform_id));
        let input_hash = std::hash::sha3_256(input_data);
        
        // Request computation via MPC
        ai_agent_mpc::request_computation(
            agent_registry,
            enclave,
            agent_cap,
            ai_agent_mpc::federated_computation_type(),
            parameters,
            input_hash,
            ctx
        );
        
        // Update agent action count
        let action_count = table::borrow_mut(&mut manager.agent_action_counts, agent_cap.agent_id);
        action_count.trend_analyses = action_count.trend_analyses + 1;
        action_count.last_action = tx_context::epoch_timestamp_ms(ctx);
    }
    
    /// Process trend analysis results
    public entry fun process_trend_analysis_results(
        computation_result: &ComputationResult,
        platform_id: ID,
        topics: vector<vector<u8>>,
        scores: vector<u64>,
        content_ids: vector<ID>,
        ctx: &mut TxContext
    ) {
        // Convert byte vectors to strings
        let string_topics = vector::empty<String>();
        let i = 0;
        let len = vector::length(&topics);
        while (i < len) {
            vector::push_back(&mut string_topics, string::utf8(*vector::borrow(&topics, i)));
            i = i + 1;
        };
        
        // Create and transfer the trend analysis result
        let trend_result = TrendAnalysisResult {
            id: object::new(ctx),
            agent_id: object::id_from_bytes(computation_result.result),
            platform_id,
            topics: string_topics,
            scores,
            content_ids,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        };
        
        transfer::transfer(trend_result, tx_context::sender(ctx));
    }
    
    // === Public Accessor Functions ===
    
    /// Get user privacy settings
    public fun get_user_privacy_settings(
        manager: &AgentIntegrationManager,
        profile_id: ID
    ): (bool, u8, vector<u8>, vector<u8>, u64) {
        if (table::contains(&manager.user_privacy_settings, profile_id)) {
            let settings = table::borrow(&manager.user_privacy_settings, profile_id);
            (
                true,
                settings.privacy_level,
                settings.agent_type_opt_ins,
                settings.agent_type_opt_outs,
                settings.last_updated
            )
        } else {
            (
                false,
                PRIVACY_PUBLIC,
                vector::empty(),
                vector::empty(),
                0
            )
        }
    }
    
    /// Get agent action counts
    public fun get_agent_action_counts(
        manager: &AgentIntegrationManager,
        agent_id: ID
    ): (bool, u64, u64, u64, u64, u64) {
        if (table::contains(&manager.agent_action_counts, agent_id)) {
            let counts = table::borrow(&manager.agent_action_counts, agent_id);
            (
                true,
                counts.recommendations,
                counts.moderations,
                counts.trend_analyses,
                counts.profile_suggestions,
                counts.last_action
            )
        } else {
            (false, 0, 0, 0, 0, 0)
        }
    }
    
    /// Check if an agent is authorized for a platform
    public fun is_agent_authorized_for_platform(
        manager: &AgentIntegrationManager,
        agent_id: ID,
        platform_id: ID
    ): bool {
        if (!table::contains(&manager.agent_platforms, agent_id)) {
            return false
        };
        
        let platforms = table::borrow(&manager.agent_platforms, agent_id);
        vector::contains(platforms, &platform_id)
    }
    
    // === Privacy Constants ===
    
    /// Get public privacy level constant
    public fun public_privacy_level(): u8 {
        PRIVACY_PUBLIC
    }
    
    /// Get aggregate privacy level constant
    public fun aggregate_privacy_level(): u8 {
        PRIVACY_AGGREGATE
    }
    
    /// Get private privacy level constant
    public fun private_privacy_level(): u8 {
        PRIVACY_PRIVATE
    }
}