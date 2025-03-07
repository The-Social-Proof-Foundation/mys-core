# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [mys.node.v2.proto](#mys-node-v2-proto)
    - [BalanceChange](#mys-node-v2-BalanceChange)
    - [BalanceChanges](#mys-node-v2-BalanceChanges)
    - [EffectsFinality](#mys-node-v2-EffectsFinality)
    - [ExecuteTransactionOptions](#mys-node-v2-ExecuteTransactionOptions)
    - [ExecuteTransactionRequest](#mys-node-v2-ExecuteTransactionRequest)
    - [ExecuteTransactionResponse](#mys-node-v2-ExecuteTransactionResponse)
    - [FullCheckpointObject](#mys-node-v2-FullCheckpointObject)
    - [FullCheckpointObjects](#mys-node-v2-FullCheckpointObjects)
    - [FullCheckpointTransaction](#mys-node-v2-FullCheckpointTransaction)
    - [GetCheckpointOptions](#mys-node-v2-GetCheckpointOptions)
    - [GetCheckpointRequest](#mys-node-v2-GetCheckpointRequest)
    - [GetCheckpointResponse](#mys-node-v2-GetCheckpointResponse)
    - [GetCommitteeRequest](#mys-node-v2-GetCommitteeRequest)
    - [GetCommitteeResponse](#mys-node-v2-GetCommitteeResponse)
    - [GetFullCheckpointOptions](#mys-node-v2-GetFullCheckpointOptions)
    - [GetFullCheckpointRequest](#mys-node-v2-GetFullCheckpointRequest)
    - [GetFullCheckpointResponse](#mys-node-v2-GetFullCheckpointResponse)
    - [GetNodeInfoRequest](#mys-node-v2-GetNodeInfoRequest)
    - [GetNodeInfoResponse](#mys-node-v2-GetNodeInfoResponse)
    - [GetObjectOptions](#mys-node-v2-GetObjectOptions)
    - [GetObjectRequest](#mys-node-v2-GetObjectRequest)
    - [GetObjectResponse](#mys-node-v2-GetObjectResponse)
    - [GetTransactionOptions](#mys-node-v2-GetTransactionOptions)
    - [GetTransactionRequest](#mys-node-v2-GetTransactionRequest)
    - [GetTransactionResponse](#mys-node-v2-GetTransactionResponse)
    - [UserSignatures](#mys-node-v2-UserSignatures)
    - [UserSignaturesBytes](#mys-node-v2-UserSignaturesBytes)
  
    - [NodeService](#mys-node-v2-NodeService)
  
- [mys.types.proto](#mys-types-proto)
    - [ActiveJwk](#mys-types-ActiveJwk)
    - [Address](#mys-types-Address)
    - [AddressDeniedForCoinError](#mys-types-AddressDeniedForCoinError)
    - [Argument](#mys-types-Argument)
    - [AuthenticatorStateExpire](#mys-types-AuthenticatorStateExpire)
    - [AuthenticatorStateUpdate](#mys-types-AuthenticatorStateUpdate)
    - [Bcs](#mys-types-Bcs)
    - [Bn254FieldElement](#mys-types-Bn254FieldElement)
    - [CancelledTransaction](#mys-types-CancelledTransaction)
    - [CancelledTransactions](#mys-types-CancelledTransactions)
    - [ChangeEpoch](#mys-types-ChangeEpoch)
    - [ChangedObject](#mys-types-ChangedObject)
    - [CheckpointCommitment](#mys-types-CheckpointCommitment)
    - [CheckpointContents](#mys-types-CheckpointContents)
    - [CheckpointContents.V1](#mys-types-CheckpointContents-V1)
    - [CheckpointSummary](#mys-types-CheckpointSummary)
    - [CheckpointedTransactionInfo](#mys-types-CheckpointedTransactionInfo)
    - [CircomG1](#mys-types-CircomG1)
    - [CircomG2](#mys-types-CircomG2)
    - [Command](#mys-types-Command)
    - [CommandArgumentError](#mys-types-CommandArgumentError)
    - [CongestedObjectsError](#mys-types-CongestedObjectsError)
    - [ConsensusCommitPrologue](#mys-types-ConsensusCommitPrologue)
    - [ConsensusDeterminedVersionAssignments](#mys-types-ConsensusDeterminedVersionAssignments)
    - [Digest](#mys-types-Digest)
    - [EndOfEpochData](#mys-types-EndOfEpochData)
    - [EndOfEpochTransaction](#mys-types-EndOfEpochTransaction)
    - [EndOfEpochTransactionKind](#mys-types-EndOfEpochTransactionKind)
    - [Event](#mys-types-Event)
    - [ExecutionStatus](#mys-types-ExecutionStatus)
    - [FailureStatus](#mys-types-FailureStatus)
    - [GasCostSummary](#mys-types-GasCostSummary)
    - [GasPayment](#mys-types-GasPayment)
    - [GenesisObject](#mys-types-GenesisObject)
    - [GenesisTransaction](#mys-types-GenesisTransaction)
    - [I128](#mys-types-I128)
    - [Identifier](#mys-types-Identifier)
    - [Input](#mys-types-Input)
    - [Jwk](#mys-types-Jwk)
    - [JwkId](#mys-types-JwkId)
    - [MakeMoveVector](#mys-types-MakeMoveVector)
    - [MergeCoins](#mys-types-MergeCoins)
    - [ModifiedAtVersion](#mys-types-ModifiedAtVersion)
    - [MoveCall](#mys-types-MoveCall)
    - [MoveError](#mys-types-MoveError)
    - [MoveField](#mys-types-MoveField)
    - [MoveLocation](#mys-types-MoveLocation)
    - [MoveModule](#mys-types-MoveModule)
    - [MovePackage](#mys-types-MovePackage)
    - [MoveStruct](#mys-types-MoveStruct)
    - [MoveStructValue](#mys-types-MoveStructValue)
    - [MoveValue](#mys-types-MoveValue)
    - [MoveVariant](#mys-types-MoveVariant)
    - [MoveVector](#mys-types-MoveVector)
    - [MultisigAggregatedSignature](#mys-types-MultisigAggregatedSignature)
    - [MultisigCommittee](#mys-types-MultisigCommittee)
    - [MultisigMember](#mys-types-MultisigMember)
    - [MultisigMemberPublicKey](#mys-types-MultisigMemberPublicKey)
    - [MultisigMemberSignature](#mys-types-MultisigMemberSignature)
    - [NestedResult](#mys-types-NestedResult)
    - [Object](#mys-types-Object)
    - [ObjectData](#mys-types-ObjectData)
    - [ObjectExist](#mys-types-ObjectExist)
    - [ObjectId](#mys-types-ObjectId)
    - [ObjectReference](#mys-types-ObjectReference)
    - [ObjectReferenceWithOwner](#mys-types-ObjectReferenceWithOwner)
    - [ObjectWrite](#mys-types-ObjectWrite)
    - [Owner](#mys-types-Owner)
    - [PackageIdDoesNotMatch](#mys-types-PackageIdDoesNotMatch)
    - [PackageUpgradeError](#mys-types-PackageUpgradeError)
    - [PackageWrite](#mys-types-PackageWrite)
    - [PasskeyAuthenticator](#mys-types-PasskeyAuthenticator)
    - [ProgrammableTransaction](#mys-types-ProgrammableTransaction)
    - [Publish](#mys-types-Publish)
    - [RandomnessStateUpdate](#mys-types-RandomnessStateUpdate)
    - [ReadOnlyRoot](#mys-types-ReadOnlyRoot)
    - [RoaringBitmap](#mys-types-RoaringBitmap)
    - [SharedObjectInput](#mys-types-SharedObjectInput)
    - [SimpleSignature](#mys-types-SimpleSignature)
    - [SizeError](#mys-types-SizeError)
    - [SplitCoins](#mys-types-SplitCoins)
    - [StructTag](#mys-types-StructTag)
    - [SystemPackage](#mys-types-SystemPackage)
    - [Transaction](#mys-types-Transaction)
    - [Transaction.TransactionV1](#mys-types-Transaction-TransactionV1)
    - [TransactionEffects](#mys-types-TransactionEffects)
    - [TransactionEffectsV1](#mys-types-TransactionEffectsV1)
    - [TransactionEffectsV2](#mys-types-TransactionEffectsV2)
    - [TransactionEvents](#mys-types-TransactionEvents)
    - [TransactionExpiration](#mys-types-TransactionExpiration)
    - [TransactionKind](#mys-types-TransactionKind)
    - [TransferObjects](#mys-types-TransferObjects)
    - [TypeArgumentError](#mys-types-TypeArgumentError)
    - [TypeOrigin](#mys-types-TypeOrigin)
    - [TypeTag](#mys-types-TypeTag)
    - [U128](#mys-types-U128)
    - [U256](#mys-types-U256)
    - [UnchangedSharedObject](#mys-types-UnchangedSharedObject)
    - [Upgrade](#mys-types-Upgrade)
    - [UpgradeInfo](#mys-types-UpgradeInfo)
    - [UserSignature](#mys-types-UserSignature)
    - [ValidatorAggregatedSignature](#mys-types-ValidatorAggregatedSignature)
    - [ValidatorCommittee](#mys-types-ValidatorCommittee)
    - [ValidatorCommitteeMember](#mys-types-ValidatorCommitteeMember)
    - [VersionAssignment](#mys-types-VersionAssignment)
    - [ZkLoginAuthenticator](#mys-types-ZkLoginAuthenticator)
    - [ZkLoginClaim](#mys-types-ZkLoginClaim)
    - [ZkLoginInputs](#mys-types-ZkLoginInputs)
    - [ZkLoginProof](#mys-types-ZkLoginProof)
    - [ZkLoginPublicIdentifier](#mys-types-ZkLoginPublicIdentifier)
  
    - [SignatureScheme](#mys-types-SignatureScheme)
  
- [google/protobuf/empty.proto](#google_protobuf_empty-proto)
    - [Empty](#google-protobuf-Empty)
  
- [google/protobuf/timestamp.proto](#google_protobuf_timestamp-proto)
    - [Timestamp](#google-protobuf-Timestamp)
  
- [Scalar Value Types](#scalar-value-types)



<a name="mys-node-v2-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## mys.node.v2.proto
The mys.node.v2 package contains API definitions for services that are
expected to run on Full nodes.


<a name="mys-node-v2-BalanceChange"></a>

### BalanceChange
The delta, or change, in balance for an address for a particular `Coin` type.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [mys.types.Address](#mys-types-Address) | optional | The account address that is affected by this balance change event. |
| coin_type | [mys.types.TypeTag](#mys-types-TypeTag) | optional | The `Coin` type of this balance change event. |
| amount | [mys.types.I128](#mys-types-I128) | optional | The amount or change in balance. |






<a name="mys-node-v2-BalanceChanges"></a>

### BalanceChanges
Set of `BalanceChange`s that occurred as the result of a transaction.

This set of events are calculated by analyzing all input and output `Coin`
type objects.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| balance_changes | [BalanceChange](#mys-node-v2-BalanceChange) | repeated |  |






<a name="mys-node-v2-EffectsFinality"></a>

### EffectsFinality
Indicates the finality of the executed transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| certified | [mys.types.ValidatorAggregatedSignature](#mys-types-ValidatorAggregatedSignature) |  | A quorum certificate certifying that a transaction is final but might not be included in a checkpoint yet. |
| checkpointed | [uint64](#uint64) |  | Sequence number of the checkpoint that includes the transaction. |
| quorum_executed | [google.protobuf.Empty](#google-protobuf-Empty) |  | Indicates that a quorum of validators has executed the transaction but that it might not be included in a checkpoint yet. |






<a name="mys-node-v2-ExecuteTransactionOptions"></a>

### ExecuteTransactionOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| effects | [bool](#bool) | optional | Include the `mys.types.TransactionEffects` message in the response.

Defaults to `false` if not included. |
| effects_bcs | [bool](#bool) | optional | Include the `TransactionEffects` formatted as BCS in the response.

Defaults to `false` if not included. |
| events | [bool](#bool) | optional | Include the `mys.types.TransactionEvents` message in the response.

Defaults to `false` if not included. |
| events_bcs | [bool](#bool) | optional | Include the `TransactionEvents` formatted as BCS in the response.

Defaults to `false` if not included. |
| balance_changes | [bool](#bool) | optional | Include the `BalanceChanges` in the response.

Defaults to `false` if not included. |






<a name="mys-node-v2-ExecuteTransactionRequest"></a>

### ExecuteTransactionRequest
Request message for `NodeService.ExecuteTransaction`.

Note: You must provide only one of `transaction` or `transaction_bcs`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [mys.types.Transaction](#mys-types-Transaction) | optional | Optional. The transaction to execute. |
| transaction_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The transaction to execute, encoded as BCS bytes. |
| signatures | [UserSignatures](#mys-node-v2-UserSignatures) | optional | Optional. Set of `UserSiganture`s authorizing the execution of the provided transaction. |
| signatures_bytes | [UserSignaturesBytes](#mys-node-v2-UserSignaturesBytes) | optional | Optional. Set of `UserSiganture`s authorizing the execution of the provided transaction, encoded as bytes. |
| options | [ExecuteTransactionOptions](#mys-node-v2-ExecuteTransactionOptions) | optional | Optional. Options for specifying which parts of the `ExecuteTransactionResponse` should be returned. |






<a name="mys-node-v2-ExecuteTransactionResponse"></a>

### ExecuteTransactionResponse
Response message for `NodeService.ExecuteTransaction`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| finality | [EffectsFinality](#mys-node-v2-EffectsFinality) | optional | Indicates the finality of the executed transaction. |
| effects | [mys.types.TransactionEffects](#mys-types-TransactionEffects) | optional | Optional. The `TransactionEffects` for this transaction. |
| effects_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [TransactionEffects](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.TransactionEffects.html) for this transaction encoded as BCS bytes. |
| events | [mys.types.TransactionEvents](#mys-types-TransactionEvents) | optional | Optional. The `TransactionEvents` for this transaction.

This field might be empty, even if it was explicitly requested, if the transaction didn&#39;t produce any events. `mys.types.TransactionEffects.events_digest` is populated if the transaction produced any events. |
| events_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [TransactionEvents](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.TransactionEvents.html) for this transaction encoded as BCS bytes. |
| balance_changes | [BalanceChanges](#mys-node-v2-BalanceChanges) | optional | Optional. Set of balance change events as a result of this transaction. |






<a name="mys-node-v2-FullCheckpointObject"></a>

### FullCheckpointObject
An object used by or produced from a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [mys.types.ObjectId](#mys-types-ObjectId) | optional | The `ObjectId` of this object. |
| version | [uint64](#uint64) | optional | The version of this object. |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | The digest of this object. |
| object | [mys.types.Object](#mys-types-Object) | optional | Optional. The object itself. |
| object_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [object](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.Object.html) encoded as BCS bytes. |






<a name="mys-node-v2-FullCheckpointObjects"></a>

### FullCheckpointObjects
Set of objects used by or produced from a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [FullCheckpointObject](#mys-node-v2-FullCheckpointObject) | repeated |  |






<a name="mys-node-v2-FullCheckpointTransaction"></a>

### FullCheckpointTransaction
A transaction, with all of its inputs and outputs.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | The digest of this transaction. |
| transaction | [mys.types.Transaction](#mys-types-Transaction) | optional | Optional. The transaction itself. |
| transaction_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [Transaction](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.Transaction.html) encoded as BCS bytes. |
| effects | [mys.types.TransactionEffects](#mys-types-TransactionEffects) | optional | Optional. The `TransactionEffects` for this transaction. |
| effects_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [TransactionEffects](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.TransactionEffects.html) for this transaction encoded as BCS bytes. |
| events | [mys.types.TransactionEvents](#mys-types-TransactionEvents) | optional | Optional. The `TransactionEvents` for this transaction.

This field might be empty, even if it was explicitly requested, if the transaction didn&#39;t produce any events. `mys.types.TransactionEffects.events_digest` is populated if the transaction produced any events. |
| events_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [TransactionEvents](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.TransactionEvents.html) for this transaction encoded as BCS bytes. |
| input_objects | [FullCheckpointObjects](#mys-node-v2-FullCheckpointObjects) | optional | Optional. Set of input objects used during the execution of this transaction. |
| output_objects | [FullCheckpointObjects](#mys-node-v2-FullCheckpointObjects) | optional | Optional. Set of output objects produced from the execution of this transaction. |






<a name="mys-node-v2-GetCheckpointOptions"></a>

### GetCheckpointOptions
Options for which parts of the `GetCheckpointResponse` should be returned.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| summary | [bool](#bool) | optional | Include the `mys.types.CheckpointSummary` in the response.

Defaults to `false` if not included. |
| summary_bcs | [bool](#bool) | optional | Include the `CheckpointSummary` formatted as BCS in the response.

Defaults to `false` if not included. |
| signature | [bool](#bool) | optional | Include the `mys.types.ValidatorAggregatedSignature` in the response.

Defaults to `false` if not included. |
| contents | [bool](#bool) | optional | Include the `mys.types.CheckpointContents` message in the response.

Defaults to `false` if not included. |
| contents_bcs | [bool](#bool) | optional | Include the `CheckpointContents` formatted as BCS in the response.

Defaults to `false` if not included. |






<a name="mys-node-v2-GetCheckpointRequest"></a>

### GetCheckpointRequest
Request message for `NodeService.GetCheckpoint`.

At most, provide one of `sequence_number` or `digest`. An error is
returned if you attempt to provide both. If you provide neither, the service
returns the latest executed checkpoint.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional | Optional. The sequence number of the requested checkpoint. |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | Optional. The digest of the requested checkpoint. |
| options | [GetCheckpointOptions](#mys-node-v2-GetCheckpointOptions) | optional | Optional. Options for specifying which parts of the `GetCheckpointResponse` should be returned. |






<a name="mys-node-v2-GetCheckpointResponse"></a>

### GetCheckpointResponse
Response message for `NodeService.GetCheckpoint`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional | The sequence number of this checkpoint. |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | The digest of this checkpoint&#39;s `CheckpointSummary`. |
| summary | [mys.types.CheckpointSummary](#mys-types-CheckpointSummary) | optional | Optional. The `CheckpointSummary` for this checkpoint. |
| summary_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [CheckpointSummary](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.CheckpointSummary.html) for this checkpoint encoded as BCS bytes. |
| signature | [mys.types.ValidatorAggregatedSignature](#mys-types-ValidatorAggregatedSignature) | optional | Optional. An aggregated quorum signature from the validator committee that certifies this checkpoint. |
| contents | [mys.types.CheckpointContents](#mys-types-CheckpointContents) | optional | Optional. The `CheckpointContents` for this checkpoint. |
| contents_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [CheckpointContents](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.CheckpointContents.html) for this checkpoint encoded as BCS bytes. |






<a name="mys-node-v2-GetCommitteeRequest"></a>

### GetCommitteeRequest
Request message for NodeService.GetCommittee.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Optional. Request the mys.types.ValidatorCommittee corresponding to the provided epoch. If no epoch is provided the committee for the current epoch will be returned. |






<a name="mys-node-v2-GetCommitteeResponse"></a>

### GetCommitteeResponse
Response message for `NodeService.GetCommittee`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| committee | [mys.types.ValidatorCommittee](#mys-types-ValidatorCommittee) | optional | The committee of either the requested epoch or the current epoch. |






<a name="mys-node-v2-GetFullCheckpointOptions"></a>

### GetFullCheckpointOptions
Options for which parts of the `GetFullCheckpointResponse` should be returned.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| summary | [bool](#bool) | optional | Include the `mys.types.CheckpointSummary` in the response.

Defaults to `false` if not included. |
| summary_bcs | [bool](#bool) | optional | Include the `CheckpointSummary` formatted as BCS in the response.

Defaults to `false` if not included. |
| signature | [bool](#bool) | optional | Include the `mys.types.ValidatorAggregatedSignature` in the response.

Defaults to `false` if not included. |
| contents | [bool](#bool) | optional | Include the `mys.types.CheckpointContents` message in the response.

Defaults to `false` if not included. |
| contents_bcs | [bool](#bool) | optional | Include the `CheckpointContents` formatted as BCS in the response.

Defaults to `false` if not included. |
| transaction | [bool](#bool) | optional | Include the `mys.types.Transaction` message in the response.

Defaults to `false` if not included. |
| transaction_bcs | [bool](#bool) | optional | Include the transaction formatted as BCS in the response.

Defaults to `false` if not included. |
| effects | [bool](#bool) | optional | Include the `mys.types.TransactionEffects` message in the response.

Defaults to `false` if not included. |
| effects_bcs | [bool](#bool) | optional | Include the `TransactionEffects` formatted as BCS in the response.

Defaults to `false` if not included. |
| events | [bool](#bool) | optional | Include the `mys.types.TransactionEvents` message in the response.

Defaults to `false` if not included. |
| events_bcs | [bool](#bool) | optional | Include the `TransactionEvents` formatted as BCS in the response.

Defaults to `false` if not included. |
| input_objects | [bool](#bool) | optional | Include the input objects for transactions in the response.

Defaults to `false` if not included. |
| output_objects | [bool](#bool) | optional | Include the output objects for transactions in the response.

Defaults to `false` if not included. |
| object | [bool](#bool) | optional | Include the `mys.types.Object` message in the response.

Defaults to `false` if not included. |
| object_bcs | [bool](#bool) | optional | Include the object formatted as BCS in the response.

Defaults to `false` if not included. |






<a name="mys-node-v2-GetFullCheckpointRequest"></a>

### GetFullCheckpointRequest
Request message for `NodeService.GetFullCheckpoint`.

At most, provide one of `sequence_number` or `digest`. An error is
returned if you provide both. If you provide neither, the service
returns the latest executed checkpoint.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional | Optional. The sequence number of the requested checkpoint. |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | Optional. The digest of the requested checkpoint. |
| options | [GetFullCheckpointOptions](#mys-node-v2-GetFullCheckpointOptions) | optional | Optional. Options for specifying which parts of the `GetFullCheckpointResponse` should be returned. |






<a name="mys-node-v2-GetFullCheckpointResponse"></a>

### GetFullCheckpointResponse
Response message for `NodeService.GetFullCheckpoint`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sequence_number | [uint64](#uint64) | optional | The sequence number of this checkpoint. |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | The digest of this checkpoint&#39;s `CheckpointSummary`. |
| summary | [mys.types.CheckpointSummary](#mys-types-CheckpointSummary) | optional | Optional. The `CheckpointSummary` for this checkpoint. |
| summary_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [CheckpointSummary](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.CheckpointSummary.html) for this checkpoint encoded as BCS bytes. |
| signature | [mys.types.ValidatorAggregatedSignature](#mys-types-ValidatorAggregatedSignature) | optional | Optional. An aggregated quorum signature from the validator committee that certifies this checkpoint. |
| contents | [mys.types.CheckpointContents](#mys-types-CheckpointContents) | optional | Optional. The `CheckpointContents` for this checkpoint. |
| contents_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [CheckpointContents](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.CheckpointContents.html) for this checkpoint encoded as BCS bytes. |
| transactions | [FullCheckpointTransaction](#mys-node-v2-FullCheckpointTransaction) | repeated | List of transactions included in this checkpoint. |






<a name="mys-node-v2-GetNodeInfoRequest"></a>

### GetNodeInfoRequest
Request message for `NodeService.GetNodeInfo`.






<a name="mys-node-v2-GetNodeInfoResponse"></a>

### GetNodeInfoResponse
Response message for `NodeService.GetNodeInfo`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| chain_id | [mys.types.Digest](#mys-types-Digest) | optional | The chain identifier of the chain that this node is on.

The chain identifier is the digest of the genesis checkpoint, the checkpoint with sequence number 0. |
| chain | [string](#string) | optional | Human-readable name of the chain that this node is on.

This is intended to be a human-readable name like `mainnet`, `testnet`, and so on. |
| epoch | [uint64](#uint64) | optional | Current epoch of the node based on its highest executed checkpoint. |
| checkpoint_height | [uint64](#uint64) | optional | Checkpoint height of the most recently executed checkpoint. |
| timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional | Unix timestamp of the most recently executed checkpoint. |
| lowest_available_checkpoint | [uint64](#uint64) | optional | The lowest checkpoint for which checkpoints and transaction data are available. |
| lowest_available_checkpoint_objects | [uint64](#uint64) | optional | The lowest checkpoint for which object data is available. |
| software_version | [string](#string) | optional | Software version of the `mys-node` binary. |






<a name="mys-node-v2-GetObjectOptions"></a>

### GetObjectOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object | [bool](#bool) | optional | Include the `mys.types.Object` message in the response.

Defaults to `false` if not included. |
| object_bcs | [bool](#bool) | optional | Include the object formatted as BCS in the response.

Defaults to `false` if not included. |






<a name="mys-node-v2-GetObjectRequest"></a>

### GetObjectRequest
Request message for `NodeService.GetObject`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [mys.types.ObjectId](#mys-types-ObjectId) | optional | Required. The `ObjectId` of the requested object. |
| version | [uint64](#uint64) | optional | Optional. Request that a specific version of the requested object is returned. If no version is provided, then then the latest version for the object is returned. |
| options | [GetObjectOptions](#mys-node-v2-GetObjectOptions) | optional | Optional. Options for specifying which parts of the `GetObjectResponse` should be returned. |






<a name="mys-node-v2-GetObjectResponse"></a>

### GetObjectResponse
Response message for `NodeService.GetObject`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [mys.types.ObjectId](#mys-types-ObjectId) | optional | The `ObjectId` of this object. |
| version | [uint64](#uint64) | optional | The version of this object. |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | The digest of this object. |
| object | [mys.types.Object](#mys-types-Object) | optional | Optional. The object itself. |
| object_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [Object](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.Object.html) encoded as BCS bytes. |






<a name="mys-node-v2-GetTransactionOptions"></a>

### GetTransactionOptions
Options for which parts of the `GetTransactionResponse` should be returned.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [bool](#bool) | optional | Include the `mys.types.Transaction` message in the response.

Defaults to `false` if not included. |
| transaction_bcs | [bool](#bool) | optional | Include the transaction formatted as BCS in the response.

Defaults to `false` if not included. |
| signatures | [bool](#bool) | optional | Include the set of `mys.types.UserSignature`s in the response.

Defaults to `false` if not included. |
| signatures_bytes | [bool](#bool) | optional | Include the set of `UserSignature`s encoded as bytes in the response.

Defaults to `false` if not included. |
| effects | [bool](#bool) | optional | Include the `mys.types.TransactionEffects` message in the response.

Defaults to `false` if not included. |
| effects_bcs | [bool](#bool) | optional | Include the `TransactionEffects` formatted as BCS in the response.

Defaults to `false` if not included. |
| events | [bool](#bool) | optional | Include the `mys.types.TransactionEvents` message in the response.

Defaults to `false` if not included. |
| events_bcs | [bool](#bool) | optional | Include the `TransactionEvents` formatted as BCS in the response.

Defaults to `false` if not included. |






<a name="mys-node-v2-GetTransactionRequest"></a>

### GetTransactionRequest
Request message for `NodeService.GetTransaction`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | Required. The digest of the requested transaction. |
| options | [GetTransactionOptions](#mys-node-v2-GetTransactionOptions) | optional | Optional. Options for specifying which parts of the `GetTransactionResponse` should be returned. |






<a name="mys-node-v2-GetTransactionResponse"></a>

### GetTransactionResponse
Response message for `NodeService.GetTransactio`n.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [mys.types.Digest](#mys-types-Digest) | optional | The digest of this [Transaction](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.Transaction.html). |
| transaction | [mys.types.Transaction](#mys-types-Transaction) | optional | Optional. The transaction itself. |
| transaction_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [Transaction](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.Transaction.html) encoded as BCS bytes. |
| signatures | [UserSignatures](#mys-node-v2-UserSignatures) | optional | Optional. List of user signatures that are used to authorize the execution of this transaction. |
| signatures_bytes | [UserSignaturesBytes](#mys-node-v2-UserSignaturesBytes) | optional | Optional. List of [UserSignature](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.UserSignature.html)s encoded as bytes. |
| effects | [mys.types.TransactionEffects](#mys-types-TransactionEffects) | optional | Optional. The `TransactionEffects` for this transaction. |
| effects_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [TransactionEffects](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.TransactionEffects.html) for this transaction encoded as BCS bytes. |
| events | [mys.types.TransactionEvents](#mys-types-TransactionEvents) | optional | Optional. The `TransactionEvents` for this transaction.

This field might be empty, even if it was explicitly requested, if the transaction didn&#39;t produce any events. `mys.types.TransactionEffects.events_digest` is populated if the transaction produced any events. |
| events_bcs | [mys.types.Bcs](#mys-types-Bcs) | optional | Optional. The [TransactionEvents](https://docs.rs/mys-sdk-types/latest/mys_sdk_types/struct.TransactionEvents.html) for this transaction encoded as BCS bytes. |
| checkpoint | [uint64](#uint64) | optional | The sequence number for the checkpoint that includes this transaction. |
| timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional | The Unix timestamp of the checkpoint that includes this transaction. |






<a name="mys-node-v2-UserSignatures"></a>

### UserSignatures
List of `UserSignature`s used to authorize a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signatures | [mys.types.UserSignature](#mys-types-UserSignature) | repeated |  |






<a name="mys-node-v2-UserSignaturesBytes"></a>

### UserSignaturesBytes
List of `UserSignature`s used to authorize a transaction encoded as bytes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signatures | [bytes](#bytes) | repeated |  |





 

 

 


<a name="mys-node-v2-NodeService"></a>

### NodeService
Service for reading data from a MySocial Full node.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetNodeInfo | [GetNodeInfoRequest](#mys-node-v2-GetNodeInfoRequest) | [GetNodeInfoResponse](#mys-node-v2-GetNodeInfoResponse) | Query a node for information about its current state. |
| GetCommittee | [GetCommitteeRequest](#mys-node-v2-GetCommitteeRequest) | [GetCommitteeResponse](#mys-node-v2-GetCommitteeResponse) | Request the validator committee for a particular epoch or for the current epoch. |
| GetObject | [GetObjectRequest](#mys-node-v2-GetObjectRequest) | [GetObjectResponse](#mys-node-v2-GetObjectResponse) | Request information for the specified object.

Use this API to request an object by its `ObjectId`. The version of the object returned is dependent on if you request a specific version. If you do not request a specific version (GetObjectRequest.version is `None`), then the most recent version (if the object is live) is returned. If you do request a version, that version is returned if it historically existed, is available, and has not been pruned.

Due to storage limitations, implementers of this service might prune older historical data, which can limit the data availability of this API. To determine the data availability range for historical objects, clients can look at `GetNodeInfoResponse.lowest_available_checkpoint_objects` to see the lowest checkpoint for which historical object data is available. |
| GetTransaction | [GetTransactionRequest](#mys-node-v2-GetTransactionRequest) | [GetTransactionResponse](#mys-node-v2-GetTransactionResponse) | Request information for the specified transaction.

Use this API to request information about a transaction by its digest.

Due to storage limitations, implementers of this service might prune older historical data, which can limit the data availability of this API. To determine the data availability range for historical transactions, clients can look at `GetNodeInfoResponse.lowest_available_checkpoint` to see the lowest checkpoint for which historical transaction data is available. |
| GetCheckpoint | [GetCheckpointRequest](#mys-node-v2-GetCheckpointRequest) | [GetCheckpointResponse](#mys-node-v2-GetCheckpointResponse) | Request information for the specified checkpoint.

Use this API to request information about a checkpoint either by its digest or its sequence number (height).

Due to storage limitations, implementers of this service might prune older historical data, which can limit the data availability of this API. To determine the data availability range for historical checkpoints, clients can look at `GetNodeInfoResponse.lowest_available_checkpoint` to see the lowest checkpoint for which historical checkpoint data is available. |
| GetFullCheckpoint | [GetFullCheckpointRequest](#mys-node-v2-GetFullCheckpointRequest) | [GetFullCheckpointResponse](#mys-node-v2-GetFullCheckpointResponse) | Request information for the entirety of the specified checkpoint.

Use this API to request information about a checkpoint either by its digest or its sequence number (height). In particular, you can use this API to request information about all the transactions included in a checkpoint, as well as their input and output objects.

Due to storage limitations, implementers of this service might prune older historical data, which can limit the data availability of this API. To determine the data availability range for historical checkpoints, clients can look at `GetNodeInfoResponse.lowest_available_checkpoint` to see the lowest checkpoint for which historical checkpoint/transaction data is available and `GetNodeInfoResponse.lowest_available_checkpoint_objects` for which historical object data is available. |
| ExecuteTransaction | [ExecuteTransactionRequest](#mys-node-v2-ExecuteTransactionRequest) | [ExecuteTransactionResponse](#mys-node-v2-ExecuteTransactionResponse) | Request that the provided transaction be relayed to the validator set for execution and inclusion in the blockchain. |

 



<a name="mys-types-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## mys.types.proto
Protobuf definitions of public MySocial core types.

This file contains a complete set of protobuf definitions for all of the
public mys core types. All mys types are intended to have a 1:1 mapping to a
protobuf message defined in this file and be able to roundtrip to/from their
rust and protobuf definitions assuming a sufficiently up-to-date version of
both these definitions.

For more information on the types these proto messages correspond with, see
the documentation for their rust versions defined in the
[`mys-sdk-types`](https://mystenlabs.github.io/mys-rust-sdk/mys_sdk_types/)
library.

## Use of `optional`

These message definitions use protobuf version 3 (proto3). In proto3, fields
that are primitives (that is, they are not a `message`) and are not present
on the wire are zero-initialized. To gain the ability to detect
[field presence](https://github.com/protocolbuffers/protobuf/blob/main/docs/field_presence.md),
these definitions follow the convention of having all fields marked
`optional`, and wrapping `repeated` fields in a message as needed.

Even if a field is marked as `optional`, it might not actually be optional from
the perspective of the MySocial protocol. Such fields are explicitly labled
as `Required` or `Optional` in their documentation.


<a name="mys-types-ActiveJwk"></a>

### ActiveJwk
A new JWK.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [JwkId](#mys-types-JwkId) | optional | Identifier used to uniquely identify a JWK. |
| jwk | [Jwk](#mys-types-Jwk) | optional | The JWK. |
| epoch | [uint64](#uint64) | optional | Most recent epoch in which the JWK was validated. |






<a name="mys-types-Address"></a>

### Address
Unique identifier for an account on the MySocial blockchain.

An `Address` is a 32-byte pseudonymous identifier used to uniquely identify an account and
asset-ownership on the MySocial blockchain. Often, human-readable addresses are encoded in
hexadecimal with a `0x` prefix. For example, this is a valid MySocial address:
`0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) | optional | Required. 32-byte address. |






<a name="mys-types-AddressDeniedForCoinError"></a>

### AddressDeniedForCoinError
Address is denied for this coin type.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#mys-types-Address) | optional | Required. Denied address. |
| coin_type | [string](#string) | optional | Required. Coin type. |






<a name="mys-types-Argument"></a>

### Argument
An argument to a programmable transaction command.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gas | [google.protobuf.Empty](#google-protobuf-Empty) |  | The gas coin. The gas coin can only be used by-ref, except for with `TransferObjects`, which can use it by-value. |
| input | [uint32](#uint32) |  | One of the input objects or primitive values (from `ProgrammableTransaction` inputs). |
| result | [uint32](#uint32) |  | The result of another command (from `ProgrammableTransaction` commands). |
| nested_result | [NestedResult](#mys-types-NestedResult) |  | Like a `Result` but it accesses a nested result. Currently, the only usage of this is to access a value from a Move call with multiple return values. |






<a name="mys-types-AuthenticatorStateExpire"></a>

### AuthenticatorStateExpire
Expire old JWKs.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min_epoch | [uint64](#uint64) | optional | Expire JWKs that have a lower epoch than this. |
| authenticator_object_initial_shared_version | [uint64](#uint64) | optional | The initial version of the authenticator object that it was shared at. |






<a name="mys-types-AuthenticatorStateUpdate"></a>

### AuthenticatorStateUpdate
Update the set of valid JWKs.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Epoch of the authenticator state update transaction. |
| round | [uint64](#uint64) | optional | Consensus round of the authenticator state update. |
| new_active_jwks | [ActiveJwk](#mys-types-ActiveJwk) | repeated | Newly active JWKs. |
| authenticator_object_initial_shared_version | [uint64](#uint64) | optional | The initial version of the authenticator object that it was shared at. |






<a name="mys-types-Bcs"></a>

### Bcs
Message that represents a type that is serialized and encoded using the
[BCS](https://mystenlabs.github.io/mys-rust-sdk/mys_sdk_types/index.html#bcs)
format.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bcs | [bytes](#bytes) | optional | Required. Bytes of a BCS encoded value. |






<a name="mys-types-Bn254FieldElement"></a>

### Bn254FieldElement
A point on the BN254 elliptic curve.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| element | [bytes](#bytes) | optional | Required. 32-byte big-endian field element. |






<a name="mys-types-CancelledTransaction"></a>

### CancelledTransaction
A transaction that was cancelled.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [Digest](#mys-types-Digest) | optional | Digest of the cancelled transaction. |
| version_assignments | [VersionAssignment](#mys-types-VersionAssignment) | repeated | List of object version assignments. |






<a name="mys-types-CancelledTransactions"></a>

### CancelledTransactions
Set of cancelled transactions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cancelled_transactions | [CancelledTransaction](#mys-types-CancelledTransaction) | repeated |  |






<a name="mys-types-ChangeEpoch"></a>

### ChangeEpoch
System transaction used to change the epoch.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | The next (to become) epoch ID. |
| protocol_version | [uint64](#uint64) | optional | The protocol version in effect in the new epoch. |
| storage_charge | [uint64](#uint64) | optional | The total amount of gas charged for storage during the epoch. |
| computation_charge | [uint64](#uint64) | optional | The total amount of gas charged for computation during the epoch. |
| storage_rebate | [uint64](#uint64) | optional | The amount of storage rebate refunded to the txn senders. |
| non_refundable_storage_fee | [uint64](#uint64) | optional | The non-refundable storage fee. |
| epoch_start_timestamp_ms | [uint64](#uint64) | optional | Unix timestamp when epoch started. |
| system_packages | [SystemPackage](#mys-types-SystemPackage) | repeated | System packages (specifically framework and Move stdlib) that are written before the new epoch starts. This tracks framework upgrades on chain. When executing the `ChangeEpoch` txn, the validator must write out the following modules. Modules are provided with the version they will be upgraded to, their modules in serialized form (which include their package ID), and a list of their transitive dependencies. |






<a name="mys-types-ChangedObject"></a>

### ChangedObject
Input/output state of an object that was changed during execution.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | Required. ID of the object. |
| not_exist | [google.protobuf.Empty](#google-protobuf-Empty) |  | Object did not exist prior to this transaction. |
| exist | [ObjectExist](#mys-types-ObjectExist) |  | Object existed prior to this transaction. |
| removed | [google.protobuf.Empty](#google-protobuf-Empty) |  | Object was removed from the store due to this transaction. |
| object_write | [ObjectWrite](#mys-types-ObjectWrite) |  | Object was written, including all of mutated, created, unwrapped. |
| package_write | [PackageWrite](#mys-types-PackageWrite) |  | Package was written. |
| none | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| created | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| deleted | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |






<a name="mys-types-CheckpointCommitment"></a>

### CheckpointCommitment
A commitment made by a checkpoint.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ecmh_live_object_set | [Digest](#mys-types-Digest) |  | An elliptic curve multiset hash attesting to the set of objects that comprise the live state of the MySocial blockchain. |






<a name="mys-types-CheckpointContents"></a>

### CheckpointContents
The committed to contents of a checkpoint.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| v1 | [CheckpointContents.V1](#mys-types-CheckpointContents-V1) |  |  |






<a name="mys-types-CheckpointContents-V1"></a>

### CheckpointContents.V1
Version 1 of `CheckpointContents`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactions | [CheckpointedTransactionInfo](#mys-types-CheckpointedTransactionInfo) | repeated |  |






<a name="mys-types-CheckpointSummary"></a>

### CheckpointSummary
A header for a checkpoint on the MySocial blockchain.

On the MySocial network, checkpoints define the history of the blockchain. They are quite similar to
the concept of blocks used by other blockchains like Bitcoin or Ethereum. The MySocial blockchain,
however, forms checkpoints after transaction execution has already happened to provide a
certified history of the chain, instead of being formed before execution.

Checkpoints commit to a variety of state, including but not limited to:
- The hash of the previous checkpoint.
- The set of transaction digests, their corresponding effects digests, as well as the set of
  user signatures that authorized its execution.
- The objects produced by a transaction.
- The set of live objects that make up the current state of the chain.
- On epoch transitions, the next validator committee.

`CheckpointSummary`s themselves don&#39;t directly include all of the previous information but they
are the top-level type by which all the information is committed to transitively via cryptographic
hashes included in the summary. `CheckpointSummary`s are signed and certified by a quorum of
the validator committee in a given epoch to allow verification of the chain&#39;s state.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Epoch that this checkpoint belongs to. |
| sequence_number | [uint64](#uint64) | optional | The height of this checkpoint. |
| total_network_transactions | [uint64](#uint64) | optional | Total number of transactions committed since genesis, including those in this checkpoint. |
| content_digest | [Digest](#mys-types-Digest) | optional | The hash of the `CheckpointContents` for this checkpoint. |
| previous_digest | [Digest](#mys-types-Digest) | optional | The hash of the previous `CheckpointSummary`.

This will be `None` only for the first, or genesis, checkpoint. |
| epoch_rolling_gas_cost_summary | [GasCostSummary](#mys-types-GasCostSummary) | optional | The running total gas costs of all transactions included in the current epoch so far until this checkpoint. |
| timestamp_ms | [uint64](#uint64) | optional | Timestamp of the checkpoint - number of milliseconds from the Unix epoch Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent checkpoints can have the same timestamp if they originate from the same underlining consensus commit. |
| commitments | [CheckpointCommitment](#mys-types-CheckpointCommitment) | repeated | Commitments to checkpoint-specific state. |
| end_of_epoch_data | [EndOfEpochData](#mys-types-EndOfEpochData) | optional | Extra data only present in the final checkpoint of an epoch. |
| version_specific_data | [bytes](#bytes) | optional | `CheckpointSummary` is not an evolvable structure - it must be readable by any version of the code. Therefore, to allow extensions to be added to `CheckpointSummary`, opaque data can be added to checkpoints, which can be deserialized based on the current protocol version. |






<a name="mys-types-CheckpointedTransactionInfo"></a>

### CheckpointedTransactionInfo
Transaction information committed to in a checkpoint.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [Digest](#mys-types-Digest) | optional | Digest of the transaction. |
| effects | [Digest](#mys-types-Digest) | optional | Digest of the effects. |
| signatures | [UserSignature](#mys-types-UserSignature) | repeated | Set of user signatures that authorized the transaction. |






<a name="mys-types-CircomG1"></a>

### CircomG1
A G1 point.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| e0 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e1 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e2 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |






<a name="mys-types-CircomG2"></a>

### CircomG2
A G2 point.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| e00 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e01 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e10 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e11 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e20 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |
| e21 | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |






<a name="mys-types-Command"></a>

### Command
A single command in a programmable transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| move_call | [MoveCall](#mys-types-MoveCall) |  | A call to either an entry or a public Move function. |
| transfer_objects | [TransferObjects](#mys-types-TransferObjects) |  | `(Vec&lt;forall T:key&#43;store. T&gt;, address)` It sends n-objects to the specified address. These objects must have store (public transfer) and either the previous owner must be an address or the object must be newly created. |
| split_coins | [SplitCoins](#mys-types-SplitCoins) |  | `(&amp;mut Coin&lt;T&gt;, Vec&lt;u64&gt;)` -&gt; `Vec&lt;Coin&lt;T&gt;&gt;` It splits off some amounts into new coins with those amounts. |
| merge_coins | [MergeCoins](#mys-types-MergeCoins) |  | `(&amp;mut Coin&lt;T&gt;, Vec&lt;Coin&lt;T&gt;&gt;)` It merges n-coins into the first coin. |
| publish | [Publish](#mys-types-Publish) |  | Publishes a Move package. It takes the package bytes and a list of the package&#39;s transitive dependencies to link against on chain. |
| make_move_vector | [MakeMoveVector](#mys-types-MakeMoveVector) |  | `forall T: Vec&lt;T&gt; -&gt; vector&lt;T&gt;` Given n-values of the same type, it constructs a vector. For non-objects or an empty vector, the type tag must be specified. |
| upgrade | [Upgrade](#mys-types-Upgrade) |  | Upgrades a Move package. Takes (in order): 1. A vector of serialized modules for the package. 2. A vector of object ids for the transitive dependencies of the new package. 3. The object ID of the package being upgraded. 4. An argument holding the `UpgradeTicket` that must have been produced from an earlier command in the same programmable transaction. |






<a name="mys-types-CommandArgumentError"></a>

### CommandArgumentError
An error with an argument to a command.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| argument | [uint32](#uint32) | optional | Required. Position of the problematic argument. |
| type_mismatch | [google.protobuf.Empty](#google-protobuf-Empty) |  | The type of the value does not match the expected type. |
| invalid_bcs_bytes | [google.protobuf.Empty](#google-protobuf-Empty) |  | The argument cannot be deserialized into a value of the specified type. |
| invalid_usage_of_pure_argument | [google.protobuf.Empty](#google-protobuf-Empty) |  | The argument cannot be instantiated from raw bytes. |
| invalid_argument_to_private_entry_function | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid argument to private entry function. Private entry functions cannot take arguments from other Move functions. |
| index_out_of_bounds | [uint32](#uint32) |  | Out of bounds access to input or results. |
| secondary_index_out_of_bounds | [NestedResult](#mys-types-NestedResult) |  | Out of bounds access to subresult. |
| invalid_result_arity | [uint32](#uint32) |  | Invalid usage of result. Expected a single result but found either no return value or multiple. |
| invalid_gas_coin_usage | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid usage of gas coin. The gas coin can only be used by-value with a `TransferObject` command. |
| invalid_value_usage | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid usage of Move value. - Mutably borrowed values require unique usage. - Immutably borrowed values cannot be taken or borrowed mutably. - Taken values cannot be used again. |
| invalid_object_by_value | [google.protobuf.Empty](#google-protobuf-Empty) |  | Immutable objects cannot be passed by-value. |
| invalid_object_by_mut_ref | [google.protobuf.Empty](#google-protobuf-Empty) |  | Immutable objects cannot be passed by mutable reference, `&amp;mut`. |
| shared_object_operation_not_allowed | [google.protobuf.Empty](#google-protobuf-Empty) |  | Shared object operations such as wrapping, freezing, or converting to owned are not allowed. |






<a name="mys-types-CongestedObjectsError"></a>

### CongestedObjectsError
Set of objects that were congested, leading to the transaction&#39;s cancellation.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| congested_objects | [ObjectId](#mys-types-ObjectId) | repeated | Set of congested objects. |






<a name="mys-types-ConsensusCommitPrologue"></a>

### ConsensusCommitPrologue
Consensus commit prologue system transaction.

This message can represent V1, V2, and V3 prologue types.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Epoch of the commit prologue transaction.

Present in V1, V2, and V3. |
| round | [uint64](#uint64) | optional | Consensus round of the commit.

Present in V1, V2, and V3. |
| commit_timestamp_ms | [uint64](#uint64) | optional | Unix timestamp from consensus.

Present in V1, V2, and V3. |
| consensus_commit_digest | [Digest](#mys-types-Digest) | optional | Digest of consensus output.

Present in V2 and V3. |
| sub_dag_index | [uint64](#uint64) | optional | The sub DAG index of the consensus commit. This field is populated if there are multiple consensus commits per round.

Present in V3. |
| consensus_determined_version_assignments | [ConsensusDeterminedVersionAssignments](#mys-types-ConsensusDeterminedVersionAssignments) | optional | Stores consensus handler determined shared object version assignments.

Present in V3. |






<a name="mys-types-ConsensusDeterminedVersionAssignments"></a>

### ConsensusDeterminedVersionAssignments
Version assignments performed by consensus.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cancelled_transactions | [CancelledTransactions](#mys-types-CancelledTransactions) |  | Cancelled transaction version assignment. |






<a name="mys-types-Digest"></a>

### Digest
32-byte output of hashing a MySocial structure using the Blake2b256 hash function.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [bytes](#bytes) | optional | Required. 32-byte hash. |






<a name="mys-types-EndOfEpochData"></a>

### EndOfEpochData
Data, which when included in a `CheckpointSummary`, signals the end of an `Epoch`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_epoch_committee | [ValidatorCommitteeMember](#mys-types-ValidatorCommitteeMember) | repeated | The set of validators that will be in the `ValidatorCommittee` for the next epoch. |
| next_epoch_protocol_version | [uint64](#uint64) | optional | The protocol version that is in effect during the next epoch. |
| epoch_commitments | [CheckpointCommitment](#mys-types-CheckpointCommitment) | repeated | Commitments to epoch specific state (live object set) |






<a name="mys-types-EndOfEpochTransaction"></a>

### EndOfEpochTransaction
Set of operations run at the end of the epoch to close out the current epoch
and start the next one.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactions | [EndOfEpochTransactionKind](#mys-types-EndOfEpochTransactionKind) | repeated |  |






<a name="mys-types-EndOfEpochTransactionKind"></a>

### EndOfEpochTransactionKind
Operation run at the end of an epoch.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| change_epoch | [ChangeEpoch](#mys-types-ChangeEpoch) |  | End the epoch and start the next one. |
| authenticator_state_expire | [AuthenticatorStateExpire](#mys-types-AuthenticatorStateExpire) |  | Expire JWKs used for zklogin. |
| authenticator_state_create | [google.protobuf.Empty](#google-protobuf-Empty) |  | Create and initialize the authenticator object used for zklogin. |
| randomness_state_create | [google.protobuf.Empty](#google-protobuf-Empty) |  | Create and initialize the randomness object. |
| deny_list_state_create | [google.protobuf.Empty](#google-protobuf-Empty) |  | Create and initialize the deny list object. |
| bridge_state_create | [Digest](#mys-types-Digest) |  | Create and initialize the bridge object. |
| bridge_committee_init | [uint64](#uint64) |  | Initialize the bridge committee. |






<a name="mys-types-Event"></a>

### Event
An event.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package_id | [ObjectId](#mys-types-ObjectId) | optional | Package ID of the top-level function invoked by a `MoveCall` command that triggered this event to be emitted. |
| module | [Identifier](#mys-types-Identifier) | optional | Module name of the top-level function invoked by a `MoveCall` command that triggered this event to be emitted. |
| sender | [Address](#mys-types-Address) | optional | Address of the account that sent the transaction where this event was emitted. |
| event_type | [StructTag](#mys-types-StructTag) | optional | The type of the event emitted. |
| contents | [bytes](#bytes) | optional | BCS serialized bytes of the event. |






<a name="mys-types-ExecutionStatus"></a>

### ExecutionStatus
The status of an executed transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| success | [bool](#bool) | optional | Required. Indicates if the transaction was successful or not. |
| status | [FailureStatus](#mys-types-FailureStatus) | optional | Optional. The error if `success` is false. |






<a name="mys-types-FailureStatus"></a>

### FailureStatus
An error that can occur during the execution of a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| command | [uint64](#uint64) | optional | The command, if any, during which the error occurred. |
| insufficient_gas | [google.protobuf.Empty](#google-protobuf-Empty) |  | Insufficient gas. |
| invalid_gas_object | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid `Gas` object. |
| invariant_violation | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invariant violation. |
| feature_not_yet_supported | [google.protobuf.Empty](#google-protobuf-Empty) |  | Attempted to use feature that is not supported yet. |
| object_too_big | [SizeError](#mys-types-SizeError) |  | Move object is larger than the maximum allowed size. |
| package_too_big | [SizeError](#mys-types-SizeError) |  | Package is larger than the maximum allowed size. |
| circular_object_ownership | [ObjectId](#mys-types-ObjectId) |  | Circular object ownership. |
| insufficient_coin_balance | [google.protobuf.Empty](#google-protobuf-Empty) |  | Coin errors.

Insufficient coin balance for requested operation. |
| coin_balance_overflow | [google.protobuf.Empty](#google-protobuf-Empty) |  | Coin balance overflowed an u64. |
| publish_error_non_zero_address | [google.protobuf.Empty](#google-protobuf-Empty) |  | Publish/Upgrade errors.

Publish error, non-zero address. The modules in the package must have their self-addresses set to zero. |
| mys_move_verification_error | [google.protobuf.Empty](#google-protobuf-Empty) |  | MySocial Move bytecode verification error. |
| move_primitive_runtime_error | [MoveError](#mys-types-MoveError) |  | MoveVm errors.

Error from a non-abort instruction. Possible causes: Arithmetic error, stack overflow, max value depth, or similar. |
| move_abort | [MoveError](#mys-types-MoveError) |  | Move runtime abort. |
| vm_verification_or_deserialization_error | [google.protobuf.Empty](#google-protobuf-Empty) |  | Bytecode verification error. |
| vm_invariant_violation | [google.protobuf.Empty](#google-protobuf-Empty) |  | MoveVm invariant violation. |
| function_not_found | [google.protobuf.Empty](#google-protobuf-Empty) |  | Programmable transaction errors.

Function not found. |
| arity_mismatch | [google.protobuf.Empty](#google-protobuf-Empty) |  | Parity mismatch for Move function. The number of arguments does not match the number of parameters. |
| type_arity_mismatch | [google.protobuf.Empty](#google-protobuf-Empty) |  | Type parity mismatch for Move function. Mismatch between the number of actual versus expected type arguments. |
| non_entry_function_invoked | [google.protobuf.Empty](#google-protobuf-Empty) |  | Non-entry function invoked. Move Call must start with an entry function. |
| command_argument_error | [CommandArgumentError](#mys-types-CommandArgumentError) |  | Invalid command argument. |
| type_argument_error | [TypeArgumentError](#mys-types-TypeArgumentError) |  | Type argument error. |
| unused_value_without_drop | [NestedResult](#mys-types-NestedResult) |  | Unused result without the drop ability. |
| invalid_public_function_return_type | [uint32](#uint32) |  | Invalid public Move function signature. Unsupported return type for return value. |
| invalid_transfer_object | [google.protobuf.Empty](#google-protobuf-Empty) |  | Invalid transfer object, object does not have public transfer. |
| effects_too_large | [SizeError](#mys-types-SizeError) |  | Post-execution errors.

Effects from the transaction are too large. |
| publish_upgrade_missing_dependency | [google.protobuf.Empty](#google-protobuf-Empty) |  | Publish or Upgrade is missing dependency. |
| publish_upgrade_dependency_downgrade | [google.protobuf.Empty](#google-protobuf-Empty) |  | Publish or upgrade dependency downgrade.

Indirect (transitive) dependency of published or upgraded package has been assigned an on-chain version that is less than the version required by one of the package&#39;s transitive dependencies. |
| package_upgrade_error | [PackageUpgradeError](#mys-types-PackageUpgradeError) |  | Invalid package upgrade. |
| written_objects_too_large | [SizeError](#mys-types-SizeError) |  | Indicates the transaction tried to write objects too large to storage. |
| certificate_denied | [google.protobuf.Empty](#google-protobuf-Empty) |  | Certificate is on the deny list. |
| mys_move_verification_timedout | [google.protobuf.Empty](#google-protobuf-Empty) |  | MySocial Move bytecode verification timed out. |
| shared_object_operation_not_allowed | [google.protobuf.Empty](#google-protobuf-Empty) |  | The requested shared object operation is not allowed. |
| input_object_deleted | [google.protobuf.Empty](#google-protobuf-Empty) |  | Requested shared object has been deleted. |
| execution_cancelled_due_to_shared_object_congestion | [CongestedObjectsError](#mys-types-CongestedObjectsError) |  | Certificate is cancelled due to congestion on shared objects. |
| address_denied_for_coin | [AddressDeniedForCoinError](#mys-types-AddressDeniedForCoinError) |  | Address is denied for this coin type. |
| coin_type_global_pause | [string](#string) |  | Coin type is globally paused for use. |
| execution_cancelled_due_to_randomness_unavailable | [google.protobuf.Empty](#google-protobuf-Empty) |  | Certificate is cancelled because randomness could not be generated this epoch. |






<a name="mys-types-GasCostSummary"></a>

### GasCostSummary
Summary of gas charges.

Storage is charged independently of computation.
There are three parts to the storage charges:
- `storage_cost`: the charge of storage at the time the transaction is executed.
                The cost of storage is the number of bytes of the objects being mutated
                multiplied by a variable storage cost per byte.
- `storage_rebate`: the amount a user gets back when manipulating an object.
                  The `storage_rebate` is the `storage_cost` for an object minus fees.
- `non_refundable_storage_fee`: not all the value of the object storage cost is
                              given back to user and there is a small fraction that
                              is kept by the system. This value tracks that charge.

When looking at a gas cost summary the amount charged to the user is
`computation_cost &#43; storage_cost - storage_rebate`
and that is the amount that is deducted from the gas coins.
`non_refundable_storage_fee` is collected from the objects being mutated/deleted
and it is tracked by the system in storage funds.

Objects deleted, including the older versions of objects mutated, have the storage field
on the objects added up to a pool of &#34;potential rebate&#34;. This rebate then is reduced
by the &#34;nonrefundable rate&#34; such that:
`potential_rebate(storage cost of deleted/mutated objects) =
storage_rebate &#43; non_refundable_storage_fee`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| computation_cost | [uint64](#uint64) | optional | Cost of computation/execution. |
| storage_cost | [uint64](#uint64) | optional | Storage cost, it&#39;s the sum of all storage cost for all objects created or mutated. |
| storage_rebate | [uint64](#uint64) | optional | The amount of storage cost refunded to the user for all objects deleted or mutated in the transaction. |
| non_refundable_storage_fee | [uint64](#uint64) | optional | The fee for the rebate. The portion of the storage rebate kept by the system. |






<a name="mys-types-GasPayment"></a>

### GasPayment
Payment information for executing a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [ObjectReference](#mys-types-ObjectReference) | repeated | Set of gas objects to use for payment. |
| owner | [Address](#mys-types-Address) | optional | Owner of the gas objects, either the transaction sender or a sponsor. |
| price | [uint64](#uint64) | optional | Gas unit price to use when charging for computation.

Must be greater than or equal to the network&#39;s current RGP (reference gas price). |
| budget | [uint64](#uint64) | optional | Total budget willing to spend for the execution of a transaction. |






<a name="mys-types-GenesisObject"></a>

### GenesisObject
An object part of the initial chain state.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional |  |
| version | [uint64](#uint64) | optional |  |
| owner | [Owner](#mys-types-Owner) | optional |  |
| object | [ObjectData](#mys-types-ObjectData) | optional |  |






<a name="mys-types-GenesisTransaction"></a>

### GenesisTransaction
The genesis transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [GenesisObject](#mys-types-GenesisObject) | repeated | Set of genesis objects. |






<a name="mys-types-I128"></a>

### I128
A signed 128-bit integer encoded in little-endian using 16-bytes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bytes | [bytes](#bytes) | optional | Required. 16-byte little-endian bytes. |






<a name="mys-types-Identifier"></a>

### Identifier
A Move identifier.

Identifiers are only valid if they conform to the following ABNF:

```text
identifier = (ALPHA *127(ALPHA / DIGIT / UNDERSCORE)) /
             (UNDERSCORE 1*127(ALPHA / DIGIT / UNDERSCORE))
UNDERSCORE = %x95
```


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| identifier | [string](#string) | optional |  |






<a name="mys-types-Input"></a>

### Input
An input to a user transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pure | [bytes](#bytes) |  | A move value serialized as BCS.

For normal operations this is required to be a move primitive type and not contain structs or objects. |
| immutable_or_owned | [ObjectReference](#mys-types-ObjectReference) |  | A Move object that is either immutable or address owned. |
| shared | [SharedObjectInput](#mys-types-SharedObjectInput) |  | A Move object whose owner is &#34;Shared&#34;. |
| receiving | [ObjectReference](#mys-types-ObjectReference) |  | A Move object that is attempted to be received in this transaction. |






<a name="mys-types-Jwk"></a>

### Jwk
A JSON web key.

Struct that contains info for a JWK. A list of them for different kinds can
be retrieved from the JWK endpoint (for example, &lt;https://www.googleapis.com/oauth2/v3/certs&gt;).
The JWK is used to verify the JWT token.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| kty | [string](#string) | optional | Key type parameter, https://datatracker.ietf.org/doc/html/rfc7517#section-4.1. |
| e | [string](#string) | optional | RSA public exponent, https://datatracker.ietf.org/doc/html/rfc7517#section-9.3. |
| n | [string](#string) | optional | RSA modulus, https://datatracker.ietf.org/doc/html/rfc7517#section-9.3. |
| alg | [string](#string) | optional | Algorithm parameter, https://datatracker.ietf.org/doc/html/rfc7517#section-4.4. |






<a name="mys-types-JwkId"></a>

### JwkId
Key to uniquely identify a JWK.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| iss | [string](#string) | optional | The issuer or identity of the OIDC provider. |
| kid | [string](#string) | optional | A key ID used to uniquely identify a key from an OIDC provider. |






<a name="mys-types-MakeMoveVector"></a>

### MakeMoveVector
Command to build a Move vector out of a set of individual elements.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| element_type | [TypeTag](#mys-types-TypeTag) | optional | Type of the individual elements.

This is required to be set when the type can&#39;t be inferred, for example when the set of provided arguments are all pure input values. |
| elements | [Argument](#mys-types-Argument) | repeated | The set individual elements to build the vector with. |






<a name="mys-types-MergeCoins"></a>

### MergeCoins
Command to merge multiple coins of the same type into a single coin.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coin | [Argument](#mys-types-Argument) | optional | Coin to merge coins into. |
| coins_to_merge | [Argument](#mys-types-Argument) | repeated | Set of coins to merge into `coin`.

All listed coins must be of the same type and be the same type as `coin` |






<a name="mys-types-ModifiedAtVersion"></a>

### ModifiedAtVersion
Indicates that an object was modified at a specific version.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | Required. `ObjectId` of the object. |
| version | [uint64](#uint64) | optional | Required. Version of the object prior to this transaction. |






<a name="mys-types-MoveCall"></a>

### MoveCall
Command to call a Move function.

Functions that can be called by a `MoveCall` command are those that have a function signature
that is either `entry` or `public` (which don&#39;t have a reference return type).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package | [ObjectId](#mys-types-ObjectId) | optional | The package containing the module and function. |
| module | [Identifier](#mys-types-Identifier) | optional | The specific module in the package containing the function. |
| function | [Identifier](#mys-types-Identifier) | optional | The function to be called. |
| type_arguments | [TypeTag](#mys-types-TypeTag) | repeated | The type arguments to the function. |
| arguments | [Argument](#mys-types-Argument) | repeated | The arguments to the function. |






<a name="mys-types-MoveError"></a>

### MoveError
Error that occurred in Move.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| location | [MoveLocation](#mys-types-MoveLocation) | optional | Location in Move where the error occurred. |
| abort_code | [uint64](#uint64) | optional | Abort code from Move. |






<a name="mys-types-MoveField"></a>

### MoveField



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [Identifier](#mys-types-Identifier) | optional |  |
| value | [MoveValue](#mys-types-MoveValue) | optional |  |






<a name="mys-types-MoveLocation"></a>

### MoveLocation
Location in Move bytecode where an error occurred.s


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package | [ObjectId](#mys-types-ObjectId) | optional | Required. The package ID. |
| module | [Identifier](#mys-types-Identifier) | optional | Required. The module name. |
| function | [uint32](#uint32) | optional | Required. The function index. |
| instruction | [uint32](#uint32) | optional | Required. Offset of the instruction where the error occurred. |
| function_name | [Identifier](#mys-types-Identifier) | optional | Optional. The name of the function, if available. |






<a name="mys-types-MoveModule"></a>

### MoveModule
Module defined by a package.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [Identifier](#mys-types-Identifier) | optional | Name of the module. |
| contents | [bytes](#bytes) | optional | Serialized bytecode of the module. |






<a name="mys-types-MovePackage"></a>

### MovePackage
A Move package.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [ObjectId](#mys-types-ObjectId) | optional | Address or ID of this package. |
| version | [uint64](#uint64) | optional | Version of the package. |
| modules | [MoveModule](#mys-types-MoveModule) | repeated | Set of modules defined by this package. |
| type_origin_table | [TypeOrigin](#mys-types-TypeOrigin) | repeated | Maps struct/module to a package version where it was first defined, stored as a vector for simple serialization and deserialization. |
| linkage_table | [UpgradeInfo](#mys-types-UpgradeInfo) | repeated | For each dependency, maps original package ID to the info about the (upgraded) dependency version that this package is using. |






<a name="mys-types-MoveStruct"></a>

### MoveStruct
A Move struct.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | `ObjectId` for this object. |
| object_type | [StructTag](#mys-types-StructTag) | optional | The type of this object. |
| has_public_transfer | [bool](#bool) | optional | DEPRECATED this field is no longer used to determine whether a tx can transfer this object. Instead, it is always calculated from the objects type when loaded in execution. |
| version | [uint64](#uint64) | optional | Version of the object. |
| contents | [bytes](#bytes) | optional | BCS bytes of a Move struct value. |






<a name="mys-types-MoveStructValue"></a>

### MoveStructValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| struct_type | [StructTag](#mys-types-StructTag) | optional |  |
| fields | [MoveField](#mys-types-MoveField) | repeated |  |






<a name="mys-types-MoveValue"></a>

### MoveValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bool | [bool](#bool) |  |  |
| u8 | [uint32](#uint32) |  |  |
| u16 | [uint32](#uint32) |  |  |
| u32 | [uint32](#uint32) |  |  |
| u64 | [uint64](#uint64) |  |  |
| u128 | [U128](#mys-types-U128) |  |  |
| u256 | [U256](#mys-types-U256) |  |  |
| address | [Address](#mys-types-Address) |  |  |
| vector | [MoveVector](#mys-types-MoveVector) |  |  |
| struct | [MoveStructValue](#mys-types-MoveStructValue) |  |  |
| signer | [Address](#mys-types-Address) |  |  |
| variant | [MoveVariant](#mys-types-MoveVariant) |  |  |






<a name="mys-types-MoveVariant"></a>

### MoveVariant



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enum_type | [StructTag](#mys-types-StructTag) | optional |  |
| variant_name | [Identifier](#mys-types-Identifier) | optional |  |
| tag | [uint32](#uint32) | optional |  |
| fields | [MoveField](#mys-types-MoveField) | repeated |  |






<a name="mys-types-MoveVector"></a>

### MoveVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [MoveValue](#mys-types-MoveValue) | repeated |  |






<a name="mys-types-MultisigAggregatedSignature"></a>

### MultisigAggregatedSignature
Aggregated signature from members of a multisig committee.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signatures | [MultisigMemberSignature](#mys-types-MultisigMemberSignature) | repeated | The plain signatures encoded with signature scheme.

The signatures must be in the same order as they are listed in the committee. |
| bitmap | [uint32](#uint32) | optional | Required. Bitmap indicating which committee members contributed to the signature. |
| legacy_bitmap | [RoaringBitmap](#mys-types-RoaringBitmap) | optional | Optional. If present, means this signature&#39;s on-chain format uses the old legacy multisig format. |
| committee | [MultisigCommittee](#mys-types-MultisigCommittee) | optional | Required. The committee to use to validate this signature. |






<a name="mys-types-MultisigCommittee"></a>

### MultisigCommittee
A multisig committee.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| members | [MultisigMember](#mys-types-MultisigMember) | repeated | A list of committee members and their corresponding weight. |
| threshold | [uint32](#uint32) | optional | Required. The threshold of signatures needed to validate a signature from this committee. |






<a name="mys-types-MultisigMember"></a>

### MultisigMember
A member in a multisig committee.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [MultisigMemberPublicKey](#mys-types-MultisigMemberPublicKey) | optional | Required. The public key of the committee member. |
| weight | [uint32](#uint32) | optional | Required. The weight of this member&#39;s signature. |






<a name="mys-types-MultisigMemberPublicKey"></a>

### MultisigMemberPublicKey
Set of valid public keys for multisig committee members.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ed25519 | [bytes](#bytes) |  | An ed25519 public key |
| secp256k1 | [bytes](#bytes) |  | A secp256k1 public key |
| secp256r1 | [bytes](#bytes) |  | A secp256r1 public key |
| zklogin | [ZkLoginPublicIdentifier](#mys-types-ZkLoginPublicIdentifier) |  | A zklogin public identifier |






<a name="mys-types-MultisigMemberSignature"></a>

### MultisigMemberSignature
A signature from a member of a multisig committee.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ed25519 | [bytes](#bytes) |  | An ed25519 signature. |
| secp256k1 | [bytes](#bytes) |  | A secp256k1 signature. |
| secp256r1 | [bytes](#bytes) |  | A secp256r1 signature. |
| zklogin | [ZkLoginAuthenticator](#mys-types-ZkLoginAuthenticator) |  | A zklogin signature. |






<a name="mys-types-NestedResult"></a>

### NestedResult
An argument type for a nested result.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [uint32](#uint32) | optional | The command index. |
| subresult | [uint32](#uint32) | optional | The index into the command&#39;s output. |






<a name="mys-types-Object"></a>

### Object
An object on the MySocial blockchain.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | `ObjectId` for this object. |
| version | [uint64](#uint64) | optional | Version of the object. |
| owner | [Owner](#mys-types-Owner) | optional | Owner of the object. |
| object | [ObjectData](#mys-types-ObjectData) | optional |  |
| previous_transaction | [Digest](#mys-types-Digest) | optional | The digest of the transaction that created or last mutated this object |
| storage_rebate | [uint64](#uint64) | optional | The amount of MYS to rebate if this object gets deleted. This number is re-calculated each time the object is mutated based on the present storage gas price. |






<a name="mys-types-ObjectData"></a>

### ObjectData
Object data, either a package or struct.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| struct | [MoveStruct](#mys-types-MoveStruct) |  |  |
| package | [MovePackage](#mys-types-MovePackage) |  |  |






<a name="mys-types-ObjectExist"></a>

### ObjectExist
Information about the old version of the object.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional | Required. Version of the object. |
| digest | [Digest](#mys-types-Digest) | optional | Required. Digest of the object. |
| owner | [Owner](#mys-types-Owner) | optional | Required. Owner of the object. |






<a name="mys-types-ObjectId"></a>

### ObjectId
Unique identifier for an object on the MySocial blockchain.

An `ObjectId` is a 32-byte identifier used to uniquely identify an object on the MySocial
blockchain.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [bytes](#bytes) | optional | Required. 32-byte object-id. |






<a name="mys-types-ObjectReference"></a>

### ObjectReference
Reference to an object.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | The object ID of this object. |
| version | [uint64](#uint64) | optional | The version of this object. |
| digest | [Digest](#mys-types-Digest) | optional | The digest of this object. |






<a name="mys-types-ObjectReferenceWithOwner"></a>

### ObjectReferenceWithOwner
An object reference with owner information.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| reference | [ObjectReference](#mys-types-ObjectReference) | optional | Required. `ObjectReference`. |
| owner | [Owner](#mys-types-Owner) | optional | Required. `Owner`. |






<a name="mys-types-ObjectWrite"></a>

### ObjectWrite
Object write, including all of mutated, created, unwrapped.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| digest | [Digest](#mys-types-Digest) | optional | Required. Digest of the new version of the object. |
| owner | [Owner](#mys-types-Owner) | optional | Required. Owner of the new version of the object. |






<a name="mys-types-Owner"></a>

### Owner
Enum of different types of ownership for an object.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#mys-types-Address) |  | Object is exclusively owned by a single address, and is mutable. |
| object | [ObjectId](#mys-types-ObjectId) |  | Object is exclusively owned by a single object, and is mutable. |
| shared | [uint64](#uint64) |  | Object is shared, can be used by any address, and is mutable. |
| immutable | [google.protobuf.Empty](#google-protobuf-Empty) |  | Object is immutable, and hence ownership doesn&#39;t matter. |






<a name="mys-types-PackageIdDoesNotMatch"></a>

### PackageIdDoesNotMatch
Package ID does not match `PackageId` in upgrade ticket.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| package_id | [ObjectId](#mys-types-ObjectId) | optional | Required. The package ID. |
| ticket_id | [ObjectId](#mys-types-ObjectId) | optional | Required. The ticket ID. |






<a name="mys-types-PackageUpgradeError"></a>

### PackageUpgradeError
An error with a upgrading a package.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unable_to_fetch_package | [ObjectId](#mys-types-ObjectId) |  | Unable to fetch package. |
| not_a_package | [ObjectId](#mys-types-ObjectId) |  | Object is not a package. |
| incompatible_upgrade | [google.protobuf.Empty](#google-protobuf-Empty) |  | Package upgrade is incompatible with previous version. |
| digets_does_not_match | [Digest](#mys-types-Digest) |  | Digest in upgrade ticket and computed digest differ. |
| unknown_upgrade_policy | [uint32](#uint32) |  | Upgrade policy is not valid. |
| package_id_does_not_match | [PackageIdDoesNotMatch](#mys-types-PackageIdDoesNotMatch) |  | Package ID does not match `PackageId` in upgrade ticket. |






<a name="mys-types-PackageWrite"></a>

### PackageWrite
Package write.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional | Version of the new package. |
| digest | [Digest](#mys-types-Digest) | optional | Required. Digest of the new package. |






<a name="mys-types-PasskeyAuthenticator"></a>

### PasskeyAuthenticator
A passkey authenticator.

See
[struct.PasskeyAuthenticator](https://mystenlabs.github.io/mys-rust-sdk/mys_sdk_types/struct.PasskeyAuthenticator.html#bcs)
for more information on the requirements on the shape of the
`client_data_json` field.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authenticator_data | [bytes](#bytes) | optional | Required. Opaque authenticator data for this passkey signature.

See [Authenticator Data](https://www.w3.org/TR/webauthn-2/#sctn-authenticator-data) for more information on this field. |
| client_data_json | [string](#string) | optional | Required. Structured, unparsed, JSON for this passkey signature.

See [CollectedClientData](https://www.w3.org/TR/webauthn-2/#dictdef-collectedclientdata) for more information on this field. |
| signature | [SimpleSignature](#mys-types-SimpleSignature) | optional | Required. A secp256r1 signature. |






<a name="mys-types-ProgrammableTransaction"></a>

### ProgrammableTransaction
A user transaction.

Contains a series of native commands and Move calls where the results of one command can be
used in future commands.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| inputs | [Input](#mys-types-Input) | repeated | Input objects or primitive values. |
| commands | [Command](#mys-types-Command) | repeated | The commands to be executed sequentially. A failure in any command results in the failure of the entire transaction. |






<a name="mys-types-Publish"></a>

### Publish
Command to publish a new Move package.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| modules | [bytes](#bytes) | repeated | The serialized Move modules. |
| dependencies | [ObjectId](#mys-types-ObjectId) | repeated | Set of packages that the to-be published package depends on. |






<a name="mys-types-RandomnessStateUpdate"></a>

### RandomnessStateUpdate
Randomness update.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Epoch of the randomness state update transaction. |
| randomness_round | [uint64](#uint64) | optional | Randomness round of the update. |
| random_bytes | [bytes](#bytes) | optional | Updated random bytes. |
| randomness_object_initial_shared_version | [uint64](#uint64) | optional | The initial version of the randomness object that it was shared at. |






<a name="mys-types-ReadOnlyRoot"></a>

### ReadOnlyRoot
Read-only shared object from the input.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional | Required. Version of the shared object. |
| digest | [Digest](#mys-types-Digest) | optional | Required. Digest of the shared object. |






<a name="mys-types-RoaringBitmap"></a>

### RoaringBitmap
A RoaringBitmap. See
[RoaringFormatSpec](https://github.com/RoaringBitmap/RoaringFormatSpec) for the
specification for the serialized format of `RoaringBitmap`s.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bitmap | [bytes](#bytes) | optional | Required. Serialized `RoaringBitmap`. |






<a name="mys-types-SharedObjectInput"></a>

### SharedObjectInput
A shared object input.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | `ObjectId` of the shared object. |
| initial_shared_version | [uint64](#uint64) | optional | Initial version of the object when it was shared. |
| mutable | [bool](#bool) | optional | Controls whether the caller asks for a mutable reference to the shared object. |






<a name="mys-types-SimpleSignature"></a>

### SimpleSignature
A basic signature.

Can either be an ed25519, secp256k1, or secp256r1 signature with
corresponding public key.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| scheme | [SignatureScheme](#mys-types-SignatureScheme) | optional | Required. Signature scheme of the signature and public key. |
| signature | [bytes](#bytes) | optional | Required. Signature bytes. |
| public_key | [bytes](#bytes) | optional | Required. Public key bytes. |






<a name="mys-types-SizeError"></a>

### SizeError
A size error.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint64](#uint64) | optional | Required. The offending size. |
| max_size | [uint64](#uint64) | optional | Required. The maximum allowable size. |






<a name="mys-types-SplitCoins"></a>

### SplitCoins
Command to split a single coin object into multiple coins.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coin | [Argument](#mys-types-Argument) | optional | The coin to split. |
| amounts | [Argument](#mys-types-Argument) | repeated | The amounts to split off. |






<a name="mys-types-StructTag"></a>

### StructTag
Type information for a Move struct.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [Address](#mys-types-Address) | optional | Address of the package where this type was defined. |
| module | [Identifier](#mys-types-Identifier) | optional | Name of the module where this type was defined. |
| name | [Identifier](#mys-types-Identifier) | optional | Name of the type itself. |
| type_parameters | [TypeTag](#mys-types-TypeTag) | repeated | List of type parameters, if any. |






<a name="mys-types-SystemPackage"></a>

### SystemPackage
System package.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [uint64](#uint64) | optional | Version of the package. |
| modules | [bytes](#bytes) | repeated | Move modules. |
| dependencies | [ObjectId](#mys-types-ObjectId) | repeated | Package dependencies. |






<a name="mys-types-Transaction"></a>

### Transaction
A transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| v1 | [Transaction.TransactionV1](#mys-types-Transaction-TransactionV1) |  |  |






<a name="mys-types-Transaction-TransactionV1"></a>

### Transaction.TransactionV1
Version 1 of `Transaction`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| kind | [TransactionKind](#mys-types-TransactionKind) | optional |  |
| sender | [Address](#mys-types-Address) | optional |  |
| gas_payment | [GasPayment](#mys-types-GasPayment) | optional |  |
| expiration | [TransactionExpiration](#mys-types-TransactionExpiration) | optional |  |






<a name="mys-types-TransactionEffects"></a>

### TransactionEffects
The output or effects of executing a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| v1 | [TransactionEffectsV1](#mys-types-TransactionEffectsV1) |  |  |
| v2 | [TransactionEffectsV2](#mys-types-TransactionEffectsV2) |  |  |






<a name="mys-types-TransactionEffectsV1"></a>

### TransactionEffectsV1
Version 1 of `TransactionEffects`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [ExecutionStatus](#mys-types-ExecutionStatus) | optional | The status of the execution. |
| epoch | [uint64](#uint64) | optional | The epoch when this transaction was executed. |
| gas_used | [GasCostSummary](#mys-types-GasCostSummary) | optional | The gas used by this transaction. |
| modified_at_versions | [ModifiedAtVersion](#mys-types-ModifiedAtVersion) | repeated | The version that every modified (mutated or deleted) object had before it was modified by this transaction. |
| shared_objects | [ObjectReference](#mys-types-ObjectReference) | repeated | The object references of the shared objects used in this transaction. Empty if no shared objects were used. |
| transaction_digest | [Digest](#mys-types-Digest) | optional | The transaction digest. |
| created | [ObjectReferenceWithOwner](#mys-types-ObjectReferenceWithOwner) | repeated | `ObjectReference` and owner of new objects created. |
| mutated | [ObjectReferenceWithOwner](#mys-types-ObjectReferenceWithOwner) | repeated | `ObjectReference` and owner of mutated objects, including gas object. |
| unwrapped | [ObjectReferenceWithOwner](#mys-types-ObjectReferenceWithOwner) | repeated | `ObjectReference` and owner of objects that are unwrapped in this transaction. Unwrapped objects are objects that were wrapped into other objects in the past, and just got extracted out. |
| deleted | [ObjectReference](#mys-types-ObjectReference) | repeated | Object refs of objects now deleted (the new refs). |
| unwrapped_then_deleted | [ObjectReference](#mys-types-ObjectReference) | repeated | Object refs of objects previously wrapped in other objects but now deleted. |
| wrapped | [ObjectReference](#mys-types-ObjectReference) | repeated | Object refs of objects now wrapped in other objects. |
| gas_object | [ObjectReferenceWithOwner](#mys-types-ObjectReferenceWithOwner) | optional | The updated gas object reference. Have a dedicated field for convenient access. It&#39;s also included in mutated. |
| events_digest | [Digest](#mys-types-Digest) | optional | The digest of the events emitted during execution, can be `None` if the transaction does not emit any event. |
| dependencies | [Digest](#mys-types-Digest) | repeated | The set of transaction digests this transaction depends on. |






<a name="mys-types-TransactionEffectsV2"></a>

### TransactionEffectsV2
Version 2 of `TransactionEffects`.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [ExecutionStatus](#mys-types-ExecutionStatus) | optional | The status of the execution. |
| epoch | [uint64](#uint64) | optional | The epoch when this transaction was executed. |
| gas_used | [GasCostSummary](#mys-types-GasCostSummary) | optional | The gas used by this transaction. |
| transaction_digest | [Digest](#mys-types-Digest) | optional | The transaction digest. |
| gas_object_index | [uint32](#uint32) | optional | The updated gas object reference, as an index into the `changed_objects` vector. Having a dedicated field for convenient access. System transaction that don&#39;t require gas will leave this as `None`. |
| events_digest | [Digest](#mys-types-Digest) | optional | The digest of the events emitted during execution, can be `None` if the transaction does not emit any event. |
| dependencies | [Digest](#mys-types-Digest) | repeated | The set of transaction digests this transaction depends on. |
| lamport_version | [uint64](#uint64) | optional | The version number of all the written Move objects by this transaction. |
| changed_objects | [ChangedObject](#mys-types-ChangedObject) | repeated | Objects whose state are changed in the object store. |
| unchanged_shared_objects | [UnchangedSharedObject](#mys-types-UnchangedSharedObject) | repeated | Shared objects that are not mutated in this transaction. Unlike owned objects, read-only shared objects&#39; version are not committed in the transaction, and in order for a node to catch up and execute it without consensus sequencing, the version needs to be committed in the effects. |
| auxiliary_data_digest | [Digest](#mys-types-Digest) | optional | Auxiliary data that are not protocol-critical, generated as part of the effects but are stored separately. Storing it separately allows us to avoid bloating the effects with data that are not critical. It also provides more flexibility on the format and type of the data. |






<a name="mys-types-TransactionEvents"></a>

### TransactionEvents
Events emitted during the successful execution of a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [Event](#mys-types-Event) | repeated |  |






<a name="mys-types-TransactionExpiration"></a>

### TransactionExpiration
A TTL for a transaction.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| none | [google.protobuf.Empty](#google-protobuf-Empty) |  | The transaction has no expiration. |
| epoch | [uint64](#uint64) |  | Validators won&#39;t sign and execute transaction unless the expiration epoch is greater than or equal to the current epoch. |






<a name="mys-types-TransactionKind"></a>

### TransactionKind
Transaction type.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| programmable_transaction | [ProgrammableTransaction](#mys-types-ProgrammableTransaction) |  | A user transaction comprised of a list of native commands and Move calls. |
| change_epoch | [ChangeEpoch](#mys-types-ChangeEpoch) |  | System transaction used to end an epoch.

The `ChangeEpoch` variant is now deprecated (but the `ChangeEpoch` struct is still used by `EndOfEpochTransaction`). |
| genesis | [GenesisTransaction](#mys-types-GenesisTransaction) |  | Transaction used to initialize the chain state.

Only valid if in the genesis checkpoint (0) and if this is the very first transaction ever executed on the chain. |
| consensus_commit_prologue_v1 | [ConsensusCommitPrologue](#mys-types-ConsensusCommitPrologue) |  | V1 consensus commit update. |
| authenticator_state_update | [AuthenticatorStateUpdate](#mys-types-AuthenticatorStateUpdate) |  | Update set of valid JWKs used for zklogin. |
| end_of_epoch | [EndOfEpochTransaction](#mys-types-EndOfEpochTransaction) |  | Set of operations to run at the end of the epoch to close out the current epoch and start the next one. |
| randomness_state_update | [RandomnessStateUpdate](#mys-types-RandomnessStateUpdate) |  | Randomness update. |
| consensus_commit_prologue_v2 | [ConsensusCommitPrologue](#mys-types-ConsensusCommitPrologue) |  | V2 consensus commit update. |
| consensus_commit_prologue_v3 | [ConsensusCommitPrologue](#mys-types-ConsensusCommitPrologue) |  | V3 consensus commit update. |






<a name="mys-types-TransferObjects"></a>

### TransferObjects
Command to transfer ownership of a set of objects to an address.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [Argument](#mys-types-Argument) | repeated | Set of objects to transfer. |
| address | [Argument](#mys-types-Argument) | optional | The address to transfer ownership to. |






<a name="mys-types-TypeArgumentError"></a>

### TypeArgumentError
Type argument error.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type_argument | [uint32](#uint32) | optional | Required. Index of the problematic type argument. |
| type_not_found | [google.protobuf.Empty](#google-protobuf-Empty) |  | A type was not found in the module specified. |
| constraint_not_satisfied | [google.protobuf.Empty](#google-protobuf-Empty) |  | A type provided did not match the specified constraint. |






<a name="mys-types-TypeOrigin"></a>

### TypeOrigin
Identifies a struct and the module it was defined in.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| module_name | [Identifier](#mys-types-Identifier) | optional |  |
| struct_name | [Identifier](#mys-types-Identifier) | optional |  |
| package_id | [ObjectId](#mys-types-ObjectId) | optional |  |






<a name="mys-types-TypeTag"></a>

### TypeTag
Type of a Move value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| u8 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u16 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u32 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u64 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u128 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| u256 | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| bool | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| address | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| signer | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| vector | [TypeTag](#mys-types-TypeTag) |  |  |
| struct | [StructTag](#mys-types-StructTag) |  |  |






<a name="mys-types-U128"></a>

### U128
An unsigned 128-bit integer encoded in little-endian using 16-bytes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bytes | [bytes](#bytes) | optional | Required. 16-byte little-endian bytes. |






<a name="mys-types-U256"></a>

### U256
An unsigned 256-bit integer encoded in little-endian using 32-bytes.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bytes | [bytes](#bytes) | optional | Required. 16-byte little-endian bytes. |






<a name="mys-types-UnchangedSharedObject"></a>

### UnchangedSharedObject
A shared object that wasn&#39;t changed during execution.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | Required. ObjectId of the shared object. |
| read_only_root | [ReadOnlyRoot](#mys-types-ReadOnlyRoot) |  | Read-only shared object from the input. |
| mutate_deleted | [uint64](#uint64) |  | Deleted shared objects that appear mutably/owned in the input. |
| read_deleted | [uint64](#uint64) |  | Deleted shared objects that appear as read-only in the input. |
| cancelled | [uint64](#uint64) |  | Shared objects that was congested and resulted in this transaction being cancelled. |
| per_epoch_config | [google.protobuf.Empty](#google-protobuf-Empty) |  | Read of a per-epoch config object that should remain the same during an epoch. |






<a name="mys-types-Upgrade"></a>

### Upgrade
Command to upgrade an already published package.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| modules | [bytes](#bytes) | repeated | The serialized Move modules. |
| dependencies | [ObjectId](#mys-types-ObjectId) | repeated | Set of packages that the to-be published package depends on. |
| package | [ObjectId](#mys-types-ObjectId) | optional | Package ID of the package to upgrade. |
| ticket | [Argument](#mys-types-Argument) | optional | Ticket authorizing the upgrade. |






<a name="mys-types-UpgradeInfo"></a>

### UpgradeInfo
Upgraded package info for the linkage table.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| original_id | [ObjectId](#mys-types-ObjectId) | optional | ID of the original package. |
| upgraded_id | [ObjectId](#mys-types-ObjectId) | optional | ID of the upgraded package. |
| upgraded_version | [uint64](#uint64) | optional | Version of the upgraded package. |






<a name="mys-types-UserSignature"></a>

### UserSignature
A signature from a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| simple | [SimpleSignature](#mys-types-SimpleSignature) |  |  |
| multisig | [MultisigAggregatedSignature](#mys-types-MultisigAggregatedSignature) |  |  |
| zklogin | [ZkLoginAuthenticator](#mys-types-ZkLoginAuthenticator) |  |  |
| passkey | [PasskeyAuthenticator](#mys-types-PasskeyAuthenticator) |  |  |






<a name="mys-types-ValidatorAggregatedSignature"></a>

### ValidatorAggregatedSignature
An aggregated signature from multiple validators.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Required. The epoch when this signature was produced.

This can be used to lookup the `ValidatorCommittee` from this epoch to verify this signature. |
| signature | [bytes](#bytes) | optional | Required. The 48-byte Bls12381 aggregated signature. |
| bitmap | [RoaringBitmap](#mys-types-RoaringBitmap) | optional | Required. Bitmap indicating which members of the committee contributed to this signature. |






<a name="mys-types-ValidatorCommittee"></a>

### ValidatorCommittee
The validator set for a particular epoch.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| epoch | [uint64](#uint64) | optional | Required. The epoch where this committee governs. |
| members | [ValidatorCommitteeMember](#mys-types-ValidatorCommitteeMember) | repeated | The committee members. |






<a name="mys-types-ValidatorCommitteeMember"></a>

### ValidatorCommitteeMember
A member of a validator committee.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| public_key | [bytes](#bytes) | optional | Required. The 96-byte Bls12381 public key for this validator. |
| stake | [uint64](#uint64) | optional | Required. Stake weight this validator possesses. |






<a name="mys-types-VersionAssignment"></a>

### VersionAssignment
Object version assignment from consensus.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object_id | [ObjectId](#mys-types-ObjectId) | optional | `ObjectId` of the object. |
| version | [uint64](#uint64) | optional | Assigned version. |






<a name="mys-types-ZkLoginAuthenticator"></a>

### ZkLoginAuthenticator
A zklogin authenticator.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| inputs | [ZkLoginInputs](#mys-types-ZkLoginInputs) | optional | Required. Zklogin proof and inputs required to perform proof verification. |
| max_epoch | [uint64](#uint64) | optional | Required. Maximum epoch for which the proof is valid. |
| signature | [SimpleSignature](#mys-types-SimpleSignature) | optional | Required. User signature with the public key attested to by the provided proof. |






<a name="mys-types-ZkLoginClaim"></a>

### ZkLoginClaim
A claim of the iss in a zklogin proof.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [string](#string) | optional | Required. |
| index_mod_4 | [uint32](#uint32) | optional | Required. |






<a name="mys-types-ZkLoginInputs"></a>

### ZkLoginInputs
A zklogin groth16 proof and the required inputs to perform proof verification.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| proof_points | [ZkLoginProof](#mys-types-ZkLoginProof) | optional | Required. |
| iss_base64_details | [ZkLoginClaim](#mys-types-ZkLoginClaim) | optional | Required. |
| header_base64 | [string](#string) | optional | Required. |
| address_seed | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |






<a name="mys-types-ZkLoginProof"></a>

### ZkLoginProof
A zklogin groth16 proof.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a | [CircomG1](#mys-types-CircomG1) | optional | Required. |
| b | [CircomG2](#mys-types-CircomG2) | optional | Required. |
| c | [CircomG1](#mys-types-CircomG1) | optional | Required. |






<a name="mys-types-ZkLoginPublicIdentifier"></a>

### ZkLoginPublicIdentifier
Public key equivalent for zklogin authenticators.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| iss | [string](#string) | optional | Required. |
| address_seed | [Bn254FieldElement](#mys-types-Bn254FieldElement) | optional | Required. |





 


<a name="mys-types-SignatureScheme"></a>

### SignatureScheme
Flag use to disambiguate the signature schemes supported by MySocial.

Note: the enum values defined by this proto message do not match their BCS
serialized values. See
[enum.SignatureScheme](https://mystenlabs.github.io/mys-rust-sdk/mys_sdk_types/enum.SignatureScheme.html)
for a mapping to their canonical serialized format.

| Name | Number | Description |
| ---- | ------ | ----------- |
| SIGNATURE_SCHEME_UNKNOWN | 0 |  |
| SIGNATURE_SCHEME_ED25519 | 1 |  |
| SIGNATURE_SCHEME_SECP256K1 | 2 |  |
| SIGNATURE_SCHEME_SECP256R1 | 3 |  |
| SIGNATURE_SCHEME_MULTISIG | 4 |  |
| SIGNATURE_SCHEME_BLS12381 | 5 |  |
| SIGNATURE_SCHEME_ZKLOGIN | 6 |  |
| SIGNATURE_SCHEME_PASSKEY | 7 |  |


 

 

 



<a name="google_protobuf_empty-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## google/protobuf/empty.proto



<a name="google-protobuf-Empty"></a>

### Empty
A generic empty message that you can re-use to avoid defining duplicated
empty messages in your APIs. A typical example is to use it as the request
or the response type of an API method. For instance:

```
service Foo {
  rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
}
```





 

 

 

 



<a name="google_protobuf_timestamp-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## google/protobuf/timestamp.proto



<a name="google-protobuf-Timestamp"></a>

### Timestamp
A Timestamp represents a point in time independent of any time zone
or calendar, represented as seconds and fractions of seconds at
nanosecond resolution in UTC Epoch time. It is encoded using the
Proleptic Gregorian Calendar which extends the Gregorian calendar
backwards to year one. It is encoded assuming all minutes are 60
seconds long, i.e. leap seconds are &#34;smeared&#34; so that no leap second
table is needed for interpretation. Range is from
`0001-01-01T00:00:00Z` to `9999-12-31T23:59:59.999999999Z`.
Restricting to that range ensures that conversion to
and from RFC 3339 date strings is possible.
See [https://www.ietf.org/rfc/rfc3339.txt](https://www.ietf.org/rfc/rfc3339.txt).

# Examples

Example 1: Compute Timestamp from POSIX `time()`.

```
Timestamp timestamp;
timestamp.set_seconds(time(NULL));
timestamp.set_nanos(0);
```

Example 2: Compute Timestamp from POSIX `gettimeofday()`.

```
struct timeval tv;
gettimeofday(&amp;tv, NULL);

Timestamp timestamp;
timestamp.set_seconds(tv.tv_sec);
timestamp.set_nanos(tv.tv_usec * 1000);
```

Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.

```
FILETIME ft;
GetSystemTimeAsFileTime(&amp;ft);
UINT64 ticks = (((UINT64)ft.dwHighDateTime) &lt;&lt; 32) | ft.dwLowDateTime;

// A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
// is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
Timestamp timestamp;
timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
timestamp.set_nanos((INT32) ((ticks % 10000000) * 100)); //
```

Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.

```
long millis = System.currentTimeMillis();

Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
    .setNanos((int) ((millis % 1000) * 1000000)).build();

```

Example 5: Compute Timestamp from current time in Python.

```
timestamp = Timestamp()
timestamp.GetCurrentTime()
```

# JSON Mapping

In JSON format, the `Timestamp` type is encoded as a string in the
[RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
format is `{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z`
where `{year}` is always expressed using four digits while `{month}`, `{day}`,
`{hour}`, `{min}`, and `{sec}` are zero-padded to two digits each. The fractional
seconds, which can go up to 9 digits (so up to 1 nanosecond resolution),
are optional. The &#34;Z&#34; suffix indicates the timezone (&#34;UTC&#34;); the timezone
is required, though only UTC (as indicated by &#34;Z&#34;) is presently supported.

For example, `2017-01-15T01:30:15.01Z` encodes 15.01 seconds past
01:30 UTC on January 15, 2017.

In JavaScript, you can convert a `Date` object to this format using the
standard [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
method. In Python, you can convert a standard `datetime.datetime` object
to this format using [`strftime`](https://docs.python.org/2/library/time.html#time.strftime)
with the time format spec `%Y-%m-%dT%H:%M:%S.%fZ`. Likewise, in Java, you
can use the Joda Time&#39;s [`ISODateTimeFormat.dateTime()`](
http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime--)
to obtain a formatter capable of generating timestamps in this format.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| seconds | [int64](#int64) |  | Represents seconds of UTC time since Unix epoch `1970-01-01T00:00:00Z`. Must be from `0001-01-01T00:00:00Z` to `9999-12-31T23:59:59Z` inclusive. |
| nanos | [int32](#int32) |  | Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nano values that count forward in time. Must be from 0 to 999,999,999 inclusive. |





 

 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

