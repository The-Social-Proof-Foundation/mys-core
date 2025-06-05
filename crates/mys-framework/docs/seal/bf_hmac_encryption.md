---
title: Module `seal::bf_hmac_encryption`
---

Implementation of decryption for Seal using Boneh-Franklin over BLS12-381 as KEM and Hmac256Ctr as DEM.


-  [Struct `EncryptedObject`](#seal_bf_hmac_encryption_EncryptedObject)
-  [Struct `VerifiedDerivedKey`](#seal_bf_hmac_encryption_VerifiedDerivedKey)
-  [Struct `PublicKey`](#seal_bf_hmac_encryption_PublicKey)
-  [Enum `KeyPurpose`](#seal_bf_hmac_encryption_KeyPurpose)
-  [Constants](#@Constants_0)
-  [Function `get_public_key`](#seal_bf_hmac_encryption_get_public_key)
-  [Function `decrypt`](#seal_bf_hmac_encryption_decrypt)
-  [Function `verify_share`](#seal_bf_hmac_encryption_verify_share)
-  [Function `create_full_id`](#seal_bf_hmac_encryption_create_full_id)
-  [Function `derive_key`](#seal_bf_hmac_encryption_derive_key)
-  [Function `xor`](#seal_bf_hmac_encryption_xor)
-  [Function `decrypt_shares_with_randomness`](#seal_bf_hmac_encryption_decrypt_shares_with_randomness)
-  [Function `verify_derived_keys`](#seal_bf_hmac_encryption_verify_derived_keys)
-  [Function `verify_derived_key`](#seal_bf_hmac_encryption_verify_derived_key)
-  [Function `parse_encrypted_object`](#seal_bf_hmac_encryption_parse_encrypted_object)
-  [Function `peel_tuple_u8`](#seal_bf_hmac_encryption_peel_tuple_u8)
-  [Function `package_id`](#seal_bf_hmac_encryption_package_id)
-  [Function `id`](#seal_bf_hmac_encryption_id)
-  [Function `services`](#seal_bf_hmac_encryption_services)
-  [Function `indices`](#seal_bf_hmac_encryption_indices)
-  [Function `threshold`](#seal_bf_hmac_encryption_threshold)
-  [Function `nonce`](#seal_bf_hmac_encryption_nonce)
-  [Function `encrypted_shares`](#seal_bf_hmac_encryption_encrypted_shares)
-  [Function `encrypted_randomness`](#seal_bf_hmac_encryption_encrypted_randomness)
-  [Function `blob`](#seal_bf_hmac_encryption_blob)
-  [Function `aad`](#seal_bf_hmac_encryption_aad)
-  [Function `mac`](#seal_bf_hmac_encryption_mac)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/group_ops.md#mys_group_ops">mys::group_ops</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/hmac.md#mys_hmac">mys::hmac</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../seal/gf256.md#seal_gf256">seal::gf256</a>;
<b>use</b> <a href="../seal/hmac256ctr.md#seal_hmac256ctr">seal::hmac256ctr</a>;
<b>use</b> <a href="../seal/kdf.md#seal_kdf">seal::kdf</a>;
<b>use</b> <a href="../seal/key_server.md#seal_key_server">seal::key_server</a>;
<b>use</b> <a href="../seal/polynomial.md#seal_polynomial">seal::polynomial</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="seal_bf_hmac_encryption_EncryptedObject"></a>

## Struct `EncryptedObject`



<pre><code><b>public</b> <b>struct</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>: <a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>: vector&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="seal_bf_hmac_encryption_VerifiedDerivedKey"></a>

## Struct `VerifiedDerivedKey`



<pre><code><b>public</b> <b>struct</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a> <b>has</b> drop, store
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
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../seal/key_server.md#seal_key_server">key_server</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="seal_bf_hmac_encryption_PublicKey"></a>

## Struct `PublicKey`



<pre><code><b>public</b> <b>struct</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">PublicKey</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../seal/key_server.md#seal_key_server">key_server</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
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

<a name="seal_bf_hmac_encryption_KeyPurpose"></a>

## Enum `KeyPurpose`

An enum representing the different purposes of the derived key.


<pre><code><b>public</b> <b>enum</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_KeyPurpose">KeyPurpose</a>
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


<a name="seal_bf_hmac_encryption_DST_DERIVE_KEY"></a>



<pre><code><b>const</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_DST_DERIVE_KEY">DST_DERIVE_KEY</a>: vector&lt;u8&gt; = vector[109, 121, 115, 45, 83, 69, 65, 76, 45, 73, 66, 69, 45, 66, 76, 83, 49, 50, 51, 56, 49, 45, 72, 51, 45, 48, 48];
</code></pre>



<a name="seal_bf_hmac_encryption_get_public_key"></a>

## Function `get_public_key`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_get_public_key">get_public_key</a>(<a href="../seal/key_server.md#seal_key_server">key_server</a>: &<a href="../seal/key_server.md#seal_key_server_KeyServer">seal::key_server::KeyServer</a>): <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">seal::bf_hmac_encryption::PublicKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_get_public_key">get_public_key</a>(<a href="../seal/key_server.md#seal_key_server">key_server</a>: &KeyServer): <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">PublicKey</a> {
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">PublicKey</a> {
        <a href="../seal/key_server.md#seal_key_server">key_server</a>: object::id(<a href="../seal/key_server.md#seal_key_server">key_server</a>),
        pk: <a href="../seal/key_server.md#seal_key_server">key_server</a>.pk_as_bf_bls12381(),
    }
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_decrypt"></a>

## Function `decrypt`

Decrypts an encrypted object using the given verified derived keys.

Call <code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_derived_keys">verify_derived_keys</a></code> to verify derived keys before calling this function.

Aborts if there are not enough verified derived keys.
Aborts if any of the key servers are not among the key servers found in the encrypted object.

If the decryption fails, e.g. the AAD or MAC is invalid, the function returns <code>none</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_decrypt">decrypt</a>(encrypted_object: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>, verified_derived_keys: &vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">seal::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, public_keys: &vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">seal::bf_hmac_encryption::PublicKey</a>&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_decrypt">decrypt</a>(
    encrypted_object: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>,
    verified_derived_keys: &vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a>&gt;,
    public_keys: &vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">PublicKey</a>&gt;,
): Option&lt;vector&lt;u8&gt;&gt; {
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> {
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>,
    } = encrypted_object;
    <b>assert</b>!(verified_derived_keys.length() &gt;= *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a> <b>as</b> u64);
    <b>assert</b>!(verified_derived_keys.all!(|vdk| vdk.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a> == *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a> && vdk.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a> == *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>));
    // Verify that the <b>public</b> keys are from the key servers in the encrypted object and in the same order.
    public_keys.zip_do_ref!(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>, |a, b| <b>assert</b>!(a.<a href="../seal/key_server.md#seal_key_server">key_server</a>.to_address() == b));
    // Find the <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a> of the key servers corresponsing to the derived keys.
    <b>let</b> given_indices = verified_derived_keys.map_ref!(
        |vdk| <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>.find_index!(|service| vdk.<a href="../seal/key_server.md#seal_key_server">key_server</a>.to_address() == service).extract(),
    );
    // Create the full ID <b>for</b> the IBE scheme.
    <b>let</b> full_id = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_create_full_id">create_full_id</a>(*<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>, *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>);
    // Decrypt shares.
    <b>let</b> decrypted_shares = given_indices.zip_map_ref!(verified_derived_keys, |i, vdk| {
        <b>let</b> symmetric_key = <a href="../seal/kdf.md#seal_kdf">kdf</a>(
            &pairing(&vdk.derived_key, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>),
            <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>,
            &hash_to_g1_with_dst(&full_id),
            <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>[*i],
            <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>[*i] <b>as</b> u8,
        );
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>[*i].zip_map!(symmetric_key, |a, b| a ^ b)
    });
    // Construct the key from the decrypted shares.
    <b>let</b> share_indices = given_indices.map!(|i| <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>[i]);
    <b>let</b> polynomials = vector::tabulate!(
        32,
        |i| <a href="../seal/polynomial.md#seal_polynomial_interpolate">polynomial::interpolate</a>(&share_indices, &decrypted_shares.map_ref!(|share| share[i])),
    );
    <b>assert</b>!(polynomials.all!(|p| p.degree() + 1 == *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a> <b>as</b> u64));
    <b>let</b> base_key = polynomials.map_ref!(|p| p.get_constant_term());
    // The encryption randomness can now be decrypted and used to <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_decrypt">decrypt</a> the rest of the shares.
    <b>let</b> randomness = scalar_from_bytes(
        &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_xor">xor</a>(
            <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
            &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_derive_key">derive_key</a>(
                KeyPurpose::EncryptedRandomness,
                &base_key,
                <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>,
                *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>,
                <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>,
            ),
        ),
    );
    <b>assert</b>!(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a> == g2_mul(&randomness, &g2_generator()));
    <b>let</b> (remaining_shares, remaining_indices) = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_decrypt_shares_with_randomness">decrypt_shares_with_randomness</a>(
        &randomness,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>,
        &public_keys.map_ref!(|pk| pk.pk),
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>,
        &full_id,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>,
        &given_indices,
    );
    // Verify the consistency of the shares, eg. that they are all consistent with the <a href="../seal/polynomial.md#seal_polynomial">polynomial</a> interpolated from the shares decrypted from the given keys.
    remaining_shares.zip_do!(remaining_indices, |share, index| {
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_share">verify_share</a>(&polynomials, &share, index);
    });
    // Decrypt the <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>.
    <a href="../seal/hmac256ctr.md#seal_hmac256ctr_decrypt">hmac256ctr::decrypt</a>(
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>,
        &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>.get_with_default(vector[]),
        &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_derive_key">derive_key</a>(KeyPurpose::DEM, &base_key, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>, *<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>),
    )
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_verify_share"></a>

## Function `verify_share`



<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_share">verify_share</a>(polynomials: &vector&lt;<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>&gt;, share: &vector&lt;u8&gt;, index: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_share">verify_share</a>(polynomials: &vector&lt;<a href="../seal/polynomial.md#seal_polynomial_Polynomial">polynomial::Polynomial</a>&gt;, share: &vector&lt;u8&gt;, index: u8) {
    polynomials.zip_do_ref!(share, |p, s| {
        <b>assert</b>!(p.evaluate(index) == s);
    });
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_create_full_id"></a>

## Function `create_full_id`



<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_create_full_id">create_full_id</a>(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_create_full_id">create_full_id</a>(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> full_id = vector::empty();
    full_id.append(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>.to_bytes());
    full_id.append(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>);
    full_id
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_derive_key"></a>

## Function `derive_key`

Derives a key for a specific purpose from the base key.


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_derive_key">derive_key</a>(purpose: <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_KeyPurpose">seal::bf_hmac_encryption::KeyPurpose</a>, key: &vector&lt;u8&gt;, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>: &vector&lt;vector&lt;u8&gt;&gt;, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>: u8, key_servers: &vector&lt;<b>address</b>&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_derive_key">derive_key</a>(
    purpose: <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_KeyPurpose">KeyPurpose</a>,
    key: &vector&lt;u8&gt;,
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>: &vector&lt;vector&lt;u8&gt;&gt;,
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>: u8,
    key_servers: &vector&lt;<b>address</b>&gt;,
): vector&lt;u8&gt; {
    <b>let</b> tag = match (purpose) {
        KeyPurpose::EncryptedRandomness =&gt; vector[0],
        KeyPurpose::DEM =&gt; vector[1],
    };
    <b>let</b> <b>mut</b> bytes = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_DST_DERIVE_KEY">DST_DERIVE_KEY</a>;
    bytes.append(*key);
    bytes.append(tag);
    bytes.push_back(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>);
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>.do_ref!(|share| bytes.append(*share));
    key_servers.do_ref!(|<a href="../seal/key_server.md#seal_key_server">key_server</a>| bytes.append((*<a href="../seal/key_server.md#seal_key_server">key_server</a>).to_bytes()));
    sha3_256(bytes)
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_xor"></a>

## Function `xor`



<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_xor">xor</a>(a: &vector&lt;u8&gt;, b: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_xor">xor</a>(a: &vector&lt;u8&gt;, b: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    a.zip_map_ref!(b, |a, b| *a ^ *b)
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_decrypt_shares_with_randomness"></a>

## Function `decrypt_shares_with_randomness`

Decrypts shares with the given randomness.
Returns the decrypted shares and the indices of the shares that were decrypted.


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_decrypt_shares_with_randomness">decrypt_shares_with_randomness</a>(randomness: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_Scalar">mys::bls12381::Scalar</a>&gt;, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>: &vector&lt;vector&lt;u8&gt;&gt;, public_keys: &vector&lt;<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;&gt;, object_ids: &vector&lt;<b>address</b>&gt;, full_id: &vector&lt;u8&gt;, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>: &vector&lt;u8&gt;, indices_to_omit: &vector&lt;u64&gt;): (vector&lt;vector&lt;u8&gt;&gt;, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_decrypt_shares_with_randomness">decrypt_shares_with_randomness</a>(
    randomness: &Element&lt;Scalar&gt;,
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>: &vector&lt;vector&lt;u8&gt;&gt;,
    public_keys: &vector&lt;Element&lt;G2&gt;&gt;,
    object_ids: &vector&lt;<b>address</b>&gt;,
    full_id: &vector&lt;u8&gt;,
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>: &vector&lt;u8&gt;,
    indices_to_omit: &vector&lt;u64&gt;,
): (vector&lt;vector&lt;u8&gt;&gt;, vector&lt;u8&gt;) {
    <b>let</b> n = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>.length();
    <b>assert</b>!(n == <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>.length());
    <b>assert</b>!(n == public_keys.length());
    <b>assert</b>!(n == object_ids.length());
    <b>let</b> gid = hash_to_g1_with_dst(full_id);
    <b>let</b> gid_r = g1_mul(randomness, &gid);
    <b>let</b> <b>mut</b> decrypted_shares = vector::empty();
    <b>let</b> <b>mut</b> remaining_indices = vector::empty();
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a> = g2_mul(randomness, &g2_generator());
    n.do!(|i| {
        <b>if</b> (!indices_to_omit.contains(&(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>[i] <b>as</b> u64))) {
            decrypted_shares.push_back(
                <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_xor">xor</a>(
                    &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>[i],
                    &<a href="../seal/kdf.md#seal_kdf">kdf</a>(
                        &pairing(&gid_r, &public_keys[i]),
                        &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>,
                        &gid,
                        object_ids[i],
                        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>[i] <b>as</b> u8,
                    ),
                ),
            );
            remaining_indices.push_back(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>[i]);
        }
    });
    (decrypted_shares, remaining_indices)
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_verify_derived_keys"></a>

## Function `verify_derived_keys`

Returns a vector of <code><a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a></code>s, asserting that all derived_keys are valid for the given full ID and key servers.
Aborts if the number of key servers does not match the number of derived keys.


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_derived_keys">verify_derived_keys</a>(derived_keys: &vector&lt;<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;&gt;, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;, public_keys: &vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">seal::bf_hmac_encryption::PublicKey</a>&gt;): vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">seal::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_derived_keys">verify_derived_keys</a>(
    derived_keys: &vector&lt;Element&lt;G1&gt;&gt;,
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>: <b>address</b>,
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>: vector&lt;u8&gt;,
    public_keys: &vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_PublicKey">PublicKey</a>&gt;,
): vector&lt;<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a>&gt; {
    <b>assert</b>!(public_keys.length() == derived_keys.length());
    <b>let</b> gid = hash_to_g1_with_dst(&<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_create_full_id">create_full_id</a>(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>, <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>));
    public_keys.zip_map_ref!(derived_keys, |vpk, derived_key| {
        <b>assert</b>!(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_derived_key">verify_derived_key</a>(derived_key, &gid, &vpk.pk));
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_VerifiedDerivedKey">VerifiedDerivedKey</a> {
            derived_key: *derived_key,
            <a href="../seal/key_server.md#seal_key_server">key_server</a>: vpk.<a href="../seal/key_server.md#seal_key_server">key_server</a>,
            <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>,
            <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>,
        }
    })
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_verify_derived_key"></a>

## Function `verify_derived_key`



<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_derived_key">verify_derived_key</a>(derived_key: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;, gid: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;, public_key: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_verify_derived_key">verify_derived_key</a>(
    derived_key: &Element&lt;G1&gt;,
    gid: &Element&lt;G1&gt;,
    public_key: &Element&lt;G2&gt;,
): bool {
    pairing(derived_key, &g2_generator()) == pairing(gid, public_key)
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_parse_encrypted_object"></a>

## Function `parse_encrypted_object`

Deserialize a BCS encoded EncryptedObject.
Fails if the version is not 0.
Fails if the object is not a valid EncryptedObject.
Fails if the encryption type is not Hmac256Ctr.
Fails if the KEM type is not Boneh-Franklin over BLS12-381.


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_parse_encrypted_object">parse_encrypted_object</a>(object: vector&lt;u8&gt;): <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_parse_encrypted_object">parse_encrypted_object</a>(object: vector&lt;u8&gt;): <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> {
    <b>let</b> <b>mut</b> bcs = <a href="../mys/bcs.md#mys_bcs_new">mys::bcs::new</a>(object);
    <b>let</b> version = bcs.peel_u8();
    <b>assert</b>!(version == 0);
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a> = bcs.peel_address();
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a> = bcs.peel_vec_u8();
    // <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a> is a vector of tuples of the form (<b>address</b>, u8).
    <b>let</b> <b>mut</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>: vector&lt;<b>address</b>&gt; = vector::empty();
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a> = bcs.peel_vec!(|service| {
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>.push_back(service.peel_address());
        service.peel_u8()
    });
    <b>assert</b>!(<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>.length() == <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>.length());
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a> = bcs.peel_u8();
    <b>let</b> ibe_type = bcs.peel_enum_tag();
    <b>assert</b>!(ibe_type == 0);
    // <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a> is an G2 element, which is 96 bytes.
    <b>let</b> nonce_bytes = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(&<b>mut</b> bcs, 96);
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a> = g2_from_bytes(&nonce_bytes);
    // Shares are 32 bytes.
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a> = bcs.peel_vec!(|share_bcs| <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(share_bcs, 32));
    // Encrypted randomness is 32 bytes.
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a> = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(&<b>mut</b> bcs, 32);
    // Move only supports Hmac256Ctr mode.
    <b>let</b> encryption_type = bcs.peel_enum_tag();
    <b>assert</b>!(encryption_type == 1);
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a> = bcs.peel_vec_u8();
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a> = bcs.peel_option!(|aad_bcs| aad_bcs.peel_vec_u8());
    // MAC is 32 bytes.
    <b>let</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a> = <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(&<b>mut</b> bcs, 32);
    <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a> {
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>,
        <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>,
    }
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_peel_tuple_u8"></a>

## Function `peel_tuple_u8`



<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(bcs: &<b>mut</b> <a href="../mys/bcs.md#mys_bcs_BCS">mys::bcs::BCS</a>, length: u64): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_peel_tuple_u8">peel_tuple_u8</a>(bcs: &<b>mut</b> <a href="../mys/bcs.md#mys_bcs_BCS">mys::bcs::BCS</a>, length: u64): vector&lt;u8&gt; {
    vector::tabulate!(length, |_| bcs.peel_u8())
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_package_id"></a>

## Function `package_id`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &<b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &<b>address</b> {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_package_id">package_id</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_id"></a>

## Function `id`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_id">id</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_services"></a>

## Function `services`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;<b>address</b>&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_services">services</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_indices"></a>

## Function `indices`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_indices">indices</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_threshold"></a>

## Function `threshold`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): u8 {
    self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_threshold">threshold</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_nonce"></a>

## Function `nonce`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &Element&lt;G2&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_nonce">nonce</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_encrypted_shares"></a>

## Function `encrypted_shares`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_shares">encrypted_shares</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_encrypted_randomness"></a>

## Function `encrypted_randomness`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_encrypted_randomness">encrypted_randomness</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_blob"></a>

## Function `blob`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_blob">blob</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_aad"></a>

## Function `aad`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_aad">aad</a>
}
</code></pre>



</details>

<a name="seal_bf_hmac_encryption_mac"></a>

## Function `mac`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">seal::bf_hmac_encryption::EncryptedObject</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>(self: &<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_EncryptedObject">EncryptedObject</a>): &vector&lt;u8&gt; {
    &self.<a href="../seal/bf_hmac_encryption.md#seal_bf_hmac_encryption_mac">mac</a>
}
</code></pre>



</details>
