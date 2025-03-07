// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use mys_json_rpc::name_service::{self, Domain};
use mys_types::{
    base_types::{ObjectID, MysAddress},
    collection_types::VecMap,
};

#[test]
fn test_parent_extraction() {
    let mut name = Domain::from_str("leaf.node.test.mys").unwrap();

    assert_eq!(name.parent().to_string(), "node.test.mys");

    name = Domain::from_str("node.test.mys").unwrap();

    assert_eq!(name.parent().to_string(), "test.mys");
}

#[test]
fn test_expirations() {
    let system_time: u64 = 100;

    let mut name = name_service::NameRecord {
        nft_id: mys_types::id::ID::new(ObjectID::random()),
        data: VecMap { contents: vec![] },
        target_address: Some(MysAddress::random_for_testing_only()),
        expiration_timestamp_ms: system_time + 10,
    };

    assert!(!name.is_node_expired(system_time));

    name.expiration_timestamp_ms = system_time - 10;

    assert!(name.is_node_expired(system_time));
}

#[test]
fn test_name_service_outputs() {
    assert_eq!("@test".parse::<Domain>().unwrap().to_string(), "test.mys");
    assert_eq!(
        "test.mys".parse::<Domain>().unwrap().to_string(),
        "test.mys"
    );
    assert_eq!(
        "test@sld".parse::<Domain>().unwrap().to_string(),
        "test.sld.mys"
    );
    assert_eq!(
        "test.test@example".parse::<Domain>().unwrap().to_string(),
        "test.test.example.mys"
    );
    assert_eq!(
        "mys@mys".parse::<Domain>().unwrap().to_string(),
        "mys.mys.mys"
    );

    assert_eq!("@mys".parse::<Domain>().unwrap().to_string(), "mys.mys");

    assert_eq!(
        "test*test@test".parse::<Domain>().unwrap().to_string(),
        "test.test.test.mys"
    );
    assert_eq!(
        "test.test.mys".parse::<Domain>().unwrap().to_string(),
        "test.test.mys"
    );
    assert_eq!(
        "test.test.test.mys".parse::<Domain>().unwrap().to_string(),
        "test.test.test.mys"
    );
}

#[test]
fn test_different_wildcard() {
    assert_eq!("test.mys".parse::<Domain>(), "test*mys".parse::<Domain>(),);

    assert_eq!("@test".parse::<Domain>(), "test*mys".parse::<Domain>(),);
}

#[test]
fn test_invalid_inputs() {
    assert!("*".parse::<Domain>().is_err());
    assert!(".".parse::<Domain>().is_err());
    assert!("@".parse::<Domain>().is_err());
    assert!("@inner.mys".parse::<Domain>().is_err());
    assert!("@inner*mys".parse::<Domain>().is_err());
    assert!("test@".parse::<Domain>().is_err());
    assert!("mys".parse::<Domain>().is_err());
    assert!("test.test@example.mys".parse::<Domain>().is_err());
    assert!("test@test@example".parse::<Domain>().is_err());
}

#[test]
fn output_tests() {
    let mut domain = "test.mys".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.mys");
    assert!(domain.format(name_service::DomainFormat::At) == "@test");

    domain = "test.test.mys".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.mys");
    assert!(domain.format(name_service::DomainFormat::At) == "test@test");

    domain = "test.test.test.mys".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.test.mys");
    assert!(domain.format(name_service::DomainFormat::At) == "test.test@test");

    domain = "test.test.test.test.mys".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.test.test.mys");
    assert!(domain.format(name_service::DomainFormat::At) == "test.test.test@test");
}
