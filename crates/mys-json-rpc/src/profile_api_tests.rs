// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use jsonrpsee::RpcModule;
    use mys_core::authority::AuthorityState;
    use mys_json_rpc_api::{JsonRpcMetrics, ProfileReadApiServer};
    use mys_json_rpc_types::ProfileData;
    use mys_storage::key_value_store::TransactionKeyValueStore;
    use mys_types::base_types::{MysAddress, ObjectID};
    use mys_types::object::{Object, Owner};

    use mockall::predicate::*;
    use mockall::*;

    use crate::authority_state::StateRead;
    use crate::profile_api::ProfileReadApi;
    use crate::MysRpcModule;

    mock! {
        AuthorityState {}
        
        #[async_trait::async_trait]
        impl StateRead for AuthorityState {
            async fn get_object(&self, object_id: &ObjectID) -> Result<Option<Object>, crate::error::Error>;
            async fn get_owned_objects(
                &self,
                owner: MysAddress,
                cursor: Option<String>,
                limit: Option<usize>,
                filter: Option<String>,
            ) -> Result<Vec<crate::authority_state::OwnedObjectRef>, crate::error::Error>;
        }
    }

    #[tokio::test]
    async fn test_get_profile_by_owner() {
        // Create mock state, transaction store, and metrics
        let mut mock_state = MockAuthorityState::new();
        let transaction_store = Arc::new(TransactionKeyValueStore::default());
        let metrics = Arc::new(JsonRpcMetrics::new_for_tests());

        // Setup test data
        let owner = MysAddress::random_for_testing_only();
        let profile_id = ObjectID::random();
        
        // Setup mock for get_owned_objects
        mock_state
            .expect_get_owned_objects()
            .with(eq(owner), any(), any(), any())
            .returning(move |_, _, _, _| {
                Ok(vec![crate::authority_state::OwnedObjectRef {
                    owner: Owner::AddressOwner(owner),
                    object_id: profile_id,
                    digest: Default::default(),
                }])
            });

        // Setup mock for get_object
        // This would normally return a valid profile object
        // For this test, we'll just have it return None to simplify
        mock_state
            .expect_get_object()
            .with(eq(profile_id))
            .returning(|_| Ok(None));

        // Create the API with mocks
        let profile_api = ProfileReadApi::new(
            Arc::new(mock_state),
            transaction_store,
            metrics,
        );

        // Call the API method
        let result = profile_api.get_profile_by_owner(owner).await;

        // Since our mock returns None for get_object, we expect None as the result
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    // Additional tests would follow the same pattern, creating appropriate mocks
    // and verifying the expected results
}