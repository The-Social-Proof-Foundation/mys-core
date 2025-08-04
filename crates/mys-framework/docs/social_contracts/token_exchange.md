---
title: Module `social_contracts::token_exchange`
---

Token Exchange module for MySocial platform.
This module provides functionality for creation and trading of both profile tokens
and post tokens using an Automated Market Maker (AMM) with a quadratic pricing curve.
It includes fee distribution mechanisms for transactions, splitting between profile owner,
platform, and ecosystem treasury.


-  [Struct `ExchangeAdminCap`](#social_contracts_token_exchange_ExchangeAdminCap)
-  [Struct `ExchangeConfig`](#social_contracts_token_exchange_ExchangeConfig)
-  [Struct `TokenRegistry`](#social_contracts_token_exchange_TokenRegistry)
-  [Struct `StakePool`](#social_contracts_token_exchange_StakePool)
-  [Struct `StakeInfo`](#social_contracts_token_exchange_StakeInfo)
-  [Struct `TokenInfo`](#social_contracts_token_exchange_TokenInfo)
-  [Struct `TokenPool`](#social_contracts_token_exchange_TokenPool)
-  [Struct `SocialToken`](#social_contracts_token_exchange_SocialToken)
-  [Struct `StakePoolObject`](#social_contracts_token_exchange_StakePoolObject)
-  [Struct `TokenPoolCreatedEvent`](#social_contracts_token_exchange_TokenPoolCreatedEvent)
-  [Struct `TokenBoughtEvent`](#social_contracts_token_exchange_TokenBoughtEvent)
-  [Struct `TokenSoldEvent`](#social_contracts_token_exchange_TokenSoldEvent)
-  [Struct `StakeCreatedEvent`](#social_contracts_token_exchange_StakeCreatedEvent)
-  [Struct `StakeWithdrawnEvent`](#social_contracts_token_exchange_StakeWithdrawnEvent)
-  [Struct `ThresholdMetEvent`](#social_contracts_token_exchange_ThresholdMetEvent)
-  [Struct `ConfigUpdatedEvent`](#social_contracts_token_exchange_ConfigUpdatedEvent)
-  [Struct `TokensAddedEvent`](#social_contracts_token_exchange_TokensAddedEvent)
-  [Struct `EmergencyKillSwitchEvent`](#social_contracts_token_exchange_EmergencyKillSwitchEvent)
-  [Constants](#@Constants_0)
-  [Function `init`](#social_contracts_token_exchange_init)
-  [Function `update_exchange_config`](#social_contracts_token_exchange_update_exchange_config)
-  [Function `toggle_emergency_kill_switch`](#social_contracts_token_exchange_toggle_emergency_kill_switch)
-  [Function `is_trading_halted`](#social_contracts_token_exchange_is_trading_halted)
-  [Function `stake_towards_post`](#social_contracts_token_exchange_stake_towards_post)
-  [Function `stake_towards_profile`](#social_contracts_token_exchange_stake_towards_profile)
-  [Function `withdraw_stake`](#social_contracts_token_exchange_withdraw_stake)
-  [Function `create_stake_pool`](#social_contracts_token_exchange_create_stake_pool)
-  [Function `can_create_auction`](#social_contracts_token_exchange_can_create_auction)
-  [Function `create_social_proof_token`](#social_contracts_token_exchange_create_social_proof_token)
-  [Function `update_token_poc_data`](#social_contracts_token_exchange_update_token_poc_data)
-  [Function `calculate_poc_split`](#social_contracts_token_exchange_calculate_poc_split)
-  [Function `apply_token_poc_redirection`](#social_contracts_token_exchange_apply_token_poc_redirection)
-  [Function `distribute_creator_fee`](#social_contracts_token_exchange_distribute_creator_fee)
-  [Function `distribute_creator_fee_from_pool`](#social_contracts_token_exchange_distribute_creator_fee_from_pool)
-  [Function `buy_tokens`](#social_contracts_token_exchange_buy_tokens)
-  [Function `buy_more_tokens`](#social_contracts_token_exchange_buy_more_tokens)
-  [Function `sell_tokens`](#social_contracts_token_exchange_sell_tokens)
-  [Function `calculate_token_price`](#social_contracts_token_exchange_calculate_token_price)
-  [Function `calculate_buy_price`](#social_contracts_token_exchange_calculate_buy_price)
-  [Function `calculate_sell_price`](#social_contracts_token_exchange_calculate_sell_price)
-  [Function `get_token_info`](#social_contracts_token_exchange_get_token_info)
-  [Function `token_exists`](#social_contracts_token_exchange_token_exists)
-  [Function `get_token_owner`](#social_contracts_token_exchange_get_token_owner)
-  [Function `get_pool_price`](#social_contracts_token_exchange_get_pool_price)
-  [Function `get_user_balance`](#social_contracts_token_exchange_get_user_balance)
-  [Function `get_poc_redirect_to`](#social_contracts_token_exchange_get_poc_redirect_to)
-  [Function `get_poc_redirect_percentage`](#social_contracts_token_exchange_get_poc_redirect_percentage)
-  [Function `has_poc_redirection`](#social_contracts_token_exchange_has_poc_redirection)
-  [Function `get_pool_associated_id`](#social_contracts_token_exchange_get_pool_associated_id)
-  [Function `set_poc_redirection`](#social_contracts_token_exchange_set_poc_redirection)
-  [Function `clear_poc_redirection`](#social_contracts_token_exchange_clear_poc_redirection)
-  [Function `registry_version`](#social_contracts_token_exchange_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_token_exchange_borrow_registry_version_mut)
-  [Function `pool_version`](#social_contracts_token_exchange_pool_version)
-  [Function `borrow_pool_version_mut`](#social_contracts_token_exchange_borrow_pool_version_mut)
-  [Function `stake_pool_version`](#social_contracts_token_exchange_stake_pool_version)
-  [Function `borrow_stake_pool_version_mut`](#social_contracts_token_exchange_borrow_stake_pool_version_mut)
-  [Function `migrate_token_registry`](#social_contracts_token_exchange_migrate_token_registry)
-  [Function `migrate_token_pool`](#social_contracts_token_exchange_migrate_token_pool)
-  [Function `migrate_stake_pool`](#social_contracts_token_exchange_migrate_stake_pool)


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



<a name="social_contracts_token_exchange_ExchangeAdminCap"></a>

## Struct `ExchangeAdminCap`

Admin capability for the token exchange


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">ExchangeAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_token_exchange_ExchangeConfig"></a>

## Struct `ExchangeConfig`

Global exchange configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a> <b>has</b> key
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
<code>total_fee_bps: u64</code>
</dt>
<dd>
 Total fee percentage in basis points
</dd>
<dt>
<code>creator_fee_bps: u64</code>
</dt>
<dd>
 Creator fee percentage in basis points
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
 Platform fee percentage in basis points
</dd>
<dt>
<code>treasury_fee_bps: u64</code>
</dt>
<dd>
 Treasury fee percentage in basis points
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
 Base price for new tokens
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
 Quadratic coefficient for pricing curve
</dd>
<dt>
<code>ecosystem_treasury: <b>address</b></code>
</dt>
<dd>
 Ecosystem treasury address
</dd>
<dt>
<code>max_hold_percent_bps: u64</code>
</dt>
<dd>
 Maximum percentage a single wallet can hold of any token
</dd>
<dt>
<code>post_threshold: u64</code>
</dt>
<dd>
 Staking thresholds for social proof token creation
</dd>
<dt>
<code>profile_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_individual_stake_bps: u64</code>
</dt>
<dd>
 Maximum percentage any individual can stake towards a single post/profile
</dd>
<dt>
<code>trading_halted: bool</code>
</dt>
<dd>
 Emergency kill switch - when true, all trading is halted
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokenRegistry"></a>

## Struct `TokenRegistry`

Registry of all tokens in the exchange


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a> <b>has</b> key
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
<code>tokens: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">social_contracts::token_exchange::TokenInfo</a>&gt;</code>
</dt>
<dd>
 Table from token ID to token info
</dd>
<dt>
<code>stake_pools: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePool">social_contracts::token_exchange::StakePool</a>&gt;</code>
</dt>
<dd>
 Table from profile/post ID to staking pool info
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_StakePool"></a>

## Struct `StakePool`

Staking pool for a specific post or profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePool">StakePool</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
 Associated profile or post ID
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
 Token type (1=profile, 2=post)
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner of the profile/post
</dd>
<dt>
<code>total_staked: u64</code>
</dt>
<dd>
 Total MYS staked towards this post/profile
</dd>
<dt>
<code>required_threshold: u64</code>
</dt>
<dd>
 Required threshold to enable auction creation
</dd>
<dt>
<code>stakers: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 List of all stakers (for efficient iteration)
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_StakeInfo"></a>

## Struct `StakeInfo`

Individual stake information


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakeInfo">StakeInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>staker: <b>address</b></code>
</dt>
<dd>
 Staker's address
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 Amount staked in MYS
</dd>
<dt>
<code>staked_at: u64</code>
</dt>
<dd>
 Timestamp when stake was created
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokenInfo"></a>

## Struct `TokenInfo`

Information about a token


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">TokenInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
 The token ID (object ID of the pool)
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
 Type of token (1=profile, 2=post)
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner/creator of the token
</dd>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
 Associated profile or post ID
</dd>
<dt>
<code>symbol: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Token symbol
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Token name
</dd>
<dt>
<code>circulating_supply: u64</code>
</dt>
<dd>
 Current supply in circulation
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
 Base price for this token
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
 Quadratic coefficient for this token's pricing curve
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokenPool"></a>

## Struct `TokenPool`

Liquidity pool for a token


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a> <b>has</b> key, store
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
<code>info: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">social_contracts::token_exchange::TokenInfo</a></code>
</dt>
<dd>
 The token's info
</dd>
<dt>
<code>mys_balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 MYS balance in the pool
</dd>
<dt>
<code>holders: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Mapping of holders' addresses to their token balances
</dd>
<dt>
<code>poc_redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 PoC revenue redirection address (for post tokens only)
</dd>
<dt>
<code>poc_redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 PoC revenue redirection percentage (for post tokens only)
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_SocialToken"></a>

## Struct `SocialToken`

Social token that represents a user's owned tokens


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">SocialToken</a> <b>has</b> key, store
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
<code>pool_id: <b>address</b></code>
</dt>
<dd>
 Token pool ID
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
 Token type (1=profile, 2=post)
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 Amount of tokens held
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_StakePoolObject"></a>

## Struct `StakePoolObject`

Staking pool for collecting MYS stakes towards posts/profiles


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a> <b>has</b> key
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
<code>info: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePool">social_contracts::token_exchange::StakePool</a></code>
</dt>
<dd>
 Stake pool info
</dd>
<dt>
<code>mys_balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 MYS balance staked in this pool
</dd>
<dt>
<code>stakes: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Mapping of stakers' addresses to their stake amounts
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokenPoolCreatedEvent"></a>

## Struct `TokenPoolCreatedEvent`

Event emitted when a token pool is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPoolCreatedEvent">TokenPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>symbol: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokenBoughtEvent"></a>

## Struct `TokenBoughtEvent`

Event emitted when tokens are bought


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenBoughtEvent">TokenBoughtEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>buyer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>mys_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokenSoldEvent"></a>

## Struct `TokenSoldEvent`

Event emitted when tokens are sold


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenSoldEvent">TokenSoldEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seller: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>mys_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_StakeCreatedEvent"></a>

## Struct `StakeCreatedEvent`

Event emitted when MYS is staked towards a post/profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakeCreatedEvent">StakeCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>staker: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_staked: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>threshold_met: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_StakeWithdrawnEvent"></a>

## Struct `StakeWithdrawnEvent`

Event emitted when MYS stake is withdrawn


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakeWithdrawnEvent">StakeWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>staker: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_staked: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>withdrawn_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_ThresholdMetEvent"></a>

## Struct `ThresholdMetEvent`

Event emitted when staking threshold is met for the first time


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ThresholdMetEvent">ThresholdMetEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_staked: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>required_threshold: u64</code>
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

<a name="social_contracts_token_exchange_ConfigUpdatedEvent"></a>

## Struct `ConfigUpdatedEvent`

Event emitted when exchange config is updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ConfigUpdatedEvent">ConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
 Who performed the update
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
 When the update occurred
</dd>
<dt>
<code>total_fee_bps: u64</code>
</dt>
<dd>
 Fee percentages
</dd>
<dt>
<code>creator_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
 Curve parameters
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_treasury: <b>address</b></code>
</dt>
<dd>
 Treasury addresses
</dd>
<dt>
<code>max_hold_percent_bps: u64</code>
</dt>
<dd>
 Maximum hold percentage
</dd>
<dt>
<code>post_threshold: u64</code>
</dt>
<dd>
 Staking thresholds
</dd>
<dt>
<code>profile_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_individual_stake_bps: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_TokensAddedEvent"></a>

## Struct `TokensAddedEvent`

Event emitted when tokens are purchased by someone who already has a social token


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokensAddedEvent">TokensAddedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <b>address</b></code>
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

<a name="social_contracts_token_exchange_EmergencyKillSwitchEvent"></a>

## Struct `EmergencyKillSwitchEvent`

Event emitted when emergency kill switch is toggled


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EmergencyKillSwitchEvent">EmergencyKillSwitchEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
 Admin who activated/deactivated the kill switch
</dd>
<dt>
<code>trading_halted: bool</code>
</dt>
<dd>
 New state of trading (true = halted, false = active)
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
 Timestamp of the action
</dd>
<dt>
<code>reason: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Reason for the action (optional)
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_token_exchange_DEFAULT_BASE_PRICE"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_BASE_PRICE">DEFAULT_BASE_PRICE</a>: u64 = 100000000;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_CREATOR_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_CREATOR_FEE_BPS">DEFAULT_CREATOR_FEE_BPS</a>: u64 = 100;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_MAX_INDIVIDUAL_STAKE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_MAX_INDIVIDUAL_STAKE_BPS">DEFAULT_MAX_INDIVIDUAL_STAKE_BPS</a>: u64 = 2000;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>: u64 = 25;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_POST_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_POST_THRESHOLD">DEFAULT_POST_THRESHOLD</a>: u64 = 1000000000000;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_PROFILE_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_PROFILE_THRESHOLD">DEFAULT_PROFILE_THRESHOLD</a>: u64 = 10000000000000;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_QUADRATIC_COEFFICIENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_QUADRATIC_COEFFICIENT">DEFAULT_QUADRATIC_COEFFICIENT</a>: u64 = 100000;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_TOTAL_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_TOTAL_FEE_BPS">DEFAULT_TOTAL_FEE_BPS</a>: u64 = 150;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_TREASURY_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_TREASURY_FEE_BPS">DEFAULT_TREASURY_FEE_BPS</a>: u64 = 25;
</code></pre>



<a name="social_contracts_token_exchange_EAuctionAlreadyFinalized"></a>

Auction already finalized


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionAlreadyFinalized">EAuctionAlreadyFinalized</a>: u64 = 18;
</code></pre>



<a name="social_contracts_token_exchange_EAuctionInProgress"></a>

Auction already in progress


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionInProgress">EAuctionInProgress</a>: u64 = 14;
</code></pre>



<a name="social_contracts_token_exchange_EAuctionNotActive"></a>

Auction not active


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionNotActive">EAuctionNotActive</a>: u64 = 16;
</code></pre>



<a name="social_contracts_token_exchange_EAuctionNotEnded"></a>

Auction not ended


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionNotEnded">EAuctionNotEnded</a>: u64 = 17;
</code></pre>



<a name="social_contracts_token_exchange_EBlockedUser"></a>

Cannot buy token from a blocked user


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EBlockedUser">EBlockedUser</a>: u64 = 20;
</code></pre>



<a name="social_contracts_token_exchange_EExceededMaxHold"></a>

Exceeded maximum token hold percentage


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EExceededMaxHold">EExceededMaxHold</a>: u64 = 4;
</code></pre>



<a name="social_contracts_token_exchange_EInsufficientFunds"></a>

Insufficient funds for operation


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientFunds">EInsufficientFunds</a>: u64 = 5;
</code></pre>



<a name="social_contracts_token_exchange_EInsufficientLiquidity"></a>

Insufficient token liquidity


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientLiquidity">EInsufficientLiquidity</a>: u64 = 8;
</code></pre>



<a name="social_contracts_token_exchange_EInvalidAuctionDuration"></a>

Invalid auction duration


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidAuctionDuration">EInvalidAuctionDuration</a>: u64 = 15;
</code></pre>



<a name="social_contracts_token_exchange_EInvalidCurveParams"></a>

Curve parameters must be positive


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidCurveParams">EInvalidCurveParams</a>: u64 = 11;
</code></pre>



<a name="social_contracts_token_exchange_EInvalidFeeConfig"></a>

Invalid fee percentages configuration


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidFeeConfig">EInvalidFeeConfig</a>: u64 = 1;
</code></pre>



<a name="social_contracts_token_exchange_EInvalidID"></a>

Invalid post or profile ID


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>: u64 = 7;
</code></pre>



<a name="social_contracts_token_exchange_EInvalidTokenType"></a>

Invalid token type


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidTokenType">EInvalidTokenType</a>: u64 = 12;
</code></pre>



<a name="social_contracts_token_exchange_ENoContribution"></a>

No contribution to auction


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENoContribution">ENoContribution</a>: u64 = 19;
</code></pre>



<a name="social_contracts_token_exchange_ENoTokensOwned"></a>

Sender doesn't own any tokens


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENoTokensOwned">ENoTokensOwned</a>: u64 = 6;
</code></pre>



<a name="social_contracts_token_exchange_ENotAuthorized"></a>

Operation can only be performed by the admin


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENotAuthorized">ENotAuthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_token_exchange_ESelfTrading"></a>

Self trading not allowed


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ESelfTrading">ESelfTrading</a>: u64 = 9;
</code></pre>



<a name="social_contracts_token_exchange_ETokenAlreadyExists"></a>

The token already exists


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenAlreadyExists">ETokenAlreadyExists</a>: u64 = 2;
</code></pre>



<a name="social_contracts_token_exchange_ETokenAlreadyInitialized"></a>

Token already initialized in pool


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenAlreadyInitialized">ETokenAlreadyInitialized</a>: u64 = 10;
</code></pre>



<a name="social_contracts_token_exchange_ETokenNotFound"></a>

The token does not exist


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenNotFound">ETokenNotFound</a>: u64 = 3;
</code></pre>



<a name="social_contracts_token_exchange_ETradingHalted"></a>

Trading is halted by emergency kill switch


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>: u64 = 21;
</code></pre>



<a name="social_contracts_token_exchange_EViralThresholdNotMet"></a>

Viral threshold not met


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EViralThresholdNotMet">EViralThresholdNotMet</a>: u64 = 13;
</code></pre>



<a name="social_contracts_token_exchange_MAX_HOLD_PERCENT_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_HOLD_PERCENT_BPS">MAX_HOLD_PERCENT_BPS</a>: u64 = 500;
</code></pre>



<a name="social_contracts_token_exchange_TOKEN_TYPE_POST"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>: u8 = 2;
</code></pre>



<a name="social_contracts_token_exchange_TOKEN_TYPE_PROFILE"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_token_exchange_init"></a>

## Function `init`

Initialize the token exchange system


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_init">init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_init">init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = tx_context::sender(ctx);
    // Create and transfer admin capability to the transaction sender
    transfer::public_transfer(
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">ExchangeAdminCap</a> {
            id: object::new(ctx),
        },
        sender
    );
    // Create and share exchange config
    transfer::share_object(
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a> {
            id: object::new(ctx),
            total_fee_bps: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_TOTAL_FEE_BPS">DEFAULT_TOTAL_FEE_BPS</a>,
            creator_fee_bps: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_CREATOR_FEE_BPS">DEFAULT_CREATOR_FEE_BPS</a>,
            platform_fee_bps: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>,
            treasury_fee_bps: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_TREASURY_FEE_BPS">DEFAULT_TREASURY_FEE_BPS</a>,
            base_price: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_BASE_PRICE">DEFAULT_BASE_PRICE</a>,
            quadratic_coefficient: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_QUADRATIC_COEFFICIENT">DEFAULT_QUADRATIC_COEFFICIENT</a>,
            ecosystem_treasury: sender, // Initially set to sender, should be updated
            max_hold_percent_bps: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_HOLD_PERCENT_BPS">MAX_HOLD_PERCENT_BPS</a>,
            post_threshold: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_POST_THRESHOLD">DEFAULT_POST_THRESHOLD</a>,
            profile_threshold: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_PROFILE_THRESHOLD">DEFAULT_PROFILE_THRESHOLD</a>,
            max_individual_stake_bps: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_MAX_INDIVIDUAL_STAKE_BPS">DEFAULT_MAX_INDIVIDUAL_STAKE_BPS</a>,
            trading_halted: <b>false</b>, // Trading is enabled by default
        }
    );
    // Create and share token registry
    transfer::share_object(
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a> {
            id: object::new(ctx),
            tokens: table::new(ctx),
            stake_pools: table::new(ctx),
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        }
    );
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_update_exchange_config"></a>

## Function `update_exchange_config`

Update exchange configuration


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_update_exchange_config">update_exchange_config</a>(_admin_cap: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">social_contracts::token_exchange::ExchangeAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, total_fee_bps: u64, creator_fee_bps: u64, platform_fee_bps: u64, treasury_fee_bps: u64, base_price: u64, quadratic_coefficient: u64, ecosystem_treasury: <b>address</b>, max_hold_percent_bps: u64, post_threshold: u64, profile_threshold: u64, max_individual_stake_bps: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_update_exchange_config">update_exchange_config</a>(
    _admin_cap: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">ExchangeAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    total_fee_bps: u64,
    creator_fee_bps: u64,
    platform_fee_bps: u64,
    treasury_fee_bps: u64,
    base_price: u64,
    quadratic_coefficient: u64,
    ecosystem_treasury: <b>address</b>,
    max_hold_percent_bps: u64,
    post_threshold: u64,
    profile_threshold: u64,
    max_individual_stake_bps: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Verify sum of fee percentages equals total
    <b>assert</b>!(creator_fee_bps + platform_fee_bps + treasury_fee_bps == total_fee_bps, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Verify curve parameters are valid
    <b>assert</b>!(base_price &gt; 0 && quadratic_coefficient &gt; 0, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidCurveParams">EInvalidCurveParams</a>);
    // Update fee config
    config.total_fee_bps = total_fee_bps;
    config.creator_fee_bps = creator_fee_bps;
    config.platform_fee_bps = platform_fee_bps;
    config.treasury_fee_bps = treasury_fee_bps;
    // Update curve parameters
    config.base_price = base_price;
    config.quadratic_coefficient = quadratic_coefficient;
    // Update treasury addresses
    config.ecosystem_treasury = ecosystem_treasury;
    config.max_hold_percent_bps = max_hold_percent_bps;
    // Update staking thresholds
    config.post_threshold = post_threshold;
    config.profile_threshold = profile_threshold;
    config.max_individual_stake_bps = max_individual_stake_bps;
    // Emit config updated event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        timestamp: tx_context::epoch(ctx),
        total_fee_bps,
        creator_fee_bps,
        platform_fee_bps,
        treasury_fee_bps,
        base_price,
        quadratic_coefficient,
        ecosystem_treasury,
        max_hold_percent_bps,
        post_threshold,
        profile_threshold,
        max_individual_stake_bps,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_toggle_emergency_kill_switch"></a>

## Function `toggle_emergency_kill_switch`

Emergency kill switch function - only callable by admin
This function can immediately halt all trading on the platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_toggle_emergency_kill_switch">toggle_emergency_kill_switch</a>(_admin_cap: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">social_contracts::token_exchange::ExchangeAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, halt_trading: bool, reason: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_toggle_emergency_kill_switch">toggle_emergency_kill_switch</a>(
    _admin_cap: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">ExchangeAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    halt_trading: bool,
    reason: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Update the trading halted status
    config.trading_halted = halt_trading;
    // Emit event <b>for</b> audit trail
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EmergencyKillSwitchEvent">EmergencyKillSwitchEvent</a> {
        admin: tx_context::sender(ctx),
        trading_halted: halt_trading,
        timestamp: tx_context::epoch(ctx),
        reason: string::utf8(reason),
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_is_trading_halted"></a>

## Function `is_trading_halted`

Check if trading is currently halted


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_is_trading_halted">is_trading_halted</a>(config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_is_trading_halted">is_trading_halted</a>(config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>): bool {
    config.trading_halted
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_stake_towards_post"></a>

## Function `stake_towards_post`

Stake MYS tokens towards a post to support social proof token creation


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_stake_towards_post">stake_towards_post</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_stake_towards_post">stake_towards_post</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    <b>let</b> staker = tx_context::sender(ctx);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> post_owner = <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Verify stake pool matches the <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(stake_pool_object.info.associated_id == post_id, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(stake_pool_object.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidTokenType">EInvalidTokenType</a>);
    // Ensure staker <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount && amount &gt; 0, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientFunds">EInsufficientFunds</a>);
    // Check individual stake limit
    <b>let</b> max_individual_stake = (config.post_threshold * config.max_individual_stake_bps) / 10000;
    <b>let</b> current_stake = <b>if</b> (table::contains(&stake_pool_object.stakes, staker)) {
        *table::borrow(&stake_pool_object.stakes, staker)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_stake + amount &lt;= max_individual_stake, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EExceededMaxHold">EExceededMaxHold</a>);
    // Extract stake payment
    <b>let</b> stake_payment = coin::split(&<b>mut</b> payment, amount, ctx);
    balance::join(&<b>mut</b> stake_pool_object.mys_balance, coin::into_balance(stake_payment));
    // Update staker's balance in the pool
    <b>if</b> (table::contains(&stake_pool_object.stakes, staker)) {
        <b>let</b> stake_balance = table::borrow_mut(&<b>mut</b> stake_pool_object.stakes, staker);
        *stake_balance = *stake_balance + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> stake_pool_object.stakes, staker, amount);
        // Add to stakers list <b>for</b> tracking
        vector::push_back(&<b>mut</b> stake_pool_object.info.stakers, staker);
    };
    // Update total staked
    stake_pool_object.info.total_staked = stake_pool_object.info.total_staked + amount;
    // Update registry
    <b>if</b> (table::contains(&registry.stake_pools, post_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.stake_pools, post_id);
        registry_pool.total_staked = stake_pool_object.info.total_staked;
    } <b>else</b> {
        // Create registry <b>entry</b> <b>if</b> it doesn't exist
        <b>let</b> stake_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePool">StakePool</a> {
            associated_id: post_id,
            token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
            owner: post_owner,
            total_staked: stake_pool_object.info.total_staked,
            required_threshold: config.post_threshold,
            stakers: stake_pool_object.info.stakers,
            created_at: now,
        };
        table::add(&<b>mut</b> registry.stake_pools, post_id, stake_pool);
    };
    // Check <b>if</b> threshold was just met
    <b>let</b> threshold_met = stake_pool_object.info.total_staked &gt;= config.post_threshold;
    <b>let</b> was_threshold_met = (stake_pool_object.info.total_staked - amount) &gt;= config.post_threshold;
    // Emit threshold met event <b>if</b> this stake pushed us over the threshold
    <b>if</b> (threshold_met && !was_threshold_met) {
        event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ThresholdMetEvent">ThresholdMetEvent</a> {
            associated_id: post_id,
            token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
            owner: post_owner,
            total_staked: stake_pool_object.info.total_staked,
            required_threshold: config.post_threshold,
            timestamp: now,
        });
    };
    // Return excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, staker);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Emit stake created event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakeCreatedEvent">StakeCreatedEvent</a> {
        associated_id: post_id,
        token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        staker,
        amount,
        total_staked: stake_pool_object.info.total_staked,
        threshold_met,
        staked_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_stake_towards_profile"></a>

## Function `stake_towards_profile`

Stake MYS tokens towards a profile to support social proof token creation


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_stake_towards_profile">stake_towards_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_stake_towards_profile">stake_towards_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    <b>let</b> staker = tx_context::sender(ctx);
    <b>let</b> profile_id = <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">profile::get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    <b>let</b> profile_owner = <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">profile::get_owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Verify stake pool matches the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(stake_pool_object.info.associated_id == profile_id, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(stake_pool_object.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidTokenType">EInvalidTokenType</a>);
    // Ensure staker <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount && amount &gt; 0, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientFunds">EInsufficientFunds</a>);
    // Check individual stake limit
    <b>let</b> max_individual_stake = (config.profile_threshold * config.max_individual_stake_bps) / 10000;
    <b>let</b> current_stake = <b>if</b> (table::contains(&stake_pool_object.stakes, staker)) {
        *table::borrow(&stake_pool_object.stakes, staker)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_stake + amount &lt;= max_individual_stake, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EExceededMaxHold">EExceededMaxHold</a>);
    // Extract stake payment
    <b>let</b> stake_payment = coin::split(&<b>mut</b> payment, amount, ctx);
    balance::join(&<b>mut</b> stake_pool_object.mys_balance, coin::into_balance(stake_payment));
    // Update staker's balance in the pool
    <b>if</b> (table::contains(&stake_pool_object.stakes, staker)) {
        <b>let</b> stake_balance = table::borrow_mut(&<b>mut</b> stake_pool_object.stakes, staker);
        *stake_balance = *stake_balance + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> stake_pool_object.stakes, staker, amount);
        // Add to stakers list <b>for</b> tracking
        vector::push_back(&<b>mut</b> stake_pool_object.info.stakers, staker);
    };
    // Update total staked
    stake_pool_object.info.total_staked = stake_pool_object.info.total_staked + amount;
    // Update registry
    <b>if</b> (table::contains(&registry.stake_pools, profile_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.stake_pools, profile_id);
        registry_pool.total_staked = stake_pool_object.info.total_staked;
    } <b>else</b> {
        // Create registry <b>entry</b> <b>if</b> it doesn't exist
        <b>let</b> stake_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePool">StakePool</a> {
            associated_id: profile_id,
            token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
            owner: profile_owner,
            total_staked: stake_pool_object.info.total_staked,
            required_threshold: config.profile_threshold,
            stakers: stake_pool_object.info.stakers,
            created_at: now,
        };
        table::add(&<b>mut</b> registry.stake_pools, profile_id, stake_pool);
    };
    // Check <b>if</b> threshold was just met
    <b>let</b> threshold_met = stake_pool_object.info.total_staked &gt;= config.profile_threshold;
    <b>let</b> was_threshold_met = (stake_pool_object.info.total_staked - amount) &gt;= config.profile_threshold;
    // Emit threshold met event <b>if</b> this stake pushed us over the threshold
    <b>if</b> (threshold_met && !was_threshold_met) {
        event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ThresholdMetEvent">ThresholdMetEvent</a> {
            associated_id: profile_id,
            token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
            owner: profile_owner,
            total_staked: stake_pool_object.info.total_staked,
            required_threshold: config.profile_threshold,
            timestamp: now,
        });
    };
    // Return excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, staker);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Emit stake created event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakeCreatedEvent">StakeCreatedEvent</a> {
        associated_id: profile_id,
        token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        staker,
        amount,
        total_staked: stake_pool_object.info.total_staked,
        threshold_met,
        staked_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_withdraw_stake"></a>

## Function `withdraw_stake`

Withdraw MYS stake from a post or profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_withdraw_stake">withdraw_stake</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_withdraw_stake">withdraw_stake</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> staker = tx_context::sender(ctx);
    <b>let</b> associated_id = stake_pool_object.info.associated_id;
    <b>let</b> now = tx_context::epoch(ctx);
    // Verify staker <b>has</b> a stake
    <b>assert</b>!(table::contains(&stake_pool_object.stakes, staker), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> current_stake = *table::borrow(&stake_pool_object.stakes, staker);
    <b>assert</b>!(current_stake &gt;= amount, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Update staker's balance
    <b>if</b> (current_stake == amount) {
        // Remove staker completely
        table::remove(&<b>mut</b> stake_pool_object.stakes, staker);
        // Remove from stakers list
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&stake_pool_object.info.stakers);
        <b>while</b> (i &lt; len) {
            <b>if</b> (*vector::borrow(&stake_pool_object.info.stakers, i) == staker) {
                vector::remove(&<b>mut</b> stake_pool_object.info.stakers, i);
                <b>break</b>
            };
            i = i + 1;
        };
    } <b>else</b> {
        // Reduce stake amount
        <b>let</b> stake_balance = table::borrow_mut(&<b>mut</b> stake_pool_object.stakes, staker);
        *stake_balance = *stake_balance - amount;
    };
    // Update total staked
    stake_pool_object.info.total_staked = stake_pool_object.info.total_staked - amount;
    // Update registry
    <b>if</b> (table::contains(&registry.stake_pools, associated_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.stake_pools, associated_id);
        registry_pool.total_staked = stake_pool_object.info.total_staked;
    };
    // Transfer staked MYS back to staker
    <b>let</b> refund_balance = balance::split(&<b>mut</b> stake_pool_object.mys_balance, amount);
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, staker);
    // Emit stake withdrawn event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakeWithdrawnEvent">StakeWithdrawnEvent</a> {
        associated_id,
        token_type: stake_pool_object.info.token_type,
        staker,
        amount,
        total_staked: stake_pool_object.info.total_staked,
        withdrawn_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_create_stake_pool"></a>

## Function `create_stake_pool`

Create a new stake pool for a post or profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_create_stake_pool">create_stake_pool</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, associated_id: <b>address</b>, token_type: u8, owner: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_create_stake_pool">create_stake_pool</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    associated_id: <b>address</b>,
    token_type: u8,
    owner: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    // Verify caller is the owner
    <b>assert</b>!(tx_context::sender(ctx) == owner, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> stake pool already exists
    <b>assert</b>!(!table::contains(&registry.stake_pools, associated_id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenAlreadyExists">ETokenAlreadyExists</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    <b>let</b> required_threshold = <b>if</b> (token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>) {
        config.post_threshold
    } <b>else</b> <b>if</b> (token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
        config.profile_threshold
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidTokenType">EInvalidTokenType</a>
    };
    // Create stake pool info
    <b>let</b> stake_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePool">StakePool</a> {
        associated_id,
        token_type,
        owner,
        total_staked: 0,
        required_threshold,
        stakers: vector::empty(),
        created_at: now,
    };
    // Add to registry
    table::add(&<b>mut</b> registry.stake_pools, associated_id, stake_pool);
    // Create stake pool object
    <b>let</b> stake_pool_object = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a> {
        id: object::new(ctx),
        info: stake_pool,
        mys_balance: balance::zero(),
        stakes: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    transfer::share_object(stake_pool_object);
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_can_create_auction"></a>

## Function `can_create_auction`

Check if staking threshold is met for auction creation


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_can_create_auction">can_create_auction</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, associated_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_can_create_auction">can_create_auction</a>(
    registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    associated_id: <b>address</b>
): bool {
    <b>if</b> (!table::contains(&registry.stake_pools, associated_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> stake_pool = table::borrow(&registry.stake_pools, associated_id);
    stake_pool.total_staked &gt;= stake_pool.required_threshold
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_create_social_proof_token"></a>

## Function `create_social_proof_token`

Create a social proof token directly from a stake pool once threshold is met
This replaces the auction system - only the post/profile owner can call this


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_create_social_proof_token">create_social_proof_token</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_create_social_proof_token">create_social_proof_token</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    stake_pool_object: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> associated_id = stake_pool_object.info.associated_id;
    // Verify caller is the owner of the <a href="../social_contracts/post.md#social_contracts_post">post</a>/<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(caller == stake_pool_object.info.owner, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> staking threshold <b>has</b> been met
    <b>assert</b>!(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_can_create_auction">can_create_auction</a>(registry, associated_id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EViralThresholdNotMet">EViralThresholdNotMet</a>);
    // Verify token <b>has</b> not already been created
    <b>assert</b>!(!table::contains(&registry.tokens, associated_id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenAlreadyExists">ETokenAlreadyExists</a>);
    // Calculate initial token supply based on total staked amount
    // Use the same scaling formula <b>as</b> the old auction system
    <b>let</b> total_staked = stake_pool_object.info.total_staked;
    <b>let</b> sqrt_staked = math::sqrt(total_staked);
    <b>let</b> cbrt_staked = math::sqrt(sqrt_staked); // approximation of cube root
    <b>let</b> <b>mut</b> scale_factor = sqrt_staked * cbrt_staked; // staked^0.75
    // Divide the scale factor to make each token worth more than 1 MYS
    scale_factor = scale_factor / 1000;
    // Apply different base multipliers <b>for</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> vs <a href="../social_contracts/post.md#social_contracts_post">post</a> tokens
    <b>let</b> <b>mut</b> initial_token_supply = <b>if</b> (stake_pool_object.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
        // Profile tokens - lower supply (more valuable per token)
        scale_factor
    } <b>else</b> {
        // Post tokens - higher supply (more collectible)
        scale_factor * 10
    };
    // Ensure we have at least 1 token
    <b>if</b> (initial_token_supply == 0) {
        initial_token_supply = 1;
    };
    // Create token info
    <b>let</b> token_info = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">TokenInfo</a> {
        id: @0x0, // Temporary, will be updated
        token_type: stake_pool_object.info.token_type,
        owner: stake_pool_object.info.owner,
        associated_id,
        symbol: <b>if</b> (stake_pool_object.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
            string::utf8(b"PUSER")
        } <b>else</b> {
            string::utf8(b"PPOST")
        },
        name: <b>if</b> (stake_pool_object.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
            string::utf8(b"Profile Token")
        } <b>else</b> {
            string::utf8(b"Post Token")
        },
        circulating_supply: initial_token_supply,
        base_price: config.base_price,
        quadratic_coefficient: config.quadratic_coefficient,
        created_at: tx_context::epoch(ctx),
    };
    // Create token pool
    <b>let</b> pool_id = object::new(ctx);
    <b>let</b> pool_address = object::uid_to_address(&pool_id);
    // Update token info with actual pool <b>address</b>
    <b>let</b> <b>mut</b> updated_token_info = token_info;
    updated_token_info.id = pool_address;
    <b>let</b> <b>mut</b> token_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a> {
        id: pool_id,
        info: updated_token_info,
        mys_balance: balance::zero(),
        holders: table::new(ctx),
        poc_redirect_to: option::none(),
        poc_redirect_percentage: option::none(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Distribute tokens to stakers proportionally
    <b>let</b> stakers = &stake_pool_object.info.stakers;
    <b>let</b> num_stakers = vector::length(stakers);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_stakers) {
        <b>let</b> staker = *vector::borrow(stakers, i);
        <b>let</b> stake_amount = *table::borrow(&stake_pool_object.stakes, staker);
        // Calculate token amount based on staker's proportion of total stake
        <b>let</b> token_amount = (stake_amount * initial_token_supply) / total_staked;
        <b>if</b> (token_amount &gt; 0) {
            // Update holder's balance in the pool
            table::add(&<b>mut</b> token_pool.holders, staker, token_amount);
            // Create social token <b>for</b> the staker
            <b>let</b> social_token = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">SocialToken</a> {
                id: object::new(ctx),
                pool_id: pool_address,
                token_type: stake_pool_object.info.token_type,
                amount: token_amount,
            };
            // Transfer social token to staker
            transfer::public_transfer(social_token, staker);
        };
        i = i + 1;
    };
    // Transfer all staked MYS to the token pool <b>as</b> initial liquidity
    balance::join(&<b>mut</b> token_pool.mys_balance, balance::withdraw_all(&<b>mut</b> stake_pool_object.mys_balance));
    // Clear the stake pool since it's now converted to a token
    stake_pool_object.info.total_staked = 0;
    // Note: We keep the stakes table <b>for</b> reference but it's no longer active
    // Add to registry
    table::add(&<b>mut</b> registry.tokens, associated_id, updated_token_info);
    // Emit token created event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPoolCreatedEvent">TokenPoolCreatedEvent</a> {
        id: pool_address,
        token_type: updated_token_info.token_type,
        owner: updated_token_info.owner,
        associated_id: updated_token_info.associated_id,
        symbol: updated_token_info.symbol,
        name: updated_token_info.name,
        base_price: updated_token_info.base_price,
        quadratic_coefficient: updated_token_info.quadratic_coefficient,
    });
    // Share the token pool
    transfer::share_object(token_pool);
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_update_token_poc_data"></a>

## Function `update_token_poc_data`

Update PoC redirection data for a token pool (called by PoC system)
This function copies PoC data from a post into the corresponding token pool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_update_token_poc_data">update_token_poc_data</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_update_token_poc_data">update_token_poc_data</a>(
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    ctx: &<b>mut</b> TxContext
) {
    // Verify this is a <a href="../social_contracts/post.md#social_contracts_post">post</a> token pool
    <b>assert</b>!(pool.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidTokenType">EInvalidTokenType</a>);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> matches the token pool
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(post_id == pool.info.associated_id, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>);
    // Verify caller is authorized (<a href="../social_contracts/post.md#social_contracts_post">post</a> owner)
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(caller == <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENotAuthorized">ENotAuthorized</a>);
    // Copy PoC data from <a href="../social_contracts/post.md#social_contracts_post">post</a> to pool
    pool.poc_redirect_to = <b>if</b> (option::is_some(<a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">post::get_revenue_redirect_to</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>))) {
        option::some(*option::borrow(<a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">post::get_revenue_redirect_to</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)))
    } <b>else</b> {
        option::none()
    };
    pool.poc_redirect_percentage = <b>if</b> (option::is_some(<a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_percentage">post::get_revenue_redirect_percentage</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>))) {
        option::some(*option::borrow(<a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_percentage">post::get_revenue_redirect_percentage</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)))
    } <b>else</b> {
        option::none()
    };
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_calculate_poc_split"></a>

## Function `calculate_poc_split`

Calculate PoC revenue split - shared utility for consistent logic


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_poc_split">calculate_poc_split</a>(amount: u64, redirect_percentage: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_poc_split">calculate_poc_split</a>(amount: u64, redirect_percentage: u64): (u64, u64) {
    <b>let</b> redirected_amount = (amount * redirect_percentage) / 100;
    <b>let</b> remaining_amount = amount - redirected_amount;
    (redirected_amount, remaining_amount)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_apply_token_poc_redirection"></a>

## Function `apply_token_poc_redirection`

Apply PoC redirection to creator fees with consolidated logic


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_apply_token_poc_redirection">apply_token_poc_redirection</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, amount: u64, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_apply_token_poc_redirection">apply_token_poc_redirection</a>(
    pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    amount: u64,
    _ctx: &<b>mut</b> TxContext
): (u64, u64) {
    <b>if</b> (<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_has_poc_redirection">has_poc_redirection</a>(pool)) {
        <b>let</b> redirect_percentage = *option::borrow(&pool.poc_redirect_percentage);
        // Use shared utility function <b>for</b> consistent calculation
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_poc_split">calculate_poc_split</a>(amount, redirect_percentage)
    } <b>else</b> {
        (0, amount)
    }
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_distribute_creator_fee"></a>

## Function `distribute_creator_fee`

Distribute creator fees with automatic PoC redirection


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee">distribute_creator_fee</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, creator_fee_amount: u64, creator_fee_coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee">distribute_creator_fee</a>(
    pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    creator_fee_amount: u64,
    creator_fee_coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee_amount == 0) {
        <b>return</b>
    };
    <b>let</b> (redirected_amount, _remaining_amount) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_apply_token_poc_redirection">apply_token_poc_redirection</a>(pool, creator_fee_amount, ctx);
    <b>let</b> <b>mut</b> fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
    <b>if</b> (redirected_amount &gt; 0) {
        // Split the fee: redirected portion goes to original creator, remainder to <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
        <b>let</b> redirected_fee = coin::split(&<b>mut</b> fee_coin, redirected_amount, ctx);
        <b>let</b> redirect_to = *option::borrow(&pool.poc_redirect_to);
        transfer::public_transfer(redirected_fee, redirect_to);
        // Send remainder to current <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
        <b>if</b> (coin::value(&fee_coin) &gt; 0) {
            transfer::public_transfer(fee_coin, pool.info.owner);
        } <b>else</b> {
            coin::destroy_zero(fee_coin);
        };
    } <b>else</b> {
        // No redirection - send full amount to current <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
        transfer::public_transfer(fee_coin, pool.info.owner);
    };
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_distribute_creator_fee_from_pool"></a>

## Function `distribute_creator_fee_from_pool`

Distribute creator fees from pool balance with PoC redirection support


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, creator_fee: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    creator_fee: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee == 0) {
        <b>return</b>
    };
    <b>let</b> (redirected_amount, _remaining_amount) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_apply_token_poc_redirection">apply_token_poc_redirection</a>(pool, creator_fee, ctx);
    <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.mys_balance, creator_fee), ctx);
    <b>if</b> (redirected_amount &gt; 0) {
        // Split the fee: redirected portion goes to original creator, remainder to <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
        <b>let</b> redirected_fee = coin::split(&<b>mut</b> fee_coin, redirected_amount, ctx);
        <b>let</b> redirect_to = *option::borrow(&pool.poc_redirect_to);
        transfer::public_transfer(redirected_fee, redirect_to);
        // Send remainder to current <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
        <b>if</b> (coin::value(&fee_coin) &gt; 0) {
            transfer::public_transfer(fee_coin, pool.info.owner);
        } <b>else</b> {
            coin::destroy_zero(fee_coin);
        };
    } <b>else</b> {
        // No redirection - send full amount to current <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
        transfer::public_transfer(fee_coin, pool.info.owner);
    };
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_buy_tokens"></a>

## Function `buy_tokens`

Buy tokens from the pool - first purchase
This function handles buying tokens for first-time buyers of a specific token


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_buy_tokens">buy_tokens</a>(_registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_buy_tokens">buy_tokens</a>(
    _registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Prevent self-trading <b>for</b> token owners
    <b>assert</b>!(buyer != pool.info.owner, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ESelfTrading">ESelfTrading</a>);
    // Check <b>if</b> token owner is blocked by the buyer
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">social_contracts::block_list::is_blocked</a>(block_list_registry, buyer, pool.info.owner), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EBlockedUser">EBlockedUser</a>);
    // Calculate the price <b>for</b> the tokens based on quadratic curve
    <b>let</b> (price, _) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_buy_price">calculate_buy_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Ensure buyer <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientFunds">EInsufficientFunds</a>);
    // Calculate fees
    <b>let</b> fee_amount = (price * config.total_fee_bps) / 10000;
    <b>let</b> creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
    <b>let</b> platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate the net amount to the liquidity pool
    <b>let</b> net_amount = price - fee_amount;
    // Extract payment and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee">distribute_creator_fee</a>(pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee - add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            // Add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, ctx);
            // Destroy the emptied coin
            coin::destroy_zero(platform_fee_coin);
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
        };
    };
    // Add remaining payment to pool
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, net_amount, ctx);
    balance::join(&<b>mut</b> pool.mys_balance, coin::into_balance(pool_payment));
    // Refund any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Update holder's balance
    <b>let</b> max_hold = (pool.info.circulating_supply + amount) * config.max_hold_percent_bps / 10000;
    <b>let</b> current_hold = <b>if</b> (table::contains(&pool.holders, buyer)) {
        *table::borrow(&pool.holders, buyer)
    } <b>else</b> {
        0
    };
    // Check max holding limit
    <b>assert</b>!(current_hold + amount &lt;= max_hold, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EExceededMaxHold">EExceededMaxHold</a>);
    // Check that this is the first purchase
    <b>assert</b>!(current_hold == 0, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenAlreadyExists">ETokenAlreadyExists</a>);
    // Update holder's balance
    table::add(&<b>mut</b> pool.holders, buyer, amount);
    // Update circulating supply
    pool.info.circulating_supply = pool.info.circulating_supply + amount;
    // Mint new social token <b>for</b> the user
    <b>let</b> social_token = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">SocialToken</a> {
        id: object::new(ctx),
        pool_id: object::uid_to_address(&pool.id),
        token_type: pool.info.token_type,
        amount,
    };
    transfer::public_transfer(social_token, buyer);
    // Calculate the new price after purchase
    <b>let</b> new_price = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit buy event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: object::uid_to_address(&pool.id),
        buyer,
        amount,
        mys_amount: price,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_buy_more_tokens"></a>

## Function `buy_more_tokens`

Buy more tokens when you already have a social token
This function allows users to add to their existing token holdings using MYS Coin


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_buy_more_tokens">buy_more_tokens</a>(_registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, social_token: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">social_contracts::token_exchange::SocialToken</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_buy_more_tokens">buy_more_tokens</a>(
    _registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    amount: u64,
    social_token: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">SocialToken</a>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Prevent self-trading <b>for</b> token owners
    <b>assert</b>!(buyer != pool.info.owner, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ESelfTrading">ESelfTrading</a>);
    // Check <b>if</b> token owner is blocked by the buyer
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">social_contracts::block_list::is_blocked</a>(block_list_registry, buyer, pool.info.owner), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EBlockedUser">EBlockedUser</a>);
    // Verify social token matches the pool
    <b>assert</b>!(social_token.pool_id == object::uid_to_address(&pool.id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>);
    // Calculate the price <b>for</b> the tokens based on quadratic curve
    <b>let</b> (price, _) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_buy_price">calculate_buy_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Ensure buyer <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientFunds">EInsufficientFunds</a>);
    // Calculate fees
    <b>let</b> fee_amount = (price * config.total_fee_bps) / 10000;
    <b>let</b> creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
    <b>let</b> platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate the net amount to the liquidity pool
    <b>let</b> net_amount = price - fee_amount;
    // Extract payment and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee">distribute_creator_fee</a>(pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee - add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            // Add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, ctx);
            // Destroy the emptied coin
            coin::destroy_zero(platform_fee_coin);
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
        };
    };
    // Add remaining payment to pool
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, net_amount, ctx);
    balance::join(&<b>mut</b> pool.mys_balance, coin::into_balance(pool_payment));
    // Refund any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Update holder's balance
    <b>let</b> max_hold = (pool.info.circulating_supply + amount) * config.max_hold_percent_bps / 10000;
    <b>let</b> current_hold = <b>if</b> (table::contains(&pool.holders, buyer)) {
        *table::borrow(&pool.holders, buyer)
    } <b>else</b> {
        0
    };
    // Check max holding limit
    <b>assert</b>!(current_hold + amount &lt;= max_hold, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EExceededMaxHold">EExceededMaxHold</a>);
    // Update holder's balance
    <b>if</b> (table::contains(&pool.holders, buyer)) {
        <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> pool.holders, buyer);
        *holder_balance = *holder_balance + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> pool.holders, buyer, amount);
    };
    // Update circulating supply
    pool.info.circulating_supply = pool.info.circulating_supply + amount;
    // Update the user's social token
    social_token.amount = social_token.amount + amount;
    // Calculate the new price after purchase
    <b>let</b> new_price = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit buy event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: object::uid_to_address(&pool.id),
        buyer,
        amount,
        mys_amount: price,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_sell_tokens"></a>

## Function `sell_tokens`

Sell tokens back to the pool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_sell_tokens">sell_tokens</a>(_registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, social_token: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">social_contracts::token_exchange::SocialToken</a>, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_sell_tokens">sell_tokens</a>(
    _registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    social_token: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">SocialToken</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(!config.trading_halted, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETradingHalted">ETradingHalted</a>);
    <b>let</b> seller = tx_context::sender(ctx);
    <b>let</b> pool_id = object::uid_to_address(&pool.id);
    // Verify social token matches the pool
    <b>assert</b>!(social_token.pool_id == pool_id, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(social_token.amount &gt;= amount, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Calculate the sell price based on quadratic curve
    <b>let</b> (refund_amount, _) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_sell_price">calculate_sell_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Calculate fees
    <b>let</b> fee_amount = (refund_amount * config.total_fee_bps) / 10000;
    <b>let</b> creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
    <b>let</b> platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate net refund
    <b>let</b> net_refund = refund_amount - fee_amount;
    // Ensure pool <b>has</b> enough liquidity
    <b>assert</b>!(balance::value(&pool.mys_balance) &gt;= net_refund, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Update holder balance
    <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> pool.holders, seller);
    *holder_balance = *holder_balance - amount;
    // Update user's social token
    social_token.amount = social_token.amount - amount;
    // Update circulating supply
    pool.info.circulating_supply = pool.info.circulating_supply - amount;
    // Extract net refund from pool
    <b>let</b> refund_balance = balance::split(&<b>mut</b> pool.mys_balance, net_refund);
    // Process and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send fee to creator with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(pool, creator_fee, ctx);
        };
        // Send fee to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> - add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.mys_balance, platform_fee), ctx);
            // Add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, ctx);
            // Destroy the emptied coin
            coin::destroy_zero(platform_fee_coin);
        };
        // Send fee to treasury
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.mys_balance, treasury_fee), ctx);
            transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
        };
    };
    // Transfer refund to seller
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, seller);
    // Calculate the new price after sale
    <b>let</b> new_price = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit sell event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenSoldEvent">TokenSoldEvent</a> {
        id: pool_id,
        seller,
        amount,
        mys_amount: refund_amount,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_calculate_token_price"></a>

## Function `calculate_token_price`

Calculate token price at current supply based on quadratic curve
Price = base_price + (quadratic_coefficient * supply^2)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(base_price: u64, quadratic_coefficient: u64, supply: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    supply: u64
): u64 {
    <b>let</b> squared_supply = supply * supply;
    base_price + (quadratic_coefficient * squared_supply / 10000)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_calculate_buy_price"></a>

## Function `calculate_buy_price`

Calculate price to buy a specific amount of tokens
Returns (total price, average price per token)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_buy_price">calculate_buy_price</a>(base_price: u64, quadratic_coefficient: u64, current_supply: u64, amount: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_buy_price">calculate_buy_price</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    current_supply: u64,
    amount: u64
): (u64, u64) {
    <b>let</b> <b>mut</b> total_price = 0;
    <b>let</b> <b>mut</b> current = current_supply;
    <b>let</b> <b>mut</b> i = 0;
    // Integrate the price curve over the purchase amount
    <b>while</b> (i &lt; amount) {
        <b>let</b> token_price = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(base_price, quadratic_coefficient, current);
        total_price = total_price + token_price;
        current = current + 1;
        i = i + 1;
    };
    <b>let</b> avg_price = <b>if</b> (amount &gt; 0) {
        total_price / amount
    } <b>else</b> {
        0
    };
    (total_price, avg_price)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_calculate_sell_price"></a>

## Function `calculate_sell_price`

Calculate refund amount when selling tokens
Returns (total refund, average price per token)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_sell_price">calculate_sell_price</a>(base_price: u64, quadratic_coefficient: u64, current_supply: u64, amount: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_sell_price">calculate_sell_price</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    current_supply: u64,
    amount: u64
): (u64, u64) {
    <b>let</b> <b>mut</b> total_refund = 0;
    <b>let</b> <b>mut</b> current = current_supply;
    <b>let</b> <b>mut</b> i = 0;
    // Integrate the price curve over the sell amount
    <b>while</b> (i &lt; amount) {
        current = current - 1;
        <b>let</b> token_price = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(base_price, quadratic_coefficient, current);
        total_refund = total_refund + token_price;
        i = i + 1;
    };
    <b>let</b> avg_price = <b>if</b> (amount &gt; 0) {
        total_refund / amount
    } <b>else</b> {
        0
    };
    (total_refund, avg_price)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_token_info"></a>

## Function `get_token_info`

Get token info from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_token_info">get_token_info</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, id: <b>address</b>): <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">social_contracts::token_exchange::TokenInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_token_info">get_token_info</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>, id: <b>address</b>): <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">TokenInfo</a> {
    <b>assert</b>!(table::contains(&registry.tokens, id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ETokenNotFound">ETokenNotFound</a>);
    *table::borrow(&registry.tokens, id)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_token_exists"></a>

## Function `token_exists`

Check if a token exists in the registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_token_exists">token_exists</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_token_exists">token_exists</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>, id: <b>address</b>): bool {
    table::contains(&registry.tokens, id)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_token_owner"></a>

## Function `get_token_owner`

Get token owner's address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_token_owner">get_token_owner</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, id: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_token_owner">get_token_owner</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>, id: <b>address</b>): <b>address</b> {
    <b>let</b> info = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_token_info">get_token_info</a>(registry, id);
    info.owner
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_pool_price"></a>

## Function `get_pool_price`

Get current token price for a specific pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_pool_price">get_pool_price</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_pool_price">get_pool_price</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): u64 {
    <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    )
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_user_balance"></a>

## Function `get_user_balance`

Get user's token balance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_user_balance">get_user_balance</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, user: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_user_balance">get_user_balance</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>, user: <b>address</b>): u64 {
    <b>if</b> (table::contains(&pool.holders, user)) {
        *table::borrow(&pool.holders, user)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_poc_redirect_to"></a>

## Function `get_poc_redirect_to`

Get PoC redirection data from token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_poc_redirect_to">get_poc_redirect_to</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_poc_redirect_to">get_poc_redirect_to</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): &Option&lt;<b>address</b>&gt; {
    &pool.poc_redirect_to
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_poc_redirect_percentage"></a>

## Function `get_poc_redirect_percentage`

Get PoC redirection percentage from token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_poc_redirect_percentage">get_poc_redirect_percentage</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_poc_redirect_percentage">get_poc_redirect_percentage</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): &Option&lt;u64&gt; {
    &pool.poc_redirect_percentage
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_has_poc_redirection"></a>

## Function `has_poc_redirection`

Check if token pool has PoC redirection configured


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_has_poc_redirection">has_poc_redirection</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_has_poc_redirection">has_poc_redirection</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): bool {
    option::is_some(&pool.poc_redirect_to) && option::is_some(&pool.poc_redirect_percentage)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_get_pool_associated_id"></a>

## Function `get_pool_associated_id`

Get the associated ID (post/profile ID) from a token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_pool_associated_id">get_pool_associated_id</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_get_pool_associated_id">get_pool_associated_id</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): <b>address</b> {
    pool.info.associated_id
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_set_poc_redirection"></a>

## Function `set_poc_redirection`

Set PoC redirection data for a token pool (called by PoC system)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_set_poc_redirection">set_poc_redirection</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_set_poc_redirection">set_poc_redirection</a>(
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    redirect_to: Option&lt;<b>address</b>&gt;,
    redirect_percentage: Option&lt;u64&gt;
) {
    pool.poc_redirect_to = redirect_to;
    pool.poc_redirect_percentage = redirect_percentage;
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_clear_poc_redirection"></a>

## Function `clear_poc_redirection`

Clear PoC redirection data from a token pool (called by PoC system)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_clear_poc_redirection">clear_poc_redirection</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_clear_poc_redirection">clear_poc_redirection</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>) {
    pool.poc_redirect_to = option::none();
    pool.poc_redirect_percentage = option::none();
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_registry_version"></a>

## Function `registry_version`

Get the version of the token registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_registry_version">registry_version</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_registry_version">registry_version</a>(registry: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>): u64 {
    registry.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_pool_version"></a>

## Function `pool_version`

Get the version of a token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_pool_version">pool_version</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_pool_version">pool_version</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): u64 {
    pool.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_borrow_pool_version_mut"></a>

## Function `borrow_pool_version_mut`

Get a mutable reference to the pool version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_pool_version_mut">borrow_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_pool_version_mut">borrow_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>): &<b>mut</b> u64 {
    &<b>mut</b> pool.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_stake_pool_version"></a>

## Function `stake_pool_version`

Get the version of a stake pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_stake_pool_version">stake_pool_version</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_stake_pool_version">stake_pool_version</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>): u64 {
    pool.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_borrow_stake_pool_version_mut"></a>

## Function `borrow_stake_pool_version_mut`

Get a mutable reference to the stake pool version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_stake_pool_version_mut">borrow_stake_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_stake_pool_version_mut">borrow_stake_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>): &<b>mut</b> u64 {
    &<b>mut</b> pool.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_migrate_token_registry"></a>

## Function `migrate_token_registry`

Migration function for TokenRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_token_registry">migrate_token_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_token_registry">migrate_token_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(registry.version &lt; current_version, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = registry.version;
    registry.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_migrate_token_pool"></a>

## Function `migrate_token_pool`

Migration function for TokenPool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_token_pool">migrate_token_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">social_contracts::token_exchange::TokenPool</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_token_pool">migrate_token_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(pool.version &lt; current_version, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = pool.version;
    pool.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> pool_id = object::id(pool);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        pool_id,
        string::utf8(b"<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_migrate_stake_pool"></a>

## Function `migrate_stake_pool`

Migration function for StakePoolObject


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_stake_pool">migrate_stake_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">social_contracts::token_exchange::StakePoolObject</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_stake_pool">migrate_stake_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(pool.version &lt; current_version, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = pool.version;
    pool.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> pool_id = object::id(pool);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        pool_id,
        string::utf8(b"<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_StakePoolObject">StakePoolObject</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>
