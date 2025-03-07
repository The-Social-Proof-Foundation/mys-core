// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mys_sdk::MysClientBuilder;

// This example shows the few basic ways to connect to a Mys network.
// There are several in-built methods for connecting to the
// Mys devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no Mys network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mys = MysClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Mys local network version: {}", mys.api_version());

    // local Mys network, like the above one but using the dedicated function
    let mys_local = MysClientBuilder::default().build_localnet().await?;
    println!("Mys local network version: {}", mys_local.api_version());

    // Mys devnet -- https://fullnode.devnet.mys.io:443
    let mys_devnet = MysClientBuilder::default().build_devnet().await?;
    println!("Mys devnet version: {}", mys_devnet.api_version());

    // Mys testnet -- https://fullnode.testnet.mys.io:443
    let mys_testnet = MysClientBuilder::default().build_testnet().await?;
    println!("Mys testnet version: {}", mys_testnet.api_version());

    // Mys mainnet -- https://fullnode.mainnet.mys.io:443
    let mys_mainnet = MysClientBuilder::default().build_mainnet().await?;
    println!("Mys mainnet version: {}", mys_mainnet.api_version());

    println!("rpc methods: {:?}", mys_testnet.available_rpc_methods());
    println!(
        "available subscriptions: {:?}",
        mys_testnet.available_subscriptions()
    );

    Ok(())
}
