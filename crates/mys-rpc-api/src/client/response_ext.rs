// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mys_sdk_types::CheckpointDigest;

/// Extension trait used to facilitate retrieval of Mys specific data from responses
pub trait ResponseExt {
    fn chain_id(&self) -> Option<CheckpointDigest>;
    fn chain(&self) -> Option<&str>;
    fn epoch(&self) -> Option<u64>;
    fn checkpoint_height(&self) -> Option<u64>;
    fn timestamp_ms(&self) -> Option<u64>;
    fn lowest_available_checkpoint(&self) -> Option<u64>;
    fn lowest_available_checkpoint_objects(&self) -> Option<u64>;
}

impl ResponseExt for tonic::metadata::MetadataMap {
    fn chain_id(&self) -> Option<CheckpointDigest> {
        self.get(crate::types::X_MYS_CHAIN_ID)
            .map(tonic::metadata::MetadataValue::as_bytes)
            .and_then(|s| CheckpointDigest::from_base58(s).ok())
    }

    fn chain(&self) -> Option<&str> {
        self.get(crate::types::X_MYS_CHAIN)
            .and_then(|h| h.to_str().ok())
    }

    fn epoch(&self) -> Option<u64> {
        self.get(crate::types::X_MYS_EPOCH)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    fn checkpoint_height(&self) -> Option<u64> {
        self.get(crate::types::X_MYS_CHECKPOINT_HEIGHT)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    fn timestamp_ms(&self) -> Option<u64> {
        self.get(crate::types::X_MYS_TIMESTAMP_MS)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    fn lowest_available_checkpoint(&self) -> Option<u64> {
        self.get(crate::types::X_MYS_LOWEST_AVAILABLE_CHECKPOINT)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    fn lowest_available_checkpoint_objects(&self) -> Option<u64> {
        self.get(crate::types::X_MYS_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
    }
}

impl<T> ResponseExt for tonic::Response<T> {
    fn chain_id(&self) -> Option<CheckpointDigest> {
        self.metadata().chain_id()
    }

    fn chain(&self) -> Option<&str> {
        self.metadata().chain()
    }

    fn epoch(&self) -> Option<u64> {
        self.metadata().epoch()
    }

    fn checkpoint_height(&self) -> Option<u64> {
        self.metadata().checkpoint_height()
    }

    fn timestamp_ms(&self) -> Option<u64> {
        self.metadata().timestamp_ms()
    }

    fn lowest_available_checkpoint(&self) -> Option<u64> {
        self.metadata().lowest_available_checkpoint()
    }

    fn lowest_available_checkpoint_objects(&self) -> Option<u64> {
        self.metadata().lowest_available_checkpoint_objects()
    }
}

impl ResponseExt for tonic::Status {
    fn chain_id(&self) -> Option<CheckpointDigest> {
        self.metadata().chain_id()
    }

    fn chain(&self) -> Option<&str> {
        self.metadata().chain()
    }

    fn epoch(&self) -> Option<u64> {
        self.metadata().epoch()
    }

    fn checkpoint_height(&self) -> Option<u64> {
        self.metadata().checkpoint_height()
    }

    fn timestamp_ms(&self) -> Option<u64> {
        self.metadata().timestamp_ms()
    }

    fn lowest_available_checkpoint(&self) -> Option<u64> {
        self.metadata().lowest_available_checkpoint()
    }

    fn lowest_available_checkpoint_objects(&self) -> Option<u64> {
        self.metadata().lowest_available_checkpoint_objects()
    }
}
