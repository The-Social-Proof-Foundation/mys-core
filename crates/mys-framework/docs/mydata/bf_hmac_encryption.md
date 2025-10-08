---
title: Module `mydata::bf_hmac_encryption`
---

Implementation of decryption for MyData using Boneh-Franklin over BLS12-381 as KEM and Hmac256Ctr as DEM.
Refer usage at docs https://mydata-docs.wal.app/UsingMyData/#on-chain-decryption


-  [Struct `EncryptedObject`](#mydata_bf_hmac_encryption_EncryptedObject)
-  [Struct `VerifiedDerivedKey`](#mydata_bf_hmac_encryption_VerifiedDerivedKey)
-  [Struct `PublicKey`](#mydata_bf_hmac_encryption_PublicKey)
-  [Enum `KeyPurpose`](#mydata_bf_hmac_encryption_KeyPurpose)
-  [Constants](#@Constants_0)
-  [Function `new_public_key`](#mydata_bf_hmac_encryption_new_public_key)
-  [Function `decrypt`](#mydata_bf_hmac_encryption_decrypt)
-  [Function `decrypt_randomness`](#mydata_bf_hmac_encryption_decrypt_randomness)
-  [Function `safe_scalar_from_bytes`](#mydata_bf_hmac_encryption_safe_scalar_from_bytes)
-  [Function `verify_nonce`](#mydata_bf_hmac_encryption_verify_nonce)
-  [Function `verify_share`](#mydata_bf_hmac_encryption_verify_share)
-  [Function `decrypt_shares_with_derived_keys`](#mydata_bf_hmac_encryption_decrypt_shares_with_derived_keys)
-  [Function `decrypt_all_shares_with_randomness`](#mydata_bf_hmac_encryption_decrypt_all_shares_with_randomness)
-  [Function `create_full_id`](#mydata_bf_hmac_encryption_create_full_id)
-  [Function `derive_key`](#mydata_bf_hmac_encryption_derive_key)
-  [Function `xor`](#mydata_bf_hmac_encryption_xor)
-  [Function `verify_derived_keys`](#mydata_bf_hmac_encryption_verify_derived_keys)
-  [Function `verify_derived_key`](#mydata_bf_hmac_encryption_verify_derived_key)
-  [Function `assert_all_unique`](#mydata_bf_hmac_encryption_assert_all_unique)
-  [Function `parse_encrypted_object`](#mydata_bf_hmac_encryption_parse_encrypted_object)
-  [Function `peel_tuple_u8`](#mydata_bf_hmac_encryption_peel_tuple_u8)
-  [Function `package_id`](#mydata_bf_hmac_encryption_package_id)
-  [Function `id`](#mydata_bf_hmac_encryption_id)
-  [Function `services`](#mydata_bf_hmac_encryption_services)
-  [Function `indices`](#mydata_bf_hmac_encryption_indices)
-  [Function `threshold`](#mydata_bf_hmac_encryption_threshold)
-  [Function `nonce`](#mydata_bf_hmac_encryption_nonce)
-  [Function `encrypted_shares`](#mydata_bf_hmac_encryption_encrypted_shares)
-  [Function `encrypted_randomness`](#mydata_bf_hmac_encryption_encrypted_randomness)
-  [Function `blob`](#mydata_bf_hmac_encryption_blob)
-  [Function `aad`](#mydata_bf_hmac_encryption_aad)
-  [Function `mac`](#mydata_bf_hmac_encryption_mac)


<pre><code><b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
<b>use</b> <a href="../mydata/polynomial.md#mydata_polynomial">mydata::polynomial</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/group_ops.md#mys_group_ops">mys::group_ops</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/hmac.md#mys_hmac">mys::hmac</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="mydata_bf_hmac_encryption_EncryptedObject"></a>

## Struct `EncryptedObject`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>: <a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>: vector&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_bf_hmac_encryption_VerifiedDerivedKey"></a>

## Struct `VerifiedDerivedKey`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>derived_key: <a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server">key_server</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_bf_hmac_encryption_PublicKey"></a>

## Struct `PublicKey`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">PublicKey</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server">key_server</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>pk: <a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_bf_hmac_encryption_KeyPurpose"></a>

## Enum `KeyPurpose`

An enum representing the different purposes of the derived key.


<pre><code><b>public</b> <b>enum</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_KeyPurpose">KeyPurpose</a>
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>DEM</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>EncryptedRandomness</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mydata_bf_hmac_encryption_DST_DERIVE_KEY"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_DST_DERIVE_KEY">DST_DERIVE_KEY</a>: vector&lt;u8&gt; = vector[77, 89, 83, 45, 77, 89, 68, 65, 84, 65, 45, 73, 66, 69, 45, 66, 76, 83, 49, 50, 51, 56, 49, 45, 72, 51, 45, 48, 48];
</code></pre>



<a name="mydata_bf_hmac_encryption_EIncompatibleInputLengths"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EIncompatibleInputLengths">EIncompatibleInputLengths</a>: u64 = 2;
</code></pre>



<a name="mydata_bf_hmac_encryption_EInsufficientDerivedKeys"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInsufficientDerivedKeys">EInsufficientDerivedKeys</a>: u64 = 4;
</code></pre>



<a name="mydata_bf_hmac_encryption_EInvalidEncryptedObject"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>: u64 = 3;
</code></pre>



<a name="mydata_bf_hmac_encryption_EInvalidVerifiedKey"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidVerifiedKey">EInvalidVerifiedKey</a>: u64 = 5;
</code></pre>



<a name="mydata_bf_hmac_encryption_EWrongPublicKeys"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongPublicKeys">EWrongPublicKeys</a>: u64 = 1;
</code></pre>



<a name="mydata_bf_hmac_encryption_EWrongVerifiedKeys"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongVerifiedKeys">EWrongVerifiedKeys</a>: u64 = 0;
</code></pre>



<a name="mydata_bf_hmac_encryption_SCALAR_BYTE_LENGTH"></a>



<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_SCALAR_BYTE_LENGTH">SCALAR_BYTE_LENGTH</a>: u64 = 32;
</code></pre>



<a name="mydata_bf_hmac_encryption_SCALAR_FIELD_ORDER"></a>

The order of the scalar field for BLS12-381.


<pre><code><b>const</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_SCALAR_FIELD_ORDER">SCALAR_FIELD_ORDER</a>: u256 = 52435875175126190479447740508185965837690552500527637822603658699938581184513;
</code></pre>



<a name="mydata_bf_hmac_encryption_new_public_key"></a>

## Function `new_public_key`

Creates PublicKey from key server ID and public key bytes.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_new_public_key">new_public_key</a>(key_server_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, pk_bytes: vector&lt;u8&gt;): <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_new_public_key">new_public_key</a>(key_server_id: ID, pk_bytes: vector&lt;u8&gt;): <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">PublicKey</a> {
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">PublicKey</a> {
        <a href="../mydata/key_server.md#mydata_key_server">key_server</a>: key_server_id,
        pk: g2_from_bytes(&pk_bytes),
    }
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_decrypt"></a>

## Function `decrypt`

Decrypts an encrypted object using the given verified derived keys.

Call <code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_derived_keys">verify_derived_keys</a></code> to verify derived keys before calling this function.

Aborts if there are not enough verified derived keys to reach the threshold.
Aborts if any of the key servers for the given verified derived keys are not among the key servers found in the encrypted object.
Aborts if the given public keys do not contain exactly one public key for all key servers in the encrypted object and no more.

If the decryption fails, e.g. the AAD or MAC is invalid, the function returns <code>none</code>.

If some key servers are weighted, each derived key contributes the weight of the key server to the threshold.
The public keys can be in any order and there should be exactly one per key server.
The provided verified derived keys can be in any order, but there should be at most one per key server.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt">decrypt</a>(encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>, verified_derived_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt">decrypt</a>(
    encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>,
    verified_derived_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a>&gt;,
    public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">PublicKey</a>&gt;,
): Option&lt;vector&lt;u8&gt;&gt; {
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> {
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>,
    } = encrypted_object;
    <b>assert</b>!(
        verified_derived_keys.all!(|vdk| vdk.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a> == *<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a> && vdk.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a> == *<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>),
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongVerifiedKeys">EWrongVerifiedKeys</a>,
    );
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_assert_all_unique">assert_all_unique</a>(&verified_derived_keys.map_ref!(|vdk| vdk.<a href="../mydata/key_server.md#mydata_key_server">key_server</a>), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongVerifiedKeys">EWrongVerifiedKeys</a>);
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_assert_all_unique">assert_all_unique</a>(&public_keys.map_ref!(|pk| pk.<a href="../mydata/key_server.md#mydata_key_server">key_server</a>), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongPublicKeys">EWrongPublicKeys</a>);
    // Find the <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a> of the <b>public</b> keys corresponding to the key servers in the encrypted object.
    // This aborts <b>if</b> there is no <b>public</b> key <b>for</b> one of the key servers in the encrypted object.
    <b>let</b> public_keys_indices = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>.map_ref!(
        |addr| public_keys.find_index!(|pk| pk.<a href="../mydata/key_server.md#mydata_key_server">key_server</a>.to_address() == addr).destroy_some(),
    );
    // Assert that all the given <b>public</b> keys are used.
    public_keys.length().do!(|i| <b>assert</b>!(public_keys_indices.contains(&i), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongPublicKeys">EWrongPublicKeys</a>));
    // Find the <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a> of the key servers corresponding to the derived keys.
    // This aborts <b>if</b> one of the given derived keys is not from a key server in the encrypted object.
    <b>let</b> indices_per_vdk = verified_derived_keys.map_ref!(|vdk| {
        <b>let</b> index_opt = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>.find_index!(|service| vdk.<a href="../mydata/key_server.md#mydata_key_server">key_server</a>.to_address() == *service);
        <b>assert</b>!(index_opt.is_some(), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EWrongVerifiedKeys">EWrongVerifiedKeys</a>);
        vector[index_opt.destroy_some()]
    });
    // Flatten the <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a> per derived key to get all the <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>.
    <b>let</b> given_indices = indices_per_vdk.flatten();
    <b>assert</b>!(given_indices.length() &gt;= *<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a> <b>as</b> u64, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInsufficientDerivedKeys">EInsufficientDerivedKeys</a>);
    // Decrypt shares.
    <b>let</b> decrypted_shares = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_shares_with_derived_keys">decrypt_shares_with_derived_keys</a>(
        &indices_per_vdk,
        verified_derived_keys,
        encrypted_object,
    );
    // Interpolate polynomials from the decrypted shares.
    <b>let</b> polynomials = interpolate_all(&given_indices.map!(|i| <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>[i]), &decrypted_shares);
    // Compute base key and derive keys <b>for</b> the randomness and DEM.
    <b>let</b> base_key = polynomials.map_ref!(|p| p.get_constant_term());
    <b>let</b> randomness_key = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_derive_key">derive_key</a>(KeyPurpose::EncryptedRandomness, &base_key, encrypted_object);
    <b>let</b> dem_key = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_derive_key">derive_key</a>(KeyPurpose::DEM, &base_key, encrypted_object);
    // Decrypt the randomness
    <b>let</b> randomness = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_randomness">decrypt_randomness</a>(
        &randomness_key,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
    );
    <b>if</b> (randomness.is_none()) {
        <b>return</b> none()
    };
    <b>let</b> randomness = randomness.destroy_some();
    // Use the randomness to verify the <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>.
    <b>if</b> (!<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_nonce">verify_nonce</a>(&randomness, &encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>)) {
        <b>return</b> none()
    };
    // Now, all shares can be decrypted using the randomness and the <b>public</b> keys.
    <b>let</b> all_shares = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_all_shares_with_randomness">decrypt_all_shares_with_randomness</a>(
        &randomness,
        encrypted_object,
        &public_keys_indices.map_ref!(|i| public_keys[*i].pk),
    );
    // Verify the consistency of the shares, eg. that they are all consistent with the <a href="../mydata/polynomial.md#mydata_polynomial">polynomial</a> interpolated from the shares decrypted from the given keys.
    <b>if</b> (
        all_shares
            .zip_map_ref!(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>, |share, index| <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_share">verify_share</a>(&polynomials, share, *index))
            .any!(|verified| !*verified)
    ) {
        <b>return</b> none()
    };
    // Decrypt the <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>.
    <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_decrypt">hmac256ctr::decrypt</a>(
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>,
        &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>.get_with_default(vector[]),
        &dem_key,
    )
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_decrypt_randomness"></a>

## Function `decrypt_randomness`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_randomness">decrypt_randomness</a>(randomness_key: &vector&lt;u8&gt;, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>: &vector&lt;u8&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_Scalar">mys::bls12381::Scalar</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_randomness">decrypt_randomness</a>(
    randomness_key: &vector&lt;u8&gt;,
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>: &vector&lt;u8&gt;,
): Option&lt;Element&lt;Scalar&gt;&gt; {
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_safe_scalar_from_bytes">safe_scalar_from_bytes</a>(
        &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_xor">xor</a>(
            <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
            randomness_key,
        ),
    )
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_safe_scalar_from_bytes"></a>

## Function `safe_scalar_from_bytes`

Converts big-endian bytes to a scalar, returning none if the bytes are not a valid scalar.


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_safe_scalar_from_bytes">safe_scalar_from_bytes</a>(be_bytes: &vector&lt;u8&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_Scalar">mys::bls12381::Scalar</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_safe_scalar_from_bytes">safe_scalar_from_bytes</a>(be_bytes: &vector&lt;u8&gt;): Option&lt;Element&lt;Scalar&gt;&gt; {
    <b>if</b> (be_bytes.length() != <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_SCALAR_BYTE_LENGTH">SCALAR_BYTE_LENGTH</a>) {
        <b>return</b> none()
    };
    // bcs peels in little-endian order, but the scalar is in big-endian order
    <b>let</b> <b>mut</b> le_bytes = *be_bytes;
    le_bytes.reverse();
    <b>let</b> as_integer = <a href="../mys/bcs.md#mys_bcs_new">mys::bcs::new</a>(le_bytes).peel_u256();
    <b>if</b> (as_integer &gt;= <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_SCALAR_FIELD_ORDER">SCALAR_FIELD_ORDER</a>) {
        <b>return</b> none()
    };
    option::some(scalar_from_bytes(be_bytes))
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_verify_nonce"></a>

## Function `verify_nonce`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_nonce">verify_nonce</a>(randomness: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_Scalar">mys::bls12381::Scalar</a>&gt;, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_nonce">verify_nonce</a>(randomness: &Element&lt;Scalar&gt;, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>: &Element&lt;G2&gt;): bool {
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a> == g2_mul(randomness, &g2_generator())
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_verify_share"></a>

## Function `verify_share`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_share">verify_share</a>(polynomials: &vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>&gt;, share: &vector&lt;u8&gt;, index: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_share">verify_share</a>(polynomials: &vector&lt;Polynomial&gt;, share: &vector&lt;u8&gt;, index: u8): bool {
    polynomials.zip_map_ref!(share, |p, s| p.evaluate(index) == s).all!(|verified| *verified)
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_decrypt_shares_with_derived_keys"></a>

## Function `decrypt_shares_with_derived_keys`

Decrypt the given shares with the derived keys.
Panics if the number of indices does not match the number of derived keys.


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_shares_with_derived_keys">decrypt_shares_with_derived_keys</a>(indices_per_vdk: &vector&lt;vector&lt;u64&gt;&gt;, derived_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): vector&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_shares_with_derived_keys">decrypt_shares_with_derived_keys</a>(
    indices_per_vdk: &vector&lt;vector&lt;u64&gt;&gt;,
    derived_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a>&gt;,
    encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>,
): vector&lt;vector&lt;u8&gt;&gt; {
    <b>let</b> gid = hash_to_g1_with_dst(
        &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_create_full_id">create_full_id</a>(encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>, encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>),
    );
    indices_per_vdk.zip_map_ref!(derived_keys, |<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>, vdk| {
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>.map_ref!(|i| {
            <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_xor">xor</a>(
                &encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>[*i],
                &<a href="../mydata/kdf.md#mydata_kdf">kdf</a>(
                    &pairing(&vdk.derived_key, &encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>),
                    &encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>,
                    &gid,
                    encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>[*i],
                    encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>[*i],
                ),
            )
        })
    }).flatten()
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_decrypt_all_shares_with_randomness"></a>

## Function `decrypt_all_shares_with_randomness`

Decrypts shares with the given randomness.


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_all_shares_with_randomness">decrypt_all_shares_with_randomness</a>(randomness: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_Scalar">mys::bls12381::Scalar</a>&gt;, encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>, public_keys: &vector&lt;<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;&gt;): vector&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_decrypt_all_shares_with_randomness">decrypt_all_shares_with_randomness</a>(
    randomness: &Element&lt;Scalar&gt;,
    encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>,
    public_keys: &vector&lt;Element&lt;G2&gt;&gt;,
): (vector&lt;vector&lt;u8&gt;&gt;) {
    <b>let</b> n = encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>.length();
    <b>assert</b>!(n == public_keys.length(), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EIncompatibleInputLengths">EIncompatibleInputLengths</a>);
    <b>let</b> gid = hash_to_g1_with_dst(
        &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_create_full_id">create_full_id</a>(encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>, encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>),
    );
    <b>let</b> gid_r = g1_mul(randomness, &gid);
    vector::tabulate!(n, |i| {
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_xor">xor</a>(
            &encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>[i],
            &<a href="../mydata/kdf.md#mydata_kdf">kdf</a>(
                &pairing(&gid_r, &public_keys[i]),
                &encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>,
                &gid,
                encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>[i],
                encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>[i],
            ),
        )
    })
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_create_full_id"></a>

## Function `create_full_id`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_create_full_id">create_full_id</a>(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_create_full_id">create_full_id</a>(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> full_id = vector::empty();
    full_id.append(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>.to_bytes());
    full_id.append(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>);
    full_id
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_derive_key"></a>

## Function `derive_key`

Derives a key for a specific purpose from the base key.


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_derive_key">derive_key</a>(purpose: <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_KeyPurpose">mydata::bf_hmac_encryption::KeyPurpose</a>, base_key: &vector&lt;u8&gt;, encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_derive_key">derive_key</a>(
    purpose: <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_KeyPurpose">KeyPurpose</a>,
    base_key: &vector&lt;u8&gt;,
    encrypted_object: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>,
): vector&lt;u8&gt; {
    <b>let</b> tag = match (purpose) {
        KeyPurpose::EncryptedRandomness =&gt; vector[0],
        KeyPurpose::DEM =&gt; vector[1],
    };
    <b>let</b> <b>mut</b> bytes = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_DST_DERIVE_KEY">DST_DERIVE_KEY</a>;
    bytes.append(*base_key);
    bytes.append(tag);
    bytes.push_back(encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>);
    encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>.do_ref!(|share| bytes.append(*share));
    encrypted_object.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>.do_ref!(|<a href="../mydata/key_server.md#mydata_key_server">key_server</a>| bytes.append((*<a href="../mydata/key_server.md#mydata_key_server">key_server</a>).to_bytes()));
    sha3_256(bytes)
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_xor"></a>

## Function `xor`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_xor">xor</a>(a: &vector&lt;u8&gt;, b: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_xor">xor</a>(a: &vector&lt;u8&gt;, b: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>assert</b>!(a.length() == b.length(), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EIncompatibleInputLengths">EIncompatibleInputLengths</a>);
    a.zip_map_ref!(b, |a, b| *a ^ *b)
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_verify_derived_keys"></a>

## Function `verify_derived_keys`

Returns a vector of <code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a></code>s, asserting that all derived_keys are valid for the given full ID and key servers.
The order of the derived keys and the public keys must match.
Aborts if the number of key servers does not match the number of derived keys.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_derived_keys">verify_derived_keys</a>(derived_keys: &vector&lt;<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;&gt;, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;, public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;): vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_derived_keys">verify_derived_keys</a>(
    derived_keys: &vector&lt;Element&lt;G1&gt;&gt;,
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>,
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;,
    public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">PublicKey</a>&gt;,
): vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a>&gt; {
    <b>assert</b>!(public_keys.length() == derived_keys.length(), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EIncompatibleInputLengths">EIncompatibleInputLengths</a>);
    <b>let</b> gid = hash_to_g1_with_dst(&<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_create_full_id">create_full_id</a>(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>));
    public_keys.zip_map_ref!(derived_keys, |pk, derived_key| {
        <b>assert</b>!(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_derived_key">verify_derived_key</a>(derived_key, &gid, &pk.pk), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidVerifiedKey">EInvalidVerifiedKey</a>);
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a> {
            derived_key: *derived_key,
            <a href="../mydata/key_server.md#mydata_key_server">key_server</a>: pk.<a href="../mydata/key_server.md#mydata_key_server">key_server</a>,
            <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>,
            <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>,
        }
    })
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_verify_derived_key"></a>

## Function `verify_derived_key`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_derived_key">verify_derived_key</a>(derived_key: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;, gid: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;, public_key: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_verify_derived_key">verify_derived_key</a>(
    derived_key: &Element&lt;G1&gt;,
    gid: &Element&lt;G1&gt;,
    public_key: &Element&lt;G2&gt;,
): bool {
    pairing(derived_key, &g2_generator()) == pairing(gid, public_key)
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_assert_all_unique"></a>

## Function `assert_all_unique`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_assert_all_unique">assert_all_unique</a>&lt;T&gt;(items: &vector&lt;T&gt;, error_code: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_assert_all_unique">assert_all_unique</a>&lt;T&gt;(items: &vector&lt;T&gt;, error_code: u64) {
    items.length().do!(|i| {
        <b>let</b> (_, j) = items.index_of(&items[i]);
        <b>assert</b>!(i == j, error_code);
    });
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_parse_encrypted_object"></a>

## Function `parse_encrypted_object`

Deserialize a BCS encoded EncryptedObject.
Fails if the version is not 0.
Fails if the object is not a valid EncryptedObject.
Fails if the encryption type is not Hmac256Ctr.
Fails if the KEM type is not Boneh-Franklin over BLS12-381.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_parse_encrypted_object">parse_encrypted_object</a>(object: vector&lt;u8&gt;): <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_parse_encrypted_object">parse_encrypted_object</a>(object: vector&lt;u8&gt;): <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> {
    <b>let</b> <b>mut</b> bcs = <a href="../mys/bcs.md#mys_bcs_new">mys::bcs::new</a>(object);
    <b>let</b> version = bcs.peel_u8();
    <b>assert</b>!(version == 0, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a> = bcs.peel_address();
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a> = bcs.peel_vec_u8();
    // <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a> is a vector of tuples of the form (<b>address</b>, u8).
    <b>let</b> <b>mut</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>: vector&lt;<b>address</b>&gt; = vector::empty();
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a> = bcs.peel_vec!(|service| {
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>.push_back(service.peel_address());
        service.peel_u8()
    });
    <b>assert</b>!(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>.length() == <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>.length(), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_assert_all_unique">assert_all_unique</a>(&<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a> = bcs.peel_u8();
    <b>assert</b>!(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a> &gt; 0 && <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a> &lt;= <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>.length() <b>as</b> u8, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    <b>let</b> ibe_type = bcs.peel_enum_tag();
    <b>assert</b>!(ibe_type == 0, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    // <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a> is an G2 element, which is 96 bytes.
    <b>let</b> nonce_bytes = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(&<b>mut</b> bcs, 96);
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a> = g2_from_bytes(&nonce_bytes);
    // Shares are 32 bytes.
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a> = bcs.peel_vec!(|share_bcs| <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(share_bcs, 32));
    <b>assert</b>!(<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>.length() == <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>.length(), <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    // Encrypted randomness is 32 bytes.
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a> = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(&<b>mut</b> bcs, 32);
    // Move only supports Hmac256Ctr mode.
    <b>let</b> encryption_type = bcs.peel_enum_tag();
    <b>assert</b>!(encryption_type == 1, <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EInvalidEncryptedObject">EInvalidEncryptedObject</a>);
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a> = bcs.peel_vec_u8();
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a> = bcs.peel_option!(|aad_bcs| aad_bcs.peel_vec_u8());
    // MAC is 32 bytes.
    <b>let</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a> = <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(&<b>mut</b> bcs, 32);
    <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> {
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>,
        <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>,
    }
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_peel_tuple_u8"></a>

## Function `peel_tuple_u8`



<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(bcs: &<b>mut</b> <a href="../mys/bcs.md#mys_bcs_BCS">mys::bcs::BCS</a>, length: u64): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(bcs: &<b>mut</b> <a href="../mys/bcs.md#mys_bcs_BCS">mys::bcs::BCS</a>, length: u64): vector&lt;u8&gt; {
    vector::tabulate!(length, |_| bcs.peel_u8())
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_package_id"></a>

## Function `package_id`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &<b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &<b>address</b> {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_package_id">package_id</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_id"></a>

## Function `id`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_id">id</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_services"></a>

## Function `services`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;<b>address</b>&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_services">services</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_indices"></a>

## Function `indices`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_indices">indices</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_threshold"></a>

## Function `threshold`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): u8 {
    self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_threshold">threshold</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_nonce"></a>

## Function `nonce`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &Element&lt;G2&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_nonce">nonce</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_encrypted_shares"></a>

## Function `encrypted_shares`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_encrypted_randomness"></a>

## Function `encrypted_randomness`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_blob"></a>

## Function `blob`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_blob">blob</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_aad"></a>

## Function `aad`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_aad">aad</a>
}
</code></pre>



</details>

<a name="mydata_bf_hmac_encryption_mac"></a>

## Function `mac`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>(self: &<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_mac">mac</a>
}
</code></pre>



</details>
