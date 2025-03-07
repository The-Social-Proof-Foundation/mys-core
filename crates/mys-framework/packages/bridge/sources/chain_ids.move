// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids {

    // Chain IDs
    const MysMainnet: u8 = 0;
    const MysTestnet: u8 = 1;
    const MysCustom: u8 = 2;

    const EthMainnet: u8 = 10;
    const EthSepolia: u8 = 11;
    const EthCustom: u8 = 12;

    const EInvalidBridgeRoute: u64 = 0;

    //////////////////////////////////////////////////////
    // Types
    //

    public struct BridgeRoute has copy, drop, store {
        source: u8,
        destination: u8,
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    public fun mys_mainnet(): u8 { MysMainnet }
    public fun mys_testnet(): u8 { MysTestnet }
    public fun mys_custom(): u8 { MysCustom }

    public fun eth_mainnet(): u8 { EthMainnet }
    public fun eth_sepolia(): u8 { EthSepolia }
    public fun eth_custom(): u8 { EthCustom }

    public use fun route_source as BridgeRoute.source;
    public fun route_source(route: &BridgeRoute): &u8 {
        &route.source
    }

    public use fun route_destination as BridgeRoute.destination;
    public fun route_destination(route: &BridgeRoute): &u8 {
        &route.destination
    }

    public fun assert_valid_chain_id(id: u8) {
        assert!(
            id == MysMainnet ||
            id == MysTestnet ||
            id == MysCustom ||
            id == EthMainnet ||
            id == EthSepolia ||
            id == EthCustom,
            EInvalidBridgeRoute
        )
    }

    public fun valid_routes(): vector<BridgeRoute> {
        vector[
            BridgeRoute { source: MysMainnet, destination: EthMainnet },
            BridgeRoute { source: EthMainnet, destination: MysMainnet },

            BridgeRoute { source: MysTestnet, destination: EthSepolia },
            BridgeRoute { source: MysTestnet, destination: EthCustom },
            BridgeRoute { source: MysCustom, destination: EthCustom },
            BridgeRoute { source: MysCustom, destination: EthSepolia },
            BridgeRoute { source: EthSepolia, destination: MysTestnet },
            BridgeRoute { source: EthSepolia, destination: MysCustom },
            BridgeRoute { source: EthCustom, destination: MysTestnet },
            BridgeRoute { source: EthCustom, destination: MysCustom }
        ]
    }

    public fun is_valid_route(source: u8, destination: u8): bool {
        let route = BridgeRoute { source, destination };
        valid_routes().contains(&route)
    }

    // Checks and return BridgeRoute if the route is supported by the bridge.
    public fun get_route(source: u8, destination: u8): BridgeRoute {
        let route = BridgeRoute { source, destination };
        assert!(valid_routes().contains(&route), EInvalidBridgeRoute);
        route
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test]
    fun test_chains_ok() {
        assert_valid_chain_id(MysMainnet);
        assert_valid_chain_id(MysTestnet);
        assert_valid_chain_id(MysCustom);
        assert_valid_chain_id(EthMainnet);
        assert_valid_chain_id(EthSepolia);
        assert_valid_chain_id(EthCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_chains_error() {
        assert_valid_chain_id(100);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_mys_chains_error() {
        // this will break if we add one more mys chain id and should be corrected
        assert_valid_chain_id(4);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_eth_chains_error() {
        // this will break if we add one more eth chain id and should be corrected
        assert_valid_chain_id(13);
    }

    #[test]
    fun test_routes() {
        let valid_routes = vector[
            BridgeRoute { source: MysMainnet, destination: EthMainnet },
            BridgeRoute { source: EthMainnet, destination: MysMainnet },

            BridgeRoute { source: MysTestnet, destination: EthSepolia },
            BridgeRoute { source: MysTestnet, destination: EthCustom },
            BridgeRoute { source: MysCustom, destination: EthCustom },
            BridgeRoute { source: MysCustom, destination: EthSepolia },
            BridgeRoute { source: EthSepolia, destination: MysTestnet },
            BridgeRoute { source: EthSepolia, destination: MysCustom },
            BridgeRoute { source: EthCustom, destination: MysTestnet },
            BridgeRoute { source: EthCustom, destination: MysCustom }
        ];
        let mut size = valid_routes.length();
        while (size > 0) {
            size = size - 1;
            let route = valid_routes[size];
            assert!(is_valid_route(route.source, route.destination)); // sould not assert
        }
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_mys_1() {
        get_route(MysMainnet, MysMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_mys_2() {
        get_route(MysMainnet, MysTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_mys_3() {
        get_route(MysMainnet, EthSepolia);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_mys_4() {
        get_route(MysMainnet, EthCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_1() {
        get_route(EthMainnet, EthMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_2() {
        get_route(EthMainnet, EthCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_3() {
        get_route(EthMainnet, MysCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_4() {
        get_route(EthMainnet, MysTestnet);
    }
}
