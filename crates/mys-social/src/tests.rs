// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mys_json_rpc_api::SocialReadApiServer;
    use mys_types::base_types::{MysAddress, ObjectID};
    use std::str::FromStr;
    
    use crate::{SocialApi, metrics::SocialApiMetrics};
    use mys_sdk::rpc_types::MysClient;

    // A minimal mock MysClient for testing
    struct MockMysClient;

    #[async_trait::async_trait]
    impl MysClient for MockMysClient {
        // Implement the necessary methods for the mock
        // This is a minimal implementation for testing
        async fn get_latest_checkpoint_sequence_number(&self) -> anyhow::Result<u64> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_get_value_by_id() {
        let client = Arc::new(MockMysClient);
        let metrics = SocialApiMetrics::new_for_tests();
        let api = SocialApi::new(client, metrics);
        
        // Create a test object ID
        let value_id = ObjectID::from_str(
            "0x1234567812345678123456781234567812345678123456781234567812345678"
        ).unwrap();
        
        // Call get_value_by_id
        let result = api.get_value_by_id(value_id).await.unwrap();
        
        // Verify result is Some
        assert!(result.is_some());
        
        // Verify the object ID matches
        let value_data = result.unwrap();
        assert_eq!(value_data.value_id, value_id);
    }

    #[tokio::test]
    async fn test_get_values_by_owner() {
        let client = Arc::new(MockMysClient);
        let metrics = SocialApiMetrics::new_for_tests();
        let api = SocialApi::new(client, metrics);
        
        // Create a test address
        let owner = MysAddress::from_str("0x1234").unwrap();
        
        // Call get_values_by_owner
        let result = api.get_values_by_owner(owner, None, None).await.unwrap();
        
        // Verify there's at least one value
        assert!(!result.data.is_empty());
        
        // Verify the owner matches
        let value_data = &result.data[0];
        assert_eq!(value_data.owner, owner);
    }

    #[tokio::test]
    async fn test_set_value() {
        let client = Arc::new(MockMysClient);
        let metrics = SocialApiMetrics::new_for_tests();
        let api = SocialApi::new(client, metrics);
        
        // Create a test address
        let owner = MysAddress::from_str("0x1234").unwrap();
        
        // Call set_value
        let result = api.set_value(owner, 100).await.unwrap();
        
        // Verify the result is true
        assert!(result);
    }
}