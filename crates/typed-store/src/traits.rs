// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::TypedStoreError;
use crate::iter_ext::UnboundedIter;
use serde::{Serialize, de::DeserializeOwned};
use std::ops::RangeBounds;
use std::{borrow::Borrow, error::Error};

pub type DbIterator<'a, T> = Box<dyn Iterator<Item = Result<T, TypedStoreError>> + 'a>;

/// A wrapper around `DbIterator` that provides `skip_to` and `skip_prior_to` as inherent methods,
/// avoiding the need to import the `DbIteratorExt` trait.
pub struct DbIteratorWrapper<'a, K, V> {
    iter: DbIterator<'a, (K, V)>,
}

impl<'a, K, V> DbIteratorWrapper<'a, K, V>
where
    K: Serialize + serde::de::DeserializeOwned,
    V: Serialize + serde::de::DeserializeOwned,
{
    pub(crate) fn new(iter: DbIterator<'a, (K, V)>) -> Self {
        Self { iter }
    }

    /// Skip to the first key that is >= the given key.
    /// Returns an iterator that yields Results.
    pub fn skip_to<Key: Serialize>(self, key: &Key) -> Result<crate::iter_ext::SkipToIter<DbIterator<'a, (K, V)>, K, V>, TypedStoreError> {
        use crate::util::be_fix_int_ser;
        Ok(crate::iter_ext::SkipToIter {
            iter: self.iter,
            target_key: Some(be_fix_int_ser(key)),
            found: false,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Skip to the last key that is <= the given key.
    /// Returns an iterator that yields Results.
    pub fn skip_prior_to<Key: Serialize>(self, key: &Key) -> Result<crate::iter_ext::SkipPriorToIter<DbIterator<'a, (K, V)>, K, V>, TypedStoreError> {
        use crate::util::be_fix_int_ser;
        Ok(crate::iter_ext::SkipPriorToIter {
            iter: self.iter,
            target_key: Some(be_fix_int_ser(key)),
            found: false,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Extract the inner `DbIterator` for use with other APIs.
    pub(crate) fn into_inner(self) -> DbIterator<'a, (K, V)> {
        self.iter
    }
}

impl<'a, K, V> Iterator for DbIteratorWrapper<'a, K, V>
where
    K: Serialize + serde::de::DeserializeOwned,
    V: Serialize + serde::de::DeserializeOwned,
{
    type Item = Result<(K, V), TypedStoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait Map<'a, K, V>
where
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    type Error: Error;
    type Iterator: Iterator<Item = (K, V)>;

    /// Returns true if the map contains a value for the specified key.
    fn contains_key(&self, key: &K) -> Result<bool, Self::Error>;

    /// Returns true if the map contains a value for the specified key.
    fn multi_contains_keys<J>(
        &self,
        keys: impl IntoIterator<Item = J>,
    ) -> Result<Vec<bool>, Self::Error>
    where
        J: Borrow<K>,
    {
        keys.into_iter()
            .map(|key| self.contains_key(key.borrow()))
            .collect()
    }

    /// Returns the value for the given key from the map, if it exists.
    fn get(&self, key: &K) -> Result<Option<V>, Self::Error>;

    /// Inserts the given key-value pair into the map.
    fn insert(&self, key: &K, value: &V) -> Result<(), Self::Error>;

    /// Removes the entry for the given key from the map.
    fn remove(&self, key: &K) -> Result<(), Self::Error>;

    /// Uses delete range on the entire key range
    fn schedule_delete_all(&self) -> Result<(), TypedStoreError>;

    /// Returns true if the map is empty, otherwise false.
    fn is_empty(&self) -> bool;

    /// Same as `iter` but performs status check.
    fn safe_iter(&'a self) -> DbIteratorWrapper<'a, K, V>;

    // Same as `iter_with_bounds` but performs status check.
    fn safe_iter_with_bounds(
        &'a self,
        lower_bound: Option<K>,
        upper_bound: Option<K>,
    ) -> DbIteratorWrapper<'a, K, V>;

    // Same as `range_iter` but performs status check.
    fn safe_range_iter(&'a self, range: impl RangeBounds<K>) -> DbIteratorWrapper<'a, K, V>;

    /// Returns an iterator over all key-value pairs without bounds.
    /// This returns an iterator that yields tuples directly (unwrapping Results).
    /// Use with caution - errors will panic.
    fn unbounded_iter(&'a self) -> UnboundedIter<'a, K, V>;

    /// Returns a vector of values corresponding to the keys provided, non-atomically.
    fn multi_get<J>(&self, keys: impl IntoIterator<Item = J>) -> Result<Vec<Option<V>>, Self::Error>
    where
        J: Borrow<K>,
    {
        keys.into_iter().map(|key| self.get(key.borrow())).collect()
    }

    /// Inserts key-value pairs, non-atomically.
    fn multi_insert<J, U>(
        &self,
        key_val_pairs: impl IntoIterator<Item = (J, U)>,
    ) -> Result<(), Self::Error>
    where
        J: Borrow<K>,
        U: Borrow<V>,
    {
        key_val_pairs
            .into_iter()
            .try_for_each(|(key, value)| self.insert(key.borrow(), value.borrow()))
    }

    /// Removes keys, non-atomically.
    fn multi_remove<J>(&self, keys: impl IntoIterator<Item = J>) -> Result<(), Self::Error>
    where
        J: Borrow<K>,
    {
        keys.into_iter()
            .try_for_each(|key| self.remove(key.borrow()))
    }

    /// Try to catch up with primary when running as secondary
    fn try_catch_up_with_primary(&self) -> Result<(), Self::Error>;
}

pub struct TableSummary {
    pub num_keys: u64,
    pub key_bytes_total: usize,
    pub value_bytes_total: usize,
    pub key_hist: hdrhistogram::Histogram<u64>,
    pub value_hist: hdrhistogram::Histogram<u64>,
}

pub trait TypedStoreDebug {
    fn dump_table(
        &self,
        table_name: String,
        page_size: u16,
        page_number: usize,
    ) -> eyre::Result<std::collections::BTreeMap<String, String>>;
    
    fn primary_db_name(&self) -> String;
    
    fn describe_all_tables(&self) -> std::collections::BTreeMap<String, (String, String)>;
    
    fn count_table_keys(&self, table_name: String) -> eyre::Result<usize>;
    
    fn table_summary(&self, table_name: String) -> eyre::Result<TableSummary>;
}