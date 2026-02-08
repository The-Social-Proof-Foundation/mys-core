// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use async_graphql::dataloader::Loader;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use prost_types::FieldMask;
use mys_indexer_alt_schema::objects::StoredObject;
use mys_indexer_alt_schema::schema::kv_objects;
use mys_rpc::field::FieldMaskUtil;
use mys_rpc::proto::mys::rpc::v2 as proto;
use mys_rpc_api::proto::types::{ObjectId as ProtoObjectId};
use mys_types::base_types::ObjectID;
use mys_types::object::Object;
use mys_types::storage::ObjectKey;
use mys_sdk_types::ObjectId as SdkObjectId;

use crate::bigtable_reader::BigtableReader;
use crate::error::Error;
use crate::ledger_grpc_reader::LedgerGrpcReader;
use crate::pg_reader::PgReader;

/// Key for fetching the contents a particular version of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VersionedObjectKey(pub ObjectID, pub u64);

#[async_trait::async_trait]
impl Loader<VersionedObjectKey> for PgReader {
    type Value = StoredObject;
    type Error = Error;

    async fn load(
        &self,
        keys: &[VersionedObjectKey],
    ) -> Result<HashMap<VersionedObjectKey, StoredObject>, Self::Error> {
        use kv_objects::dsl as o;

        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        let mut conn = self.connect().await?;

        let mut query = o::kv_objects.into_boxed();

        for VersionedObjectKey(id, version) in keys {
            query = query.or_filter(
                o::object_id
                    .eq(id.into_bytes())
                    .and(o::object_version.eq(*version as i64)),
            );
        }

        let objects: Vec<StoredObject> = conn.results(query).await?;

        let key_to_stored: HashMap<_, _> = objects
            .iter()
            .map(|stored| {
                let id = &stored.object_id[..];
                let version = stored.object_version as u64;
                ((id, version), stored)
            })
            .collect();

        Ok(keys
            .iter()
            .filter_map(|key| {
                let slice: &[u8] = key.0.as_ref();
                let stored = *key_to_stored.get(&(slice, key.1))?;
                Some((*key, stored.clone()))
            })
            .collect())
    }
}

#[async_trait::async_trait]
impl Loader<VersionedObjectKey> for BigtableReader {
    type Value = Object;
    type Error = Error;

    async fn load(
        &self,
        keys: &[VersionedObjectKey],
    ) -> Result<HashMap<VersionedObjectKey, Object>, Self::Error> {
        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        let object_keys: Vec<ObjectKey> = keys
            .iter()
            .map(|key| ObjectKey(key.0, key.1.into()))
            .collect();

        Ok(self
            .objects(&object_keys)
            .await?
            .into_iter()
            .map(|o| (VersionedObjectKey(o.id(), o.version().into()), o))
            .collect())
    }
}

#[async_trait::async_trait]
impl Loader<VersionedObjectKey> for LedgerGrpcReader {
    type Value = Object;
    type Error = Error;

    async fn load(
        &self,
        keys: &[VersionedObjectKey],
    ) -> Result<HashMap<VersionedObjectKey, Object>, Self::Error> {
        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        let requests = keys
            .iter()
            .map(|key| {
                let sdk_object_id: SdkObjectId = key.0.into();
                // Convert ObjectId to proto ObjectId
                let proto_object_id: ProtoObjectId = sdk_object_id.into();
                // Construct mys-rpc-api's GetObjectRequest
                let mut api_req = mys_rpc_api::proto::node::v2::GetObjectRequest::new(proto_object_id);
                api_req.version = Some(key.1);
                // Convert api_req to grpc::GetObjectRequest using prost encode/decode
                let api_bytes = prost::Message::encode_to_vec(&api_req);
                let mut req: proto::GetObjectRequest = prost::Message::decode(api_bytes.as_slice())
                    .map_err(|e| Error::from(anyhow::anyhow!("Failed to convert GetObjectRequest: {}", e)))?;
                // Ensure version is set
                req.version = Some(key.1);
                Ok::<proto::GetObjectRequest, Error>(req)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut request = proto::BatchGetObjectsRequest::default();
        request.requests = requests;
        request.read_mask = Some(FieldMask::from_paths(["bcs"]));

        let batch_response = self.batch_get_objects(request).await?;

        let mut results = HashMap::new();
        for obj_result in batch_response.objects {
            if let Some(proto::get_object_result::Result::Object(object)) = obj_result.result {
                let obj: Object = object
                    .bcs
                    .as_ref()
                    .context("Missing bcs in object")?
                    .deserialize()
                    .context("Failed to deserialize object")?;
                results.insert(VersionedObjectKey(obj.id(), obj.version().into()), obj);
            }
        }
        Ok(results)
    }
}
