---
title: Module `social_contracts::social_proof_of_truth`
---

Social Proof of Truth (SPoT)
Prediction market for post truthfulness. Users bet YES/NO on whether a post is true.
All bets go directly to escrow. Oracle/DAO resolves the outcome, and winners receive
pro-rata payouts from the total escrow pool.


-  [Struct `SpotAdminCap`](#social_contracts_social_proof_of_truth_SpotAdminCap)
-  [Struct `SpotConfig`](#social_contracts_social_proof_of_truth_SpotConfig)
-  [Struct `SpotBet`](#social_contracts_social_proof_of_truth_SpotBet)
-  [Struct `SpotRecord`](#social_contracts_social_proof_of_truth_SpotRecord)
-  [Struct `SpotBetPlacedEvent`](#social_contracts_social_proof_of_truth_SpotBetPlacedEvent)
-  [Struct `SpotResolvedEvent`](#social_contracts_social_proof_of_truth_SpotResolvedEvent)
-  [Struct `SpotDaoRequiredEvent`](#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent)
-  [Struct `SpotPayoutEvent`](#social_contracts_social_proof_of_truth_SpotPayoutEvent)
-  [Struct `SpotRefundEvent`](#social_contracts_social_proof_of_truth_SpotRefundEvent)
-  [Constants](#@Constants_0)
-  [Function `get_status`](#social_contracts_social_proof_of_truth_get_status)
-  [Function `get_total_yes_escrow`](#social_contracts_social_proof_of_truth_get_total_yes_escrow)
-  [Function `get_total_no_escrow`](#social_contracts_social_proof_of_truth_get_total_no_escrow)
-  [Function `get_bets_len`](#social_contracts_social_proof_of_truth_get_bets_len)
-  [Function `bootstrap_init`](#social_contracts_social_proof_of_truth_bootstrap_init)
-  [Function `update_spot_config`](#social_contracts_social_proof_of_truth_update_spot_config)
-  [Function `create_spot_record_for_post`](#social_contracts_social_proof_of_truth_create_spot_record_for_post)
-  [Function `place_spot_bet`](#social_contracts_social_proof_of_truth_place_spot_bet)
-  [Function `oracle_resolve`](#social_contracts_social_proof_of_truth_oracle_resolve)
-  [Function `finalize_via_dao`](#social_contracts_social_proof_of_truth_finalize_via_dao)
-  [Function `refund_unresolved`](#social_contracts_social_proof_of_truth_refund_unresolved)
-  [Function `finalize_resolution_and_payout`](#social_contracts_social_proof_of_truth_finalize_resolution_and_payout)


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



<a name="social_contracts_social_proof_of_truth_SpotAdminCap"></a>

## Struct `SpotAdminCap`

Admin capability for SPoT


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_social_proof_of_truth_SpotConfig"></a>

## Struct `SpotConfig`

Global configuration for SPoT


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a> <b>has</b> key
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
<code>enable_flag: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>confidence_threshold_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_window_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payout_delay_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_split_bps_platform: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_treasury: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>chain_treasury: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>max_single_bet: u64</code>
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

<a name="social_contracts_social_proof_of_truth_SpotBet"></a>

## Struct `SpotBet`

A single bet


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_yes: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
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

<a name="social_contracts_social_proof_of_truth_SpotRecord"></a>

## Struct `SpotRecord`

SPoT record per post


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a> <b>has</b> key, store
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
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>escrow: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_yes_escrow: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_no_escrow: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>bets: vector&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">social_contracts::social_proof_of_truth::SpotBet</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>last_resolution_epoch: u64</code>
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

<a name="social_contracts_social_proof_of_truth_SpotBetPlacedEvent"></a>

## Struct `SpotBetPlacedEvent`

Events


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetPlacedEvent">SpotBetPlacedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_yes: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotResolvedEvent"></a>

## Struct `SpotResolvedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>total_escrow: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_taken: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotDaoRequiredEvent"></a>

## Struct `SpotDaoRequiredEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent">SpotDaoRequiredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>confidence_bps: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotPayoutEvent"></a>

## Struct `SpotPayoutEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPayoutEvent">SpotPayoutEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotRefundEvent"></a>

## Struct `SpotRefundEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRefundEvent">SpotRefundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_social_proof_of_truth_DEFAULT_CONFIDENCE_THRESHOLD_BPS"></a>

Config defaults


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CONFIDENCE_THRESHOLD_BPS">DEFAULT_CONFIDENCE_THRESHOLD_BPS</a>: u64 = 7000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_ENABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_ENABLE">DEFAULT_ENABLE</a>: bool = <b>true</b>;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>: u64 = 100;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_FEE_SPLIT_PLATFORM_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_FEE_SPLIT_PLATFORM_BPS">DEFAULT_FEE_SPLIT_PLATFORM_BPS</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS">DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS</a>: u64 = 144;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_PAYOUT_DELAY_EPOCHS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_PAYOUT_DELAY_EPOCHS">DEFAULT_PAYOUT_DELAY_EPOCHS</a>: u64 = 0;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_RESOLUTION_WINDOW_EPOCHS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_RESOLUTION_WINDOW_EPOCHS">DEFAULT_RESOLUTION_WINDOW_EPOCHS</a>: u64 = 72;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EAlreadyResolved"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyResolved">EAlreadyResolved</a>: u64 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EDisabled"></a>

Errors


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENoBets"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>: u64 = 8;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENotOracle"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotOracle">ENotOracle</a>: u64 = 7;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooClose"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooClose">ETooClose</a>: u64 = 5;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooEarly"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>: u64 = 4;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWrongStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>: u64 = 6;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_DRAW"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a>: u8 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_NO"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_NO">OUTCOME_NO</a>: u8 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>: u8 = 4;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_YES"></a>

Outcomes


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_YES">OUTCOME_YES</a>: u8 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_OPEN"></a>

Status


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>: u8 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_REFUNDABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_REFUNDABLE">STATUS_REFUNDABLE</a>: u8 = 4;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_RESOLVED"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_get_status"></a>

## Function `get_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_status">get_status</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_status">get_status</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): u8 { rec.status }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_total_yes_escrow"></a>

## Function `get_total_yes_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_total_yes_escrow">get_total_yes_escrow</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_total_yes_escrow">get_total_yes_escrow</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): u64 { rec.total_yes_escrow }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_total_no_escrow"></a>

## Function `get_total_no_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_total_no_escrow">get_total_no_escrow</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_total_no_escrow">get_total_no_escrow</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): u64 { rec.total_no_escrow }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_bets_len"></a>

## Function `get_bets_len`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_bets_len">get_bets_len</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_bets_len">get_bets_len</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): u64 { vector::length(&rec.bets) }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    transfer::share_object(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a> {
        id: object::new(ctx),
        enable_flag: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_ENABLE">DEFAULT_ENABLE</a>,
        confidence_threshold_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CONFIDENCE_THRESHOLD_BPS">DEFAULT_CONFIDENCE_THRESHOLD_BPS</a>,
        resolution_window_epochs: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_RESOLUTION_WINDOW_EPOCHS">DEFAULT_RESOLUTION_WINDOW_EPOCHS</a>,
        max_resolution_window_epochs: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS">DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS</a>,
        payout_delay_epochs: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_PAYOUT_DELAY_EPOCHS">DEFAULT_PAYOUT_DELAY_EPOCHS</a>,
        fee_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>,
        fee_split_bps_platform: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_FEE_SPLIT_PLATFORM_BPS">DEFAULT_FEE_SPLIT_PLATFORM_BPS</a>,
        platform_treasury: admin,
        chain_treasury: admin,
        oracle_address: admin,
        max_single_bet: 0,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    });
    transfer::public_transfer(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> { id: object::new(ctx) }, admin);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_update_spot_config"></a>

## Function `update_spot_config`

Update SPoT configuration (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_update_spot_config">update_spot_config</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">social_contracts::social_proof_of_truth::SpotAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, enable_flag: bool, confidence_threshold_bps: u64, resolution_window_epochs: u64, max_resolution_window_epochs: u64, payout_delay_epochs: u64, fee_bps: u64, fee_split_bps_platform: u64, platform_treasury: <b>address</b>, chain_treasury: <b>address</b>, oracle_address: <b>address</b>, max_single_bet: u64, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_update_spot_config">update_spot_config</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    enable_flag: bool,
    confidence_threshold_bps: u64,
    resolution_window_epochs: u64,
    max_resolution_window_epochs: u64,
    payout_delay_epochs: u64,
    fee_bps: u64,
    fee_split_bps_platform: u64,
    platform_treasury: <b>address</b>,
    chain_treasury: <b>address</b>,
    oracle_address: <b>address</b>,
    max_single_bet: u64,
    _ctx: &<b>mut</b> TxContext
) {
    // Basic bounds
    <b>assert</b>!(confidence_threshold_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    // windows may be zero in tests to resolve immediately
    config.enable_flag = enable_flag;
    config.confidence_threshold_bps = confidence_threshold_bps;
    config.resolution_window_epochs = resolution_window_epochs;
    config.max_resolution_window_epochs = max_resolution_window_epochs;
    config.payout_delay_epochs = payout_delay_epochs;
    config.fee_bps = fee_bps;
    config.fee_split_bps_platform = fee_split_bps_platform;
    config.platform_treasury = platform_treasury;
    config.chain_treasury = chain_treasury;
    config.oracle_address = oracle_address;
    config.max_single_bet = max_single_bet;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_record_for_post"></a>

## Function `create_spot_record_for_post`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_record_for_post">create_spot_record_for_post</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_record_for_post">create_spot_record_for_post</a>(
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>let</b> record = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a> {
        id: object::new(ctx),
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        created_epoch: tx_context::epoch(ctx),
        status: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>,
        outcome: option::none(),
        escrow: balance::zero(),
        total_yes_escrow: 0,
        total_no_escrow: 0,
        bets: vector::empty&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a>&gt;(),
        last_resolution_epoch: 0,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    transfer::share_object(record);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_place_spot_bet"></a>

## Function `place_spot_bet`

Place bet - all funds go to escrow


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet">place_spot_bet</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, is_yes: bool, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet">place_spot_bet</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    is_yes: bool,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(spot_config.enable_flag, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (spot_config.max_single_bet &gt; 0) { <b>assert</b>!(amount &lt;= spot_config.max_single_bet, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>); };
    <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    // All funds go to escrow
    <b>let</b> bet_coin = coin::split(&<b>mut</b> payment, amount, ctx);
    balance::join(&<b>mut</b> record.escrow, coin::into_balance(bet_coin));
    <b>if</b> (is_yes) {
        record.total_yes_escrow = record.total_yes_escrow + amount;
    } <b>else</b> {
        record.total_no_escrow = record.total_no_escrow + amount;
    };
    // Refund any excess
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, tx_context::sender(ctx));
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Record bet
    vector::push_back(&<b>mut</b> record.bets, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a> {
        user: tx_context::sender(ctx),
        is_yes,
        amount,
        timestamp: tx_context::epoch(ctx),
    });
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetPlacedEvent">SpotBetPlacedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        user: tx_context::sender(ctx),
        is_yes,
        amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_oracle_resolve"></a>

## Function `oracle_resolve`

Oracle resolution (YES/NO, or too close → DAO_REQUIRED)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_resolve">oracle_resolve</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, outcome_yes: bool, confidence_bps: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_resolve">oracle_resolve</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    outcome_yes: bool,
    confidence_bps: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == spot_config.oracle_address, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotOracle">ENotOracle</a>);
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    // Enforce resolution window
    <b>let</b> now = tx_context::epoch(ctx);
    <b>assert</b>!(now &gt;= record.created_epoch + spot_config.resolution_window_epochs, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <b>if</b> (confidence_bps &lt; spot_config.confidence_threshold_bps) {
        record.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>;
        event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent">SpotDaoRequiredEvent</a> { post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), confidence_bps });
        <b>return</b>
    };
    // Resolve outcome
    <b>let</b> outcome = <b>if</b> (outcome_yes) { <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_YES">OUTCOME_YES</a> } <b>else</b> { <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_NO">OUTCOME_NO</a> };
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config, record, <a href="../social_contracts/post.md#social_contracts_post">post</a>, outcome, ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_via_dao"></a>

## Function `finalize_via_dao`

DAO finalization (YES/NO/DRAW/UNAPPLICABLE)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_via_dao">finalize_via_dao</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, outcome: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_via_dao">finalize_via_dao</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    outcome: u8,
    ctx: &<b>mut</b> TxContext
) {
    // Allow when DAO_REQUIRED or still OPEN (off-chain DAO direct)
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a> || record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config, record, <a href="../social_contracts/post.md#social_contracts_post">post</a>, outcome, ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_refund_unresolved"></a>

## Function `refund_unresolved`

Refund all escrow if unresolved beyond max window


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_refund_unresolved">refund_unresolved</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_refund_unresolved">refund_unresolved</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> now = tx_context::epoch(ctx);
    <b>assert</b>!(now &gt;= record.created_epoch + spot_config.max_resolution_window_epochs, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> || record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(vector::length(&record.bets) &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>);
    // Iterate all bets and refund escrow
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&record.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&record.bets, i);
        <b>if</b> (bet.amount &gt; 0) {
            <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> record.escrow, bet.amount), ctx);
            transfer::public_transfer(c, bet.user);
            event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRefundEvent">SpotRefundEvent</a> { post_id: record.post_id, user: bet.user, amount: bet.amount });
        };
        i = i + 1;
    };
    record.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_REFUNDABLE">STATUS_REFUNDABLE</a>;
    record.outcome = option::none();
    record.last_resolution_epoch = now;
    // Any dust stays in escrow balance <b>if</b> math rounding occurred
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_resolution_and_payout"></a>

## Function `finalize_resolution_and_payout`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, outcome: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    outcome: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> || record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(vector::length(&record.bets) &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>);
    // Winner side total
    <b>let</b> total_yes = record.total_yes_escrow;
    <b>let</b> total_no = record.total_no_escrow;
    <b>let</b> total_escrow = total_yes + total_no;
    // Handle DRAW/UNAPPLICABLE: refund all escrow
    <b>if</b> (outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a> || outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>) {
        <b>let</b> <b>mut</b> i = 0; <b>let</b> len = vector::length(&record.bets);
        <b>while</b> (i &lt; len) {
            <b>let</b> bet = vector::borrow(&record.bets, i);
            <b>if</b> (bet.amount &gt; 0) {
                <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> record.escrow, bet.amount), ctx);
                transfer::public_transfer(c, bet.user);
                event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRefundEvent">SpotRefundEvent</a> { post_id: record.post_id, user: bet.user, amount: bet.amount });
            };
            i = i + 1;
        };
        record.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>;
        record.outcome = option::some(outcome);
        record.last_resolution_epoch = tx_context::epoch(ctx);
        event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> { post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), outcome, total_escrow, fee_taken: 0 });
        <b>return</b>
    };
    <b>let</b> (winning_total, is_yes_winning) = <b>if</b> (outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_YES">OUTCOME_YES</a>) { (total_yes, <b>true</b>) } <b>else</b> { (total_no, <b>false</b>) };
    // Fees on payouts (apply to total escrow)
    <b>let</b> <b>mut</b> fee = 0;
    <b>if</b> (spot_config.fee_bps &gt; 0) { fee = (total_escrow * spot_config.fee_bps) / 10000; };
    <b>let</b> distributable = total_escrow - fee;
    // Split fee 50/50 (configurable)
    <b>if</b> (fee &gt; 0) {
        <b>let</b> platform_part = (fee * spot_config.fee_split_bps_platform) / 10000;
        <b>let</b> chain_part = fee - platform_part;
        <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> record.escrow, fee), ctx);
        <b>let</b> platform_coin = coin::split(&<b>mut</b> fee_coin, platform_part, ctx);
        transfer::public_transfer(platform_coin, spot_config.platform_treasury);
        transfer::public_transfer(fee_coin, spot_config.chain_treasury);
    };
    // Distribute to winners pro-rata of total escrow
    <b>let</b> <b>mut</b> i = 0; <b>let</b> len = vector::length(&record.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&record.bets, i);
        <b>let</b> winner = (bet.is_yes && is_yes_winning) || (!bet.is_yes && !is_yes_winning);
        <b>if</b> (winner && winning_total &gt; 0 && bet.amount &gt; 0) {
            <b>let</b> payout = (((bet.amount <b>as</b> u128) * (distributable <b>as</b> u128)) / (winning_total <b>as</b> u128)) <b>as</b> u64;
            <b>if</b> (payout &gt; 0) {
                <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> record.escrow, payout), ctx);
                transfer::public_transfer(c, bet.user);
                event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPayoutEvent">SpotPayoutEvent</a> { post_id: record.post_id, user: bet.user, amount: payout });
            };
        };
        i = i + 1;
    };
    record.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>;
    record.outcome = option::some(outcome);
    record.last_resolution_epoch = tx_context::epoch(ctx);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> { post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), outcome, total_escrow, fee_taken: fee });
}
</code></pre>



</details>
