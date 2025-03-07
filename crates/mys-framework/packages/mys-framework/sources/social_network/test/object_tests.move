#[test_only]
module mys::object_tests {
    use std::vector;
    use mys::object::{Self, ID, UID};
    use mys::test_scenario::{Self as ts, Scenario};
    use mys::tx_context::TxContext;

    const TEST_SENDER: address = @0xCAFE;

    /// Test object to verify ID functionality
    struct TestObject has key {
        id: UID,
        value: u64,
    }

    #[test]
    fun test_object_creation_and_deletion() {
        let scenario = ts::begin(TEST_SENDER);
        test_create_delete_flow(&mut scenario);
        ts::end(scenario);
    }

    fun test_create_delete_flow(test: &mut Scenario) {
        // Test object creation
        ts::next_tx(test, TEST_SENDER);
        {
            let ctx = ts::ctx(test);
            
            // Create a new UID
            let id = object::new(ctx);
            
            // Create a test object with the UID
            let obj = TestObject {
                id,
                value: 42,
            };
            
            // Get the ID from the object
            let obj_id = object::id(&obj);
            
            // Test ID to bytes conversion
            let id_bytes = object::id_to_bytes(&obj_id);
            assert!(vector::length(&id_bytes) > 0, 0);
            
            // Test ID to address conversion
            let id_addr = object::id_to_address(&obj_id);
            
            // Reconstruct ID from address
            let reconstructed_id = object::id_from_address(id_addr);
            
            // Verify reconstructed ID matches original
            assert!(object::id_to_address(&reconstructed_id) == object::id_to_address(&obj_id), 0);
            
            // Test UID functions
            let uid_inner = object::uid_as_inner(&obj.id);
            assert!(object::id_to_address(uid_inner) == object::id_to_address(&obj_id), 0);
            
            let uid_to_inner = object::uid_to_inner(&obj.id);
            assert!(object::id_to_address(&uid_to_inner) == object::id_to_address(&obj_id), 0);
            
            let uid_bytes = object::uid_to_bytes(&obj.id);
            assert!(vector::length(&uid_bytes) > 0, 0);
            
            let uid_addr = object::uid_to_address(&obj.id);
            assert!(uid_addr == id_addr, 0);
            
            // Unpack and delete the object
            let TestObject { id, value: _ } = obj;
            object::delete(id);
        };
    }
    
    #[test]
    fun test_id_from_bytes() {
        // Create test bytes for an ID
        let test_bytes = x"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        
        // Convert bytes to ID
        let id = object::id_from_bytes(test_bytes);
        
        // Convert ID back to bytes and check they match
        let bytes_from_id = object::id_to_bytes(&id);
        assert!(bytes_from_id == test_bytes, 0);
        
        // Also test address conversion
        let addr = object::id_to_address(&id);
        let id_from_addr = object::id_from_address(addr);
        
        // Verify ID from address matches original ID
        assert!(object::id_to_address(&id) == object::id_to_address(&id_from_addr), 0);
    }
    
    #[test]
    fun test_object_id_methods() {
        let scenario = ts::begin(TEST_SENDER);
        
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create a test object
            let test_obj = create_test_object(ctx);
            
            // Test various ID methods on the object
            let obj_id = object::id(&test_obj);
            let obj_borrow_id = object::borrow_id(&test_obj);
            let obj_id_bytes = object::id_bytes(&test_obj);
            let obj_id_addr = object::id_address(&test_obj);
            
            // Verify ID methods return consistent results
            assert!(object::id_to_address(&obj_id) == object::id_to_address(obj_borrow_id), 0);
            assert!(object::id_to_address(&obj_id) == obj_id_addr, 0);
            assert!(object::id_to_bytes(&obj_id) == obj_id_bytes, 0);
            
            // Clean up
            let TestObject { id, value: _ } = test_obj;
            object::delete(id);
        };
        
        ts::end(scenario);
    }
    
    #[test]
    fun test_last_created() {
        let scenario = ts::begin(TEST_SENDER);
        
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            let ctx = ts::ctx(&mut scenario);
            
            // Create a UID
            let id = object::new(ctx);
            
            // Get the last created ID
            let last_id = object::last_created(ctx);
            
            // They should match since this was the last object created
            assert!(object::id_to_address(&object::uid_to_inner(&id)) == 
                    object::id_to_address(&last_id), 0);
            
            // Clean up
            object::delete(id);
        };
        
        ts::end(scenario);
    }
    
    // Helper to create a test object
    fun create_test_object(ctx: &mut TxContext): TestObject {
        TestObject {
            id: object::new(ctx),
            value: 42,
        }
    }
}