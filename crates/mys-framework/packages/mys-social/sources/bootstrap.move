// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Bootstrap service for MySocial - claims all admin capabilities in one call.
/// Uses the framework's centralized BootstrapKey for one-time initialization.

#[allow(duplicate_alias, unused_use)]
module social_contracts::bootstrap {
    use mys::{
        tx_context::{Self, TxContext},
        transfer,
        bootstrap_key::{Self, BootstrapKey}
    };
    
    // Import admin capability types and modules
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::social_proof_tokens::{Self, SocialProofTokensAdminCap};
    use social_contracts::post::{Self, PostAdminCap};
    use social_contracts::proof_of_creativity::{Self, PoCAdminCap};
    use social_contracts::platform::{Self, PlatformAdminCap};
    use social_contracts::governance::{Self, GovernanceAdminCap};
    use social_contracts::mydata::{Self, MyDataAdminCap};
    use social_contracts::social_proof_of_truth::{Self, SpotAdminCap, SpotOracleAdminCap};
    use social_contracts::insurance::{Self, InsuranceAdminCap};
    use social_contracts::profile::{Self, EcosystemTreasuryAdminCap};
    
    // Import framework coin module for coin creation admin cap
    use mys::coin::{Self, CoinCreationAdminCap};
    
    /// Claim all admin capabilities (one-time only)
    /// Creates and transfers all admin capabilities to caller, then seals the bootstrap key.
    public entry fun claim_all_admin_capabilities(
        bootstrap_key: &mut BootstrapKey,
        ctx: &mut TxContext
    ) {
        bootstrap_key::assert_not_used(bootstrap_key);
        let admin = tx_context::sender(ctx);
        
        // Initialize shared objects
        social_contracts::platform::bootstrap_init(ctx);
        social_contracts::social_graph::bootstrap_init(ctx);
        social_contracts::profile::bootstrap_init(ctx);
        social_contracts::block_list::bootstrap_init(ctx);
        social_contracts::mydata::bootstrap_init(ctx);
        social_contracts::governance::bootstrap_init(ctx);
        social_contracts::post::bootstrap_init(ctx);
        social_contracts::social_proof_tokens::bootstrap_init(ctx);
        social_contracts::proof_of_creativity::bootstrap_init(ctx);
        social_contracts::message::bootstrap_init(ctx);
        social_contracts::social_proof_of_truth::bootstrap_init(ctx);
        social_contracts::insurance::bootstrap_init(ctx);
        
        // Create admin capabilities
        transfer::public_transfer(upgrade::create_upgrade_admin_cap(ctx), admin);
        transfer::public_transfer(social_proof_tokens::create_social_proof_tokens_admin_cap(ctx), admin);
        transfer::public_transfer(post::create_post_admin_cap(ctx), admin);
        transfer::public_transfer(proof_of_creativity::create_poc_admin_cap(ctx), admin);
        transfer::public_transfer(platform::create_platform_admin_cap(ctx), admin);
        transfer::public_transfer(governance::create_governance_admin_cap(ctx), admin);
        transfer::public_transfer(mydata::create_mydata_admin_cap(ctx), admin);
        transfer::public_transfer(social_proof_of_truth::create_spot_admin_cap(ctx), admin);
        transfer::public_transfer(social_proof_of_truth::create_spot_oracle_admin_cap(ctx), admin);
        transfer::public_transfer(profile::create_ecosystem_treasury_admin_cap(ctx), admin);
        transfer::public_transfer(insurance::create_insurance_admin_cap(ctx), admin);
        transfer::public_transfer(coin::create_coin_creation_admin_cap_for_bootstrap(ctx), admin);

        // Seal the bootstrap key permanently (prevents any future bootstrap attempts)
        bootstrap_key::finalize_bootstrap(bootstrap_key);
    }
    
    public fun is_bootstrap_used(key: &BootstrapKey): bool {
        bootstrap_key::is_used(key)
    }
    
    public fun bootstrap_version(key: &BootstrapKey): u64 {
        bootstrap_key::version(key)
    }
}
