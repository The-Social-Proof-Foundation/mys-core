---
title: Module `social_contracts::profile`
---

Profile module for the MySocial network
Handles user identity, profile creation, management, and username registration


-  [Struct `EcosystemTreasury`](#social_contracts_profile_EcosystemTreasury)
-  [Struct `UsernameRegistry`](#social_contracts_profile_UsernameRegistry)
-  [Struct `Profile`](#social_contracts_profile_Profile)
-  [Struct `ProfileBadge`](#social_contracts_profile_ProfileBadge)
-  [Struct `VestingWallet`](#social_contracts_profile_VestingWallet)
-  [Struct `BadgeAssignedEvent`](#social_contracts_profile_BadgeAssignedEvent)
-  [Struct `BadgeRevokedEvent`](#social_contracts_profile_BadgeRevokedEvent)
-  [Struct `ProfileCreatedEvent`](#social_contracts_profile_ProfileCreatedEvent)
-  [Struct `ProfileUpdatedEvent`](#social_contracts_profile_ProfileUpdatedEvent)
-  [Struct `ProfileOfferCreatedEvent`](#social_contracts_profile_ProfileOfferCreatedEvent)
-  [Struct `ProfileOfferAcceptedEvent`](#social_contracts_profile_ProfileOfferAcceptedEvent)
-  [Struct `ProfileOfferRejectedEvent`](#social_contracts_profile_ProfileOfferRejectedEvent)
-  [Struct `ProfileOffer`](#social_contracts_profile_ProfileOffer)
-  [Struct `ProfileSaleFeeEvent`](#social_contracts_profile_ProfileSaleFeeEvent)
-  [Struct `TokensVestedEvent`](#social_contracts_profile_TokensVestedEvent)
-  [Struct `TokensClaimedEvent`](#social_contracts_profile_TokensClaimedEvent)
-  [Constants](#@Constants_0)
-  [Function `init`](#social_contracts_profile_init)
-  [Function `is_reserved_name`](#social_contracts_profile_is_reserved_name)
-  [Function `to_lowercase_bytes`](#social_contracts_profile_to_lowercase_bytes)
-  [Function `to_lowercase_byte`](#social_contracts_profile_to_lowercase_byte)
-  [Function `ascii_to_string`](#social_contracts_profile_ascii_to_string)
-  [Function `create_profile`](#social_contracts_profile_create_profile)
-  [Function `transfer_profile`](#social_contracts_profile_transfer_profile)
-  [Function `update_profile`](#social_contracts_profile_update_profile)
-  [Function `display_name`](#social_contracts_profile_display_name)
-  [Function `bio`](#social_contracts_profile_bio)
-  [Function `profile_picture`](#social_contracts_profile_profile_picture)
-  [Function `cover_photo`](#social_contracts_profile_cover_photo)
-  [Function `owner`](#social_contracts_profile_owner)
-  [Function `id`](#social_contracts_profile_id)
-  [Function `last_updated`](#social_contracts_profile_last_updated)
-  [Function `username`](#social_contracts_profile_username)
-  [Function `lookup_profile_by_username`](#social_contracts_profile_lookup_profile_by_username)
-  [Function `lookup_profile_by_owner`](#social_contracts_profile_lookup_profile_by_owner)
-  [Function `is_authorized_service`](#social_contracts_profile_is_authorized_service)
-  [Function `authorize_service`](#social_contracts_profile_authorize_service)
-  [Function `revoke_authorization`](#social_contracts_profile_revoke_authorization)
-  [Function `get_id_address`](#social_contracts_profile_get_id_address)
-  [Function `get_owner`](#social_contracts_profile_get_owner)
-  [Function `get_followers_count`](#social_contracts_profile_get_followers_count)
-  [Function `get_post_count`](#social_contracts_profile_get_post_count)
-  [Function `get_tips_received`](#social_contracts_profile_get_tips_received)
-  [Function `increment_followers_count`](#social_contracts_profile_increment_followers_count)
-  [Function `decrement_followers_count`](#social_contracts_profile_decrement_followers_count)
-  [Function `increment_post_count`](#social_contracts_profile_increment_post_count)
-  [Function `decrement_post_count`](#social_contracts_profile_decrement_post_count)
-  [Function `add_tips_received`](#social_contracts_profile_add_tips_received)
-  [Function `get_following_count`](#social_contracts_profile_get_following_count)
-  [Function `increment_following_count`](#social_contracts_profile_increment_following_count)
-  [Function `decrement_following_count`](#social_contracts_profile_decrement_following_count)
-  [Function `create_subscription_service`](#social_contracts_profile_create_subscription_service)
-  [Function `has_valid_subscription`](#social_contracts_profile_has_valid_subscription)
-  [Function `attach_my_ip`](#social_contracts_profile_attach_my_ip)
-  [Function `has_my_ip_attached`](#social_contracts_profile_has_my_ip_attached)
-  [Function `detach_my_ip`](#social_contracts_profile_detach_my_ip)
-  [Function `get_attached_my_ips`](#social_contracts_profile_get_attached_my_ips)
-  [Function `batch_attach_my_ips`](#social_contracts_profile_batch_attach_my_ips)
-  [Function `batch_detach_my_ips`](#social_contracts_profile_batch_detach_my_ips)
-  [Function `create_offer`](#social_contracts_profile_create_offer)
-  [Function `accept_offer`](#social_contracts_profile_accept_offer)
-  [Function `reject_or_revoke_offer`](#social_contracts_profile_reject_or_revoke_offer)
-  [Function `has_offer_from`](#social_contracts_profile_has_offer_from)
-  [Function `has_offers`](#social_contracts_profile_has_offers)
-  [Function `get_treasury_address`](#social_contracts_profile_get_treasury_address)
-  [Function `version`](#social_contracts_profile_version)
-  [Function `borrow_version_mut`](#social_contracts_profile_borrow_version_mut)
-  [Function `migrate_registry`](#social_contracts_profile_migrate_registry)
-  [Function `min_offer_amount`](#social_contracts_profile_min_offer_amount)
-  [Function `is_for_sale`](#social_contracts_profile_is_for_sale)
-  [Function `add_badge_to_profile`](#social_contracts_profile_add_badge_to_profile)
-  [Function `remove_badge_from_profile`](#social_contracts_profile_remove_badge_from_profile)
-  [Function `get_profile_badges`](#social_contracts_profile_get_profile_badges)
-  [Function `has_badge`](#social_contracts_profile_has_badge)
-  [Function `get_badge`](#social_contracts_profile_get_badge)
-  [Function `get_platform_badges`](#social_contracts_profile_get_platform_badges)
-  [Function `badge_count`](#social_contracts_profile_badge_count)
-  [Function `vest_myso`](#social_contracts_profile_vest_myso)
-  [Function `claim_vested_tokens`](#social_contracts_profile_claim_vested_tokens)
-  [Function `claimable`](#social_contracts_profile_claimable)
-  [Function `calculate_claimable`](#social_contracts_profile_calculate_claimable)
-  [Function `sqrt_approximation`](#social_contracts_profile_sqrt_approximation)
-  [Function `delete_vesting_wallet`](#social_contracts_profile_delete_vesting_wallet)
-  [Function `vesting_balance`](#social_contracts_profile_vesting_balance)
-  [Function `vesting_owner`](#social_contracts_profile_vesting_owner)
-  [Function `vesting_start_time`](#social_contracts_profile_vesting_start_time)
-  [Function `vesting_duration`](#social_contracts_profile_vesting_duration)
-  [Function `vesting_total_amount`](#social_contracts_profile_vesting_total_amount)
-  [Function `vesting_claimed_amount`](#social_contracts_profile_vesting_claimed_amount)
-  [Function `vesting_curve_factor`](#social_contracts_profile_vesting_curve_factor)


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
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
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



<a name="social_contracts_profile_EcosystemTreasury"></a>

## Struct `EcosystemTreasury`

Social Ecosystem Treasury that receives fees from profile sales


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">EcosystemTreasury</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_address: <b>address</b></code>
</dt>
<dd>
 Treasury address that receives fees
</dd>
</dl>


</details>

<a name="social_contracts_profile_UsernameRegistry"></a>

## Struct `UsernameRegistry`

Username Registry that stores mappings between usernames and profiles


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>usernames: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>address_profiles: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_Profile"></a>

## Struct `Profile`

Profile object that contains user information


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Display name of the profile (optional)
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Bio of the profile
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;</code>
</dt>
<dd>
 Profile picture URL
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;</code>
</dt>
<dd>
 Cover photo URL
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
 Profile owner address
</dd>
<dt>
<code>x_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 X/Twitter username as encrypted string (optional)
</dd>
<dt>
<code>mastodon_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Mastodon username as encrypted string (optional)
</dd>
<dt>
<code>facebook_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Facebook username as encrypted string (optional)
</dd>
<dt>
<code>reddit_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Reddit username as encrypted string (optional)
</dd>
<dt>
<code>github_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 GitHub username as encrypted string (optional)
</dd>
<dt>
<code>instagram_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Instagram username as encrypted string (optional)
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_last_updated">last_updated</a>: u64</code>
</dt>
<dd>
 Last updated timestamp for profile data
</dd>
<dt>
<code>followers_count: u64</code>
</dt>
<dd>
 Number of followers
</dd>
<dt>
<code>following_count: u64</code>
</dt>
<dd>
 Number of profiles this user is following
</dd>
<dt>
<code>post_count: u64</code>
</dt>
<dd>
 Number of posts created by this profile
</dd>
<dt>
<code>tips_received: u64</code>
</dt>
<dd>
 Total amount of tips received
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 Minimum offer amount in MYSO tokens the owner is willing to accept (optional)
</dd>
<dt>
<code>badges: vector&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">social_contracts::profile::ProfileBadge</a>&gt;</code>
</dt>
<dd>
 Collection of badges assigned to the profile
</dd>
<dt>
<code>attached_my_ip_ids: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Vector tracking attached MyIP IDs for efficient iteration
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileBadge"></a>

## Struct `ProfileBadge`

Profile Badge that can be assigned to profiles by platform admins/moderators
These badges cannot be transferred or sold and stay with the profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>badge_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Unique identifier for the badge (platform ID + badge name)
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Name of the badge
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Description of what the badge represents
</dd>
<dt>
<code>image_url: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Image URL for the badge
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
 ID of the platform that issued the badge
</dd>
<dt>
<code>issued_at: u64</code>
</dt>
<dd>
 Timestamp when the badge was issued
</dd>
<dt>
<code>issued_by: <b>address</b></code>
</dt>
<dd>
 Address of the admin/moderator who issued the badge
</dd>
<dt>
<code>badge_type: u8</code>
</dt>
<dd>
 Badge type/tier (1-100), allows for badge hierarchy
</dd>
</dl>


</details>

<a name="social_contracts_profile_VestingWallet"></a>

## Struct `VestingWallet`

Vesting Wallet contains MYS coins that are available for claiming over time


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 Balance of MYS coins remaining in the wallet
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
 Address of the wallet owner who can claim the tokens
</dd>
<dt>
<code>start_time: u64</code>
</dt>
<dd>
 Time when the vesting started (in milliseconds)
</dd>
<dt>
<code>claimed_amount: u64</code>
</dt>
<dd>
 Amount of coins that have been claimed
</dd>
<dt>
<code>duration: u64</code>
</dt>
<dd>
 Total duration of the vesting schedule (in milliseconds)
</dd>
<dt>
<code>total_amount: u64</code>
</dt>
<dd>
 Total amount originally vested
</dd>
<dt>
<code>curve_factor: u64</code>
</dt>
<dd>
 Curve factor (1000 = linear, >1000 = more at end, <1000 = more at start)
</dd>
</dl>


</details>

<a name="social_contracts_profile_BadgeAssignedEvent"></a>

## Struct `BadgeAssignedEvent`

Event emitted when a badge is assigned to a profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_BadgeAssignedEvent">BadgeAssignedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 ID of the profile receiving the badge
</dd>
<dt>
<code>badge_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Badge identifier
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Badge name
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
 Platform ID that issued the badge
</dd>
<dt>
<code>assigned_by: <b>address</b></code>
</dt>
<dd>
 Admin/moderator who assigned the badge
</dd>
<dt>
<code>assigned_at: u64</code>
</dt>
<dd>
 Timestamp when assigned
</dd>
<dt>
<code>badge_type: u8</code>
</dt>
<dd>
 Badge type/tier
</dd>
</dl>


</details>

<a name="social_contracts_profile_BadgeRevokedEvent"></a>

## Struct `BadgeRevokedEvent`

Event emitted when a badge is revoked from a profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_BadgeRevokedEvent">BadgeRevokedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 ID of the profile losing the badge
</dd>
<dt>
<code>badge_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Badge identifier
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
 Platform ID that issued the badge
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
</dt>
<dd>
 Admin/moderator who revoked the badge
</dd>
<dt>
<code>revoked_at: u64</code>
</dt>
<dd>
 Timestamp when revoked
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileCreatedEvent"></a>

## Struct `ProfileCreatedEvent`

Profile created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileCreatedEvent">ProfileCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileUpdatedEvent"></a>

## Struct `ProfileUpdatedEvent`

Profile updated event with all profile details


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileUpdatedEvent">ProfileUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>x_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>mastodon_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>facebook_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>reddit_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>github_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>instagram_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileOfferCreatedEvent"></a>

## Struct `ProfileOfferCreatedEvent`

Event emitted when an offer is created for a profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOfferCreatedEvent">ProfileOfferCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>offeror: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileOfferAcceptedEvent"></a>

## Struct `ProfileOfferAcceptedEvent`

Event emitted when an offer is accepted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOfferAcceptedEvent">ProfileOfferAcceptedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>offeror: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>previous_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accepted_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileOfferRejectedEvent"></a>

## Struct `ProfileOfferRejectedEvent`

Event emitted when an offer is rejected or revoked


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOfferRejectedEvent">ProfileOfferRejectedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>offeror: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>rejected_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rejected_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>is_revoked: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileOffer"></a>

## Struct `ProfileOffer`

Represents an offer to purchase a profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>offeror: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>locked_myso: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_ProfileSaleFeeEvent"></a>

## Struct `ProfileSaleFeeEvent`

Event emitted when a fee is collected from a profile sale


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileSaleFeeEvent">ProfileSaleFeeEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>offeror: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>previous_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>sale_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_recipient: <b>address</b></code>
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

<a name="social_contracts_profile_TokensVestedEvent"></a>

## Struct `TokensVestedEvent`

Event emitted when MYS tokens are vested


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_TokensVestedEvent">TokensVestedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>wallet_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_amount: u64</code>
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
<dt>
<code>curve_factor: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>vested_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_profile_TokensClaimedEvent"></a>

## Struct `TokensClaimedEvent`

Event emitted when vested tokens are claimed


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/profile.md#social_contracts_profile_TokensClaimedEvent">TokensClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>wallet_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>remaining_balance: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_profile_CURVE_PRECISION"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_profile_EBadgeAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EBadgeAlreadyExists">EBadgeAlreadyExists</a>: u64 = 14;
</code></pre>



<a name="social_contracts_profile_EBadgeNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EBadgeNotFound">EBadgeNotFound</a>: u64 = 13;
</code></pre>



<a name="social_contracts_profile_ECannotOfferOwnProfile"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_ECannotOfferOwnProfile">ECannotOfferOwnProfile</a>: u64 = 9;
</code></pre>



<a name="social_contracts_profile_EInsufficientTokens"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EInsufficientTokens">EInsufficientTokens</a>: u64 = 10;
</code></pre>



<a name="social_contracts_profile_EInvalidStartTime"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EInvalidStartTime">EInvalidStartTime</a>: u64 = 15;
</code></pre>



<a name="social_contracts_profile_EInvalidUsername"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EInvalidUsername">EInvalidUsername</a>: u64 = 2;
</code></pre>



<a name="social_contracts_profile_ENotAuthorizedService"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_ENotAuthorizedService">ENotAuthorizedService</a>: u64 = 6;
</code></pre>



<a name="social_contracts_profile_ENotVestingWalletOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_ENotVestingWalletOwner">ENotVestingWalletOwner</a>: u64 = 16;
</code></pre>



<a name="social_contracts_profile_EOfferAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EOfferAlreadyExists">EOfferAlreadyExists</a>: u64 = 7;
</code></pre>



<a name="social_contracts_profile_EOfferBelowMinimum"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EOfferBelowMinimum">EOfferBelowMinimum</a>: u64 = 12;
</code></pre>



<a name="social_contracts_profile_EOfferDoesNotExist"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EOfferDoesNotExist">EOfferDoesNotExist</a>: u64 = 8;
</code></pre>



<a name="social_contracts_profile_EProfileAlreadyExists"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EProfileAlreadyExists">EProfileAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="social_contracts_profile_EProfileCreateFailed"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EProfileCreateFailed">EProfileCreateFailed</a>: u64 = 3;
</code></pre>



<a name="social_contracts_profile_EReservedName"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EReservedName">EReservedName</a>: u64 = 4;
</code></pre>



<a name="social_contracts_profile_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="social_contracts_profile_EUnauthorizedOfferAction"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorizedOfferAction">EUnauthorizedOfferAction</a>: u64 = 11;
</code></pre>



<a name="social_contracts_profile_EUsernameNotAvailable"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_EUsernameNotAvailable">EUsernameNotAvailable</a>: u64 = 5;
</code></pre>



<a name="social_contracts_profile_MY_IP_DATA_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>: vector&lt;u8&gt; = vector[109, 121, 95, 105, 112, 95, 100, 97, 116, 97];
</code></pre>



<a name="social_contracts_profile_OFFERS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>: vector&lt;u8&gt; = vector[112, 114, 111, 102, 105, 108, 101, 95, 111, 102, 102, 101, 114, 115];
</code></pre>



<a name="social_contracts_profile_PROFILE_SALE_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_PROFILE_SALE_FEE_BPS">PROFILE_SALE_FEE_BPS</a>: u64 = 500;
</code></pre>



<a name="social_contracts_profile_RESERVED_NAMES"></a>

Reserved usernames that cannot be registered


<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_RESERVED_NAMES">RESERVED_NAMES</a>: vector&lt;vector&lt;u8&gt;&gt; = vector[vector[97, 100, 109, 105, 110], vector[97, 100, 109, 105, 110, 105, 115, 116, 114, 97, 116, 111, 114], vector[111, 119, 110, 101, 114], vector[109, 111, 100], vector[109, 111, 100, 101, 114, 97, 116, 111, 114], vector[115, 116, 97, 102, 102], vector[115, 117, 112, 112, 111, 114, 116], vector[109, 121, 115, 111], vector[109, 121, 115, 111, 99, 105, 97, 108], vector[115, 121, 115, 116, 101, 109], vector[114, 111, 111, 116], vector[111, 102, 102, 105, 99, 105, 97, 108], vector[102, 117, 99, 107], vector[115, 104, 105, 116], vector[97, 115, 115], vector[112, 105, 115, 115], vector[99, 117, 110, 116], vector[97, 115, 115, 104, 111, 108, 101], vector[100, 105, 99, 107], vector[112, 117, 115, 115, 121], vector[115, 101, 120]];
</code></pre>



<a name="social_contracts_profile_USERNAME_FIELD"></a>

Field names for dynamic fields


<pre><code><b>const</b> <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>: vector&lt;u8&gt; = vector[117, 115, 101, 114, 110, 97, 109, 101];
</code></pre>



<a name="social_contracts_profile_init"></a>

## Function `init`

Module initializer to create the username registry


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_init">init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_init">init</a>(ctx: &<b>mut</b> TxContext) {
    // Import current <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> from <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> <b>module</b>
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>let</b> registry = <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a> {
        <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: object::new(ctx),
        usernames: table::new(ctx),
        address_profiles: table::new(ctx),
        <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>: current_version,
    };
    // Create the Ecosystem treasury owned by the contract deployer
    <b>let</b> treasury = <a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">EcosystemTreasury</a> {
        <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: object::new(ctx),
        treasury_address: tx_context::sender(ctx),
    };
    // Share the registry to make it globally accessible
    transfer::share_object(registry);
    // Share the treasury to make it globally accessible
    transfer::share_object(treasury);
}
</code></pre>



</details>

<a name="social_contracts_profile_is_reserved_name"></a>

## Function `is_reserved_name`

Check if a name is reserved and cannot be registered


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_is_reserved_name">is_reserved_name</a>(name: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_is_reserved_name">is_reserved_name</a>(name: &String): bool {
    // Convert name string to lowercase <b>for</b> comparison
    <b>let</b> name_bytes = string::as_bytes(name);
    <b>let</b> lowercase_name = <a href="../social_contracts/profile.md#social_contracts_profile_to_lowercase_bytes">to_lowercase_bytes</a>(name_bytes);
    // Make a local <b>copy</b> of <a href="../social_contracts/profile.md#social_contracts_profile_RESERVED_NAMES">RESERVED_NAMES</a> to avoid implicit copies
    <b>let</b> reserved_names = <a href="../social_contracts/profile.md#social_contracts_profile_RESERVED_NAMES">RESERVED_NAMES</a>;
    <b>let</b> reserved_count = vector::length(&reserved_names);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; reserved_count) {
        <b>let</b> reserved = *vector::borrow(&reserved_names, i);
        // Exact match with reserved name (case-insensitive)
        <b>if</b> (vector::length(&lowercase_name) == vector::length(&reserved)) {
            <b>let</b> <b>mut</b> is_match = <b>true</b>;
            <b>let</b> <b>mut</b> j = 0;
            <b>while</b> (j &lt; vector::length(&reserved)) {
                <b>if</b> (*vector::borrow(&lowercase_name, j) != *vector::borrow(&reserved, j)) {
                    is_match = <b>false</b>;
                    <b>break</b>
                };
                j = j + 1;
            };
            <b>if</b> (is_match) {
                <b>return</b> <b>true</b>
            };
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_profile_to_lowercase_bytes"></a>

## Function `to_lowercase_bytes`

Convert a byte vector to lowercase


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_to_lowercase_bytes">to_lowercase_bytes</a>(bytes: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_to_lowercase_bytes">to_lowercase_bytes</a>(bytes: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> result = vector::empty&lt;u8&gt;();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(bytes);
    <b>while</b> (i &lt; len) {
        <b>let</b> b = *vector::borrow(bytes, i);
        vector::push_back(&<b>mut</b> result, <a href="../social_contracts/profile.md#social_contracts_profile_to_lowercase_byte">to_lowercase_byte</a>(b));
        i = i + 1;
    };
    result
}
</code></pre>



</details>

<a name="social_contracts_profile_to_lowercase_byte"></a>

## Function `to_lowercase_byte`

Convert a single ASCII byte to lowercase


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_to_lowercase_byte">to_lowercase_byte</a>(b: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_to_lowercase_byte">to_lowercase_byte</a>(b: u8): u8 {
    <b>if</b> (b &gt;= 65 && b &lt;= 90) { // A-Z
        <b>return</b> b + 32 // convert to a-z
    };
    b
}
</code></pre>



</details>

<a name="social_contracts_profile_ascii_to_string"></a>

## Function `ascii_to_string`

Convert an ASCII String to a String


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(ascii_str: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(ascii_str: ascii::String): String {
    string::utf8(ascii::into_bytes(ascii_str))
}
</code></pre>



</details>

<a name="social_contracts_profile_create_profile"></a>

## Function `create_profile`

Create a new profile with a required username
This is the main entry point for new users, combining profile and username creation


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_create_profile">create_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, profile_picture_url: vector&lt;u8&gt;, cover_photo_url: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_create_profile">create_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>,
    <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: String,
    <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: String,
    <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: String,
    profile_picture_url: vector&lt;u8&gt;,
    cover_photo_url: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), 1);
    <b>let</b> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> = tx_context::sender(ctx);
    <b>let</b> now = tx_context::epoch(ctx);
    // Check that the sender doesn't already have a <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(!table::contains(&registry.address_profiles, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>), <a href="../social_contracts/profile.md#social_contracts_profile_EProfileAlreadyExists">EProfileAlreadyExists</a>);
    // Validate the <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>
    <b>let</b> username_bytes = string::as_bytes(&<a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>);
    <b>let</b> username_length = vector::length(username_bytes);
    // Username length validation - between 2 and 50 characters
    <b>assert</b>!(username_length &gt;= 2 && username_length &lt;= 50, <a href="../social_contracts/profile.md#social_contracts_profile_EInvalidUsername">EInvalidUsername</a>);
    // Check <b>if</b> <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a> is reserved in the hard coded list
    <b>assert</b>!(!<a href="../social_contracts/profile.md#social_contracts_profile_is_reserved_name">is_reserved_name</a>(&<a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>), <a href="../social_contracts/profile.md#social_contracts_profile_EReservedName">EReservedName</a>);
    // Check that the <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a> isn't already registered
    <b>assert</b>!(!table::contains(&registry.usernames, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>), <a href="../social_contracts/profile.md#social_contracts_profile_EUsernameNotAvailable">EUsernameNotAvailable</a>);
    // Create the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> object
    <b>let</b> <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a> = <b>if</b> (vector::length(&profile_picture_url) &gt; 0) {
        option::some(url::new_unsafe_from_bytes(profile_picture_url))
    } <b>else</b> {
        option::none()
    };
    <b>let</b> <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a> = <b>if</b> (vector::length(&cover_photo_url) &gt; 0) {
        option::some(url::new_unsafe_from_bytes(cover_photo_url))
    } <b>else</b> {
        option::none()
    };
    <b>let</b> display_name_option = <b>if</b> (string::length(&<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>) &gt; 0) {
        option::some(<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>)
    } <b>else</b> {
        option::none()
    };
    <b>let</b> <b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> = <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a> {
        <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: object::new(ctx),
        <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: display_name_option,
        <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>,
        created_at: now,
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>,
        x_username: option::none(),
        mastodon_username: option::none(),
        facebook_username: option::none(),
        reddit_username: option::none(),
        github_username: option::none(),
        instagram_username: option::none(),
        <a href="../social_contracts/profile.md#social_contracts_profile_last_updated">last_updated</a>: now,
        followers_count: 0,
        following_count: 0,
        post_count: 0,
        tips_received: 0,
        <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: option::none(),
        badges: vector::empty&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a>&gt;(),
        attached_my_ip_ids: vector::empty&lt;<b>address</b>&gt;(),
    };
    // Get the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id = object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    // Store the <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a> directly on the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    // We'll create the authorized_services table only when needed (lazy initialization)
    <b>if</b> (dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>)) {
        // This should never happen but we check <b>as</b> a safeguard
        <b>abort</b> <a href="../social_contracts/profile.md#social_contracts_profile_EProfileCreateFailed">EProfileCreateFailed</a>
    };
    dynamic_field::add(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>);
    // Add to registry mappings
    table::add(&<b>mut</b> registry.usernames, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>, profile_id);
    table::add(&<b>mut</b> registry.address_profiles, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, profile_id);
    // Extract display name value <b>for</b> the event (<b>if</b> available)
    <b>let</b> display_name_value = <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>)) {
        <b>let</b> name_copy = *option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>);
        name_copy
    } <b>else</b> {
        string::utf8(b"")
    };
    // Convert URL to String <b>for</b> events
    <b>let</b> profile_picture_string = <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>)) {
        <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>);
        option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
    } <b>else</b> {
        option::none()
    };
    // Convert URL to String <b>for</b> events
    <b>let</b> cover_photo_string = <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>)) {
        <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>);
        option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
    } <b>else</b> {
        option::none()
    };
    // Emit <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> creation event
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileCreatedEvent">ProfileCreatedEvent</a> {
        profile_id,
        <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: display_name_value,
        <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: option::some(<a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>),
        <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: profile_picture_string,
        <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: cover_photo_string,
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>,
        created_at: tx_context::epoch(ctx),
    });
    // Transfer <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> to <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    transfer::transfer(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>);
}
</code></pre>



</details>

<a name="social_contracts_profile_transfer_profile"></a>

## Function `transfer_profile`

Transfer a profile to a new owner
The username stays with the profile, and the transfer updates registry mappings


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_transfer_profile">transfer_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, new_owner: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_transfer_profile">transfer_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>,
    <b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    new_owner: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), 1);
    <b>let</b> sender = tx_context::sender(ctx);
    // Verify sender is the <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Get the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id = object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    // Update registry mappings
    table::remove(&<b>mut</b> registry.address_profiles, sender);
    // Check <b>if</b> the offeror already <b>has</b> a <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> in the registry
    // If so, remove it before adding the new mapping (allows <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> swapping)
    <b>if</b> (table::contains(&registry.address_profiles, new_owner)) {
        table::remove(&<b>mut</b> registry.address_profiles, new_owner);
    };
    table::add(&<b>mut</b> registry.address_profiles, new_owner, profile_id);
    // Update the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> = new_owner;
    // Emit a comprehensive <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> updated event to indicate ownership change
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileUpdatedEvent">ProfileUpdatedEvent</a> {
        profile_id,
        <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: <b>if</b> (dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>)) {
            option::some(*dynamic_field::borrow&lt;vector&lt;u8&gt;, String&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>))
        } <b>else</b> {
            option::none()
        },
        <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>)) {
            <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>);
            option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
        } <b>else</b> {
            option::none()
        },
        <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>)) {
            <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>);
            option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
        } <b>else</b> {
            option::none()
        },
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: new_owner,
        updated_at: tx_context::epoch(ctx),
        // Social media usernames
        x_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.x_username,
        mastodon_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.mastodon_username,
        facebook_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.facebook_username,
        reddit_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.reddit_username,
        github_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.github_username,
        instagram_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.instagram_username,
        <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>,
    });
    // Transfer <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> to new <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    transfer::public_transfer(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>, new_owner);
}
</code></pre>



</details>

<a name="social_contracts_profile_update_profile"></a>

## Function `update_profile`

Only the profile owner can update profile information
Authorized services (via authorize_read_service) can only read data, never modify it


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_update_profile">update_profile</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, new_display_name: <a href="../std/string.md#std_string_String">std::string::String</a>, new_bio: <a href="../std/string.md#std_string_String">std::string::String</a>, new_profile_picture_url: vector&lt;u8&gt;, new_cover_photo_url: vector&lt;u8&gt;, x_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, mastodon_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, facebook_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, reddit_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, github_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, instagram_username: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_update_profile">update_profile</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    // Basic <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> fields
    new_display_name: String,
    new_bio: String,
    new_profile_picture_url: vector&lt;u8&gt;,
    new_cover_photo_url: vector&lt;u8&gt;,
    // Social media usernames (all optional)
    x_username: Option&lt;String&gt;,
    mastodon_username: Option&lt;String&gt;,
    facebook_username: Option&lt;String&gt;,
    reddit_username: Option&lt;String&gt;,
    github_username: Option&lt;String&gt;,
    instagram_username: Option&lt;String&gt;,
    <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify sender is the <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Get current timestamp
    <b>let</b> now = tx_context::epoch(ctx);
    // Update basic <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> information
    // Set display name <b>if</b> provided, otherwise keep existing
    <b>if</b> (string::length(&new_display_name) &gt; 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a> = option::some(new_display_name);
    };
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a> = new_bio;
    <b>if</b> (vector::length(&new_profile_picture_url) &gt; 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a> = option::some(url::new_unsafe_from_bytes(new_profile_picture_url));
    };
    <b>if</b> (vector::length(&new_cover_photo_url) &gt; 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a> = option::some(url::new_unsafe_from_bytes(new_cover_photo_url));
    };
    // Update social media usernames <b>if</b> provided
    <b>if</b> (option::is_some(&x_username)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.x_username = x_username;
    };
    <b>if</b> (option::is_some(&mastodon_username)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.mastodon_username = mastodon_username;
    };
    <b>if</b> (option::is_some(&facebook_username)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.facebook_username = facebook_username;
    };
    <b>if</b> (option::is_some(&reddit_username)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.reddit_username = reddit_username;
    };
    <b>if</b> (option::is_some(&github_username)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.github_username = github_username;
    };
    <b>if</b> (option::is_some(&instagram_username)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.instagram_username = instagram_username;
    };
    <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>)) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a> = <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>;
    };
    // Update the last updated timestamp
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_last_updated">last_updated</a> = now;
    // Get current <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>
    <b>let</b> username_option = <b>if</b> (dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>)) {
        option::some(*dynamic_field::borrow&lt;vector&lt;u8&gt;, String&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>))
    } <b>else</b> {
        option::none()
    };
    // Convert URL to String <b>for</b> events
    <b>let</b> profile_picture_string = <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>)) {
        <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>);
        option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
    } <b>else</b> {
        option::none()
    };
    // Convert URL to String <b>for</b> events
    <b>let</b> cover_photo_string = <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>)) {
        <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>);
        option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
    } <b>else</b> {
        option::none()
    };
    // Emit comprehensive <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> update event with all fields
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileUpdatedEvent">ProfileUpdatedEvent</a> {
        profile_id: object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>),
        <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: username_option,
        <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: profile_picture_string,
        <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: cover_photo_string,
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>,
        updated_at: now,
        // Social media usernames
        x_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.x_username,
        mastodon_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.mastodon_username,
        facebook_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.facebook_username,
        reddit_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.reddit_username,
        github_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.github_username,
        instagram_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.instagram_username,
        <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_profile_display_name"></a>

## Function `display_name`

Get the display name of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): Option&lt;String&gt; {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_bio"></a>

## Function `bio`

Get the bio of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): String {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_profile_picture"></a>

## Function `profile_picture`

Get the profile picture URL of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): &Option&lt;Url&gt; {
    &<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_cover_photo"></a>

## Function `cover_photo`

Get the cover photo URL of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): &Option&lt;Url&gt; {
    &<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_owner"></a>

## Function `owner`

Get the owner of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): <b>address</b> {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_id"></a>

## Function `id`

Get the ID of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): &UID {
    &<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_last_updated"></a>

## Function `last_updated`

Get the last update timestamp for profile data


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_last_updated">last_updated</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_last_updated">last_updated</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_last_updated">last_updated</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_username"></a>

## Function `username`

Get the username string for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): Option&lt;String&gt; {
    <b>if</b> (dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>)) {
        option::some(*dynamic_field::borrow&lt;vector&lt;u8&gt;, String&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_profile_lookup_profile_by_username"></a>

## Function `lookup_profile_by_username`

Lookup profile ID by username in the registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_username">lookup_profile_by_username</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_username">lookup_profile_by_username</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: String): Option&lt;<b>address</b>&gt; {
    <b>if</b> (table::contains(&registry.usernames, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>)) {
        option::some(*table::borrow(&registry.usernames, <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_profile_lookup_profile_by_owner"></a>

## Function `lookup_profile_by_owner`

Lookup profile ID by owner address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">lookup_profile_by_owner</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">lookup_profile_by_owner</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: <b>address</b>): Option&lt;<b>address</b>&gt; {
    <b>if</b> (table::contains(&registry.address_profiles, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>)) {
        option::some(*table::borrow(&registry.address_profiles, <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_profile_is_authorized_service"></a>

## Function `is_authorized_service`

Check if an address is registered in the authorized_services table
Tests if the address is in the authorized_services table
Returns false if the address is not authorized or if the authorization table doesn't exist


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_is_authorized_service">is_authorized_service</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, <b>address</b>: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_is_authorized_service">is_authorized_service</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, <b>address</b>: <b>address</b>): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services")) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> authorized_services = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, String&gt;&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services");
    table::contains(authorized_services, <b>address</b>)
}
</code></pre>



</details>

<a name="social_contracts_profile_authorize_service"></a>

## Function `authorize_service`

Add an authorized service to a profile, initializing the table if needed
Only the profile owner can authorize services


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_authorize_service">authorize_service</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, service_address: <b>address</b>, service_name: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_authorize_service">authorize_service</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    service_address: <b>address</b>,
    service_name: String,
    ctx: &<b>mut</b> TxContext
) {
    // Verify the sender is the <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> - only <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> can authorize services
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Verify service <b>address</b> is not the same <b>as</b> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> (would be redundant)
    <b>assert</b>!(service_address != <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, <a href="../social_contracts/profile.md#social_contracts_profile_ENotAuthorizedService">ENotAuthorizedService</a>);
    // Create the table <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services")) {
        <b>let</b> authorized_services = table::new&lt;<b>address</b>, String&gt;(ctx);
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services", authorized_services);
    };
    // Get the table and add the service
    <b>let</b> authorized_services = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, String&gt;&gt;(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services");
    // Only add <b>if</b> not already in the table
    <b>if</b> (!table::contains(authorized_services, service_address)) {
        table::add(authorized_services, service_address, service_name);
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_revoke_authorization"></a>

## Function `revoke_authorization`

Remove an authorized service from a profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_revoke_authorization">revoke_authorization</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, service_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_revoke_authorization">revoke_authorization</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    service_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Verify the sender is the <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> - only <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> can revoke authorizations
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> authorized_services table exists
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services")) {
        <b>return</b>
    };
    // Get the table and remove the service <b>if</b> it exists
    <b>let</b> authorized_services = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, String&gt;&gt;(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, b"authorized_services");
    <b>if</b> (table::contains(authorized_services, service_address)) {
        table::remove(authorized_services, service_address);
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_get_id_address"></a>

## Function `get_id_address`

Get the ID address of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): <b>address</b> {
    object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>)
}
</code></pre>



</details>

<a name="social_contracts_profile_get_owner"></a>

## Function `get_owner`

Get the owner of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">get_owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">get_owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): <b>address</b> {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_get_followers_count"></a>

## Function `get_followers_count`

Get the followers count for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_followers_count">get_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_followers_count">get_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count
}
</code></pre>



</details>

<a name="social_contracts_profile_get_post_count"></a>

## Function `get_post_count`

Get the post count for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_post_count">get_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_post_count">get_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count
}
</code></pre>



</details>

<a name="social_contracts_profile_get_tips_received"></a>

## Function `get_tips_received`

Get the tips received for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_tips_received">get_tips_received</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_tips_received">get_tips_received</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.tips_received
}
</code></pre>



</details>

<a name="social_contracts_profile_increment_followers_count"></a>

## Function `increment_followers_count`

Increment followers count (called by follow module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_increment_followers_count">increment_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_increment_followers_count">increment_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count + 1;
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count
}
</code></pre>



</details>

<a name="social_contracts_profile_decrement_followers_count"></a>

## Function `decrement_followers_count`

Decrement followers count (called by follow module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_decrement_followers_count">decrement_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_decrement_followers_count">decrement_followers_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <b>if</b> (<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count &gt; 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count - 1;
    };
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.followers_count
}
</code></pre>



</details>

<a name="social_contracts_profile_increment_post_count"></a>

## Function `increment_post_count`

Increment post count (called by post module when creating a post)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_increment_post_count">increment_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_increment_post_count">increment_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count + 1;
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count
}
</code></pre>



</details>

<a name="social_contracts_profile_decrement_post_count"></a>

## Function `decrement_post_count`

Decrement post count (called by post module when deleting a post)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_decrement_post_count">decrement_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_decrement_post_count">decrement_post_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <b>if</b> (<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count &gt; 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count - 1;
    };
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.post_count
}
</code></pre>



</details>

<a name="social_contracts_profile_add_tips_received"></a>

## Function `add_tips_received`

Add tips received (called by post/comment module when tipping)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_add_tips_received">add_tips_received</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_add_tips_received">add_tips_received</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, amount: u64): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.tips_received = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.tips_received + amount;
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.tips_received
}
</code></pre>



</details>

<a name="social_contracts_profile_get_following_count"></a>

## Function `get_following_count`

Get the following count for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_following_count">get_following_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_following_count">get_following_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count
}
</code></pre>



</details>

<a name="social_contracts_profile_increment_following_count"></a>

## Function `increment_following_count`

Increment following count (called when this profile follows another profile)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_increment_following_count">increment_following_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_increment_following_count">increment_following_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count + 1;
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count
}
</code></pre>



</details>

<a name="social_contracts_profile_decrement_following_count"></a>

## Function `decrement_following_count`

Decrement following count (called when this profile unfollows another profile)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_decrement_following_count">decrement_following_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_decrement_following_count">decrement_following_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    <b>if</b> (<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count &gt; 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count - 1;
    };
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.following_count
}
</code></pre>



</details>

<a name="social_contracts_profile_create_subscription_service"></a>

## Function `create_subscription_service`

Create a subscription service for this profile (creates separate service object)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_create_subscription_service">create_subscription_service</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, monthly_fee: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_create_subscription_service">create_subscription_service</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    monthly_fee: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Create the <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> service and share it
    <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service_entry">subscription::create_profile_service_entry</a>(monthly_fee, ctx);
}
</code></pre>



</details>

<a name="social_contracts_profile_has_valid_subscription"></a>

## Function `has_valid_subscription`

Check if a viewer has a valid subscription (uses subscription module functions)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_valid_subscription">has_valid_subscription</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_valid_subscription">has_valid_subscription</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &ProfileSubscription,
    service: &ProfileSubscriptionService,
    clock: &Clock,
): bool {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">subscription::is_subscription_valid</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>, service, clock)
}
</code></pre>



</details>

<a name="social_contracts_profile_attach_my_ip"></a>

## Function `attach_my_ip`

Attach MyIP to profile for data monetization


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_attach_my_ip">attach_my_ip</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, my_ip_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_attach_my_ip">attach_my_ip</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    my_ip_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Initialize table <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>)) {
        <b>let</b> tbl = table::new&lt;<b>address</b>, bool&gt;(ctx);
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>, tbl);
    };
    <b>let</b> tbl = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, bool&gt;&gt;(
        &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>,
    );
    // Only add <b>if</b> not already attached
    <b>if</b> (!table::contains(tbl, my_ip_id)) {
        table::add(tbl, my_ip_id, <b>true</b>);
        // Also add to the tracking vector <b>for</b> efficient iteration
        vector::push_back(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids, my_ip_id);
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_has_my_ip_attached"></a>

## Function `has_my_ip_attached`

Check if a MyIP is attached to this profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_my_ip_attached">has_my_ip_attached</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, my_ip_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_my_ip_attached">has_my_ip_attached</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, my_ip_id: <b>address</b>): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> tbl = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, bool&gt;&gt;(
        &<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>,
    );
    table::contains(tbl, my_ip_id)
}
</code></pre>



</details>

<a name="social_contracts_profile_detach_my_ip"></a>

## Function `detach_my_ip`

Remove a MyIP attachment from the profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_detach_my_ip">detach_my_ip</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, my_ip_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_detach_my_ip">detach_my_ip</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    my_ip_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>)) {
        <b>return</b>
    };
    <b>let</b> tbl = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, bool&gt;&gt;(
        &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>,
    );
    <b>if</b> (table::contains(tbl, my_ip_id)) {
        table::remove(tbl, my_ip_id);
        // Also remove from the tracking vector
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids);
        <b>while</b> (i &lt; len) {
            <b>if</b> (*vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids, i) == my_ip_id) {
                vector::remove(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids, i);
                <b>break</b>
            };
            i = i + 1;
        };
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_get_attached_my_ips"></a>

## Function `get_attached_my_ips`

Get all attached MyIP IDs for this profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_attached_my_ips">get_attached_my_ips</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_attached_my_ips">get_attached_my_ips</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): vector&lt;<b>address</b>&gt; {
    // Return a <b>copy</b> of the attached MyIP IDs vector <b>for</b> efficient iteration
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids
}
</code></pre>



</details>

<a name="social_contracts_profile_batch_attach_my_ips"></a>

## Function `batch_attach_my_ips`

Batch attach multiple MyIPs to profile for gas optimization


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_batch_attach_my_ips">batch_attach_my_ips</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, my_ip_ids: vector&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_batch_attach_my_ips">batch_attach_my_ips</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    my_ip_ids: vector&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Initialize table <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>)) {
        <b>let</b> tbl = table::new&lt;<b>address</b>, bool&gt;(ctx);
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>, tbl);
    };
    <b>let</b> tbl = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, bool&gt;&gt;(
        &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>,
    );
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&my_ip_ids);
    <b>while</b> (i &lt; len) {
        <b>let</b> my_ip_id = *vector::borrow(&my_ip_ids, i);
        // Only add <b>if</b> not already attached
        <b>if</b> (!table::contains(tbl, my_ip_id)) {
            table::add(tbl, my_ip_id, <b>true</b>);
            vector::push_back(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids, my_ip_id);
        };
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_batch_detach_my_ips"></a>

## Function `batch_detach_my_ips`

Batch detach multiple MyIPs from profile for gas optimization


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_batch_detach_my_ips">batch_detach_my_ips</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, my_ip_ids: vector&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_batch_detach_my_ips">batch_detach_my_ips</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    my_ip_ids: vector&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>)) {
        <b>return</b>
    };
    <b>let</b> tbl = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, bool&gt;&gt;(
        &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_MY_IP_DATA_FIELD">MY_IP_DATA_FIELD</a>,
    );
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&my_ip_ids);
    <b>while</b> (i &lt; len) {
        <b>let</b> my_ip_id = *vector::borrow(&my_ip_ids, i);
        <b>if</b> (table::contains(tbl, my_ip_id)) {
            table::remove(tbl, my_ip_id);
            // Remove from tracking vector
            <b>let</b> <b>mut</b> j = 0;
            <b>let</b> vec_len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids);
            <b>while</b> (j &lt; vec_len) {
                <b>if</b> (*vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids, j) == my_ip_id) {
                    vector::remove(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.attached_my_ip_ids, j);
                    <b>break</b>
                };
                j = j + 1;
            };
        };
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_create_offer"></a>

## Function `create_offer`

Create an offer to purchase a profile
Locks MYSO tokens in the offer


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_create_offer">create_offer</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_create_offer">create_offer</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> profile_owner = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>;
    <b>let</b> profile_id = object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Cannot offer on your own <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(sender != profile_owner, <a href="../social_contracts/profile.md#social_contracts_profile_ECannotOfferOwnProfile">ECannotOfferOwnProfile</a>);
    // Check <b>if</b> there's sufficient tokens
    <b>assert</b>!(coin::value(coin) &gt;= amount && amount &gt; 0, <a href="../social_contracts/profile.md#social_contracts_profile_EInsufficientTokens">EInsufficientTokens</a>);
    // Check <b>if</b> the offer meets the minimum amount requirement (<b>if</b> set)
    <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>)) {
        <b>let</b> min_amount = *option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>);
        <b>assert</b>!(amount &gt;= min_amount, <a href="../social_contracts/profile.md#social_contracts_profile_EOfferBelowMinimum">EOfferBelowMinimum</a>);
    };
    // Initialize offers table <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>)) {
        <b>let</b> offers = table::new&lt;<b>address</b>, <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a>&gt;(ctx);
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>, offers);
    };
    // Get the offers table
    <b>let</b> offers = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>);
    // Check <b>if</b> the sender already <b>has</b> an offer
    <b>assert</b>!(!table::contains(offers, sender), <a href="../social_contracts/profile.md#social_contracts_profile_EOfferAlreadyExists">EOfferAlreadyExists</a>);
    // Split tokens from the coin and convert to a balance <b>for</b> secure storage
    <b>let</b> offer_coin = coin::split(coin, amount, ctx);
    // Convert to balance to lock tokens in the offer
    <b>let</b> locked_myso = coin::into_balance(offer_coin);
    // Create and store the offer with locked tokens
    <b>let</b> offer = <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a> {
        offeror: sender,
        amount,
        created_at: now,
        locked_myso,
    };
    table::add(offers, sender, offer);
    // Emit an event to track offer creation
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileOfferCreatedEvent">ProfileOfferCreatedEvent</a> {
        profile_id,
        offeror: sender,
        amount,
        created_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_profile_accept_offer"></a>

## Function `accept_offer`

Accept an offer to purchase a profile
Transfers tokens to the profile owner and profile ownership to the offeror


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_accept_offer">accept_offer</a>(registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, offeror: <b>address</b>, new_main_profile: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_accept_offer">accept_offer</a>(
    registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>,
    <b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">EcosystemTreasury</a>,
    offeror: <b>address</b>,
    new_main_profile: Option&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> profile_id = object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Verify sender is the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> offers table exists
    <b>assert</b>!(dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>), <a href="../social_contracts/profile.md#social_contracts_profile_EOfferDoesNotExist">EOfferDoesNotExist</a>);
    // Get the offers table
    <b>let</b> offers = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>);
    // Check <b>if</b> the offer exists
    <b>assert</b>!(table::contains(offers, offeror), <a href="../social_contracts/profile.md#social_contracts_profile_EOfferDoesNotExist">EOfferDoesNotExist</a>);
    // Remove the offer from the table and get the locked tokens
    <b>let</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a> { offeror: _, amount, created_at: _, locked_myso } = table::remove(offers, offeror);
    // Calculate the fee amount (5% of the total)
    <b>let</b> fee_amount = (amount * <a href="../social_contracts/profile.md#social_contracts_profile_PROFILE_SALE_FEE_BPS">PROFILE_SALE_FEE_BPS</a>) / 10000;
    // Convert the locked balance to a coin
    <b>let</b> <b>mut</b> payment = coin::from_balance(locked_myso, ctx);
    // Split the fee amount to send to the treasury
    <b>let</b> fee_payment = coin::split(&<b>mut</b> payment, fee_amount, ctx);
    // Send the fee to the treasury treasury
    transfer::public_transfer(fee_payment, treasury.treasury_address);
    // Send the remaining amount to the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    transfer::public_transfer(payment, sender);
    // Update registry mappings to reflect new ownership
    table::remove(&<b>mut</b> registry.address_profiles, sender);
    // Check <b>if</b> the offeror already <b>has</b> a <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> in the registry
    // If so, remove it before adding the new mapping (allows <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> swapping)
    <b>if</b> (table::contains(&registry.address_profiles, offeror)) {
        table::remove(&<b>mut</b> registry.address_profiles, offeror);
    };
    // Add new mapping <b>for</b> buyer
    table::add(&<b>mut</b> registry.address_profiles, offeror, profile_id);
    // If the seller provided a new main <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>, register it <b>as</b> their main <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>if</b> (option::is_some(&new_main_profile)) {
        <b>let</b> new_profile_id = *option::borrow(&new_main_profile);
        // Add the new <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> mapping <b>for</b> the seller
        table::add(&<b>mut</b> registry.address_profiles, sender, new_profile_id);
    };
    // Update the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <b>let</b> previous_owner = <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>;
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> = offeror;
    // Emit an event to track offer acceptance and token transfer
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileOfferAcceptedEvent">ProfileOfferAcceptedEvent</a> {
        profile_id,
        offeror,
        previous_owner,
        amount,
        accepted_at: now,
    });
    // Emit a comprehensive <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> updated event to indicate ownership change
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileUpdatedEvent">ProfileUpdatedEvent</a> {
        profile_id,
        <a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_display_name">display_name</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_username">username</a>: <b>if</b> (dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>)) {
            option::some(*dynamic_field::borrow&lt;vector&lt;u8&gt;, String&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_USERNAME_FIELD">USERNAME_FIELD</a>))
        } <b>else</b> {
            option::none()
        },
        <a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_bio">bio</a>,
        <a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>: <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>)) {
            <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_profile_picture">profile_picture</a>);
            option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
        } <b>else</b> {
            option::none()
        },
        <a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>: <b>if</b> (option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>)) {
            <b>let</b> url = option::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_cover_photo">cover_photo</a>);
            option::some(<a href="../social_contracts/profile.md#social_contracts_profile_ascii_to_string">ascii_to_string</a>(url::inner_url(url)))
        } <b>else</b> {
            option::none()
        },
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: offeror,
        updated_at: now,
        // Social media usernames
        x_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.x_username,
        mastodon_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.mastodon_username,
        facebook_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.facebook_username,
        reddit_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.reddit_username,
        github_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.github_username,
        instagram_username: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.instagram_username,
        <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>: <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>,
    });
    // Emit a fee event
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileSaleFeeEvent">ProfileSaleFeeEvent</a> {
        profile_id,
        offeror,
        previous_owner,
        sale_amount: amount,
        fee_amount,
        fee_recipient: treasury.treasury_address,
        timestamp: now,
    });
    // Transfer the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> object to the new <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    transfer::public_transfer(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>, offeror);
}
</code></pre>



</details>

<a name="social_contracts_profile_reject_or_revoke_offer"></a>

## Function `reject_or_revoke_offer`

Reject or revoke an offer on a profile
Can be called by the profile owner to reject or the offeror to revoke
Returns locked MYSO tokens to the offeror


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_reject_or_revoke_offer">reject_or_revoke_offer</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, offeror: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_reject_or_revoke_offer">reject_or_revoke_offer</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    offeror: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> profile_id = object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Check <b>if</b> offers table exists
    <b>assert</b>!(dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>), <a href="../social_contracts/profile.md#social_contracts_profile_EOfferDoesNotExist">EOfferDoesNotExist</a>);
    // Get the offers table
    <b>let</b> offers = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>);
    // Check <b>if</b> the offer exists
    <b>assert</b>!(table::contains(offers, offeror), <a href="../social_contracts/profile.md#social_contracts_profile_EOfferDoesNotExist">EOfferDoesNotExist</a>);
    // Verify sender is either the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> or the offeror
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender || offeror == sender, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorizedOfferAction">EUnauthorizedOfferAction</a>);
    // Remove the offer from the table and get the locked tokens
    <b>let</b> <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a> { offeror, amount, created_at: _, locked_myso } = table::remove(offers, offeror);
    // Convert the locked balance back to a coin and <b>return</b> to the offeror
    // This unlocks the tokens and returns them to the original offeror
    <b>let</b> refund = coin::from_balance(locked_myso, ctx);
    transfer::public_transfer(refund, offeror);
    // Determine <b>if</b> this is a rejection (by <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>) or revocation (by offeror)
    <b>let</b> is_revoked = offeror == sender;
    // Emit an event to track offer rejection/revocation and token <b>return</b>
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_ProfileOfferRejectedEvent">ProfileOfferRejectedEvent</a> {
        profile_id,
        offeror,
        rejected_by: sender,
        amount,
        rejected_at: now,
        is_revoked,
    });
}
</code></pre>



</details>

<a name="social_contracts_profile_has_offer_from"></a>

## Function `has_offer_from`

Check if a profile has an offer from a specific address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_offer_from">has_offer_from</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, offeror: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_offer_from">has_offer_from</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, offeror: <b>address</b>): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> offers = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a>&gt;&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>);
    table::contains(offers, offeror)
}
</code></pre>



</details>

<a name="social_contracts_profile_has_offers"></a>

## Function `has_offers`

Check if a profile has any active offers


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_offers">has_offers</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_offers">has_offers</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> offers = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/profile.md#social_contracts_profile_ProfileOffer">ProfileOffer</a>&gt;&gt;(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>, <a href="../social_contracts/profile.md#social_contracts_profile_OFFERS_FIELD">OFFERS_FIELD</a>);
    table::length(offers) &gt; 0
}
</code></pre>



</details>

<a name="social_contracts_profile_get_treasury_address"></a>

## Function `get_treasury_address`

Get the treasury address from the EcosystemTreasury


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">get_treasury_address</a>(treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">get_treasury_address</a>(treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">EcosystemTreasury</a>): <b>address</b> {
    treasury.treasury_address
}
</code></pre>



</details>

<a name="social_contracts_profile_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>): u64 {
    registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_borrow_version_mut"></a>

## Function `borrow_version_mut`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_borrow_version_mut">borrow_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_borrow_version_mut">borrow_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_migrate_registry"></a>

## Function `migrate_registry`

Migration function for the registry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_migrate_registry">migrate_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>,
    _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">upgrade::UpgradeAdminCap</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> &gt; current <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>)
    <b>assert</b>!(registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> &lt; current_version, 1);
    // Remember old <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> and update to new <a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>
    <b>let</b> old_version = registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a>;
    registry.<a href="../social_contracts/profile.md#social_contracts_profile_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">UsernameRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_profile_min_offer_amount"></a>

## Function `min_offer_amount`

Get the minimum offer amount for a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): &Option&lt;u64&gt; {
    &<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_is_for_sale"></a>

## Function `is_for_sale`

Check if a profile is for sale (has a minimum offer amount set)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_is_for_sale">is_for_sale</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_is_for_sale">is_for_sale</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): bool {
    option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_min_offer_amount">min_offer_amount</a>)
}
</code></pre>



</details>

<a name="social_contracts_profile_add_badge_to_profile"></a>

## Function `add_badge_to_profile`

Adds a badge to a profile - called by platform module
This function trusts the caller has done authorization checks


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_add_badge_to_profile">add_badge_to_profile</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_id: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_name: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_description: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, platform_id: <b>address</b>, timestamp: u64, issuer: <b>address</b>, badge_type: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_add_badge_to_profile">add_badge_to_profile</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    badge_id: String,
    badge_name: String,
    badge_description: String,
    badge_image_url: String,
    platform_id: <b>address</b>,
    timestamp: u64,
    issuer: <b>address</b>,
    badge_type: u8
) {
    // Create the new badge
    <b>let</b> badge = <a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a> {
        badge_id: badge_id,
        name: badge_name,
        description: badge_description,
        image_url: badge_image_url,
        platform_id,
        issued_at: timestamp,
        issued_by: issuer,
        badge_type,
    };
    // Check <b>if</b> badge with same ID already exists
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges);
    <b>while</b> (i &lt; len) {
        <b>let</b> existing_badge = vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, i);
        <b>if</b> (string::as_bytes(&existing_badge.badge_id) == string::as_bytes(&badge_id)) {
            <b>abort</b> <a href="../social_contracts/profile.md#social_contracts_profile_EBadgeAlreadyExists">EBadgeAlreadyExists</a>
        };
        i = i + 1;
    };
    // Add the badge to the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    vector::push_back(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, badge);
    // Emit badge assigned event
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_BadgeAssignedEvent">BadgeAssignedEvent</a> {
        profile_id: object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>),
        badge_id: badge_id,
        name: badge_name,
        platform_id,
        assigned_by: issuer,
        assigned_at: timestamp,
        badge_type,
    });
}
</code></pre>



</details>

<a name="social_contracts_profile_remove_badge_from_profile"></a>

## Function `remove_badge_from_profile`

Removes a badge from a profile - called by platform module
This function trusts the caller has done authorization checks


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_remove_badge_from_profile">remove_badge_from_profile</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_id: &<a href="../std/string.md#std_string_String">std::string::String</a>, platform_id: <b>address</b>, revoker: <b>address</b>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_remove_badge_from_profile">remove_badge_from_profile</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>,
    badge_id: &String,
    platform_id: <b>address</b>,
    revoker: <b>address</b>,
    timestamp: u64
) {
    // Search <b>for</b> and remove the badge with the given ID
    <b>let</b> <b>mut</b> found = <b>false</b>;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges);
    <b>while</b> (i &lt; len) {
        <b>let</b> badge = vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, i);
        <b>if</b> (string::as_bytes(&badge.badge_id) == string::as_bytes(badge_id)) {
            // Ensure badge was issued by this <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
            <b>assert</b>!(badge.platform_id == platform_id, <a href="../social_contracts/profile.md#social_contracts_profile_EUnauthorized">EUnauthorized</a>);
            // Remove the badge at this index
            vector::remove(&<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, i);
            found = <b>true</b>;
            // Emit badge revoked event
            event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_BadgeRevokedEvent">BadgeRevokedEvent</a> {
                profile_id: object::uid_to_address(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>),
                badge_id: *badge_id,
                platform_id,
                revoked_by: revoker,
                revoked_at: timestamp,
            });
            <b>break</b>
        };
        i = i + 1;
    };
    // Make sure we found and removed the badge
    <b>assert</b>!(found, <a href="../social_contracts/profile.md#social_contracts_profile_EBadgeNotFound">EBadgeNotFound</a>);
}
</code></pre>



</details>

<a name="social_contracts_profile_get_profile_badges"></a>

## Function `get_profile_badges`

Get all badges associated with a profile


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_profile_badges">get_profile_badges</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): vector&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">social_contracts::profile::ProfileBadge</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_profile_badges">get_profile_badges</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): vector&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a>&gt; {
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges
}
</code></pre>



</details>

<a name="social_contracts_profile_has_badge"></a>

## Function `has_badge`

Check if a profile has a specific badge


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_badge">has_badge</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_id: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_has_badge">has_badge</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, badge_id: &String): bool {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges);
    <b>while</b> (i &lt; len) {
        <b>let</b> badge = vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, i);
        <b>if</b> (string::as_bytes(&badge.badge_id) == string::as_bytes(badge_id)) {
            <b>return</b> <b>true</b>
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_profile_get_badge"></a>

## Function `get_badge`

Get a specific badge from a profile by badge ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_badge">get_badge</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_id: &<a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">social_contracts::profile::ProfileBadge</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_badge">get_badge</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, badge_id: &String): Option&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a>&gt; {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges);
    <b>while</b> (i &lt; len) {
        <b>let</b> badge = vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, i);
        <b>if</b> (string::as_bytes(&badge.badge_id) == string::as_bytes(badge_id)) {
            <b>return</b> option::some(*badge)
        };
        i = i + 1;
    };
    option::none()
}
</code></pre>



</details>

<a name="social_contracts_profile_get_platform_badges"></a>

## Function `get_platform_badges`

Get badges issued by a specific platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_platform_badges">get_platform_badges</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, platform_id: <b>address</b>): vector&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">social_contracts::profile::ProfileBadge</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_get_platform_badges">get_platform_badges</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>, platform_id: <b>address</b>): vector&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a>&gt; {
    <b>let</b> <b>mut</b> result = vector::empty&lt;<a href="../social_contracts/profile.md#social_contracts_profile_ProfileBadge">ProfileBadge</a>&gt;();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges);
    <b>while</b> (i &lt; len) {
        <b>let</b> badge = vector::borrow(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges, i);
        <b>if</b> (badge.platform_id == platform_id) {
            vector::push_back(&<b>mut</b> result, *badge);
        };
        i = i + 1;
    };
    result
}
</code></pre>



</details>

<a name="social_contracts_profile_badge_count"></a>

## Function `badge_count`

Count the number of badges a profile has


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_badge_count">badge_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_badge_count">badge_count</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">Profile</a>): u64 {
    vector::length(&<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>.badges)
}
</code></pre>



</details>

<a name="social_contracts_profile_vest_myso"></a>

## Function `vest_myso`

Create a new vesting wallet with MYS tokens that vest over time with configurable curve
The start time must be in the future and duration must be greater than 0
curve_factor: 0 or 1000 = linear, >1000 = more tokens at end, <1000 = more tokens at start

Example Curves:
Exponential (curve_factor = 2000):
25% time → ~6% tokens
50% time → ~25% tokens
75% time → ~56% tokens
100% time → 100% tokens
Logarithmic (curve_factor = 500):
25% time → ~44% tokens
50% time → ~75% tokens
75% time → ~94% tokens
100% time → 100% tokens


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vest_myso">vest_myso</a>(coin: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, recipient: <b>address</b>, start_time: u64, duration: u64, curve_factor: u64, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vest_myso">vest_myso</a>(
    coin: Coin&lt;MYS&gt;,
    recipient: <b>address</b>,
    start_time: u64,
    duration: u64,
    curve_factor: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Validate that start time is in the future
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>assert</b>!(start_time &gt; current_time, <a href="../social_contracts/profile.md#social_contracts_profile_EInvalidStartTime">EInvalidStartTime</a>);
    // Validate that duration is greater than 0
    <b>assert</b>!(duration &gt; 0, <a href="../social_contracts/profile.md#social_contracts_profile_EInvalidStartTime">EInvalidStartTime</a>);
    <b>let</b> total_amount = coin::value(&coin);
    <b>assert</b>!(total_amount &gt; 0, <a href="../social_contracts/profile.md#social_contracts_profile_EInsufficientTokens">EInsufficientTokens</a>);
    // Default to linear <b>if</b> curve_factor is 0
    <b>let</b> final_curve_factor = <b>if</b> (curve_factor == 0) {
        <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> // 1000 = linear
    } <b>else</b> {
        // Validate curve factor is reasonable (between 100 and 10000, i.e., 0.1x to 10x)
        <b>assert</b>!(curve_factor &gt;= 100 && curve_factor &lt;= 10000, <a href="../social_contracts/profile.md#social_contracts_profile_EInvalidStartTime">EInvalidStartTime</a>);
        curve_factor
    };
    // Create the vesting wallet
    <b>let</b> wallet = <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a> {
        <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>: object::new(ctx),
        balance: coin::into_balance(coin),
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: recipient,
        start_time,
        claimed_amount: 0,
        duration,
        total_amount,
        curve_factor: final_curve_factor,
    };
    <b>let</b> wallet_id = object::uid_to_address(&wallet.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    // Emit vesting event
    event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_TokensVestedEvent">TokensVestedEvent</a> {
        wallet_id,
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: recipient,
        total_amount,
        start_time,
        duration,
        curve_factor: final_curve_factor,
        vested_at: current_time,
    });
    // Transfer the vesting wallet to the recipient
    transfer::public_transfer(wallet, recipient);
}
</code></pre>



</details>

<a name="social_contracts_profile_claim_vested_tokens"></a>

## Function `claim_vested_tokens`

Claim vested tokens from a vesting wallet
Only the wallet owner can claim tokens, and only claimable amounts


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_claim_vested_tokens">claim_vested_tokens</a>(wallet: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_claim_vested_tokens">claim_vested_tokens</a>(
    wallet: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    // Verify sender is the wallet <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <b>assert</b>!(wallet.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender, <a href="../social_contracts/profile.md#social_contracts_profile_ENotVestingWalletOwner">ENotVestingWalletOwner</a>);
    <b>let</b> claimable_amount = <a href="../social_contracts/profile.md#social_contracts_profile_calculate_claimable">calculate_claimable</a>(wallet, clock);
    // Only proceed <b>if</b> there are tokens to claim
    <b>if</b> (claimable_amount &gt; 0) {
        // Update claimed amount
        wallet.claimed_amount = wallet.claimed_amount + claimable_amount;
        // Create coin from the <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a> balance and transfer to <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
        <b>let</b> claimed_coin = coin::from_balance&lt;MYS&gt;(
            balance::split(&<b>mut</b> wallet.balance, claimable_amount),
            ctx
        );
        <b>let</b> wallet_id = object::uid_to_address(&wallet.<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
        <b>let</b> remaining_balance = balance::value(&wallet.balance);
        // Emit claim event
        event::emit(<a href="../social_contracts/profile.md#social_contracts_profile_TokensClaimedEvent">TokensClaimedEvent</a> {
            wallet_id,
            <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: sender,
            claimed_amount: claimable_amount,
            remaining_balance,
            claimed_at: clock::timestamp_ms(clock),
        });
        // Transfer claimed tokens to the <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
        transfer::public_transfer(claimed_coin, sender);
    };
}
</code></pre>



</details>

<a name="social_contracts_profile_claimable"></a>

## Function `claimable`

Calculate how many tokens can be claimed from a vesting wallet at the current time


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>, clock: &Clock): u64 {
    <a href="../social_contracts/profile.md#social_contracts_profile_calculate_claimable">calculate_claimable</a>(wallet, clock)
}
</code></pre>



</details>

<a name="social_contracts_profile_calculate_claimable"></a>

## Function `calculate_claimable`

Internal function to calculate claimable amount


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_calculate_claimable">calculate_claimable</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_calculate_claimable">calculate_claimable</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>, clock: &Clock): u64 {
    <b>let</b> current_time = clock::timestamp_ms(clock);
    // If vesting hasn't started yet, nothing is <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a>
    <b>if</b> (current_time &lt; wallet.start_time) {
        <b>return</b> 0
    };
    // If vesting period is complete, all remaining balance is <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a>
    <b>if</b> (current_time &gt;= wallet.start_time + wallet.duration) {
        <b>return</b> balance::value(&wallet.balance)
    };
    // Calculate progress <b>as</b> a percentage (0 to <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a>)
    <b>let</b> elapsed_time = current_time - wallet.start_time;
    <b>let</b> progress = ((elapsed_time <b>as</b> u128) * (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128)) / (wallet.duration <b>as</b> u128);
    // Apply curve based on curve_factor
    <b>let</b> curved_progress = <b>if</b> (wallet.curve_factor == <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a>) {
        // Linear vesting (curve_factor = 1000)
        progress
    } <b>else</b> <b>if</b> (wallet.curve_factor &gt; <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a>) {
        // Exponential curve - more tokens at the end
        // Use simplified exponential: progress^2 scaled by curve_factor
        <b>let</b> steepness = wallet.curve_factor - <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a>; // How much above linear
        <b>let</b> quadratic = (progress * progress) / (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128);
        <b>let</b> linear_part = (progress * (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128)) / (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128);
        // Blend between linear and quadratic based on steepness
        (linear_part * (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128) + quadratic * (steepness <b>as</b> u128)) / (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128)
    } <b>else</b> {
        // Logarithmic curve - more tokens at the start
        // Use simplified square root approximation <b>for</b> early release
        <b>let</b> steepness = <a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> - wallet.curve_factor; // How much below linear
        <b>let</b> sqrt_approx = <a href="../social_contracts/profile.md#social_contracts_profile_sqrt_approximation">sqrt_approximation</a>(progress * (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128)) * (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128) / (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128);
        <b>let</b> linear_part = progress;
        // Blend between square root and linear based on steepness
        (sqrt_approx * (steepness <b>as</b> u128) + linear_part * (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128)) / (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128)
    };
    // Convert back to total <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a> amount
    <b>let</b> total_claimable = ((wallet.total_amount <b>as</b> u128) * curved_progress) / (<a href="../social_contracts/profile.md#social_contracts_profile_CURVE_PRECISION">CURVE_PRECISION</a> <b>as</b> u128);
    // Subtract already claimed amount to get newly <a href="../social_contracts/profile.md#social_contracts_profile_claimable">claimable</a> amount
    <b>let</b> newly_claimable = (total_claimable <b>as</b> u64) - wallet.claimed_amount;
    // Ensure we don't exceed the remaining balance
    <b>let</b> remaining_balance = balance::value(&wallet.balance);
    <b>if</b> (newly_claimable &gt; remaining_balance) {
        remaining_balance
    } <b>else</b> {
        newly_claimable
    }
}
</code></pre>



</details>

<a name="social_contracts_profile_sqrt_approximation"></a>

## Function `sqrt_approximation`

Simple square root approximation using Newton's method


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_sqrt_approximation">sqrt_approximation</a>(n: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_sqrt_approximation">sqrt_approximation</a>(n: u128): u128 {
    <b>if</b> (n == 0) <b>return</b> 0;
    <b>if</b> (n == 1) <b>return</b> 1;
    <b>let</b> <b>mut</b> x = n;
    <b>let</b> <b>mut</b> y = (x + 1) / 2;
    // Newton's method with limited iterations
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (y &lt; x && i &lt; 10) {
        x = y;
        y = (x + n / x) / 2;
        i = i + 1;
    };
    x
}
</code></pre>



</details>

<a name="social_contracts_profile_delete_vesting_wallet"></a>

## Function `delete_vesting_wallet`

Delete an empty vesting wallet
Can only be called when the wallet balance is zero


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_delete_vesting_wallet">delete_vesting_wallet</a>(wallet: <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_delete_vesting_wallet">delete_vesting_wallet</a>(wallet: <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = tx_context::sender(ctx);
    // Verify sender is the wallet <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
    <b>assert</b>!(wallet.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a> == sender, <a href="../social_contracts/profile.md#social_contracts_profile_ENotVestingWalletOwner">ENotVestingWalletOwner</a>);
    <b>let</b> <a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a> {
        <a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>,
        balance,
        <a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>: _,
        start_time: _,
        claimed_amount: _,
        duration: _,
        total_amount: _,
        curve_factor: _
    } = wallet;
    // Delete the wallet ID
    object::delete(<a href="../social_contracts/profile.md#social_contracts_profile_id">id</a>);
    // Destroy the empty balance
    balance::destroy_zero(balance);
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_balance"></a>

## Function `vesting_balance`

Get the remaining balance in a vesting wallet


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_balance">vesting_balance</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_balance">vesting_balance</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): u64 {
    balance::value(&wallet.balance)
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_owner"></a>

## Function `vesting_owner`

Get the owner of a vesting wallet


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_owner">vesting_owner</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_owner">vesting_owner</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): <b>address</b> {
    wallet.<a href="../social_contracts/profile.md#social_contracts_profile_owner">owner</a>
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_start_time"></a>

## Function `vesting_start_time`

Get the start time of a vesting schedule


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_start_time">vesting_start_time</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_start_time">vesting_start_time</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): u64 {
    wallet.start_time
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_duration"></a>

## Function `vesting_duration`

Get the duration of a vesting schedule


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_duration">vesting_duration</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_duration">vesting_duration</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): u64 {
    wallet.duration
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_total_amount"></a>

## Function `vesting_total_amount`

Get the total amount originally vested


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_total_amount">vesting_total_amount</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_total_amount">vesting_total_amount</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): u64 {
    wallet.total_amount
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_claimed_amount"></a>

## Function `vesting_claimed_amount`

Get the amount already claimed from a vesting wallet


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_claimed_amount">vesting_claimed_amount</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_claimed_amount">vesting_claimed_amount</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): u64 {
    wallet.claimed_amount
}
</code></pre>



</details>

<a name="social_contracts_profile_vesting_curve_factor"></a>

## Function `vesting_curve_factor`

Get the curve factor of a vesting wallet


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_curve_factor">vesting_curve_factor</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">social_contracts::profile::VestingWallet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/profile.md#social_contracts_profile_vesting_curve_factor">vesting_curve_factor</a>(wallet: &<a href="../social_contracts/profile.md#social_contracts_profile_VestingWallet">VestingWallet</a>): u64 {
    wallet.curve_factor
}
</code></pre>



</details>
