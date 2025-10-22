---
title: Module `social_contracts::mydata`
---

Universal MyData module for encrypted data monetization
Supports both one-time purchases and subscription access
Can be attached to posts (gated content) or profiles (data monetization)


-  [Struct `MyData`](#social_contracts_mydata_MyData)
-  [Struct `MyDataRegistry`](#social_contracts_mydata_MyDataRegistry)
-  [Struct `MyDataCreatedEvent`](#social_contracts_mydata_MyDataCreatedEvent)
-  [Struct `PurchaseEvent`](#social_contracts_mydata_PurchaseEvent)
-  [Struct `AccessGrantedEvent`](#social_contracts_mydata_AccessGrantedEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_mydata_bootstrap_init)
-  [Function `create`](#social_contracts_mydata_create)
-  [Function `create_and_share`](#social_contracts_mydata_create_and_share)
-  [Function `purchase_one_time`](#social_contracts_mydata_purchase_one_time)
-  [Function `purchase_subscription`](#social_contracts_mydata_purchase_subscription)
-  [Function `update_pricing`](#social_contracts_mydata_update_pricing)
-  [Function `update_content`](#social_contracts_mydata_update_content)
-  [Function `has_access`](#social_contracts_mydata_has_access)
-  [Function `decrypt_data`](#social_contracts_mydata_decrypt_data)
-  [Function `grant_access`](#social_contracts_mydata_grant_access)
-  [Function `owner`](#social_contracts_mydata_owner)
-  [Function `media_type`](#social_contracts_mydata_media_type)
-  [Function `tags`](#social_contracts_mydata_tags)
-  [Function `platform_id`](#social_contracts_mydata_platform_id)
-  [Function `one_time_price`](#social_contracts_mydata_one_time_price)
-  [Function `subscription_price`](#social_contracts_mydata_subscription_price)
-  [Function `subscription_duration_days`](#social_contracts_mydata_subscription_duration_days)
-  [Function `created_at`](#social_contracts_mydata_created_at)
-  [Function `last_updated`](#social_contracts_mydata_last_updated)
-  [Function `timestamp_start`](#social_contracts_mydata_timestamp_start)
-  [Function `timestamp_end`](#social_contracts_mydata_timestamp_end)
-  [Function `geographic_region`](#social_contracts_mydata_geographic_region)
-  [Function `data_quality`](#social_contracts_mydata_data_quality)
-  [Function `sample_size`](#social_contracts_mydata_sample_size)
-  [Function `collection_method`](#social_contracts_mydata_collection_method)
-  [Function `is_updating`](#social_contracts_mydata_is_updating)
-  [Function `update_frequency`](#social_contracts_mydata_update_frequency)
-  [Function `purchaser_count`](#social_contracts_mydata_purchaser_count)
-  [Function `subscriber_count`](#social_contracts_mydata_subscriber_count)
-  [Function `is_one_time_for_sale`](#social_contracts_mydata_is_one_time_for_sale)
-  [Function `is_subscription_available`](#social_contracts_mydata_is_subscription_available)
-  [Function `has_active_subscription`](#social_contracts_mydata_has_active_subscription)
-  [Function `get_subscription_expiry`](#social_contracts_mydata_get_subscription_expiry)
-  [Function `get_revenue_potential`](#social_contracts_mydata_get_revenue_potential)
-  [Function `has_any_sales`](#social_contracts_mydata_has_any_sales)
-  [Function `registry_get_owner`](#social_contracts_mydata_registry_get_owner)
-  [Function `is_registered`](#social_contracts_mydata_is_registered)
-  [Function `register_in_registry`](#social_contracts_mydata_register_in_registry)
-  [Function `unregister_from_registry`](#social_contracts_mydata_unregister_from_registry)
-  [Function `version`](#social_contracts_mydata_version)
-  [Function `borrow_version_mut`](#social_contracts_mydata_borrow_version_mut)
-  [Function `registry_version`](#social_contracts_mydata_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_mydata_borrow_registry_version_mut)
-  [Function `migrate_mydata`](#social_contracts_mydata_migrate_mydata)
-  [Function `migrate_registry`](#social_contracts_mydata_migrate_registry)


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



<a name="social_contracts_mydata_MyData"></a>

## Struct `MyData`

Universal MyData for encrypted data monetization


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> <b>has</b> key, store
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Content metadata (title and description removed)
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64</code>
</dt>
<dd>
 Time and context
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_data: vector&lt;u8&gt;</code>
</dt>
<dd>
 Properly sealed content using MyData encryption
</dd>
<dt>
<code>encryption_id: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 Pricing options - user controlled
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>purchasers: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Access tracking
</dd>
<dt>
<code>subscribers: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Extended metadata for data discovery
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
 Version for future upgrades
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataRegistry"></a>

## Struct `MyDataRegistry`

Registry for tracking MyData ownership


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a> <b>has</b> key
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
<code>ip_to_owner: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataCreatedEvent"></a>

## Struct `MyDataCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataCreatedEvent">MyDataCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ip_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_PurchaseEvent"></a>

## Struct `PurchaseEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ip_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>buyer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>purchase_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="social_contracts_mydata_AccessGrantedEvent"></a>

## Struct `AccessGrantedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ip_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>access_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>granted_by: <b>address</b></code>
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

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_mydata_EActiveSubscription"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EActiveSubscription">EActiveSubscription</a>: u64 = 6;
</code></pre>



<a name="social_contracts_mydata_EAlreadyPurchased"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EAlreadyPurchased">EAlreadyPurchased</a>: u64 = 5;
</code></pre>



<a name="social_contracts_mydata_EInvalidInput"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>: u64 = 7;
</code></pre>



<a name="social_contracts_mydata_EInvalidTimeRange"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidTimeRange">EInvalidTimeRange</a>: u64 = 10;
</code></pre>



<a name="social_contracts_mydata_ENotForSale"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>: u64 = 2;
</code></pre>



<a name="social_contracts_mydata_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>: u64 = 9;
</code></pre>



<a name="social_contracts_mydata_EPriceMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>: u64 = 3;
</code></pre>



<a name="social_contracts_mydata_ESelfPurchase"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>: u64 = 4;
</code></pre>



<a name="social_contracts_mydata_ESubscriptionExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ESubscriptionExpired">ESubscriptionExpired</a>: u64 = 8;
</code></pre>



<a name="social_contracts_mydata_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="social_contracts_mydata_MAX_FREE_ACCESS_GRANTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_FREE_ACCESS_GRANTS">MAX_FREE_ACCESS_GRANTS</a>: u64 = 100000;
</code></pre>



<a name="social_contracts_mydata_MAX_SUBSCRIPTION_DAYS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>: u64 = 365;
</code></pre>



<a name="social_contracts_mydata_MAX_TAGS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_TAGS">MAX_TAGS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_mydata_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_mydata_MILLISECONDS_PER_DAY"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_mydata_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the MyData registry


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> registry = <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a> {
        id: object::new(ctx),
        ip_to_owner: table::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_mydata_create"></a>

## Function `create`

Create new MyData data with proper MyData encryption


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,  // Pre-encrypted data from client
    encryption_id: vector&lt;u8&gt;,   // <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> encryption ID
    <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> {
    // Input validation
    <b>assert</b>!(vector::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>) &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_TAGS">MAX_TAGS</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Validate prices with overflow protection
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>)) {
        <b>let</b> price_val = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>);
        <b>assert</b>!(price_val &gt; 0 && price_val &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>)) {
        <b>let</b> price_val = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>);
        <b>assert</b>!(price_val &gt; 0 && price_val &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    // Validate <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> duration with overflow protection
    <b>let</b> sub_duration = <b>if</b> (<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> == 0) { 30 } <b>else</b> { <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> };
    <b>assert</b>!(sub_duration &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Check <b>for</b> potential overflow in millisecond conversion
    <b>let</b> duration_ms = (sub_duration <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>assert</b>!(duration_ms &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    // Validate time range
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>)) {
        <b>let</b> end_time = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>);
        <b>assert</b>!(end_time &gt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidTimeRange">EInvalidTimeRange</a>);
    };
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> {
        id: object::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: tx_context::sender(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: current_time,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>: current_time,
        encrypted_data,
        encryption_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: sub_duration,
        purchasers: table::new(ctx),
        subscribers: table::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataCreatedEvent">MyDataCreatedEvent</a> {
        ip_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
    });
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_and_share"></a>

## Function `create_and_share`

Create and share MyData publicly


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share">create_and_share</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share">create_and_share</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,
    encryption_id: vector&lt;u8&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        encrypted_data,
        encryption_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        clock,
        ctx,
    );
    // Register in the registry
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    table::add(&<b>mut</b> registry.ip_to_owner, ip_id, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_one_time"></a>

## Function `purchase_one_time`

Purchase one-time access to MyData data


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time">purchase_one_time</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time">purchase_one_time</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    payment: Coin&lt;MYS&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> buyer = tx_context::sender(ctx);
    // Check <b>if</b> one-time purchase is available
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>);
    // Check payment amount
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    // Check <b>if</b> buyer already <b>has</b> access
    <b>assert</b>!(!table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, buyer), <a href="../social_contracts/mydata.md#social_contracts_mydata_EAlreadyPurchased">EAlreadyPurchased</a>);
    // Prevent self-purchase
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    // Handle payment
    transfer::public_transfer(payment, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    // Grant access
    table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, buyer, <b>true</b>);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"one_time"),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_subscription"></a>

## Function `purchase_subscription`

Purchase subscription access to MyData data


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription">purchase_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription">purchase_subscription</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    payment: Coin&lt;MYS&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> buyer = tx_context::sender(ctx);
    // Check <b>if</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> is available
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>);
    // Check payment amount
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    // Prevent self-purchase
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    // Validate <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> duration to prevent overflow
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Calculate <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> expiry safely with overflow protection
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> duration_ms = (<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
    // Ensure we don't overflow u64
    <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
    // Handle payment
    transfer::public_transfer(payment, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    // Grant/extend <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> access
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer)) {
        // Extend existing <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
        <b>let</b> current_expiry = table::remove(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer);
        <b>let</b> new_expiry = <b>if</b> (current_expiry &gt; current_time) {
            // Add to existing time, but check <b>for</b> overflow
            <b>let</b> extended_time = (current_expiry <b>as</b> u128) + duration_ms;
            <b>assert</b>!(extended_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
            extended_time <b>as</b> u64
        } <b>else</b> {
            expiry_time_u64
        };
        table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer, new_expiry);
    } <b>else</b> {
        // New <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
        table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer, expiry_time_u64);
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>"),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_pricing"></a>

## Function `update_pricing`

Update pricing (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_pricing">update_pricing</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, new_one_time_price: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, new_subscription_price: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, new_subscription_duration_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_pricing">update_pricing</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    new_one_time_price: Option&lt;u64&gt;,
    new_subscription_price: Option&lt;u64&gt;,
    new_subscription_duration_days: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    // Validate new prices
    <b>if</b> (option::is_some(&new_one_time_price)) {
        <b>let</b> price_val = *option::borrow(&new_one_time_price);
        <b>assert</b>!(price_val &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    <b>if</b> (option::is_some(&new_subscription_price)) {
        <b>let</b> price_val = *option::borrow(&new_subscription_price);
        <b>assert</b>!(price_val &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a> = new_one_time_price;
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a> = new_subscription_price;
    <b>if</b> (option::is_some(&new_subscription_duration_days)) {
        <b>let</b> duration = *option::borrow(&new_subscription_duration_days);
        <b>if</b> (duration &gt; 0) {
            <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> = duration;
        };
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        access_type: string::utf8(b"pricing_update"),
        granted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_content"></a>

## Function `update_content`

Update MyData content and metadata (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_content">update_content</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, new_encrypted_data: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, new_tags: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_content">update_content</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    new_encrypted_data: Option&lt;vector&lt;u8&gt;&gt;,
    new_tags: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>if</b> (option::is_some(&new_encrypted_data)) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encrypted_data = *option::borrow(&new_encrypted_data);
    };
    <b>if</b> (option::is_some(&new_tags)) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a> = *option::borrow(&new_tags);
    };
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a> = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        access_type: string::utf8(b"content_update"),
        granted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_has_access"></a>

## Function `has_access`

Check if user has access to MyData data


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, user: <b>address</b>, clock: &Clock): bool {
    // Owner always <b>has</b> access
    <b>if</b> (user == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>) <b>return</b> <b>true</b>;
    // Check one-time purchase
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user)) <b>return</b> <b>true</b>;
    // Check active <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
        <b>let</b> expiry = *table::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
        <b>let</b> current_time = clock::timestamp_ms(clock);
        <b>return</b> current_time &lt;= expiry
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_mydata_decrypt_data"></a>

## Function `decrypt_data`

Decrypt MyData data for authorized users


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_decrypt_data">decrypt_data</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, viewer: <b>address</b>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, pks: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_decrypt_data">decrypt_data</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    viewer: <b>address</b>,
    clock: &Clock,
    keys: &vector&lt;VerifiedDerivedKey&gt;,
    pks: &vector&lt;PublicKey&gt;,
): Option&lt;vector&lt;u8&gt;&gt; {
    // Only allow access <b>if</b> user <b>has</b> direct access to this <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>
    <b>if</b> (<a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, viewer, clock)) {
        <b>let</b> obj = bf_hmac_encryption::parse_encrypted_object(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encrypted_data);
        <b>return</b> bf_hmac_encryption::decrypt(&obj, keys, pks)
    };
    option::none()
}
</code></pre>



</details>

<a name="social_contracts_mydata_grant_access"></a>

## Function `grant_access`

Grant free access (owner only) - useful for samples or promotions


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_grant_access">grant_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, access_type: u8, subscription_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_grant_access">grant_access</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    user: <b>address</b>,
    access_type: u8, // 0 = one-time, 1 = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
    subscription_days: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(user != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>); // Owner doesn't need granted access
    <b>if</b> (access_type == 0) {
        // Grant one-time access
        <b>if</b> (!table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user)) {
            table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user, <b>true</b>);
        };
    } <b>else</b> {
        // Grant <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> access
        <b>let</b> duration_days = <b>if</b> (option::is_some(&subscription_days)) {
            <b>let</b> days = *option::borrow(&subscription_days);
            <b>assert</b>!(days &gt; 0 && days &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
            days
        } <b>else</b> {
            <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>
        };
        <b>let</b> current_time = clock::timestamp_ms(clock);
        <b>let</b> duration_ms = (duration_days <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
        <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
        // Ensure we don't overflow u64
        <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
        <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
        <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
            table::remove(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
        };
        table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user, expiry_time_u64);
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user,
        access_type: <b>if</b> (access_type == 0) { string::utf8(b"one_time") } <b>else</b> { string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>") },
        granted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_owner"></a>

## Function `owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): <b>address</b> { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_media_type"></a>

## Function `media_type`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): String { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_tags"></a>

## Function `tags`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): vector&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_platform_id"></a>

## Function `platform_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;<b>address</b>&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_one_time_price"></a>

## Function `one_time_price`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_subscription_price"></a>

## Function `subscription_price`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_subscription_duration_days"></a>

## Function `subscription_duration_days`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_created_at"></a>

## Function `created_at`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_last_updated"></a>

## Function `last_updated`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_timestamp_start"></a>

## Function `timestamp_start`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_timestamp_end"></a>

## Function `timestamp_end`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_geographic_region"></a>

## Function `geographic_region`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_data_quality"></a>

## Function `data_quality`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_sample_size"></a>

## Function `sample_size`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_collection_method"></a>

## Function `collection_method`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_is_updating"></a>

## Function `is_updating`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_update_frequency"></a>

## Function `update_frequency`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_purchaser_count"></a>

## Function `purchaser_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchaser_count">purchaser_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchaser_count">purchaser_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers) }
</code></pre>



</details>

<a name="social_contracts_mydata_subscriber_count"></a>

## Function `subscriber_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers) }
</code></pre>



</details>

<a name="social_contracts_mydata_is_one_time_for_sale"></a>

## Function `is_one_time_for_sale`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_one_time_for_sale">is_one_time_for_sale</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_one_time_for_sale">is_one_time_for_sale</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>) }
</code></pre>



</details>

<a name="social_contracts_mydata_is_subscription_available"></a>

## Function `is_subscription_available`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_subscription_available">is_subscription_available</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_subscription_available">is_subscription_available</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>) }
</code></pre>



</details>

<a name="social_contracts_mydata_has_active_subscription"></a>

## Function `has_active_subscription`

Check if a user has an active subscription


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_active_subscription">has_active_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_active_subscription">has_active_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, user: <b>address</b>, clock: &Clock): bool {
    <b>if</b> (!table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) <b>return</b> <b>false</b>;
    <b>let</b> expiry = *table::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    current_time &lt;= expiry
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_subscription_expiry"></a>

## Function `get_subscription_expiry`

Get subscription expiry time for a user


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_subscription_expiry">get_subscription_expiry</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_subscription_expiry">get_subscription_expiry</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, user: <b>address</b>): Option&lt;u64&gt; {
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
        option::some(*table::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_revenue_potential"></a>

## Function `get_revenue_potential`

Get total revenue potential (for analytics) with overflow protection


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_revenue_potential">get_revenue_potential</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_revenue_potential">get_revenue_potential</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    <b>let</b> one_time_revenue = <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>)) {
        <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>);
        <b>let</b> count = table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers);
        // Use u128 <b>for</b> calculation to detect overflow
        <b>let</b> revenue = (price <b>as</b> u128) * (count <b>as</b> u128);
        <b>if</b> (revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
            <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
        } <b>else</b> {
            revenue <b>as</b> u64
        }
    } <b>else</b> {
        0
    };
    <b>let</b> subscription_revenue = <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>)) {
        <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>);
        <b>let</b> count = table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers);
        // Use u128 <b>for</b> calculation to detect overflow
        <b>let</b> revenue = (price <b>as</b> u128) * (count <b>as</b> u128);
        <b>if</b> (revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
            <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
        } <b>else</b> {
            revenue <b>as</b> u64
        }
    } <b>else</b> {
        0
    };
    // Safe addition with overflow protection
    <b>let</b> total_revenue = (one_time_revenue <b>as</b> u128) + (subscription_revenue <b>as</b> u128);
    <b>if</b> (total_revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
    } <b>else</b> {
        total_revenue <b>as</b> u64
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_has_any_sales"></a>

## Function `has_any_sales`

Check if MyData has any sales (one-time or subscription)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_any_sales">has_any_sales</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_any_sales">has_any_sales</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool {
    table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers) &gt; 0 || table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers) &gt; 0
}
</code></pre>



</details>

<a name="social_contracts_mydata_registry_get_owner"></a>

## Function `registry_get_owner`

Get owner of a MyData by ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_get_owner">registry_get_owner</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, ip_id: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_get_owner">registry_get_owner</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>, ip_id: <b>address</b>): Option&lt;<b>address</b>&gt; {
    <b>if</b> (table::contains(&registry.ip_to_owner, ip_id)) {
        option::some(*table::borrow(&registry.ip_to_owner, ip_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_is_registered"></a>

## Function `is_registered`

Check if a MyData is registered


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_registered">is_registered</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, ip_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_registered">is_registered</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>, ip_id: <b>address</b>): bool {
    table::contains(&registry.ip_to_owner, ip_id)
}
</code></pre>



</details>

<a name="social_contracts_mydata_register_in_registry"></a>

## Function `register_in_registry`

Register a MyData in the registry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_register_in_registry">register_in_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_register_in_registry">register_in_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    <b>if</b> (!table::contains(&registry.ip_to_owner, ip_id)) {
        table::add(&<b>mut</b> registry.ip_to_owner, ip_id, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_unregister_from_registry"></a>

## Function `unregister_from_registry`

Remove a MyData from the registry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_unregister_from_registry">unregister_from_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, ip_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_unregister_from_registry">unregister_from_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    ip_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>if</b> (table::contains(&registry.ip_to_owner, ip_id)) {
        <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a> = *table::borrow(&registry.ip_to_owner, ip_id);
        <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
        table::remove(&<b>mut</b> registry.ip_to_owner, ip_id);
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_borrow_version_mut"></a>

## Function `borrow_version_mut`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_registry_version"></a>

## Function `registry_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_version">registry_version</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_version">registry_version</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>): u64 {
    registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_migrate_mydata"></a>

## Function `migrate_mydata`

Migration function for MyData


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_mydata">migrate_mydata</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_mydata">migrate_mydata</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> &lt; current_version, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> old_version = <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>;
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> = current_version;
    <b>let</b> mydata_id = object::id(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        mydata_id,
        string::utf8(b"<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_migrate_registry"></a>

## Function `migrate_registry`

Migration function for MyDataRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_registry">migrate_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> &lt; current_version, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> old_version = registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>;
    registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> = current_version;
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>
