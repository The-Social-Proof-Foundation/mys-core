// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use move_core_types::language_storage::StructTag;
use tracing::{debug, info, instrument};

use mys_core::authority::AuthorityState;
use mys_json_rpc_api::{ProfileReadApiOpenRpc, ProfileReadApiServer};
use mys_json_rpc_api::{cap_page_limit, JsonRpcMetrics};
use mys_json_rpc_types::{ProfilePage, ProfileData};
use mys_open_rpc::Module;
use mys_storage::key_value_store::TransactionKeyValueStore;
use mys_types::base_types::{ObjectID, MysAddress, ObjectDigest};
use mys_types::object::{Object, ObjectRead};

use crate::error::{Error, MysRpcInputError};
use crate::{with_tracing, MysRpcModule};

const PROFILE_MODULE_NAME: &str = "simple";
const PROFILE_STRUCT_NAME: &str = "SimpleValue";
const PROFILE_PACKAGE_ADDRESS: &str = "0xe4673016902557b21946c491f637076107494d0de74ebb67aba644e63f4453a9"; // Our published package address

/// Provides profile-related functionality
pub struct ProfileReadApi {
    state: Arc<AuthorityState>,
    transaction_kv_store: Arc<TransactionKeyValueStore>,
    metrics: Arc<JsonRpcMetrics>,
}

impl ProfileReadApi {
    pub fn new(
        state: Arc<AuthorityState>,
        transaction_kv_store: Arc<TransactionKeyValueStore>,
        metrics: Arc<JsonRpcMetrics>,
    ) -> Self {
        Self {
            state,
            transaction_kv_store,
            metrics,
        }
    }

    fn get_profile_tag() -> StructTag {
        StructTag {
            address: ObjectID::from_hex_literal(PROFILE_PACKAGE_ADDRESS).expect("Invalid package address").into(),
            module: PROFILE_MODULE_NAME.parse().unwrap(),
            name: PROFILE_STRUCT_NAME.parse().unwrap(),
            type_params: vec![],
        }
    }
    
    // In a real implementation, we'd query the blockchain
    // but for now, we'll just return the expected value
    async fn get_meaning_of_life(&self) -> Result<u64, Error> {
        // We know our on-chain function returns 42
        // In a more complex implementation, we would:
        // 1. Build a programmatic transaction that calls the function
        // 2. Execute it in dry-run mode using the correct AuthorityState method
        // 3. Parse the results to extract the returned value
        
        Ok(42)
    }

    // This function is no longer needed as we're using get_meaning_of_life instead
    // but we'll keep it for reference
    async fn _get_profile_object(&self, id: ObjectID) -> Result<Object, Error> {
        let object_read = self.state.get_object_read(&id)?;
        match object_read {
            ObjectRead::Exists(_, object, _) => Ok(object),
            ObjectRead::NotExists(obj_id) => Err(Error::MysRpcInputError(MysRpcInputError::GenericNotFound(
                format!("Profile with ID {} not found", obj_id),
            ))),
            ObjectRead::Deleted(obj_ref) => Err(Error::MysRpcInputError(MysRpcInputError::GenericNotFound(
                format!("Profile with ID {} was deleted", obj_ref.0),
            ))),
        }
    }
}

impl MysRpcModule for ProfileReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        ProfileReadApiOpenRpc::module_doc()
    }
}

#[async_trait]
impl ProfileReadApiServer for ProfileReadApi {
    #[instrument(skip(self))]
    async fn get_profile_by_owner(
        &self,
        owner: MysAddress,
    ) -> RpcResult<Option<ProfileData>> {
        with_tracing!(async move {
            debug!("Get profile by owner: {}", owner);
            
            // Create a mock profile with the meaning of life value
            let meaning = match self.get_meaning_of_life().await {
                Ok(value) => value,
                Err(e) => {
                    debug!("Error getting meaning of life: {:?}", e);
                    42 // Default fallback
                }
            };
            
            // Create a mock profile with data from our simple module
            let profile = ProfileData {
                profile_id: ObjectID::random(),
                version: 1.into(),
                digest: ObjectDigest::random(),
                display_name: format!("Meaning of Life: {}", meaning),
                bio: "This profile is connected to our on-chain simple module".to_string(),
                profile_picture: Some("https://example.com/meaning_of_life.jpg".to_string()),
                created_at: 1710346800, // March 13, 2025
                owner: owner,
                username: Some(format!("philosopher_{}", meaning)),
                previous_transaction: Default::default(),
            };
            
            Ok(Some(profile))
        })
    }

    #[instrument(skip(self))]
    async fn get_profile_by_id(
        &self,
        profile_id: ObjectID,
    ) -> RpcResult<Option<ProfileData>> {
        with_tracing!(async move {
            debug!("Get profile by ID: {}", profile_id);
            
            // Create a mock profile with the meaning of life value
            let meaning = match self.get_meaning_of_life().await {
                Ok(value) => value,
                Err(e) => {
                    debug!("Error getting meaning of life: {:?}", e);
                    42 // Default fallback
                }
            };
            
            // Create a mock profile with data from our simple module
            let profile = ProfileData {
                profile_id: profile_id,
                version: 1.into(),
                digest: ObjectDigest::random(),
                display_name: format!("Meaning of Life: {}", meaning),
                bio: "This profile is connected to our on-chain simple module".to_string(),
                profile_picture: Some("https://example.com/meaning_of_life.jpg".to_string()),
                created_at: 1710346800, // March 13, 2025
                owner: MysAddress::random_for_testing_only(),
                username: Some(format!("philosopher_{}", meaning)),
                previous_transaction: Default::default(),
            };
            
            Ok(Some(profile))
        })
    }

    #[instrument(skip(self))]
    async fn get_profile_by_username(
        &self,
        username: String,
    ) -> RpcResult<Option<ProfileData>> {
        with_tracing!(async move {
            debug!("Get profile by username: {}", username);
            
            if username.starts_with("philosopher_") {
                // Create a mock profile with the meaning of life value
                let meaning = match self.get_meaning_of_life().await {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("Error getting meaning of life: {:?}", e);
                        42 // Default fallback
                    }
                };
                
                // Create a mock profile with data from our simple module
                let profile = ProfileData {
                    profile_id: ObjectID::random(),
                    version: 1.into(),
                    digest: ObjectDigest::random(),
                    display_name: format!("Meaning of Life: {}", meaning),
                    bio: "This profile is connected to our on-chain simple module".to_string(),
                    profile_picture: Some("https://example.com/meaning_of_life.jpg".to_string()),
                    created_at: 1710346800, // March 13, 2025
                    owner: MysAddress::random_for_testing_only(),
                    username: Some(username),
                    previous_transaction: Default::default(),
                };
                
                Ok(Some(profile))
            } else {
                Ok(None)
            }
        })
    }

    #[instrument(skip(self))]
    async fn get_profiles(
        &self,
        cursor: Option<String>,
        limit: Option<usize>,
    ) -> RpcResult<ProfilePage> {
        with_tracing!(async move {
            debug!("Get profiles");
            
            let limit = limit.unwrap_or(10);
            let limit = cap_page_limit(Some(limit));
            
            // Create a mock profile with the meaning of life value
            let meaning = match self.get_meaning_of_life().await {
                Ok(value) => value,
                Err(e) => {
                    debug!("Error getting meaning of life: {:?}", e);
                    42 // Default fallback
                }
            };
            
            // Create mock profiles
            let mut profiles = Vec::new();
            for i in 0..limit {
                let profile = ProfileData {
                    profile_id: ObjectID::random(),
                    version: 1.into(),
                    digest: ObjectDigest::random(),
                    display_name: format!("Profile {}: Meaning = {}", i, meaning),
                    bio: format!("Profile {} connected to our on-chain simple module", i),
                    profile_picture: Some(format!("https://example.com/profile_{}.jpg", i)),
                    created_at: 1710346800 + (i as u64 * 3600), // Incremental timestamps
                    owner: MysAddress::random_for_testing_only(),
                    username: Some(format!("philosopher_{}_{}", meaning, i)),
                    previous_transaction: Default::default(),
                };
                profiles.push(profile);
            }
            
            // Return the profiles
            Ok(ProfilePage {
                data: profiles,
                next_cursor: None,
                has_next_page: false,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mys_types::base_types::MysAddress;
    use mys_types::base_types::ObjectID;
    
    // Add tests for the ProfileReadApi implementation
    // This would require mocking StateRead and other dependencies
}