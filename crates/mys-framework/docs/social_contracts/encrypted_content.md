---
title: Module `social_contracts::encrypted_content`
---

A module for handling encrypted content on MySocial platform
Provides cryptographically secure methods for storing, accessing,
and sharing encrypted content with payment-based access control.


-  [Struct `EncryptedContent`](#social_contracts_encrypted_content_EncryptedContent)
-  [Struct `PaymentTier`](#social_contracts_encrypted_content_PaymentTier)
-  [Struct `AccessGrant`](#social_contracts_encrypted_content_AccessGrant)
-  [Struct `ContentMetadata`](#social_contracts_encrypted_content_ContentMetadata)
-  [Struct `ContentCreatedEvent`](#social_contracts_encrypted_content_ContentCreatedEvent)
-  [Struct `AccessGrantedEvent`](#social_contracts_encrypted_content_AccessGrantedEvent)
-  [Struct `AccessRevokedEvent`](#social_contracts_encrypted_content_AccessRevokedEvent)
-  [Struct `ContentUpdatedEvent`](#social_contracts_encrypted_content_ContentUpdatedEvent)
-  [Struct `TierCreatedEvent`](#social_contracts_encrypted_content_TierCreatedEvent)
-  [Constants](#@Constants_0)
-  [Function `create_encrypted_content`](#social_contracts_encrypted_content_create_encrypted_content)
-  [Function `create_payment_tier`](#social_contracts_encrypted_content_create_payment_tier)
-  [Function `unlock_content_with_payment`](#social_contracts_encrypted_content_unlock_content_with_payment)
-  [Function `grant_free_access`](#social_contracts_encrypted_content_grant_free_access)
-  [Function `revoke_access`](#social_contracts_encrypted_content_revoke_access)
-  [Function `update_content`](#social_contracts_encrypted_content_update_content)
-  [Function `update_content_metadata`](#social_contracts_encrypted_content_update_content_metadata)
-  [Function `link_to_profile`](#social_contracts_encrypted_content_link_to_profile)
-  [Function `verify_access`](#social_contracts_encrypted_content_verify_access)
-  [Function `client_encrypt_access_key`](#social_contracts_encrypted_content_client_encrypt_access_key)
-  [Function `get_encrypted_data`](#social_contracts_encrypted_content_get_encrypted_data)
-  [Function `get_access_key`](#social_contracts_encrypted_content_get_access_key)
-  [Function `has_access`](#social_contracts_encrypted_content_has_access)
-  [Function `owner`](#social_contracts_encrypted_content_owner)
-  [Function `content_type`](#social_contracts_encrypted_content_content_type)
-  [Function `content_hash`](#social_contracts_encrypted_content_content_hash)
-  [Function `public_metadata`](#social_contracts_encrypted_content_public_metadata)
-  [Function `created_at`](#social_contracts_encrypted_content_created_at)
-  [Function `updated_at`](#social_contracts_encrypted_content_updated_at)
-  [Function `get_content_metadata`](#social_contracts_encrypted_content_get_content_metadata)
-  [Function `get_tier_details`](#social_contracts_encrypted_content_get_tier_details)
-  [Function `get_tier_ids`](#social_contracts_encrypted_content_get_tier_ids)
-  [Function `platform_fee_bps`](#social_contracts_encrypted_content_platform_fee_bps)


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



<a name="social_contracts_encrypted_content_EncryptedContent"></a>

## Struct `EncryptedContent`

Primary struct representing encrypted content


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a> <b>has</b> key, store
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
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
 Owner of the content
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>: u8</code>
</dt>
<dd>
 Type of content (post, profile, message, etc.)
</dd>
<dt>
<code>encrypted_data: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted content data
</dd>
<dt>
<code>encryption_scheme: u8</code>
</dt>
<dd>
 Content encryption scheme identifier
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Public metadata visible to everyone
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 Hash of the original content (for integrity verification)
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>: u64</code>
</dt>
<dd>
 Last updated timestamp
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>: u64</code>
</dt>
<dd>
 Platform fee in basis points (e.g., 250 = 2.5%)
</dd>
<dt>
<code>tier_ids: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 List of tier IDs for tracking
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_PaymentTier"></a>

## Struct `PaymentTier`

Payment tier with different access levels and pricing


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tier_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Tier identifier
</dd>
<dt>
<code>price: u64</code>
</dt>
<dd>
 Price in MYS tokens
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Human-readable name for this tier
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Description of what this tier provides
</dd>
<dt>
<code>duration_epochs: u64</code>
</dt>
<dd>
 Duration of access in epochs (0 for permanent)
</dd>
<dt>
<code>encrypted_tier_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted tier key (encrypted with owner's public key)
</dd>
<dt>
<code>tier_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Public key used for content encryption in this tier
</dd>
<dt>
<code>tier_key_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
 Hash of the tier key for verification
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_AccessGrant"></a>

## Struct `AccessGrant`

Information about granted access to content


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
 Address that has access
</dd>
<dt>
<code>encrypted_access_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted access key for this recipient
</dd>
<dt>
<code>granted_at: u64</code>
</dt>
<dd>
 Timestamp when access was granted
</dd>
<dt>
<code>expires_at: u64</code>
</dt>
<dd>
 Expiration timestamp (0 for never)
</dd>
<dt>
<code>tier_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Tier ID this access belongs to
</dd>
<dt>
<code>payment_amount: u64</code>
</dt>
<dd>
 Payment amount
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
 Current status of this access
</dd>
<dt>
<code>nonce: vector&lt;u8&gt;</code>
</dt>
<dd>
 Cryptographic nonce used for this specific access grant
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_ContentMetadata"></a>

## Struct `ContentMetadata`

Content metadata (partially encrypted)


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentMetadata">ContentMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Public title/name
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Public description/preview
</dd>
<dt>
<code>tags: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Public tags
</dd>
<dt>
<code>public_attributes: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Additional public attributes (serialized as JSON)
</dd>
<dt>
<code>encrypted_attributes_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
 Hash of encrypted attributes
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_ContentCreatedEvent"></a>

## Struct `ContentCreatedEvent`

Event emitted when encrypted content is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentCreatedEvent">ContentCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>content_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_AccessGrantedEvent"></a>

## Struct `AccessGrantedEvent`

Event emitted when access is granted to content


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrantedEvent">AccessGrantedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>content_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>tier_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>payment_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>granted_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_AccessRevokedEvent"></a>

## Struct `AccessRevokedEvent`

Event emitted when access is revoked


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessRevokedEvent">AccessRevokedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>content_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_ContentUpdatedEvent"></a>

## Struct `ContentUpdatedEvent`

Event emitted when content is updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentUpdatedEvent">ContentUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>content_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_encrypted_content_TierCreatedEvent"></a>

## Struct `TierCreatedEvent`

Event emitted when a payment tier is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TierCreatedEvent">TierCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>content_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>tier_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_encrypted_content_ACCESS_KEYS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>: vector&lt;u8&gt; = vector[97, 99, 99, 101, 115, 115, 95, 107, 101, 121, 115];
</code></pre>



<a name="social_contracts_encrypted_content_ACCESS_STATUS_ACTIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_ACTIVE">ACCESS_STATUS_ACTIVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_encrypted_content_ACCESS_STATUS_REVOKED"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_REVOKED">ACCESS_STATUS_REVOKED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_encrypted_content_CONTENT_METADATA_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_CONTENT_METADATA_FIELD">CONTENT_METADATA_FIELD</a>: vector&lt;u8&gt; = vector[99, 111, 110, 116, 101, 110, 116, 95, 109, 101, 116, 97, 100, 97, 116, 97];
</code></pre>



<a name="social_contracts_encrypted_content_CONTENT_TYPE_PROFILE"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_CONTENT_TYPE_PROFILE">CONTENT_TYPE_PROFILE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_encrypted_content_EAccessKeyNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EAccessKeyNotFound">EAccessKeyNotFound</a>: u64 = 6;
</code></pre>



<a name="social_contracts_encrypted_content_EInsufficientPayment"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EInsufficientPayment">EInsufficientPayment</a>: u64 = 3;
</code></pre>



<a name="social_contracts_encrypted_content_EInvalidEncryptionScheme"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EInvalidEncryptionScheme">EInvalidEncryptionScheme</a>: u64 = 8;
</code></pre>



<a name="social_contracts_encrypted_content_ETierNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ETierNotFound">ETierNotFound</a>: u64 = 10;
</code></pre>



<a name="social_contracts_encrypted_content_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="social_contracts_encrypted_content_SIG_FLAG_ED25519"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_SIG_FLAG_ED25519">SIG_FLAG_ED25519</a>: u8 = 0;
</code></pre>



<a name="social_contracts_encrypted_content_SIG_FLAG_SECP256K1"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_SIG_FLAG_SECP256K1">SIG_FLAG_SECP256K1</a>: u8 = 1;
</code></pre>



<a name="social_contracts_encrypted_content_SIG_FLAG_SECP256R1"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_SIG_FLAG_SECP256R1">SIG_FLAG_SECP256R1</a>: u8 = 2;
</code></pre>



<a name="social_contracts_encrypted_content_TIERS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>: vector&lt;u8&gt; = vector[116, 105, 101, 114, 115];
</code></pre>



<a name="social_contracts_encrypted_content_create_encrypted_content"></a>

## Function `create_encrypted_content`

Create new encrypted content


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_create_encrypted_content">create_encrypted_content</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, encrypted_data: vector&lt;u8&gt;, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>: u8, encryption_scheme: u8, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>: vector&lt;u8&gt;, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_create_encrypted_content">create_encrypted_content</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    encrypted_data: vector&lt;u8&gt;,
    <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>: u8,
    encryption_scheme: u8,
    <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>: String,
    <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>: vector&lt;u8&gt;,
    <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Verify sender is the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile_owner">profile::owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>) == sender, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    // Validate encryption scheme
    <b>assert</b>!(
        encryption_scheme == <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_SIG_FLAG_ED25519">SIG_FLAG_ED25519</a> ||
        encryption_scheme == <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_SIG_FLAG_SECP256K1">SIG_FLAG_SECP256K1</a> ||
        encryption_scheme == <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_SIG_FLAG_SECP256R1">SIG_FLAG_SECP256R1</a>,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EInvalidEncryptionScheme">EInvalidEncryptionScheme</a>
    );
    <b>let</b> now = tx_context::epoch(ctx);
    // Create content object
    <b>let</b> <b>mut</b> content = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a> {
        id: object::new(ctx),
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>: sender,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>,
        encrypted_data,
        encryption_scheme,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>: now,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>: now,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>,
        tier_ids: vector::empty(),
    };
    // Initialize access keys table <b>as</b> a dynamic field
    <b>let</b> access_keys = table::new&lt;<b>address</b>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a>&gt;(ctx);
    dynamic_field::add(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>, access_keys);
    // Initialize tiers table
    <b>let</b> tiers = table::new&lt;String, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a>&gt;(ctx);
    dynamic_field::add(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>, tiers);
    // Initialize content metadata
    <b>let</b> metadata = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentMetadata">ContentMetadata</a> {
        title: string::utf8(b""),
        description: string::utf8(b""),
        tags: vector::empty(),
        public_attributes: string::utf8(b"{}"),
        encrypted_attributes_hash: vector::empty(),
    };
    dynamic_field::add(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_CONTENT_METADATA_FIELD">CONTENT_METADATA_FIELD</a>, metadata);
    <b>let</b> content_id = object::uid_to_address(&content.id);
    // If this is <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> content, link it to the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>if</b> (<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a> == <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_CONTENT_TYPE_PROFILE">CONTENT_TYPE_PROFILE</a>) {
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_link_to_profile">link_to_profile</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>, content_id, ctx);
    };
    // Emit content creation event
    event::emit(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentCreatedEvent">ContentCreatedEvent</a> {
        content_id,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>: sender,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>: now,
    });
    // Transfer content object to <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>
    transfer::transfer(content, sender);
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_create_payment_tier"></a>

## Function `create_payment_tier`

Create a new payment tier for content


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_create_payment_tier">create_payment_tier</a>(content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, tier_id: <a href="../std/string.md#std_string_String">std::string::String</a>, price: u64, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, duration_epochs: u64, encrypted_tier_key: vector&lt;u8&gt;, tier_public_key: vector&lt;u8&gt;, tier_key_hash: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_create_payment_tier">create_payment_tier</a>(
    content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    tier_id: String,
    price: u64,
    name: String,
    description: String,
    duration_epochs: u64,
    encrypted_tier_key: vector&lt;u8&gt;,
    tier_public_key: vector&lt;u8&gt;,
    tier_key_hash: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify sender is the content <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == sender, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Create new payment tier
    <b>let</b> tier = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a> {
        tier_id,
        price,
        name,
        description,
        duration_epochs,
        encrypted_tier_key,
        tier_public_key,
        tier_key_hash,
    };
    // Add tier to the tiers table
    <b>let</b> tiers = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;String, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a>&gt;&gt;(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>);
    <b>assert</b>!(!table::contains(tiers, tier_id), <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ETierNotFound">ETierNotFound</a>); // Tier ID must be unique
    table::add(tiers, tier_id, tier);
    // Add tier_id to the content's tier_ids list <b>for</b> tracking
    vector::push_back(&<b>mut</b> content.tier_ids, tier_id);
    // Emit tier creation event
    event::emit(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TierCreatedEvent">TierCreatedEvent</a> {
        content_id: object::uid_to_address(&content.id),
        tier_id,
        price,
        name,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_unlock_content_with_payment"></a>

## Function `unlock_content_with_payment`

Pay to unlock encrypted content


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_unlock_content_with_payment">unlock_content_with_payment</a>(content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, payment: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, tier_id: <a href="../std/string.md#std_string_String">std::string::String</a>, recipient_public_key: vector&lt;u8&gt;, nonce: vector&lt;u8&gt;, platform_treasury: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_unlock_content_with_payment">unlock_content_with_payment</a>(
    content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    payment: &<b>mut</b> Coin&lt;MYS&gt;,
    tier_id: String,
    recipient_public_key: vector&lt;u8&gt;,
    nonce: vector&lt;u8&gt;,
    platform_treasury: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> now = tx_context::epoch(ctx);
    <b>let</b> content_id = object::uid_to_address(&content.id);
    <b>let</b> content_owner = content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>;
    // Get the tiers table
    <b>let</b> tiers = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;String, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a>&gt;&gt;(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>);
    // Verify tier exists
    <b>assert</b>!(table::contains(tiers, tier_id), <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ETierNotFound">ETierNotFound</a>);
    // Get the payment tier
    <b>let</b> tier = table::borrow(tiers, tier_id);
    // Verify sufficient payment
    <b>let</b> price = tier.price;
    <b>assert</b>!(coin::value(payment) &gt;= price, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EInsufficientPayment">EInsufficientPayment</a>);
    // Split the payment
    <b>let</b> <b>mut</b> paid_coin = coin::split(payment, price, ctx);
    // Calculate <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee
    <b>let</b> platform_fee = (price * content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>) / 10000;
    // If <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee is non-zero, split it off
    <b>if</b> (platform_fee &gt; 0) {
        // Split off <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee and send to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>let</b> platform_fee_coin = coin::split(&<b>mut</b> paid_coin, platform_fee, ctx);
        transfer::public_transfer(platform_fee_coin, platform_treasury);
    };
    // Send remaining payment directly to content <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>
    transfer::public_transfer(paid_coin, content_owner);
    // Generate access key using recipient's <b>public</b> key and tier key
    <b>let</b> encrypted_access_key = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_client_encrypt_access_key">client_encrypt_access_key</a>(
        tier.encrypted_tier_key,
        tier.tier_public_key,
        recipient_public_key,
        nonce,
        content.encryption_scheme
    );
    // Calculate expiration time <b>if</b> applicable
    <b>let</b> expires_at = <b>if</b> (tier.duration_epochs == 0) {
        0 // Never expires
    } <b>else</b> {
        now + tier.duration_epochs
    };
    // Create access grant
    <b>let</b> access_grant = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a> {
        recipient: sender,
        encrypted_access_key,
        granted_at: now,
        expires_at,
        tier_id,
        payment_amount: price,
        status: <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_ACTIVE">ACCESS_STATUS_ACTIVE</a>,
        nonce,
    };
    // Get the access keys table
    <b>let</b> access_keys = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a>&gt;&gt;(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>);
    // Add or update access <b>for</b> the sender
    <b>if</b> (table::contains(access_keys, sender)) {
        // Update existing access
        *table::borrow_mut(access_keys, sender) = access_grant;
    } <b>else</b> {
        // Add new access
        table::add(access_keys, sender, access_grant);
    };
    // Emit access granted event
    event::emit(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrantedEvent">AccessGrantedEvent</a> {
        content_id,
        recipient: sender,
        tier_id,
        payment_amount: price,
        granted_at: now,
        expires_at,
    });
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_grant_free_access"></a>

## Function `grant_free_access`

Grant free access to content (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_grant_free_access">grant_free_access</a>(content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, recipient: <b>address</b>, tier_id: <a href="../std/string.md#std_string_String">std::string::String</a>, recipient_public_key: vector&lt;u8&gt;, nonce: vector&lt;u8&gt;, duration_epochs: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_grant_free_access">grant_free_access</a>(
    content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    recipient: <b>address</b>,
    tier_id: String,
    recipient_public_key: vector&lt;u8&gt;,
    nonce: vector&lt;u8&gt;,
    duration_epochs: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == sender, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    <b>let</b> content_id = object::uid_to_address(&content.id);
    // Get the tiers table
    <b>let</b> tiers = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;String, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a>&gt;&gt;(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>);
    // Verify tier exists
    <b>assert</b>!(table::contains(tiers, tier_id), <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ETierNotFound">ETierNotFound</a>);
    // Get the payment tier
    <b>let</b> tier = table::borrow(tiers, tier_id);
    // Generate access key using recipient's <b>public</b> key and tier key
    <b>let</b> encrypted_access_key = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_client_encrypt_access_key">client_encrypt_access_key</a>(
        tier.encrypted_tier_key,
        tier.tier_public_key,
        recipient_public_key,
        nonce,
        content.encryption_scheme
    );
    // Calculate expiration time <b>if</b> applicable
    <b>let</b> expires_at = <b>if</b> (duration_epochs == 0) {
        0 // Never expires
    } <b>else</b> {
        now + duration_epochs
    };
    // Create access grant with zero payment
    <b>let</b> access_grant = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a> {
        recipient,
        encrypted_access_key,
        granted_at: now,
        expires_at,
        tier_id,
        payment_amount: 0, // Free access
        status: <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_ACTIVE">ACCESS_STATUS_ACTIVE</a>,
        nonce,
    };
    // Get the access keys table
    <b>let</b> access_keys = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a>&gt;&gt;(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>);
    // Add or update access <b>for</b> the recipient
    <b>if</b> (table::contains(access_keys, recipient)) {
        // Update existing access
        *table::borrow_mut(access_keys, recipient) = access_grant;
    } <b>else</b> {
        // Add new access
        table::add(access_keys, recipient, access_grant);
    };
    // Emit access granted event
    event::emit(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrantedEvent">AccessGrantedEvent</a> {
        content_id,
        recipient,
        tier_id,
        payment_amount: 0, // Free access
        granted_at: now,
        expires_at,
    });
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_revoke_access"></a>

## Function `revoke_access`

Revoke access to content (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_revoke_access">revoke_access</a>(content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_revoke_access">revoke_access</a>(
    content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == sender, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Get the access keys table
    <b>let</b> access_keys = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a>&gt;&gt;(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>);
    // Verify recipient <b>has</b> access
    <b>assert</b>!(table::contains(access_keys, recipient), <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EAccessKeyNotFound">EAccessKeyNotFound</a>);
    // Mark access <b>as</b> revoked
    <b>let</b> access_grant = table::borrow_mut(access_keys, recipient);
    access_grant.status = <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_REVOKED">ACCESS_STATUS_REVOKED</a>;
    // Emit access revoked event
    event::emit(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessRevokedEvent">AccessRevokedEvent</a> {
        content_id: object::uid_to_address(&content.id),
        recipient,
        revoked_by: sender,
        revoked_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_update_content"></a>

## Function `update_content`

Update encrypted content (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_update_content">update_content</a>(content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, new_encrypted_data: vector&lt;u8&gt;, new_content_hash: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_update_content">update_content</a>(
    content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    new_encrypted_data: vector&lt;u8&gt;,
    new_content_hash: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == sender, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    <b>let</b> now = tx_context::epoch(ctx);
    // Update content
    content.encrypted_data = new_encrypted_data;
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a> = new_content_hash;
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a> = now;
    // Emit content updated event
    event::emit(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentUpdatedEvent">ContentUpdatedEvent</a> {
        content_id: object::uid_to_address(&content.id),
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>: sender,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>: new_content_hash,
        <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_update_content_metadata"></a>

## Function `update_content_metadata`

Update content metadata


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_update_content_metadata">update_content_metadata</a>(content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, tags: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, public_attributes: <a href="../std/string.md#std_string_String">std::string::String</a>, encrypted_attributes_hash: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_update_content_metadata">update_content_metadata</a>(
    content: &<b>mut</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    title: String,
    description: String,
    tags: vector&lt;String&gt;,
    public_attributes: String,
    encrypted_attributes_hash: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == sender, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    // Get and update the metadata
    <b>let</b> metadata = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentMetadata">ContentMetadata</a>&gt;(&<b>mut</b> content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_CONTENT_METADATA_FIELD">CONTENT_METADATA_FIELD</a>);
    metadata.title = title;
    metadata.description = description;
    metadata.tags = tags;
    metadata.public_attributes = public_attributes;
    metadata.encrypted_attributes_hash = encrypted_attributes_hash;
    // Update the content's last updated timestamp
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a> = tx_context::epoch(ctx);
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_link_to_profile"></a>

## Function `link_to_profile`

Link encrypted content to a profile


<pre><code><b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_link_to_profile">link_to_profile</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, _content_id: <b>address</b>, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_link_to_profile">link_to_profile</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    _content_id: <b>address</b>,
    _ctx: &<b>mut</b> TxContext
) {
    // Since we can't modify <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> directly through <a href="../social_contracts/profile.md#social_contracts_profile_id">profile::id</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>),
    // this function would need to be implemented differently.
    // For now, we'll leave it <b>as</b> a placeholder.
    // In a real implementation, we would need to:
    // 1. Have a separate mapping of profiles to their content
    // 2. Or add a function to the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> <b>module</b> to link content
    // Get <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> logging purposes only
    <b>let</b> _profile_id = <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">profile::get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_verify_access"></a>

## Function `verify_access`

Verify access to encrypted content


<pre><code><b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_verify_access">verify_access</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, user: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_verify_access">verify_access</a>(
    content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    user: <b>address</b>,
    ctx: &TxContext
): bool {
    // Get the access keys table
    <b>if</b> (!dynamic_field::exists_(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> access_keys = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a>&gt;&gt;(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>);
    // Check <b>if</b> user <b>has</b> access
    <b>if</b> (!table::contains(access_keys, user)) {
        <b>return</b> <b>false</b>
    };
    // Get the access grant
    <b>let</b> access = table::borrow(access_keys, user);
    // Check <b>if</b> access is active
    <b>if</b> (access.status != <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_ACTIVE">ACCESS_STATUS_ACTIVE</a>) {
        <b>return</b> <b>false</b>
    };
    // Check <b>if</b> access <b>has</b> expired
    <b>let</b> now = tx_context::epoch(ctx);
    <b>if</b> (access.expires_at != 0 && now &gt; access.expires_at) {
        <b>return</b> <b>false</b>
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_client_encrypt_access_key"></a>

## Function `client_encrypt_access_key`

Client proxy function for access key generation
NOTE: In production, this is a placeholder where clients would provide their own encrypted keys.
This function should be replaced with one that accepts a client-encrypted access key.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_client_encrypt_access_key">client_encrypt_access_key</a>(_encrypted_tier_key: vector&lt;u8&gt;, _tier_public_key: vector&lt;u8&gt;, _recipient_public_key: vector&lt;u8&gt;, nonce: vector&lt;u8&gt;, _encryption_scheme: u8): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_client_encrypt_access_key">client_encrypt_access_key</a>(
    _encrypted_tier_key: vector&lt;u8&gt;,
    _tier_public_key: vector&lt;u8&gt;,
    _recipient_public_key: vector&lt;u8&gt;,
    nonce: vector&lt;u8&gt;,
    _encryption_scheme: u8
): vector&lt;u8&gt; {
    // In production, the client should encrypt access keys and provide them directly
    // This is a temporary placeholder that returns the nonce <b>as</b> a mock "encrypted" key
    // IMPORTANT: Replace this with actual client-provided encrypted keys in production!
    nonce
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_get_encrypted_data"></a>

## Function `get_encrypted_data`

Get the encrypted data if user has access


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_encrypted_data">get_encrypted_data</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_encrypted_data">get_encrypted_data</a>(
    content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    ctx: &TxContext
): vector&lt;u8&gt; {
    <b>let</b> sender = tx_context::sender(ctx);
    // If sender is the <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>, they always have access
    <b>if</b> (content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == sender) {
        <b>return</b> content.encrypted_data
    };
    // Otherwise, verify access
    <b>assert</b>!(<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_verify_access">verify_access</a>(content, sender, ctx), <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EUnauthorized">EUnauthorized</a>);
    content.encrypted_data
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_get_access_key"></a>

## Function `get_access_key`

Get the encrypted access key for a user


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_access_key">get_access_key</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_access_key">get_access_key</a>(
    content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    ctx: &TxContext
): Option&lt;vector&lt;u8&gt;&gt; {
    <b>let</b> sender = tx_context::sender(ctx);
    // If user <b>has</b> no access, <b>return</b> none
    <b>if</b> (!dynamic_field::exists_(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>)) {
        <b>return</b> option::none()
    };
    <b>let</b> access_keys = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;<b>address</b>, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_AccessGrant">AccessGrant</a>&gt;&gt;(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_KEYS_FIELD">ACCESS_KEYS_FIELD</a>);
    <b>if</b> (!table::contains(access_keys, sender)) {
        <b>return</b> option::none()
    };
    <b>let</b> access = table::borrow(access_keys, sender);
    // Check <b>if</b> access is active and not expired
    <b>let</b> now = tx_context::epoch(ctx);
    <b>if</b> (access.status != <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ACCESS_STATUS_ACTIVE">ACCESS_STATUS_ACTIVE</a> || (access.expires_at != 0 && now &gt; access.expires_at)) {
        <b>return</b> option::none()
    };
    option::some(access.encrypted_access_key)
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_has_access"></a>

## Function `has_access`

Check if a user has access to content


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_has_access">has_access</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, user: <b>address</b>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_has_access">has_access</a>(
    content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    user: <b>address</b>,
    ctx: &TxContext
): bool {
    // If user is the <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>, they always have access
    <b>if</b> (content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a> == user) {
        <b>return</b> <b>true</b>
    };
    <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_verify_access">verify_access</a>(content, user, ctx)
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_owner"></a>

## Function `owner`

Get the owner of the content


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): <b>address</b> {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_owner">owner</a>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_content_type"></a>

## Function `content_type`

Get the content type


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): u8 {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_type">content_type</a>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_content_hash"></a>

## Function `content_hash`

Get the content hash for verification


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): vector&lt;u8&gt; {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_content_hash">content_hash</a>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_public_metadata"></a>

## Function `public_metadata`

Get the public metadata


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): String {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_public_metadata">public_metadata</a>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_created_at"></a>

## Function `created_at`

Get content creation timestamp


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): u64 {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_created_at">created_at</a>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_updated_at"></a>

## Function `updated_at`

Get content last updated timestamp


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): u64 {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_updated_at">updated_at</a>
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_get_content_metadata"></a>

## Function `get_content_metadata`

Get detailed content metadata


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_content_metadata">get_content_metadata</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentMetadata">social_contracts::encrypted_content::ContentMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_content_metadata">get_content_metadata</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentMetadata">ContentMetadata</a> {
    *dynamic_field::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_ContentMetadata">ContentMetadata</a>&gt;(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_CONTENT_METADATA_FIELD">CONTENT_METADATA_FIELD</a>)
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_get_tier_details"></a>

## Function `get_tier_details`

Get a tier's details


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_tier_details">get_tier_details</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>, tier_id: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">social_contracts::encrypted_content::PaymentTier</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_tier_details">get_tier_details</a>(
    content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>,
    tier_id: String
): Option&lt;<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a>&gt; {
    <b>if</b> (!dynamic_field::exists_(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>)) {
        <b>return</b> option::none()
    };
    <b>let</b> tiers = dynamic_field::borrow&lt;vector&lt;u8&gt;, Table&lt;String, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_PaymentTier">PaymentTier</a>&gt;&gt;(&content.id, <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_TIERS_FIELD">TIERS_FIELD</a>);
    <b>if</b> (!table::contains(tiers, tier_id)) {
        <b>return</b> option::none()
    };
    option::some(*table::borrow(tiers, tier_id))
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_get_tier_ids"></a>

## Function `get_tier_ids`

Get all tier IDs for a piece of content


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_tier_ids">get_tier_ids</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_get_tier_ids">get_tier_ids</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): vector&lt;String&gt; {
    // Return the tracked tier IDs
    content.tier_ids
}
</code></pre>



</details>

<a name="social_contracts_encrypted_content_platform_fee_bps"></a>

## Function `platform_fee_bps`

Get the platform fee in basis points


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">social_contracts::encrypted_content::EncryptedContent</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>(content: &<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_EncryptedContent">EncryptedContent</a>): u64 {
    content.<a href="../social_contracts/encrypted_content.md#social_contracts_encrypted_content_platform_fee_bps">platform_fee_bps</a>
}
</code></pre>



</details>
