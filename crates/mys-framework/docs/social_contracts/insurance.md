---
title: Module `social_contracts::insurance`
---

Insurance module for SPoT positions
Sells coverage against losing outcomes and pays out deterministically on SPoT resolution.


-  [Struct `InsuranceAdminCap`](#social_contracts_insurance_InsuranceAdminCap)
-  [Struct `InsuranceConfig`](#social_contracts_insurance_InsuranceConfig)
-  [Struct `UnderwriterVault`](#social_contracts_insurance_UnderwriterVault)
-  [Struct `MarketExposure`](#social_contracts_insurance_MarketExposure)
-  [Struct `CoveragePolicy`](#social_contracts_insurance_CoveragePolicy)
-  [Struct `ConfigInitializedEvent`](#social_contracts_insurance_ConfigInitializedEvent)
-  [Struct `UnderwriterVaultCreatedEvent`](#social_contracts_insurance_UnderwriterVaultCreatedEvent)
-  [Struct `UnderwriterVaultDepositedEvent`](#social_contracts_insurance_UnderwriterVaultDepositedEvent)
-  [Struct `UnderwriterVaultWithdrawnEvent`](#social_contracts_insurance_UnderwriterVaultWithdrawnEvent)
-  [Struct `CoveragePurchasedEvent`](#social_contracts_insurance_CoveragePurchasedEvent)
-  [Struct `CoverageCancelledEvent`](#social_contracts_insurance_CoverageCancelledEvent)
-  [Struct `CoverageClaimedEvent`](#social_contracts_insurance_CoverageClaimedEvent)
-  [Struct `ConfigUpdatedEvent`](#social_contracts_insurance_ConfigUpdatedEvent)
-  [Struct `PolicyExpiredEvent`](#social_contracts_insurance_PolicyExpiredEvent)
-  [Constants](#@Constants_0)
-  [Function `init_config`](#social_contracts_insurance_init_config)
-  [Function `set_config`](#social_contracts_insurance_set_config)
-  [Function `set_paused`](#social_contracts_insurance_set_paused)
-  [Function `create_insurance_admin_cap`](#social_contracts_insurance_create_insurance_admin_cap)
-  [Function `bootstrap_init`](#social_contracts_insurance_bootstrap_init)
-  [Function `create_vault`](#social_contracts_insurance_create_vault)
-  [Function `deposit_capital`](#social_contracts_insurance_deposit_capital)
-  [Function `withdraw_capital`](#social_contracts_insurance_withdraw_capital)
-  [Function `quote_premium`](#social_contracts_insurance_quote_premium)
-  [Function `buy_coverage`](#social_contracts_insurance_buy_coverage)
-  [Function `cancel_coverage`](#social_contracts_insurance_cancel_coverage)
-  [Function `claim`](#social_contracts_insurance_claim)
-  [Function `expire_policy`](#social_contracts_insurance_expire_policy)
-  [Function `compute_reserve`](#social_contracts_insurance_compute_reserve)
-  [Function `enforce_exposure_limits`](#social_contracts_insurance_enforce_exposure_limits)
-  [Function `add_exposure`](#social_contracts_insurance_add_exposure)
-  [Function `release_exposure`](#social_contracts_insurance_release_exposure)
-  [Function `get_market_exposure_mut`](#social_contracts_insurance_get_market_exposure_mut)
-  [Function `get_user_exposure`](#social_contracts_insurance_get_user_exposure)
-  [Function `set_user_exposure`](#social_contracts_insurance_set_user_exposure)
-  [Function `get_option_reserved`](#social_contracts_insurance_get_option_reserved)
-  [Function `set_option_reserved`](#social_contracts_insurance_set_option_reserved)


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
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth">social_contracts::social_proof_of_truth</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_insurance_InsuranceAdminCap"></a>

## Struct `InsuranceAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_InsuranceConfig"></a>

## Struct `InsuranceConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> <b>has</b> key
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
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>min_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVault"></a>

## Struct `UnderwriterVault`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a> <b>has</b> key
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
<code>underwriter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>capital: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_rate_bps_per_day: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>utilization_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>market_exposures: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>user_exposures: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_MarketExposure"></a>

## Struct `MarketExposure`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_by_option: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u8, u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoveragePolicy"></a>

## Struct `CoveragePolicy`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a> <b>has</b> key, store
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
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>covered_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_paid: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>start_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_ConfigInitializedEvent"></a>

## Struct `ConfigInitializedEvent`

Events


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigInitializedEvent">ConfigInitializedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>min_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVaultCreatedEvent"></a>

## Struct `UnderwriterVaultCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultCreatedEvent">UnderwriterVaultCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>underwriter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>base_rate_bps_per_day: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>utilization_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_user: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVaultDepositedEvent"></a>

## Struct `UnderwriterVaultDepositedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultDepositedEvent">UnderwriterVaultDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVaultWithdrawnEvent"></a>

## Struct `UnderwriterVaultWithdrawnEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultWithdrawnEvent">UnderwriterVaultWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoveragePurchasedEvent"></a>

## Struct `CoveragePurchasedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePurchasedEvent">CoveragePurchasedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>covered_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_paid: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserve_locked: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoverageCancelledEvent"></a>

## Struct `CoverageCancelledEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageCancelledEvent">CoverageCancelledEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_paid: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoverageClaimedEvent"></a>

## Struct `CoverageClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageClaimedEvent">CoverageClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payout: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_ConfigUpdatedEvent"></a>

## Struct `ConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>min_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_PolicyExpiredEvent"></a>

## Struct `PolicyExpiredEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_PolicyExpiredEvent">PolicyExpiredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reserve_released: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_insurance_BPS_DENOM"></a>

Constants


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_insurance_DAY_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DAY_MS">DAY_MS</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>: u64 = 50;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS">DEFAULT_MAX_COVERAGE_BPS</a>: u64 = 9000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MAX_DURATION_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_DURATION_MS">DEFAULT_MAX_DURATION_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS">DEFAULT_MIN_COVERAGE_BPS</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>: u64 = 1;
</code></pre>



<a name="social_contracts_insurance_EExposureInvariantBroken"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>: u64 = 16;
</code></pre>



<a name="social_contracts_insurance_EExposureLimit"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>: u64 = 14;
</code></pre>



<a name="social_contracts_insurance_EInsufficientCapital"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>: u64 = 7;
</code></pre>



<a name="social_contracts_insurance_EInsufficientPremium"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientPremium">EInsufficientPremium</a>: u64 = 15;
</code></pre>



<a name="social_contracts_insurance_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>: u64 = 5;
</code></pre>



<a name="social_contracts_insurance_EInvalidCoverage"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>: u64 = 3;
</code></pre>



<a name="social_contracts_insurance_EInvalidDuration"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>: u64 = 4;
</code></pre>



<a name="social_contracts_insurance_EInvalidVault"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidVault">EInvalidVault</a>: u64 = 6;
</code></pre>



<a name="social_contracts_insurance_EMarketClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>: u64 = 8;
</code></pre>



<a name="social_contracts_insurance_EMarketMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketMismatch">EMarketMismatch</a>: u64 = 13;
</code></pre>



<a name="social_contracts_insurance_ENotAdmin"></a>

Errors


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotAdmin">ENotAdmin</a>: u64 = 1;
</code></pre>



<a name="social_contracts_insurance_ENotPolicyOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotPolicyOwner">ENotPolicyOwner</a>: u64 = 11;
</code></pre>



<a name="social_contracts_insurance_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>: u64 = 12;
</code></pre>



<a name="social_contracts_insurance_EPaused"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EPaused">EPaused</a>: u64 = 2;
</code></pre>



<a name="social_contracts_insurance_EPolicyExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyExpired">EPolicyExpired</a>: u64 = 10;
</code></pre>



<a name="social_contracts_insurance_EPolicyNotActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyNotActive">EPolicyNotActive</a>: u64 = 9;
</code></pre>



<a name="social_contracts_insurance_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_insurance_STATUS_ACTIVE"></a>

Status


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_insurance_STATUS_CANCELLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CANCELLED">STATUS_CANCELLED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_insurance_STATUS_CLAIMED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CLAIMED">STATUS_CLAIMED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_insurance_STATUS_EXPIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_EXPIRED">STATUS_EXPIRED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_insurance_init_config"></a>

## Function `init_config`

Initialize config (package only)
Creates InsuranceConfig and transfers InsuranceAdminCap to caller.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_init_config">init_config</a>(min_coverage_bps: u64, max_coverage_bps: u64, max_duration_ms: u64, fee_bps: u64, treasury: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_init_config">init_config</a>(
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
    treasury: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(min_coverage_bps &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(min_coverage_bps &lt;= max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_coverage_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_duration_ms &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(fee_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>let</b> admin = tx_context::sender(ctx);
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> {
        id: object::new(ctx),
        paused: <b>true</b>,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        treasury,
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    });
    transfer::public_transfer(<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> { id: object::new(ctx) }, admin);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigInitializedEvent">ConfigInitializedEvent</a> {
        admin,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        treasury,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_config"></a>

## Function `set_config`

Update config (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_config">set_config</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, min_coverage_bps: u64, max_coverage_bps: u64, max_duration_ms: u64, fee_bps: u64, treasury: <b>address</b>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_config">set_config</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
    treasury: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(min_coverage_bps &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(min_coverage_bps &lt;= max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_coverage_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_duration_ms &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(fee_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    config.min_coverage_bps = min_coverage_bps;
    config.max_coverage_bps = max_coverage_bps;
    config.max_duration_ms = max_duration_ms;
    config.fee_bps = fee_bps;
    config.treasury = treasury;
    <b>let</b> updated_by = tx_context::sender(ctx);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by,
        paused: config.paused,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        treasury,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_paused"></a>

## Function `set_paused`

Emergency pause toggle (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_paused">set_paused</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, paused: bool, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_paused">set_paused</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    paused: bool,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    config.paused = paused;
    <b>let</b> updated_by = tx_context::sender(ctx);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by,
        paused: config.paused,
        min_coverage_bps: config.min_coverage_bps,
        max_coverage_bps: config.max_coverage_bps,
        max_duration_ms: config.max_duration_ms,
        fee_bps: config.fee_bps,
        treasury: config.treasury,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_create_insurance_admin_cap"></a>

## Function `create_insurance_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_insurance_admin_cap">create_insurance_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_insurance_admin_cap">create_insurance_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> {
    <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_insurance_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    // Create and share the <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> object with default values
    // Admin cap will be transferred separately in <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a>.<b>move</b>
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> {
        id: object::new(ctx),
        paused: <b>true</b>,
        min_coverage_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS">DEFAULT_MIN_COVERAGE_BPS</a>,
        max_coverage_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS">DEFAULT_MAX_COVERAGE_BPS</a>,
        max_duration_ms: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_DURATION_MS">DEFAULT_MAX_DURATION_MS</a>,
        fee_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>,
        treasury: admin,
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_create_vault"></a>

## Function `create_vault`

Create an underwriter vault


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_vault">create_vault</a>(base_rate_bps_per_day: u64, utilization_multiplier_bps: u64, max_exposure_per_market: u64, max_exposure_per_user: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_vault">create_vault</a>(
    base_rate_bps_per_day: u64,
    utilization_multiplier_bps: u64,
    max_exposure_per_market: u64,
    max_exposure_per_user: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> underwriter = tx_context::sender(ctx);
    <b>let</b> vault = <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a> {
        id: object::new(ctx),
        underwriter,
        capital: balance::zero(),
        reserved: 0,
        base_rate_bps_per_day,
        utilization_multiplier_bps,
        max_exposure_per_market,
        max_exposure_per_user,
        market_exposures: table::new(ctx),
        user_exposures: table::new(ctx),
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    };
    <b>let</b> vault_id = object::id(&vault);
    transfer::share_object(vault);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultCreatedEvent">UnderwriterVaultCreatedEvent</a> {
        vault_id,
        underwriter,
        base_rate_bps_per_day,
        utilization_multiplier_bps,
        max_exposure_per_market,
        max_exposure_per_user,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_deposit_capital"></a>

## Function `deposit_capital`

Deposit capital into vault


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_deposit_capital">deposit_capital</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_deposit_capital">deposit_capital</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    payment: Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!config.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPaused">EPaused</a>);
    <b>let</b> deposit_amount = coin::value(&payment);
    <b>assert</b>!(deposit_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    balance::join(&<b>mut</b> vault.capital, coin::into_balance(payment));
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultDepositedEvent">UnderwriterVaultDepositedEvent</a> {
        vault_id: object::id(vault),
        amount: deposit_amount,
        new_balance: balance::value(&vault.capital),
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_withdraw_capital"></a>

## Function `withdraw_capital`

Withdraw unreserved capital (underwriter only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_withdraw_capital">withdraw_capital</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_withdraw_capital">withdraw_capital</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!config.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPaused">EPaused</a>);
    <b>assert</b>!(tx_context::sender(ctx) == vault.underwriter, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotAdmin">ENotAdmin</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>assert</b>!(capital_value &gt;= vault.reserved, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> free_capital = capital_value - vault.reserved;
    <b>assert</b>!(free_capital &gt;= amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    <b>let</b> payout_balance = balance::split(&<b>mut</b> vault.capital, amount);
    <b>let</b> payout_coin = coin::from_balance(payout_balance, ctx);
    transfer::public_transfer(payout_coin, vault.underwriter);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultWithdrawnEvent">UnderwriterVaultWithdrawnEvent</a> {
        vault_id: object::id(vault),
        amount,
        new_balance: balance::value(&vault.capital),
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_quote_premium"></a>

## Function `quote_premium`

Quote premium based on vault utilization


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium">quote_premium</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, covered_amount: u64, coverage_bps: u64, duration_ms: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium">quote_premium</a>(
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    covered_amount: u64,
    coverage_bps: u64,
    duration_ms: u64
): u64 {
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>let</b> utilization_bps = <b>if</b> (capital_value == 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>
    } <b>else</b> {
        <b>let</b> utilization_u128 = (vault.reserved <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (capital_value <b>as</b> u128);
        <b>assert</b>!(utilization_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        utilization_u128 <b>as</b> u64
    };
    <b>let</b> utilization_factor = (utilization_bps * vault.utilization_multiplier_bps) / <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> total_rate_bps_per_day = vault.base_rate_bps_per_day + utilization_factor;
    <b>let</b> numerator = (covered_amount <b>as</b> u128)
        * (coverage_bps <b>as</b> u128)
        * (total_rate_bps_per_day <b>as</b> u128)
        * (duration_ms <b>as</b> u128);
    <b>let</b> denominator = (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_DAY_MS">DAY_MS</a> <b>as</b> u128);
    <b>let</b> premium_u128 = numerator / denominator;
    <b>assert</b>!(premium_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    premium_u128 <b>as</b> u64
}
</code></pre>



</details>

<a name="social_contracts_insurance_buy_coverage"></a>

## Function `buy_coverage`

Buy coverage for a SPoT position


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage">buy_coverage</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, option_id: u8, requested_coverage_amount: u64, coverage_bps: u64, duration_ms: u64, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage">buy_coverage</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    spot_config: &spot::SpotConfig,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    option_id: u8,
    requested_coverage_amount: u64,
    coverage_bps: u64,
    duration_ms: u64,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!config.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPaused">EPaused</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(spot::is_open(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(coverage_bps &gt;= config.min_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(coverage_bps &lt;= config.max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(duration_ms &gt; 0 && duration_ms &lt;= config.max_duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(requested_coverage_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> insured = tx_context::sender(ctx);
    <b>let</b> market_id = spot::get_id_address(record);
    <b>let</b> position_amount = spot::get_user_option_amount(record, insured, option_id);
    <b>let</b> covered_amount = <b>if</b> (requested_coverage_amount &lt;= position_amount) {
        requested_coverage_amount
    } <b>else</b> {
        position_amount
    };
    <b>assert</b>!(covered_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(covered_amount, coverage_bps);
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>assert</b>!(capital_value &gt;= vault.reserved, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> free_capital = capital_value - vault.reserved;
    <b>assert</b>!(free_capital &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    <b>assert</b>!(vault.reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_enforce_exposure_limits">enforce_exposure_limits</a>(vault, market_id, insured, option_id, reserve_amount, ctx);
    <b>let</b> premium = <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium">quote_premium</a>(vault, covered_amount, coverage_bps, duration_ms);
    <b>assert</b>!(premium &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientPremium">EInsufficientPremium</a>);
    <b>assert</b>!(coin::value(&payment) &gt;= premium, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientPremium">EInsufficientPremium</a>);
    <b>let</b> premium_coin = coin::split(&<b>mut</b> payment, premium, ctx);
    balance::join(&<b>mut</b> vault.capital, coin::into_balance(premium_coin));
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, insured);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    vault.reserved = vault.reserved + reserve_amount;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_add_exposure">add_exposure</a>(vault, market_id, insured, option_id, reserve_amount, ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_ms = now + duration_ms;
    <b>let</b> policy = <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a> {
        id: object::new(ctx),
        market_id,
        insured,
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid: premium,
        start_time_ms: now,
        expiry_time_ms,
        vault_id: object::id(vault),
        status: <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>,
    };
    <b>let</b> policy_id = object::id(&policy);
    transfer::public_transfer(policy, insured);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePurchasedEvent">CoveragePurchasedEvent</a> {
        policy_id,
        market_id,
        insured,
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid: premium,
        reserve_locked: reserve_amount,
        expiry_time_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_cancel_coverage"></a>

## Function `cancel_coverage`

Cancel coverage while the market is open
Cancellation can result in 0 refund due to fee + rounding


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_cancel_coverage">cancel_coverage</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">social_contracts::insurance::CoveragePolicy</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_cancel_coverage">cancel_coverage</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    spot_config: &spot::SpotConfig,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!config.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPaused">EPaused</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(spot::is_open(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(policy.status == <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyNotActive">EPolicyNotActive</a>);
    <b>assert</b>!(tx_context::sender(ctx) == policy.insured, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotPolicyOwner">ENotPolicyOwner</a>);
    <b>assert</b>!(policy.market_id == spot::get_id_address(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketMismatch">EMarketMismatch</a>);
    <b>assert</b>!(policy.vault_id == object::id(vault), <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidVault">EInvalidVault</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt; policy.expiry_time_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyExpired">EPolicyExpired</a>);
    <b>let</b> total_duration = policy.expiry_time_ms - policy.start_time_ms;
    <b>let</b> remaining = policy.expiry_time_ms - now;
    <b>let</b> refund_u128 = (policy.premium_paid <b>as</b> u128) * (remaining <b>as</b> u128) / (total_duration <b>as</b> u128);
    <b>assert</b>!(refund_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> original_refund = refund_u128 <b>as</b> u64;
    <b>let</b> fee = (original_refund * config.fee_bps) / <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> net_refund = original_refund - fee;
    // original_refund == fee + net_refund; ensure vault can fund both splits
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>assert</b>!(capital_value &gt;= original_refund, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    <b>if</b> (fee &gt; 0) {
        <b>let</b> fee_balance = balance::split(&<b>mut</b> vault.capital, fee);
        <b>let</b> fee_coin = coin::from_balance(fee_balance, ctx);
        transfer::public_transfer(fee_coin, config.treasury);
    };
    <b>if</b> (net_refund &gt; 0) {
        <b>let</b> refund_balance = balance::split(&<b>mut</b> vault.capital, net_refund);
        <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, policy.insured);
    };
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(policy.covered_amount, policy.coverage_bps);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
    <b>assert</b>!(vault.reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    vault.reserved = vault.reserved - reserve_amount;
    policy.status = <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CANCELLED">STATUS_CANCELLED</a>;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageCancelledEvent">CoverageCancelledEvent</a> {
        policy_id: object::id(policy),
        insured: policy.insured,
        refunded_amount: net_refund,
        fee_paid: fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_claim"></a>

## Function `claim`

Claim payout after SPoT resolution
Payout is calculated as min(current_position, covered_amount) * coverage_bps / BPS_DENOM
Dynamic coverage: payout adjusts if user reduces their SPoT position after buying insurance.
This prevents exploitation where user buys insurance then exits bet.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_claim">claim</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">social_contracts::insurance::CoveragePolicy</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_claim">claim</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    spot_config: &spot::SpotConfig,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!config.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPaused">EPaused</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(policy.status == <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyNotActive">EPolicyNotActive</a>);
    <b>assert</b>!(tx_context::sender(ctx) == policy.insured, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotPolicyOwner">ENotPolicyOwner</a>);
    <b>assert</b>!(policy.market_id == spot::get_id_address(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketMismatch">EMarketMismatch</a>);
    <b>assert</b>!(policy.vault_id == object::id(vault), <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidVault">EInvalidVault</a>);
    <b>assert</b>!(spot::is_resolved(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= policy.expiry_time_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyExpired">EPolicyExpired</a>);
    <b>let</b> outcome_opt = spot::get_outcome(record);
    <b>assert</b>!(option::is_some(outcome_opt), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>let</b> outcome = *option::borrow(outcome_opt);
    <b>let</b> <b>mut</b> payout = 0;
    <b>if</b> (outcome != spot::outcome_draw() && outcome != spot::outcome_unapplicable()) {
        <b>if</b> (outcome != policy.option_id) {
            // Dynamic coverage: payout adjusts <b>if</b> user reduces their SPoT position after buying <a href="../social_contracts/insurance.md#social_contracts_insurance">insurance</a>
            <b>let</b> current_position = spot::get_user_option_amount(record, policy.insured, policy.option_id);
            <b>let</b> eligible_amount = <b>if</b> (current_position &lt; policy.covered_amount) {
                current_position
            } <b>else</b> {
                policy.covered_amount
            };
            <b>let</b> payout_u128 = (eligible_amount <b>as</b> u128) * (policy.coverage_bps <b>as</b> u128) / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
            <b>assert</b>!(payout_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
            payout = payout_u128 <b>as</b> u64;
        };
    };
    <b>if</b> (payout &gt; 0) {
        <b>let</b> payout_balance = balance::split(&<b>mut</b> vault.capital, payout);
        <b>let</b> payout_coin = coin::from_balance(payout_balance, ctx);
        transfer::public_transfer(payout_coin, policy.insured);
    };
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(policy.covered_amount, policy.coverage_bps);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
    <b>assert</b>!(vault.reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    vault.reserved = vault.reserved - reserve_amount;
    policy.status = <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CLAIMED">STATUS_CLAIMED</a>;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageClaimedEvent">CoverageClaimedEvent</a> {
        policy_id: object::id(policy),
        insured: policy.insured,
        payout,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_expire_policy"></a>

## Function `expire_policy`

Expire policy and release reserves


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_expire_policy">expire_policy</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">social_contracts::insurance::CoveragePolicy</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_expire_policy">expire_policy</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a>,
    clock: &Clock
) {
    <b>if</b> (policy.status != <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>) {
        <b>return</b>
    };
    <b>if</b> (policy.vault_id != object::id(vault)) {
        <b>return</b>
    };
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (now &lt; policy.expiry_time_ms) {
        <b>return</b>
    };
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(policy.covered_amount, policy.coverage_bps);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
    <b>assert</b>!(vault.reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    vault.reserved = vault.reserved - reserve_amount;
    policy.status = <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_EXPIRED">STATUS_EXPIRED</a>;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_PolicyExpiredEvent">PolicyExpiredEvent</a> {
        policy_id: object::id(policy),
        insured: policy.insured,
        market_id: policy.market_id,
        vault_id: policy.vault_id,
        reserve_released: reserve_amount,
        expiry_time_ms: policy.expiry_time_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_compute_reserve"></a>

## Function `compute_reserve`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(covered_amount: u64, coverage_bps: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(covered_amount: u64, coverage_bps: u64): u64 {
    <b>let</b> reserve_u128 = (covered_amount <b>as</b> u128) * (coverage_bps <b>as</b> u128);
    <b>let</b> reserve_u128 = reserve_u128 / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
    <b>assert</b>!(reserve_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    reserve_u128 <b>as</b> u64
}
</code></pre>



</details>

<a name="social_contracts_insurance_enforce_exposure_limits"></a>

## Function `enforce_exposure_limits`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_enforce_exposure_limits">enforce_exposure_limits</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, reserve_amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_enforce_exposure_limits">enforce_exposure_limits</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    reserve_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Read limit values before creating mutable borrows
    <b>let</b> max_exposure_per_market = vault.max_exposure_per_market;
    <b>let</b> max_exposure_per_user = vault.max_exposure_per_user;
    // Check user exposure limit first (doesn't require market exposure)
    <b>if</b> (max_exposure_per_user &gt; 0) {
        <b>let</b> current_user = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault, insured);
        <b>assert</b>!(current_user &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        <b>let</b> new_user = current_user + reserve_amount;
        <b>assert</b>!(new_user &lt;= max_exposure_per_user, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    // Now get mutable reference to market exposure <b>for</b> market and option checks
    <b>let</b> exposure = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(vault, market_id, ctx);
    <b>if</b> (max_exposure_per_market &gt; 0) {
        <b>assert</b>!(exposure.total_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        <b>let</b> new_total = exposure.total_reserved + reserve_amount;
        <b>assert</b>!(new_total &lt;= max_exposure_per_market, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    <b>let</b> option_reserved = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure, option_id);
    <b>assert</b>!(option_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
}
</code></pre>



</details>

<a name="social_contracts_insurance_add_exposure"></a>

## Function `add_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_add_exposure">add_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, reserve_amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_add_exposure">add_exposure</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    reserve_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> exposure = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(vault, market_id, ctx);
    <b>assert</b>!(exposure.total_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    exposure.total_reserved = exposure.total_reserved + reserve_amount;
    <b>let</b> option_reserved = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure, option_id);
    <b>assert</b>!(option_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> new_option_reserved = option_reserved + reserve_amount;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_set_option_reserved">set_option_reserved</a>(exposure, option_id, new_option_reserved);
    <b>let</b> current_user = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault, insured);
    <b>assert</b>!(current_user &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> new_user = current_user + reserve_amount;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_set_user_exposure">set_user_exposure</a>(vault, insured, new_user);
}
</code></pre>



</details>

<a name="social_contracts_insurance_release_exposure"></a>

## Function `release_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, reserve_amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    reserve_amount: u64
) {
    <b>if</b> (reserve_amount == 0) {
        <b>return</b>
    };
    <b>assert</b>!(table::contains(&vault.market_exposures, market_id), <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> exposure = table::borrow_mut(&<b>mut</b> vault.market_exposures, market_id);
    <b>assert</b>!(exposure.total_reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    exposure.total_reserved = exposure.total_reserved - reserve_amount;
    <b>assert</b>!(table::contains(&exposure.reserved_by_option, option_id), <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> current_option = *table::borrow(&exposure.reserved_by_option, option_id);
    <b>assert</b>!(current_option &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> option_ref = table::borrow_mut(&<b>mut</b> exposure.reserved_by_option, option_id);
    *option_ref = current_option - reserve_amount;
    <b>assert</b>!(table::contains(&vault.user_exposures, insured), <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> current_user = *table::borrow(&vault.user_exposures, insured);
    <b>assert</b>!(current_user &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> user_ref = table::borrow_mut(&<b>mut</b> vault.user_exposures, insured);
    *user_ref = current_user - reserve_amount;
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_market_exposure_mut"></a>

## Function `get_market_exposure_mut`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
): &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a> {
    <b>if</b> (!table::contains(&vault.market_exposures, market_id)) {
        <b>let</b> exposure = <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a> {
            market_id,
            total_reserved: 0,
            reserved_by_option: table::new(ctx),
        };
        table::add(&<b>mut</b> vault.market_exposures, market_id, exposure);
    };
    table::borrow_mut(&<b>mut</b> vault.market_exposures, market_id)
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_user_exposure"></a>

## Function `get_user_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, insured: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>, insured: <b>address</b>): u64 {
    <b>if</b> (table::contains(&vault.user_exposures, insured)) {
        *table::borrow(&vault.user_exposures, insured)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_user_exposure"></a>

## Function `set_user_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_user_exposure">set_user_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, insured: <b>address</b>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_user_exposure">set_user_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>, insured: <b>address</b>, amount: u64) {
    <b>if</b> (table::contains(&vault.user_exposures, insured)) {
        <b>let</b> user_ref = table::borrow_mut(&<b>mut</b> vault.user_exposures, insured);
        *user_ref = amount;
    } <b>else</b> {
        table::add(&<b>mut</b> vault.user_exposures, insured, amount);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_option_reserved"></a>

## Function `get_option_reserved`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure: &<a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure: &<a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a>, option_id: u8): u64 {
    <b>if</b> (table::contains(&exposure.reserved_by_option, option_id)) {
        *table::borrow(&exposure.reserved_by_option, option_id)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_option_reserved"></a>

## Function `set_option_reserved`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_option_reserved">set_option_reserved</a>(exposure: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>, option_id: u8, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_option_reserved">set_option_reserved</a>(exposure: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a>, option_id: u8, amount: u64) {
    <b>if</b> (table::contains(&exposure.reserved_by_option, option_id)) {
        <b>let</b> option_ref = table::borrow_mut(&<b>mut</b> exposure.reserved_by_option, option_id);
        *option_ref = amount;
    } <b>else</b> {
        table::add(&<b>mut</b> exposure.reserved_by_option, option_id, amount);
    };
}
</code></pre>



</details>
