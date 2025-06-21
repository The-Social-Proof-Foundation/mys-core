---
title: Module `social_contracts::subscription`
---

Subscription module for the MySocial network
Handles subscription services for profiles & MyIP


-  [Struct `ProfileSubscriptionService`](#social_contracts_subscription_ProfileSubscriptionService)
-  [Struct `ProfileSubscription`](#social_contracts_subscription_ProfileSubscription)
-  [Struct `ProfileSubscriptionCreatedEvent`](#social_contracts_subscription_ProfileSubscriptionCreatedEvent)
-  [Struct `ProfileSubscriptionRenewedEvent`](#social_contracts_subscription_ProfileSubscriptionRenewedEvent)
-  [Struct `ProfileSubscriptionCancelledEvent`](#social_contracts_subscription_ProfileSubscriptionCancelledEvent)
-  [Struct `ProfileSubscriptionUpdatedEvent`](#social_contracts_subscription_ProfileSubscriptionUpdatedEvent)
-  [Constants](#@Constants_0)
-  [Function `create_profile_service`](#social_contracts_subscription_create_profile_service)
-  [Function `create_profile_service_entry`](#social_contracts_subscription_create_profile_service_entry)
-  [Function `subscribe_to_profile`](#social_contracts_subscription_subscribe_to_profile)
-  [Function `renew_subscription`](#social_contracts_subscription_renew_subscription)
-  [Function `auto_renew_subscription`](#social_contracts_subscription_auto_renew_subscription)
-  [Function `can_auto_renew`](#social_contracts_subscription_can_auto_renew)
-  [Function `fund_renewal_balance`](#social_contracts_subscription_fund_renewal_balance)
-  [Function `is_subscription_valid`](#social_contracts_subscription_is_subscription_valid)
-  [Function `seal_approve`](#social_contracts_subscription_seal_approve)
-  [Function `update_service_fee`](#social_contracts_subscription_update_service_fee)
-  [Function `deactivate_service`](#social_contracts_subscription_deactivate_service)
-  [Function `cancel_subscription`](#social_contracts_subscription_cancel_subscription)
-  [Function `service_monthly_fee`](#social_contracts_subscription_service_monthly_fee)
-  [Function `service_subscriber_count`](#social_contracts_subscription_service_subscriber_count)
-  [Function `subscription_expires_at`](#social_contracts_subscription_subscription_expires_at)
-  [Function `subscription_auto_renew`](#social_contracts_subscription_subscription_auto_renew)
-  [Function `subscription_renewal_balance`](#social_contracts_subscription_subscription_renewal_balance)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/clock.md#mys_clock">mys::clock</a>;
<b>use</b> <a href="../mys/coin.md#mys_coin">mys::coin</a>;
<b>use</b> <a href="../mys/config.md#mys_config">mys::config</a>;
<b>use</b> <a href="../mys/deny_list.md#mys_deny_list">mys::deny_list</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/dynamic_object_field.md#mys_dynamic_object_field">mys::dynamic_object_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_subscription_ProfileSubscriptionService"></a>

## Struct `ProfileSubscriptionService`

Profile subscription service - one per profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a> <b>has</b> key
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
<code>profile_owner: <b>address</b></code>
</dt>
<dd>
 Profile owner who receives subscription fees
</dd>
<dt>
<code>monthly_fee: u64</code>
</dt>
<dd>
 Monthly subscription fee in MYS
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
 Whether this service allows new subscriptions
</dd>
<dt>
<code>subscriber_count: u64</code>
</dt>
<dd>
 Total number of active subscribers
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscription"></a>

## Struct `ProfileSubscription`

Individual subscription to a profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a> <b>has</b> key
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
<code>service_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
 The profile service this subscription is for
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
 Subscriber's address
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 When the subscription was created
</dd>
<dt>
<code>expires_at: u64</code>
</dt>
<dd>
 When the subscription expires (timestamp in ms)
</dd>
<dt>
<code>auto_renew: bool</code>
</dt>
<dd>
 Whether auto-renewal is enabled
</dd>
<dt>
<code>renewal_balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 Balance for auto-renewal payments
</dd>
<dt>
<code>renewal_count: u64</code>
</dt>
<dd>
 Number of times this subscription has been renewed
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionCreatedEvent"></a>

## Struct `ProfileSubscriptionCreatedEvent`

Events


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCreatedEvent">ProfileSubscriptionCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>service_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>auto_renew: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionRenewedEvent"></a>

## Struct `ProfileSubscriptionRenewedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionRenewedEvent">ProfileSubscriptionRenewedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>subscription_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_expires_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>renewal_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>auto_renewed: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionCancelledEvent"></a>

## Struct `ProfileSubscriptionCancelledEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>subscription_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionUpdatedEvent"></a>

## Struct `ProfileSubscriptionUpdatedEvent`

Additional event for fee updates


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionUpdatedEvent">ProfileSubscriptionUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>service_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>old_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_subscription_EAutoRenewalDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EAutoRenewalDisabled">EAutoRenewalDisabled</a>: u64 = 79;
</code></pre>



<a name="social_contracts_subscription_EInvalidFee"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>: u64 = 12;
</code></pre>



<a name="social_contracts_subscription_ENoAccess"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>: u64 = 77;
</code></pre>



<a name="social_contracts_subscription_ENotSubscriptionOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>: u64 = 80;
</code></pre>



<a name="social_contracts_subscription_ESubscriptionExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ESubscriptionExpired">ESubscriptionExpired</a>: u64 = 78;
</code></pre>



<a name="social_contracts_subscription_create_profile_service"></a>

## Function `create_profile_service`

Create a subscription service for a profile (called by profile owner)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service">create_profile_service</a>(profile_owner: <b>address</b>, monthly_fee: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service">create_profile_service</a>(
    profile_owner: <b>address</b>,
    monthly_fee: u64,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a> {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a> {
        id: object::new(ctx),
        profile_owner,
        monthly_fee,
        active: <b>true</b>,
        subscriber_count: 0,
        version: 1,
    }
}
</code></pre>



</details>

<a name="social_contracts_subscription_create_profile_service_entry"></a>

## Function `create_profile_service_entry`

Entry function to create and share a profile subscription service


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service_entry">create_profile_service_entry</a>(monthly_fee: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service_entry">create_profile_service_entry</a>(
    monthly_fee: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> service = <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service">create_profile_service</a>(
        tx_context::sender(ctx),
        monthly_fee,
        ctx
    );
    transfer::share_object(service);
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscribe_to_profile"></a>

## Function `subscribe_to_profile`

Subscribe to a profile with optional auto-renewal


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile">subscribe_to_profile</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, payment: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, auto_renew: bool, renewal_months: u64, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile">subscribe_to_profile</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    payment: &<b>mut</b> Coin&lt;MYS&gt;,
    auto_renew: bool,
    renewal_months: u64, // How many months to fund <b>for</b> auto-renewal (0 <b>if</b> not auto-renewing)
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.active, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    // Calculate required payment (1 month + renewal months <b>if</b> auto-renew)
    <b>let</b> months_to_pay = <b>if</b> (auto_renew) { 1 + renewal_months } <b>else</b> { 1 };
    <b>let</b> total_required = service.monthly_fee * months_to_pay;
    <b>assert</b>!(coin::value(payment) &gt;= total_required, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    // Take payment <b>for</b> first month
    <b>let</b> first_month_payment = coin::split(payment, service.monthly_fee, ctx);
    transfer::public_transfer(first_month_payment, service.profile_owner);
    // Take renewal payment <b>if</b> auto-renew enabled
    <b>let</b> renewal_balance = <b>if</b> (auto_renew && renewal_months &gt; 0) {
        <b>let</b> renewal_payment = coin::split(payment, service.monthly_fee * renewal_months, ctx);
        coin::into_balance(renewal_payment)
    } <b>else</b> {
        balance::zero&lt;MYS&gt;()
    };
    // Calculate expiration (30 days from now)
    <b>let</b> expires_at = now + (30 * 24 * 60 * 60 * 1000); // 30 days in milliseconds
    <b>let</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> = <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a> {
        id: object::new(ctx),
        service_id: object::id(service),
        subscriber,
        created_at: now,
        expires_at,
        auto_renew,
        renewal_balance,
        renewal_count: 0,
    };
    service.subscriber_count = service.subscriber_count + 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCreatedEvent">ProfileSubscriptionCreatedEvent</a> {
        service_id: object::id(service),
        subscriber,
        expires_at,
        monthly_fee: service.monthly_fee,
        auto_renew,
    });
    transfer::transfer(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>, subscriber);
}
</code></pre>



</details>

<a name="social_contracts_subscription_renew_subscription"></a>

## Function `renew_subscription`

Manually renew a subscription


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription">renew_subscription</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription">renew_subscription</a>(
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    payment: Coin&lt;MYS&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(coin::value(&payment) &gt;= service.monthly_fee, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    transfer::public_transfer(payment, service.profile_owner);
    // Extend expiration by 30 days
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> extension = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds
    // If <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> is expired, start from now, otherwise extend current expiration
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at = <b>if</b> (now &gt; <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at) {
        now + extension
    } <b>else</b> {
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at + extension
    };
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count + 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionRenewedEvent">ProfileSubscriptionRenewedEvent</a> {
        subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber,
        new_expires_at: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at,
        renewal_count: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count,
        auto_renewed: <b>false</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_auto_renew_subscription"></a>

## Function `auto_renew_subscription`

Gas-optimized auto-renew using pre-funded renewal balance
Now includes protection against fee changes and service deactivation


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription">auto_renew_subscription</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription">auto_renew_subscription</a>(
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew, <a href="../social_contracts/subscription.md#social_contracts_subscription_EAutoRenewalDisabled">EAutoRenewalDisabled</a>);
    // Check that the service is still active
    <b>assert</b>!(service.active, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    // Only allow auto-renewal <b>if</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> <b>has</b> actually expired
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &lt;= now, <a href="../social_contracts/subscription.md#social_contracts_subscription_ESubscriptionExpired">ESubscriptionExpired</a>);
    // Check <b>if</b> there's enough balance <b>for</b> renewal at current fee
    <b>let</b> renewal_balance_value = balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance);
    // Protection: If fee increased beyond what user <b>has</b> in renewal balance, cancel auto-renewal
    <b>if</b> (renewal_balance_value &lt; service.monthly_fee) {
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew = <b>false</b>;
        // Emit event indicating auto-renewal was cancelled due to insufficient funds/fee increase
        event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> {
            subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
            subscriber: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber,
            refunded_amount: 0, // No refund in this case
        });
        <b>return</b>
    };
    // Use renewal balance (gas optimized - avoid intermediate coin creation when possible)
    <b>let</b> renewal_payment = coin::from_balance(
        balance::split(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance, service.monthly_fee),
        ctx
    );
    transfer::public_transfer(renewal_payment, service.profile_owner);
    // Pre-calculate extension to avoid repeated calculations
    <b>let</b> extension = 2_592_000_000; // 30 days in milliseconds (pre-calculated)
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at = now + extension;
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count + 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionRenewedEvent">ProfileSubscriptionRenewedEvent</a> {
        subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber,
        new_expires_at: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at,
        renewal_count: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count,
        auto_renewed: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_can_auto_renew"></a>

## Function `can_auto_renew`

Check if subscription is eligible for auto-renewal without expensive operations
Now includes service activation check


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_can_auto_renew">can_auto_renew</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_can_auto_renew">can_auto_renew</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    clock: &Clock
): bool {
    <b>if</b> (!<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew) <b>return</b> <b>false</b>;
    <b>if</b> (<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id != object::id(service)) <b>return</b> <b>false</b>;
    <b>if</b> (!service.active) <b>return</b> <b>false</b>; // Check service is active
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &gt; now) <b>return</b> <b>false</b>;
    balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance) &gt;= service.monthly_fee
}
</code></pre>



</details>

<a name="social_contracts_subscription_fund_renewal_balance"></a>

## Function `fund_renewal_balance`

User funds their renewal balance for auto-renewal


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_fund_renewal_balance">fund_renewal_balance</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_fund_renewal_balance">fund_renewal_balance</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    payment: Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    balance::join(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance, coin::into_balance(payment));
}
</code></pre>



</details>

<a name="social_contracts_subscription_is_subscription_valid"></a>

## Function `is_subscription_valid`

Check if a subscription is valid for access


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">is_subscription_valid</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">is_subscription_valid</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    clock: &Clock,
): bool {
    <b>if</b> (object::id(service) != <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &gt; now
}
</code></pre>



</details>

<a name="social_contracts_subscription_seal_approve"></a>

## Function `seal_approve`

Seal integration for encrypted content access


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_seal_approve">seal_approve</a>(_id: vector&lt;u8&gt;, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_seal_approve">seal_approve</a>(
    _id: vector&lt;u8&gt;,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    clock: &Clock,
) {
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">is_subscription_valid</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>, service, clock), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
}
</code></pre>



</details>

<a name="social_contracts_subscription_update_service_fee"></a>

## Function `update_service_fee`

Update service fee (profile owner only)
Now emits event when fee changes


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_update_service_fee">update_service_fee</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, new_fee: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_update_service_fee">update_service_fee</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    new_fee: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == service.profile_owner, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>let</b> old_fee = service.monthly_fee;
    service.monthly_fee = new_fee;
    // Emit event about fee change
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionUpdatedEvent">ProfileSubscriptionUpdatedEvent</a> {
        service_id: object::id(service),
        old_fee,
        new_fee,
        updated_by: tx_context::sender(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_deactivate_service"></a>

## Function `deactivate_service`

Deactivate service (profile owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_deactivate_service">deactivate_service</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_deactivate_service">deactivate_service</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == service.profile_owner, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    service.active = <b>false</b>;
}
</code></pre>



</details>

<a name="social_contracts_subscription_cancel_subscription"></a>

## Function `cancel_subscription`

Cancel subscription and get refund of unused renewal balance


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_cancel_subscription">cancel_subscription</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_cancel_subscription">cancel_subscription</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    // Refund any remaining renewal balance
    <b>let</b> refund_amount = balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance);
    <b>if</b> (refund_amount &gt; 0) {
        <b>let</b> refund = coin::from_balance(
            balance::withdraw_all(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance),
            ctx
        );
        transfer::public_transfer(refund, subscriber);
    };
    service.subscriber_count = service.subscriber_count - 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> {
        subscription_id: object::id(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber,
        refunded_amount: refund_amount,
    });
    // Destroy the <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
    <b>let</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a> {
        id,
        service_id: _,
        subscriber: _,
        created_at: _,
        expires_at: _,
        auto_renew: _,
        renewal_balance,
        renewal_count: _,
    } = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>;
    balance::destroy_zero(renewal_balance);
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_subscription_service_monthly_fee"></a>

## Function `service_monthly_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_monthly_fee">service_monthly_fee</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_monthly_fee">service_monthly_fee</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>): u64 {
    service.monthly_fee
}
</code></pre>



</details>

<a name="social_contracts_subscription_service_subscriber_count"></a>

## Function `service_subscriber_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_subscriber_count">service_subscriber_count</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_subscriber_count">service_subscriber_count</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>): u64 {
    service.subscriber_count
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscription_expires_at"></a>

## Function `subscription_expires_at`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_expires_at">subscription_expires_at</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_expires_at">subscription_expires_at</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>): u64 {
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscription_auto_renew"></a>

## Function `subscription_auto_renew`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_auto_renew">subscription_auto_renew</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_auto_renew">subscription_auto_renew</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>): bool {
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscription_renewal_balance"></a>

## Function `subscription_renewal_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_renewal_balance">subscription_renewal_balance</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_renewal_balance">subscription_renewal_balance</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>): u64 {
    balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance)
}
</code></pre>



</details>
