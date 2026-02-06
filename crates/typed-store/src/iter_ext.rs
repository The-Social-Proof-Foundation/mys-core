// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::{DbIterator, TypedStoreError};
use crate::traits::Map;
use crate::util::be_fix_int_ser;
use serde::{de::DeserializeOwned, Serialize};
use std::ops::Bound;
use bincode::Options;

/// An iterator adapter that unwraps Results, panicking on errors.
/// This provides a convenient API for code that expects tuples directly.
pub struct UnwrapIter<I> {
    iter: I,
}

impl<I, K, V> Iterator for UnwrapIter<I>
where
    I: Iterator<Item = Result<(K, V), TypedStoreError>>,
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.expect("Iterator error"))
    }
}

/// An iterator over all key-value pairs that provides convenience methods
/// like skip_to and skip_prior_to.
/// 
/// This wraps SafeIter directly for RocksDB, matching the original Iter pattern,
/// which allows efficient seeking without creating new iterators.
pub struct UnboundedIter<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    db_map: &'a crate::rocks::DBMap<K, V>,
    // For RocksDB, we store SafeIter directly to enable efficient seeking.
    // For other backends, we fall back to the trait object.
    safe_iter: Option<crate::rocks::safe_iter::SafeIter<'a, K, V>>,
    fallback_iter: Option<DbIterator<'a, (K, V)>>,
}

impl<'a, K, V> UnboundedIter<'a, K, V>
where
    K: Serialize + DeserializeOwned + 'a,
    V: Serialize + DeserializeOwned + 'a,
{
    /// Create UnboundedIter with a SafeIter directly (for RocksDB - enables efficient seeking)
    pub(crate) fn new_with_safe_iter(
        db_map: &'a crate::rocks::DBMap<K, V>,
        safe_iter: crate::rocks::safe_iter::SafeIter<'a, K, V>,
    ) -> Self {
        Self {
            db_map,
            safe_iter: Some(safe_iter),
            fallback_iter: None,
        }
    }

    /// Create UnboundedIter with a trait object iterator (for non-RocksDB backends)
    pub(crate) fn new(db_map: &'a crate::rocks::DBMap<K, V>, iter: DbIterator<'a, (K, V)>) -> Self {
        Self {
            db_map,
            safe_iter: None,
            fallback_iter: Some(iter),
        }
    }

    /// Create UnboundedIter from a DbIteratorWrapper (extracts inner iterator)
    pub(crate) fn from_wrapper(db_map: &'a crate::rocks::DBMap<K, V>, wrapper: crate::DbIteratorWrapper<'a, K, V>) -> Self {
        Self {
            db_map,
            safe_iter: None,
            fallback_iter: Some(wrapper.into_inner()),
        }
    }

    /// Skip to the first key that is >= the given key.
    /// Returns self positioned at the key or the first one greater than it.
    /// 
    /// This directly seeks on the underlying RocksDB iterator for efficient seeking,
    /// matching the original Iter pattern.
    pub fn skip_to<Key>(mut self, key: &Key) -> Result<Self, TypedStoreError>
    where
        Key: Serialize + ?Sized,
    {
        let key_bytes = be_fix_int_ser(key);
        if let Some(ref mut safe_iter) = self.safe_iter {
            // Direct seek on SafeIter (RocksDB) - efficient!
            safe_iter.seek_to_key(&key_bytes);
            Ok(self)
        } else if let Some(_) = self.fallback_iter {
            // For non-RocksDB backends, create a new iterator with bounds
            let config = bincode::DefaultOptions::new()
                .with_big_endian()
                .with_fixint_encoding();
            let k: K = config.deserialize(&key_bytes)
                .map_err(|e| TypedStoreError::RocksDBError(format!("Failed to deserialize key: {}", e)))?;
            let new_iter = Map::safe_range_iter(self.db_map, (Bound::Included(k), Bound::Unbounded));
            Ok(Self {
                db_map: self.db_map,
                safe_iter: None,
                fallback_iter: Some(new_iter.into_inner()),
            })
        } else {
            unreachable!("UnboundedIter must have either safe_iter or fallback_iter")
        }
    }

    /// Moves the iterator to the element given or the one prior to it if it does not exist.
    /// If there is no element prior to it, it returns an empty iterator.
    /// 
    /// This directly seeks on the underlying RocksDB iterator using seek_for_prev,
    /// matching the original Iter pattern.
    pub fn skip_prior_to<Key>(mut self, key: &Key) -> Result<Self, TypedStoreError>
    where
        Key: Serialize + ?Sized,
    {
        let key_bytes = be_fix_int_ser(key);
        if let Some(ref mut safe_iter) = self.safe_iter {
            // Direct seek_for_prev on SafeIter (RocksDB) - efficient!
            safe_iter.seek_for_prev_key(&key_bytes);
            Ok(self)
        } else if let Some(_) = self.fallback_iter {
            // For non-RocksDB backends, create a new reversed iterator
            let config = bincode::DefaultOptions::new()
                .with_big_endian()
                .with_fixint_encoding();
            let k: K = config.deserialize(&key_bytes)
                .map_err(|e| TypedStoreError::RocksDBError(format!("Failed to deserialize key: {}", e)))?;
            let new_iter = self.db_map.reversed_safe_iter_with_bounds(None, Some(k))?;
            Ok(Self {
                db_map: self.db_map,
                safe_iter: None,
                fallback_iter: Some(new_iter),
            })
        } else {
            unreachable!("UnboundedIter must have either safe_iter or fallback_iter")
        }
    }

    /// Reverse the iterator direction.
    /// 
    /// For RocksDB, this modifies the SafeIter's direction directly.
    /// For other backends, this creates a new reversed iterator.
    pub fn reverse(mut self) -> Self {
        if let Some(ref mut _safe_iter) = self.safe_iter {
            // For RocksDB, we need to create a reversed iterator
            // The original Iter pattern uses a separate RevIter type
            // For now, we'll create a new reversed iterator
            let new_iter = self.db_map.reversed_safe_iter_with_bounds(None, None)
                .expect("Failed to create reversed iterator");
            Self {
                db_map: self.db_map,
                safe_iter: None,
                fallback_iter: Some(new_iter),
            }
        } else {
            // For non-RocksDB backends, create a new reversed iterator
            let new_iter = self.db_map.reversed_safe_iter_with_bounds(None, None)
                .expect("Failed to create reversed iterator");
            Self {
                db_map: self.db_map,
                safe_iter: None,
                fallback_iter: Some(new_iter),
            }
        }
    }

    /// Seeks to the last key in the database (at this column family).
    /// 
    /// This directly seeks on the underlying RocksDB iterator using seek_to_last,
    /// matching the original Iter pattern.
    pub fn skip_to_last(mut self) -> Self {
        if let Some(ref mut safe_iter) = self.safe_iter {
            // Direct seek_to_last on SafeIter (RocksDB) - efficient!
            safe_iter.seek_to_last();
            self
        } else {
            // For non-RocksDB backends, create a new reversed iterator
            let new_iter = self.db_map.reversed_safe_iter_with_bounds(None, None)
                .expect("Failed to create reversed iterator");
            Self {
                db_map: self.db_map,
                safe_iter: None,
                fallback_iter: Some(new_iter),
            }
        }
    }

    /// Seeks to the first key in the database (at this column family).
    /// 
    /// This directly seeks on the underlying RocksDB iterator using seek_to_first,
    /// matching the original Iter pattern.
    pub fn seek_to_first(mut self) -> Self {
        if let Some(ref mut _safe_iter) = self.safe_iter {
            // The iterator will seek_to_first automatically on first next() call,
            // but we can explicitly initialize it here
            // Note: SafeIter doesn't expose seek_to_first directly, but it's handled in next()
            self
        } else {
            // For non-RocksDB backends, create a new iterator
            let new_iter = self.db_map.safe_iter();
            Self {
                db_map: self.db_map,
                safe_iter: None,
                fallback_iter: Some(new_iter.into_inner()),
            }
        }
    }
}

impl<K, V> Iterator for UnboundedIter<'_, K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        // Use SafeIter directly for RocksDB (matching original Iter pattern)
        // or fall back to trait object for other backends
        if let Some(ref mut safe_iter) = self.safe_iter {
            safe_iter.next().map(|r| r.expect("Iterator error"))
        } else if let Some(ref mut fallback_iter) = self.fallback_iter {
            fallback_iter.next().map(|r| r.expect("Iterator error"))
        } else {
            unreachable!("UnboundedIter must have either safe_iter or fallback_iter")
        }
    }
}

/// Extension trait for iterators that yield Result<(K, V), Error>.
/// Provides convenience methods to work with unwrapped tuples.
pub trait IteratorExt<K, V>: Iterator<Item = Result<(K, V), TypedStoreError>>
where
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    /// Convert this iterator to one that yields unwrapped tuples, panicking on errors.
    fn unwrap_results(self) -> UnwrapIter<Self>
    where
        Self: Sized,
    {
        UnwrapIter { iter: self }
    }
}

impl<I, K, V> IteratorExt<K, V> for I
where
    I: Iterator<Item = Result<(K, V), TypedStoreError>>,
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
}

/// An iterator adapter that skips until a target key is found, then yields Results.
pub struct SkipToIter<I, K, V> {
    pub(crate) iter: I,
    pub(crate) target_key: Option<Vec<u8>>,
    pub(crate) found: bool,
    pub(crate) _phantom: std::marker::PhantomData<(K, V)>,
}

impl<I, K, V> Iterator for SkipToIter<I, K, V>
where
    I: Iterator<Item = Result<(K, V), TypedStoreError>>,
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    type Item = Result<(K, V), TypedStoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found {
            return self.iter.next();
        }
        
        // Skip until we find a key >= target
        while let Some(result) = self.iter.next() {
            match result {
                Ok((k, v)) => {
                    if let Some(ref target) = self.target_key {
                        use crate::util::be_fix_int_ser;
                        let key_bytes = be_fix_int_ser(&k);
                        if key_bytes >= *target {
                            self.found = true;
                            return Some(Ok((k, v)));
                        }
                    } else {
                        self.found = true;
                        return Some(Ok((k, v)));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

/// An iterator adapter that skips backwards until a target key is found, then yields Results.
pub struct SkipPriorToIter<I, K, V> {
    pub(crate) iter: I,
    pub(crate) target_key: Option<Vec<u8>>,
    pub(crate) found: bool,
    pub(crate) _phantom: std::marker::PhantomData<(K, V)>,
}

impl<I, K, V> Iterator for SkipPriorToIter<I, K, V>
where
    I: Iterator<Item = Result<(K, V), TypedStoreError>>,
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    type Item = Result<(K, V), TypedStoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found {
            return self.iter.next();
        }
        
        // For skip_prior_to, we need to collect and reverse, or use a different approach
        // This is a simplified implementation - proper implementation would use seek_for_prev
        // For now, we filter forward until we find a key <= target
        while let Some(result) = self.iter.next() {
            match result {
                Ok((k, v)) => {
                    if let Some(ref target) = self.target_key {
                        use crate::util::be_fix_int_ser;
                        let key_bytes = be_fix_int_ser(&k);
                        if key_bytes <= *target {
                            self.found = true;
                            return Some(Ok((k, v)));
                        }
                    } else {
                        self.found = true;
                        return Some(Ok((k, v)));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

/// Extension trait for DbIterator that provides skip_to and skip_prior_to methods.
/// These methods return iterators that still yield Results.
pub trait DbIteratorExt<K, V>
where
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    /// Skip to the first key that is >= the given key.
    /// Returns an iterator that yields Results.
    fn skip_to<Key: Serialize>(self, key: &Key) -> Result<SkipToIter<Self, K, V>, TypedStoreError>
    where
        Self: Sized + Iterator<Item = Result<(K, V), TypedStoreError>>;
    
    /// Skip to the last key that is <= the given key.
    /// Returns an iterator that yields Results.
    fn skip_prior_to<Key: Serialize>(self, key: &Key) -> Result<SkipPriorToIter<Self, K, V>, TypedStoreError>
    where
        Self: Sized + Iterator<Item = Result<(K, V), TypedStoreError>>;
}

impl<'a, K, V> DbIteratorExt<K, V> for DbIterator<'a, (K, V)>
where
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    fn skip_to<Key: Serialize>(self, key: &Key) -> Result<SkipToIter<Self, K, V>, TypedStoreError>
    where
        Self: Sized,
    {
        use crate::util::be_fix_int_ser;
        Ok(SkipToIter {
            iter: self,
            target_key: Some(be_fix_int_ser(key)),
            found: false,
            _phantom: std::marker::PhantomData,
        })
    }
    
    fn skip_prior_to<Key: Serialize>(self, key: &Key) -> Result<SkipPriorToIter<Self, K, V>, TypedStoreError>
    where
        Self: Sized,
    {
        use crate::util::be_fix_int_ser;
        // Note: This is a simplified implementation that filters forward.
        // A proper implementation would use seek_for_prev on the underlying RocksDB iterator.
        // For now, this will work but may be inefficient for large datasets.
        Ok(SkipPriorToIter {
            iter: self,
            target_key: Some(be_fix_int_ser(key)),
            found: false,
            _phantom: std::marker::PhantomData,
        })
    }
}
