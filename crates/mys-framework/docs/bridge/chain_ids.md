---
title: Module `bridge::chain_ids`
---



-  [Struct `BridgeRoute`](#bridge_chain_ids_BridgeRoute)
-  [Constants](#@Constants_0)
-  [Function `mys_mainnet`](#bridge_chain_ids_mys_mainnet)
-  [Function `mys_testnet`](#bridge_chain_ids_mys_testnet)
-  [Function `mys_custom`](#bridge_chain_ids_mys_custom)
-  [Function `eth_mainnet`](#bridge_chain_ids_eth_mainnet)
-  [Function `eth_sepolia`](#bridge_chain_ids_eth_sepolia)
-  [Function `eth_custom`](#bridge_chain_ids_eth_custom)
-  [Function `route_source`](#bridge_chain_ids_route_source)
-  [Function `route_destination`](#bridge_chain_ids_route_destination)
-  [Function `assert_valid_chain_id`](#bridge_chain_ids_assert_valid_chain_id)
-  [Function `valid_routes`](#bridge_chain_ids_valid_routes)
-  [Function `is_valid_route`](#bridge_chain_ids_is_valid_route)
-  [Function `get_route`](#bridge_chain_ids_get_route)


<pre><code><b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="bridge_chain_ids_BridgeRoute"></a>

## Struct `BridgeRoute`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>destination: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_chain_ids_EInvalidBridgeRoute"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>: u64 = 0;
</code></pre>



<a name="bridge_chain_ids_EthCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>: u8 = 12;
</code></pre>



<a name="bridge_chain_ids_EthMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a>: u8 = 10;
</code></pre>



<a name="bridge_chain_ids_EthSepolia"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a>: u8 = 11;
</code></pre>



<a name="bridge_chain_ids_MysCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a>: u8 = 2;
</code></pre>



<a name="bridge_chain_ids_MysMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_MysMainnet">MysMainnet</a>: u8 = 0;
</code></pre>



<a name="bridge_chain_ids_MysTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a>: u8 = 1;
</code></pre>



<a name="bridge_chain_ids_mys_mainnet"></a>

## Function `mys_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_mys_mainnet">mys_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_mys_mainnet">mys_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_MysMainnet">MysMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_mys_testnet"></a>

## Function `mys_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_mys_testnet">mys_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_mys_testnet">mys_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_mys_custom"></a>

## Function `mys_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_mys_custom">mys_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_mys_custom">mys_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_eth_mainnet"></a>

## Function `eth_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">eth_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">eth_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_eth_sepolia"></a>

## Function `eth_sepolia`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">eth_sepolia</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">eth_sepolia</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_eth_custom"></a>

## Function `eth_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">eth_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">eth_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_route_source"></a>

## Function `route_source`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_source">route_source</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): &u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_source">route_source</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a>): &u8 {
    &route.source
}
</code></pre>



</details>

<a name="bridge_chain_ids_route_destination"></a>

## Function `route_destination`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_destination">route_destination</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): &u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_destination">route_destination</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a>): &u8 {
    &route.destination
}
</code></pre>



</details>

<a name="bridge_chain_ids_assert_valid_chain_id"></a>

## Function `assert_valid_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_assert_valid_chain_id">assert_valid_chain_id</a>(id: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_assert_valid_chain_id">assert_valid_chain_id</a>(id: u8) {
    <b>assert</b>!(
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_MysMainnet">MysMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>,
        <a href="../bridge/chain_ids.md#bridge_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>
    )
}
</code></pre>



</details>

<a name="bridge_chain_ids_valid_routes"></a>

## Function `valid_routes`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>(): vector&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>(): vector&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a>&gt; {
    vector[
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysMainnet">MysMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysMainnet">MysMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysTestnet">MysTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_MysCustom">MysCustom</a> }
    ]
}
</code></pre>



</details>

<a name="bridge_chain_ids_is_valid_route"></a>

## Function `is_valid_route`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool {
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>().contains(&route)
}
</code></pre>



</details>

<a name="bridge_chain_ids_get_route"></a>

## Function `get_route`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">get_route</a>(source: u8, destination: u8): <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">get_route</a>(source: u8, destination: u8): <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> {
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>().contains(&route), <a href="../bridge/chain_ids.md#bridge_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    route
}
</code></pre>



</details>
