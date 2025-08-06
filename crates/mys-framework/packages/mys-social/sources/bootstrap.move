// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Lightweight bootstrap service for MySocial genesis bootstrap
/// One function to claim all admin capabilities AND auto-configure treasuries.

#[allow(duplicate_alias, unused_use)]
module social_contracts::bootstrap {
    use mys::{
        object::{Self, UID},
        tx_context::{Self, TxContext},
        transfer,
        package::{Self, Publisher}
    };
    
    // Import admin capability types and modules
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::social_proof_tokens::{Self, SocialProofTokensAdminCap};
    use social_contracts::post::{Self, PostAdminCap};
    use social_contracts::proof_of_creativity::{Self, PoCAdminCap};
    use social_contracts::platform::{Self, PlatformAdminCap};
    use social_contracts::governance::{Self, GovernanceAdminCap};
    
    // === ERROR CODES ===
    const ENotAuthorized: u64 = 0;
    const EAlreadyUsed: u64 = 1;
    
    /// One-time bootstrap key - can only be used once, ever
    public struct BootstrapKey has key {
        id: UID,
        /// Whether this key has been used
        used: bool,
        /// Version for future compatibility
        version: u64,
    }
    
    /// Initialize the bootstrap service - creates the one-time bootstrap key
    fun init(ctx: &mut TxContext) {
        transfer::share_object(BootstrapKey {
            id: object::new(ctx),
            used: false,
            version: upgrade::current_version(),
        });
    }
    
    /// Claim all admin capabilities and auto-configure treasuries - ONE FUNCTION, DONE FOREVER
    /// This function creates and transfers all admin capabilities to the caller,
    /// automatically configures all treasury addresses to the caller's address,
    /// then permanently seals the bootstrap key to prevent future use.
    /// 
    /// Security: 
    /// - Can only be called once in the history of the blockchain
    /// - Requires valid Publisher capability
    /// - Transfers all admin rights to the caller
    /// - Auto-configures all treasuries to caller's address
    public entry fun claim_all_admin_capabilities(
        key: &mut BootstrapKey,
        publisher: &Publisher,
        social_proof_tokens_config: &mut social_contracts::social_proof_tokens::SocialProofTokensConfig,
        post_config: &mut social_contracts::post::PostConfig,
        poc_config: &mut social_contracts::proof_of_creativity::PoCConfig,
        ctx: &mut TxContext
    ) {
        // === SECURITY CHECKS ===
        
        // Ensure this can only be called once, ever
        assert!(!key.used, EAlreadyUsed);
        
        // Verify caller has valid publisher capability for this package
        assert!(package::from_package<BootstrapKey>(publisher), ENotAuthorized);
        
        let admin = tx_context::sender(ctx);
        
        // === CREATE ALL ADMIN CAPABILITIES ===
        // Creates and transfers 6 admin capabilities to solve genesis deployment issue:
        // 1. UpgradeAdminCap - Package upgrades
        // 2. SocialProofTokensAdminCap - Social proof tokens system configuration
        // 3. PostAdminCap - Post system configuration  
        // 4. PoCAdminCap - Proof of Creativity configuration
        // 5. PlatformAdminCap - Platform approval and management
        // 6. GovernanceAdminCap - Governance parameter updates
        
        // Create UpgradeAdminCap for package upgrades
        let upgrade_admin_cap = upgrade::create_upgrade_admin_cap(ctx);
        transfer::public_transfer(upgrade_admin_cap, admin);
        
        // Create SocialProofTokensAdminCap for social proof tokens administration
        let social_proof_tokens_admin_cap = social_proof_tokens::create_social_proof_tokens_admin_cap(ctx);
        transfer::public_transfer(social_proof_tokens_admin_cap, admin);
        
        // Create PostAdminCap for post system administration
        let post_admin_cap = post::create_post_admin_cap(ctx);
        transfer::public_transfer(post_admin_cap, admin);
        
        // Create PoCAdminCap for Proof of Creativity administration
        let poc_admin_cap = proof_of_creativity::create_poc_admin_cap(ctx);
        transfer::public_transfer(poc_admin_cap, admin);
        
        // Create PlatformAdminCap for platform administration
        let platform_admin_cap = platform::create_platform_admin_cap(ctx);
        transfer::public_transfer(platform_admin_cap, admin);
        
        // Create GovernanceAdminCap for governance administration
        let governance_admin_cap = governance::create_governance_admin_cap(ctx);
        transfer::public_transfer(governance_admin_cap, admin);
        
        // === AUTO-CONFIGURE ALL TREASURIES ===
        // Automatically set all treasury addresses to the admin's address
        // This eliminates the need for manual configuration after bootstrap
        
        // Configure social proof tokens ecosystem treasury
        social_proof_tokens::auto_configure_treasury(social_proof_tokens_config, admin);
        
        // Configure post prediction treasury
        post::auto_configure_prediction_treasury(post_config, admin);
        
        // Configure proof of creativity ecosystem treasury
        proof_of_creativity::auto_configure_ecosystem_treasury(poc_config, admin);
        
        // Enable trading - system is now fully configured and ready for use
        social_proof_tokens::auto_enable_trading(social_proof_tokens_config);
        
        // === PERMANENT SEAL ===
        
        // Mark the bootstrap key as used - this cannot be undone
        key.used = true;
        
        // Note: The BootstrapKey object remains shared but is now permanently unusable
        // This provides a permanent on-chain record that bootstrap has occurred
    }
    
    // === UTILITY FUNCTIONS ===
    
    /// Check if the bootstrap key has been used
    public fun is_used(key: &BootstrapKey): bool {
        key.used
    }
    
    /// Get the version of the bootstrap key
    public fun version(key: &BootstrapKey): u64 {
        key.version
    }
    
    // === TEST-ONLY FUNCTIONS ===
    
    #[test_only]
    /// Initialize the bootstrap for testing
    public fun init_for_testing(ctx: &mut TxContext) {
        init(ctx)
    }
    
    #[test_only]
    /// Create a test bootstrap key (for testing only)
    public fun create_test_bootstrap_key(ctx: &mut TxContext): BootstrapKey {
        BootstrapKey {
            id: object::new(ctx),
            used: false,
            version: 1,
        }
    }
    
    #[test_only]
    /// Mark a bootstrap key as used (for testing only)
    public fun mark_used_for_testing(key: &mut BootstrapKey) {
        key.used = true;
    }
}