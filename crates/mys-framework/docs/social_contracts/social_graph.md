---
title: Module `social_contracts::social_graph`
---

Social graph module for the MySocial network
Manages social relationships between users (following/followers)


-  [Struct `SocialGraph`](#social_contracts_social_graph_SocialGraph)
-  [Struct `FollowEvent`](#social_contracts_social_graph_FollowEvent)
-  [Struct `UnfollowEvent`](#social_contracts_social_graph_UnfollowEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_social_graph_bootstrap_init)
-  [Function `follow`](#social_contracts_social_graph_follow)
-  [Function `unfollow`](#social_contracts_social_graph_unfollow)
-  [Function `unfollow_internal`](#social_contracts_social_graph_unfollow_internal)
-  [Function `migrate_social_graph`](#social_contracts_social_graph_migrate_social_graph)
-  [Function `borrow_version_mut`](#social_contracts_social_graph_borrow_version_mut)
-  [Function `version`](#social_contracts_social_graph_version)
-  [Function `is_following`](#social_contracts_social_graph_is_following)
-  [Function `following_count`](#social_contracts_social_graph_following_count)
-  [Function `follower_count`](#social_contracts_social_graph_follower_count)
-  [Function `get_following`](#social_contracts_social_graph_get_following)
-  [Function `get_followers`](#social_contracts_social_graph_get_followers)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key">mys::bootstrap_key</a>;
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
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_social_graph_SocialGraph"></a>

## Struct `SocialGraph`

Global social graph object that tracks relationships between users


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a> <b>has</b> key
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
<code>following: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Table mapping profile IDs to sets of profiles they are following
</dd>
<dt>
<code>followers: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Table mapping profile IDs to sets of profiles following them
</dd>
<dt>
<code><a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>: u64</code>
</dt>
<dd>
 Current version of the object
</dd>
</dl>


</details>

<a name="social_contracts_social_graph_FollowEvent"></a>

## Struct `FollowEvent`

Follow event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_FollowEvent">FollowEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>follower: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>following: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_graph_UnfollowEvent"></a>

## Struct `UnfollowEvent`

Unfollow event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_UnfollowEvent">UnfollowEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>follower: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>unfollowed: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_social_graph_EAlreadyFollowing"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EAlreadyFollowing">EAlreadyFollowing</a>: u64 = 0;
</code></pre>



<a name="social_contracts_social_graph_ECannotFollowSelf"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ECannotFollowSelf">ECannotFollowSelf</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_graph_ENotFollowing"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_graph_EProfileNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EProfileNotFound">EProfileNotFound</a>: u64 = 3;
</code></pre>



<a name="social_contracts_social_graph_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>: u64 = 4;
</code></pre>



<a name="social_contracts_social_graph_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the social graph shared object


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a> = <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a> {
        id: object::new(ctx),
        following: table::new(ctx),
        followers: table::new(ctx),
        <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Share the social graph to make it globally accessible
    transfer::share_object(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_follow"></a>

## Function `follow`

Follow a profile by address


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, following_profile_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">profile::UsernameRegistry</a>,
    following_profile_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    // Look up the caller's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID from registry
    <b>let</b> <b>mut</b> caller_profile_id_opt = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(registry, sender);
    <b>assert</b>!(option::is_some(&caller_profile_id_opt), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EProfileNotFound">EProfileNotFound</a>);
    // Extract follower <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> follower_profile_id = option::extract(&<b>mut</b> caller_profile_id_opt);
    // Cannot <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a> self
    <b>assert</b>!(follower_profile_id != following_profile_id, <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ECannotFollowSelf">ECannotFollowSelf</a>);
    // Initialize follower's following set <b>if</b> it doesn't exist
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id)) {
        table::add(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id, vec_set::empty());
    };
    // Initialize followed's followers set <b>if</b> it doesn't exist
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id)) {
        table::add(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id, vec_set::empty());
    };
    // Get mutable references to the sets
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id);
    // Check <b>if</b> already following
    <b>assert</b>!(!vec_set::contains(follower_following, &following_profile_id), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EAlreadyFollowing">EAlreadyFollowing</a>);
    // Add to sets
    vec_set::insert(follower_following, following_profile_id);
    vec_set::insert(following_followers, follower_profile_id);
    // Emit <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a> event
    event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_FollowEvent">FollowEvent</a> {
        follower: follower_profile_id,
        following: following_profile_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unfollow"></a>

## Function `unfollow`

Unfollow a profile by address


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, following_profile_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">profile::UsernameRegistry</a>,
    following_profile_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    // Look up the caller's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID from registry
    <b>let</b> <b>mut</b> caller_profile_id_opt = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(registry, sender);
    <b>assert</b>!(option::is_some(&caller_profile_id_opt), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EProfileNotFound">EProfileNotFound</a>);
    // Extract follower <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> follower_profile_id = option::extract(&<b>mut</b> caller_profile_id_opt);
    // Check <b>if</b> following sets exist
    <b>assert</b>!(table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>);
    <b>assert</b>!(table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>);
    // Get mutable references to the sets
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id);
    // Check <b>if</b> following
    <b>assert</b>!(vec_set::contains(follower_following, &following_profile_id), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>);
    // Remove from sets
    vec_set::remove(follower_following, &following_profile_id);
    vec_set::remove(following_followers, &follower_profile_id);
    // Emit <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a> event
    event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_UnfollowEvent">UnfollowEvent</a> {
        follower: follower_profile_id,
        unfollowed: following_profile_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unfollow_internal"></a>

## Function `unfollow_internal`

Internal unfollow function that accepts explicit profile IDs
Used for bidirectional unfollow during blocking operations
Returns true if unfollow occurred, false if not following


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, follower_profile_id: <b>address</b>, following_profile_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    follower_profile_id: <b>address</b>,
    following_profile_id: <b>address</b>
): bool {
    // Check <b>if</b> following relationship exists
    <b>if</b> (!<a href="../social_contracts/social_graph.md#social_contracts_social_graph_is_following">is_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>, follower_profile_id, following_profile_id)) {
        <b>return</b> <b>false</b>  // Not following, nothing to do
    };
    // Check <b>if</b> following sets exist (defensive)
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id)) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id)) {
        <b>return</b> <b>false</b>
    };
    // Get mutable references to the sets
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_profile_id);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_profile_id);
    // Remove <b>if</b> present (defensive check)
    <b>if</b> (vec_set::contains(follower_following, &following_profile_id)) {
        vec_set::remove(follower_following, &following_profile_id);
        vec_set::remove(following_followers, &follower_profile_id);
        // Emit <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a> event
        event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_UnfollowEvent">UnfollowEvent</a> {
            follower: follower_profile_id,
            unfollowed: following_profile_id,
        });
        <b>return</b> <b>true</b>
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_social_graph_migrate_social_graph"></a>

## Function `migrate_social_graph`

Migrate the social graph to a new version
Only callable by the admin with the AdminCap


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_migrate_social_graph">migrate_social_graph</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_migrate_social_graph">migrate_social_graph</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">upgrade::UpgradeAdminCap</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> &gt; current <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>)
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> &lt; current_version, <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> and update to new <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>
    <b>let</b> old_version = <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>;
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> graph_id = object::id(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        graph_id,
        string::utf8(b"<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_social_graph_borrow_version_mut"></a>

## Function `borrow_version_mut`

Get a mutable reference to the version field (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_social_graph_version"></a>

## Function `version`

Get the version of the social graph


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>): u64 {
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_social_graph_is_following"></a>

## Function `is_following`

Check if a profile is following another profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_is_following">is_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, follower_id: <b>address</b>, following_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_is_following">is_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, follower_id: <b>address</b>, following_id: <b>address</b>): bool {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> follower_following = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_id);
    vec_set::contains(follower_following, &following_id)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_following_count"></a>

## Function `following_count`

Get the number of profiles a user is following


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_following_count">following_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, profile_id: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_following_count">following_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, profile_id: <b>address</b>): u64 {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, profile_id)) {
        <b>return</b> 0
    };
    <b>let</b> following = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, profile_id);
    vec_set::size(following)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_follower_count"></a>

## Function `follower_count`

Get the number of followers a profile has


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follower_count">follower_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, profile_id: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follower_count">follower_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, profile_id: <b>address</b>): u64 {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, profile_id)) {
        <b>return</b> 0
    };
    <b>let</b> followers = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, profile_id);
    vec_set::size(followers)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_get_following"></a>

## Function `get_following`

Get the list of profiles a user is following


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_following">get_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, profile_id: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_following">get_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, profile_id: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, profile_id)) {
        <b>return</b> vector::empty()
    };
    <b>let</b> following = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, profile_id);
    vec_set::into_keys(*following)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_get_followers"></a>

## Function `get_followers`

Get the list of followers for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_followers">get_followers</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, profile_id: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_followers">get_followers</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, profile_id: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, profile_id)) {
        <b>return</b> vector::empty()
    };
    <b>let</b> followers = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, profile_id);
    vec_set::into_keys(*followers)
}
</code></pre>



</details>
