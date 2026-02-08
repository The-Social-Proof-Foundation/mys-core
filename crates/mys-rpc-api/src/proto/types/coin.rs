// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use mys_rpc::proto::mys::rpc::v2::{
    CoinMetadata as ProtoCoinMetadata, CoinTreasury as ProtoCoinTreasury,
    RegulatedCoinMetadata as ProtoRegulatedCoinMetadata,
};
use mys_rpc::proto::mys::rpc::v2::coin_treasury::SupplyState as ProtoSupplyState;
use mys_rpc::proto::mys::types::ObjectId as ProtoObjectId;
use mys_sdk_types::{Address, ObjectId as SdkObjectId};
use mys_types::base_types::ObjectID as MysObjectID;
use mys_types::coin::{CoinMetadata, RegulatedCoinMetadata, TreasuryCap};
use mys_types::coin_registry::{Currency, RegulatedState, SupplyState};

// Helper for ObjectId conversion (Foreign -> Foreign)
pub fn proto_object_id_from_sdk(id: SdkObjectId) -> ProtoObjectId {
    ProtoObjectId {
        object_id: Some(id.as_bytes().to_vec().into()),
    }
}

// Helper for ObjectId conversion from mys_types::base_types::ObjectID
pub fn proto_object_id_from_mys(id: MysObjectID) -> ProtoObjectId {
    // Construct ProtoObjectId directly from MysObjectID bytes
    ProtoObjectId {
        object_id: Some(id.as_bytes().to_vec().into()),
    }
}

// ---------------------------------------------------------------------------
// CoinMetadata conversions
// ---------------------------------------------------------------------------

pub fn from_coin_metadata(value: CoinMetadata) -> ProtoCoinMetadata {
    let mut metadata = ProtoCoinMetadata::default();
    metadata.id = Some(proto_object_id_from_sdk(SdkObjectId::from(value.id.id.bytes)));
    metadata.decimals = Some(value.decimals.into());
    metadata.name = Some(value.name);
    metadata.symbol = Some(value.symbol);
    metadata.description = Some(value.description);
    metadata.icon_url = value.icon_url;
    metadata
}

pub fn from_currency(value: &Currency) -> ProtoCoinMetadata {
    let mut metadata = ProtoCoinMetadata::default();
    metadata.id = Some(proto_object_id_from_sdk(SdkObjectId::from(value.id)));
    metadata.decimals = Some(value.decimals.into());
    metadata.name = Some(value.name.clone());
    metadata.symbol = Some(value.symbol.clone());
    metadata.description = Some(value.description.clone());
    metadata.icon_url = Some(value.icon_url.clone());
    metadata
}

// ---------------------------------------------------------------------------
// RegulatedCoinMetadata conversions
// ---------------------------------------------------------------------------

pub fn from_regulated_coin_metadata(value: RegulatedCoinMetadata) -> ProtoRegulatedCoinMetadata {
    let mut message = ProtoRegulatedCoinMetadata::default();
    message.id = Some(proto_object_id_from_sdk(SdkObjectId::from(value.id.id.bytes)));
    message.coin_metadata_object = Some(proto_object_id_from_sdk(SdkObjectId::from(value.coin_metadata_object.bytes)));
    message.deny_cap_object = Some(proto_object_id_from_sdk(SdkObjectId::from(value.deny_cap_object.bytes)));
    message
}

pub fn from_regulated_state(value: RegulatedState) -> ProtoRegulatedCoinMetadata {
    let mut message = ProtoRegulatedCoinMetadata::default();
    match value {
        RegulatedState::Regulated {
            cap,
            allow_global_pause: _,
            variant: _,
        } => {
            message.deny_cap_object = Some(proto_object_id_from_sdk(SdkObjectId::from(cap)));
        }
        RegulatedState::Unregulated | RegulatedState::Unknown => {
            // For unregulated or unknown, we don't set any fields
        }
    }
    message
}

// ---------------------------------------------------------------------------
// CoinTreasury conversions
// ---------------------------------------------------------------------------

pub fn from_treasury_cap(value: TreasuryCap) -> ProtoCoinTreasury {
    let mut treasury = ProtoCoinTreasury::default();
    treasury.id = Some(proto_object_id_from_sdk(SdkObjectId::from(value.id.id.bytes)));
    treasury.total_supply = Some(value.total_supply.value);
    treasury
}

pub fn from_supply_state(value: SupplyState) -> ProtoSupplyState {
    match value {
        SupplyState::Fixed(_) => ProtoSupplyState::Fixed,
        SupplyState::BurnOnly(_) => ProtoSupplyState::BurnOnly,
        SupplyState::Unknown => ProtoSupplyState::Unknown,
    }
}
