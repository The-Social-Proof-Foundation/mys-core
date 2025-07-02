// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use camino::Utf8PathBuf;
use mys_config::local_ip_utils;
use mys_genesis_builder::validator_info::ValidatorInfo;
use mys_genesis_builder::Builder;
use mys_types::base_types::MysAddress;
use mys_types::crypto::{
    generate_proof_of_possession, get_key_pair_from_rng, AccountKeyPair, AuthorityKeyPair,
    KeypairTraits, NetworkKeyPair,
};

#[tokio::main]
async fn main() {
    let dir = std::env::current_dir().unwrap();
    let dir = Utf8PathBuf::try_from(dir).unwrap();

    let mut builder = Builder::new();
    
    // Add custom token parameters
    builder = builder.with_token_parameters(
        "MySo".to_string(),
        "MySocial".to_string(),
        "The native token of the MySocial blockchain.".to_string()
    );
    
    // Example: Add treasury vesting allocations
    // Treasury team allocation with 4-year linear vesting starting in 1 year
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    let one_year_ms = 365 * 24 * 60 * 60 * 1000; // 1 year in milliseconds
    let four_years_ms = 4 * one_year_ms; // 4 years in milliseconds
    
    let treasury_start = now + one_year_ms; // Start vesting in 1 year
    let treasury_duration = four_years_ms; // Vest over 4 years
    
    // Example treasury allocations (10% of total supply for treasury)
    let total_supply_mist = 1_000_000_000_000_000_000u64; // 1 billion tokens
    let treasury_allocation = total_supply_mist / 10; // 10% for treasury
    
    // Split treasury between multiple recipients
    let treasury_recipients = vec![
        // (address, amount_mist)
        (MysAddress::random_for_testing_only(), treasury_allocation / 3), // Core team 1
        (MysAddress::random_for_testing_only(), treasury_allocation / 3), // Core team 2  
        (MysAddress::random_for_testing_only(), treasury_allocation / 3), // Foundation
    ];
    
    builder = builder.add_treasury_vesting_batch(
        treasury_recipients,
        treasury_start,
        treasury_duration,
    );
    
    let mut keys = Vec::new();
    for i in 0..2 {
        let key: AuthorityKeyPair = get_key_pair_from_rng(&mut rand::rngs::OsRng).1;
        let worker_key: NetworkKeyPair = get_key_pair_from_rng(&mut rand::rngs::OsRng).1;
        let account_key: AccountKeyPair = get_key_pair_from_rng(&mut rand::rngs::OsRng).1;
        let network_key: NetworkKeyPair = get_key_pair_from_rng(&mut rand::rngs::OsRng).1;
        let validator = ValidatorInfo {
            name: format!("Validator {}", i),
            protocol_key: key.public().into(),
            worker_key: worker_key.public().clone(),
            account_address: MysAddress::from(account_key.public()),
            network_key: network_key.public().clone(),
            gas_price: mys_config::node::DEFAULT_VALIDATOR_GAS_PRICE,
            commission_rate: mys_config::node::DEFAULT_COMMISSION_RATE,
            network_address: local_ip_utils::new_local_tcp_address_for_testing(),
            p2p_address: local_ip_utils::new_local_udp_address_for_testing(),
            narwhal_primary_address: local_ip_utils::new_local_udp_address_for_testing(),
            narwhal_worker_address: local_ip_utils::new_local_udp_address_for_testing(),
            description: String::new(),
            image_url: String::new(),
            project_url: String::new(),
        };
        let pop = generate_proof_of_possession(&key, account_key.public().into());
        keys.push(key);
        builder = builder.add_validator(validator, pop);
    }

    for key in keys {
        builder = builder.add_validator_signature(&key);
    }

    builder.save(dir).unwrap();
}
