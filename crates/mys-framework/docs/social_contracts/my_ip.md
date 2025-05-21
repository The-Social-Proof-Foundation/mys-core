---
title: Module `social_contracts::my_ip`
---

MyIP module for the MySocial network
Manages user-owned intellectual property (IP) objects with flexible licensing options


-  [Struct `MyIP`](#social_contracts_my_ip_MyIP)
-  [Struct `MyIPRegistry`](#social_contracts_my_ip_MyIPRegistry)
-  [Struct `LicenseAdminCap`](#social_contracts_my_ip_LicenseAdminCap)
-  [Struct `LicenseCreatedEvent`](#social_contracts_my_ip_LicenseCreatedEvent)
-  [Struct `LicenseUpdatedEvent`](#social_contracts_my_ip_LicenseUpdatedEvent)
-  [Struct `LicenseTransferredEvent`](#social_contracts_my_ip_LicenseTransferredEvent)
-  [Struct `LicenseStateChangedEvent`](#social_contracts_my_ip_LicenseStateChangedEvent)
-  [Struct `LicenseLinkedEvent`](#social_contracts_my_ip_LicenseLinkedEvent)
-  [Struct `LicenseRegisteredEvent`](#social_contracts_my_ip_LicenseRegisteredEvent)
-  [Constants](#@Constants_0)
-  [Function `init`](#social_contracts_my_ip_init)
-  [Function `create`](#social_contracts_my_ip_create)
-  [Function `create_license`](#social_contracts_my_ip_create_license)
-  [Function `update_license_permissions`](#social_contracts_my_ip_update_license_permissions)
-  [Function `update_revenue_recipient`](#social_contracts_my_ip_update_revenue_recipient)
-  [Function `set_license_state`](#social_contracts_my_ip_set_license_state)
-  [Function `register_license_internal`](#social_contracts_my_ip_register_license_internal)
-  [Function `register_license`](#social_contracts_my_ip_register_license)
-  [Function `update_license_in_registry`](#social_contracts_my_ip_update_license_in_registry)
-  [Function `transfer_license`](#social_contracts_my_ip_transfer_license)
-  [Function `transfer_admin_cap`](#social_contracts_my_ip_transfer_admin_cap)
-  [Function `set_poc_id`](#social_contracts_my_ip_set_poc_id)
-  [Function `is_registered`](#social_contracts_my_ip_is_registered)
-  [Function `registry_has_permission`](#social_contracts_my_ip_registry_has_permission)
-  [Function `registry_is_commenting_allowed`](#social_contracts_my_ip_registry_is_commenting_allowed)
-  [Function `registry_is_reactions_allowed`](#social_contracts_my_ip_registry_is_reactions_allowed)
-  [Function `registry_is_reposting_allowed`](#social_contracts_my_ip_registry_is_reposting_allowed)
-  [Function `registry_is_quoting_allowed`](#social_contracts_my_ip_registry_is_quoting_allowed)
-  [Function `registry_is_tipping_allowed`](#social_contracts_my_ip_registry_is_tipping_allowed)
-  [Function `registry_is_commercial_use_allowed`](#social_contracts_my_ip_registry_is_commercial_use_allowed)
-  [Function `registry_is_derivatives_allowed`](#social_contracts_my_ip_registry_is_derivatives_allowed)
-  [Function `registry_is_public_license`](#social_contracts_my_ip_registry_is_public_license)
-  [Function `registry_is_revenue_redirected`](#social_contracts_my_ip_registry_is_revenue_redirected)
-  [Function `registry_get_revenue_recipient`](#social_contracts_my_ip_registry_get_revenue_recipient)
-  [Function `registry_get_creator`](#social_contracts_my_ip_registry_get_creator)
-  [Function `is_commenting_allowed`](#social_contracts_my_ip_is_commenting_allowed)
-  [Function `is_reactions_allowed`](#social_contracts_my_ip_is_reactions_allowed)
-  [Function `is_reposting_allowed`](#social_contracts_my_ip_is_reposting_allowed)
-  [Function `is_quoting_allowed`](#social_contracts_my_ip_is_quoting_allowed)
-  [Function `is_tipping_allowed`](#social_contracts_my_ip_is_tipping_allowed)
-  [Function `is_commercial_use_allowed`](#social_contracts_my_ip_is_commercial_use_allowed)
-  [Function `is_derivatives_allowed`](#social_contracts_my_ip_is_derivatives_allowed)
-  [Function `is_public_license`](#social_contracts_my_ip_is_public_license)
-  [Function `is_authority_required`](#social_contracts_my_ip_is_authority_required)
-  [Function `is_share_alike_required`](#social_contracts_my_ip_is_share_alike_required)
-  [Function `is_attribution_required`](#social_contracts_my_ip_is_attribution_required)
-  [Function `is_revenue_redirected`](#social_contracts_my_ip_is_revenue_redirected)
-  [Function `has_permission`](#social_contracts_my_ip_has_permission)
-  [Function `is_expired`](#social_contracts_my_ip_is_expired)
-  [Function `validate_license_for_operation`](#social_contracts_my_ip_validate_license_for_operation)
-  [Function `creator`](#social_contracts_my_ip_creator)
-  [Function `name`](#social_contracts_my_ip_name)
-  [Function `description`](#social_contracts_my_ip_description)
-  [Function `creation_time`](#social_contracts_my_ip_creation_time)
-  [Function `license_type`](#social_contracts_my_ip_license_type)
-  [Function `permission_flags`](#social_contracts_my_ip_permission_flags)
-  [Function `license_state`](#social_contracts_my_ip_license_state)
-  [Function `proof_of_creativity_id`](#social_contracts_my_ip_proof_of_creativity_id)
-  [Function `custom_license_uri`](#social_contracts_my_ip_custom_license_uri)
-  [Function `revenue_recipient`](#social_contracts_my_ip_revenue_recipient)
-  [Function `is_transferable`](#social_contracts_my_ip_is_transferable)
-  [Function `expires_at`](#social_contracts_my_ip_expires_at)
-  [Function `id`](#social_contracts_my_ip_id)
-  [Function `id_address`](#social_contracts_my_ip_id_address)
-  [Function `cc0_license_flags`](#social_contracts_my_ip_cc0_license_flags)
-  [Function `cc_by_license_flags`](#social_contracts_my_ip_cc_by_license_flags)
-  [Function `cc_by_sa_license_flags`](#social_contracts_my_ip_cc_by_sa_license_flags)
-  [Function `cc_by_nc_license_flags`](#social_contracts_my_ip_cc_by_nc_license_flags)
-  [Function `cc_by_nc_sa_license_flags`](#social_contracts_my_ip_cc_by_nc_sa_license_flags)
-  [Function `cc_by_nd_license_flags`](#social_contracts_my_ip_cc_by_nd_license_flags)
-  [Function `personal_use_license_flags`](#social_contracts_my_ip_personal_use_license_flags)
-  [Function `token_bound_license_flags`](#social_contracts_my_ip_token_bound_license_flags)
-  [Function `private_license_flags`](#social_contracts_my_ip_private_license_flags)
-  [Function `add_revenue_redirection`](#social_contracts_my_ip_add_revenue_redirection)
-  [Function `version`](#social_contracts_my_ip_version)
-  [Function `borrow_version_mut`](#social_contracts_my_ip_borrow_version_mut)
-  [Function `registry_version`](#social_contracts_my_ip_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_my_ip_borrow_registry_version_mut)
-  [Function `migrate_my_ip`](#social_contracts_my_ip_migrate_my_ip)
-  [Function `migrate_registry`](#social_contracts_my_ip_migrate_registry)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
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
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_my_ip_MyIP"></a>

## Struct `MyIP`

Intellectual property object with enhanced licensing capabilities


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Basic metadata
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: u8</code>
</dt>
<dd>
 License properties
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Optional fields
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_custom_license_uri">custom_license_uri</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>transferable: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_MyIPRegistry"></a>

## Struct `MyIPRegistry`

Registry for MyIP licenses and their permissions


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>permissions: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Maps license IDs to their permissions bitmap
</dd>
<dt>
<code>license_types: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u8&gt;</code>
</dt>
<dd>
 Maps license IDs to their license types
</dd>
<dt>
<code>revenue_recipients: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
 Maps license IDs to revenue recipients (if redirected)
</dd>
<dt>
<code>states: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u8&gt;</code>
</dt>
<dd>
 Maps license IDs to license states (active, expired, revoked)
</dd>
<dt>
<code>creators: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
 Maps license IDs to their creators
</dd>
<dt>
<code>expirations: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Maps license IDs to expiration timestamps
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseAdminCap"></a>

## Struct `LicenseAdminCap`

License capability to manage licenses
This capability grants permission to modify specific licenses


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseCreatedEvent"></a>

## Struct `LicenseCreatedEvent`

Events
Event emitted when a new license is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseCreatedEvent">LicenseCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseUpdatedEvent"></a>

## Struct `LicenseUpdatedEvent`

Event emitted when a license is updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseUpdatedEvent">LicenseUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>updater: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>old_permission_flags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_permission_flags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>update_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseTransferredEvent"></a>

## Struct `LicenseTransferredEvent`

Event emitted when a license is transferred


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseTransferredEvent">LicenseTransferredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>from: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>to: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>transfer_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseStateChangedEvent"></a>

## Struct `LicenseStateChangedEvent`

Event emitted when a license state changes


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseStateChangedEvent">LicenseStateChangedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>old_state: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>new_state: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>changer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>change_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseLinkedEvent"></a>

## Struct `LicenseLinkedEvent`

Event emitted when a license is linked to a post


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseLinkedEvent">LicenseLinkedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>linker: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>link_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_my_ip_LicenseRegisteredEvent"></a>

## Struct `LicenseRegisteredEvent`

Event emitted when a license is registered in the registry


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseRegisteredEvent">LicenseRegisteredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>license_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>registry_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_my_ip_EInvalidLicenseState"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseState">EInvalidLicenseState</a>: u64 = 4;
</code></pre>



<a name="social_contracts_my_ip_EInvalidLicenseType"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseType">EInvalidLicenseType</a>: u64 = 1;
</code></pre>



<a name="social_contracts_my_ip_EInvalidPermission"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidPermission">EInvalidPermission</a>: u64 = 2;
</code></pre>



<a name="social_contracts_my_ip_ELicenseNonTransferable"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_ELicenseNonTransferable">ELicenseNonTransferable</a>: u64 = 3;
</code></pre>



<a name="social_contracts_my_ip_ELicenseNotRegistered"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_ELicenseNotRegistered">ELicenseNotRegistered</a>: u64 = 6;
</code></pre>



<a name="social_contracts_my_ip_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_my_ip_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EWrongVersion">EWrongVersion</a>: u64 = 5;
</code></pre>



<a name="social_contracts_my_ip_LICENSE_STATE_ACTIVE"></a>

License states


<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>: u8 = 0;
</code></pre>



<a name="social_contracts_my_ip_LICENSE_STATE_EXPIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_EXPIRED">LICENSE_STATE_EXPIRED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_my_ip_LICENSE_STATE_REVOKED"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_REVOKED">LICENSE_STATE_REVOKED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_my_ip_LICENSE_TYPE_CREATIVE_COMMONS"></a>

License types


<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_CREATIVE_COMMONS">LICENSE_TYPE_CREATIVE_COMMONS</a>: u8 = 0;
</code></pre>



<a name="social_contracts_my_ip_LICENSE_TYPE_CUSTOM"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_CUSTOM">LICENSE_TYPE_CUSTOM</a>: u8 = 2;
</code></pre>



<a name="social_contracts_my_ip_LICENSE_TYPE_TOKEN_BOUND"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_TOKEN_BOUND">LICENSE_TYPE_TOKEN_BOUND</a>: u8 = 1;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS"></a>

Social interaction permissions - for controlling post interactions


<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a>: u64 = 1024;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_ALLOW_QUOTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a>: u64 = 8192;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a>: u64 = 4096;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_ALLOW_TIPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>: u64 = 16384;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_AUTHORITY_REQUIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_AUTHORITY_REQUIRED">PERMISSION_AUTHORITY_REQUIRED</a>: u64 = 8;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_COMMERCIAL_USE"></a>

Permission flags (stored as bits in a u64)


<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a>: u64 = 1;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a>: u64 = 2;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a>: u64 = 4;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a>: u64 = 32;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>: u64 = 64;
</code></pre>



<a name="social_contracts_my_ip_PERMISSION_SHARE_ALIKE"></a>



<pre><code><b>const</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_SHARE_ALIKE">PERMISSION_SHARE_ALIKE</a>: u64 = 16;
</code></pre>



<a name="social_contracts_my_ip_init"></a>

## Function `init`

Module initialization


<pre><code><b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_init">init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_init">init</a>(ctx: &<b>mut</b> TxContext) {
    // Create and share the registry
    <b>let</b> registry = <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a> {
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>: object::new(ctx),
        permissions: table::new(ctx),
        license_types: table::new(ctx),
        revenue_recipients: table::new(ctx),
        states: table::new(ctx),
        creators: table::new(ctx),
        expirations: table::new(ctx),
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Share the registry
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_my_ip_create"></a>

## Function `create`

Create a new IP object with license


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_create">create</a>(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: u8, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, custom_license_uri_bytes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, transferable: bool, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_create">create</a>(
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>: String,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>: String,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: u8,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>: Option&lt;<b>address</b>&gt;,
    <b>mut</b> custom_license_uri_bytes: Option&lt;vector&lt;u8&gt;&gt;,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>: Option&lt;<b>address</b>&gt;,
    transferable: bool,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a> {
    // Validate license type
    <b>assert</b>!(
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_CREATIVE_COMMONS">LICENSE_TYPE_CREATIVE_COMMONS</a> ||
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_TOKEN_BOUND">LICENSE_TYPE_TOKEN_BOUND</a> ||
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_CUSTOM">LICENSE_TYPE_CUSTOM</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseType">EInvalidLicenseType</a>
    );
    // For custom licenses, require a custom URI
    <b>if</b> (<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_TYPE_CUSTOM">LICENSE_TYPE_CUSTOM</a>) {
        <b>assert</b>!(option::is_some(&custom_license_uri_bytes), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseType">EInvalidLicenseType</a>);
    };
    // If revenue redirection is enabled, require a recipient
    <b>if</b> ((<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>) != 0) {
        <b>assert</b>!(option::is_some(&<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidPermission">EInvalidPermission</a>);
    };
    // Convert URI bytes to URL object <b>if</b> provided
    <b>let</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_custom_license_uri">custom_license_uri</a> = <b>if</b> (option::is_some(&custom_license_uri_bytes)) {
        <b>let</b> uri_bytes = option::extract(&<b>mut</b> custom_license_uri_bytes);
        option::some(url::new_unsafe_from_bytes(uri_bytes))
    } <b>else</b> {
        option::none&lt;Url&gt;()
    };
    <b>let</b> license = <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a> {
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>: object::new(ctx),
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>: tx_context::sender(ctx),
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>: tx_context::epoch_timestamp_ms(ctx),
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>: <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_custom_license_uri">custom_license_uri</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>,
        transferable,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    // Emit license created event
    event::emit(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseCreatedEvent">LicenseCreatedEvent</a> {
        license_id,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>,
    });
    license
}
</code></pre>



</details>

<a name="social_contracts_my_ip_create_license"></a>

## Function `create_license`

Create and register a new IP license transferring to creator


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_create_license">create_license</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, creator_profile: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: u8, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, custom_license_uri_bytes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, transferable: bool, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_create_license">create_license</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    creator_profile: &Profile,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>: String,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>: String,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>: u8,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: u64,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>: Option&lt;<b>address</b>&gt;,
    custom_license_uri_bytes: Option&lt;vector&lt;u8&gt;&gt;,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>: Option&lt;<b>address</b>&gt;,
    transferable: bool,
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller owns the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile_owner">profile::owner</a>(creator_profile), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>let</b> license = <a href="../social_contracts/my_ip.md#social_contracts_my_ip_create">create</a>(
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>,
        custom_license_uri_bytes,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>,
        transferable,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>,
        ctx
    );
    // Create admin capability
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    <b>let</b> admin_cap = <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a> {
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>: object::new(ctx),
        license_id,
        admin: tx_context::sender(ctx),
    };
    // Register in the registry
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_register_license_internal">register_license_internal</a>(registry, &license);
    // Transfer license and capability to <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>
    transfer::transfer(license, tx_context::sender(ctx));
    transfer::transfer(admin_cap, tx_context::sender(ctx));
}
</code></pre>



</details>

<a name="social_contracts_my_ip_update_license_permissions"></a>

## Function `update_license_permissions`

Update license permissions


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_update_license_permissions">update_license_permissions</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">social_contracts::my_ip::LicenseAdminCap</a>, new_permission_flags: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_update_license_permissions">update_license_permissions</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a>,
    new_permission_flags: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Verify admin capability
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    <b>assert</b>!(admin_cap.license_id == license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(admin_cap.admin == tx_context::sender(ctx), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    // Verify license is active
    <b>assert</b>!(license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseState">EInvalidLicenseState</a>);
    <b>let</b> old_flags = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>;
    license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> = new_permission_flags;
    // Update in registry <b>if</b> present
    <b>if</b> (table::contains(&registry.permissions, license_id)) {
        *table::borrow_mut(&<b>mut</b> registry.permissions, license_id) = new_permission_flags;
        // Update revenue recipient info <b>if</b> needed
        <b>if</b> ((new_permission_flags & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>) != 0) {
            <b>if</b> (option::is_some(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>)) {
                <b>let</b> recipient = *option::borrow(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>);
                <b>if</b> (table::contains(&registry.revenue_recipients, license_id)) {
                    *table::borrow_mut(&<b>mut</b> registry.revenue_recipients, license_id) = recipient;
                } <b>else</b> {
                    table::add(&<b>mut</b> registry.revenue_recipients, license_id, recipient);
                }
            }
        } <b>else</b> {
            // Remove revenue recipient <b>if</b> redirection is turned off
            <b>if</b> (table::contains(&registry.revenue_recipients, license_id)) {
                table::remove(&<b>mut</b> registry.revenue_recipients, license_id);
            }
        }
    };
    // Emit license updated event
    event::emit(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseUpdatedEvent">LicenseUpdatedEvent</a> {
        license_id,
        updater: tx_context::sender(ctx),
        old_permission_flags: old_flags,
        new_permission_flags: new_permission_flags,
        update_time: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_my_ip_update_revenue_recipient"></a>

## Function `update_revenue_recipient`

Update revenue recipient


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_update_revenue_recipient">update_revenue_recipient</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">social_contracts::my_ip::LicenseAdminCap</a>, new_recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_update_revenue_recipient">update_revenue_recipient</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a>,
    new_recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Verify admin capability
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    <b>assert</b>!(admin_cap.license_id == license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(admin_cap.admin == tx_context::sender(ctx), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    // Verify license is active
    <b>assert</b>!(license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseState">EInvalidLicenseState</a>);
    // Update recipient
    license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a> = option::some(new_recipient);
    // Ensure revenue redirect flag is set
    license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> | <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>;
    // Update in registry <b>if</b> present
    <b>if</b> (table::contains(&registry.permissions, license_id)) {
        *table::borrow_mut(&<b>mut</b> registry.permissions, license_id) = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>;
        // Update revenue recipient
        <b>if</b> (table::contains(&registry.revenue_recipients, license_id)) {
            *table::borrow_mut(&<b>mut</b> registry.revenue_recipients, license_id) = new_recipient;
        } <b>else</b> {
            table::add(&<b>mut</b> registry.revenue_recipients, license_id, new_recipient);
        }
    };
    // Emit license updated event
    event::emit(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseUpdatedEvent">LicenseUpdatedEvent</a> {
        license_id,
        updater: tx_context::sender(ctx),
        old_permission_flags: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>,
        new_permission_flags: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>,
        update_time: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_my_ip_set_license_state"></a>

## Function `set_license_state`

Set license state (active, expired, revoked)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_set_license_state">set_license_state</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">social_contracts::my_ip::LicenseAdminCap</a>, new_state: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_set_license_state">set_license_state</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a>,
    new_state: u8,
    ctx: &<b>mut</b> TxContext
) {
    // Verify admin capability
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    <b>assert</b>!(admin_cap.license_id == license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(admin_cap.admin == tx_context::sender(ctx), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    // Validate state
    <b>assert</b>!(
        new_state == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a> ||
        new_state == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_EXPIRED">LICENSE_STATE_EXPIRED</a> ||
        new_state == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_REVOKED">LICENSE_STATE_REVOKED</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseState">EInvalidLicenseState</a>
    );
    <b>let</b> old_state = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>;
    license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> = new_state;
    // Update in registry <b>if</b> present
    <b>if</b> (table::contains(&registry.states, license_id)) {
        *table::borrow_mut(&<b>mut</b> registry.states, license_id) = new_state;
    };
    // Emit license state changed event
    event::emit(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseStateChangedEvent">LicenseStateChangedEvent</a> {
        license_id,
        old_state,
        new_state,
        changer: tx_context::sender(ctx),
        change_time: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_my_ip_register_license_internal"></a>

## Function `register_license_internal`

Internal function to register a license in the registry


<pre><code><b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_register_license_internal">register_license_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_register_license_internal">register_license_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>) {
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    // Store license info in registry tables
    table::add(&<b>mut</b> registry.permissions, license_id, license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>);
    table::add(&<b>mut</b> registry.license_types, license_id, license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>);
    table::add(&<b>mut</b> registry.states, license_id, license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>);
    table::add(&<b>mut</b> registry.creators, license_id, license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>);
    // Add revenue recipient <b>if</b> set
    <b>if</b> (option::is_some(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>)) {
        <b>let</b> recipient = *option::borrow(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>);
        table::add(&<b>mut</b> registry.revenue_recipients, license_id, recipient);
    };
    // Add expiration time <b>if</b> set
    <b>if</b> (option::is_some(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>)) {
        <b>let</b> expires = *option::borrow(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>);
        table::add(&<b>mut</b> registry.expirations, license_id, expires);
    };
    // Emit license registered event
    event::emit(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseRegisteredEvent">LicenseRegisteredEvent</a> {
        license_id,
        registry_id: object::uid_to_address(&registry.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>),
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>,
        <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>: license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_my_ip_register_license"></a>

## Function `register_license`

Register an existing license in the registry (for admin use)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_register_license">register_license</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_register_license">register_license</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    ctx: &<b>mut</b> TxContext
) {
    // Only <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a> or admin can register
    <b>assert</b>!(tx_context::sender(ctx) == license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    // Register the license
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_register_license_internal">register_license_internal</a>(registry, license);
}
</code></pre>



</details>

<a name="social_contracts_my_ip_update_license_in_registry"></a>

## Function `update_license_in_registry`

Update a license in the registry (for keeping registry synchronized)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_update_license_in_registry">update_license_in_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_update_license_in_registry">update_license_in_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    ctx: &<b>mut</b> TxContext
) {
    // Only <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a> can update
    <b>assert</b>!(tx_context::sender(ctx) == license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    // Verify license is in registry
    <b>assert</b>!(table::contains(&registry.permissions, license_id), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_ELicenseNotRegistered">ELicenseNotRegistered</a>);
    // Update registry information
    *table::borrow_mut(&<b>mut</b> registry.permissions, license_id) = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>;
    *table::borrow_mut(&<b>mut</b> registry.license_types, license_id) = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>;
    *table::borrow_mut(&<b>mut</b> registry.states, license_id) = license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>;
    // Update revenue recipient <b>if</b> needed
    <b>if</b> (option::is_some(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>)) {
        <b>let</b> recipient = *option::borrow(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>);
        <b>if</b> (table::contains(&registry.revenue_recipients, license_id)) {
            *table::borrow_mut(&<b>mut</b> registry.revenue_recipients, license_id) = recipient;
        } <b>else</b> {
            table::add(&<b>mut</b> registry.revenue_recipients, license_id, recipient);
        }
    } <b>else</b> <b>if</b> (table::contains(&registry.revenue_recipients, license_id)) {
        table::remove(&<b>mut</b> registry.revenue_recipients, license_id);
    };
    // Update expiration <b>if</b> needed
    <b>if</b> (option::is_some(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>)) {
        <b>let</b> expires = *option::borrow(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>);
        <b>if</b> (table::contains(&registry.expirations, license_id)) {
            *table::borrow_mut(&<b>mut</b> registry.expirations, license_id) = expires;
        } <b>else</b> {
            table::add(&<b>mut</b> registry.expirations, license_id, expires);
        }
    } <b>else</b> <b>if</b> (table::contains(&registry.expirations, license_id)) {
        table::remove(&<b>mut</b> registry.expirations, license_id);
    };
}
</code></pre>



</details>

<a name="social_contracts_my_ip_transfer_license"></a>

## Function `transfer_license`

Transfer license to a new owner


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_transfer_license">transfer_license</a>(license: <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">social_contracts::my_ip::LicenseAdminCap</a>, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_transfer_license">transfer_license</a>(
    license: <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a>,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Verify admin capability and transferability
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    <b>assert</b>!(admin_cap.license_id == license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(admin_cap.admin == tx_context::sender(ctx), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(license.transferable, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_ELicenseNonTransferable">ELicenseNonTransferable</a>);
    // Verify license is active
    <b>assert</b>!(license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> == <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EInvalidLicenseState">EInvalidLicenseState</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    // Emit license transferred event
    event::emit(<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseTransferredEvent">LicenseTransferredEvent</a> {
        license_id,
        from: sender,
        to: recipient,
        transfer_time: tx_context::epoch_timestamp_ms(ctx),
    });
    // Transfer license to recipient
    transfer::transfer(license, recipient);
}
</code></pre>



</details>

<a name="social_contracts_my_ip_transfer_admin_cap"></a>

## Function `transfer_admin_cap`

Transfer admin capability to a new admin


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_transfer_admin_cap">transfer_admin_cap</a>(admin_cap: <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">social_contracts::my_ip::LicenseAdminCap</a>, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_transfer_admin_cap">transfer_admin_cap</a>(
    admin_cap: <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a>,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(admin_cap.admin == tx_context::sender(ctx), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    transfer::transfer(admin_cap, recipient);
}
</code></pre>



</details>

<a name="social_contracts_my_ip_set_poc_id"></a>

## Function `set_poc_id`

Set proof of creativity ID


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_set_poc_id">set_poc_id</a>(license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">social_contracts::my_ip::LicenseAdminCap</a>, poc_id: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_set_poc_id">set_poc_id</a>(
    license: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    admin_cap: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_LicenseAdminCap">LicenseAdminCap</a>,
    poc_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Verify admin capability
    <b>let</b> license_id = object::uid_to_address(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>);
    <b>assert</b>!(admin_cap.license_id == license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(admin_cap.admin == tx_context::sender(ctx), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EUnauthorized">EUnauthorized</a>);
    license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a> = option::some(poc_id);
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_registered"></a>

## Function `is_registered`

Check if a license is registered in the registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_registered">is_registered</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_registered">is_registered</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>): bool {
    table::contains(&registry.permissions, license_id)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_has_permission"></a>

## Function `registry_has_permission`

Check if a specific permission is granted for a license


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, permission: u64, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, permission: u64, ctx: &TxContext): bool {
    <b>if</b> (!table::contains(&registry.permissions, license_id)) <b>return</b> <b>false</b>;
    <b>if</b> (!table::contains(&registry.states, license_id)) <b>return</b> <b>false</b>;
    // Check license state first
    <b>let</b> state = *table::borrow(&registry.states, license_id);
    <b>if</b> (state != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) <b>return</b> <b>false</b>;
    // Check <b>for</b> expiration
    <b>if</b> (table::contains(&registry.expirations, license_id)) {
        <b>let</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a> = *table::borrow(&registry.expirations, license_id);
        <b>let</b> current = tx_context::epoch_timestamp_ms(ctx);
        <b>if</b> (current &gt;= <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>) <b>return</b> <b>false</b>;
    };
    // Check specific permission
    <b>let</b> permissions = *table::borrow(&registry.permissions, license_id);
    (permissions & permission) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_commenting_allowed"></a>

## Function `registry_is_commenting_allowed`

Check if commenting is allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_commenting_allowed">registry_is_commenting_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_commenting_allowed">registry_is_commenting_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_reactions_allowed"></a>

## Function `registry_is_reactions_allowed`

Check if reactions/likes are allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_reactions_allowed">registry_is_reactions_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_reactions_allowed">registry_is_reactions_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_reposting_allowed"></a>

## Function `registry_is_reposting_allowed`

Check if reposting is allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_reposting_allowed">registry_is_reposting_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_reposting_allowed">registry_is_reposting_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_quoting_allowed"></a>

## Function `registry_is_quoting_allowed`

Check if quote posting is allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_quoting_allowed">registry_is_quoting_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_quoting_allowed">registry_is_quoting_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_tipping_allowed"></a>

## Function `registry_is_tipping_allowed`

Check if tipping is allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_tipping_allowed">registry_is_tipping_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_tipping_allowed">registry_is_tipping_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_commercial_use_allowed"></a>

## Function `registry_is_commercial_use_allowed`

Check if commercial use is allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_commercial_use_allowed">registry_is_commercial_use_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_commercial_use_allowed">registry_is_commercial_use_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_derivatives_allowed"></a>

## Function `registry_is_derivatives_allowed`

Check if derivatives are allowed (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_derivatives_allowed">registry_is_derivatives_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_derivatives_allowed">registry_is_derivatives_allowed</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_public_license"></a>

## Function `registry_is_public_license`

Check if it's a public license (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_public_license">registry_is_public_license</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_public_license">registry_is_public_license</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_is_revenue_redirected"></a>

## Function `registry_is_revenue_redirected`

Check if revenue is redirected (registry version)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_revenue_redirected">registry_is_revenue_redirected</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_is_revenue_redirected">registry_is_revenue_redirected</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>, ctx: &TxContext): bool {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_has_permission">registry_has_permission</a>(registry, license_id, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>, ctx)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_get_revenue_recipient"></a>

## Function `registry_get_revenue_recipient`

Get revenue recipient from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_get_revenue_recipient">registry_get_revenue_recipient</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_get_revenue_recipient">registry_get_revenue_recipient</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>): <b>address</b> {
    <b>assert</b>!(table::contains(&registry.revenue_recipients, license_id), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_ELicenseNotRegistered">ELicenseNotRegistered</a>);
    *table::borrow(&registry.revenue_recipients, license_id)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_get_creator"></a>

## Function `registry_get_creator`

Get creator from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_get_creator">registry_get_creator</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, license_id: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_get_creator">registry_get_creator</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>, license_id: <b>address</b>): <b>address</b> {
    <b>assert</b>!(table::contains(&registry.creators, license_id), <a href="../social_contracts/my_ip.md#social_contracts_my_ip_ELicenseNotRegistered">ELicenseNotRegistered</a>);
    *table::borrow(&registry.creators, license_id)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_commenting_allowed"></a>

## Function `is_commenting_allowed`

Check if commenting is allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_commenting_allowed">is_commenting_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_commenting_allowed">is_commenting_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_reactions_allowed"></a>

## Function `is_reactions_allowed`

Check if reactions/likes are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_reactions_allowed">is_reactions_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_reactions_allowed">is_reactions_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_reposting_allowed"></a>

## Function `is_reposting_allowed`

Check if reposting is allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_reposting_allowed">is_reposting_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_reposting_allowed">is_reposting_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_quoting_allowed"></a>

## Function `is_quoting_allowed`

Check if quote posting is allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_quoting_allowed">is_quoting_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_quoting_allowed">is_quoting_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_tipping_allowed"></a>

## Function `is_tipping_allowed`

Check if tipping is allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_tipping_allowed">is_tipping_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_tipping_allowed">is_tipping_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_commercial_use_allowed"></a>

## Function `is_commercial_use_allowed`

Check if commercial use is allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_commercial_use_allowed">is_commercial_use_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_commercial_use_allowed">is_commercial_use_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_derivatives_allowed"></a>

## Function `is_derivatives_allowed`

Check if derivatives are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_derivatives_allowed">is_derivatives_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_derivatives_allowed">is_derivatives_allowed</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_public_license"></a>

## Function `is_public_license`

Check if it's a public license


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_public_license">is_public_license</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_public_license">is_public_license</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_authority_required"></a>

## Function `is_authority_required`

Check if authority is required


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_authority_required">is_authority_required</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_authority_required">is_authority_required</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_AUTHORITY_REQUIRED">PERMISSION_AUTHORITY_REQUIRED</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_share_alike_required"></a>

## Function `is_share_alike_required`

Check if share-alike is required


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_share_alike_required">is_share_alike_required</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_share_alike_required">is_share_alike_required</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_SHARE_ALIKE">PERMISSION_SHARE_ALIKE</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_attribution_required"></a>

## Function `is_attribution_required`

Check if attribution is required


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_attribution_required">is_attribution_required</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_attribution_required">is_attribution_required</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_revenue_redirected"></a>

## Function `is_revenue_redirected`

Check if revenue is redirected


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_revenue_redirected">is_revenue_redirected</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_revenue_redirected">is_revenue_redirected</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_has_permission"></a>

## Function `has_permission`

Check if a specific permission is granted


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_has_permission">has_permission</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, permission: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_has_permission">has_permission</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>, permission: u64): bool {
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a> & permission) != 0
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_expired"></a>

## Function `is_expired`

Check if license has expired


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_expired">is_expired</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, current_epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_expired">is_expired</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>, current_epoch: u64): bool {
    <b>if</b> (option::is_some(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>)) {
        <b>let</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a> = option::borrow(&license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>);
        <b>return</b> current_epoch &gt;= *<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_validate_license_for_operation"></a>

## Function `validate_license_for_operation`

Validate license for a specific operation


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_validate_license_for_operation">validate_license_for_operation</a>(license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, required_permission: u64, current_epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_validate_license_for_operation">validate_license_for_operation</a>(
    license: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    required_permission: u64,
    current_epoch: u64
): bool {
    // Check license state
    <b>if</b> (license.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a> != <a href="../social_contracts/my_ip.md#social_contracts_my_ip_LICENSE_STATE_ACTIVE">LICENSE_STATE_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    // Check expiration
    <b>if</b> (<a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_expired">is_expired</a>(license, current_epoch)) {
        <b>return</b> <b>false</b>
    };
    // Check permission
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_has_permission">has_permission</a>(license, required_permission)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_creator"></a>

## Function `creator`

Get creator of the IP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): <b>address</b> {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creator">creator</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_name"></a>

## Function `name`

Get name of the IP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): String {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_name">name</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_description"></a>

## Function `description`

Get description of the IP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): String {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_description">description</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_creation_time"></a>

## Function `creation_time`

Get creation time of the IP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): u64 {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_creation_time">creation_time</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_license_type"></a>

## Function `license_type`

Get license type


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): u8 {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_type">license_type</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_permission_flags"></a>

## Function `permission_flags`

Get permission flags


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): u64 {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_permission_flags">permission_flags</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_license_state"></a>

## Function `license_state`

Get license state


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): u8 {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_license_state">license_state</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_proof_of_creativity_id"></a>

## Function `proof_of_creativity_id`

Get proof of creativity ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): &Option&lt;<b>address</b>&gt; {
    &ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_proof_of_creativity_id">proof_of_creativity_id</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_custom_license_uri"></a>

## Function `custom_license_uri`

Get custom license URI


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_custom_license_uri">custom_license_uri</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_custom_license_uri">custom_license_uri</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): &Option&lt;Url&gt; {
    &ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_custom_license_uri">custom_license_uri</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_revenue_recipient"></a>

## Function `revenue_recipient`

Get revenue recipient


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): &Option&lt;<b>address</b>&gt; {
    &ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_revenue_recipient">revenue_recipient</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_is_transferable"></a>

## Function `is_transferable`

Is license transferable


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_transferable">is_transferable</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_is_transferable">is_transferable</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): bool {
    ip.transferable
}
</code></pre>



</details>

<a name="social_contracts_my_ip_expires_at"></a>

## Function `expires_at`

Get expiration time


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): &Option&lt;u64&gt; {
    &ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_expires_at">expires_at</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_id"></a>

## Function `id`

Get the ID of the MyIP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): &UID {
    &ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_id_address"></a>

## Function `id_address`

Get the address of the MyIP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id_address">id_address</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_id_address">id_address</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): <b>address</b> {
    object::uid_to_address(&ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_id">id</a>)
}
</code></pre>



</details>

<a name="social_contracts_my_ip_cc0_license_flags"></a>

## Function `cc0_license_flags`

Create a Creative Commons Zero license (CC0 - public domain)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc0_license_flags">cc0_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc0_license_flags">cc0_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_cc_by_license_flags"></a>

## Function `cc_by_license_flags`

Create a Creative Commons BY license (Attribution)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_license_flags">cc_by_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_license_flags">cc_by_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_cc_by_sa_license_flags"></a>

## Function `cc_by_sa_license_flags`

Create a Creative Commons BY-SA license (Attribution-ShareAlike)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_sa_license_flags">cc_by_sa_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_sa_license_flags">cc_by_sa_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_SHARE_ALIKE">PERMISSION_SHARE_ALIKE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_cc_by_nc_license_flags"></a>

## Function `cc_by_nc_license_flags`

Create a Creative Commons BY-NC license (Attribution-NonCommercial)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_nc_license_flags">cc_by_nc_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_nc_license_flags">cc_by_nc_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
    // Note: No COMMERCIAL_USE flag
}
</code></pre>



</details>

<a name="social_contracts_my_ip_cc_by_nc_sa_license_flags"></a>

## Function `cc_by_nc_sa_license_flags`

Create a Creative Commons BY-NC-SA license (Attribution-NonCommercial-ShareAlike)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_nc_sa_license_flags">cc_by_nc_sa_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_nc_sa_license_flags">cc_by_nc_sa_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_DERIVATIVES_ALLOWED">PERMISSION_DERIVATIVES_ALLOWED</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_SHARE_ALIKE">PERMISSION_SHARE_ALIKE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
    // Note: No COMMERCIAL_USE flag
}
</code></pre>



</details>

<a name="social_contracts_my_ip_cc_by_nd_license_flags"></a>

## Function `cc_by_nd_license_flags`

Create a Creative Commons BY-ND license (Attribution-NoDerivatives)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_nd_license_flags">cc_by_nd_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_cc_by_nd_license_flags">cc_by_nd_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
    // Note: No DERIVATIVES_ALLOWED, ALLOW_REPOSTS, ALLOW_QUOTES flags
}
</code></pre>



</details>

<a name="social_contracts_my_ip_personal_use_license_flags"></a>

## Function `personal_use_license_flags`

Create a personal use only license


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_personal_use_license_flags">personal_use_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_personal_use_license_flags">personal_use_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_PUBLIC_LICENSE">PERMISSION_PUBLIC_LICENSE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>
    // Note: No COMMERCIAL_USE, DERIVATIVES_ALLOWED, ALLOW_REPOSTS, ALLOW_QUOTES, ALLOW_TIPS flags
}
</code></pre>



</details>

<a name="social_contracts_my_ip_token_bound_license_flags"></a>

## Function `token_bound_license_flags`

Create a token bound license


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_token_bound_license_flags">token_bound_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_token_bound_license_flags">token_bound_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_COMMERCIAL_USE">PERMISSION_COMMERCIAL_USE</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_AUTHORITY_REQUIRED">PERMISSION_AUTHORITY_REQUIRED</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_private_license_flags"></a>

## Function `private_license_flags`

Create a private license (view only)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_private_license_flags">private_license_flags</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_private_license_flags">private_license_flags</a>(): u64 {
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REQUIRE_ATTRIBUTION">PERMISSION_REQUIRE_ATTRIBUTION</a> |
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>
    // No other permissions allowed
}
</code></pre>



</details>

<a name="social_contracts_my_ip_add_revenue_redirection"></a>

## Function `add_revenue_redirection`

Add revenue redirection to a license


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_add_revenue_redirection">add_revenue_redirection</a>(base_flags: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_add_revenue_redirection">add_revenue_redirection</a>(base_flags: u64): u64 {
    base_flags | <a href="../social_contracts/my_ip.md#social_contracts_my_ip_PERMISSION_REVENUE_REDIRECT">PERMISSION_REVENUE_REDIRECT</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_version"></a>

## Function `version`

=== Versioning Functions ===
Get the version of a MyIP


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>(ip: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): u64 {
    ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_borrow_version_mut"></a>

## Function `borrow_version_mut`

Get a mutable reference to the MyIP version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_borrow_version_mut">borrow_version_mut</a>(ip: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_borrow_version_mut">borrow_version_mut</a>(ip: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>): &<b>mut</b> u64 {
    &<b>mut</b> ip.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_registry_version"></a>

## Function `registry_version`

Get the version of the registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_version">registry_version</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_registry_version">registry_version</a>(registry: &<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>): u64 {
    registry.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_my_ip_migrate_my_ip"></a>

## Function `migrate_my_ip`

Migration function for MyIP


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_migrate_my_ip">migrate_my_ip</a>(<a href="../social_contracts/my_ip.md#social_contracts_my_ip">my_ip</a>: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">social_contracts::my_ip::MyIP</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_migrate_my_ip">migrate_my_ip</a>(
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip">my_ip</a>: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> &gt; current <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>)
    <b>assert</b>!(<a href="../social_contracts/my_ip.md#social_contracts_my_ip">my_ip</a>.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> &lt; current_version, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> and update to new <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>
    <b>let</b> old_version = <a href="../social_contracts/my_ip.md#social_contracts_my_ip">my_ip</a>.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>;
    <a href="../social_contracts/my_ip.md#social_contracts_my_ip">my_ip</a>.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> my_ip_id = object::id(<a href="../social_contracts/my_ip.md#social_contracts_my_ip">my_ip</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        my_ip_id,
        string::utf8(b"<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIP">MyIP</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_my_ip_migrate_registry"></a>

## Function `migrate_registry`

Migration function for MyIPRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">social_contracts::my_ip::MyIPRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_migrate_registry">migrate_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> &gt; current <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>)
    <b>assert</b>!(registry.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> &lt; current_version, <a href="../social_contracts/my_ip.md#social_contracts_my_ip_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> and update to new <a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>
    <b>let</b> old_version = registry.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a>;
    registry.<a href="../social_contracts/my_ip.md#social_contracts_my_ip_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/my_ip.md#social_contracts_my_ip_MyIPRegistry">MyIPRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>
