// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Minimal conversions from mys-rpc proto types to mys-types, scoped to what
//! the indexer-alt framework needs. This avoids pulling in the full 3k+ line
//! conversion file from upstream Mys until a full tonic version upgrade is done.

use fastcrypto::traits::ToFromBytes;
use mys_rpc::field::FieldMaskTree;
use mys_rpc::merge::Merge;
use mys_rpc::proto::TryFromProtoError;
use mys_rpc::proto::mys::rpc::v2::{
    Checkpoint as ProtoCheckpoint, CheckpointSummary as ProtoCheckpointSummary,
    CheckpointContents as ProtoCheckpointContents, ExecutedTransaction as ProtoExecutedTransaction,
    ObjectSet as ProtoObjectSet, Object as ProtoObject, ValidatorAggregatedSignature,
    Argument as ProtoArgument,
};
use prost_types::Timestamp;

use crate::crypto::{AggregateAuthoritySignature, AuthorityQuorumSignInfo};
use crate::effects::{TransactionEffects, TransactionEvents};
use crate::full_checkpoint_content::{Checkpoint, ExecutedTransaction, ObjectSet};
use crate::gas::GasCostSummary;
use crate::message_envelope::Message;
use crate::messages_checkpoint::{CertifiedCheckpointSummary, CheckpointContents, CheckpointSummary};
use crate::object::{Data, MoveObject, Object};
use crate::move_package::MovePackage;
use crate::signature::GenericSignature;
use crate::storage::ObjectKey;
use crate::transaction::TransactionData;
use mys_rpc::proto::mys::rpc::v2::Bcs;

fn ms_to_timestamp(ms: u64) -> Timestamp {
    Timestamp {
        seconds: (ms / 1000) as _,
        nanos: ((ms % 1000) * 1_000_000) as _,
    }
}

// ---------------------------------------------------------------------------
// AuthorityQuorumSignInfo  <-->  ValidatorAggregatedSignature
// ---------------------------------------------------------------------------

/// Deserialize a roaring bitmap from raw bytes, sanitizing for duplicates.
fn deserialize_bitmap(bytes: &[u8]) -> std::io::Result<roaring::RoaringBitmap> {
    const MAX_VALIDATOR_COUNT: u64 = 2000;

    let orig_bitmap = roaring::RoaringBitmap::deserialize_from(bytes)?;

    if orig_bitmap.len() > MAX_VALIDATOR_COUNT {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "bitmap cardinality {} exceeds max {}",
                orig_bitmap.len(),
                MAX_VALIDATOR_COUNT
            ),
        ));
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut new_bitmap = roaring::RoaringBitmap::new();
    for v in orig_bitmap.iter() {
        if seen.insert(v) {
            new_bitmap.insert(v);
        }
    }

    Ok(new_bitmap)
}

impl<const T: bool> TryFrom<&ValidatorAggregatedSignature> for AuthorityQuorumSignInfo<T> {
    type Error = TryFromProtoError;

    fn try_from(value: &ValidatorAggregatedSignature) -> Result<Self, Self::Error> {
        Ok(Self {
            epoch: value.epoch(),
            signature: AggregateAuthoritySignature::from_bytes(value.signature())
                .map_err(|e| TryFromProtoError::invalid("signature", e))?,
            signers_map: deserialize_bitmap(value.bitmap())
                .map_err(|e| TryFromProtoError::invalid("bitmap", e))?,
        })
    }
}

// ---------------------------------------------------------------------------
// Checkpoint  <--  ProtoCheckpoint
// ---------------------------------------------------------------------------

impl TryFrom<&ProtoCheckpoint> for Checkpoint {
    type Error = TryFromProtoError;

    fn try_from(checkpoint: &ProtoCheckpoint) -> Result<Self, Self::Error> {
        let summary = checkpoint
            .summary()
            .bcs()
            .deserialize()
            .map_err(|e| TryFromProtoError::invalid("summary.bcs", e))?;

        let signature = AuthorityQuorumSignInfo::try_from(checkpoint.signature())?;

        let summary = CertifiedCheckpointSummary::new_from_data_and_sig(summary, signature);

        let contents: CheckpointContents = checkpoint
            .contents()
            .bcs()
            .deserialize()
            .map_err(|e| TryFromProtoError::invalid("contents.bcs", e))?;

        let user_signatures: Vec<_> = contents
            .clone()
            .into_iter_with_signatures()
            .map(|(_, user_signatures)| user_signatures)
            .collect();

        let transactions = checkpoint
            .transactions()
            .iter()
            .zip(user_signatures)
            .map(|(tx, user_signatures)| {
                let mut executed_tx = ExecutedTransaction::try_from(tx)?;
                executed_tx.signatures = user_signatures;
                Ok(executed_tx)
            })
            .collect::<Result<_, TryFromProtoError>>()?;

        let object_set = ObjectSet::try_from(checkpoint.objects())?;

        Ok(Self {
            summary,
            contents,
            transactions,
            object_set,
        })
    }
}

// ---------------------------------------------------------------------------
// ExecutedTransaction  <--  ProtoExecutedTransaction
// ---------------------------------------------------------------------------

impl TryFrom<&ProtoExecutedTransaction> for ExecutedTransaction {
    type Error = TryFromProtoError;

    fn try_from(value: &ProtoExecutedTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: value
                .transaction()
                .bcs()
                .deserialize()
                .map_err(|e| TryFromProtoError::invalid("transaction.bcs", e))?,
            // Signatures are populated by the caller from CheckpointContents
            signatures: Vec::new(),
            effects: value
                .effects()
                .bcs()
                .deserialize()
                .map_err(|e| TryFromProtoError::invalid("effects.bcs", e))?,
            events: value
                .events_opt()
                .map(|events| {
                    events
                        .bcs()
                        .deserialize()
                        .map_err(|e| TryFromProtoError::invalid("events.bcs", e))
                })
                .transpose()?,
            unchanged_loaded_runtime_objects: value
                .effects()
                .unchanged_loaded_runtime_objects()
                .iter()
                .map(|obj_ref| {
                    Ok(ObjectKey(
                        obj_ref
                            .object_id()
                            .parse()
                            .map_err(|e| TryFromProtoError::invalid("object_id", e))?,
                        obj_ref.version().into(),
                    ))
                })
                .collect::<Result<_, TryFromProtoError>>()?,
        })
    }
}

// ---------------------------------------------------------------------------
// ObjectSet  <--  ProtoObjectSet
// ---------------------------------------------------------------------------

impl TryFrom<&ProtoObjectSet> for ObjectSet {
    type Error = TryFromProtoError;

    fn try_from(value: &ProtoObjectSet) -> Result<Self, Self::Error> {
        let mut objects = Self::default();

        for o in value.objects() {
            objects.insert(
                o.bcs()
                    .deserialize()
                    .map_err(|e| TryFromProtoError::invalid("object.bcs", e))?,
            );
        }

        Ok(objects)
    }
}

// ---------------------------------------------------------------------------
// Merge implementation for Checkpoint
// ---------------------------------------------------------------------------

impl Merge<&Checkpoint> for ProtoCheckpoint {
    fn merge(&mut self, source: &Checkpoint, mask: &FieldMaskTree) {
        let sequence_number = source.summary.sequence_number;
        let timestamp_ms = source.summary.timestamp_ms;

        let summary = source.summary.data();
        let signature = source.summary.auth_sig();

        self.merge(summary, mask);
        self.merge(signature.clone(), mask);

        if mask.contains(ProtoCheckpoint::CONTENTS_FIELD.name) {
            self.merge(&source.contents, mask);
        }

        if let Some(submask) = mask
            .subtree(ProtoCheckpoint::OBJECTS_FIELD)
            .and_then(|submask| submask.subtree(mys_rpc::proto::mys::rpc::v2::ObjectSet::OBJECTS_FIELD))
        {
            let set = source
                .object_set
                .iter()
                .map(|o| mys_rpc::proto::mys::rpc::v2::Object::merge_from(o, &submask))
                .collect();
            self.objects = Some(mys_rpc::proto::mys::rpc::v2::ObjectSet::default().with_objects(set));
        }

        if let Some(submask) = mask.subtree(ProtoCheckpoint::TRANSACTIONS_FIELD.name) {
            self.transactions = source
                .transactions
                .iter()
                .map(|t| {
                    let mut transaction = ProtoExecutedTransaction::merge_from(t, &submask);
                    transaction.checkpoint = submask
                        .contains(ProtoExecutedTransaction::CHECKPOINT_FIELD)
                        .then_some(sequence_number);
                    transaction.timestamp = submask
                        .contains(ProtoExecutedTransaction::TIMESTAMP_FIELD)
                        .then(|| ms_to_timestamp(timestamp_ms));
                    transaction
                })
                .collect();
        }
    }
}

// ---------------------------------------------------------------------------
// From implementations for GasCostSummary, CheckpointCommitment, EndOfEpochData
// ---------------------------------------------------------------------------

impl From<GasCostSummary> for mys_rpc::proto::mys::rpc::v2::GasCostSummary {
    fn from(
        GasCostSummary {
            computation_cost,
            storage_cost,
            storage_rebate,
            non_refundable_storage_fee,
        }: GasCostSummary,
    ) -> Self {
        let mut message = Self::default();
        message.computation_cost = Some(computation_cost);
        message.storage_cost = Some(storage_cost);
        message.storage_rebate = Some(storage_rebate);
        message.non_refundable_storage_fee = Some(non_refundable_storage_fee);
        message
    }
}

impl From<crate::messages_checkpoint::CheckpointCommitment> for mys_rpc::proto::mys::rpc::v2::CheckpointCommitment {
    fn from(value: crate::messages_checkpoint::CheckpointCommitment) -> Self {
        use mys_rpc::proto::mys::rpc::v2::checkpoint_commitment::CheckpointCommitmentKind;

        let mut message = Self::default();

        let kind = match value {
            crate::messages_checkpoint::CheckpointCommitment::ECMHLiveObjectSetDigest(digest) => {
                message.digest = Some(digest.digest.to_string());
                CheckpointCommitmentKind::EcmhLiveObjectSet
            }
            crate::messages_checkpoint::CheckpointCommitment::CheckpointArtifactsDigest(digest) => {
                message.digest = Some(digest.to_string());
                CheckpointCommitmentKind::CheckpointArtifacts
            }
        };

        message.set_kind(kind);
        message
    }
}

impl From<crate::messages_checkpoint::EndOfEpochData> for mys_rpc::proto::mys::rpc::v2::EndOfEpochData {
    fn from(
        crate::messages_checkpoint::EndOfEpochData {
            next_epoch_committee,
            next_epoch_protocol_version,
            epoch_commitments,
        }: crate::messages_checkpoint::EndOfEpochData,
    ) -> Self {
        let mut message = Self::default();

        message.next_epoch_committee = next_epoch_committee
            .into_iter()
            .map(|(name, weight)| {
                let mut member = mys_rpc::proto::mys::rpc::v2::ValidatorCommitteeMember::default();
                member.public_key = Some(name.0.to_vec().into());
                member.weight = Some(weight);
                member
            })
            .collect();
        message.next_epoch_protocol_version = Some(next_epoch_protocol_version.as_u64());
        message.epoch_commitments = epoch_commitments.into_iter().map(Into::into).collect();

        message
    }
}

// ---------------------------------------------------------------------------
// From implementations for Owner, TypeOrigin, ObjectKey, AuthorityQuorumSignInfo
// ---------------------------------------------------------------------------

impl From<crate::object::Owner> for mys_rpc::proto::mys::rpc::v2::Owner {
    fn from(value: crate::object::Owner) -> Self {
        use crate::object::Owner as O;
        use mys_rpc::proto::mys::rpc::v2::owner::OwnerKind;

        let mut message = Self::default();

        let kind = match value {
            O::AddressOwner(address) => {
                message.address = Some(address.to_string());
                OwnerKind::Address
            }
            O::ObjectOwner(address) => {
                message.address = Some(address.to_string());
                OwnerKind::Object
            }
            O::Shared {
                initial_shared_version,
            } => {
                message.version = Some(initial_shared_version.value());
                OwnerKind::Shared
            }
            O::Immutable => OwnerKind::Immutable,
            O::ConsensusAddressOwner {
                start_version,
                owner,
            } => {
                message.version = Some(start_version.value());
                message.address = Some(owner.to_string());
                OwnerKind::ConsensusAddress
            }
        };

        message.kind = Some(kind.into());
        message
    }
}

impl From<crate::move_package::TypeOrigin> for mys_rpc::proto::mys::rpc::v2::TypeOrigin {
    fn from(value: crate::move_package::TypeOrigin) -> Self {
        let mut message = Self::default();
        message.module_name = Some(value.module_name.to_string());
        message.datatype_name = Some(value.datatype_name.to_string());
        message.package_id = Some(value.package.to_canonical_string(true));
        message
    }
}

impl From<&crate::storage::ObjectKey> for mys_rpc::proto::mys::rpc::v2::ObjectReference {
    fn from(value: &crate::storage::ObjectKey) -> Self {
        Self::default()
            .with_object_id(value.0.to_canonical_string(true))
            .with_version(value.1.value())
    }
}

impl<const T: bool> From<AuthorityQuorumSignInfo<T>> for mys_rpc::proto::mys::rpc::v2::ValidatorAggregatedSignature {
    fn from(value: AuthorityQuorumSignInfo<T>) -> Self {
        let mut bitmap = Vec::new();
        value.signers_map.serialize_into(&mut bitmap).unwrap();

        Self::default()
            .with_epoch(value.epoch)
            .with_signature(value.signature.as_ref().to_vec())
            .with_bitmap(bitmap)
    }
}

// ---------------------------------------------------------------------------
// Merge implementations for CheckpointSummary, CheckpointContents, AuthorityQuorumSignInfo, Object
// ---------------------------------------------------------------------------

impl Merge<&CheckpointSummary> for ProtoCheckpoint {
    fn merge(&mut self, source: &CheckpointSummary, mask: &FieldMaskTree) {
        if mask.contains(ProtoCheckpoint::SEQUENCE_NUMBER_FIELD) {
            self.sequence_number = Some(source.sequence_number);
        }

        if mask.contains(ProtoCheckpoint::DIGEST_FIELD) {
            self.digest = Some(source.digest().to_string());
        }

        if let Some(submask) = mask.subtree(ProtoCheckpoint::SUMMARY_FIELD) {
            self.summary = Some(ProtoCheckpointSummary::merge_from(source.clone(), &submask));
        }
    }
}

impl Merge<CheckpointSummary> for ProtoCheckpointSummary {
    fn merge(&mut self, source: CheckpointSummary, mask: &FieldMaskTree) {
        if mask.contains(ProtoCheckpointSummary::BCS_FIELD) {
            let mut bcs = Bcs::serialize(&source).unwrap();
            bcs.name = Some("CheckpointSummary".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(ProtoCheckpointSummary::DIGEST_FIELD) {
            self.digest = Some(source.digest().to_string());
        }

        let CheckpointSummary {
            epoch,
            sequence_number,
            network_total_transactions,
            content_digest,
            previous_digest,
            epoch_rolling_gas_cost_summary,
            timestamp_ms,
            checkpoint_commitments,
            end_of_epoch_data,
            version_specific_data,
        } = source;

        if mask.contains(ProtoCheckpointSummary::EPOCH_FIELD) {
            self.epoch = Some(epoch);
        }

        if mask.contains(ProtoCheckpointSummary::SEQUENCE_NUMBER_FIELD) {
            self.sequence_number = Some(sequence_number);
        }

        if mask.contains(ProtoCheckpointSummary::TOTAL_NETWORK_TRANSACTIONS_FIELD) {
            self.total_network_transactions = Some(network_total_transactions);
        }

        if mask.contains(ProtoCheckpointSummary::CONTENT_DIGEST_FIELD) {
            self.content_digest = Some(content_digest.to_string());
        }

        if mask.contains(ProtoCheckpointSummary::PREVIOUS_DIGEST_FIELD) {
            self.previous_digest = previous_digest.map(|d| d.to_string());
        }

        if mask.contains(ProtoCheckpointSummary::EPOCH_ROLLING_GAS_COST_SUMMARY_FIELD) {
            self.epoch_rolling_gas_cost_summary = Some(epoch_rolling_gas_cost_summary.into());
        }

        if mask.contains(ProtoCheckpointSummary::TIMESTAMP_FIELD) {
            self.timestamp = Some(ms_to_timestamp(timestamp_ms));
        }

        if mask.contains(ProtoCheckpointSummary::COMMITMENTS_FIELD) {
            self.commitments = checkpoint_commitments.into_iter().map(Into::into).collect();
        }

        if mask.contains(ProtoCheckpointSummary::END_OF_EPOCH_DATA_FIELD) {
            self.end_of_epoch_data = end_of_epoch_data.map(Into::into);
        }

        if mask.contains(ProtoCheckpointSummary::VERSION_SPECIFIC_DATA_FIELD) {
            self.version_specific_data = Some(version_specific_data.into());
        }
    }
}

impl Merge<&CheckpointContents> for ProtoCheckpoint {
    fn merge(&mut self, source: &CheckpointContents, mask: &FieldMaskTree) {
        if let Some(submask) = mask.subtree(ProtoCheckpoint::CONTENTS_FIELD.name) {
            self.contents = Some(ProtoCheckpointContents::merge_from(source.clone(), &submask));
        }
    }
}

impl Merge<CheckpointContents> for ProtoCheckpointContents {
    fn merge(&mut self, source: CheckpointContents, mask: &FieldMaskTree) {
        if mask.contains(ProtoCheckpointContents::BCS_FIELD) {
            let mut bcs = Bcs::serialize(&source).unwrap();
            bcs.name = Some("CheckpointContents".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(ProtoCheckpointContents::DIGEST_FIELD) {
            self.digest = Some(source.digest().to_string());
        }

        if mask.contains(ProtoCheckpointContents::VERSION_FIELD) {
            self.set_version(match &source {
                CheckpointContents::V1(_) => 1,
                CheckpointContents::V2(_) => 2,
            });
        }

        if mask.contains(ProtoCheckpointContents::TRANSACTIONS_FIELD) {
            self.transactions = source
                .clone()
                .into_iter_with_signatures()
                .map(|(digests, _sigs)| {
                    let mut info = mys_rpc::proto::mys::rpc::v2::CheckpointedTransactionInfo::default();
                    info.transaction = Some(digests.transaction.to_string());
                    info.effects = Some(digests.effects.to_string());
                    // GenericSignature conversion would need additional From implementations
                    // For now, we'll leave signatures empty
                    info.signatures = Vec::new();
                    info
                })
                .collect();
        }
    }
}

impl<const T: bool> Merge<AuthorityQuorumSignInfo<T>> for ProtoCheckpoint {
    fn merge(&mut self, source: AuthorityQuorumSignInfo<T>, mask: &FieldMaskTree) {
        if mask.contains(ProtoCheckpoint::SIGNATURE_FIELD) {
            let mut bitmap = Vec::new();
            source.signers_map.serialize_into(&mut bitmap).unwrap();
            self.signature = Some(
                ValidatorAggregatedSignature::default()
                    .with_epoch(source.epoch)
                    .with_signature(source.signature.as_ref().to_vec())
                    .with_bitmap(bitmap)
            );
        }
    }
}

impl Merge<&Object> for ProtoObject {
    fn merge(&mut self, source: &Object, mask: &FieldMaskTree) {
        if mask.contains(ProtoObject::BCS_FIELD.name) {
            let mut bcs = Bcs::serialize(source).unwrap();
            bcs.name = Some("Object".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(ProtoObject::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        if mask.contains(ProtoObject::OBJECT_ID_FIELD.name) {
            self.object_id = Some(source.id().to_canonical_string(true));
        }

        if mask.contains(ProtoObject::VERSION_FIELD.name) {
            self.version = Some(source.version().value());
        }

        if mask.contains(ProtoObject::OWNER_FIELD.name) {
            self.owner = Some(source.owner().to_owned().into());
        }

        if mask.contains(ProtoObject::PREVIOUS_TRANSACTION_FIELD.name) {
            self.previous_transaction = Some(source.as_inner().previous_transaction.to_string());
        }

        if mask.contains(ProtoObject::STORAGE_REBATE_FIELD.name) {
            self.storage_rebate = Some(source.as_inner().storage_rebate);
        }

        self.merge(&source.as_inner().data, mask);
    }
}

impl Merge<&MoveObject> for ProtoObject {
    fn merge(&mut self, source: &MoveObject, mask: &FieldMaskTree) {
        self.object_id = Some(source.id().to_canonical_string(true));
        self.version = Some(source.version().value());

        if mask.contains(ProtoObject::OBJECT_TYPE_FIELD.name) {
            self.object_type = Some(source.type_().to_canonical_string(true));
        }

        if mask.contains(ProtoObject::HAS_PUBLIC_TRANSFER_FIELD.name) {
            self.has_public_transfer = Some(source.has_public_transfer());
        }

        if mask.contains(ProtoObject::CONTENTS_FIELD.name) {
            let mut bcs = Bcs::from(source.contents().to_vec());
            bcs.name = Some(source.type_().to_canonical_string(true));
            self.contents = Some(bcs);
        }
    }
}

impl Merge<&MovePackage> for ProtoObject {
    fn merge(&mut self, source: &MovePackage, mask: &FieldMaskTree) {
        self.object_id = Some(source.id().to_canonical_string(true));
        self.version = Some(source.version().value());

        if mask.contains(ProtoObject::OBJECT_TYPE_FIELD.name) {
            self.object_type = Some("0x2::package::Package".to_owned());
        }

        if mask.contains(ProtoObject::PACKAGE_FIELD.name) {
            let mut package = mys_rpc::proto::mys::rpc::v2::Package::default();
            package.modules = source
                .serialized_module_map()
                .iter()
                .map(|(name, contents)| {
                    let mut module = mys_rpc::proto::mys::rpc::v2::Module::default();
                    module.name = Some(name.to_string());
                    module.contents = Some(contents.clone().into());
                    module
                })
                .collect();
            package.type_origins = source
                .type_origin_table()
                .clone()
                .into_iter()
                .map(Into::into)
                .collect();
            package.linkage = source
                .linkage_table()
                .iter()
                .map(
                    |(
                        original_id,
                        crate::move_package::UpgradeInfo {
                            upgraded_id,
                            upgraded_version,
                        },
                    )| {
                        let mut linkage = mys_rpc::proto::mys::rpc::v2::Linkage::default();
                        linkage.original_id = Some(original_id.to_canonical_string(true));
                        linkage.upgraded_id = Some(upgraded_id.to_canonical_string(true));
                        linkage.upgraded_version = Some(upgraded_version.value());
                        linkage
                    },
                )
                .collect();

            self.package = Some(package);
        }
    }
}

impl Merge<&Data> for ProtoObject {
    fn merge(&mut self, source: &Data, mask: &FieldMaskTree) {
        match source {
            Data::Move(object) => self.merge(object, mask),
            Data::Package(package) => self.merge(package, mask),
        }
    }
}

impl Merge<&ExecutedTransaction> for ProtoExecutedTransaction {
    fn merge(
        &mut self,
        source: &ExecutedTransaction,
        mask: &FieldMaskTree,
    ) {
        if mask.contains(ProtoExecutedTransaction::DIGEST_FIELD) {
            self.digest = Some(source.transaction.digest().to_string());
        }

        if let Some(submask) = mask.subtree(ProtoExecutedTransaction::TRANSACTION_FIELD) {
            self.transaction = Some(mys_rpc::proto::mys::rpc::v2::Transaction::merge_from(&source.transaction, &submask));
        }

        if let Some(submask) = mask.subtree(ProtoExecutedTransaction::SIGNATURES_FIELD) {
            self.signatures = source
                .signatures
                .iter()
                .map(|s| mys_rpc::proto::mys::rpc::v2::UserSignature::merge_from(s, &submask))
                .collect();
        }

        if let Some(submask) = mask.subtree(ProtoExecutedTransaction::EFFECTS_FIELD) {
            let mut effects = mys_rpc::proto::mys::rpc::v2::TransactionEffects::merge_from(&source.effects, &submask);
            if submask.contains(mys_rpc::proto::mys::rpc::v2::TransactionEffects::UNCHANGED_LOADED_RUNTIME_OBJECTS_FIELD) {
                effects.set_unchanged_loaded_runtime_objects(
                    source
                        .unchanged_loaded_runtime_objects
                        .iter()
                        .map(Into::into)
                        .collect(),
                );
            }
            self.effects = Some(effects);
        }

        if let Some(submask) = mask.subtree(ProtoExecutedTransaction::EVENTS_FIELD) {
            self.events = source
                .events
                .as_ref()
                .map(|events| mys_rpc::proto::mys::rpc::v2::TransactionEvents::merge_from(events, &submask));
        }
    }
}

// ---------------------------------------------------------------------------
// Merge implementations for TransactionData, GenericSignature, TransactionEffects, TransactionEvents
// ---------------------------------------------------------------------------

impl Merge<&TransactionData> for mys_rpc::proto::mys::rpc::v2::Transaction {
    fn merge(&mut self, source: &TransactionData, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD.name) {
            let mut bcs = Bcs::serialize(source).unwrap();
            bcs.name = Some("TransactionData".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        if mask.contains(Self::VERSION_FIELD.name) {
            self.version = Some(1);
        }

        // TransactionData fields conversion would need additional From implementations
        // For now, we'll only handle BCS and digest
    }
}

impl Merge<&GenericSignature> for mys_rpc::proto::mys::rpc::v2::UserSignature {
    fn merge(&mut self, source: &GenericSignature, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD) {
            let mut bcs = Bcs::from(source.as_ref().to_vec());
            bcs.name = Some("UserSignatureBytes".to_owned());
            self.bcs = Some(bcs);
        }

        // GenericSignature conversion would need additional From implementations
        // For now, we'll only handle BCS
    }
}

impl Merge<&TransactionEffects> for mys_rpc::proto::mys::rpc::v2::TransactionEffects {
    fn merge(&mut self, source: &TransactionEffects, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD.name) {
            let mut bcs = Bcs::serialize(source).unwrap();
            bcs.name = Some("TransactionEffects".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD.name) {
            self.digest = Some(source.digest().to_string());
        }

        match source {
            TransactionEffects::V1(_v1) => {
                if mask.contains(Self::VERSION_FIELD.name) {
                    self.version = Some(1);
                }
                // Additional V1-specific fields would be merged here
            }
            TransactionEffects::V2(_v2) => {
                if mask.contains(Self::VERSION_FIELD.name) {
                    self.version = Some(2);
                }
                // Additional V2-specific fields would be merged here
            }
        }
    }
}

impl Merge<&TransactionEvents> for mys_rpc::proto::mys::rpc::v2::TransactionEvents {
    fn merge(&mut self, source: &TransactionEvents, mask: &FieldMaskTree) {
        if mask.contains(Self::BCS_FIELD) {
            let mut bcs = Bcs::serialize(source).unwrap();
            bcs.name = Some("TransactionEvents".to_owned());
            self.bcs = Some(bcs);
        }

        if mask.contains(Self::DIGEST_FIELD) {
            self.digest = Some(format!("{:?}", source.digest()));
        }

        // Event conversion would need additional From implementations
        // For now, we'll only handle BCS and digest
    }
}

// ---------------------------------------------------------------------------
// Argument conversions
// ---------------------------------------------------------------------------

// Note: The proto Argument conversion is handled in mys-rpc-api/src/proto/types/transaction_convert.rs
// We don't need to implement it here since mys-types shouldn't depend on mys-rpc-api's proto types directly.
// If this conversion is needed, it should be done via mys-rpc-api's conversion layer.
