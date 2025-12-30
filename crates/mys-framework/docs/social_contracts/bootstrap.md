---
title: Module `social_contracts::bootstrap`
---

Bootstrap service for MySocial - claims all admin capabilities in one call.
Uses the framework's centralized BootstrapKey for one-time initialization.


-  [Function `claim_all_admin_capabilities`](#social_contracts_bootstrap_claim_all_admin_capabilities)
-  [Function `is_bootstrap_used`](#social_contracts_bootstrap_is_bootstrap_used)
-  [Function `bootstrap_version`](#social_contracts_bootstrap_bootstrap_version)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
<b>use</b> <a href="../mydata/polynomial.md#mydata_polynomial">mydata::polynomial</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key">mys::bootstrap_key</a>;
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
<b>use</b> <a href="../mys/vec_map.md#mys_vec_map">mys::vec_map</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/message.md#social_contracts_message">social_contracts::message</a>;
<b>use</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity">social_contracts::proof_of_creativity</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth">social_contracts::social_proof_of_truth</a>;
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



<a name="social_contracts_bootstrap_claim_all_admin_capabilities"></a>

## Function `claim_all_admin_capabilities`

Claim all admin capabilities (one-time only)
Creates and transfers all admin capabilities to caller, then seals the bootstrap key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_claim_all_admin_capabilities">claim_all_admin_capabilities</a>(bootstrap_key: &<b>mut</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_claim_all_admin_capabilities">claim_all_admin_capabilities</a>(
    bootstrap_key: &<b>mut</b> BootstrapKey,
    ctx: &<b>mut</b> TxContext
) {
    bootstrap_key::assert_not_used(bootstrap_key);
    <b>let</b> admin = tx_context::sender(ctx);
    // Initialize shared objects
    <a href="../social_contracts/platform.md#social_contracts_platform_bootstrap_init">social_contracts::platform::bootstrap_init</a>(ctx);
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_bootstrap_init">social_contracts::social_graph::bootstrap_init</a>(ctx);
    <a href="../social_contracts/profile.md#social_contracts_profile_bootstrap_init">social_contracts::profile::bootstrap_init</a>(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_bootstrap_init">social_contracts::block_list::bootstrap_init</a>(ctx);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_bootstrap_init">social_contracts::mydata::bootstrap_init</a>(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">social_contracts::governance::bootstrap_init</a>(ctx);
    <a href="../social_contracts/post.md#social_contracts_post_bootstrap_init">social_contracts::post::bootstrap_init</a>(ctx);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_init">social_contracts::social_proof_tokens::bootstrap_init</a>(ctx);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_bootstrap_init">social_contracts::proof_of_creativity::bootstrap_init</a>(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_bootstrap_init">social_contracts::message::bootstrap_init</a>(ctx);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_bootstrap_init">social_contracts::social_proof_of_truth::bootstrap_init</a>(ctx);
    // Create admin capabilities
    transfer::public_transfer(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">upgrade::create_upgrade_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap">social_proof_tokens::create_social_proof_tokens_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">post::create_post_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_create_poc_admin_cap">proof_of_creativity::create_poc_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">platform::create_platform_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">governance::create_governance_admin_cap</a>(ctx), admin);
    transfer::public_transfer(mydata::create_mydata_admin_cap(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_admin_cap">social_proof_of_truth::create_spot_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap">social_proof_of_truth::create_spot_oracle_admin_cap</a>(ctx), admin);
    transfer::public_transfer(coin::create_coin_creation_admin_cap_for_bootstrap(ctx), admin);
    // Seal the <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> key permanently (prevents any future <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> attempts)
    bootstrap_key::finalize_bootstrap(bootstrap_key);
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_is_bootstrap_used"></a>

## Function `is_bootstrap_used`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_is_bootstrap_used">is_bootstrap_used</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_is_bootstrap_used">is_bootstrap_used</a>(key: &BootstrapKey): bool {
    bootstrap_key::is_used(key)
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_bootstrap_version"></a>

## Function `bootstrap_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_bootstrap_version">bootstrap_version</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_bootstrap_version">bootstrap_version</a>(key: &BootstrapKey): u64 {
    bootstrap_key::version(key)
}
</code></pre>



</details>
