---
title: Module `social_contracts::platform`
---

Platform module for the MySocial network
Manages social media platforms and their timelines


-  [Struct `PlatformStatus`](#social_contracts_platform_PlatformStatus)
-  [Struct `PlatformAdminCap`](#social_contracts_platform_PlatformAdminCap)
-  [Struct `Platform`](#social_contracts_platform_Platform)
-  [Struct `PlatformRegistry`](#social_contracts_platform_PlatformRegistry)
-  [Struct `PlatformCreatedEvent`](#social_contracts_platform_PlatformCreatedEvent)
-  [Struct `PlatformUpdatedEvent`](#social_contracts_platform_PlatformUpdatedEvent)
-  [Struct `PlatformBlockedProfileEvent`](#social_contracts_platform_PlatformBlockedProfileEvent)
-  [Struct `PlatformUnblockedProfileEvent`](#social_contracts_platform_PlatformUnblockedProfileEvent)
-  [Struct `ModeratorAddedEvent`](#social_contracts_platform_ModeratorAddedEvent)
-  [Struct `ModeratorRemovedEvent`](#social_contracts_platform_ModeratorRemovedEvent)
-  [Struct `PlatformApprovalChangedEvent`](#social_contracts_platform_PlatformApprovalChangedEvent)
-  [Struct `UserJoinedPlatformEvent`](#social_contracts_platform_UserJoinedPlatformEvent)
-  [Struct `UserLeftPlatformEvent`](#social_contracts_platform_UserLeftPlatformEvent)
-  [Struct `TokenAirdropEvent`](#social_contracts_platform_TokenAirdropEvent)
-  [Struct `TreasuryFundedEvent`](#social_contracts_platform_TreasuryFundedEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_platform_bootstrap_init)
-  [Function `create_platform`](#social_contracts_platform_create_platform)
-  [Function `update_platform`](#social_contracts_platform_update_platform)
-  [Function `platform_version`](#social_contracts_platform_platform_version)
-  [Function `borrow_platform_version_mut`](#social_contracts_platform_borrow_platform_version_mut)
-  [Function `registry_version`](#social_contracts_platform_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_platform_borrow_registry_version_mut)
-  [Function `add_to_treasury`](#social_contracts_platform_add_to_treasury)
-  [Function `add_moderator`](#social_contracts_platform_add_moderator)
-  [Function `remove_moderator`](#social_contracts_platform_remove_moderator)
-  [Function `block_profile`](#social_contracts_platform_block_profile)
-  [Function `unblock_profile`](#social_contracts_platform_unblock_profile)
-  [Function `toggle_platform_approval`](#social_contracts_platform_toggle_platform_approval)
-  [Function `new_status`](#social_contracts_platform_new_status)
-  [Function `status_value`](#social_contracts_platform_status_value)
-  [Function `join_platform`](#social_contracts_platform_join_platform)
-  [Function `leave_platform`](#social_contracts_platform_leave_platform)
-  [Function `is_approved`](#social_contracts_platform_is_approved)
-  [Function `has_joined_platform`](#social_contracts_platform_has_joined_platform)
-  [Function `is_developer_or_moderator`](#social_contracts_platform_is_developer_or_moderator)
-  [Function `name`](#social_contracts_platform_name)
-  [Function `tagline`](#social_contracts_platform_tagline)
-  [Function `description`](#social_contracts_platform_description)
-  [Function `logo`](#social_contracts_platform_logo)
-  [Function `developer`](#social_contracts_platform_developer)
-  [Function `terms_of_service`](#social_contracts_platform_terms_of_service)
-  [Function `privacy_policy`](#social_contracts_platform_privacy_policy)
-  [Function `get_platforms`](#social_contracts_platform_get_platforms)
-  [Function `get_links`](#social_contracts_platform_get_links)
-  [Function `status`](#social_contracts_platform_status)
-  [Function `release_date`](#social_contracts_platform_release_date)
-  [Function `shutdown_date`](#social_contracts_platform_shutdown_date)
-  [Function `created_at`](#social_contracts_platform_created_at)
-  [Function `treasury_balance`](#social_contracts_platform_treasury_balance)
-  [Function `id`](#social_contracts_platform_id)
-  [Function `is_moderator`](#social_contracts_platform_is_moderator)
-  [Function `get_moderators`](#social_contracts_platform_get_moderators)
-  [Function `get_platform_by_name`](#social_contracts_platform_get_platform_by_name)
-  [Function `get_platforms_by_developer`](#social_contracts_platform_get_platforms_by_developer)
-  [Function `is_profile_blocked`](#social_contracts_platform_is_profile_blocked)
-  [Function `is_profile_blocked_by_id`](#social_contracts_platform_is_profile_blocked_by_id)
-  [Function `get_blocked_profiles`](#social_contracts_platform_get_blocked_profiles)
-  [Function `wants_dao_governance`](#social_contracts_platform_wants_dao_governance)
-  [Function `governance_registry_id`](#social_contracts_platform_governance_registry_id)
-  [Function `governance_parameters`](#social_contracts_platform_governance_parameters)
-  [Function `airdrop_from_treasury`](#social_contracts_platform_airdrop_from_treasury)
-  [Function `assign_badge`](#social_contracts_platform_assign_badge)
-  [Function `revoke_badge`](#social_contracts_platform_revoke_badge)
-  [Function `add_moderator_register`](#social_contracts_platform_add_moderator_register)
-  [Function `remove_moderator_unregister`](#social_contracts_platform_remove_moderator_unregister)
-  [Function `create_platform_admin_cap`](#social_contracts_platform_create_platform_admin_cap)


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
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
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



<a name="social_contracts_platform_PlatformStatus"></a>

## Struct `PlatformStatus`

Platform status enum


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformAdminCap"></a>

## Struct `PlatformAdminCap`

Admin capability for Platform system management


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_Platform"></a>

## Struct `Platform`

Platform object that contains information about a social media platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform name
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform tagline
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform description
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform logo URL
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b></code>
</dt>
<dd>
 Platform developer address
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform terms of service URL
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform privacy policy URL
</dd>
<dt>
<code>platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Platform names
</dd>
<dt>
<code>links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Platform URLs
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a></code>
</dt>
<dd>
 Platform status
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform release date
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Platform shutdown date (optional)
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code>treasury: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 Platform-specific MYS tokens treasury
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool</code>
</dt>
<dd>
 Whether the platform wants DAO governance
</dd>
<dt>
<code>delegate_count: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 DAO governance configuration parameters (all optional)
</dd>
<dt>
<code>delegate_term_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_submission_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>min_on_chain_age_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;</code>
</dt>
<dd>
 ID of governance registry if created
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformRegistry"></a>

## Struct `PlatformRegistry`

Platform registry that keeps track of all platforms


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platforms_by_name: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <b>address</b>&gt;</code>
</dt>
<dd>
 Table mapping platform names to platform IDs
</dd>
<dt>
<code>platforms_by_developer: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Table mapping developer addresses to their platforms
</dd>
<dt>
<code>platform_approvals: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Table mapping platform IDs to their approval status (admin-controlled)
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformCreatedEvent"></a>

## Struct `PlatformCreatedEvent`

Platform created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformCreatedEvent">PlatformCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformUpdatedEvent"></a>

## Struct `PlatformUpdatedEvent`

Platform updated event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformUpdatedEvent">PlatformUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformBlockedProfileEvent"></a>

## Struct `PlatformBlockedProfileEvent`

Profile blocked by platform event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockedProfileEvent">PlatformBlockedProfileEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>blocked_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformUnblockedProfileEvent"></a>

## Struct `PlatformUnblockedProfileEvent`

Profile unblocked by platform event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformUnblockedProfileEvent">PlatformUnblockedProfileEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>unblocked_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_ModeratorAddedEvent"></a>

## Struct `ModeratorAddedEvent`

Moderator added event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorAddedEvent">ModeratorAddedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>moderator_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>added_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_ModeratorRemovedEvent"></a>

## Struct `ModeratorRemovedEvent`

Moderator removed event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorRemovedEvent">ModeratorRemovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>moderator_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>removed_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformApprovalChangedEvent"></a>

## Struct `PlatformApprovalChangedEvent`

Platform approval status changed event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformApprovalChangedEvent">PlatformApprovalChangedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>approved: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>changed_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_UserJoinedPlatformEvent"></a>

## Struct `UserJoinedPlatformEvent`

Event emitted when a user joins a platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_UserJoinedPlatformEvent">UserJoinedPlatformEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
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

<a name="social_contracts_platform_UserLeftPlatformEvent"></a>

## Struct `UserLeftPlatformEvent`

Event emitted when a user leaves a platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_UserLeftPlatformEvent">UserLeftPlatformEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
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

<a name="social_contracts_platform_TokenAirdropEvent"></a>

## Struct `TokenAirdropEvent`

Event emitted when tokens are airdropped from the platform treasury


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_TokenAirdropEvent">TokenAirdropEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>executed_by: <b>address</b></code>
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

<a name="social_contracts_platform_TreasuryFundedEvent"></a>

## Struct `TreasuryFundedEvent`

Event emitted when tokens are added to platform treasury


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_TreasuryFundedEvent">TreasuryFundedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>funded_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
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


<a name="social_contracts_platform_BLOCKED_PROFILES_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>: vector&lt;u8&gt; = vector[98, 108, 111, 99, 107, 101, 100, 95, 112, 114, 111, 102, 105, 108, 101, 115];
</code></pre>



<a name="social_contracts_platform_EAlreadyBlocked"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EAlreadyBlocked">EAlreadyBlocked</a>: u64 = 2;
</code></pre>



<a name="social_contracts_platform_EAlreadyJoined"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EAlreadyJoined">EAlreadyJoined</a>: u64 = 5;
</code></pre>



<a name="social_contracts_platform_EBadgeDescriptionTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeDescriptionTooLong">EBadgeDescriptionTooLong</a>: u64 = 12;
</code></pre>



<a name="social_contracts_platform_EBadgeImageUrlTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeImageUrlTooLong">EBadgeImageUrlTooLong</a>: u64 = 13;
</code></pre>



<a name="social_contracts_platform_EBadgeNameTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeNameTooLong">EBadgeNameTooLong</a>: u64 = 11;
</code></pre>



<a name="social_contracts_platform_EEmptyRecipientsList"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EEmptyRecipientsList">EEmptyRecipientsList</a>: u64 = 9;
</code></pre>



<a name="social_contracts_platform_EInsufficientTreasuryFunds"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInsufficientTreasuryFunds">EInsufficientTreasuryFunds</a>: u64 = 8;
</code></pre>



<a name="social_contracts_platform_EInvalidBadgeType"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidBadgeType">EInvalidBadgeType</a>: u64 = 10;
</code></pre>



<a name="social_contracts_platform_EInvalidReasoning"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidReasoning">EInvalidReasoning</a>: u64 = 14;
</code></pre>



<a name="social_contracts_platform_EInvalidTokenAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidTokenAmount">EInvalidTokenAmount</a>: u64 = 4;
</code></pre>



<a name="social_contracts_platform_ENotBlocked"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_ENotBlocked">ENotBlocked</a>: u64 = 3;
</code></pre>



<a name="social_contracts_platform_ENotJoined"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_ENotJoined">ENotJoined</a>: u64 = 6;
</code></pre>



<a name="social_contracts_platform_EPlatformAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EPlatformAlreadyExists">EPlatformAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="social_contracts_platform_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_platform_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>: u64 = 7;
</code></pre>



<a name="social_contracts_platform_JOINED_PROFILES_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>: vector&lt;u8&gt; = vector[106, 111, 105, 110, 101, 100, 95, 112, 114, 111, 102, 105, 108, 101, 115];
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_DESCRIPTION_LENGTH">MAX_BADGE_DESCRIPTION_LENGTH</a>: u64 = 500;
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_IMAGE_URL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_IMAGE_URL_LENGTH">MAX_BADGE_IMAGE_URL_LENGTH</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_NAME_LENGTH"></a>

Maximum lengths for badge fields


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_NAME_LENGTH">MAX_BADGE_NAME_LENGTH</a>: u64 = 100;
</code></pre>



<a name="social_contracts_platform_MAX_REASONING_LENGTH"></a>

Maximum length for approval reasoning


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>: u64 = 2000;
</code></pre>



<a name="social_contracts_platform_MODERATORS_FIELD"></a>

Field names for dynamic fields


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>: vector&lt;u8&gt; = vector[109, 111, 100, 101, 114, 97, 116, 111, 114, 115];
</code></pre>



<a name="social_contracts_platform_STATUS_ALPHA"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_ALPHA">STATUS_ALPHA</a>: u8 = 1;
</code></pre>



<a name="social_contracts_platform_STATUS_BETA"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_BETA">STATUS_BETA</a>: u8 = 2;
</code></pre>



<a name="social_contracts_platform_STATUS_DEVELOPMENT"></a>

Platform status constants


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_DEVELOPMENT">STATUS_DEVELOPMENT</a>: u8 = 0;
</code></pre>



<a name="social_contracts_platform_STATUS_LIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_LIVE">STATUS_LIVE</a>: u8 = 3;
</code></pre>



<a name="social_contracts_platform_STATUS_MAINTENANCE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_MAINTENANCE">STATUS_MAINTENANCE</a>: u8 = 4;
</code></pre>



<a name="social_contracts_platform_STATUS_SHUTDOWN"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SHUTDOWN">STATUS_SHUTDOWN</a>: u8 = 6;
</code></pre>



<a name="social_contracts_platform_STATUS_SUNSET"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SUNSET">STATUS_SUNSET</a>: u8 = 5;
</code></pre>



<a name="social_contracts_platform_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the platform registry


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> registry = <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a> {
        <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: object::new(ctx),
        platforms_by_name: table::new(ctx),
        platforms_by_developer: table::new(ctx),
        platform_approvals: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_platform_create_platform"></a>

## Function `create_platform`

Create a new platform and transfer to developer


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform">create_platform</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, logo_url: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8, <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool, delegate_count: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, delegate_term_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, proposal_submission_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, min_on_chain_age_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, max_votes_per_user: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, quadratic_base_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, voting_period_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, quorum_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform">create_platform</a>(
    registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: String,
    logo_url: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: String,
    platforms: vector&lt;String&gt;,
    links: vector&lt;String&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8,
    <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool,
    delegate_count: Option&lt;u64&gt;,
    delegate_term_epochs: Option&lt;u64&gt;,
    proposal_submission_cost: Option&lt;u64&gt;,
    min_on_chain_age_days: Option&lt;u64&gt;,
    max_votes_per_user: Option&lt;u64&gt;,
    quadratic_base_cost: Option&lt;u64&gt;,
    voting_period_epochs: Option&lt;u64&gt;,
    quorum_votes: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> platform_id = object::new(ctx);
    <b>let</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> = tx_context::sender(ctx);
    <b>let</b> now = tx_context::epoch(ctx);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a> is already taken
    <b>assert</b>!(!table::contains(&registry.platforms_by_name, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>), <a href="../social_contracts/platform.md#social_contracts_platform_EPlatformAlreadyExists">EPlatformAlreadyExists</a>);
    // Validate <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> code is one of the defined constants
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_DEVELOPMENT">STATUS_DEVELOPMENT</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_ALPHA">STATUS_ALPHA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_BETA">STATUS_BETA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_LIVE">STATUS_LIVE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_MAINTENANCE">STATUS_MAINTENANCE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SUNSET">STATUS_SUNSET</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SHUTDOWN">STATUS_SHUTDOWN</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>
    );
    // If DAO <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> is not wanted, set all <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> parameters to None
    <b>let</b> actual_delegate_count = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) delegate_count <b>else</b> option::none();
    <b>let</b> actual_delegate_term_epochs = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) delegate_term_epochs <b>else</b> option::none();
    <b>let</b> actual_proposal_submission_cost = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) proposal_submission_cost <b>else</b> option::none();
    <b>let</b> actual_min_on_chain_age_days = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) min_on_chain_age_days <b>else</b> option::none();
    <b>let</b> actual_max_votes_per_user = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) max_votes_per_user <b>else</b> option::none();
    <b>let</b> actual_quadratic_base_cost = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) quadratic_base_cost <b>else</b> option::none();
    <b>let</b> actual_voting_period_epochs = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) voting_period_epochs <b>else</b> option::none();
    <b>let</b> actual_quorum_votes = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) quorum_votes <b>else</b> option::none();
    <b>let</b> <b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> = <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a> {
        <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: platform_id,
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: logo_url,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>,
        platforms,
        links,
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: option::none(),
        <a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>: now,
        treasury: balance::zero(),
        <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>,
        delegate_count: actual_delegate_count,
        delegate_term_epochs: actual_delegate_term_epochs,
        proposal_submission_cost: actual_proposal_submission_cost,
        min_on_chain_age_days: actual_min_on_chain_age_days,
        max_votes_per_user: actual_max_votes_per_user,
        quadratic_base_cost: actual_quadratic_base_cost,
        voting_period_epochs: actual_voting_period_epochs,
        quorum_votes: actual_quorum_votes,
        <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>: option::none(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Create empty moderators set
    <b>let</b> <b>mut</b> moderators = vec_set::empty&lt;<b>address</b>&gt;();
    // Add <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> <b>as</b> a moderator
    vec_set::insert(&<b>mut</b> moderators, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>);
    // Add moderators <b>as</b> a dynamic field
    dynamic_field::add(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>, moderators);
    // Register <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> in registry
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    // Add to platforms by <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>
    table::add(&<b>mut</b> registry.platforms_by_name, *&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>, platform_id);
    // Add to platforms by <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>if</b> (!table::contains(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)) {
        table::add(&<b>mut</b> registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, vector::empty&lt;<b>address</b>&gt;());
    };
    <b>let</b> developer_platforms = table::borrow_mut(&<b>mut</b> registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>);
    vector::push_back(developer_platforms, platform_id);
    // Add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> approvals (starts <b>as</b> not approved)
    table::add(&<b>mut</b> registry.platform_approvals, platform_id, <b>false</b>);
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> created event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformCreatedEvent">PlatformCreatedEvent</a> {
        platform_id,
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>,
        platforms: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms,
        links: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links,
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>,
    });
    // If <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> wants DAO <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>, create <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> registry immediately
    <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) {
        // Use default values <b>if</b> options are None
        <b>let</b> delegate_count = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count)
        } <b>else</b> {
            7 // Default value
        };
        <b>let</b> delegate_term_epochs = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs)
        } <b>else</b> {
            30 // Default value
        };
        <b>let</b> proposal_submission_cost = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost)
        } <b>else</b> {
            50_000_000 // Default value
        };
        <b>let</b> min_on_chain_age_days = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.min_on_chain_age_days)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.min_on_chain_age_days)
        } <b>else</b> {
            7 // Default value
        };
        <b>let</b> max_votes_per_user = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user)
        } <b>else</b> {
            5 // Default value
        };
        <b>let</b> quadratic_base_cost = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost)
        } <b>else</b> {
            5_000_000 // Default value
        };
        <b>let</b> voting_period_epochs = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs)
        } <b>else</b> {
            3 // Default value
        };
        <b>let</b> quorum_votes = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes)
        } <b>else</b> {
            15 // Default value
        };
        // Create <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> registry <b>for</b> this <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
        <b>let</b> registry_id = <a href="../social_contracts/governance.md#social_contracts_governance_create_platform_governance">governance::create_platform_governance</a>(
            delegate_count,
            delegate_term_epochs,
            proposal_submission_cost,
            min_on_chain_age_days,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_epochs,
            quorum_votes,
            ctx
        );
        // Store registry ID in the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a> = option::some(registry_id);
    };
    // Share <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>as</b> a shared object (publicly accessible)
    transfer::share_object(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
}
</code></pre>



</details>

<a name="social_contracts_platform_update_platform"></a>

## Function `update_platform`

Update platform information


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_update_platform">update_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, new_name: <a href="../std/string.md#std_string_String">std::string::String</a>, new_tagline: <a href="../std/string.md#std_string_String">std::string::String</a>, new_description: <a href="../std/string.md#std_string_String">std::string::String</a>, new_logo_url: <a href="../std/string.md#std_string_String">std::string::String</a>, new_terms_of_service: <a href="../std/string.md#std_string_String">std::string::String</a>, new_privacy_policy: <a href="../std/string.md#std_string_String">std::string::String</a>, new_platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, new_links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>: u8, new_release_date: <a href="../std/string.md#std_string_String">std::string::String</a>, new_shutdown_date: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_update_platform">update_platform</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    new_name: String,
    new_tagline: String,
    new_description: String,
    new_logo_url: String,
    new_terms_of_service: String,
    new_privacy_policy: String,
    new_platforms: vector&lt;String&gt;,
    new_links: vector&lt;String&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>: u8,
    new_release_date: String,
    new_shutdown_date: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == tx_context::sender(ctx), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Update <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> information
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a> = new_name;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a> = new_tagline;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a> = new_description;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a> = new_logo_url;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a> = new_terms_of_service;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a> = new_privacy_policy;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms = new_platforms;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links = new_links;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> = <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>);
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a> = new_release_date;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a> = new_shutdown_date;
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> updated event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformUpdatedEvent">PlatformUpdatedEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>,
        platforms: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms,
        links: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links,
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>,
        updated_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_platform_version"></a>

## Function `platform_version`

Get the version of a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u64 {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version
}
</code></pre>



</details>

<a name="social_contracts_platform_borrow_platform_version_mut"></a>

## Function `borrow_platform_version_mut`

Get a mutable reference to the platform version (only for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_platform_version_mut">borrow_platform_version_mut</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_platform_version_mut">borrow_platform_version_mut</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version
}
</code></pre>



</details>

<a name="social_contracts_platform_registry_version"></a>

## Function `registry_version`

Get the version of the platform registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_registry_version">registry_version</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_registry_version">registry_version</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>): u64 {
    registry.version
}
</code></pre>



</details>

<a name="social_contracts_platform_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version (only for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.version
}
</code></pre>



</details>

<a name="social_contracts_platform_add_to_treasury"></a>

## Function `add_to_treasury`

Add MYS tokens to platform treasury


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Check amount validity
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidTokenAmount">EInvalidTokenAmount</a>);
    // Split coin and add to treasury
    <b>let</b> treasury_coin = coin::split(coin, amount, ctx);
    balance::join(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury, coin::into_balance(treasury_coin));
    // Emit treasury funded event
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>let</b> new_balance = balance::value(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury);
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_TreasuryFundedEvent">TreasuryFundedEvent</a> {
        platform_id,
        amount,
        funded_by: caller,
        new_balance,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_add_moderator"></a>

## Function `add_moderator`

Add a moderator to a platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator">add_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, moderator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator">add_moderator</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    moderator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == caller, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Get moderators set
    <b>let</b> moderators = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    // Add moderator <b>if</b> not already a moderator
    <b>if</b> (!vec_set::contains(moderators, &moderator_address)) {
        vec_set::insert(moderators, moderator_address);
        // Emit moderator added event
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorAddedEvent">ModeratorAddedEvent</a> {
            platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
            moderator_address,
            added_by: caller,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_remove_moderator"></a>

## Function `remove_moderator`

Remove a moderator from a platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator">remove_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, moderator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator">remove_moderator</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    moderator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == caller, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Cannot remove <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> <b>as</b> moderator
    <b>assert</b>!(moderator_address != <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Get moderators set
    <b>let</b> moderators = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    // Remove moderator <b>if</b> they are a moderator
    <b>if</b> (vec_set::contains(moderators, &moderator_address)) {
        vec_set::remove(moderators, &moderator_address);
        // Emit moderator removed event
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorRemovedEvent">ModeratorRemovedEvent</a> {
            platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
            moderator_address,
            removed_by: caller,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_block_profile"></a>

## Function `block_profile`

Block a profile from the platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_block_profile">block_profile</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, profile_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_block_profile">block_profile</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    profile_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Create blocked profiles set <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>)) {
        <b>let</b> blocked_profiles = vec_set::empty&lt;<b>address</b>&gt;();
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>, blocked_profiles);
    };
    // Get blocked profiles set
    <b>let</b> blocked_profiles = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>);
    // Check <b>if</b> already blocked and <b>abort</b> <b>if</b> <b>true</b>
    <b>assert</b>!(!vec_set::contains(blocked_profiles, &profile_id), <a href="../social_contracts/platform.md#social_contracts_platform_EAlreadyBlocked">EAlreadyBlocked</a>);
    // Add <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> to blocked set
    vec_set::insert(blocked_profiles, profile_id);
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>-specific block event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockedProfileEvent">PlatformBlockedProfileEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        profile_id,
        blocked_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_unblock_profile"></a>

## Function `unblock_profile`

Unblock a profile from the platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_unblock_profile">unblock_profile</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, profile_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_unblock_profile">unblock_profile</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    profile_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> blocked profiles set exists
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>)) {
        // Profile can't be blocked <b>if</b> there's no blocked profiles set
        <b>abort</b> <a href="../social_contracts/platform.md#social_contracts_platform_ENotBlocked">ENotBlocked</a>
    };
    // Get blocked profiles set
    <b>let</b> blocked_profiles = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>);
    // Check <b>if</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> is actually blocked and <b>abort</b> <b>if</b> not
    <b>assert</b>!(vec_set::contains(blocked_profiles, &profile_id), <a href="../social_contracts/platform.md#social_contracts_platform_ENotBlocked">ENotBlocked</a>);
    // Remove <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> from blocked set
    vec_set::remove(blocked_profiles, &profile_id);
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>-specific unblock event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformUnblockedProfileEvent">PlatformUnblockedProfileEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        profile_id,
        unblocked_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_toggle_platform_approval"></a>

## Function `toggle_platform_approval`

Toggle platform approval status (requires PlatformAdminCap only)
Optional reasoning can be provided to explain the decision


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_toggle_platform_approval">toggle_platform_approval</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, platform_id: <b>address</b>, _: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">social_contracts::platform::PlatformAdminCap</a>, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_toggle_platform_approval">toggle_platform_approval</a>(
    registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    platform_id: <b>address</b>,
    _: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a>,
    reasoning: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Admin capability verification is handled by type system
    // Verify the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> exists in the registry
    <b>assert</b>!(table::contains(&registry.platform_approvals, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Validate reasoning length <b>if</b> provided
    <b>if</b> (option::is_some(&reasoning)) {
        <b>let</b> reasoning_val = option::borrow(&reasoning);
        <b>assert</b>!(string::length(reasoning_val) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidReasoning">EInvalidReasoning</a>);
    };
    // Get current approval <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> and toggle it
    <b>let</b> current_approval = *table::borrow(&registry.platform_approvals, platform_id);
    <b>let</b> new_approval = !current_approval;
    // Update the approval <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> in the registry
    *table::borrow_mut(&<b>mut</b> registry.platform_approvals, platform_id) = new_approval;
    // Emit approval <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> changed event with reasoning
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformApprovalChangedEvent">PlatformApprovalChangedEvent</a> {
        platform_id,
        approved: new_approval,
        changed_by: tx_context::sender(ctx),
        reasoning,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_new_status"></a>

## Function `new_status`

Create a new platform status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a> {
    // Validate the <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> code is one of the defined constants
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_DEVELOPMENT">STATUS_DEVELOPMENT</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_ALPHA">STATUS_ALPHA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_BETA">STATUS_BETA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_LIVE">STATUS_LIVE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_MAINTENANCE">STATUS_MAINTENANCE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SUNSET">STATUS_SUNSET</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SHUTDOWN">STATUS_SHUTDOWN</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>
    );
    <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a> { <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> }
}
</code></pre>



</details>

<a name="social_contracts_platform_status_value"></a>

## Function `status_value`

Get the status value


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status_value">status_value</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status_value">status_value</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a>): u8 {
    <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_join_platform"></a>

## Function `join_platform`

Join a platform - establishes initial connection between profile and platform
Checks for blocks before allowing the join and verifies platform is approved
Uses the caller's wallet address to find their profile for security


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_join_platform">join_platform</a>(profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_join_platform">join_platform</a>(
    profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">profile::UsernameRegistry</a>,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> platform_id = object::id(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Look up the caller's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID from registry
    <b>let</b> <b>mut</b> caller_profile_id_opt = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, caller);
    <b>assert</b>!(option::is_some(&caller_profile_id_opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Extract <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID and convert to ID type
    <b>let</b> profile_id_addr = option::extract(&<b>mut</b> caller_profile_id_opt);
    <b>let</b> profile_id = object::id_from_address(profile_id_addr);
    // Check <b>if</b> the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>has</b> blocked this <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(!<a href="../social_contracts/platform.md#social_contracts_platform_is_profile_blocked">is_profile_blocked</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, profile_id_addr), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved by the contract owner (<b>use</b> registry)
    <b>let</b> platform_id_addr = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(platform_registry, platform_id_addr), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Create joined profiles set <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>)) {
        <b>let</b> joined_profiles = vec_set::empty&lt;ID&gt;();
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>, joined_profiles);
    };
    // Get joined profiles set
    <b>let</b> joined_profiles = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;ID&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>);
    // Check <b>if</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> is already joined to the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>assert</b>!(!vec_set::contains(joined_profiles, &profile_id), <a href="../social_contracts/platform.md#social_contracts_platform_EAlreadyJoined">EAlreadyJoined</a>);
    // Add <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> to joined profiles
    vec_set::insert(joined_profiles, profile_id);
    // Emit event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_UserJoinedPlatformEvent">UserJoinedPlatformEvent</a> {
        profile_id,
        platform_id,
        user: caller,
        timestamp: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_leave_platform"></a>

## Function `leave_platform`

Leave a platform - removes the connection between profile and platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_leave_platform">leave_platform</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_leave_platform">leave_platform</a>(
    registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">profile::UsernameRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> platform_id = object::id(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Look up the caller's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID from registry
    <b>let</b> <b>mut</b> caller_profile_id_opt = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(registry, caller);
    <b>assert</b>!(option::is_some(&caller_profile_id_opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Extract <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID and convert to ID type
    <b>let</b> profile_id_addr = option::extract(&<b>mut</b> caller_profile_id_opt);
    <b>let</b> profile_id = object::id_from_address(profile_id_addr);
    // Check <b>if</b> joined profiles set exists
    <b>assert</b>!(dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>), <a href="../social_contracts/platform.md#social_contracts_platform_ENotJoined">ENotJoined</a>);
    // Get joined profiles set
    <b>let</b> joined_profiles = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;ID&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>);
    // Check <b>if</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> is a member of the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>assert</b>!(vec_set::contains(joined_profiles, &profile_id), <a href="../social_contracts/platform.md#social_contracts_platform_ENotJoined">ENotJoined</a>);
    // Remove <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> from joined profiles
    vec_set::remove(joined_profiles, &profile_id);
    // Emit event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_UserLeftPlatformEvent">UserLeftPlatformEvent</a> {
        profile_id,
        platform_id,
        user: caller,
        timestamp: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_is_approved"></a>

## Function `is_approved`

Get platform approval status from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, platform_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>, platform_id: <b>address</b>): bool {
    <b>if</b> (!table::contains(&registry.platform_approvals, platform_id)) {
        <b>return</b> <b>false</b>
    };
    *table::borrow(&registry.platform_approvals, platform_id)
}
</code></pre>



</details>

<a name="social_contracts_platform_has_joined_platform"></a>

## Function `has_joined_platform`

Check if a profile has joined a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>, profile_id: ID): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> joined_profiles = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;ID&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_PROFILES_FIELD">JOINED_PROFILES_FIELD</a>);
    vec_set::contains(joined_profiles, &profile_id)
}
</code></pre>



</details>

<a name="social_contracts_platform_is_developer_or_moderator"></a>

## Function `is_developer_or_moderator`

Check if an address is the platform developer or a moderator


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>, addr: <b>address</b>): bool {
    <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == addr) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> moderators = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    vec_set::contains(moderators, &addr)
}
</code></pre>



</details>

<a name="social_contracts_platform_name"></a>

## Function `name`

Get platform name


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_tagline"></a>

## Function `tagline`

Get platform tagline


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_description"></a>

## Function `description`

Get platform description


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_logo"></a>

## Function `logo`

Get platform logo URL


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &String {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_developer"></a>

## Function `developer`

Get platform developer


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): <b>address</b> {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_terms_of_service"></a>

## Function `terms_of_service`

Get platform terms of service


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_privacy_policy"></a>

## Function `privacy_policy`

Get platform privacy policy


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_get_platforms"></a>

## Function `get_platforms`

Get platform platforms


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms">get_platforms</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms">get_platforms</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &vector&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms
}
</code></pre>



</details>

<a name="social_contracts_platform_get_links"></a>

## Function `get_links`

Get platform links


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_links">get_links</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_links">get_links</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &vector&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links
}
</code></pre>



</details>

<a name="social_contracts_platform_status"></a>

## Function `status`

Get platform status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u8 {
    <a href="../social_contracts/platform.md#social_contracts_platform_status_value">status_value</a>(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>)
}
</code></pre>



</details>

<a name="social_contracts_platform_release_date"></a>

## Function `release_date`

Get platform release date


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_shutdown_date"></a>

## Function `shutdown_date`

Get platform shutdown date


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_created_at"></a>

## Function `created_at`

Get platform creation timestamp


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u64 {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_treasury_balance"></a>

## Function `treasury_balance`

Get platform treasury balance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_treasury_balance">treasury_balance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_treasury_balance">treasury_balance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u64 {
    balance::value(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury)
}
</code></pre>



</details>

<a name="social_contracts_platform_id"></a>

## Function `id`

Get platform ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &UID {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_is_moderator"></a>

## Function `is_moderator`

Check if an address is a moderator


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_moderator">is_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_moderator">is_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>, addr: <b>address</b>): bool {
    <b>let</b> moderators = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    vec_set::contains(moderators, &addr)
}
</code></pre>



</details>

<a name="social_contracts_platform_get_moderators"></a>

## Function `get_moderators`

Get the list of moderators for a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_moderators">get_moderators</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_moderators">get_moderators</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): vector&lt;<b>address</b>&gt; {
    <b>let</b> moderators = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    vec_set::into_keys(*moderators)
}
</code></pre>



</details>

<a name="social_contracts_platform_get_platform_by_name"></a>

## Function `get_platform_by_name`

Get platform by name from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platform_by_name">get_platform_by_name</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platform_by_name">get_platform_by_name</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: String): Option&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&registry.platforms_by_name, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>)) {
        <b>return</b> option::none()
    };
    option::some(*table::borrow(&registry.platforms_by_name, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>))
}
</code></pre>



</details>

<a name="social_contracts_platform_get_platforms_by_developer"></a>

## Function `get_platforms_by_developer`

Get platforms owned by a developer


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms_by_developer">get_platforms_by_developer</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms_by_developer">get_platforms_by_developer</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)) {
        <b>return</b> vector::empty()
    };
    *table::borrow(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)
}
</code></pre>



</details>

<a name="social_contracts_platform_is_profile_blocked"></a>

## Function `is_profile_blocked`

Check if a profile is blocked in a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_profile_blocked">is_profile_blocked</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, profile_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_profile_blocked">is_profile_blocked</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>, profile_id: <b>address</b>): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> blocked_profiles = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>);
    vec_set::contains(blocked_profiles, &profile_id)
}
</code></pre>



</details>

<a name="social_contracts_platform_is_profile_blocked_by_id"></a>

## Function `is_profile_blocked_by_id`

Check if a profile is blocked in a platform by ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_profile_blocked_by_id">is_profile_blocked_by_id</a>(_platform_id: <b>address</b>, _profile_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_profile_blocked_by_id">is_profile_blocked_by_id</a>(_platform_id: <b>address</b>, _profile_id: <b>address</b>): bool {
    <b>false</b> // Placeholder implementation (would need to borrow object by ID)
}
</code></pre>



</details>

<a name="social_contracts_platform_get_blocked_profiles"></a>

## Function `get_blocked_profiles`

Get list of blocked profiles for a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_blocked_profiles">get_blocked_profiles</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_blocked_profiles">get_blocked_profiles</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>)) {
        <b>return</b> vector::empty()
    };
    <b>let</b> blocked_profiles = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_BLOCKED_PROFILES_FIELD">BLOCKED_PROFILES_FIELD</a>);
    vec_set::into_keys(*blocked_profiles)
}
</code></pre>



</details>

<a name="social_contracts_platform_wants_dao_governance"></a>

## Function `wants_dao_governance`

Check if platform wants DAO governance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): bool {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_governance_registry_id"></a>

## Function `governance_registry_id`

Get platform's governance registry ID if available


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;ID&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_governance_parameters"></a>

## Function `governance_parameters`

Get platform's governance parameters


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_parameters">governance_parameters</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): (<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_parameters">governance_parameters</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): (Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;) {
    (
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.min_on_chain_age_days,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes
    )
}
</code></pre>



</details>

<a name="social_contracts_platform_airdrop_from_treasury"></a>

## Function `airdrop_from_treasury`

Airdrop tokens to multiple recipients from the platform treasury
Can only be called by platform developer or moderator


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_airdrop_from_treasury">airdrop_from_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, recipients: vector&lt;<b>address</b>&gt;, amount_per_recipient: u64, reason_code: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_airdrop_from_treasury">airdrop_from_treasury</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    recipients: vector&lt;<b>address</b>&gt;,
    amount_per_recipient: u64,
    reason_code: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> or moderator
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Check that recipients list is not empty
    <b>let</b> recipients_count = vector::length(&recipients);
    <b>assert</b>!(recipients_count &gt; 0, <a href="../social_contracts/platform.md#social_contracts_platform_EEmptyRecipientsList">EEmptyRecipientsList</a>);
    // Calculate total amount needed
    <b>let</b> total_amount = amount_per_recipient * recipients_count;
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury <b>has</b> enough funds
    <b>assert</b>!(balance::value(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury) &gt;= total_amount, <a href="../social_contracts/platform.md#social_contracts_platform_EInsufficientTreasuryFunds">EInsufficientTreasuryFunds</a>);
    // Get current timestamp <b>for</b> events
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    // Send tokens to each recipient
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; recipients_count) {
        <b>let</b> recipient = *vector::borrow(&recipients, i);
        // Create coin from <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury balance
        <b>let</b> airdrop_coin = coin::from_balance(
            balance::split(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury, amount_per_recipient),
            ctx
        );
        // Transfer to recipient
        transfer::public_transfer(airdrop_coin, recipient);
        // Emit airdrop event <b>for</b> tracking
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_TokenAirdropEvent">TokenAirdropEvent</a> {
            platform_id,
            recipient,
            amount: amount_per_recipient,
            reason_code,
            executed_by: caller,
            timestamp: current_time,
        });
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_assign_badge"></a>

## Function `assign_badge`

Assign a badge to a profile - can only be called by platform admin/moderator
This is the primary entry point for badge assignment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_assign_badge">assign_badge</a>(platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_name: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_description: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_type: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_assign_badge">assign_badge</a>(
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">profile::Profile</a>,
    badge_name: String,
    badge_description: String,
    badge_image_url: String,
    badge_type: u8,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> admin or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Validate badge type (1-100 <b>as</b> documented)
    <b>assert</b>!(badge_type &gt;= 1 && badge_type &lt;= 100, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidBadgeType">EInvalidBadgeType</a>);
    // Validate badge field lengths
    <b>assert</b>!(string::length(&badge_name) &gt; 0 && string::length(&badge_name) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_NAME_LENGTH">MAX_BADGE_NAME_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeNameTooLong">EBadgeNameTooLong</a>);
    <b>assert</b>!(string::length(&badge_description) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_DESCRIPTION_LENGTH">MAX_BADGE_DESCRIPTION_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeDescriptionTooLong">EBadgeDescriptionTooLong</a>);
    <b>assert</b>!(string::length(&badge_image_url) &gt; 0 && string::length(&badge_image_url) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_IMAGE_URL_LENGTH">MAX_BADGE_IMAGE_URL_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeImageUrlTooLong">EBadgeImageUrlTooLong</a>);
    // Get current time
    <b>let</b> now = tx_context::epoch(ctx);
    // Create a unique badge ID by including <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> ID to prevent collisions
    <b>let</b> <b>mut</b> badge_id = string::utf8(b"badge_");
    // Convert <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> ID to hex string and append to ensure uniqueness
    <b>let</b> platform_id_str = <a href="../mys/address.md#mys_address_to_string">mys::address::to_string</a>(platform_id);
    string::append(&<b>mut</b> badge_id, platform_id_str);
    string::append(&<b>mut</b> badge_id, string::utf8(b"_"));
    string::append(&<b>mut</b> badge_id, badge_name);
    // Add the badge directly to the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <a href="../social_contracts/profile.md#social_contracts_profile_add_badge_to_profile">profile::add_badge_to_profile</a>(
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>,
        badge_id,
        badge_name,
        badge_description,
        badge_image_url,
        platform_id,
        now,
        caller,
        badge_type
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_revoke_badge"></a>

## Function `revoke_badge`

Revoke a badge from a profile - can only be called by platform admin/moderator
This is the primary entry point for badge revocation


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_revoke_badge">revoke_badge</a>(platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_id: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_revoke_badge">revoke_badge</a>(
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">profile::Profile</a>,
    badge_id: String,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> admin or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Get current time
    <b>let</b> now = tx_context::epoch(ctx);
    // Remove the badge directly from the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <a href="../social_contracts/profile.md#social_contracts_profile_remove_badge_from_profile">profile::remove_badge_from_profile</a>(
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>,
        &badge_id,
        platform_id,
        caller,
        now
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_add_moderator_register"></a>

## Function `add_moderator_register`

When adding a moderator to a platform, register them with the profile module


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator_register">add_moderator_register</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, moderator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator_register">add_moderator_register</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    moderator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == caller, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Get moderators set
    <b>let</b> moderators = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    // Add moderator <b>if</b> not already a moderator
    <b>if</b> (!vec_set::contains(moderators, &moderator_address)) {
        vec_set::insert(moderators, moderator_address);
        // Emit moderator added event
        <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorAddedEvent">ModeratorAddedEvent</a> {
            platform_id,
            moderator_address,
            added_by: caller,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_remove_moderator_unregister"></a>

## Function `remove_moderator_unregister`

When removing a moderator from a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator_unregister">remove_moderator_unregister</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, moderator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator_unregister">remove_moderator_unregister</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    moderator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == caller, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Cannot remove <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> <b>as</b> moderator
    <b>assert</b>!(moderator_address != <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Get moderators set
    <b>let</b> moderators = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_MODERATORS_FIELD">MODERATORS_FIELD</a>);
    // Remove moderator <b>if</b> they are a moderator
    <b>if</b> (vec_set::contains(moderators, &moderator_address)) {
        vec_set::remove(moderators, &moderator_address);
        // Emit moderator removed event
        <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorRemovedEvent">ModeratorRemovedEvent</a> {
            platform_id,
            moderator_address,
            removed_by: caller,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_create_platform_admin_cap"></a>

## Function `create_platform_admin_cap`

Create a PlatformAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">create_platform_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">social_contracts::platform::PlatformAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">create_platform_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a> {
    <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a> {
        <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: object::new(ctx)
    }
}
</code></pre>



</details>
