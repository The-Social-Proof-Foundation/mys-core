---
title: Module `social_contracts::bootstrap`
---

Lightweight bootstrap service for MySocial genesis bootstrap
One function to claim all admin capabilities AND auto-configure treasuries.


-  [Struct `BootstrapKey`](#social_contracts_bootstrap_BootstrapKey)
-  [Constants](#@Constants_0)
-  [Function `init`](#social_contracts_bootstrap_init)
-  [Function `claim_all_admin_capabilities`](#social_contracts_bootstrap_claim_all_admin_capabilities)
-  [Function `is_used`](#social_contracts_bootstrap_is_used)
-  [Function `version`](#social_contracts_bootstrap_version)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/clock.md#mys_clock">mys::clock</a>;
<b>use</b> <a href="../mys/coin.md#mys_coin">mys::coin</a>;
<b>use</b> <a href="../mys/config.md#mys_config">mys::config</a>;
<b>use</b> <a href="../mys/deny_list.md#mys_deny_list">mys::deny_list</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/dynamic_object_field.md#mys_dynamic_object_field">mys::dynamic_object_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/group_ops.md#mys_group_ops">mys::group_ops</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/hmac.md#mys_hmac">mys::hmac</a>;
<b>use</b> <a href="../mys/math.md#mys_math">mys::math</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption">seal::bf_hmac_encryption</a>;
<b>use</b> <a href="../seal/gf256.md#seal_gf256">seal::gf256</a>;
<b>use</b> <a href="../seal/hmac256ctr.md#seal_hmac256ctr">seal::hmac256ctr</a>;
<b>use</b> <a href="../seal/kdf.md#seal_kdf">seal::kdf</a>;
<b>use</b> <a href="../seal/key_server.md#seal_key_server">seal::key_server</a>;
<b>use</b> <a href="../seal/polynomial.md#seal_polynomial">seal::polynomial</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity">social_contracts::proof_of_creativity</a>;
<b>use</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens">social_contracts::social_proof_tokens</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_bootstrap_BootstrapKey"></a>

## Struct `BootstrapKey`

One-time bootstrap key - can only be used once, ever


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>used: bool</code>
</dt>
<dd>
 Whether this key has been used
</dd>
<dt>
<code><a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_version">version</a>: u64</code>
</dt>
<dd>
 Version for future compatibility
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_bootstrap_EAlreadyUsed"></a>



<pre><code><b>const</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_EAlreadyUsed">EAlreadyUsed</a>: u64 = 1;
</code></pre>



<a name="social_contracts_bootstrap_ENotAuthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_ENotAuthorized">ENotAuthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_bootstrap_init"></a>

## Function `init`

Initialize the bootstrap service - creates the one-time bootstrap key


<pre><code><b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_init">init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_init">init</a>(ctx: &<b>mut</b> TxContext) {
    transfer::share_object(<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a> {
        id: object::new(ctx),
        used: <b>false</b>,
        <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    });
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_claim_all_admin_capabilities"></a>

## Function `claim_all_admin_capabilities`

Claim all admin capabilities and auto-configure treasuries - ONE FUNCTION, DONE FOREVER
This function creates and transfers all admin capabilities to the caller,
automatically configures all treasury addresses to the caller's address,
then permanently seals the bootstrap key to prevent future use.

Security:
- Can only be called once in the history of the blockchain
- Requires valid Publisher capability
- Transfers all admin rights to the caller
- Auto-configures all treasuries to caller's address


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_claim_all_admin_capabilities">claim_all_admin_capabilities</a>(key: &<b>mut</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">social_contracts::bootstrap::BootstrapKey</a>, publisher: &<a href="../mys/package.md#mys_package_Publisher">mys::package::Publisher</a>, social_proof_tokens_config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, post_config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, poc_config: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_claim_all_admin_capabilities">claim_all_admin_capabilities</a>(
    key: &<b>mut</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a>,
    publisher: &Publisher,
    social_proof_tokens_config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>,
    post_config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>,
    poc_config: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>,
    ctx: &<b>mut</b> TxContext
) {
    // === SECURITY CHECKS ===
    // Ensure this can only be called once, ever
    <b>assert</b>!(!key.used, <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_EAlreadyUsed">EAlreadyUsed</a>);
    // Verify caller <b>has</b> valid publisher capability <b>for</b> this package
    <b>assert</b>!(package::from_package&lt;<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a>&gt;(publisher), <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_ENotAuthorized">ENotAuthorized</a>);
    <b>let</b> admin = tx_context::sender(ctx);
    // === CREATE ALL ADMIN CAPABILITIES ===
    // Creates and transfers 6 admin capabilities to solve genesis deployment issue:
    // 1. UpgradeAdminCap - Package upgrades
    // 2. SocialProofTokensAdminCap - Social proof tokens system configuration
    // 3. PostAdminCap - Post system configuration
    // 4. PoCAdminCap - Proof of Creativity configuration
    // 5. PlatformAdminCap - Platform approval and management
    // 6. GovernanceAdminCap - Governance parameter updates
    // Create UpgradeAdminCap <b>for</b> package upgrades
    <b>let</b> upgrade_admin_cap = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">upgrade::create_upgrade_admin_cap</a>(ctx);
    transfer::public_transfer(upgrade_admin_cap, admin);
    // Create SocialProofTokensAdminCap <b>for</b> social proof tokens administration
    <b>let</b> social_proof_tokens_admin_cap = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap">social_proof_tokens::create_social_proof_tokens_admin_cap</a>(ctx);
    transfer::public_transfer(social_proof_tokens_admin_cap, admin);
    // Create PostAdminCap <b>for</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> system administration
    <b>let</b> post_admin_cap = <a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">post::create_post_admin_cap</a>(ctx);
    transfer::public_transfer(post_admin_cap, admin);
    // Create PoCAdminCap <b>for</b> Proof of Creativity administration
    <b>let</b> poc_admin_cap = <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_create_poc_admin_cap">proof_of_creativity::create_poc_admin_cap</a>(ctx);
    transfer::public_transfer(poc_admin_cap, admin);
    // Create PlatformAdminCap <b>for</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> administration
    <b>let</b> platform_admin_cap = <a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">platform::create_platform_admin_cap</a>(ctx);
    transfer::public_transfer(platform_admin_cap, admin);
    // Create GovernanceAdminCap <b>for</b> <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> administration
    <b>let</b> governance_admin_cap = <a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">governance::create_governance_admin_cap</a>(ctx);
    transfer::public_transfer(governance_admin_cap, admin);
    // === AUTO-CONFIGURE ALL TREASURIES ===
    // Automatically set all treasury addresses to the admin's <b>address</b>
    // This eliminates the need <b>for</b> manual configuration after <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a>
    // Configure social proof tokens ecosystem treasury
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_auto_configure_treasury">social_proof_tokens::auto_configure_treasury</a>(social_proof_tokens_config, admin);
    // Configure <a href="../social_contracts/post.md#social_contracts_post">post</a> prediction treasury
    <a href="../social_contracts/post.md#social_contracts_post_auto_configure_prediction_treasury">post::auto_configure_prediction_treasury</a>(post_config, admin);
    // Configure proof of creativity ecosystem treasury
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_auto_configure_ecosystem_treasury">proof_of_creativity::auto_configure_ecosystem_treasury</a>(poc_config, admin);
    // Enable trading - system is now fully configured and ready <b>for</b> <b>use</b>
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_auto_enable_trading">social_proof_tokens::auto_enable_trading</a>(social_proof_tokens_config);
    // === PERMANENT SEAL ===
    // Mark the <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> key <b>as</b> used - this cannot be undone
    key.used = <b>true</b>;
    // Note: The <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a> object remains shared but is now permanently unusable
    // This provides a permanent on-chain record that <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> <b>has</b> occurred
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_is_used"></a>

## Function `is_used`

Check if the bootstrap key has been used


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_is_used">is_used</a>(key: &<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">social_contracts::bootstrap::BootstrapKey</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_is_used">is_used</a>(key: &<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a>): bool {
    key.used
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_version"></a>

## Function `version`

Get the version of the bootstrap key


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_version">version</a>(key: &<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">social_contracts::bootstrap::BootstrapKey</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_version">version</a>(key: &<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_BootstrapKey">BootstrapKey</a>): u64 {
    key.<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_version">version</a>
}
</code></pre>



</details>
