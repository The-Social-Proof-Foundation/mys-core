// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// AI Agent MPC (Multi-Party Computation) module for MySocial network.
/// This module enables secure, private computation for AI agents running on the network,
/// allowing them to perform operations on sensitive data without revealing the data itself.
module mys::ai_agent_mpc {
    use std::vector;
    use std::string::{Self, String};
    use mys::object::{Self, UID, ID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::table::{Self, Table};
    use mys::bag::{Self, Bag};
    use mys::crypto::hash::{self as hash};
    use mys::crypto::hmac::{Self as hmac};
    
    // === Errors ===
    /// Unauthorized access to MPC computation
    const EUnauthorized: u64 = 0;
    /// Invalid computation request
    const EInvalidRequest: u64 = 1;
    /// Computation failed
    const EComputationFailed: u64 = 2;
    /// Invalid agent type
    const EInvalidAgentType: u64 = 3;
    /// Agent already registered
    const EAgentAlreadyRegistered: u64 = 4;
    /// Agent not found
    const EAgentNotFound: u64 = 5;
    /// Secure enclave not available
    const EEnclaveNotAvailable: u64 = 6;
    /// Invalid computation result
    const EInvalidResult: u64 = 7;

    // === Agent Types ===
    /// Recommendation agent type
    const AGENT_TYPE_RECOMMENDATION: u8 = 0;
    /// Content moderation agent type
    const AGENT_TYPE_MODERATION: u8 = 1;
    /// Trend analysis agent type
    const AGENT_TYPE_TREND_ANALYSIS: u8 = 2;
    /// Custom agent type
    const AGENT_TYPE_CUSTOM: u8 = 3;
    
    // === Computation Types ===
    /// Private input calculation (no data revealed)
    const COMPUTATION_PRIVATE: u8 = 0;
    /// Threshold computation (results only revealed when threshold met)
    const COMPUTATION_THRESHOLD: u8 = 1;
    /// Federated computation (aggregated results only)
    const COMPUTATION_FEDERATED: u8 = 2;
    /// Secure multiparty computation
    const COMPUTATION_MPC: u8 = 3;
    
    // === Structs ===
    
    /// AI Agent MPC Registry - maintains the list of authorized AI agents
    struct AgentRegistry has key {
        id: UID,
        /// Map from agent ID to agent info
        agents: Table<ID, AgentInfo>,
        /// Map from agent type to list of agent IDs of that type
        agents_by_type: Table<u8, vector<ID>>,
        /// Storage for verification keys
        verification_keys: Bag,
    }
    
    /// Information about an AI agent
    struct AgentInfo has store, drop {
        /// ID of the agent
        id: ID,
        /// Type of agent (recommendation, moderation, etc.)
        agent_type: u8,
        /// Name of the agent
        name: String,
        /// Description of the agent
        description: String,
        /// Address of the agent owner
        owner: address,
        /// Verification code for the agent
        verification_code: vector<u8>,
        /// Whether the agent is certified
        is_certified: bool,
    }
    
    /// AI Agent capability - authorizes an agent to perform computations
    struct AgentCap has key, store {
        id: UID,
        /// ID of the agent this capability is for
        agent_id: ID,
        /// Type of agent
        agent_type: u8,
        /// Owner address
        owner: address,
    }
    
    /// Secure computation enclave - where private computations happen
    struct SecureEnclave has key {
        id: UID,
        /// Map from computation ID to computation state
        computations: Table<ID, ComputationState>,
        /// Map from node ID to node info
        nodes: Table<ID, NodeInfo>,
        /// Number of active computations
        active_computations: u64,
    }
    
    /// Information about an MPC computation node
    struct NodeInfo has store, drop {
        /// ID of the node
        id: ID,
        /// Public key of the node
        public_key: vector<u8>,
        /// Address of the node operator
        operator: address,
        /// Stake amount
        stake: u64,
        /// Is node active
        is_active: bool,
    }
    
    /// State of a computation
    struct ComputationState has store, drop {
        /// ID of the computation
        id: ID,
        /// Type of computation
        computation_type: u8,
        /// IDs of participating agents
        agent_ids: vector<ID>,
        /// Inputs hashes (to verify integrity)
        input_hashes: vector<vector<u8>>,
        /// Result hash
        result_hash: vector<u8>,
        /// Status of computation (0=pending, 1=in progress, 2=completed, 3=failed)
        status: u8,
        /// Timestamp when computation was requested
        request_timestamp: u64,
        /// Timestamp when computation was completed
        completion_timestamp: u64,
    }
    
    /// Computation request
    struct ComputationRequest has key, store {
        id: UID,
        /// Agent initiating the request
        requester_agent_id: ID,
        /// Type of computation
        computation_type: u8,
        /// Parameters for the computation (encrypted)
        parameters: vector<u8>,
        /// Hash of input data
        input_hash: vector<u8>,
        /// Whether the request has been processed
        processed: bool,
        /// Timestamp of request
        timestamp: u64,
    }
    
    /// Computation result
    struct ComputationResult has key, store {
        id: UID,
        /// ID of the original computation
        computation_id: ID,
        /// Encrypted result data
        result: vector<u8>,
        /// Hash of the result (for verification)
        result_hash: vector<u8>,
        /// Proof of correct computation
        verification_proof: vector<u8>,
        /// Timestamp of completion
        completion_timestamp: u64,
    }
    
    // === Events ===
    
    /// Event emitted when a new AI agent is registered
    struct AgentRegisteredEvent has copy, drop {
        agent_id: ID,
        agent_type: u8,
        name: String,
        owner: address,
        timestamp: u64,
    }
    
    /// Event emitted when a computation is requested
    struct ComputationRequestedEvent has copy, drop {
        computation_id: ID,
        requester_agent_id: ID,
        computation_type: u8,
        input_hash: vector<u8>,
        timestamp: u64,
    }
    
    /// Event emitted when a computation is completed
    struct ComputationCompletedEvent has copy, drop {
        computation_id: ID,
        result_hash: vector<u8>,
        timestamp: u64,
    }
    
    // === Initialization ===
    
    /// Initialize the AI Agent MPC system
    fun init(ctx: &mut TxContext) {
        // Create and share the agent registry
        transfer::share_object(
            AgentRegistry {
                id: object::new(ctx),
                agents: table::new(ctx),
                agents_by_type: table::new(ctx),
                verification_keys: bag::new(ctx),
            }
        );
        
        // Create and share the secure enclave
        transfer::share_object(
            SecureEnclave {
                id: object::new(ctx),
                computations: table::new(ctx),
                nodes: table::new(ctx),
                active_computations: 0,
            }
        );
    }
    
    // === Agent Registration ===
    
    /// Register a new AI agent
    public entry fun register_agent(
        registry: &mut AgentRegistry,
        name: vector<u8>,
        description: vector<u8>,
        agent_type: u8,
        verification_code: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent type is valid
        assert!(
            agent_type == AGENT_TYPE_RECOMMENDATION ||
            agent_type == AGENT_TYPE_MODERATION ||
            agent_type == AGENT_TYPE_TREND_ANALYSIS ||
            agent_type == AGENT_TYPE_CUSTOM,
            EInvalidAgentType
        );
        
        // Create a new agent ID
        let agent_id_obj = object::new(ctx);
        let agent_id = object::uid_to_inner(&agent_id_obj);
        
        // Create agent info
        let agent_info = AgentInfo {
            id: agent_id,
            agent_type,
            name: string::utf8(name),
            description: string::utf8(description),
            owner: tx_context::sender(ctx),
            verification_code,
            is_certified: false,
        };
        
        // Add to registry
        table::add(&mut registry.agents, agent_id, agent_info);
        
        // Add to type mapping
        if (!table::contains(&registry.agents_by_type, agent_type)) {
            table::add(&mut registry.agents_by_type, agent_type, vector::empty<ID>());
        };
        let agents_of_type = table::borrow_mut(&mut registry.agents_by_type, agent_type);
        vector::push_back(agents_of_type, agent_id);
        
        // Create and transfer agent capability to the owner
        let agent_cap = AgentCap {
            id: agent_id_obj,
            agent_id,
            agent_type,
            owner: tx_context::sender(ctx),
        };
        transfer::transfer(agent_cap, tx_context::sender(ctx));
        
        // Emit event
        event::emit(AgentRegisteredEvent {
            agent_id,
            agent_type,
            name: string::utf8(name),
            owner: tx_context::sender(ctx),
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Certify an AI agent (admin only)
    public entry fun certify_agent(
        registry: &mut AgentRegistry,
        agent_id: ID,
        ctx: &mut TxContext
    ) {
        // In a real implementation, we would verify the caller has admin privileges
        // Here, we're assuming the caller is authorized
        
        // Update agent info
        let agent_info = table::borrow_mut(&mut registry.agents, agent_id);
        agent_info.is_certified = true;
    }
    
    // === MPC Computation ===
    
    /// Request a secure MPC computation
    public entry fun request_computation(
        registry: &AgentRegistry,
        enclave: &mut SecureEnclave,
        agent_cap: &AgentCap,
        computation_type: u8,
        parameters: vector<u8>,
        input_hash: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify agent capability
        assert!(table::contains(&registry.agents, agent_cap.agent_id), EAgentNotFound);
        
        // Verify computation type
        assert!(
            computation_type == COMPUTATION_PRIVATE ||
            computation_type == COMPUTATION_THRESHOLD ||
            computation_type == COMPUTATION_FEDERATED ||
            computation_type == COMPUTATION_MPC,
            EInvalidRequest
        );
        
        // Create computation request
        let request = ComputationRequest {
            id: object::new(ctx),
            requester_agent_id: agent_cap.agent_id,
            computation_type,
            parameters,
            input_hash,
            processed: false,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Start the computation in the secure enclave
        let computation_id = start_computation(enclave, &request, ctx);
        
        // Emit event
        event::emit(ComputationRequestedEvent {
            computation_id,
            requester_agent_id: agent_cap.agent_id,
            computation_type,
            input_hash,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
        
        // Transfer request to sender
        transfer::transfer(request, tx_context::sender(ctx));
    }
    
    /// Start a computation in the secure enclave
    fun start_computation(
        enclave: &mut SecureEnclave,
        request: &ComputationRequest,
        ctx: &mut TxContext
    ): ID {
        // Create a new computation ID
        let computation_id_obj = object::new(ctx);
        let computation_id = object::uid_to_inner(&computation_id_obj);
        
        // Create empty vectors for participating agents and input hashes
        let agent_ids = vector::empty<ID>();
        vector::push_back(&mut agent_ids, request.requester_agent_id);
        
        let input_hashes = vector::empty<vector<u8>>();
        vector::push_back(&mut input_hashes, request.input_hash);
        
        // Create computation state
        let computation_state = ComputationState {
            id: computation_id,
            computation_type: request.computation_type,
            agent_ids,
            input_hashes,
            result_hash: vector::empty(),
            status: 0, // pending
            request_timestamp: tx_context::epoch_timestamp_ms(ctx),
            completion_timestamp: 0,
        };
        
        // Add to enclave's computations
        table::add(&mut enclave.computations, computation_id, computation_state);
        
        // Increment active computations
        enclave.active_computations = enclave.active_computations + 1;
        
        // Delete the computation ID object (it was just used to generate an ID)
        object::delete(computation_id_obj);
        
        computation_id
    }
    
    /// Submit a computation result (from an MPC node)
    public entry fun submit_computation_result(
        enclave: &mut SecureEnclave,
        computation_id: ID,
        result: vector<u8>,
        verification_proof: vector<u8>,
        ctx: &mut TxContext
    ) {
        // In a real implementation, we would verify the caller is an authorized MPC node
        
        // Verify computation exists
        assert!(table::contains(&enclave.computations, computation_id), EComputationFailed);
        
        // Get computation state
        let computation = table::borrow_mut(&mut enclave.computations, computation_id);
        
        // Update state
        computation.status = 2; // completed
        computation.completion_timestamp = tx_context::epoch_timestamp_ms(ctx);
        
        // Calculate result hash
        let result_hash = hash::sha3_256(&result);
        computation.result_hash = result_hash;
        
        // Create computation result
        let result_obj = ComputationResult {
            id: object::new(ctx),
            computation_id,
            result,
            result_hash,
            verification_proof,
            completion_timestamp: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Emit event
        event::emit(ComputationCompletedEvent {
            computation_id,
            result_hash,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
        
        // Transfer result to original requester
        // In a real implementation, we would need to look up the requester
        transfer::transfer(result_obj, tx_context::sender(ctx));
        
        // Decrement active computations
        enclave.active_computations = enclave.active_computations - 1;
    }
    
    // === MPC Node Management ===
    
    /// Register an MPC computation node
    public entry fun register_node(
        enclave: &mut SecureEnclave,
        public_key: vector<u8>,
        stake_amount: u64,
        ctx: &mut TxContext
    ) {
        // Create node ID
        let node_id_obj = object::new(ctx);
        let node_id = object::uid_to_inner(&node_id_obj);
        
        // Create node info
        let node_info = NodeInfo {
            id: node_id,
            public_key,
            operator: tx_context::sender(ctx),
            stake: stake_amount,
            is_active: true,
        };
        
        // Add to enclave
        table::add(&mut enclave.nodes, node_id, node_info);
        
        // Delete the node ID object (it was just used to generate an ID)
        object::delete(node_id_obj);
    }
    
    // === Public Query Functions ===
    
    /// Get agent info
    public fun get_agent_info(registry: &AgentRegistry, agent_id: ID): (bool, u8, String, String, address, bool) {
        if (table::contains(&registry.agents, agent_id)) {
            let agent = table::borrow(&registry.agents, agent_id);
            (
                true,
                agent.agent_type,
                agent.name,
                agent.description,
                agent.owner,
                agent.is_certified
            )
        } else {
            (
                false,
                0,
                string::utf8(b""),
                string::utf8(b""),
                @0x0,
                false
            )
        }
    }
    
    /// Get agents by type
    public fun get_agents_by_type(registry: &AgentRegistry, agent_type: u8): vector<ID> {
        if (table::contains(&registry.agents_by_type, agent_type)) {
            *table::borrow(&registry.agents_by_type, agent_type)
        } else {
            vector::empty()
        }
    }
    
    /// Get computation status
    public fun get_computation_status(enclave: &SecureEnclave, computation_id: ID): (bool, u8, u64, u64) {
        if (table::contains(&enclave.computations, computation_id)) {
            let computation = table::borrow(&enclave.computations, computation_id);
            (
                true,
                computation.status,
                computation.request_timestamp,
                computation.completion_timestamp
            )
        } else {
            (false, 0, 0, 0)
        }
    }
    
    /// Get number of active computations
    public fun get_active_computations(enclave: &SecureEnclave): u64 {
        enclave.active_computations
    }
    
    // === Type Constants ===
    
    /// Get recommendation agent type constant
    public fun recommendation_agent_type(): u8 {
        AGENT_TYPE_RECOMMENDATION
    }
    
    /// Get moderation agent type constant
    public fun moderation_agent_type(): u8 {
        AGENT_TYPE_MODERATION
    }
    
    /// Get trend analysis agent type constant
    public fun trend_analysis_agent_type(): u8 {
        AGENT_TYPE_TREND_ANALYSIS
    }
    
    /// Get custom agent type constant
    public fun custom_agent_type(): u8 {
        AGENT_TYPE_CUSTOM
    }
    
    /// Get private computation type constant
    public fun private_computation_type(): u8 {
        COMPUTATION_PRIVATE
    }
    
    /// Get threshold computation type constant
    public fun threshold_computation_type(): u8 {
        COMPUTATION_THRESHOLD
    }
    
    /// Get federated computation type constant
    public fun federated_computation_type(): u8 {
        COMPUTATION_FEDERATED
    }
    
    /// Get MPC computation type constant
    public fun mpc_computation_type(): u8 {
        COMPUTATION_MPC
    }
}