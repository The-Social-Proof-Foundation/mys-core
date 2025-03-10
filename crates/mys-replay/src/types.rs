// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::ClientError as JsonRpseeError;
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use mys_json_rpc_types::MysEvent;
use mys_json_rpc_types::MysTransactionBlockEffects;
use mys_protocol_config::{Chain, ProtocolVersion};
use mys_sdk::error::Error as MysRpcError;
use mys_types::base_types::{ObjectID, ObjectRef, SequenceNumber, MysAddress, VersionNumber};
use mys_types::digests::{ObjectDigest, TransactionDigest};
use mys_types::error::{MysError, MysObjectResponseError, MysResult, UserInputError};
use mys_types::object::Object;
use mys_types::transaction::{InputObjectKind, SenderSignedData, TransactionKind};
use thiserror::Error;
use tokio::time::Duration;
use tracing::{error, warn};

use crate::config::ReplayableNetworkConfigSet;

// TODO: make these configurable
pub(crate) const RPC_TIMEOUT_ERR_SLEEP_RETRY_PERIOD: Duration = Duration::from_millis(100_000);
pub(crate) const RPC_TIMEOUT_ERR_NUM_RETRIES: u32 = 3;
pub(crate) const MAX_CONCURRENT_REQUESTS: usize = 1_000;

// Struct tag used in system epoch change events
pub(crate) const EPOCH_CHANGE_STRUCT_TAG: &str =
    "0x3::mys_system_state_inner::SystemEpochInfoEvent";

// TODO: A lot of the information in OnChainTransactionInfo is redundant from what's already in
// SenderSignedData. We should consider removing them.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OnChainTransactionInfo {
    pub tx_digest: TransactionDigest,
    pub sender_signed_data: SenderSignedData,
    pub sender: MysAddress,
    pub input_objects: Vec<InputObjectKind>,
    pub kind: TransactionKind,
    pub modified_at_versions: Vec<(ObjectID, SequenceNumber)>,
    pub shared_object_refs: Vec<ObjectRef>,
    pub gas: Vec<(ObjectID, SequenceNumber, ObjectDigest)>,
    pub gas_budget: u64,
    pub gas_price: u64,
    pub executed_epoch: u64,
    pub dependencies: Vec<TransactionDigest>,
    #[serde(skip)]
    pub receiving_objs: Vec<(ObjectID, SequenceNumber)>,
    #[serde(skip)]
    pub config_objects: Vec<(ObjectID, SequenceNumber)>,
    // TODO: There are two problems with this being a json-rpc type:
    // 1. The json-rpc type is not a perfect mirror with TransactionEffects since v2. We lost the
    // ability to replay effects v2 specific forks. We need to fix this asap. Unfortunately at the moment
    // it is really difficult to get the raw effects given a transaction digest.
    // 2. This data structure is not bcs/bincode friendly. It makes it much more expensive to
    // store the sandbox state for batch replay.
    pub effects: MysTransactionBlockEffects,
    pub protocol_version: ProtocolVersion,
    pub epoch_start_timestamp: u64,
    pub reference_gas_price: u64,
    #[serde(default = "unspecified_chain")]
    pub chain: Chain,
}

fn unspecified_chain() -> Chain {
    warn!("Unable to determine chain id. Defaulting to unknown");
    Chain::Unknown
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error, Clone)]
pub enum ReplayEngineError {
    #[error("MysError: {:#?}", err)]
    MysError { err: MysError },

    #[error("MysRpcError: {:#?}", err)]
    MysRpcError { err: String },

    #[error("MysObjectResponseError: {:#?}", err)]
    MysObjectResponseError { err: MysObjectResponseError },

    #[error("UserInputError: {:#?}", err)]
    UserInputError { err: UserInputError },

    #[error("GeneralError: {:#?}", err)]
    GeneralError { err: String },

    #[error("MysRpcRequestTimeout")]
    MysRpcRequestTimeout,

    #[error("ObjectNotExist: {:#?}", id)]
    ObjectNotExist { id: ObjectID },

    #[error("ObjectVersionNotFound: {:#?} version {}", id, version)]
    ObjectVersionNotFound {
        id: ObjectID,
        version: SequenceNumber,
    },

    #[error(
        "ObjectVersionTooHigh: {:#?}, requested version {}, latest version found {}",
        id,
        asked_version,
        latest_version
    )]
    ObjectVersionTooHigh {
        id: ObjectID,
        asked_version: SequenceNumber,
        latest_version: SequenceNumber,
    },

    #[error(
        "ObjectDeleted: {:#?} at version {:#?} digest {:#?}",
        id,
        version,
        digest
    )]
    ObjectDeleted {
        id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },

    #[error(
        "EffectsForked: Effects for digest {} forked with diff {}",
        digest,
        diff
    )]
    EffectsForked {
        digest: TransactionDigest,
        diff: String,
        on_chain: Box<MysTransactionBlockEffects>,
        local: Box<MysTransactionBlockEffects>,
    },

    #[error(
        "Transaction {:#?} not supported by replay. Reason: {:?}",
        digest,
        reason
    )]
    TransactionNotSupported {
        digest: TransactionDigest,
        reason: String,
    },

    #[error(
        "Fatal! No framework versions for protocol version {protocol_version}. Make sure version tables are populated"
    )]
    FrameworkObjectVersionTableNotPopulated { protocol_version: u64 },

    #[error("Protocol version not found for epoch {epoch}")]
    ProtocolVersionNotFound { epoch: u64 },

    #[error("Error querying system events for epoch {epoch}")]
    ErrorQueryingSystemEvents { epoch: u64 },

    #[error("Invalid epoch change transaction in events for epoch {epoch}")]
    InvalidEpochChangeTx { epoch: u64 },

    #[error("Unexpected event format {:#?}", event)]
    UnexpectedEventFormat { event: MysEvent },

    #[error("Unable to find event for epoch {epoch}")]
    EventNotFound { epoch: u64 },

    #[error("Unable to find checkpoints for epoch {epoch}")]
    UnableToDetermineCheckpoint { epoch: u64 },

    #[error("Unable to query system events; {}", rpc_err)]
    UnableToQuerySystemEvents { rpc_err: String },

    #[error("Internal error or cache corrupted! Object {id}{} should be in cache.", version.map(|q| format!(" version {:#?}", q)).unwrap_or_default() )]
    InternalCacheInvariantViolation {
        id: ObjectID,
        version: Option<SequenceNumber>,
    },

    #[error("Error getting dynamic fields loaded objects: {}", rpc_err)]
    UnableToGetDynamicFieldLoadedObjects { rpc_err: String },

    #[error("Unable to open yaml cfg file at {}: {}", path, err)]
    UnableToOpenYamlFile { path: String, err: String },

    #[error("Unable to write yaml file at {}: {}", path, err)]
    UnableToWriteYamlFile { path: String, err: String },

    #[error("Unable to convert string {} to URL {}", url, err)]
    InvalidUrl { url: String, err: String },

    #[error(
        "Unable to execute transaction with existing network configs {:#?}",
        cfgs
    )]
    UnableToExecuteWithNetworkConfigs { cfgs: ReplayableNetworkConfigSet },

    #[error("Unable to get chain id: {}", err)]
    UnableToGetChainId { err: String },
}

impl From<MysObjectResponseError> for ReplayEngineError {
    fn from(err: MysObjectResponseError) -> Self {
        match err {
            MysObjectResponseError::NotExists { object_id } => {
                ReplayEngineError::ObjectNotExist { id: object_id }
            }
            MysObjectResponseError::Deleted {
                object_id,
                digest,
                version,
            } => ReplayEngineError::ObjectDeleted {
                id: object_id,
                version,
                digest,
            },
            _ => ReplayEngineError::MysObjectResponseError { err },
        }
    }
}

impl From<ReplayEngineError> for MysError {
    fn from(err: ReplayEngineError) -> Self {
        MysError::Unknown(format!("{:#?}", err))
    }
}

impl From<MysError> for ReplayEngineError {
    fn from(err: MysError) -> Self {
        ReplayEngineError::MysError { err }
    }
}
impl From<MysRpcError> for ReplayEngineError {
    fn from(err: MysRpcError) -> Self {
        match err {
            MysRpcError::RpcError(JsonRpseeError::RequestTimeout) => {
                ReplayEngineError::MysRpcRequestTimeout
            }
            _ => ReplayEngineError::MysRpcError {
                err: format!("{:?}", err),
            },
        }
    }
}

impl From<UserInputError> for ReplayEngineError {
    fn from(err: UserInputError) -> Self {
        ReplayEngineError::UserInputError { err }
    }
}

impl From<anyhow::Error> for ReplayEngineError {
    fn from(err: anyhow::Error) -> Self {
        ReplayEngineError::GeneralError {
            err: format!("{:#?}", err),
        }
    }
}

/// TODO: Limited set but will add more
#[derive(Debug)]
pub enum ExecutionStoreEvent {
    BackingPackageGetPackageObject {
        package_id: ObjectID,
        result: MysResult<Option<Object>>,
    },
    ChildObjectResolverStoreReadChildObject {
        parent: ObjectID,
        child: ObjectID,
        result: MysResult<Option<Object>>,
    },
    ParentSyncStoreGetLatestParentEntryRef {
        object_id: ObjectID,
        result: Option<ObjectRef>,
    },
    ResourceResolverGetResource {
        address: AccountAddress,
        typ: StructTag,
        result: MysResult<Option<Vec<u8>>>,
    },
    ModuleResolverGetModule {
        module_id: ModuleId,
        result: MysResult<Option<Vec<u8>>>,
    },
    ObjectStoreGetObject {
        object_id: ObjectID,
        result: MysResult<Option<Object>>,
    },
    ObjectStoreGetObjectByKey {
        object_id: ObjectID,
        version: VersionNumber,
        result: MysResult<Option<Object>>,
    },
    GetModuleGetModuleByModuleId {
        id: ModuleId,
        result: MysResult<Option<CompiledModule>>,
    },
    ReceiveObject {
        owner: ObjectID,
        receive: ObjectID,
        receive_at_version: SequenceNumber,
        result: MysResult<Option<Object>>,
    },
}
