// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
};
use serde::{Deserialize, Serialize};

use crate::{
    MYS_FRAMEWORK_ADDRESS,
    base_types::{ObjectID, SequenceNumber},
    collection_types::VecMap,
    error::MysResult,
    object::Owner,
    storage::ObjectStore,
};
use crate::dynamic_field;

pub const COIN_REGISTRY_MODULE_NAME: &IdentStr = ident_str!("coin_registry");
pub const CURRENCY_KEY_STRUCT_NAME: &IdentStr = ident_str!("CurrencyKey");

// Note: MYS_COIN_REGISTRY_OBJECT_ID should be defined in lib.rs if needed
// For now, use a placeholder - this should match the actual coin registry object ID
pub const MYS_COIN_REGISTRY_OBJECT_ID: ObjectID = ObjectID::from_address(
    move_core_types::account_address::AccountAddress::ZERO
);

/// Rust representation of `mys::coin_registry::CurrencyKey<T>`.
#[derive(Serialize, Deserialize, Copy, Clone, Default, PartialEq, Eq)]
pub struct CurrencyKey(bool);

/// Rust representation of `mys::coin_registry::Currency<phantom T>`.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Currency {
    pub id: ObjectID,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub icon_url: String,
    pub supply: Option<SupplyState>,
    pub regulated: RegulatedState,
    pub treasury_cap_id: Option<ObjectID>,
    pub metadata_cap_id: MetadataCapState,
    pub extra_fields: VecMap<String, ExtraField>,
}

/// Rust representation of `mys::coin_registry::SupplyState<phantom T>`.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum SupplyState {
    Fixed(u64),
    BurnOnly(u64),
    Unknown,
}

/// Rust representation of `mys::coin_registry::RegulatedState`.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum RegulatedState {
    Regulated {
        cap: ObjectID,
        allow_global_pause: Option<bool>,
        variant: u8,
    },
    Unregulated,
    Unknown,
}

/// Rust representation of `mys::coin_registry::MetadataCapState`.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum MetadataCapState {
    Claimed(ObjectID),
    Unclaimed,
    Deleted,
}

/// Rust representation of `mys::coin_registry::ExtraField`.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ExtraField {
    pub type_: String,
    pub value: Vec<u8>,
}

impl Currency {
    /// Derive the ObjectID for `mys::coin_registry::Currency<$coin_type>`.
    pub fn derive_object_id(coin_type: TypeTag) -> Result<ObjectID, bcs::Error> {
        use move_core_types::identifier::Identifier;
        let key = TypeTag::Struct(Box::new(StructTag {
            address: MYS_FRAMEWORK_ADDRESS,
            module: COIN_REGISTRY_MODULE_NAME.to_owned(),
            name: CURRENCY_KEY_STRUCT_NAME.to_owned(),
            type_params: vec![coin_type],
        }));

        // Use dynamic_field to derive the object ID
        let wrapper_type_tag = TypeTag::Struct(Box::new(StructTag {
            address: MYS_FRAMEWORK_ADDRESS,
            module: Identifier::new("derived_object").unwrap(),
            name: Identifier::new("DerivedObjectKey").unwrap(),
            type_params: vec![key],
        }));

        dynamic_field::derive_dynamic_field_id(
            MYS_COIN_REGISTRY_OBJECT_ID,
            &wrapper_type_tag,
            &bcs::to_bytes(&CurrencyKey::default())?,
        )
    }

    /// Is this `StructTag` a `mys::coin_registry::Currency<...>`?
    pub fn is_currency(tag: &StructTag) -> bool {
        tag.address == MYS_FRAMEWORK_ADDRESS
            && tag.module.as_str() == "coin_registry"
            && tag.name.as_str() == "Currency"
    }
}

pub fn get_coin_registry_obj_initial_shared_version(
    object_store: &dyn ObjectStore,
) -> MysResult<Option<SequenceNumber>> {
    Ok(object_store
        .get_object(&MYS_COIN_REGISTRY_OBJECT_ID)
        .map(|obj| match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => initial_shared_version,
            _ => unreachable!("CoinRegistry object must be shared"),
        }))
}
