#[test_only]
module mys::coin_tests {
    use std::ascii;
    use std::string;
    use mys::coin::{Self, Coin, TreasuryCap, CoinMetadata};
    use mys::test_scenario::{Self as ts, Scenario};
    use mys::url;

    // One-time witness for the test coin
    struct TEST_COIN has drop {}

    const TEST_SENDER: address = @0xCAFE;

    #[test]
    fun test_coin_creation() {
        let scenario = ts::begin(TEST_SENDER);
        test_coin_creation_(&mut scenario);
        ts::end(scenario);
    }

    fun test_coin_creation_(test: &mut Scenario) {
        let dummy_witness = TEST_COIN {};
        
        // Test creating a new currency
        ts::next_tx(test, TEST_SENDER);
        {
            let (treasury_cap, metadata) = coin::create_currency(
                dummy_witness,
                8, // decimals
                b"TEST", // symbol
                b"Test Coin", // name
                b"A test coin for unit tests", // description
                option::none(), // no icon url
                ts::ctx(test)
            );

            // Verify metadata was created correctly
            assert!(coin::get_name(&metadata) == string::utf8(b"Test Coin"), 0);
            assert!(coin::get_symbol(&metadata) == ascii::string(b"TEST"), 0);
            assert!(coin::get_description(&metadata) == string::utf8(b"A test coin for unit tests"), 0);
            assert!(option::is_none(&coin::get_icon_url(&metadata)), 0);
            assert!(coin::get_decimals(&metadata) == 8, 0);

            // Verify total supply is initially zero
            assert!(coin::total_supply(&treasury_cap) == 0, 0);

            // Transfer treasury cap and metadata to sender
            ts::return_to_sender(test, treasury_cap);
            ts::return_to_sender(test, metadata);
        };

        // Test minting coins
        ts::next_tx(test, TEST_SENDER);
        {
            let treasury_cap = ts::take_from_sender<TreasuryCap<TEST_COIN>>(test);
            let metadata = ts::take_from_sender<CoinMetadata<TEST_COIN>>(test);
            
            // Mint some coins
            let coin = coin::mint(&mut treasury_cap, 100, ts::ctx(test));
            assert!(coin::value(&coin) == 100, 0);
            assert!(coin::total_supply(&treasury_cap) == 100, 0);
            
            // Split the coin
            let split_coin = coin::split(&mut coin, 30, ts::ctx(test));
            assert!(coin::value(&coin) == 70, 0);
            assert!(coin::value(&split_coin) == 30, 0);
            
            // Join the coins back
            coin::join(&mut coin, split_coin);
            assert!(coin::value(&coin) == 100, 0);
            
            // Burn the coin
            let burned_amount = coin::burn(&mut treasury_cap, coin);
            assert!(burned_amount == 100, 0);
            assert!(coin::total_supply(&treasury_cap) == 0, 0);
            
            // Update metadata
            coin::update_name(&treasury_cap, &mut metadata, string::utf8(b"Updated Test Coin"));
            coin::update_description(&treasury_cap, &mut metadata, string::utf8(b"Updated description"));
            coin::update_symbol(&treasury_cap, &mut metadata, ascii::string(b"UTEST"));
            coin::update_icon_url(&treasury_cap, &mut metadata, ascii::string(b"https://test.com/icon.png"));
            
            assert!(coin::get_name(&metadata) == string::utf8(b"Updated Test Coin"), 0);
            assert!(coin::get_symbol(&metadata) == ascii::string(b"UTEST"), 0);
            assert!(coin::get_description(&metadata) == string::utf8(b"Updated description"), 0);
            assert!(option::is_some(&coin::get_icon_url(&metadata)), 0);
            
            ts::return_to_sender(test, treasury_cap);
            ts::return_to_sender(test, metadata);
        };
    }

    #[test]
    fun test_regulated_currency() {
        let scenario = ts::begin(TEST_SENDER);
        
        let dummy_witness = TEST_COIN {};
        
        // Test creating a regulated currency
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let (treasury_cap, deny_cap, metadata) = coin::create_regulated_currency_v2(
                dummy_witness,
                6, // decimals
                b"RTST", // symbol
                b"Regulated Test Coin", // name
                b"A regulated test coin", // description
                option::none(), // no icon url
                true, // allow global pause
                ts::ctx(&mut scenario)
            );
            
            assert!(coin::get_name(&metadata) == string::utf8(b"Regulated Test Coin"), 0);
            
            // Transfer caps and metadata
            ts::return_to_sender(&mut scenario, treasury_cap);
            ts::return_to_sender(&mut scenario, deny_cap);
            ts::return_to_sender(&mut scenario, metadata);
        };
        
        ts::end(scenario);
    }

    #[test]
    fun test_divide_into_n() {
        let scenario = ts::begin(TEST_SENDER);
        
        // Create coin and treasury cap
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let dummy_witness = TEST_COIN {};
            let (treasury_cap, metadata) = coin::create_currency(
                dummy_witness,
                8,
                b"TEST",
                b"Test Coin",
                b"A test coin for unit tests",
                option::none(),
                ts::ctx(&mut scenario)
            );
            
            // Mint a coin with value 100
            let coin = coin::mint(&mut treasury_cap, 100, ts::ctx(&mut scenario));
            
            // Divide into 4 coins (should get 3 coins of value 25 each, with 25 remaining in original)
            let coins = coin::divide_into_n(&mut coin, 4, ts::ctx(&mut scenario));
            assert!(vector::length(&coins) == 3, 0);
            assert!(coin::value(&coin) == 25, 0);
            
            // Verify each coin has value 25
            let i = 0;
            while (i < vector::length(&coins)) {
                let split_coin = vector::borrow(&coins, i);
                assert!(coin::value(split_coin) == 25, 0);
                i = i + 1;
            };
            
            // Burn all coins
            burn_coin_vector(&mut treasury_cap, coins);
            let _ = coin::burn(&mut treasury_cap, coin);
            
            ts::return_to_sender(&mut scenario, treasury_cap);
            ts::return_to_sender(&mut scenario, metadata);
        };
        
        ts::end(scenario);
    }

    // Helper to burn a vector of coins
    fun burn_coin_vector<T>(treasury_cap: &mut TreasuryCap<T>, coins: vector<Coin<T>>) {
        while (!vector::is_empty(&coins)) {
            let coin = vector::pop_back(&mut coins);
            let _ = coin::burn(treasury_cap, coin);
        };
        vector::destroy_empty(coins);
    }
}