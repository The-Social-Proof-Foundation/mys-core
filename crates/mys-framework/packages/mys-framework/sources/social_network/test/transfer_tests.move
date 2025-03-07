#[test_only]
module mys::transfer_tests {
    use mys::transfer;
    use mys::object::{Self, UID};
    use mys::test_scenario::{Self as ts, Scenario};

    const TEST_SENDER: address = @0xCAFE;
    const RECIPIENT: address = @0xDEAD;

    /// Object that can be transferred
    struct TransferableObj has key, store {
        id: UID,
        value: u64,
    }

    /// Object without store ability - can only be transferred within its module
    struct RestrictedObj has key {
        id: UID,
        value: u64,
    }

    #[test]
    fun test_public_transfer() {
        let scenario = ts::begin(TEST_SENDER);
        test_public_transfer_(&mut scenario);
        ts::end(scenario);
    }

    fun test_public_transfer_(test: &mut Scenario) {
        // Create and transfer an object
        ts::next_tx(test, TEST_SENDER);
        {
            let ctx = ts::ctx(test);
            
            // Create object with store ability
            let obj = TransferableObj {
                id: object::new(ctx),
                value: 42,
            };
            
            // Public transfer should work because object has 'store' ability
            transfer::public_transfer(obj, RECIPIENT);
        };
        
        // Verify the recipient received the object
        ts::next_tx(test, RECIPIENT);
        {
            let obj = ts::take_from_sender<TransferableObj>(test);
            assert!(obj.value == 42, 0);
            
            // Transfer it back to test_sender
            transfer::public_transfer(obj, TEST_SENDER);
        };
    }

    #[test]
    fun test_module_transfer() {
        let scenario = ts::begin(TEST_SENDER);
        
        // Create and transfer an object using module-internal transfer
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create object without store ability
            let obj = RestrictedObj {
                id: object::new(ctx),
                value: 42,
            };
            
            // Use module-internal transfer (this works because we're in the defining module)
            transfer::transfer(obj, RECIPIENT);
        };
        
        // Verify the recipient received the object
        ts::next_tx(&mut scenario, RECIPIENT);
        {
            let obj = ts::take_from_sender<RestrictedObj>(&mut scenario);
            assert!(obj.value == 42, 0);
            
            // Transfer it back to test_sender
            transfer::transfer(obj, TEST_SENDER);
        };
        
        ts::end(scenario);
    }

    #[test]
    fun test_freeze_object() {
        let scenario = ts::begin(TEST_SENDER);
        
        // Create and freeze an object
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create a transferable object
            let obj = TransferableObj {
                id: object::new(ctx),
                value: 42,
            };
            
            // Freeze the object using public_freeze_object
            transfer::public_freeze_object(obj);
        };
        
        // Try to access the frozen object
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            // Verify the object is now immutable and can be taken
            assert!(ts::has_most_recent_immutable<TransferableObj>(), 0);
            
            let frozen_obj = ts::take_immutable<TransferableObj>(&mut scenario);
            assert!(frozen_obj.value == 42, 0);
            
            // Return the immutable object
            ts::return_immutable(frozen_obj);
        };
        
        ts::end(scenario);
    }
    
    #[test]
    fun test_freeze_restricted_object() {
        let scenario = ts::begin(TEST_SENDER);
        
        // Create and freeze a restricted object
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create a restricted object
            let obj = RestrictedObj {
                id: object::new(ctx),
                value: 42,
            };
            
            // Freeze the object using module-internal freeze_object
            transfer::freeze_object(obj);
        };
        
        // Try to access the frozen object
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            // Verify the object is now immutable and can be taken
            assert!(ts::has_most_recent_immutable<RestrictedObj>(), 0);
            
            let frozen_obj = ts::take_immutable<RestrictedObj>(&mut scenario);
            assert!(frozen_obj.value == 42, 0);
            
            // Return the immutable object
            ts::return_immutable(frozen_obj);
        };
        
        ts::end(scenario);
    }
    
    #[test]
    fun test_share_object() {
        let scenario = ts::begin(TEST_SENDER);
        
        // Create and share an object
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create a transferable object
            let obj = TransferableObj {
                id: object::new(ctx),
                value: 42,
            };
            
            // Share the object using public_share_object
            transfer::public_share_object(obj);
        };
        
        // Access the shared object from the original sender
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            // Verify the object is now shared and can be taken
            assert!(ts::has_most_recent_shared<TransferableObj>(), 0);
            
            let shared_obj = ts::take_shared<TransferableObj>(&mut scenario);
            assert!(shared_obj.value == 42, 0);
            
            // Modify the shared object
            shared_obj.value = 100;
            
            // Return the shared object
            ts::return_shared(shared_obj);
        };
        
        // Access the shared object from a different sender
        ts::next_tx(&mut scenario, RECIPIENT);
        {
            // Verify the object is shared and can be taken by a different account
            assert!(ts::has_most_recent_shared<TransferableObj>(), 0);
            
            let shared_obj = ts::take_shared<TransferableObj>(&mut scenario);
            
            // Verify the modification from the previous transaction
            assert!(shared_obj.value == 100, 0);
            
            // Modify the shared object again
            shared_obj.value = 200;
            
            // Return the shared object
            ts::return_shared(shared_obj);
        };
        
        ts::end(scenario);
    }
    
    #[test]
    fun test_share_restricted_object() {
        let scenario = ts::begin(TEST_SENDER);
        
        // Create and share a restricted object
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create a restricted object
            let obj = RestrictedObj {
                id: object::new(ctx),
                value: 42,
            };
            
            // Share the object using module-internal share_object
            transfer::share_object(obj);
        };
        
        // Access the shared object from a different sender
        ts::next_tx(&mut scenario, RECIPIENT);
        {
            // Verify the object is shared and can be taken by a different account
            assert!(ts::has_most_recent_shared<RestrictedObj>(), 0);
            
            let shared_obj = ts::take_shared<RestrictedObj>(&mut scenario);
            assert!(shared_obj.value == 42, 0);
            
            // Modify the shared object
            shared_obj.value = 100;
            
            // Return the shared object
            ts::return_shared(shared_obj);
        };
        
        ts::end(scenario);
    }
}