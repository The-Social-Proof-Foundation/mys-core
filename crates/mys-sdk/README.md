This crate provides the MySocial Rust SDK, containing APIs to interact with the MySocial network. Auto-generated documentation for this crate is [here](https://mystenlabs.github.io/mys/mys_sdk/index.html).

## Getting started

Add the `mys-sdk` dependency as following:

```toml
mys_sdk = { git = "https://github.com/mystenlabs/mys", package = "mys-sdk"}
tokio = { version = "1.2", features = ["full"] }
anyhow = "1.0"
```

The main building block for the MySocial Rust SDK is the `MysClientBuilder`, which provides a simple and straightforward way of connecting to a MySocial network and having access to the different available APIs.

In the following example, the application connects to the MySocial `testnet` and `devnet` networks and prints out their respective RPC API versions.

```rust
use mys_sdk::MysClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // MySocial testnet -- https://fullnode.testnet.mys.io:443
    let mys_testnet = MysClientBuilder::default().build_testnet().await?;
    println!("MySocial testnet version: {}", mys_testnet.api_version());

     // MySocial devnet -- https://fullnode.devnet.mys.io:443
    let mys_devnet = MysClientBuilder::default().build_devnet().await?;
    println!("MySocial devnet version: {}", mys_devnet.api_version());

    // MySocial mainnet -- https://fullnode.mainnet.mys.io:443
    let mys_mainnet = MysClientBuilder::default().build_mainnet().await?;
    println!("MySocial mainnet version: {}", mys_mainnet.api_version());

    Ok(())
}

```

## Documentation for mys-sdk crate

[GitHub Pages](https://mystenlabs.github.io/mys/mys_sdk/index.html) hosts the generated documentation for all Rust crates in the MySocial repository.

### Building documentation locally

You can also build the documentation locally. To do so,

1. Clone the `mys` repo locally. Open a Terminal or Console and go to the `mys/crates/mys-sdk` directory.

1. Run `cargo doc` to build the documentation into the `mys/target` directory. Take note of location of the generated file from the last line of the output, for example `Generated /Users/foo/mys/target/doc/mys_sdk/index.html`.

1. Use a web browser, like Chrome, to open the `.../target/doc/mys_sdk/index.html` file at the location your console reported in the previous step.

## Rust SDK examples

The [examples](https://github.com/MystenLabs/mys/tree/main/crates/mys-sdk/examples) folder provides both basic and advanced examples.

There are serveral files ending in `_api.rs` which provide code examples of the corresponding APIs and their methods. These showcase how to use the MySocial Rust SDK, and can be run against the MySocial testnet. Below are instructions on the prerequisites and how to run these examples.

### Prerequisites

Unless otherwise specified, most of these examples assume `Rust` and `cargo` are installed, and that there is an available internet connection. The examples connect to the MySocial testnet (`https://fullnode.testnet.mys.io:443`) and execute different APIs using the active address from the local wallet. If there is no local wallet, it will create one, generate two addresses, set one of them to be active, and it will request 1 MYS from the testnet faucet for the active address.

### Running the existing examples

In the root folder of the `mys` repository (or in the `mys-sdk` crate folder), you can individually run examples using the command  `cargo run --example filename` (without `.rs` extension). For example:
* `cargo run --example mys_client` -- this one requires a local MySocial network running (see [here](#Connecting to MySocial Network
)). If you do not have a local MySocial network running, please skip this example.
* `cargo run --example coin_read_api`
* `cargo run --example event_api` -- note that this will subscribe to a stream and thus the program will not terminate unless forced (Ctrl+C)
* `cargo run --example governance_api`
* `cargo run --example read_api`
* `cargo run --example programmable_transactions_api`
* `cargo run --example sign_tx_guide`

### Basic Examples

#### Connecting to MySocial Network
The `MysClientBuilder` struct provides a connection to the JSON-RPC server that you use for all read-only operations. The default URLs to connect to the MySocial network are:

- Local: http://127.0.0.1:9000
- Devnet: https://fullnode.devnet.mys.io:443
- Testnet: https://fullnode.testnet.mys.io:443
- Mainnet: https://fullnode.mainnet.mys.io:443

For all available servers, see [here](https://mys.io/networkinfo).

For running a local MySocial network, please follow [this guide](https://docs.mys.io/build/mys-local-network) for installing MySocial and [this guide](https://docs.mys.io/build/mys-local-network#start-the-local-network) for starting the local MySocial network.


```rust
use mys_sdk::MysClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mys = MysClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("MySocial local network version: {}", mys.api_version());

    // local MySocial network, like the above one but using the dedicated function
    let mys_local = MysClientBuilder::default().build_localnet().await?;
    println!("MySocial local network version: {}", mys_local.api_version());

    // MySocial devnet -- https://fullnode.devnet.mys.io:443
    let mys_devnet = MysClientBuilder::default().build_devnet().await?;
    println!("MySocial devnet version: {}", mys_devnet.api_version());

    // MySocial testnet -- https://fullnode.testnet.mys.io:443
    let mys_testnet = MysClientBuilder::default().build_testnet().await?;
    println!("MySocial testnet version: {}", mys_testnet.api_version());

    Ok(())
}
```

#### Read the total coin balance for each coin type owned by this address
```rust
use std::str::FromStr;
use mys_sdk::types::base_types::MysAddress;
use mys_sdk::{ MysClientBuilder};
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

   let mys_local = MysClientBuilder::default().build_localnet().await?;
   println!("MySocial local network version: {}", mys_local.api_version());

   let active_address = MysAddress::from_str("<YOUR MYS ADDRESS>")?; // change to your MySocial address

   let total_balance = mys_local
      .coin_read_api()
      .get_all_balances(active_address)
      .await?;
   println!("The balances for all coins owned by address: {active_address} are {:#?}", total_balance);
   Ok(())
}
```

## Advanced examples

See the programmable transactions [example](https://github.com/MystenLabs/mys/blob/main/crates/mys-sdk/examples/programmable_transactions_api.rs).

## Games examples

### Tic Tac Toe quick start

1. Prepare the environment
   1. Install `mys` binary following the [MySocial installation](https://github.com/MystenLabs/mys/blob/main/docs/content/guides/developer/getting-started/mys-install.mdx) docs.
   1. [Connect to MySocial Devnet](https://github.com/MystenLabs/mys/blob/main/docs/content/guides/developer/getting-started/connect.mdx).
   1. [Make sure you have two addresses with gas](https://github.com/MystenLabs/mys/blob/main/docs/content/guides/developer/getting-started/get-address.mdx) by using the `new-address` command to create new addresses:
      ```shell
      mys client new-address ed25519
      ```
      You must specify the key scheme, one of `ed25519` or `secp256k1` or `secp256r1`.
      You can skip this step if you are going to play with a friend. :)
   1. [Request MySocial tokens](https://github.com/MystenLabs/mys/blob/main/docs/content/guides/developer/getting-started/get-coins.mdx) for all addresses that will be used to join the game.

2. Publish the move contract
   1. [Download the MySocial source code](https://github.com/MystenLabs/mys/blob/main/docs/content/guides/developer/getting-started/mys-install.mdx).
   1. Publish the [`tic-tac-toe` package](https://github.com/MystenLabs/mys/tree/main/examples/tic-tac-toe/move)
      using the MySocial client:
      ```shell
      mys client publish --path /path-to-mys-source-code/examples/tic-tac-toe/move
      ```
   1. Record the package object ID.

3. Create a new tic-tac-toe game
   1. Run the following command in the [`tic-tac-toe/cli` directory](https://github.com/MystenLabs/mys/tree/main/examples/tic-tac-toe/cli) to start a new game, replacing the game package objects ID with the one you recorded:
      ```shell
      cargo run -- new --package-id <<tic-tac-toe package object ID>> <<player O address>>
      ```
      This will create a game between the active address in the keystore, and the specified Player O.
   1. Copy the game ID and pass it to your friend to join the game.

4. Making a move

   Run the following command in the [`tic-tac-toe/cli` directory](https://github.com/MystenLabs/mys/tree/main/examples/tic-tac-toe/cli) to make a move in an existing game, as the active address in the CLI, replacing the game ID and address accordingly:
   ```shell
   cargo run -- move --package-id <<tic-tac-toe package object ID>> --row $R --col $C <<game ID>>
   ```

## License

[SPDX-License-Identifier: Apache-2.0](https://github.com/MystenLabs/mys/blob/main/LICENSE)
