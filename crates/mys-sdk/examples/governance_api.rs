// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod utils;
use utils::setup_for_read;

// This example connects to the Mys testnet
// and collects information about the stakes in the network,
// the committee information,
// lists all the validators' name, description, and mys address,
// and prints the reference gas price.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (mys, active_address) = setup_for_read().await?;

    // ************ GOVERNANCE API ************ //

    // Stakes
    let stakes = mys.governance_api().get_stakes(active_address).await?;

    println!(" *** Stakes ***");
    println!("{:?}", stakes);
    println!(" *** Stakes ***\n");

    // Committee Info
    let committee = mys.governance_api().get_committee_info(None).await?; // None defaults to the latest epoch

    println!(" *** Committee Info ***");
    println!("{:?}", committee);
    println!(" *** Committee Info ***\n");

    // Latest Mys System State
    let mys_system_state = mys.governance_api().get_latest_mys_system_state().await?;

    println!(" *** Mys System State ***");
    println!("{:?}", mys_system_state);
    println!(" *** Mys System State ***\n");

    // List all active validators

    println!(" *** List active validators *** ");
    mys_system_state
        .active_validators
        .into_iter()
        .for_each(|validator| {
            println!(
                "Name: {}, Description: {}, MysAddress: {:?}",
                validator.name, validator.description, validator.mys_address
            )
        });

    println!(" *** List active validators ***\n");
    // Reference Gas Price
    let reference_gas_price = mys.governance_api().get_reference_gas_price().await?;

    println!(" *** Reference Gas Price ***");
    println!("{:?}", reference_gas_price);
    println!(" *** Reference Gas Price ***\n");

    // ************ END OF GOVERNANCE API ************ //
    Ok(())
}
