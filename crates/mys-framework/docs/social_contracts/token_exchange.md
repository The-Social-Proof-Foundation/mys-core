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
-  [Struct `TokenInfo`](#social_contracts_token_exchange_TokenInfo)
-  [Struct `TokenPool`](#social_contracts_token_exchange_TokenPool)
-  [Struct `SocialToken`](#social_contracts_token_exchange_SocialToken)
-  [Struct `AuctionInfo`](#social_contracts_token_exchange_AuctionInfo)
-  [Struct `AuctionPool`](#social_contracts_token_exchange_AuctionPool)
-  [Struct `TokenPoolCreatedEvent`](#social_contracts_token_exchange_TokenPoolCreatedEvent)
-  [Struct `TokenBoughtEvent`](#social_contracts_token_exchange_TokenBoughtEvent)
-  [Struct `TokenSoldEvent`](#social_contracts_token_exchange_TokenSoldEvent)
-  [Struct `AuctionCreatedEvent`](#social_contracts_token_exchange_AuctionCreatedEvent)
-  [Struct `AuctionContributionEvent`](#social_contracts_token_exchange_AuctionContributionEvent)
-  [Struct `AuctionFinalizedEvent`](#social_contracts_token_exchange_AuctionFinalizedEvent)
-  [Struct `ConfigUpdatedEvent`](#social_contracts_token_exchange_ConfigUpdatedEvent)
-  [Struct `TokensAddedEvent`](#social_contracts_token_exchange_TokensAddedEvent)
-  [Constants](#@Constants_0)
-  [Function `init`](#social_contracts_token_exchange_init)
-  [Function `update_exchange_config`](#social_contracts_token_exchange_update_exchange_config)
-  [Function `check_post_viral_threshold`](#social_contracts_token_exchange_check_post_viral_threshold)
-  [Function `check_profile_viral_threshold`](#social_contracts_token_exchange_check_profile_viral_threshold)
-  [Function `start_post_auction`](#social_contracts_token_exchange_start_post_auction)
-  [Function `start_profile_auction`](#social_contracts_token_exchange_start_profile_auction)
-  [Function `contribute_to_auction`](#social_contracts_token_exchange_contribute_to_auction)
-  [Function `is_auction_ended`](#social_contracts_token_exchange_is_auction_ended)
-  [Function `finalize_auction`](#social_contracts_token_exchange_finalize_auction)
-  [Function `buy_tokens`](#social_contracts_token_exchange_buy_tokens)
-  [Function `buy_more_tokens`](#social_contracts_token_exchange_buy_more_tokens)
-  [Function `sell_tokens`](#social_contracts_token_exchange_sell_tokens)
-  [Function `calculate_token_price`](#social_contracts_token_exchange_calculate_token_price)
-  [Function `calculate_buy_price`](#social_contracts_token_exchange_calculate_buy_price)
-  [Function `calculate_sell_price`](#social_contracts_token_exchange_calculate_sell_price)
-  [Function `get_token_info`](#social_contracts_token_exchange_get_token_info)
-  [Function `get_token_owner`](#social_contracts_token_exchange_get_token_owner)
-  [Function `get_pool_price`](#social_contracts_token_exchange_get_pool_price)
-  [Function `get_user_balance`](#social_contracts_token_exchange_get_user_balance)
-  [Function `registry_version`](#social_contracts_token_exchange_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_token_exchange_borrow_registry_version_mut)
-  [Function `pool_version`](#social_contracts_token_exchange_pool_version)
-  [Function `borrow_pool_version_mut`](#social_contracts_token_exchange_borrow_pool_version_mut)
-  [Function `auction_version`](#social_contracts_token_exchange_auction_version)
-  [Function `borrow_auction_version_mut`](#social_contracts_token_exchange_borrow_auction_version_mut)
-  [Function `migrate_token_registry`](#social_contracts_token_exchange_migrate_token_registry)
-  [Function `migrate_token_pool`](#social_contracts_token_exchange_migrate_token_pool)
-  [Function `migrate_auction_pool`](#social_contracts_token_exchange_migrate_auction_pool)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/clock.md#mys_clock">mys::clock</a>;
<b>use</b> <a href="../mys/coin.md#mys_coin">mys::coin</a>;
<b>use</b> <a href="../mys/config.md#mys_config">mys::config</a>;
<b>use</b> <a href="../mys/deny_list.md#mys_deny_list">mys::deny_list</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/dynamic_object_field.md#mys_dynamic_object_field">mys::dynamic_object_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
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
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip">social_contracts::my_ip</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
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
<code>post_likes_weight: u64</code>
</dt>
<dd>
 Post viral thresholds & weights
</dd>
<dt>
<code>post_comments_weight: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>post_tips_weight: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>post_viral_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>profile_follows_weight: u64</code>
</dt>
<dd>
 Profile viral thresholds & weights
</dd>
<dt>
<code>profile_posts_weight: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>profile_tips_weight: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>profile_viral_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_post_auction_duration: u64</code>
</dt>
<dd>
 Auction duration limits (in seconds)
</dd>
<dt>
<code>max_post_auction_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_profile_auction_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_profile_auction_duration: u64</code>
</dt>
<dd>
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
<code>auctions: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">social_contracts::token_exchange::AuctionInfo</a>&gt;</code>
</dt>
<dd>
 Table from profile/post ID to auction info
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
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


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a> <b>has</b> key
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

<a name="social_contracts_token_exchange_AuctionInfo"></a>

## Struct `AuctionInfo`

Information about an auction


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">AuctionInfo</a> <b>has</b> <b>copy</b>, drop, store
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
<code>status: u8</code>
</dt>
<dd>
 Status of the auction
</dd>
<dt>
<code>start_time: u64</code>
</dt>
<dd>
 Time when the auction was started
</dd>
<dt>
<code>duration: u64</code>
</dt>
<dd>
 Duration of the auction in seconds
</dd>
<dt>
<code>total_contribution: u64</code>
</dt>
<dd>
 Total MYS contributed to the auction
</dd>
<dt>
<code>total_tokens: u64</code>
</dt>
<dd>
 Total tokens to be distributed
</dd>
<dt>
<code>contributors: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 List of contributors' addresses
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_AuctionPool"></a>

## Struct `AuctionPool`

Pre-launch auction pool


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a> <b>has</b> key
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
<code>info: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">social_contracts::token_exchange::AuctionInfo</a></code>
</dt>
<dd>
 Auction info
</dd>
<dt>
<code>mys_balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 MYS balance contributed to the auction
</dd>
<dt>
<code>contributions: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Mapping of contributors' addresses to their MYS contributions
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

<a name="social_contracts_token_exchange_AuctionCreatedEvent"></a>

## Struct `AuctionCreatedEvent`

Event emitted when an auction is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionCreatedEvent">AuctionCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>auction_id: <b>address</b></code>
</dt>
<dd>
</dd>
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
<code>start_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>duration: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_AuctionContributionEvent"></a>

## Struct `AuctionContributionEvent`

Event emitted when a user contributes to an auction


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionContributionEvent">AuctionContributionEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>auction_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>contributor: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_contribution: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_token_exchange_AuctionFinalizedEvent"></a>

## Struct `AuctionFinalizedEvent`

Event emitted when an auction is finalized


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionFinalizedEvent">AuctionFinalizedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>auction_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_contribution: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_tokens: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>token_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <b>address</b></code>
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
<code>post_viral_threshold: u64</code>
</dt>
<dd>
 Viral thresholds and weights
</dd>
<dt>
<code>profile_viral_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_post_auction_duration: u64</code>
</dt>
<dd>
 Auction durations
</dd>
<dt>
<code>max_post_auction_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_profile_auction_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_profile_auction_duration: u64</code>
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

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_token_exchange_AUCTION_STATUS_ACTIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ACTIVE">AUCTION_STATUS_ACTIVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_token_exchange_AUCTION_STATUS_ENDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ENDED">AUCTION_STATUS_ENDED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_token_exchange_AUCTION_STATUS_FINALIZED"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_FINALIZED">AUCTION_STATUS_FINALIZED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_token_exchange_AUCTION_STATUS_PENDING"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_PENDING">AUCTION_STATUS_PENDING</a>: u8 = 0;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_BASE_PRICE"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_BASE_PRICE">DEFAULT_BASE_PRICE</a>: u64 = 100000000;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_CREATOR_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_CREATOR_FEE_BPS">DEFAULT_CREATOR_FEE_BPS</a>: u64 = 100;
</code></pre>



<a name="social_contracts_token_exchange_DEFAULT_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>: u64 = 25;
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



<a name="social_contracts_token_exchange_EViralThresholdNotMet"></a>

Viral threshold not met


<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EViralThresholdNotMet">EViralThresholdNotMet</a>: u64 = 13;
</code></pre>



<a name="social_contracts_token_exchange_MAX_HOLD_PERCENT_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_HOLD_PERCENT_BPS">MAX_HOLD_PERCENT_BPS</a>: u64 = 500;
</code></pre>



<a name="social_contracts_token_exchange_MAX_POST_AUCTION_DURATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_POST_AUCTION_DURATION">MAX_POST_AUCTION_DURATION</a>: u64 = 10800;
</code></pre>



<a name="social_contracts_token_exchange_MAX_PROFILE_AUCTION_DURATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_PROFILE_AUCTION_DURATION">MAX_PROFILE_AUCTION_DURATION</a>: u64 = 259200;
</code></pre>



<a name="social_contracts_token_exchange_MIN_POST_AUCTION_DURATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MIN_POST_AUCTION_DURATION">MIN_POST_AUCTION_DURATION</a>: u64 = 3600;
</code></pre>



<a name="social_contracts_token_exchange_MIN_PROFILE_AUCTION_DURATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MIN_PROFILE_AUCTION_DURATION">MIN_PROFILE_AUCTION_DURATION</a>: u64 = 86400;
</code></pre>



<a name="social_contracts_token_exchange_POST_COMMENTS_WEIGHT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_COMMENTS_WEIGHT">POST_COMMENTS_WEIGHT</a>: u64 = 3;
</code></pre>



<a name="social_contracts_token_exchange_POST_LIKES_WEIGHT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_LIKES_WEIGHT">POST_LIKES_WEIGHT</a>: u64 = 1;
</code></pre>



<a name="social_contracts_token_exchange_POST_TIPS_WEIGHT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_TIPS_WEIGHT">POST_TIPS_WEIGHT</a>: u64 = 10;
</code></pre>



<a name="social_contracts_token_exchange_POST_VIRAL_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_VIRAL_THRESHOLD">POST_VIRAL_THRESHOLD</a>: u64 = 100;
</code></pre>



<a name="social_contracts_token_exchange_PROFILE_FOLLOWS_WEIGHT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_FOLLOWS_WEIGHT">PROFILE_FOLLOWS_WEIGHT</a>: u64 = 1;
</code></pre>



<a name="social_contracts_token_exchange_PROFILE_POSTS_WEIGHT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_POSTS_WEIGHT">PROFILE_POSTS_WEIGHT</a>: u64 = 1;
</code></pre>



<a name="social_contracts_token_exchange_PROFILE_TIPS_WEIGHT"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_TIPS_WEIGHT">PROFILE_TIPS_WEIGHT</a>: u64 = 5;
</code></pre>



<a name="social_contracts_token_exchange_PROFILE_VIRAL_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_VIRAL_THRESHOLD">PROFILE_VIRAL_THRESHOLD</a>: u64 = 100;
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
            post_likes_weight: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_LIKES_WEIGHT">POST_LIKES_WEIGHT</a>,
            post_comments_weight: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_COMMENTS_WEIGHT">POST_COMMENTS_WEIGHT</a>,
            post_tips_weight: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_TIPS_WEIGHT">POST_TIPS_WEIGHT</a>,
            post_viral_threshold: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_VIRAL_THRESHOLD">POST_VIRAL_THRESHOLD</a>,
            profile_follows_weight: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_FOLLOWS_WEIGHT">PROFILE_FOLLOWS_WEIGHT</a>,
            profile_posts_weight: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_POSTS_WEIGHT">PROFILE_POSTS_WEIGHT</a>,
            profile_tips_weight: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_TIPS_WEIGHT">PROFILE_TIPS_WEIGHT</a>,
            profile_viral_threshold: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_VIRAL_THRESHOLD">PROFILE_VIRAL_THRESHOLD</a>,
            min_post_auction_duration: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MIN_POST_AUCTION_DURATION">MIN_POST_AUCTION_DURATION</a>,
            max_post_auction_duration: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_POST_AUCTION_DURATION">MAX_POST_AUCTION_DURATION</a>,
            min_profile_auction_duration: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MIN_PROFILE_AUCTION_DURATION">MIN_PROFILE_AUCTION_DURATION</a>,
            max_profile_auction_duration: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_PROFILE_AUCTION_DURATION">MAX_PROFILE_AUCTION_DURATION</a>,
        }
    );
    // Create and share token registry
    transfer::share_object(
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a> {
            id: object::new(ctx),
            tokens: table::new(ctx),
            auctions: table::new(ctx),
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        }
    );
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_update_exchange_config"></a>

## Function `update_exchange_config`

Update exchange configuration


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_update_exchange_config">update_exchange_config</a>(_admin_cap: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeAdminCap">social_contracts::token_exchange::ExchangeAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, total_fee_bps: u64, creator_fee_bps: u64, platform_fee_bps: u64, treasury_fee_bps: u64, base_price: u64, quadratic_coefficient: u64, ecosystem_treasury: <b>address</b>, max_hold_percent_bps: u64, post_likes_weight: u64, post_comments_weight: u64, post_tips_weight: u64, post_viral_threshold: u64, profile_follows_weight: u64, profile_posts_weight: u64, profile_tips_weight: u64, profile_viral_threshold: u64, min_post_auction_duration: u64, max_post_auction_duration: u64, min_profile_auction_duration: u64, max_profile_auction_duration: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
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
    post_likes_weight: u64,
    post_comments_weight: u64,
    post_tips_weight: u64,
    post_viral_threshold: u64,
    profile_follows_weight: u64,
    profile_posts_weight: u64,
    profile_tips_weight: u64,
    profile_viral_threshold: u64,
    min_post_auction_duration: u64,
    max_post_auction_duration: u64,
    min_profile_auction_duration: u64,
    max_profile_auction_duration: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Verify sum of fee percentages equals total
    <b>assert</b>!(creator_fee_bps + platform_fee_bps + treasury_fee_bps == total_fee_bps, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Verify curve parameters are valid
    <b>assert</b>!(base_price &gt; 0 && quadratic_coefficient &gt; 0, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidCurveParams">EInvalidCurveParams</a>);
    // Verify auction durations are valid
    <b>assert</b>!(min_post_auction_duration &lt; max_post_auction_duration, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidAuctionDuration">EInvalidAuctionDuration</a>);
    <b>assert</b>!(min_profile_auction_duration &lt; max_profile_auction_duration, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidAuctionDuration">EInvalidAuctionDuration</a>);
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
    // Update viral thresholds & weights
    config.post_likes_weight = post_likes_weight;
    config.post_comments_weight = post_comments_weight;
    config.post_tips_weight = post_tips_weight;
    config.post_viral_threshold = post_viral_threshold;
    config.profile_follows_weight = profile_follows_weight;
    config.profile_posts_weight = profile_posts_weight;
    config.profile_tips_weight = profile_tips_weight;
    config.profile_viral_threshold = profile_viral_threshold;
    // Update auction duration limits
    config.min_post_auction_duration = min_post_auction_duration;
    config.max_post_auction_duration = max_post_auction_duration;
    config.min_profile_auction_duration = min_profile_auction_duration;
    config.max_profile_auction_duration = max_profile_auction_duration;
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
        post_viral_threshold,
        profile_viral_threshold,
        min_post_auction_duration,
        max_post_auction_duration,
        min_profile_auction_duration,
        max_profile_auction_duration,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_check_post_viral_threshold"></a>

## Function `check_post_viral_threshold`

Check if a post has reached the viral threshold


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_check_post_viral_threshold">check_post_viral_threshold</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_check_post_viral_threshold">check_post_viral_threshold</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post
): (bool, u64) {
    // Calculate viral score based on <a href="../social_contracts/post.md#social_contracts_post">post</a> metrics
    <b>let</b> likes = <a href="../social_contracts/post.md#social_contracts_post_get_reaction_count">post::get_reaction_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) * <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_LIKES_WEIGHT">POST_LIKES_WEIGHT</a>;
    <b>let</b> comments = <a href="../social_contracts/post.md#social_contracts_post_get_comment_count">post::get_comment_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) * <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_COMMENTS_WEIGHT">POST_COMMENTS_WEIGHT</a>;
    <b>let</b> tips = <a href="../social_contracts/post.md#social_contracts_post_get_tips_received">post::get_tips_received</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) * <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_TIPS_WEIGHT">POST_TIPS_WEIGHT</a>;
    <b>let</b> viral_score = likes + comments + tips;
    // Check <b>if</b> the score exceeds the threshold
    (viral_score &gt;= <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_POST_VIRAL_THRESHOLD">POST_VIRAL_THRESHOLD</a>, viral_score)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_check_profile_viral_threshold"></a>

## Function `check_profile_viral_threshold`

Check if a profile has reached the viral threshold


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_check_profile_viral_threshold">check_profile_viral_threshold</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, _registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_check_profile_viral_threshold">check_profile_viral_threshold</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    _registry: &UsernameRegistry
): (bool, u64) {
    // Use accessor functions instead of direct field access
    <b>let</b> follows = <a href="../social_contracts/profile.md#social_contracts_profile_get_followers_count">profile::get_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>) * <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_FOLLOWS_WEIGHT">PROFILE_FOLLOWS_WEIGHT</a>;
    <b>let</b> posts = <a href="../social_contracts/profile.md#social_contracts_profile_get_post_count">profile::get_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>) * <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_POSTS_WEIGHT">PROFILE_POSTS_WEIGHT</a>;
    <b>let</b> tips = <a href="../social_contracts/profile.md#social_contracts_profile_get_tips_received">profile::get_tips_received</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>) * <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_TIPS_WEIGHT">PROFILE_TIPS_WEIGHT</a>;
    <b>let</b> viral_score = follows + posts + tips;
    // Check <b>if</b> the score exceeds the threshold
    (viral_score &gt;= <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_PROFILE_VIRAL_THRESHOLD">PROFILE_VIRAL_THRESHOLD</a>, viral_score)
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_start_post_auction"></a>

## Function `start_post_auction`

Start a pre-launch auction for a post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_start_post_auction">start_post_auction</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, _symbol: vector&lt;u8&gt;, _name: vector&lt;u8&gt;, duration_hours: u64, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_start_post_auction">start_post_auction</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    _symbol: vector&lt;u8&gt;,
    _name: vector&lt;u8&gt;,
    duration_hours: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> owner = <a href="../social_contracts/post.md#social_contracts_post_get_owner">post::get_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    // Verify caller is the <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
    <b>assert</b>!(tx_context::sender(ctx) == owner, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> an auction already exists <b>for</b> this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(!table::contains(&registry.auctions, post_id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionInProgress">EAuctionInProgress</a>);
    // Check <b>if</b> the <a href="../social_contracts/post.md#social_contracts_post">post</a> <b>has</b> reached the viral threshold
    <b>let</b> (is_viral, _viral_score) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_check_post_viral_threshold">check_post_viral_threshold</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(is_viral, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EViralThresholdNotMet">EViralThresholdNotMet</a>);
    // Validate auction duration
    <b>let</b> duration_seconds = duration_hours * 60 * 60;
    <b>assert</b>!(
        duration_seconds &gt;= <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MIN_POST_AUCTION_DURATION">MIN_POST_AUCTION_DURATION</a> &&
        duration_seconds &lt;= <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_POST_AUCTION_DURATION">MAX_POST_AUCTION_DURATION</a>,
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidAuctionDuration">EInvalidAuctionDuration</a>
    );
    // Create auction info
    <b>let</b> start_time = clock::timestamp_ms(clock) / 1000; // Convert to seconds
    <b>let</b> auction_info = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">AuctionInfo</a> {
        associated_id: post_id,
        token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        owner,
        status: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ACTIVE">AUCTION_STATUS_ACTIVE</a>,
        start_time,
        duration: duration_seconds,
        total_contribution: 0,
        total_tokens: 0,
        contributors: vector::empty(),
    };
    // Create auction pool
    <b>let</b> auction_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a> {
        id: object::new(ctx),
        info: auction_info,
        mys_balance: balance::zero(),
        contributions: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Add to registry
    table::add(&<b>mut</b> registry.auctions, post_id, auction_info);
    // Emit event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionCreatedEvent">AuctionCreatedEvent</a> {
        auction_id: object::uid_to_address(&auction_pool.id),
        associated_id: post_id,
        token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        owner,
        start_time,
        duration: duration_seconds,
    });
    // Share the auction pool
    transfer::share_object(auction_pool);
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_start_profile_auction"></a>

## Function `start_profile_auction`

Start a pre-launch auction for a profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_start_profile_auction">start_profile_auction</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, _symbol: vector&lt;u8&gt;, _name: vector&lt;u8&gt;, duration_days: u64, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_start_profile_auction">start_profile_auction</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    username_registry: &UsernameRegistry,
    _symbol: vector&lt;u8&gt;,
    _name: vector&lt;u8&gt;,
    duration_days: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> profile_id = <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">profile::get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    <b>let</b> owner = <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">profile::get_owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    // Verify caller is the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> owner
    <b>assert</b>!(tx_context::sender(ctx) == owner, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> an auction already exists <b>for</b> this <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(!table::contains(&registry.auctions, profile_id), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionInProgress">EAuctionInProgress</a>);
    // Check <b>if</b> the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <b>has</b> reached the viral threshold
    <b>let</b> (is_viral, _viral_score) = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_check_profile_viral_threshold">check_profile_viral_threshold</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>, username_registry);
    <b>assert</b>!(is_viral, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EViralThresholdNotMet">EViralThresholdNotMet</a>);
    // Validate auction duration
    <b>let</b> duration_seconds = duration_days * 24 * 60 * 60;
    <b>assert</b>!(
        duration_seconds &gt;= <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MIN_PROFILE_AUCTION_DURATION">MIN_PROFILE_AUCTION_DURATION</a> &&
        duration_seconds &lt;= <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_MAX_PROFILE_AUCTION_DURATION">MAX_PROFILE_AUCTION_DURATION</a>,
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidAuctionDuration">EInvalidAuctionDuration</a>
    );
    // Create auction info
    <b>let</b> start_time = clock::timestamp_ms(clock) / 1000; // Convert to seconds
    <b>let</b> auction_info = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">AuctionInfo</a> {
        associated_id: profile_id,
        token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        owner,
        status: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ACTIVE">AUCTION_STATUS_ACTIVE</a>,
        start_time,
        duration: duration_seconds,
        total_contribution: 0,
        total_tokens: 0,
        contributors: vector::empty(),
    };
    // Create auction pool
    <b>let</b> auction_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a> {
        id: object::new(ctx),
        info: auction_info,
        mys_balance: balance::zero(),
        contributions: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Add to registry
    table::add(&<b>mut</b> registry.auctions, profile_id, auction_info);
    // Emit event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionCreatedEvent">AuctionCreatedEvent</a> {
        auction_id: object::uid_to_address(&auction_pool.id),
        associated_id: profile_id,
        token_type: <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        owner,
        start_time,
        duration: duration_seconds,
    });
    // Share the auction pool
    transfer::share_object(auction_pool);
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_contribute_to_auction"></a>

## Function `contribute_to_auction`

Contribute MYS to an auction


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_contribute_to_auction">contribute_to_auction</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, auction_pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">social_contracts::token_exchange::AuctionPool</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_contribute_to_auction">contribute_to_auction</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    auction_pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a>,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> contributor = tx_context::sender(ctx);
    // Verify auction is active
    <b>assert</b>!(auction_pool.info.status == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ACTIVE">AUCTION_STATUS_ACTIVE</a>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionNotActive">EAuctionNotActive</a>);
    // Verify auction info matches registry
    <b>let</b> stored_info = table::borrow(&registry.auctions, auction_pool.info.associated_id);
    <b>assert</b>!(
        stored_info.owner == auction_pool.info.owner &&
        stored_info.start_time == auction_pool.info.start_time,
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInvalidID">EInvalidID</a>
    );
    // Ensure contributor <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EInsufficientFunds">EInsufficientFunds</a>);
    // Extract payment
    <b>let</b> contribution = coin::split(&<b>mut</b> payment, amount, ctx);
    // Update contribution record
    <b>if</b> (table::contains(&auction_pool.contributions, contributor)) {
        <b>let</b> current_contribution = table::borrow_mut(&<b>mut</b> auction_pool.contributions, contributor);
        *current_contribution = *current_contribution + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> auction_pool.contributions, contributor, amount);
        // Add to contributors list <b>for</b> tracking
        vector::push_back(&<b>mut</b> auction_pool.info.contributors, contributor);
    };
    // Add to pool balance
    balance::join(&<b>mut</b> auction_pool.mys_balance, coin::into_balance(contribution));
    // Update total contribution
    auction_pool.info.total_contribution = auction_pool.info.total_contribution + amount;
    // Update registry
    <b>let</b> <b>mut</b> updated_info = *stored_info;
    updated_info.total_contribution = auction_pool.info.total_contribution;
    // If this is a new contributor, add them to the registry's contributor list too
    <b>if</b> (!table::contains(&auction_pool.contributions, contributor)) {
        vector::push_back(&<b>mut</b> updated_info.contributors, contributor);
    };
    *table::borrow_mut(&<b>mut</b> registry.auctions, auction_pool.info.associated_id) = updated_info;
    // Return any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, contributor);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Emit contribution event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionContributionEvent">AuctionContributionEvent</a> {
        auction_id: object::uid_to_address(&auction_pool.id),
        contributor,
        amount,
        total_contribution: auction_pool.info.total_contribution,
    });
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_is_auction_ended"></a>

## Function `is_auction_ended`

Check if an auction has ended


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_is_auction_ended">is_auction_ended</a>(auction_info: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">social_contracts::token_exchange::AuctionInfo</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_is_auction_ended">is_auction_ended</a>(
    auction_info: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionInfo">AuctionInfo</a>,
    clock: &Clock
): bool {
    <b>let</b> current_time = clock::timestamp_ms(clock) / 1000; // Convert to seconds
    <b>let</b> end_time = auction_info.start_time + auction_info.duration;
    current_time &gt;= end_time
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_finalize_auction"></a>

## Function `finalize_auction`

Finalize an auction and create the token pool
This function checks if the auction has ended and finalizes it by creating a token pool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_finalize_auction">finalize_auction</a>(registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">social_contracts::token_exchange::TokenRegistry</a>, config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">social_contracts::token_exchange::ExchangeConfig</a>, auction_pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">social_contracts::token_exchange::AuctionPool</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_finalize_auction">finalize_auction</a>(
    registry: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ExchangeConfig">ExchangeConfig</a>,
    auction_pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> auction <b>has</b> ended but status not updated
    <b>if</b> (auction_pool.info.status == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ACTIVE">AUCTION_STATUS_ACTIVE</a> && <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_is_auction_ended">is_auction_ended</a>(&auction_pool.info, clock)) {
        // Update status to ended
        auction_pool.info.status = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ENDED">AUCTION_STATUS_ENDED</a>;
        // Update registry
        <b>let</b> <b>mut</b> updated_info = *table::borrow(&registry.auctions, auction_pool.info.associated_id);
        updated_info.status = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ENDED">AUCTION_STATUS_ENDED</a>;
        *table::borrow_mut(&<b>mut</b> registry.auctions, auction_pool.info.associated_id) = updated_info;
    };
    // Verify auction <b>has</b> ended
    <b>assert</b>!(auction_pool.info.status == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_ENDED">AUCTION_STATUS_ENDED</a>, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionNotEnded">EAuctionNotEnded</a>);
    <b>assert</b>!(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_is_auction_ended">is_auction_ended</a>(&auction_pool.info, clock), <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionNotEnded">EAuctionNotEnded</a>);
    // Verify auction <b>has</b> not been finalized
    <b>assert</b>!(
        !table::contains(&registry.tokens, auction_pool.info.associated_id),
        <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_EAuctionAlreadyFinalized">EAuctionAlreadyFinalized</a>
    );
    // Verify there are contributions
    <b>assert</b>!(auction_pool.info.total_contribution &gt; 0, <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_ENoContribution">ENoContribution</a>);
    // Calculate initial token supply with dynamic scaling based on contribution size
    // This creates a non-linear relationship where larger pools get proportionally
    // more tokens, helping to prevent front-running and maintain AMM efficiency
    // Use square root scaling to balance between very large and small pools
    // We <b>use</b> total_contribution^0.75 <b>as</b> our scaling factor
    // (Using integer math <b>for</b> the calculation)
    <b>let</b> contribution = auction_pool.info.total_contribution;
    <b>let</b> sqrt_contribution = math::sqrt(contribution);
    <b>let</b> cbrt_contribution = math::sqrt(sqrt_contribution); // approximation of cube root
    <b>let</b> <b>mut</b> scale_factor = sqrt_contribution * cbrt_contribution; // contribution^0.75
    // Divide the scale factor to make each token worth more than 1 MYSO
    // This ensures tokens are premium assets compared to the base currency
    scale_factor = scale_factor / 1000;
    // Apply different base multipliers <b>for</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> vs <a href="../social_contracts/post.md#social_contracts_post">post</a> tokens
    // Profile tokens have lower supply (more valuable per token)
    // Post tokens have higher supply (more collectible, less valuable per token)
    <b>let</b> <b>mut</b> initial_token_supply = <b>if</b> (auction_pool.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
        // Profile tokens - lower supply (1x base multiplier)
        // These represent long-term investment in a person/brand
        scale_factor
    } <b>else</b> {
        // Post tokens - higher supply (10x base multiplier)
        // These are more collectible with many tokens per viral <a href="../social_contracts/post.md#social_contracts_post">post</a>
        scale_factor * 10
    };
    // Ensure we have at least 1 token
    <b>if</b> (initial_token_supply == 0) {
        initial_token_supply = 1;
    };
    <b>let</b> token_price = auction_pool.info.total_contribution / initial_token_supply;
    // Create token info
    <b>let</b> token_info = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenInfo">TokenInfo</a> {
        id: @0x0, // Temporary, will be updated
        token_type: auction_pool.info.token_type,
        owner: auction_pool.info.owner,
        associated_id: auction_pool.info.associated_id,
        symbol: <b>if</b> (auction_pool.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
            string::utf8(b"PUSER")
        } <b>else</b> {
            string::utf8(b"PPOST")
        },
        name: <b>if</b> (auction_pool.info.token_type == <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
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
    // Create pool with updated token info
    <b>let</b> <b>mut</b> updated_token_info = token_info;
    updated_token_info.id = pool_address;
    <b>let</b> <b>mut</b> token_pool = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_TokenPool">TokenPool</a> {
        id: pool_id,
        info: updated_token_info,
        mys_balance: balance::zero(),
        holders: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Distribute tokens to contributors
    // Production implementation that efficiently distributes tokens to all contributors
    <b>let</b> contributors = &auction_pool.info.contributors;
    <b>let</b> num_contributors = vector::length(contributors);
    // Iterate through all contributors who participated in the auction
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_contributors) {
        <b>let</b> contributor = *vector::borrow(contributors, i);
        <b>let</b> contribution_amount = *table::borrow(&auction_pool.contributions, contributor);
        // Calculate token amount based on contributor's proportion of total contribution
        <b>let</b> token_amount = (contribution_amount * initial_token_supply) / auction_pool.info.total_contribution;
        // Only process non-zero token amounts
        <b>if</b> (token_amount &gt; 0) {
            // Update holder's balance in the pool
            table::add(&<b>mut</b> token_pool.holders, contributor, token_amount);
            // Create social token
            <b>let</b> social_token = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_SocialToken">SocialToken</a> {
                id: object::new(ctx),
                pool_id: pool_address,
                token_type: auction_pool.info.token_type,
                amount: token_amount,
            };
            // Transfer social token to contributor
            transfer::public_transfer(social_token, contributor);
        };
        i = i + 1;
    };
    // Add contribution to pool balance
    balance::join(&<b>mut</b> token_pool.mys_balance, balance::withdraw_all(&<b>mut</b> auction_pool.mys_balance));
    // Update the registry
    table::add(&<b>mut</b> registry.tokens, auction_pool.info.associated_id, updated_token_info);
    // Update auction status
    auction_pool.info.status = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_FINALIZED">AUCTION_STATUS_FINALIZED</a>;
    auction_pool.info.total_tokens = initial_token_supply;
    // Update registry auction info
    <b>let</b> <b>mut</b> updated_auction_info = *table::borrow(&registry.auctions, auction_pool.info.associated_id);
    updated_auction_info.status = <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AUCTION_STATUS_FINALIZED">AUCTION_STATUS_FINALIZED</a>;
    updated_auction_info.total_tokens = initial_token_supply;
    *table::borrow_mut(&<b>mut</b> registry.auctions, auction_pool.info.associated_id) = updated_auction_info;
    // Emit finalized event
    event::emit(<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionFinalizedEvent">AuctionFinalizedEvent</a> {
        auction_id: object::uid_to_address(&auction_pool.id),
        associated_id: auction_pool.info.associated_id,
        total_contribution: auction_pool.info.total_contribution,
        total_tokens: initial_token_supply,
        token_price,
        pool_id: pool_address,
    });
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
    // Extract payment and distribute fees directly
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee
        <b>if</b> (creator_fee &gt; 0) {
            <b>let</b> creator_fee_coin = coin::split(&<b>mut</b> payment, creator_fee, ctx);
            transfer::public_transfer(creator_fee_coin, pool.info.owner);
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
    // Extract payment and distribute fees directly
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee
        <b>if</b> (creator_fee &gt; 0) {
            <b>let</b> creator_fee_coin = coin::split(&<b>mut</b> payment, creator_fee, ctx);
            transfer::public_transfer(creator_fee_coin, pool.info.owner);
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
    // Process and distribute fees
    <b>if</b> (fee_amount &gt; 0) {
        // Send fee to creator
        <b>if</b> (creator_fee &gt; 0) {
            <b>let</b> creator_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.mys_balance, creator_fee), ctx);
            transfer::public_transfer(creator_fee_coin, pool.info.owner);
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

<a name="social_contracts_token_exchange_auction_version"></a>

## Function `auction_version`

Get the version of an auction pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_auction_version">auction_version</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">social_contracts::token_exchange::AuctionPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_auction_version">auction_version</a>(pool: &<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a>): u64 {
    pool.version
}
</code></pre>



</details>

<a name="social_contracts_token_exchange_borrow_auction_version_mut"></a>

## Function `borrow_auction_version_mut`

Get a mutable reference to the auction pool version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_auction_version_mut">borrow_auction_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">social_contracts::token_exchange::AuctionPool</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_borrow_auction_version_mut">borrow_auction_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a>): &<b>mut</b> u64 {
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

<a name="social_contracts_token_exchange_migrate_auction_pool"></a>

## Function `migrate_auction_pool`

Migration function for AuctionPool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_auction_pool">migrate_auction_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">social_contracts::token_exchange::AuctionPool</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_migrate_auction_pool">migrate_auction_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a>,
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
        string::utf8(b"<a href="../social_contracts/token_exchange.md#social_contracts_token_exchange_AuctionPool">AuctionPool</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>
