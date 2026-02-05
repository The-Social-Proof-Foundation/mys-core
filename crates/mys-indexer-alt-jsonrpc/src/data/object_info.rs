// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{BTreeSet, HashMap},
    future::Future,
    sync::Arc,
};

use async_graphql::dataloader::Loader;
use diesel::{ExpressionMethods, QueryDsl};
use mys_indexer_alt_schema::{objects::StoredObjInfo, schema::obj_info};
use mys_types::base_types::ObjectID;

use super::reader::{ReadError, Reader};

/// Key for fetching the latest object info record for an object. This record corresponds to the
/// last time the object's ownership information changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LatestObjectInfoKey(pub ObjectID);

impl Loader<LatestObjectInfoKey> for Reader {
    type Value = StoredObjInfo;
    type Error = Arc<ReadError>;

    fn load(
        &self,
        keys: &[LatestObjectInfoKey],
    ) -> impl Future<Output = Result<HashMap<LatestObjectInfoKey, StoredObjInfo>, Self::Error>> + Send {
        let self_clone = self.clone();
        let keys_vec = keys.to_vec();
        async move {
            use obj_info::dsl as i;

            if keys_vec.is_empty() {
                return Ok(HashMap::new());
            }

            let mut conn = self_clone.connect().await.map_err(Arc::new)?;

            let ids: BTreeSet<_> = keys_vec.iter().map(|k| k.0.into_bytes()).collect();
            let obj_info: Vec<StoredObjInfo> = conn
                .results(
                    i::obj_info
                        .filter(i::object_id.eq_any(ids))
                        .distinct_on(i::object_id)
                        .order((i::object_id, i::cp_sequence_number.desc())),
                )
                .await
                .map_err(Arc::new)?;

            let id_to_stored: HashMap<_, _> = obj_info
                .into_iter()
                .map(|stored| (stored.object_id.clone(), stored))
                .collect();

            Ok(keys_vec
                .iter()
                .filter_map(|key| {
                    let slice: &[u8] = key.0.as_ref();
                    Some((*key, id_to_stored.get(slice).cloned()?))
                })
                .collect())
        }
    }
}
