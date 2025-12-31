---
title: Module `social_contracts::social_proof_of_truth`
---

Social Proof of Truth (SPoT)
Prediction market for post truthfulness. Users bet on custom options (2-10 options per record).
All bets go directly to escrow. Oracle/DAO resolves the outcome, and winners receive
pro-rata payouts from the total escrow pool. Users can withdraw bets before resolution
with the same fee structure as payouts. Time-based resolution windows are optional per record.


-  [Struct `SpotAdminCap`](#social_contracts_social_proof_of_truth_SpotAdminCap)
-  [Struct `SpotOracleAdminCap`](#social_contracts_social_proof_of_truth_SpotOracleAdminCap)
-  [Struct `SpotConfig`](#social_contracts_social_proof_of_truth_SpotConfig)
-  [Struct `SpotBet`](#social_contracts_social_proof_of_truth_SpotBet)
-  [Struct `SpotRecord`](#social_contracts_social_proof_of_truth_SpotRecord)
-  [Struct `SpotBetPlacedEvent`](#social_contracts_social_proof_of_truth_SpotBetPlacedEvent)
-  [Struct `SpotResolvedEvent`](#social_contracts_social_proof_of_truth_SpotResolvedEvent)
-  [Struct `SpotDaoRequiredEvent`](#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent)
-  [Struct `SpotPayoutEvent`](#social_contracts_social_proof_of_truth_SpotPayoutEvent)
-  [Struct `SpotRefundEvent`](#social_contracts_social_proof_of_truth_SpotRefundEvent)
-  [Struct `SpotConfigUpdatedEvent`](#social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent)
-  [Struct `SpotBetWithdrawnEvent`](#social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent)
-  [Struct `SpotRecordCreatedEvent`](#social_contracts_social_proof_of_truth_SpotRecordCreatedEvent)
-  [Constants](#@Constants_0)
-  [Function `get_status`](#social_contracts_social_proof_of_truth_get_status)
-  [Function `get_bets_len`](#social_contracts_social_proof_of_truth_get_bets_len)
-  [Function `get_betting_options`](#social_contracts_social_proof_of_truth_get_betting_options)
-  [Function `get_option_escrow`](#social_contracts_social_proof_of_truth_get_option_escrow)
-  [Function `get_id_address`](#social_contracts_social_proof_of_truth_get_id_address)
-  [Function `get_outcome`](#social_contracts_social_proof_of_truth_get_outcome)
-  [Function `is_open`](#social_contracts_social_proof_of_truth_is_open)
-  [Function `is_resolved`](#social_contracts_social_proof_of_truth_is_resolved)
-  [Function `outcome_draw`](#social_contracts_social_proof_of_truth_outcome_draw)
-  [Function `outcome_unapplicable`](#social_contracts_social_proof_of_truth_outcome_unapplicable)
-  [Function `get_user_option_amount`](#social_contracts_social_proof_of_truth_get_user_option_amount)
-  [Function `is_enabled`](#social_contracts_social_proof_of_truth_is_enabled)
-  [Function `bootstrap_init`](#social_contracts_social_proof_of_truth_bootstrap_init)
-  [Function `create_spot_admin_cap`](#social_contracts_social_proof_of_truth_create_spot_admin_cap)
-  [Function `create_spot_oracle_admin_cap`](#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap)
-  [Function `update_spot_config`](#social_contracts_social_proof_of_truth_update_spot_config)
-  [Function `create_spot_record_for_post`](#social_contracts_social_proof_of_truth_create_spot_record_for_post)
-  [Function `withdraw_spot_bet`](#social_contracts_social_proof_of_truth_withdraw_spot_bet)
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
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
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

Admin capability for SPoT (controls SpotConfig updates)


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

<a name="social_contracts_social_proof_of_truth_SpotOracleAdminCap"></a>

## Struct `SpotOracleAdminCap`

Oracle admin capability for SPoT (controls oracle decisions: record creation and resolution)


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a> <b>has</b> key, store
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
<code>option_id: u8</code>
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
<code>betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>option_escrow: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u8, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>user_option_amounts: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, vector&lt;u64&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>bets: vector&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">social_contracts::social_proof_of_truth::SpotBet</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_window_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
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
<code>option_id: u8</code>
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
<dt>
<code>reasoning: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_urls: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
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
<dt>
<code>reasoning: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent"></a>

## Struct `SpotConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent">SpotConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent"></a>

## Struct `SpotBetWithdrawnEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent">SpotBetWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
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

<a name="social_contracts_social_proof_of_truth_SpotRecordCreatedEvent"></a>

## Struct `SpotRecordCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecordCreatedEvent">SpotRecordCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>record_id: <b>address</b></code>
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
<code>betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_window_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
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



<a name="social_contracts_social_proof_of_truth_EAlreadyInitialized"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyInitialized">EAlreadyInitialized</a>: u64 = 14;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EAlreadyResolved"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyResolved">EAlreadyResolved</a>: u64 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EBetNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EBetNotFound">EBetNotFound</a>: u64 = 13;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EDisabled"></a>

Errors


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EDuplicateOption"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDuplicateOption">EDuplicateOption</a>: u64 = 15;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidOptionId"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidOptionId">EInvalidOptionId</a>: u64 = 11;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidReasoning"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENoBets"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>: u64 = 8;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENotOracle"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotOracle">ENotOracle</a>: u64 = 7;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>: u64 = 9;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooClose"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooClose">ETooClose</a>: u64 = 5;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooEarly"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>: u64 = 4;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWithdrawalNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWithdrawalNotAllowed">EWithdrawalNotAllowed</a>: u64 = 12;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWrongStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>: u64 = 6;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MAX_BETTING_OPTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_BETTING_OPTIONS">MAX_BETTING_OPTIONS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MAX_EVIDENCE_URLS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_EVIDENCE_URLS">MAX_EVIDENCE_URLS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MAX_REASONING_LENGTH"></a>

Validation constants


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MAX_U64"></a>

Maximum u64 value for overflow protection


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MIN_BETTING_OPTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_BETTING_OPTIONS">MIN_BETTING_OPTIONS</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MIN_REASONING_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_REASONING_LENGTH">MIN_REASONING_LENGTH</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_DRAW"></a>

Outcomes
Note: For multi-option betting, outcome is the winning option_id (0-indexed)
Special outcomes: DRAW = 255, UNAPPLICABLE = 254


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a>: u8 = 255;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>: u8 = 254;
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

<a name="social_contracts_social_proof_of_truth_get_bets_len"></a>

## Function `get_bets_len`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_bets_len">get_bets_len</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_bets_len">get_bets_len</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): u64 { vector::length(&rec.bets) }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_betting_options"></a>

## Function `get_betting_options`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_betting_options">get_betting_options</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_betting_options">get_betting_options</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): vector&lt;String&gt; { rec.betting_options }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_option_escrow"></a>

## Function `get_option_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_option_escrow">get_option_escrow</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_option_escrow">get_option_escrow</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>, option_id: u8): u64 {
    <b>if</b> (table::contains(&rec.option_escrow, option_id)) {
        *table::borrow(&rec.option_escrow, option_id)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_id_address"></a>

## Function `get_id_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_id_address">get_id_address</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_id_address">get_id_address</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): <b>address</b> {
    object::uid_to_address(&rec.id)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_outcome"></a>

## Function `get_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_outcome">get_outcome</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_outcome">get_outcome</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): &Option&lt;u8&gt; { &rec.outcome }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_is_open"></a>

## Function `is_open`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_open">is_open</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_open">is_open</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): bool { rec.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_is_resolved"></a>

## Function `is_resolved`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_resolved">is_resolved</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_resolved">is_resolved</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>): bool { rec.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_outcome_draw"></a>

## Function `outcome_draw`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_draw">outcome_draw</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_draw">outcome_draw</a>(): u8 { <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_outcome_unapplicable"></a>

## Function `outcome_unapplicable`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_unapplicable">outcome_unapplicable</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_unapplicable">outcome_unapplicable</a>(): u8 { <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_user_option_amount"></a>

## Function `get_user_option_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_user_option_amount">get_user_option_amount</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, user: <b>address</b>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_user_option_amount">get_user_option_amount</a>(rec: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>, user: <b>address</b>, option_id: u8): u64 {
    <b>if</b> (!table::contains(&rec.user_option_amounts, user)) {
        0
    } <b>else</b> {
        <b>let</b> amounts = table::borrow(&rec.user_option_amounts, user);
        <b>let</b> idx = option_id <b>as</b> u64;
        <b>if</b> (idx &gt;= vector::length(amounts)) {
            0
        } <b>else</b> {
            *vector::borrow(amounts, idx)
        }
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_is_enabled"></a>

## Function `is_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_enabled">is_enabled</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_enabled">is_enabled</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>): bool { config.enable_flag }
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
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_admin_cap"></a>

## Function `create_spot_admin_cap`

Create a SpotAdminCap for bootstrap (package visibility only)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_admin_cap">create_spot_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">social_contracts::social_proof_of_truth::SpotAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_admin_cap">create_spot_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap"></a>

## Function `create_spot_oracle_admin_cap`

Create a SpotOracleAdminCap for bootstrap (package visibility only)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap">create_spot_oracle_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap">create_spot_oracle_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_update_spot_config"></a>

## Function `update_spot_config`

Update SPoT configuration (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_update_spot_config">update_spot_config</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">social_contracts::social_proof_of_truth::SpotAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, enable_flag: bool, confidence_threshold_bps: u64, resolution_window_epochs: u64, max_resolution_window_epochs: u64, payout_delay_epochs: u64, fee_bps: u64, fee_split_bps_platform: u64, platform_treasury: <b>address</b>, chain_treasury: <b>address</b>, oracle_address: <b>address</b>, max_single_bet: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
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
    ctx: &<b>mut</b> TxContext
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
    // Emit config updated event
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent">SpotConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        enable_flag,
        confidence_threshold_bps,
        resolution_window_epochs,
        max_resolution_window_epochs,
        payout_delay_epochs,
        fee_bps,
        fee_split_bps_platform,
        platform_treasury,
        chain_treasury,
        oracle_address,
        max_single_bet,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_record_for_post"></a>

## Function `create_spot_record_for_post`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_record_for_post">create_spot_record_for_post</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, resolution_window_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, max_resolution_window_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_record_for_post">create_spot_record_for_post</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> Post,
    betting_options: vector&lt;String&gt;,
    resolution_window_epochs: Option&lt;u64&gt;,
    max_resolution_window_epochs: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    // Verify SPoT is enabled <b>for</b> this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_is_spot_enabled">social_contracts::post::is_spot_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    // Validate betting options
    <b>let</b> options_len = vector::length(&betting_options);
    <b>assert</b>!(options_len &gt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_BETTING_OPTIONS">MIN_BETTING_OPTIONS</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(options_len &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_BETTING_OPTIONS">MAX_BETTING_OPTIONS</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    // Check <b>for</b> duplicate options (case-sensitive comparison)
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; options_len) {
        <b>let</b> option_i = vector::borrow(&betting_options, i);
        <b>let</b> <b>mut</b> j = i + 1;
        <b>while</b> (j &lt; options_len) {
            <b>let</b> option_j = vector::borrow(&betting_options, j);
            <b>assert</b>!(*option_i != *option_j, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDuplicateOption">EDuplicateOption</a>);
            j = j + 1;
        };
        i = i + 1;
    };
    <b>let</b> record = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a> {
        id: object::new(ctx),
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        created_epoch: tx_context::epoch(ctx),
        status: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>,
        outcome: option::none(),
        escrow: balance::zero(),
        betting_options,
        option_escrow: table::new(ctx),
        user_option_amounts: table::new(ctx),
        bets: vector::empty&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a>&gt;(),
        resolution_window_epochs,
        max_resolution_window_epochs,
        last_resolution_epoch: 0,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> record_id = object::uid_to_address(&record.id);
    <b>let</b> created_epoch = record.created_epoch;
    <b>let</b> post_id = record.post_id;
    <b>let</b> betting_options_copy = record.betting_options;
    <b>let</b> resolution_window = record.resolution_window_epochs;
    <b>let</b> max_resolution_window = record.max_resolution_window_epochs;
    // Store SPoT record ID in <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <a href="../social_contracts/post.md#social_contracts_post_set_spot_id">social_contracts::post::set_spot_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, record_id);
    transfer::share_object(record);
    // Emit record created event
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecordCreatedEvent">SpotRecordCreatedEvent</a> {
        record_id,
        post_id,
        created_epoch,
        betting_options: betting_options_copy,
        resolution_window_epochs: resolution_window,
        max_resolution_window_epochs: max_resolution_window,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_withdraw_spot_bet"></a>

## Function `withdraw_spot_bet`

Withdraw a bet before resolution
Applies same fee structure as payouts
Only allowed when status is OPEN (not DAO_REQUIRED, not RESOLVED, not REFUNDABLE)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_withdraw_spot_bet">withdraw_spot_bet</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, bet_index: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_withdraw_spot_bet">withdraw_spot_bet</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    bet_index: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(spot_config.enable_flag, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    // Only allow withdrawal when status is OPEN (not DAO_REQUIRED or RESOLVED)
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWithdrawalNotAllowed">EWithdrawalNotAllowed</a>);
    <b>let</b> bets_len = vector::length(&record.bets);
    <b>assert</b>!(bet_index &lt; bets_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EBetNotFound">EBetNotFound</a>);
    // Copy bet data before mutating vector
    <b>let</b> bet = *vector::borrow(&record.bets, bet_index);
    <b>assert</b>!(bet.user == tx_context::sender(ctx), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>); // User must own the bet
    <b>assert</b>!(bet.amount &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    // Calculate fee (same <b>as</b> payout fee structure)
    <b>let</b> <b>mut</b> fee = 0;
    <b>if</b> (spot_config.fee_bps &gt; 0) {
        fee = (bet.amount * spot_config.fee_bps) / 10000;
    };
    <b>let</b> refund_amount = bet.amount - fee;
    // Split fee between <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> and chain treasuries
    <b>if</b> (fee &gt; 0) {
        <b>let</b> platform_part = (fee * spot_config.fee_split_bps_platform) / 10000;
        <b>let</b> chain_part = fee - platform_part;
        <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> record.escrow, fee), ctx);
        <b>let</b> platform_coin = coin::split(&<b>mut</b> fee_coin, platform_part, ctx);
        transfer::public_transfer(platform_coin, spot_config.platform_treasury);
        transfer::public_transfer(fee_coin, spot_config.chain_treasury);
    };
    // Refund remaining amount to user
    <b>if</b> (refund_amount &gt; 0) {
        <b>let</b> refund_coin = coin::from_balance(balance::split(&<b>mut</b> record.escrow, refund_amount), ctx);
        transfer::public_transfer(refund_coin, bet.user);
    };
    // Update option escrow table
    <b>let</b> option_id = bet.option_id;
    <b>if</b> (table::contains(&record.option_escrow, option_id)) {
        <b>let</b> current_escrow = *table::borrow(&record.option_escrow, option_id);
        <b>if</b> (current_escrow &gt;= bet.amount) {
            <b>let</b> escrow_ref = table::borrow_mut(&<b>mut</b> record.option_escrow, option_id);
            *escrow_ref = current_escrow - bet.amount;
        };
    };
    <b>if</b> (table::contains(&record.user_option_amounts, bet.user)) {
        <b>let</b> user_amounts = table::borrow_mut(&<b>mut</b> record.user_option_amounts, bet.user);
        <b>let</b> idx = bet.option_id <b>as</b> u64;
        <b>if</b> (idx &lt; vector::length(user_amounts)) {
            <b>let</b> current_user_amount = *vector::borrow(user_amounts, idx);
            <b>if</b> (current_user_amount &gt;= bet.amount) {
                <b>let</b> user_amount_ref = vector::borrow_mut(user_amounts, idx);
                *user_amount_ref = current_user_amount - bet.amount;
            };
        };
    };
    // Remove bet from vector (swap with last and pop)
    <b>let</b> last_index = bets_len - 1;
    <b>if</b> (bet_index != last_index) {
        <b>let</b> last_bet = *vector::borrow(&record.bets, last_index);
        <b>let</b> bet_ref = vector::borrow_mut(&<b>mut</b> record.bets, bet_index);
        *bet_ref = last_bet;
    };
    vector::pop_back(&<b>mut</b> record.bets);
    // Emit withdrawal event
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent">SpotBetWithdrawnEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        user: bet.user,
        option_id: bet.option_id,
        amount: bet.amount,
        fee_taken: fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_place_spot_bet"></a>

## Function `place_spot_bet`

Place bet - all funds go to escrow


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet">place_spot_bet</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, option_id: u8, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet">place_spot_bet</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    option_id: u8,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(spot_config.enable_flag, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (spot_config.max_single_bet &gt; 0) { <b>assert</b>!(amount &lt;= spot_config.max_single_bet, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>); };
    <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    // Validate option_id exists
    <b>let</b> options_len = vector::length(&record.betting_options);
    <b>assert</b>!((option_id <b>as</b> u64) &lt; options_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidOptionId">EInvalidOptionId</a>);
    // All funds go to escrow
    <b>let</b> bet_coin = coin::split(&<b>mut</b> payment, amount, ctx);
    balance::join(&<b>mut</b> record.escrow, coin::into_balance(bet_coin));
    // Update escrow totals with overflow protection
    <b>let</b> current_escrow = <b>if</b> (table::contains(&record.option_escrow, option_id)) {
        *table::borrow(&record.option_escrow, option_id)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_escrow &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>);
    <b>if</b> (table::contains(&record.option_escrow, option_id)) {
        <b>let</b> escrow_ref = table::borrow_mut(&<b>mut</b> record.option_escrow, option_id);
        *escrow_ref = current_escrow + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> record.option_escrow, option_id, amount);
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
        option_id,
        amount,
        timestamp: tx_context::epoch(ctx),
    });
    <b>let</b> user = tx_context::sender(ctx);
    <b>let</b> options_len = vector::length(&record.betting_options);
    <b>if</b> (!table::contains(&record.user_option_amounts, user)) {
        <b>let</b> <b>mut</b> amounts = vector::empty&lt;u64&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; options_len) {
            vector::push_back(&<b>mut</b> amounts, 0);
            i = i + 1;
        };
        table::add(&<b>mut</b> record.user_option_amounts, user, amounts);
    };
    <b>let</b> user_amounts = table::borrow_mut(&<b>mut</b> record.user_option_amounts, user);
    <b>let</b> idx = option_id <b>as</b> u64;
    <b>let</b> current_user_amount = *vector::borrow(user_amounts, idx);
    <b>assert</b>!(current_user_amount &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>);
    <b>let</b> user_amount_ref = vector::borrow_mut(user_amounts, idx);
    *user_amount_ref = current_user_amount + amount;
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetPlacedEvent">SpotBetPlacedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        user: tx_context::sender(ctx),
        option_id,
        amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_oracle_resolve"></a>

## Function `oracle_resolve`

Oracle resolution (option_id, or too close → DAO_REQUIRED)
Requires reasoning and at least one evidence URL for transparency and accountability


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_resolve">oracle_resolve</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, outcome_option_id: u8, confidence_bps: u64, reasoning: <a href="../std/string.md#std_string_String">std::string::String</a>, evidence_urls: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_resolve">oracle_resolve</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    outcome_option_id: u8,
    confidence_bps: u64,
    reasoning: String,
    evidence_urls: vector&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Prevent resolving already resolved or refundable markets
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(option::is_none(&record.outcome), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyResolved">EAlreadyResolved</a>);
    // Enforce resolution window <b>if</b> specified
    <b>let</b> now = tx_context::epoch(ctx);
    <b>if</b> (option::is_some(&record.resolution_window_epochs)) {
        <b>let</b> window = *option::borrow(&record.resolution_window_epochs);
        <b>assert</b>!(now &gt;= record.created_epoch + window, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    };
    // Validate outcome_option_id exists
    <b>let</b> options_len = vector::length(&record.betting_options);
    <b>assert</b>!((outcome_option_id <b>as</b> u64) &lt; options_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidOptionId">EInvalidOptionId</a>);
    // Validate reasoning is required and within limits
    <b>let</b> reasoning_len = string::length(&reasoning);
    <b>assert</b>!(reasoning_len &gt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_REASONING_LENGTH">MIN_REASONING_LENGTH</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>assert</b>!(reasoning_len &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    // Validate evidence URLs - at least one required
    <b>let</b> evidence_urls_len = vector::length(&evidence_urls);
    <b>assert</b>!(evidence_urls_len &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>); // At least one evidence URL required
    <b>assert</b>!(evidence_urls_len &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_EVIDENCE_URLS">MAX_EVIDENCE_URLS</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (confidence_bps &lt; spot_config.confidence_threshold_bps) {
        record.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>;
        event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent">SpotDaoRequiredEvent</a> {
            post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
            confidence_bps,
            reasoning,
        });
        <b>return</b>
    };
    // Resolve outcome - outcome_option_id is the winning option
    // Convert required vector to Option <b>for</b> internal function
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config, record, <a href="../social_contracts/post.md#social_contracts_post">post</a>, outcome_option_id, reasoning, option::some(evidence_urls), ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_via_dao"></a>

## Function `finalize_via_dao`

DAO finalization (YES/NO/DRAW/UNAPPLICABLE)
Reasoning is optional as it represents culmination of community discussion


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_via_dao">finalize_via_dao</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, outcome: u8, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_via_dao">finalize_via_dao</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    outcome: u8,
    <b>mut</b> reasoning: Option&lt;String&gt;,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Allow when DAO_REQUIRED or still OPEN (off-chain DAO direct)
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a> || record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    // Prevent resolving already resolved markets
    <b>assert</b>!(option::is_none(&record.outcome), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyResolved">EAlreadyResolved</a>);
    // Validate reasoning <b>if</b> provided
    <b>if</b> (option::is_some(&reasoning)) {
        <b>let</b> reasoning_val = option::borrow(&reasoning);
        <b>let</b> reasoning_len = string::length(reasoning_val);
        <b>assert</b>!(reasoning_len &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    };
    // Validate evidence URLs <b>if</b> provided
    <b>if</b> (option::is_some(&evidence_urls)) {
        <b>let</b> urls = option::borrow(&evidence_urls);
        <b>assert</b>!(vector::length(urls) &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_EVIDENCE_URLS">MAX_EVIDENCE_URLS</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    };
    // Use provided reasoning or default <a href="../social_contracts/message.md#social_contracts_message">message</a> <b>if</b> not provided
    <b>let</b> final_reasoning = <b>if</b> (option::is_some(&reasoning)) {
        option::extract(&<b>mut</b> reasoning)
    } <b>else</b> {
        string::utf8(b"DAO resolution based on community discussion")
    };
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config, record, <a href="../social_contracts/post.md#social_contracts_post">post</a>, outcome, final_reasoning, evidence_urls, ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_refund_unresolved"></a>

## Function `refund_unresolved`

Refund all escrow if unresolved beyond max window
Requires SpotOracleAdminCap authorization
If max_resolution_window_epochs is None, this function cannot be called (must be explicitly set)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_refund_unresolved">refund_unresolved</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_refund_unresolved">refund_unresolved</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    ctx: &<b>mut</b> TxContext
) {
    // Require max_resolution_window_epochs to be Some - prevents permissionless abuse
    <b>assert</b>!(option::is_some(&record.max_resolution_window_epochs), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    <b>let</b> max_window = *option::borrow(&record.max_resolution_window_epochs);
    <b>assert</b>!(now &gt;= record.created_epoch + max_window, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
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



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, outcome: u8, reasoning: <a href="../std/string.md#std_string_String">std::string::String</a>, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    record: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">SpotRecord</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    outcome: u8, // Winning option_id, or <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a>/<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>
    reasoning: String,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> || record.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(vector::length(&record.bets) &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>);
    // Calculate total escrow across all options
    <b>let</b> <b>mut</b> total_escrow = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> options_len = vector::length(&record.betting_options);
    <b>while</b> (i &lt; options_len) {
        <b>let</b> option_id = (i <b>as</b> u8);
        <b>if</b> (table::contains(&record.option_escrow, option_id)) {
            total_escrow = total_escrow + *table::borrow(&record.option_escrow, option_id);
        };
        i = i + 1;
    };
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
        // Convert Option to vector <b>for</b> event (<b>use</b> empty vector <b>if</b> None)
        <b>let</b> evidence_urls_vec = <b>if</b> (option::is_some(&evidence_urls)) {
            *option::borrow(&evidence_urls)
        } <b>else</b> {
            vector::empty&lt;String&gt;()
        };
        event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> {
            post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
            outcome,
            total_escrow,
            fee_taken: 0,
            reasoning,
            evidence_urls: evidence_urls_vec,
        });
        <b>return</b>
    };
    // Get winning option escrow total
    <b>let</b> winning_total = <b>if</b> (table::contains(&record.option_escrow, outcome)) {
        *table::borrow(&record.option_escrow, outcome)
    } <b>else</b> {
        0
    };
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
    // Distribute to winners pro-rata of winning option escrow
    <b>let</b> <b>mut</b> i = 0; <b>let</b> len = vector::length(&record.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&record.bets, i);
        <b>let</b> winner = bet.option_id == outcome;
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
    // Convert Option to vector <b>for</b> event (<b>use</b> empty vector <b>if</b> None)
    <b>let</b> evidence_urls_vec = <b>if</b> (option::is_some(&evidence_urls)) {
        *option::borrow(&evidence_urls)
    } <b>else</b> {
        vector::empty&lt;String&gt;()
    };
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        outcome,
        total_escrow,
        fee_taken: fee,
        reasoning,
        evidence_urls: evidence_urls_vec,
    });
}
</code></pre>



</details>
