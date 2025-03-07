#[test_only]
module mys::event_tests {
    use std::vector;
    use mys::event;
    use mys::test_scenario::{Self as ts, Scenario};

    const TEST_SENDER: address = @0xCAFE;

    /// Event to emit in tests
    struct TestEvent has copy, drop {
        id: u64,
        value: u64,
    }

    /// Another event type to test multiple event types
    struct AnotherEvent has copy, drop {
        name: vector<u8>,
    }

    #[test]
    fun test_event_emit_basic() {
        let scenario = ts::begin(TEST_SENDER);
        test_event_emit_(&mut scenario);
        ts::end(scenario);
    }

    fun test_event_emit_(test: &mut Scenario) {
        // Initial state: no events
        ts::next_tx(test, TEST_SENDER);
        {
            assert!(event::num_events() == 0, 0);
        };
        
        // Emit a single event
        ts::next_tx(test, TEST_SENDER);
        {
            // Emit a test event
            event::emit(TestEvent { id: 1, value: 100 });
            
            // Verify event was emitted
            assert!(event::num_events() == 1, 0);
            
            // Check events by type
            let events = event::events_by_type<TestEvent>();
            assert!(vector::length(&events) == 1, 0);
            
            // Verify event data
            let emitted_event = vector::borrow(&events, 0);
            assert!(emitted_event.id == 1, 0);
            assert!(emitted_event.value == 100, 0);
        };
        
        // Emit multiple events of the same type
        ts::next_tx(test, TEST_SENDER);
        {
            // Emit multiple events
            event::emit(TestEvent { id: 2, value: 200 });
            event::emit(TestEvent { id: 3, value: 300 });
            
            // Events accumulate across transactions in test mode
            assert!(event::num_events() == 3, 0);
            
            // Check all events
            let events = event::events_by_type<TestEvent>();
            assert!(vector::length(&events) == 3, 0);
            
            // Verify the first event from previous transaction
            let event1 = vector::borrow(&events, 0);
            assert!(event1.id == 1, 0);
            assert!(event1.value == 100, 0);
            
            // Verify events from this transaction
            let event2 = vector::borrow(&events, 1);
            assert!(event2.id == 2, 0);
            assert!(event2.value == 200, 0);
            
            let event3 = vector::borrow(&events, 2);
            assert!(event3.id == 3, 0);
            assert!(event3.value == 300, 0);
        };
    }
    
    #[test]
    fun test_multiple_event_types() {
        let scenario = ts::begin(TEST_SENDER);
        
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            // Emit different event types
            event::emit(TestEvent { id: 1, value: 100 });
            event::emit(AnotherEvent { name: b"test" });
            
            // Check total event count
            assert!(event::num_events() == 2, 0);
            
            // Check events by specific type
            let test_events = event::events_by_type<TestEvent>();
            assert!(vector::length(&test_events) == 1, 0);
            
            let another_events = event::events_by_type<AnotherEvent>();
            assert!(vector::length(&another_events) == 1, 0);
            
            // Verify event data for different types
            let test_event = vector::borrow(&test_events, 0);
            assert!(test_event.id == 1, 0);
            assert!(test_event.value == 100, 0);
            
            let another_event = vector::borrow(&another_events, 0);
            assert!(another_event.name == b"test", 0);
        };
        
        ts::end(scenario);
    }
    
    #[test]
    fun test_event_in_loop() {
        let scenario = ts::begin(TEST_SENDER);
        
        ts::next_tx(&mut scenario, TEST_SENDER);
        {
            // Emit events in a loop
            let i = 0;
            while (i < 5) {
                event::emit(TestEvent { id: i, value: i * 100 });
                i = i + 1;
            };
            
            // Check total count
            assert!(event::num_events() == 5, 0);
            
            // Check all events
            let events = event::events_by_type<TestEvent>();
            assert!(vector::length(&events) == 5, 0);
            
            // Verify each event data
            let i = 0;
            while (i < 5) {
                let event = vector::borrow(&events, i);
                assert!(event.id == i, 0);
                assert!(event.value == i * 100, 0);
                i = i + 1;
            };
        };
        
        ts::end(scenario);
    }
}